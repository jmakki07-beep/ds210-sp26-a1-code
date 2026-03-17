use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    chat_session: Chat<Llama>,
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        let chat_session = model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        ChatbotV2 { chat_session }
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        let response = self.chat_session
            .add_message(ChatMessage::new(MessageType::UserMessage, message))
            .await;

        match response {
            Ok(reply) => reply,
            Err(_) => String::from("Sorry, something went wrong."),
        }
    }
}