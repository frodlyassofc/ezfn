use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct UnlockRequest {
    item_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct UnlockResponse {
    success: bool,
    message: String,
}

struct Ezfn {
    client: Client,
}

impl Ezfn {
    fn new() -> Self {
        let client = Client::new();
        Ezfn { client }
    }

    async fn unlock_items(&self, item_ids: Vec<String>) -> Result<UnlockResponse, Box<dyn Error>> {
        let request = UnlockRequest { item_ids };
        let response = self.client.post("https://api.fortnite.com/unlock")
            .json(&request)
            .send()
            .await?
            .json::<UnlockResponse>()
            .await?;
        Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ezfn = Ezfn::new();
    let items_to_unlock = vec![
        "outfit1".to_string(),
        "emote1".to_string(),
        "pickaxe1".to_string(),
    ];
    let response = ezfn.unlock_items(items_to_unlock).await?;
    if response.success {
        println!("Items unlocked successfully: {}", response.message);
    } else {
        println!("Failed to unlock items: {}", response.message);
    }
    Ok(())
}