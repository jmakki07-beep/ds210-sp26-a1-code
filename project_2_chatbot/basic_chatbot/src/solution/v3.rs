use kalosm::language::*;
use std::collections::HashMap;
#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama,
    sessions: HashMap<String, Chat<Llama>>,
}
impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
    model,
    sessions: HashMap::new(),
};
    }

    #[allow(dead_code)]

    
pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
    if !self.sessions.contains_key(&username) {
        let new_session = self
            .model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        self.sessions.insert(username.clone(), new_session);
    }

    let chat_session = self.sessions.get_mut(&username).unwrap();

    let response = chat_session
        .add_message(ChatMessage::new(MessageType::UserMessage, message))
        .await;

    match response {
        Ok(reply) => reply,
        Err(_) => String::from("Sorry, something went wrong."),
    }
}

    #[allow(dead_code)]
pub fn get_history(&self, username: String) -> Vec<String> {
    if let Some(chat) = self.sessions.get(&username) {
        let history = chat.session().unwrap().history();

        let mut result = Vec::new();
        let mut i = 0;

        for msg in history {
            if i > 0 {
                result.push(msg.content().to_string());
            }
            i += 1;
        }

        return result;
    }

    Vec::new()
}
}