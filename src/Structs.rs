use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]

pub struct SurveyRequest {
    pub code: i32,
    pub message: String,
    pub data: Vec<Problem>,
}

impl SurveyRequest {
    pub fn new(code: i32, message: Option<String>, data: Vec<Problem>) -> Self {
        Self {
            code,
            message: message.unwrap_or("none".to_string()),
            data,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    pub id: i32,
    pub is_optional: bool,
    pub question: String,
    pub answer: String,
}

impl Problem {
    pub fn new(id: i32, is_optional: bool, question: String, answer: String) -> Self {
        Self {
            id,
            is_optional,
            question,
            answer,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitResponse {
    pub code: i32,
    pub message: String,
}

impl SubmitResponse {
    pub fn new(code: i32, message: Option<String>) -> Self {
        Self {
            code,
            message: message.unwrap_or("none".to_string()),
        }
    }
}
