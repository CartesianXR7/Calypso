use reqwest::Client;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
struct ValueRange {
    values: Option<Vec<Vec<String>>>,  
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Load environment variables
    let google_sheets_api_key = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"; //Enter Sheets API Here
    let google_sheet_id = "YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY"; //Enter Sheet ID
    let slack_token = env::var("SLACK_TOKEN").expect("SLACK_TOKEN not set");
    let slack_channel = "XXXXXXXXXXX"; //Slack Channel ID

    let client = Client::new();

    // Manually trigger the Slack post function
    fetch_and_post_to_slack(&client, google_sheet_id, google_sheets_api_key, &slack_token, slack_channel).await?;

    Ok(())
}

async fn fetch_and_post_to_slack(
    client: &Client,
    google_sheet_id: &str,
    google_sheets_api_key: &str,
    slack_token: &str,
    slack_channel: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch values from Google Sheets to determine where to post (Use Checkboxes!)
    let control_a1 = fetch_cell_data(client, google_sheet_id, google_sheets_api_key, "TABNAME!A1").await?;
    let control_b1 = fetch_cell_data(client, google_sheet_id, google_sheets_api_key, "TABNAME!A2").await?;
    let _control_c1 = fetch_cell_data(client, google_sheet_id, google_sheets_api_key, "TABNAME!A3").await?;

    // If A1 is "TRUE"
    if control_a1.to_uppercase() == "TRUE" {
        let text1 = fetch_cell_data(client, google_sheet_id, google_sheets_api_key, "TABNAME!B1").await?;
        let text2 = fetch_cell_data(client, google_sheet_id, google_sheets_api_key, "TABNAME!B2").await?;
        let text3 = fetch_cell_data(client, google_sheet_id, google_sheets_api_key, "TABNAME!B3").await?;
        let text4 = fetch_cell_data(client, google_sheet_id, google_sheets_api_key, "TABNAME!B4").await?;
        let text5 = fetch_cell_data(client, google_sheet_id, google_sheets_api_key, "TABNAME!B5").await?;

        let message = format!("{}{}{}{}{}", text1, text2, text3, text4, text5);
        post_to_slack(client, slack_token, slack_channel, &message).await?;
    }

    // Check if A2 is "TRUE"
    if control_b1.to_uppercase() == "TRUE" {
        let text6 = fetch_cell_data(client, google_sheet_id, google_sheets_api_key, "TABNAME!C1").await?;
        let text7 = fetch_cell_data(client, google_sheet_id, google_sheets_api_key, "TABNAME!C2").await?;
        let text8 = fetch_cell_data(client, google_sheet_id, google_sheets_api_key, "TABNAME!C3").await?;
        let text9 = fetch_cell_data(client, google_sheet_id, google_sheets_api_key, "TABNAME!C4").await?;
        let text10 = fetch_cell_data(client, google_sheet_id, google_sheets_api_key, "TABNAME!C5").await?;

        let message2 = format!("{}{}{}{}{}", text6, text7, text8, text9, text10);
        let new_slack_channel = "XXXXXXXXXXX"; // Enter channel ID for this condition
        post_to_slack(client, slack_token, new_slack_channel, &message2).await?;
    }

    // If both A1 & A2 are "FALSE", post to this channel
    if control_a1.to_uppercase() != "TRUE" && control_b1.to_uppercase() != "TRUE" {
        let new_slack_channel = "XXXXXXXXXXX"; // Channel ID for posting message
        let message3 = "Please choose a report destination";
        post_to_slack(client, slack_token, new_slack_channel, message3).await?;
    }

    Ok(())
}

async fn fetch_cell_data(
    client: &Client,
    sheet_id: &str,
    api_key: &str,
    range: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?key={}",
        sheet_id, range, api_key
    );

    let response = client.get(&url).send().await?;

    // Check if the response was successful
    if response.status().is_success() {
        let value_range: ValueRange = response.json().await?;
        // Retrieve all text in the cell and handle line breaks without trimming
        let result = value_range.values.and_then(|v| {
            v.get(0).map(|row| {
                row.get(0).map(|cell| {
                    // Replace line breaks (CHAR(10)) with newline characters for Slack
                    cell.replace("<a href=\"", "")
                        .replace("\">", " ")
                        .replace("</a>", "")
                        .replace("&nbsp;", " ")
                        .replace("\r\n", "\n")  // Normalize Windows-style line breaks
                        .replace("\r", "\n")    // Normalize old Mac-style line breaks
                        .replace("\n", "\n")    // Preserve newline characters for Slack
                        .to_string()
                }).unwrap_or_else(|| "".to_string())
            })
        }).unwrap_or_else(|| "".to_string());
        Ok(result)
    } else {
        // Log the error for better debugging
        let error_text = response.text().await?;
        eprintln!("Failed to fetch data from Google Sheets: {:?}", error_text);
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to fetch data from Google Sheets")))
    }
}

async fn post_to_slack(
    client: &Client,
    token: &str,
    channel: &str,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let slack_url = "https://slack.com/api/chat.postMessage";
    let payload = [("channel", channel), ("text", message)];

    let response = client.post(slack_url)
        .bearer_auth(token)
        .form(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Message posted to Slack successfully.");
    } else {
        eprintln!("Failed to post message to Slack. Status: {:?}", response.status());
    }

    Ok(())
}

// Email sending function (commented out)
/*
async fn send_email(
    header: &str,
    sub_header: &str,
    body1: &str,
    body2: &str,
    footer: &str,
    smtp_user: &str,
    smtp_password: &str,
    smtp_server: &str,
    recipient: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Implement email sending using lettre crate
    Ok(())
}
*/