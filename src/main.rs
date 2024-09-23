use std::env;
use reqwest;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, serde::ts_seconds};

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Timestamp {
    #[serde(with = "ts_seconds")]
    timestamp: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    name: String,
    latitude: f64,
    longitude: f64,
    height: i32,
    #[serde(with = "ts_seconds")]
    modelrun_utc: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    modelrun_updatetime_utc: DateTime<Utc>
}

#[derive(Serialize,Deserialize, Debug)]
struct TimeSeries {
    #[serde(flatten)]
    time: Vec<Timestamp>
}

#[derive(Serialize,Deserialize, Debug)]
struct BasicPayload {
    metadata: Metadata,
    data_1h: TimeSeries
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = match env::var("API_KEY") {
        Ok(v) => v,
        Err(_e) => panic!("Darn!")
    };
    let url = format!("http://my.meteoblue.com/packages/basic-1h_basic-day?lat=47.558&lon=7.573&asl=279&tz=Europe%2FZurich&name=Basel&format=json&timeformat=timestamp_utc&apikey={}", api_key);
    match reqwest::get(url).await {
        Ok(resp) => {
            let json = resp.json::<BasicPayload>().await?;
            println!("{:?}", json)
        }
        Err(err) => {
            println!("Reqwest Error: {}", err)
        }
    }
    Ok(())
}
