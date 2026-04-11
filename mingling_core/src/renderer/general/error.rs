#[derive(Debug)]
pub struct GeneralRendererSerializeError {
    error: String,
}

impl GeneralRendererSerializeError {
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
