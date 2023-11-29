use tokio::sync::mpsc::{Receiver, Sender};
use std::collections::{HashMap, HashSet};

// Define the Actor struct
pub struct Actor {
    pub id: usize, // Unique identifier for the actor
    receiver: Receiver<Message>, // Channel receiver to receive messages
    peers: HashMap<usize, Sender<Message>>, // Map of peer IDs to their message senders
    received_shares: HashMap<usize, i32>, // Stores shares received from peers
    expected_peers: HashSet<usize>, // Set of peer IDs from whom messages are expected
}

// Define the types of messages that can be sent between actors
pub enum Message {
    SendInt(usize, i32), // Message to send an integer to another actor (recipient_id, value)
    ReceiveInt(usize, i32), // Message indicating an integer is received (sender_id, value)
}

impl Actor {
    // Constructor for a new Actor
    pub fn new(id: usize, receiver: Receiver<Message>, peers: HashMap<usize, Sender<Message>>) -> Self {
        println!("\x1b[34mActor created id {}, nr of peers {}\x1b[0m", id, peers.len());
        // Create a set of expected peers, excluding self
        let expected_peers: HashSet<usize> = peers.keys().cloned().filter(|&k| k != id).collect();
        Self { id, receiver, peers, received_shares: HashMap::new(), expected_peers }
    }

    // Asynchronous function for the actor to process incoming messages
    pub async fn run(mut self) {
        while let Some(message) = self.receiver.recv().await {
            match message {
                // Handle SendInt message: send a ReceiveInt message to the specified peer
                Message::SendInt(recipient_id, value) => {
                    println!("SendInt Self id {}, Recipient id {}, Value: {}", self.id, recipient_id, value);
                    if let Some(peer) = self.peers.get(&recipient_id) {
                        let _ = peer.send(Message::ReceiveInt(self.id, value)).await;
                    }
                },
                // Handle ReceiveInt message: store the received value and check if all shares are received
                Message::ReceiveInt(sender_id, value) => {
                    println!("ReceiveInt Self id {}, Recipient id {}, Value: {}", self.id, sender_id, value);
                    self.received_shares.insert(sender_id, value);
                    // Check if shares from all expected peers are received
                    if self.received_shares.len() == self.expected_peers.len() {
                        let sum: i32 = self.received_shares.values().sum();
                        println!("\x1b[32mActor {} received all shares. Sum: {}\x1b[0m", self.id, sum);
                      }
                }
            }
        }
    }
}
