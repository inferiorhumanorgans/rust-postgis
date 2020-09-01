#[macro_use] extern crate diesel;

use std::env;
use std::error::Error;
use std::result::Result;

use diesel::{prelude::*, PgConnection};

use postgis::ewkb::{LineString, Point};

fn main() -> Result<(), Box<dyn Error>> {
    // The schema module is typically generated automatically by diesel
    pub mod schema {
        table! {
            // If you're using an auto-generated schema these need to be
            // specified in diesel.toml
            use diesel::sql_types::Varchar;
            use postgis::diesel_shim::PostgisGeometry;
            
            busline (name) {
                // diesel needs tables to have a primary key
                name -> Varchar,
                route -> PostgisGeometry,
            }
        }
        
        table! {
            use diesel::sql_types::Varchar;
            use postgis::diesel_shim::PostgisGeometry;

            stops (name) {
                name -> Varchar,
                stop -> PostgisGeometry,
            }
        }
    }
    use schema::*;

    #[derive(Queryable, QueryableByName, Debug, PartialEq)]
    #[table_name = "busline"]
    struct BusLine {
        pub name: String,
        pub route: LineString,
    }

    #[derive(Insertable, Queryable, QueryableByName, Debug, PartialEq)]
    #[table_name = "stops"]
    struct Stop {
        pub name: String,
        pub stop: Point,
    }

    let database_url = env::var("DATABASE_URL")?;
    let connection = PgConnection::establish(&database_url)?;

    let results = busline::table
        .load::<BusLine>(&connection)?;

    let stops : Vec<Stop> = results.iter().enumerate().map(|(i, busline)| {
        let BusLine { route, name } = busline;

        Stop {
            name: format!("{}{:03}", name, i),
            stop: *route.points.last().unwrap(),
        }
    }).collect();

    diesel::insert_into(stops::table)
        .values(&stops)
        .execute(&connection)?;

    Ok(())
}
