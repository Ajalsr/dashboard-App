use thiserror::Error;

#[derive(Debug, Error)]
pub enum PersonError {
    #[error("number not found")]
    PersonNotFound,
    #[error("failed to update member")]
    PersonUpdateFailure,
    #[error("failed to create member")]
    PersonCreationFailure,
    #[error("failed to delete member")]
    PersonDeleteFailure
}

pub type ErrorMessage = String;

pub trait ResponseErrorTrait {
    fn create(person_error: PersonError) -> ErrorMessage ;
}

impl ResponseErrorTrait for ErrorMessage {
    fn create(person_error: PersonError) -> ErrorMessage {
        match person_error {
            PersonError::PersonNotFound => ErrorMessage::from("Member not found"),
            PersonError::PersonUpdateFailure => ErrorMessage::from("failed to update"),
            PersonError::PersonCreationFailure => ErrorMessage::from("Failed to create member"),
            PersonError::PersonDeleteFailure => ErrorMessage::from("Failed to delete member"),
        }
    }
}