// {
//     "results": [
//      {
//       "ticker": "A",
//       "name": "Agilent Technologies Inc.",
//       "market": "stocks",
//       "locale": "us",
//       "primary_exchange": "XNYS",
//       "type": "CS",
//       "active": true,
//       "currency_name": "usd",
//       "cik": "0001090872",
//       "composite_figi": "BBG000BWQYZ5",
//       "share_class_figi": "BBG001SCTQY4",
//       "last_updated_utc": "2021-04-25T00:00:00Z"
//      }
//     ],
//     "status": "OK",
//     "request_id": "e70013d92930de90e089dc8fa098888e",
//     "count": 1,
//     "next_url": "https://api.polygon.io/vX/reference/tickers?cursor=YWN0aXZlPXRydWUmZGF0ZT0yMDIxLTA0LTI1JmxpbWl0PTEmb3JkZXI9YXNjJnBhZ2VfbWFya2VyPUElN0M5YWRjMjY0ZTgyM2E1ZjBiOGUyNDc5YmZiOGE1YmYwNDVkYzU0YjgwMDcyMWE2YmI1ZjBjMjQwMjU4MjFmNGZiJnNvcnQ9dGlja2Vy"
//    }


// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }
//
// #[get("/stock")]
// async fn check() ->  Result<String, Box<std::error::Error>> {
//
//     let api_key = "";
//
//     // let request_url = format!("{}", user);
//     let request_url = "https://api.polygon.io/v3/reference/tickers?ticker=VOO&market=stocks&active=true&sort=ticker&order=asc&limit=10&apiKey=pGl8Up9MF6RzMrGn3TPhFzFJvce8iQTv";
//
//
//     let json: Root = reqwest::get(request_url).await?.json()?;
//
//     println!("{:#?}", json);
// }
use vestor::configuration::get_configuration;
use vestor::startup::Application;
// use vestor::telemetry::{get_subscriber, init_subscriber};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    println!("Vestor Started!");
    Ok(())
}

//
//
// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Root {
//     pub results: Vec<StockList>,
//     pub status: String,
//     #[serde(rename = "request_id")]
//     pub request_id: String,
//     pub count: i64,
//     #[serde(rename = "next_url")]
//     pub next_url: String,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct StockList {
//     pub ticker: String,
//     pub name: String,
//     pub market: String,
//     pub locale: String,
//     #[serde(rename = "primary_exchange")]
//     pub primary_exchange: String,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub active: bool,
//     #[serde(rename = "currency_name")]
//     pub currency_name: String,
//     pub cik: String,
//     #[serde(rename = "composite_figi")]
//     pub composite_figi: String,
//     #[serde(rename = "share_class_figi")]
//     pub share_class_figi: String,
//     #[serde(rename = "last_updated_utc")]
//     pub last_updated_utc: String,
// }
