use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                let mut chat_session = self
                    .model
                    .chat()
                    .with_system_prompt("The assistant will act like a pirate");

                match file_library::load_chat_session_from_file(filename) {
                    None => {}
                    Some(session) => {
                        chat_session = chat_session.with_session(session);
                    }
                }
                let response = chat_session.add_message(message).await.unwrap();
                let session = chat_session.session().unwrap().try_clone().unwrap();
                file_library::save_chat_session_to_file(filename, &session);
                self.cache.insert_chat(username, chat_session);
                return response;
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                let response = chat_session.add_message(message).await.unwrap();

                let session = chat_session.session().unwrap().try_clone().unwrap();
                file_library::save_chat_session_to_file(filename, &session);

                return response;

            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");

                match file_library::load_chat_session_from_file(filename) {
                    None => {
                        return Vec::new();
                    }
                    Some(session) => {
                        let history = session.history();
                        let mut result = Vec::new();

                        for msg in history {
                            result.push(msg.content().to_string());
                        }

                        return result;
                    }
                }
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");

                let history = chat_session.session().unwrap().history();
                let mut result = Vec::new();

                for msg in history {
                    result.push(msg.content().to_string());
                }

                return result;
            }
        }
    }
}