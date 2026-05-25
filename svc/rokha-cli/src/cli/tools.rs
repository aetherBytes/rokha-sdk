use crate::api_client::RokhaClient;

pub async fn list(client: &RokhaClient) {
    match client.list_tools().await {
        Ok(tools) => {
            if tools.is_empty() {
                println!("No tools found.");
                return;
            }
            println!("{:<24} {:<12} {}", "NAME", "CATEGORY", "DESCRIPTION");
            println!("{}", "-".repeat(60));
            for tool in &tools {
                println!("{:<24} {:<12} {}", tool.name, tool.category, tool.description);
            }
        }
        Err(e) => {
            eprintln!("Failed to list tools: {}", e);
        }
    }
}

pub async fn info(client: &RokhaClient, name: &str) {
    match client.get_tool_info(name).await {
        Ok(tool) => {
            println!("Name:        {}", tool.name);
            println!("Category:    {}", tool.category);
            println!("Description: {}", tool.description);
        }
        Err(e) => {
            eprintln!("Failed to get tool info: {}", e);
        }
    }
}
