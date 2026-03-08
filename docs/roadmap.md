# mini-tube — System Design Roadmap

A learning roadmap for building a production-like video platform.
Each item is a real system design concept with a concrete implementation target.

---

## Already built

- S3 for video storage
- Background transcoding with ffmpeg (`tokio::spawn` + `spawn_blocking`)
- Pre-signed S3 URLs for video playback
- Postgres for video metadata
- Automatic migrations on startup (`sqlx::migrate!`)

---

## Networking & Delivery

### CloudFront CDN
Put a CDN in front of S3. The first time a video is requested, CloudFront fetches it
from S3 and caches it at an edge node close to the user. Every subsequent request is
served from cache — faster and cheaper.

Replace pre-signed S3 URLs with signed CloudFront URLs so the bucket stays fully private.

### Custom domain + HTTPS
Route 53 for DNS, ACM for the TLS certificate. Put CloudFront and the API load balancer
behind a real domain instead of raw AWS endpoints.

### Rate limiting
Limit how many requests a single client can make in a time window. Prevents abuse and
protects the DB. Can be implemented as an Axum middleware or at the API Gateway level.

---

## Compute & Scaling

### Containerize with Docker
Package the Rust API into a Docker image. This is the prerequisite for deploying to
any managed container platform and makes the environment reproducible.

### Load balancer + multiple instances (ECS or EKS)
Run multiple instances of the API behind an Application Load Balancer (ALB). The ALB
distributes incoming requests across instances. If one crashes, the others keep serving.

ECS is the simpler path (managed containers). EKS is Kubernetes — more control, more complexity.

### Auto-scaling
Define rules: if average CPU goes above 70%, add an instance. If it drops below 20%,
remove one. The platform handles it automatically. This is the core of horizontal scaling.

---

## Async Processing

### Decouple transcoding with a message queue (SQS)
Currently the API spawns a background task directly. If the server crashes mid-transcode,
the job is lost.

Better approach:
1. Upload handler pushes a message to an SQS queue: `{ video_id, source_key }`
2. A separate **worker service** polls the queue, transcodes, uploads, marks ready
3. SQS retains the message until the worker explicitly deletes it — no job is ever lost

This also means the API and the worker scale independently. Transcoding is CPU-heavy;
you can run 1 API instance and 10 worker instances during a spike.

### Dead letter queue (DLQ)
If a transcode job fails repeatedly (e.g. corrupt file), SQS moves it to a dead letter
queue after N attempts. The job is preserved for inspection and manual retry instead of
silently disappearing.

---

## Storage & Database

### RDS (managed Postgres)
Replace the local Postgres container with AWS RDS. Gets you automated backups, point-in-time
recovery, and Multi-AZ failover (if the primary goes down, a standby takes over automatically).

### Read replica
Read queries (`GET /videos`, `GET /videos/:id`) go to a read replica.
Write queries (`INSERT`, `UPDATE`) go to the primary.
This offloads read traffic from the primary and increases overall throughput.

### Redis cache
Cache video metadata in Redis with a short TTL (e.g. 60 seconds).
`GET /videos/:id` hits Redis first — if it's a cache hit, the DB is never touched.
Dramatically reduces DB load for popular videos.

---

## Observability

### Structured logging
Emit JSON logs with a consistent schema: request ID, user ID, duration, status code.
This makes logs machine-readable and lets you trace a single request across log lines.

### Distributed tracing
Attach a trace ID to every request and propagate it through the API, worker, DB calls,
and S3 calls. Tools like AWS X-Ray or OpenTelemetry let you see a waterfall of exactly
what happened and where time was spent for any given request.

### Metrics + dashboards
Track: request latency (p50, p95, p99), error rate, queue depth, transcoding duration.
Visualize in CloudWatch or Grafana. These are the numbers you look at when something breaks.

### Alerting
Define thresholds and get notified when they are breached: error rate > 1%, queue depth
growing for more than 5 minutes, no successful transcodings in the last hour.
PagerDuty or SNS → email/Slack.

---

## Security

### Authentication (JWT)
Replace the `Uuid::nil()` placeholder owner_id with real user identity.
Issue JWTs on login, verify them in an Axum middleware, attach the user ID to every
DB write. Pre-signed stream URLs should only be issued to the video's owner (or public,
depending on business logic).

### IAM roles (no hardcoded credentials)
EC2/ECS instances get S3 and SQS access via an IAM role attached to the instance/task.
No `AWS_ACCESS_KEY_ID` in env files. Credentials are fetched from the instance metadata
service automatically by the AWS SDK.

### VPC (network isolation)
Put RDS, ElastiCache (Redis), and the worker in a **private subnet** — no public internet
access. Only the ALB lives in the public subnet. The API instances sit in a private subnet
too and reach S3/SQS via VPC endpoints (traffic never leaves the AWS network).

### Secrets Manager
`DATABASE_URL`, Redis URL, and any API keys are stored in AWS Secrets Manager.
The application fetches them at startup. No secrets in environment files or Docker images.

---

## Reliability

### Health check endpoint (`GET /health`)
Returns `200 OK` if the server is up and can reach the DB. The ALB calls this every few
seconds and removes an instance from rotation if it starts failing. Required for
auto-scaling and zero-downtime deploys to work correctly.

### Graceful shutdown
When the platform wants to terminate an instance (scale-in, deploy), it sends SIGTERM.
The server should stop accepting new requests, finish in-flight ones, then exit.
Without this, active requests get cut off mid-response.

### Idempotent job processing
If a transcode job is retried (e.g. worker crashed after transcoding but before deleting
the SQS message), it should not create a duplicate video or fail loudly.
Check if `processed_key` already exists in S3 before transcoding again.

---

## Suggested order

1. **Health check endpoint** — small, unblocks load balancer setup
2. **Docker** — prerequisite for everything below
3. **SQS-based transcoding** — biggest architectural improvement, teaches decoupling
4. **CloudFront** — high impact on perceived performance
5. **Auth (JWT)** — needed before anything user-facing is real
6. **VPC + IAM roles + Secrets Manager** — do these together as a "security pass"
7. **RDS + read replica** — replace local DB
8. **Redis cache** — add after read replica to reduce DB load further
9. **ECS + ALB + auto-scaling** — deploy the whole thing
10. **Observability** — structured logging → tracing → metrics → alerting
