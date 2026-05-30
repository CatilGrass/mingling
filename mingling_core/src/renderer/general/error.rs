/// Represents an error that occurs during serialization of a general renderer.
///
/// This error stores a human-readable message describing what went wrong
/// during the serialization process.
#[derive(Debug)]
pub struct GeneralRendererSerializeError {
    /// The underlying error message.
    error: String,
}

impl GeneralRendererSerializeError {
    #[must_use]
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

impl From<&str> for GeneralRendererSerializeError {
    fn from(s: &str) -> Self {
        Self::new(s.to_string())
    }
}

impl std::ops::Deref for GeneralRendererSerializeError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.error
    }
}

impl From<GeneralRendererSerializeError> for String {
    fn from(val: GeneralRendererSerializeError) -> Self {
        val.error
    }
}
