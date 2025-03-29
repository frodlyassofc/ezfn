pub mod ezfn {
    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use std::error::Error;

    #[derive(Serialize, Deserialize)]
    pub struct UnlockRequest {
        pub item_ids: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct UnlockResponse {
        pub success: bool,
        pub message: String,
    }

    pub struct Ezfn {
        client: Client,
    }

    impl Ezfn {
        pub fn new() -> Self {
            let client = Client::new();
            Ezfn { client }
        }

        pub async fn unlock_items(&self, item_ids: Vec<String>) -> Result<UnlockResponse, Box<dyn Error>> {
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
}