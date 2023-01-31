use tokio::sync::mpsc;
use tokio::task;

// Define the message type that the actor will receive
#[derive(Debug)]
enum Message {
    Increment,
    Decrement,
    Print,
}

// Define the actor
async fn counter(mut receiver: mpsc::Receiver<Message>) {
    let mut count = 0;

    while let Some(message) = receiver.recv().await {
        match message {
            Message::Increment => count += 1,
            Message::Decrement => count -= 1,
            Message::Print => println!("count = {}", count),
        }
    }
}

#[tokio::main]
async fn main() {
    let (mut sender, receiver) = mpsc::channel(10);

    task::spawn(counter(receiver));

    sender.send(Message::Increment).await.unwrap();
    sender.send(Message::Print).await.unwrap();
    sender.send(Message::Decrement).await.unwrap();
    sender.send(Message::Print).await.unwrap();
}
