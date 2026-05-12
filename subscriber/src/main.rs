use borsh::{BorshDeserialize, BorshSerialize};
use lapin::{
    Connection, ConnectionProperties,
};
use futures_util::StreamExt;
use std::time::Duration;

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to RabbitMQ
    let conn = Connection::connect(
        "amqp://guest:guest@localhost:5672",
        ConnectionProperties::default(),
    )
    .await?;

    let channel = conn.create_channel().await?;
    
    // Declare queue
    let queue = channel
        .queue_declare(
            "user_created",
            lapin::options::QueueDeclareOptions::default(),
            Default::default(),
        )
        .await?;

    println!("Declared queue: {:?}", queue);

    // Create consumer
    let mut consumer = channel
        .basic_consume(
            "user_created",
            "subscriber",
            lapin::options::BasicConsumeOptions::default(),
            Default::default(),
        )
        .await?;

    println!("Started consuming messages...");

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery?;
        
        match UserCreatedEventMessage::try_from_slice(&delivery.data) {
            Ok(message) => {
                println!("In Wasis's Computer [2406362646]. Message received: {:?}", message);
                let ten_millis = Duration::from_millis(1000);
                tokio::time::sleep(ten_millis).await;
            }
            Err(e) => {
                println!("Failed to deserialize message: {:?}", e);
            }
        }
        
        delivery.ack(lapin::options::BasicAckOptions::default()).await?;
    }

    Ok(())
}