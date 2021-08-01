use std::error::{Error};
use std::fmt;


#[derive(Debug)]
pub struct HxError {
    pub message: String,
    pub retry: bool,
    pub origin: Option<Box<dyn Error>>
}

impl HxError {
    pub fn warp(retry: bool, origin: Box<dyn Error>) -> HxError{
        return HxError{message: format!("{}", origin), retry: retry, origin: Option::Some(origin)}
    }
    pub fn warp_str(retry: bool, message: String) -> HxError{
        return HxError{message: message, retry: retry, origin: Option::None}
    }
}

impl Error for HxError {
    fn description(&self) -> &str{
        return self.message.as_ref();
    }
    fn cause(&self) -> Option<&dyn Error> {
        match self.origin {
            Some(ref val) => Option::Some(val.as_ref()),
            None => Option::None
        }
    } 
}

impl fmt::Display for HxError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<HxFsErr: {}>", self.message)
    }
}
