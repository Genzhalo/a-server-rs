use validator::Validate;

use crate::app::errors::{BaseError, FieldError};

pub fn validate<T: Validate>(data: &T) -> Result<(), BaseError> {
  match data.validate() {
      Ok(_) => Ok(()),
      Err(e) => {
          let mut errors: Vec<FieldError> = vec![];
          e.field_errors().iter().for_each(|f| {
              errors.push(FieldError {
                  field: f.0.to_string(),
                  message: f.1.first().unwrap().clone().message.unwrap().to_string(),
              })
          });
          return Err(BaseError {
              message: "".to_string(),
              fields: Some(errors),
          });
      }
  }
}