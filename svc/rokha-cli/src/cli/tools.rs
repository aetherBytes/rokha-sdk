use crate::api_client::RokhaClient;

/// `ro tools list [query]` — browse the live Rokha Registry. With no query it
/// lists the newest listings; with one it searches.
pub async fn list(client: &RokhaClient, query: &str) {
    match client.search_registry(query, 50).await {
        Ok(page) => {
            if page.items.is_empty() {
                println!("No listings found for '{query}'.");
                return;
            }
            println!(
                "{} of {} listing(s){}\n",
                page.items.len(),
                page.total_count,
                if query.is_empty() {
                    String::new()
                } else {
                    format!(" matching '{query}'")
                }
            );
            println!("{:<30} {:<10} {:<10} VERIFIED", "NAME", "CLASS", "PROVIDER");
            println!("{}", "-".repeat(70));
            for l in &page.items {
                println!(
                    "{:<30} {:<10} {:<10} {}",
                    truncate(l.name(), 30),
                    truncate(l.class(), 10),
                    truncate(l.provider(), 10),
                    if l.probe_verified.unwrap_or(false) {
                        "✓"
                    } else {
                        ""
                    }
                );
            }
        }
        Err(e) => eprintln!("Failed to search registry: {e}"),
    }
}

/// `ro tools info <name>` — show the top registry match for a name.
pub async fn info(client: &RokhaClient, name: &str) {
    match client.search_registry(name, 1).await {
        Ok(page) => match page.items.first() {
            Some(l) => {
                println!("Name:        {}", l.name());
                println!("Class:       {}", l.class());
                println!("Provider:    {}", l.provider());
                if let Some(v) = &l.version {
                    println!("Version:     {v}");
                }
                if let Some(a) = &l.author {
                    println!("Author:      {a}");
                }
                if !l.tags.is_empty() {
                    println!("Tags:        {}", l.tags.join(", "));
                }
                println!("Verified:    {}", l.probe_verified.unwrap_or(false));
                if let Some(d) = &l.description {
                    println!("\n{}", d.trim());
                }
            }
            None => println!("No listing found matching '{name}'."),
        },
        Err(e) => eprintln!("Failed to look up '{name}': {e}"),
    }
}

fn truncate(s: &str, max: usize) -> String {
    let count = s.chars().count();
    if count <= max {
        s.to_string()
    } else {
        format!(
            "{}…",
            s.chars().take(max.saturating_sub(1)).collect::<String>()
        )
    }
}
