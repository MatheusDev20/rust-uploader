# Mini-Tube — Project Plan

A mini YouTube clone built in Rust, for learning purposes. The goal is to understand Rust, its ecosystem, and how real systems are built — not to build a production-ready product.

---

## Part 1 — Video Upload & Metadata Storage

### Video Upload
- Accept video file uploads via HTTP
- Enforce a file size limit
- Store the video file in AWS S3 (object storage)

### Metadata Storage
- After a successful upload, store video metadata (e.g. title, filename, S3 key, upload date, size) in a database
- Metadata and the S3 object are linked by a unique identifier

---

## Parts to be defined
- Video listing / retrieval
- Playback
- Authentication
