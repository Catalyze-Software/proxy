pub use catalyze_shared::api_error::{ApiError};

// use std::fmt;
//
// use candid::CandidType;
// use ic_cdk::api::time;
// use serde::{Deserialize, Serialize};
//
// use super::validation::ValidationResponse;
//
// #[derive(Clone, CandidType, Debug, Serialize, Deserialize)]
// pub struct ApiError {
//     tag: Option<String>,
//     message: Option<String>,
//     method_name: Option<String>,
//     error_type: ApiErrorType,
//     info: Option<Vec<String>>,
//     timestamp: u64,
// }
//
// impl ApiError {
//     pub fn new(error_type: ApiErrorType) -> Self {
//         ApiError {
//             tag: None,
//             message: None,
//             method_name: None,
//             error_type,
//             info: None,
//             timestamp: time(),
//         }
//     }
//
//     pub fn validation_response(validation_response: Vec<ValidationResponse>) -> Self {
//         Self::new(ApiErrorType::ValidationError(Box::new(validation_response)))
//     }
//
//     pub fn serialize() -> Self {
//         Self::new(ApiErrorType::SerializeError)
//     }
//
//     pub fn deserialize() -> Self {
//         Self::new(ApiErrorType::DeserializeError)
//     }
//
//     pub fn unexpected() -> Self {
//         Self::new(ApiErrorType::Unexpected)
//     }
//
//     pub fn not_implemented() -> Self {
//         Self::new(ApiErrorType::NotImplemented)
//     }
//
//     pub fn unauthorized() -> Self {
//         Self::new(ApiErrorType::Unauthorized)
//     }
//
//     pub fn not_found() -> Self {
//         Self::new(ApiErrorType::NotFound)
//     }
//
//     pub fn bad_request() -> Self {
//         Self::new(ApiErrorType::BadRequest)
//     }
//
//     pub fn unsupported() -> Self {
//         Self::new(ApiErrorType::Unsupported)
//     }
//
//     pub fn duplicate() -> Self {
//         Self::new(ApiErrorType::Duplicate)
//     }
//
//     pub fn add_tag(mut self, tag: &str) -> Self {
//         self.tag = Some(tag.to_string());
//         self
//     }
//
//     pub fn add_message(mut self, message: &str) -> Self {
//         self.message = Some(message.to_string());
//         self
//     }
//
//     pub fn add_info(mut self, info: &str) -> Self {
//         if let Some(mut info_vec) = self.info {
//             info_vec.push(info.to_string());
//             self.info = Some(info_vec);
//         } else {
//             self.info = Some(vec![info.to_string()]);
//         }
//         self
//     }
//
//     pub fn add_method_name(mut self, method_name: &str) -> Self {
//         self.method_name = Some(method_name.to_string());
//         self
//     }
// }
//
// #[derive(Clone, CandidType, Debug, Deserialize, Serialize)]
// pub enum ApiErrorType {
//     NotImplemented,
//     Unexpected,
//     Unauthorized,
//     NotFound,
//     BadRequest,
//     Unsupported,
//     Duplicate,
//     ValidationError(Box<Vec<ValidationResponse>>),
//     SerializeError,
//     DeserializeError,
// }
//
// impl fmt::Display for ApiError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "ApiError: tag: {:?}, message: {:?}, method_name: {:?}, error_type: {:?}, info: {:?}",
//             self.tag, self.message, self.method_name, self.error_type, self.info
//         )
//     }
// }
//
// impl fmt::Display for ApiErrorType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         use ApiErrorType::*;
//         match self {
//             NotImplemented => write!(f, "NotImplemented"),
//             Unexpected => write!(f, "Unexpected"),
//             Unauthorized => write!(f, "Unauthorized"),
//             NotFound => write!(f, "NotFound"),
//             BadRequest => write!(f, "BadRequest"),
//             Unsupported => write!(f, "Unsupported"),
//             Duplicate => write!(f, "Duplicate"),
//             ValidationError(_) => write!(f, "ValidationError"),
//             SerializeError => write!(f, "SerializeError"),
//             DeserializeError => write!(f, "DeserializeError"),
//         }
//     }
// }
