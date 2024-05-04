use iced::{
    widget::{Column, Container, Row, Text, TextInput, text_input, Scrollable, scrollable, Rule},
    Element, Length, Sandbox, Settings, Color,
};

//ChatApp struct
struct ChatApp {
    alice_input_value: String,
    bob_input_value: String,
    messages: Vec<(String, String)>, // input sender and message
    alice_input_state: text_input::State, // Alice's text input state
    bob_input_state: text_input::State, // Bob's text input state
    scroll: scrollable::State,  // scrollable container state for messages UI
}

// Message enum
#[derive(Debug, Clone)]
enum Message {
    AliceInputChanged(String),
    BobInputChanged(String),
    AliceSendMessage,
    BobSendMessage,
}

// implement iced Sandbox trait for ChatApp
impl Sandbox for ChatApp {
    type Message = Message;

    // construct new ChatApp instance
    fn new() -> Self {
        Self {
            alice_input_value: String::new(),
            bob_input_value: String::new(),
            messages: Vec::new(),
            alice_input_state: text_input::State::new(),
            bob_input_state: text_input::State::new(),
            scroll: scrollable::State::new(),
        }
    }

    // set title of ChatApp
    fn title(&self) -> String {
        "Secure Instant Point-to-Point (P2P) Chat App".to_string()
    }

    // update ChatApp state based on message input (sender and message text)
    fn update(&mut self, message: Self::Message) {
        // match message input to update ChatApp state
        match message {
            // change Alice's input value
            Message::AliceInputChanged(value) => {
                self.alice_input_value = value;
            },
            // change Bob's input value
            Message::BobInputChanged(value) => {
                self.bob_input_value = value;
            },
            // send Alice's message to ChatApp
            Message::AliceSendMessage => {
                if !self.alice_input_value.trim().is_empty() {
                    self.messages.push(("Alice".to_string(), self.alice_input_value.trim().to_string()));
                    self.alice_input_value.clear();
                }
            },
            // send Bob's message to ChatApp
            Message::BobSendMessage => {
                if !self.bob_input_value.trim().is_empty() {
                    self.messages.push(("Bob".to_string(), self.bob_input_value.trim().to_string()));
                    self.bob_input_value.clear();
                }
            }
        }
    }

    // UI view of ChatApp
    fn view(&mut self) -> Element<'_, Self::Message> {
        // set header and its components
        let title = Text::new("Secure Instant Point-to-Point (P2P) Chat App")
            .size(30)
            .horizontal_alignment(iced::alignment::Horizontal::Center);
        let subtitle = Text::new("Welcome to the Chat App! Alice and Bob can communicate securely, with the peace of mind that their messages are encrypted with perfect forward secrecy. Get started by typing Alice's and Bob's messages below. Enjoy! -- Final Computer Security Project by Charlotte Cabrera, Kate Carbonell, Luke Terry, and Natalia Mora.")
            .size(20)
            .horizontal_alignment(iced::alignment::Horizontal::Center);
        let separator = Rule::horizontal(10);
        let header = Column::new()
            .push(title)
            .push(subtitle)
            .push(separator)
            .spacing(5);
    
        // set Alice's input components
        let alice_input = TextInput::new(
            &mut self.alice_input_state,
            "Alice, type your message here...",
            &self.alice_input_value,
            Message::AliceInputChanged
        )
        .on_submit(Message::AliceSendMessage)
        .padding(10)
        .size(20);
    
        // set Bob's input components
        let bob_input = TextInput::new(
            &mut self.bob_input_state,
            "Bob, type your message here...",
            &self.bob_input_value,
            Message::BobInputChanged
        )
        .on_submit(Message::BobSendMessage)
        .padding(10)
        .size(20);
    
        // set input components
        let inputs = Row::new()
            .push(alice_input)
            .push(bob_input)
            .spacing(20);
    
        // set scrollable container for messages
        let chat_messages = self.messages.iter().fold(
            Scrollable::new(&mut self.scroll).spacing(10).padding(20).width(Length::Fill).height(Length::FillPortion(5)),
            |scroll, (sender, message)| {
                let color = if sender == "Alice" { Color::from_rgb(1.0, 0.0, 0.0) } else { Color::from_rgb(0.0, 0.0, 1.0) }; // Alice is red, Bob is blue
                let label = Text::new(format!("{}: ", sender)).color(color);
                // clone message and send to scrollable container
                let user_message = Text::new(message.clone());
                scroll.push(Row::new().push(label).push(user_message))
            },
        );
    
        // set content components (text inside scrollable container)
        let content = Column::new()
            .push(header)
            .push(chat_messages)
            .push(inputs)
            .max_width(800)
            .padding(20);  // Adds padding around the entire content
    
        // set content container (centered in the window)
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
    
}

// main func to run ChatApp
fn main() {
    let settings = Settings::default();
    // run ChatApp, print error if failed
    if let Err(e) = ChatApp::run(settings) {
        eprintln!("ERROR: Failed to start the Chat App: {:?}", e);
    }
}
