use serde::{Deserialize, Serialize};
use mongodb::bson::{doc, oid::ObjectId};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Flight {
    pub _id: Option<ObjectId>,
    pub airline: String,
    pub airlineid: i32,
    pub srcairport: String,
    pub srcairportid: i32,
    pub destairport: String,
    pub destairportid: i32,
    pub codeshare: String,
    pub stop: i32,
    pub eq: String,
    pub airlinename: String,
    pub srcairportname: String,
    pub srccity: String,
    pub srccountry: String,
    pub destairportname: String,
    pub destcity: String,
    pub destcountry: String,
    pub price: i32,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub date: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Hotel {
    pub _id: Option<ObjectId>,
    pub city: String,
    pub hotel_name: String,
    pub price: i32,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub date: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FoundFlights {
  pub city: String, 
  pub departure_date: String, 
  pub departure_airline: String, 
  pub departure_price: i32, 
  pub return_date: String, 
  pub return_airline: String, 
  pub return_price: i32
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FoundHotel {
    pub city: String,
    pub check_in_date: String, 
    pub check_out_date: String,
    pub hotel: String,
    pub price: i32,
}