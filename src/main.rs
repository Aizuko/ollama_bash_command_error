use std::env;
use std::io;
use std::io::Write;
use serde_json::Value;
use std::error::Error;
use bytes::BufMut;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let args: Vec<String> = env::args().collect();

    let mut response = client.post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "openhermes2.5-mistral:latest",
            //"model": "dolphin-mistral:latest",
            "prompt": format!("Reply rudely as an Australian man who swears about a user messing up a bash command by typing \"{}\" and offer an alternative", args[1]),
        }))
        .send()
        .await?
        .bytes_stream();

    let mut buffer = bytes::BytesMut::new();

    while let Some(item) = (response.next()).await {
        let chunk = item?;
        buffer.put(chunk);

        match serde_json::from_slice::<Value>(&buffer) {
            Ok(json) => {
                // Successfully parsed a JSON object
                print!("{}", json["response"].as_str().unwrap());
                io::stdout().flush()?;
                buffer.clear(); // Clear the buffer for the next JSON object
            }
            Err(e) if e.is_eof() => {
                // Incomplete JSON, wait for more data
                continue;
            }
            Err(e) => {
                // An actual error occurred
                eprintln!("Error parsing JSON: {}", e);
                break;
            }
        }
    }

    println!();

    Ok(())
}
