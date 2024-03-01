pub struct Item {
    name: String,
    state: String,
}

impl Item {
    pub fn new(name: &str, state: &str) -> Self {
        Item {
            name: name.to_string(),
            state: state.to_string(),
        }
    }

    pub async fn trigger(&self) {
        let client = reqwest::Client::new();
        let _ = client
            .post(format!("http://192.168.1.12:8080/rest/items/{}", self.name))
            .header("Content-Type", "text/plain")
            .header("Accept", "application/json")
            .body(self.state.clone())
            .send()
            .await
            .expect("Failed to send command to OpenHAB");
    }
}
