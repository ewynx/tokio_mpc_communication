mod actor;

use actor::{Actor, Message};
use tokio::sync::mpsc;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let mut senders = HashMap::new();
    let mut receivers = HashMap::new();

    // Create channels for 5 actors
    for id in 0..5 {
        let (sender, receiver) = mpsc::channel(32);
        senders.insert(id, sender);
        receivers.insert(id, receiver);
    }

    // Spawn actors
    for id in 0..5 {
        let receiver = receivers.remove(&id).unwrap();
        let peers = senders.clone();
        let actor = Actor::new(id, receiver, peers);

        tokio::spawn(async move {
            actor.run().await;
        });
    }

    // Send integers between actors
    for (sender_id, sender) in senders.iter() {
        for receiver_id in 0..5 {
            if *sender_id != receiver_id {
                sender.send(Message::SendInt(receiver_id, 10)).await.unwrap(); // Example: sending 10
            }
        }
    }

    // Delay to allow actors to process messages
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}
