use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SurveyRequest {
    pub start_time: u64,
    pub data: Vec<Problem>,
}

impl SurveyRequest {
    pub fn new(start_time: u64, data: Vec<Problem>) -> Self {
        Self { start_time, data }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Problem {
    pub id: i32,
    pub is_required: bool,
    pub question: String,
    pub answer: Option<String>,
    #[serde(default)]
    pub is_ai: Option<bool>,
}

impl Problem {
    pub fn new(
        id: i32,
        is_required: bool,
        question: String,
        answer: Option<String>,
        is_ai: Option<bool>,
    ) -> Self {
        Self {
            id,
            is_required,
            question,
            answer: answer,
            is_ai: is_ai,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
