use crate::router::handlers::handler_trait::Handler;
use chrono::{DateTime, Utc};
use http::{
    request::{Request, Resource},
    response::Response,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashMap, env, fs};
use tracing::{instrument, trace};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Status {
    Delivered,
    Pending,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    id: i32,
    #[serde(
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    date: DateTime<Utc>,
    status: Status,
}

fn serialize_date<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.to_rfc3339())
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let date = String::deserialize(deserializer)?;
    let date = DateTime::parse_from_rfc3339(&date)
        .map_err(|err| serde::de::Error::custom(format!("invalid date format: {}", err)))?;
    Ok(date.with_timezone(&Utc))
}

pub struct WebServiceHandler;

impl WebServiceHandler {
    #[instrument]
    fn load_json() -> Vec<Order> {
        let data_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(data_path);
        let orders_path = format!("{}/{}", data_path, "orders.json");
        let contents = fs::read_to_string(orders_path).unwrap();
        let orders: Vec<Order> = serde_json::from_str(contents.as_str()).unwrap();
        trace!(orders = ?orders);
        orders
    }
}

impl Handler for WebServiceHandler {
    fn handle(req: &Request) -> Response {
        let Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split('/').collect();
        if route[2] == "orders" {
            let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
            let mut headers: HashMap<&str, &str> = HashMap::new();
            headers.insert("Content-Type", "application/json");
            Response::new(200, Some(headers), body)
        } else {
            Response::new(404, None, Self::load_file("404.html"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_order_serde() {
        let order = Order {
            id: 1,
            date: Utc.with_ymd_and_hms(2023, 5, 1, 16, 15, 0).unwrap(),
            status: Status::Pending,
        };
        let json = serde_json::to_string(&order).unwrap();
        let deserialized_order: Order = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized_order.id, order.id);
        assert_eq!(deserialized_order.date, order.date);
        assert_eq!(deserialized_order.status, order.status);
    }
}
