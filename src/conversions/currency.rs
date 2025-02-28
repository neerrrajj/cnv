use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::PathBuf;

const API_URL: &str = "https://api.neerrrajj.me/currency_rates.json";

#[derive(Debug, Deserialize, Serialize)]
struct ExchangeRates {
    meta: Meta,
    data: HashMap<String, CurrencyData>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Meta {
    #[serde(rename = "last_updated_at")]
    last_updated: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CurrencyData {
    code: String,
    value: f64,
}

pub fn convert(value: f64, from: &str, to: &str) -> Result<(f64, String), &'static str> {
    let rates = get_exchange_rates()?;

    let from_code = resolve_currency_code(from, &rates.data)?;
    let to_code = resolve_currency_code(to, &rates.data)?;

    let from_rate = rates
        .data
        .get(&from_code)
        .ok_or("Invalid 'from' currency")?
        .value;
    let to_rate = rates
        .data
        .get(&to_code)
        .ok_or("Invalid 'to' currency")?
        .value;

    let result = value * (to_rate / from_rate);
    let formatted_date = format_date(&rates.meta.last_updated)?;

    Ok(((result * 10000.0).round() / 10000.0, formatted_date))
}

fn format_date(datetime_str: &str) -> Result<String, &'static str> {
    let dt = DateTime::parse_from_rfc3339(datetime_str)
        .map_err(|_| "Invalid date format in response")?;
    Ok(dt.format("%d %b %H:%M UTC").to_string())
}

fn resolve_currency_code(
    input: &str,
    data: &HashMap<String, CurrencyData>,
) -> Result<String, &'static str> {
    if data.contains_key(input.to_uppercase().as_str()) {
        return Ok(input.to_uppercase());
    }

    Err("Invalid currency. Use --list to see available options.")
}

pub fn help_text() -> String {
    match get_exchange_rates() {
        Ok(rates) => {
            let mut help = String::from("Supported currencies (Code - Country):\n");
            for currency in rates.data.values() {
                help.push_str(&format!("- {}\n", currency.code));
            }
            help
        }
        Err(e) => format!("Error loading currencies: {}", e),
    }
}

fn get_exchange_rates() -> Result<ExchangeRates, &'static str> {
    let cache_path = get_cache_path()?;
    let api_url = API_URL;

    if let Ok(cached_rates) = load_cached_rates(&cache_path) {
        if is_cache_current(&cached_rates) {
            return Ok(cached_rates);
        }
    }

    let new_rates = fetch_rates(api_url)?;
    save_rates(&new_rates, &cache_path)?;
    Ok(new_rates)
}

fn load_cached_rates(path: &PathBuf) -> Result<ExchangeRates, &'static str> {
    let file = File::open(path).map_err(|_| "Failed to open cache")?;
    serde_json::from_reader(file).map_err(|_| "Invalid cache format")
}

fn is_cache_current(rates: &ExchangeRates) -> bool {
    match DateTime::parse_from_rfc3339(&rates.meta.last_updated) {
        Ok(dt) => dt.date_naive() == Utc::now().date_naive(),
        Err(_) => false,
    }
}

fn fetch_rates(url: &str) -> Result<ExchangeRates, &'static str> {
    let response = get(url).map_err(|_| "Network error")?;
    let text = response
        .text()
        .map_err(|_| "Failed to read response body")?;
    serde_json::from_str(&text).map_err(|_| "Invalid API response format")
}

fn save_rates(rates: &ExchangeRates, path: &PathBuf) -> Result<(), &'static str> {
    let file = File::create(path).map_err(|_| "Failed to create cache")?;
    serde_json::to_writer(file, rates).map_err(|_| "Failed to write cache")
}

fn get_cache_path() -> Result<PathBuf, &'static str> {
    let dirs = ProjectDirs::from("", "", "cnv").ok_or("System not supported")?;
    let cache_dir = dirs.cache_dir();
    fs::create_dir_all(cache_dir).map_err(|_| "Can't create cache dir")?;
    Ok(cache_dir.join("exchange_rates.json"))
}
