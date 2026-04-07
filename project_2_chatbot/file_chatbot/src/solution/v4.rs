use kalosm::language::*;
use crate::solution::file_library;

pub struct ChatbotV4 {
    model: Llama,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);

        let mut chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");
        match file_library::load_chat_session_from_file(filename) {
            None => {}
            Some(session) => {
                chat_session = chat_session.with_session(session);
            }
        }

        let response = chat_session.add_message(message).await.unwrap();

        let session = chat_session.session().unwrap();
        file_library::save_chat_session_to_file(filename, &session);

        return response;
    }

pub fn get_history(&self, username: String) -> Vec<String> {
    let filename = &format!("{}.txt", username);

    match file_library::load_chat_session_from_file(filename) {
        None => Vec::new(),
        Some(session) => {
            let history = session.history();
            let mut result = Vec::new();

            let mut i = 0;

            for msg in history {
                if i > 0 {
                    result.push(msg.content().to_string());
                }
                i += 1;
            }

            result
        }
    }
}
    
}
