use aws_sdk_s3::{Client, primitives::ByteStream};

pub async fn create_uploader() -> S3Uploader {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    //println!("{:?}", config);
    // Borrow the client -> Ownership system
    let client = Client::new(&config);
    let bucket = std::env::var("AWS_S3_BUCKET").expect("AWS_S3_BUCKET must be set");

    S3Uploader::new(client, bucket)
}

// Clone is required by Axum's State — it clones the uploader to inject it into each handler.
// The AWS Client is already cheap to clone (it uses Arc internally, like a JS reference).
#[derive(Clone)]
pub struct S3Uploader {
    client: Client,
    bucket: String,
}

impl S3Uploader {
    // Associated function (static method in JS) — this is the constructor.
    // `pub` means it's accessible from outside this file (like `export` in JS).
    pub fn new(client: Client, bucket: String) -> S3Uploader {
        S3Uploader { client, bucket }
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
        data: Vec<u8>,
    ) -> Result<(), aws_sdk_s3::Error> {
        let body = ByteStream::from(data);

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
}
