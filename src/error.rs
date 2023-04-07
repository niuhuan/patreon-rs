use reqwest::StatusCode;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

pub type PatreonResult<A> = std::result::Result<A, PatreonError>;

#[derive(Debug)]
pub enum PatreonError {
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    PatreonOAuth(StatusCode, String),
    PatreonApi(StatusCode, Vec<ApiError>),
    Message(String),
}

impl Display for PatreonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PatreonError::Reqwest(err) => Display::fmt(err, f),
            PatreonError::SerdeJson(err) => Display::fmt(err, f),
            PatreonError::PatreonOAuth(code, msg) => {
                f.write_str(format!("PatreonOAuth{{ {code} : {msg} }}").as_str())
            }
            PatreonError::PatreonApi(code, erros) => {
                f.write_str(format!("PatreonApi {{ code : {code}, errors : [ ").as_str())?;
                for x in erros {
                    Display::fmt(x, f)?;
                    f.write_str(", ")?;
                }
                f.write_str(" ] }")?;
                Ok(())
            }
            PatreonError::Message(msg) => {
                write!(f, "Message ( {msg} ) ,")
            }
        }
    }
}

impl std::error::Error for PatreonError {}

impl From<reqwest::Error> for PatreonError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<serde_json::Error> for PatreonError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ApiError {
    pub code: Option<i64>,
    pub code_name: String,
    pub detail: String,
    pub id: String,
    pub status: String,
    pub title: String,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ApiError(")?;
        write!(f, "code : {:?}, ", self.code)?;
        write!(f, "code_name : {}, ", self.code_name)?;
        write!(f, "detail : {}, ", self.detail)?;
        write!(f, "id : {}, ", self.id)?;
        write!(f, "status : {}, ", self.status)?;
        write!(f, "title : {}, ", self.title)?;
        f.write_str(")")
    }
}

impl std::error::Error for ApiError {}
