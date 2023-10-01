use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FieldError {
    pub message: String,
    pub field: String,
}

#[derive(Debug, Serialize)]
pub struct BaseError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FieldError>>,
}

impl BaseError {
    pub fn new(msg: String) -> Self {
        Self {
            message: msg,
            fields: None,
        }
    }
}
