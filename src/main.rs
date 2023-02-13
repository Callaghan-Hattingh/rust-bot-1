use serde::{Serialize, Deserialize};
use chrono;
use hex;
use hmac::{Hmac, Mac};
use sha2::Sha512;


#[derive(Debug, Serialize, Deserialize)]
struct OrderBook {
    orders:Vec<Ask>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ask {
    side:String,
    quantity: f64,
    price: u32,
    #[serde(rename = "currencyPair")]
    currency_pair: String,
    #[serde(rename = "orderCount")]
    order_count: u32
}


#[derive(Debug, Serialize, Deserialize)]
struct ServerTime {
    #[serde(rename = "epochTime")]
    epoch_time: u64,
    time: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Receive type-checked JSON
    let t4 = reqwest::Client::new()
        .get("https://api.valr.com/v1/public/time")
        .send()
        .await?
        .json::<ServerTime>()
        .await?;

    println!("{:#?}", t4);


    let time: i64 = chrono::offset::Utc::now().timestamp_millis(); // get current time
    // let time: u64 = 1661139891463; // get current time
    let verb = String::from("GET");
    let path = String::from("/v1/orders/open");
    let body = String::from("");
    let secret = String::from("5a94dc2ae51fc0efce5476859100f202bf2c77c74829873d758c5c9c9f652a85"); // get secret from env

    let sign = signaure(time, &verb, &path, &body, &secret);

    println!("{sign}");


    Ok(())
}


fn signaure(t: i64, v: &String, p: &String, b: &String, k: &String) -> String {
    type HmacSha512 = Hmac<Sha512>;

    let payload = format!("{}{}{}{}", t, v, p, b);
    let mut mac = HmacSha512::new_from_slice(k.as_bytes()).expect("");
    mac.update(payload.as_bytes());
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    let resp = hex::encode(code_bytes);

    return resp;
}