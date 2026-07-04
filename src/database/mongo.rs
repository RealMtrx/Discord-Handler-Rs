use mongodb::{Client, Database};
use std::time::Duration;
use tokio::net::lookup_host;
use tokio::time::timeout;

pub async fn connect(uri: &str) -> Option<Database> {
    if uri.is_empty() || uri == "#" {
        println!("  ❌ MongoDB: No URI provided");
        return None;
    }

    match timeout(Duration::from_secs(10), async {
        let client = Client::with_uri_str(uri).await.map_err(|e| e.to_string())?;

        let db = client.database("discord_bot");

        client
            .database("discord_bot")
            .run_command(mongodb::bson::doc! { "ping": 1 })
            .await
            .map_err(|e| e.to_string())?;

        Ok::<Database, String>(db)
    })
    .await
    {
        Ok(Ok(db)) => {
            println!("  ✅ MongoDB: Connected successfully");
            Some(db)
        }
        Ok(Err(e)) => {
            println!("  ❌ MongoDB: Connection failed: {}", e);
            None
        }
        Err(_) => {
            println!("  ❌ MongoDB: Connection timed out");
            None
        }
    }
}

pub async fn disconnect() {
    println!("  ℹ️ MongoDB: Disconnected");
}
