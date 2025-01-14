use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct ValueRange {
    range: String,
    major_dimension: Option<String>, // Optional
    values: Vec<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Load environment variables
    let trello_key = env::var("TRELLO_KEY").expect("TRELLO_KEY not set");
    let trello_token = env::var("TRELLO_TOKEN").expect("TRELLO_TOKEN not set");
    let google_sheet_id = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"; // Google Sheet ID

    let client = Client::new();

    // Function to fetch cell data for a given range
    async fn fetch_cell_data(client: &Client, google_sheet_id: &str, cell_range: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sheets_url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?key={}",
            google_sheet_id, cell_range, env::var("GOOGLE_SHEETS_API_KEY").expect("GOOGLE_SHEETS_API_KEY not set")
        );

        let response = client.get(&sheets_url).send().await?;
        let value_range: ValueRange = response.json().await?;

        let data = value_range.values.iter().map(|row| row.get(0).unwrap_or(&String::from("")).clone()).collect();

        Ok(data)
    }

    // Fetch data from the specified cells
    let card_ids = fetch_cell_data(&client, google_sheet_id, "Epics!A2:A").await?;
    let r_data = fetch_cell_data(&client, google_sheet_id, "Epics!D2:D").await?;
    let s_data = fetch_cell_data(&client, google_sheet_id, "Epics!H2:H").await?;
    let w_data = fetch_cell_data(&client, google_sheet_id, "Epics!L2:L").await?;
    let x_data = fetch_cell_data(&client, google_sheet_id, "Epics!P2:P").await?;

    for (i, card_id) in card_ids.iter().enumerate() {
        if card_id.is_empty() {
            continue;
        }

        // Define the default empty string outside the closure
        let default_str = String::from("");

        let r_value = r_data.get(i).unwrap_or(&default_str).clone();
        let s_value = s_data.get(i).unwrap_or(&default_str).clone();
        let w_value = w_data.get(i).unwrap_or(&default_str).clone();
        let x_value = x_data.get(i).unwrap_or(&default_str).clone();

        if r_value.is_empty() && s_value.is_empty() && w_value.is_empty() && x_value.is_empty() {
            continue;
        }

        // Format the comment text
        let comment_text = format!(
            "User Stories in Review: {} ({})\nUser Stories Complete: {} ({})",
            s_value, r_value, w_value, x_value
        );

        // Fetch existing comments on the card
        let actions_url = format!(
            "https://api.trello.com/1/cards/{}/actions?filter=commentCard&key={}&token={}",
            card_id, trello_key, trello_token
        );

        let actions_response = client.get(&actions_url).send().await?;
        let actions: Vec<Action> = actions_response.json().await?;

        // Check if the comment already exists
        let existing_comment_id = actions.iter().find_map(|action| {
            if action.data.text.as_deref() == Some(&comment_text) {
                Some(action.id.clone())
            } else {
                None
            }
        });

        // Update or add the comment
        if let Some(comment_id) = existing_comment_id {
            // Update existing comment
            let update_url = format!(
                "https://api.trello.com/1/cards/{}/actions/{}?key={}&token={}&text={}",
                card_id, comment_id, trello_key, trello_token, urlencoding::encode(&comment_text)
            );

            let update_response = client.put(&update_url).send().await?;

            if update_response.status().is_success() {
                println!("Successfully updated the existing comment on Trello card ID: {}", card_id);
            } else {
                println!("Failed to update the existing comment on Trello card ID: {}. Status: {:?}", card_id, update_response.status());
            }
        } else {
            // Add a new comment
            let add_url = format!(
                "https://api.trello.com/1/cards/{}/actions/comments?key={}&token={}&text={}",
                card_id, trello_key, trello_token, urlencoding::encode(&comment_text)
            );

            let add_response = client.post(&add_url).send().await?;

            if add_response.status().is_success() {
                println!("Successfully added a comment to Trello card ID: {}", card_id);
            } else {
                println!("Failed to add a comment to Trello card ID: {}. Status: {:?}", card_id, add_response.status());
            }
        }
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Action {
    id: String,
    data: ActionData,
}

#[derive(Debug, Serialize, Deserialize)]
struct ActionData {
    text: Option<String>,
}
