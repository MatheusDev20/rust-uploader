use serde::{ser::SerializeStruct, Serialize};

pub struct UploadResponse {
    pub status: &'static str,
    pub message: &'static str,
}

impl Serialize for UploadResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("UploadResponse", 2)?;
        state.serialize_field("status", &self.status)?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}
