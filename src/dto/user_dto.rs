use serde::{Deserialize, Serialize};
use validator::{Validate};

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct User {
    #[validate(length(min = 1))]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String
}