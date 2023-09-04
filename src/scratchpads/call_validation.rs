use serde::Deserialize;
use std::collections::HashMap;


#[derive(Debug, Deserialize)]
pub struct CursorPosition {
    pub file: String,
    pub line: i32,
    pub character: i32,
}


#[derive(Debug, Deserialize)]
pub struct CodeCompletionInputs {
    pub sources: HashMap<String, String>,
    pub cursor: CursorPosition,
    pub multiline: bool,
}


#[derive(Debug, Deserialize)]
pub struct CodeCompletionPost {
    pub model: String,
    pub stream: bool,
    pub inputs: CodeCompletionInputs,
}

