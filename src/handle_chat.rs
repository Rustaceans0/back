use chatgpt_rs::prelude::*;

pub struct ModelConfiguration {
    pub engine: ChatGPTEngine,
    pub temperature: f32,
    pub top_p: f32,
    pub presence_penalty: f32,
    pub frequency_penalty: f32,
    pub reply_count: u32,
    pub api_url: Url,
}
//here get message and response
