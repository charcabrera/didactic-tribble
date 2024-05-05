use iced::{
    widget::{Column, Container, Row, Text, TextInput, text_input, Scrollable, scrollable, Rule, Button},
    Element, Length, Sandbox, Settings, Color
};
use std::net::TcpStream;
use ring::aead::{LessSafeKey, UnboundKey, AES_256_GCM};
use std::io::Write;
use iced::button::State;
use std::str::from_utf8;
mod encryption;
mod tcp_comms;

/*
this file creates a GUI and uses the other modules to maintain an encrypted messaging session between instances of the app
*/

//ChatApp struct
struct ChatApp {
    alice_input_value: String,
    bob_input_value: String,
    messages: Vec<(String, String)>, // input sender and message
    alice_input_state: text_input::State, // Alice's text input state
    bob_input_state: text_input::State, // Bob's text input state
    scroll: scrollable::State,  // scrollable container state for messages UILessSafeKey
    seed: i32,
    password: String,
    buf: Vec<u8>,
    stream: TcpStream,
    key: LessSafeKey,
    is_alice: bool,
    refresh_button_state: iced::button::State,
}

// Message enum
#[derive(Debug, Clone)]
enum Message {
    AliceInputChanged(String),
    BobInputChanged(String),
    AliceSendMessage,
    BobSendMessage,
    Refresh
}

// implement iced Sandbox trait for ChatApp
impl Sandbox for ChatApp {
    type Message = Message;

    // construct new ChatApp instance
    fn new() -> Self {
        let password: String = "Password".to_owned();
        let mut seed: i32 = encryption::generate_random_number();
        let og_seed: i32 = seed;
        let mut buf: Vec<u8> = vec![];
        // negotiate a TCP connection with the other party
        let stream : TcpStream = tcp_comms::establish_tcp_conn(&mut seed, &mut buf).expect("TCP Connection Could Not Be Established");
        let mut key = encryption::build_key_from_password(password.to_owned(), seed);
        let is_alice = og_seed == seed;
        let mut state = State::new();

        Self {
            alice_input_value: String::new(),
            bob_input_value: String::new(),
            messages: Vec::new(),
            alice_input_state: text_input::State::new(),
            bob_input_state: text_input::State::new(),
            scroll: scrollable::State::new(),
            seed: seed,
            password: password,
            buf: buf,
            stream: stream,
            key: key,
            is_alice: is_alice,
            refresh_button_state: state,
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
                    let ciphertext = send_message(&self.alice_input_value, &self.stream, &mut self.key, &self.seed);
                    self.messages.push(("Alice".to_string(), self.alice_input_value.trim().to_string()));
                    self.messages.push(("Ciphertext".to_string(), ciphertext));
                    self.alice_input_value.clear();
                }
            },
            // send Bob's message to ChatApp
            Message::BobSendMessage => {
                if !self.bob_input_value.trim().is_empty() {
                    let ciphertext = send_message(&self.bob_input_value, &self.stream, &mut self.key, &self.seed);
                    self.messages.push(("Bob".to_string(), self.bob_input_value.trim().to_string()));
                    self.messages.push(("Ciphertext".to_string(), ciphertext));
                    self.bob_input_value.clear();
                }
            }

            Message::Refresh => {
                // poll for received tcp messages
                let omr = |msg: &mut Vec<u8>|{
                    let (text, ciphertext) : (String, String) = on_message_received(msg, &mut self.key, &self.seed);
                    if text != "" {
                        self.messages.push(("Ciphertext".to_string(), ciphertext));
                        if self.is_alice {
                            self.messages.push(("Bob".to_string(), text));
                        } else {
                            self.messages.push(("Alice".to_string(), text));
                        }
                    }
                    (*msg).clear();
                };
                tcp_comms::poll_tcp_stream(&mut self.buf, &self.stream, omr);

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
        let refresh_button = Button::new(&mut self.refresh_button_state, Text::new("Refresh Messages"))
            .padding(10)
            .on_press(Message::Refresh);
        let mut inputs = Row::new();
        if self.is_alice {
            inputs = Row::new()
                .push(alice_input)
                .push(refresh_button)
                .spacing(20);
        } else {
            inputs = Row::new()
            .push(bob_input)
            .push(refresh_button)
            .spacing(20);
        }
    
        // set scrollable container for messages
        let chat_messages = self.messages.iter().fold(
            Scrollable::new(&mut self.scroll).spacing(10).padding(20).width(Length::Fill).height(Length::FillPortion(5)),
            |scroll, (sender, message)| {
                let color = if sender == "Alice" { Color::from_rgb(1.0, 0.0, 0.0) } else if sender == "Bob" { Color::from_rgb(0.0, 0.0, 1.0) } else { Color::from_rgb(0.0, 0.5, 0.5) }; // Alice is red, Bob is blue
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

// called whenever a message is received...
fn on_message_received(messages : &mut Vec<u8>, k: &mut LessSafeKey, seed: &i32) -> (String, String){
    // decrypt the message
    //    println!("{:?}", messages);
    let ciphertext = String::from_utf8_lossy(messages).into_owned();
    encryption::decrypt_message(k.clone(), messages);

    // remove the last 32 bytes for the key
    let mlen = messages.len();
    let buf = messages.as_slice()[mlen-32..].to_vec();
    messages.drain(mlen-32..);
    //println!("{:?}", buf);

    // display message
    let text = from_utf8(messages).unwrap();

    // generate a new key based on the last 32 bits of the plaintext
    *k = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, &buf).unwrap());

    (text.to_string(), ciphertext)
}

fn send_message(msg: &String, mut stream: &TcpStream, k: &mut LessSafeKey, seed: &i32) -> String{
    // encrypt the message
    let message : String = (*msg).clone();
    // generate a 32 bit random key
    let buf = encryption::generate_random_key();

    let ciphertext: &mut Vec<u8> = &mut message.clone().into_bytes();
    
    // append the key to the plaintext before encryption
    ciphertext.append(&mut buf.to_vec());
    encryption::encrypt_message(k.clone(), ciphertext);

    // generate a new key based on the random generator
    *k = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, &buf).unwrap());

    // write the message to the TCP Stream
    let _ = stream.write(ciphertext);
    return String::from_utf8_lossy(ciphertext).into_owned();
}

// main func to run ChatApp
fn main() {
    let settings = Settings::default();
    // run ChatApp, print error if failed
    if let Err(e) = ChatApp::run(settings) {
        eprintln!("ERROR: Failed to start the Chat App: {:?}", e);
    }
}
