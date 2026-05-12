use borsh::{BorshDeserialize, BorshSerialize};
use lapin::Connection;

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to RabbitMQ
    let conn = Connection::connect(
        "amqp://guest:guest@localhost:5672",
        lapin::ConnectionProperties::default(),
    )
    .await?;

    let channel = conn.create_channel().await?;

    // Declare exchange and queue
    let _exchange = channel
        .exchange_declare(
            "user_events",
            lapin::ExchangeKind::Topic,
            lapin::options::ExchangeDeclareOptions::default(),
            Default::default(),
        )
        .await?;

    // Publish messages
    let messages = vec![
        UserCreatedEventMessage {
            user_id: "1".to_owned(),
            user_name: "2406362646-Amir".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "2".to_owned(),
            user_name: "2406362646-Budi".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "3".to_owned(),
            user_name: "2406362646-Cica".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "4".to_owned(),
            user_name: "2406362646-Dira".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "5".to_owned(),
            user_name: "2406362646-Emir".to_owned(),
        },
    ];

    for msg in messages {
        let payload = borsh::to_vec(&msg)?;
        channel
            .basic_publish(
                "",
                "user_created",
                lapin::options::BasicPublishOptions::default(),
                &payload,
                Default::default(),
            )
            .await?
            .await?;
        println!("Published message: {:?}", msg);
    }

    println!("All messages published!");
    Ok(())
}