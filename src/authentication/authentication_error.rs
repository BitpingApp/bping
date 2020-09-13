use std::fmt;

#[derive(Debug, Clone)]
pub struct AuthenticationError;


impl fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to authenticate.")
    }
}