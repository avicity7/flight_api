mod model;
use model::{Flight, FoundFlights, Hotel, FoundHotel};
use qstring::QString;
use actix_web::{get, web, App, HttpResponse, HttpServer, HttpRequest};
use chrono::{Utc, DateTime, NaiveDate};
use mongodb::{Client, options:: FindOneOptions};
use mongodb::bson::{doc};
use futures::stream::StreamExt;

#[get("/flight")]
async fn get_cheapest_flights(client: web::Data<Client>, req: HttpRequest) -> HttpResponse {

    let query_str = req.query_string();
    let qs = QString::from(query_str);
    let query_departure_date = qs.get("departureDate").unwrap();
    let query_return_date = qs.get("returnDate").unwrap();
    let query_destination = qs.get("destination").unwrap();

    let flights = client.database("minichallenge").collection::<Flight>("flights");
    let departure_dates = query_departure_date.split("-").collect::<Vec<&str>>();
    let departure_date = DateTime::<chrono::Utc>
        ::from_utc(NaiveDate::from_ymd_opt(departure_dates[0].parse().unwrap(), departure_dates[1].parse().unwrap(), departure_dates[2].parse().unwrap())
        .unwrap().and_hms_milli_opt(0, 0, 0, 000).unwrap(), Utc);
    let return_dates = query_return_date.split("-").collect::<Vec<&str>>();
    let return_date = DateTime::<chrono::Utc>
        ::from_utc(NaiveDate::from_ymd_opt(return_dates[0].parse().unwrap(), return_dates[1].parse().unwrap(), return_dates[2].parse().unwrap())
        .unwrap().and_hms_milli_opt(0, 0, 0, 000).unwrap(), Utc);

    match flights
        .find_one(
            doc! {
                "srcairportname": "Singapore Changi Airport",
                "destcity": query_destination,
                "date": departure_date
            },
            FindOneOptions::builder().sort(doc! { "price": 1 }).build(),
        )
        .await
    {
        Ok(Some(departure_flight)) => {
            let mut obj = FoundFlights::default();
            obj.city = departure_flight.destcity;
            obj.departure_date = departure_flight.date.format("%Y-%m-%d").to_string();
            obj.departure_airline = departure_flight.airlinename;
            obj.departure_price = departure_flight.price;
            match flights
                .find_one(
                    doc! {
                        "srccity": query_destination,
                        "destairportname": "Singapore Changi Airport",
                        "date": return_date
                    },
                    FindOneOptions::builder().sort(doc! { "price": 1 }).build(),
                )
                .await
            {
                Ok(Some(return_flight)) => {
                    obj.return_date = return_flight.date.format("%Y-%m-%d").to_string();
                    obj.return_airline = return_flight.airlinename;
                    obj.return_price = return_flight.price;

                    HttpResponse::Ok().json(obj)
                },
                Ok(None) => {
                    HttpResponse::NotFound().body(format!("No flights found"))
                }
                Err(_) => HttpResponse::NotFound().body(format!("No flights found"))
            }
        },
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No flights found"))
        }
        Err(_) => HttpResponse::NotFound().body(format!("No flights found"))
    }
}


#[get("/hotel")]
async fn get_cheapest_hotels(client: web::Data<Client>, req: HttpRequest) -> HttpResponse {
    let query_str = req.query_string();
    let qs = QString::from(query_str);
    let query_check_in_date = qs.get("checkInDate").unwrap();
    let query_check_out_date = qs.get("checkOutDate").unwrap();
    let query_destination = qs.get("destination").unwrap();

    let hotels = client.database("minichallenge").collection::<Hotel>("hotels");
    let check_in_dates = query_check_in_date.split("-").collect::<Vec<&str>>();
    let check_in_date = DateTime::<chrono::Utc>
        ::from_utc(NaiveDate::from_ymd_opt(check_in_dates[0].parse().unwrap(), check_in_dates[1].parse().unwrap(), check_in_dates[2].parse().unwrap())
        .unwrap().and_hms_milli_opt(0, 0, 0, 000).unwrap(), Utc);
    let check_out_dates = query_check_out_date.split("-").collect::<Vec<&str>>();
    let check_out_date = DateTime::<chrono::Utc>
        ::from_utc(NaiveDate::from_ymd_opt(check_out_dates[0].parse().unwrap(), check_out_dates[1].parse().unwrap(), check_out_dates[2].parse().unwrap())
        .unwrap().and_hms_milli_opt(0, 0, 0, 000).unwrap(), Utc);

    let pipeline = vec![
        doc! {
            "$match": {
                "city" : query_destination
            }
        },
        doc! {
            "$match": {
                "date" : {
                    "$gte": check_in_date,
                    "$lte": check_out_date
                }
            }
        },
        doc! {
            "$group": {
                "_id": "$hotelName",
                "hotelName": {"$first": "$hotelName"},
                "price": {"$sum": "$price"}
            }
        },
        doc! {
            "$sort": {
                "price": 1
            }
        }
    ]; 

    let mut cursor = hotels.aggregate(pipeline, None).await.unwrap();
    let mut results: Vec<FoundHotel> = Vec::new();

    while let Some(hotel) = cursor.next().await {
        let hotel_reference = hotel.unwrap();
        // let destination_city = hotel_reference.get("city").unwrap();
        let name = hotel_reference.get("hotelName").unwrap();
        let hotel_price= hotel_reference.get_i32("price").unwrap();
        let found_hotel = FoundHotel {
            city: query_destination.to_string(),
            check_in_date: query_check_in_date.to_string(),
            check_out_date: query_check_out_date.to_string(),
            hotel: name.to_string().replace(&['"', '\"'], ""), 
            price: hotel_price
        };

        results.push(found_hotel);
    }

    if results.len() != 0 {
        HttpResponse::Ok().json(&results[0])
    } else {
        HttpResponse::NotFound().body(format!("No hotels found"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_uri = "mongodb+srv://userReadOnly:7ZT817O8ejDfhnBM@minichallenge.q4nve1r.mongodb.net/";
    let client = Client::with_uri_str(client_uri).await.expect("failed to connect");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(get_cheapest_flights)
            .service(get_cheapest_hotels)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
