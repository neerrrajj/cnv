use std::{env, error::Error, fs, path::Path};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
  dotenv()?;
  let currency_api_key = env::var("CURRENCY_API_KEY")?;
  let home_dir = env::var("HOME")?;

  let url = format!("https://api.currencyapi.com/v3/latest?apikey={}", currency_api_key);
  let response = reqwest::get(url).await?;
  let json_response = response.text().await?;

  let file_path = format!("{}/currency_rates.json", home_dir);
  if let Some(parent) = Path::new(&file_path).parent() {
    fs::create_dir_all(parent)?
  }

  fs::write(file_path, json_response)?;
  println!("Currency rates updated!");

  Ok(())
}