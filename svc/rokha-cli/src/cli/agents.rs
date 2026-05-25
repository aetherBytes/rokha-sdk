use crate::api_client::RokhaClient;

pub async fn chat(client: &RokhaClient, message: &str) {
    match client.chat(message).await {
        Ok(response) => {
            println!("{}", response);
        }
        Err(e) => {
            eprintln!("Chat failed: {}", e);
        }
    }
}
