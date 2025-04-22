#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {

    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content)
    }
}

impl <T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}


fn main() {
    let ChatMessage = ChatMessage {
        content: "Hello world",
        time: "12:00".to_string(),
    };

    let ChatMessage = ChatMessage {
        content: String::from("Hello world"),
        time: "12:00".to_string(),
    };

    let ChatMessage = ChatMessage {
        content: DigitalContent::AudioFile,
        time: "12:00".to_string(),
    };
    

    ChatMessage.consume_entertainment();
    println!("The time is {}", ChatMessage.retrieve_time());
}
