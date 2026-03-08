use aws_sdk_s3::{Client, presigning::PresigningConfig, primitives::ByteStream, types::ObjectCannedAcl};
use std::time::Duration;

pub async fn create_uploader() -> S3Uploader {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = Client::new(&config);
    let bucket = std::env::var("AWS_S3_BUCKET").expect("AWS_S3_BUCKET must be set");
    let region = std::env::var("AWS_REGION").expect("AWS_REGION must be set");

    S3Uploader::new(client, bucket, region)
}

#[derive(Clone)]
pub struct S3Uploader {
    client: Client,
    bucket: String,
    region: String,
}

impl S3Uploader {
    pub fn new(client: Client, bucket: String, region: String) -> S3Uploader {
        S3Uploader { client, bucket, region }
    }

    // Method — called on an instance: uploader.upload(...)
    //
    // &self         → read-only reference to the instance (like `this` in JS, but explicit)
    // key: &str     → the S3 object key, e.g. "videos/my-video.mp4"
    // data: Vec<u8> → the raw file bytes (Vec<u8> is like a byte array: Uint8Array in JS)
    //
    // -> Result<(), aws_sdk_s3::Error>
    //    Result is Rust's way of handling errors — no try/catch, no thrown exceptions.
    //    Result<(), E> means: "on success return nothing () — on failure return an error E"
    //    Think of it like a Promise in JS: it either resolves or rejects.
    pub async fn upload(
        &self,
        key: &str,
        data: &[u8],
    ) -> Result<(), aws_sdk_s3::Error> {
        let body = ByteStream::from(data.to_vec()); // owns the bytes

        // This is the SDK call — same as PutObjectCommand in the JS SDK.
        // The chained .bucket().key().body() calls are the builder pattern:
        // each call sets one field and returns the builder itself so you can keep chaining.
        // .send().await is the equivalent of `await client.send(command)` in JS.
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body)
            .send()
            .await?; // the `?` operator: if this fails, immediately return the error
                     // to the caller — like a throw, but explicit and part of the type system

        Ok(()) // everything went fine — wrap "nothing" in Ok() to signal success
    }

    // Upload an object with public-read ACL and return its permanent public URL.
    // Unlike `upload`, this makes the object readable by anyone without credentials.
    //
    // The public URL format is:
    //   https://<bucket>.s3.<region>.amazonaws.com/<key>
    //
    // NOTE: this requires "Block Public Access" to be DISABLED on the S3 bucket,
    // and the bucket must allow ACLs (Object Ownership set to "ACL enabled").
    // These settings are configured in the AWS Console under the bucket's Permissions tab.
    pub async fn upload_public(
        &self,
        key: &str,
        data: &[u8],
    ) -> Result<String, aws_sdk_s3::Error> {
        let body = ByteStream::from(data.to_vec());

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body)
            // `public-read` — anyone can GET this object without credentials.
            // In JS SDK: { ACL: 'public-read' }
            .acl(ObjectCannedAcl::PublicRead)
            .send()
            .await?;

        // Build the permanent public URL — no expiry, no signature needed.
        let url = format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            self.bucket, self.region, key
        );

        Ok(url)
    }

    // Generate a temporary pre-signed GET URL for an S3 object.
    //
    // A pre-signed URL is a regular HTTPS URL with AWS credentials baked in as
    // query parameters (X-Amz-Signature, X-Amz-Expires, etc.). Anyone who holds
    // the URL can GET the object — no AWS credentials needed on the client.
    //
    // `expires_in` controls how long the URL is valid. After that it returns 403.
    // This is equivalent to getSignedUrl(client, new GetObjectCommand({...}), { expiresIn: 3600 })
    // in the JS SDK.
    //
    // Returns `Result<String, String>` — success is the URL, failure is an error message.
    // We map SDK errors to String so callers don't need to import SDK error types.
    pub async fn presign_url(
        &self,
        key: &str,
        expires_in: Duration,
    ) -> Result<String, String> {
        // PresigningConfig::expires_in can fail if the duration is zero or too large.
        let config = PresigningConfig::expires_in(expires_in)
            .map_err(|e| e.to_string())?;

        // `.presigned()` doesn't actually make an HTTP request — it just signs the URL locally.
        // This is different from `.send()` which executes the request.
        let presigned = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(config)
            .await
            .map_err(|e| e.to_string())?;

        Ok(presigned.uri().to_string())
    }
}
