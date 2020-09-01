use dotenv::dotenv;
use postgres::{Client, NoTls};
use std::env;
use std::error::Error;
use std::result::Result;

fn db_setup() -> Result<(), Box<dyn Error>> {
    const INITIAL_DB_STATE: &str = r##"
    -- DB init
    BEGIN;

    -- Create table for Geometry type data
    DROP TABLE IF EXISTS geometry_test;
    CREATE TABLE geometry_test (name VARCHAR PRIMARY KEY, obj GEOMETRY NOT NULL);
    INSERT INTO geometry_test VALUES ('point',                  'SRID=4326;POINT(-126.4 45.32)');
    INSERT INTO geometry_test VALUES ('multipoint',             'SRID=4326;MULTIPOINT((-126.4 45.32), (0 0))');
    INSERT INTO geometry_test VALUES ('linestring',             'SRID=4326;LINESTRING (30 10, 10 30, 40 40)');
    INSERT INTO geometry_test VALUES ('multilinestring',        'SRID=4326;MULTILINESTRING ((10 10, 20 20, 10 40), (40 40, 30 30, 40 20, 30 10))');
    INSERT INTO geometry_test VALUES ('polygon',                'SRID=4326;POLYGON ((30 10, 40 40, 20 40, 10 20, 30 10))');
    INSERT INTO geometry_test VALUES ('multipolygon',           'SRID=4326;MULTIPOLYGON (((40 40, 20 45, 45 30, 40 40)), ((20 35, 10 30, 10 10, 30 5, 45 20, 20 35), (30 20, 20 15, 20 25, 30 20)))');
    INSERT INTO geometry_test VALUES ('geometry_collection',    'SRID=4326;GEOMETRYCOLLECTION (POINT (40 10), LINESTRING (10 10, 20 20, 10 40), POLYGON ((40 40, 20 45, 45 30, 40 40)))');

    -- Create table for Geography type data
    DROP TABLE IF EXISTS geography_test;
    CREATE TABLE geography_test (name VARCHAR PRIMARY KEY, obj GEOMETRY NOT NULL);
    INSERT INTO geography_test VALUES ('point',                 'SRID=4326;POINT(-126.4 45.32)');
    INSERT INTO geography_test VALUES ('multipoint',            'SRID=4326;MULTIPOINT((-126.4 45.32), (0 0))');
    INSERT INTO geography_test VALUES ('linestring',            'SRID=4326;LINESTRING (30 10, 10 30, 40 40)');
    INSERT INTO geography_test VALUES ('multilinestring',       'SRID=4326;MULTILINESTRING ((10 10, 20 20, 10 40), (40 40, 30 30, 40 20, 30 10))');
    INSERT INTO geography_test VALUES ('polygon',               'SRID=4326;POLYGON ((30 10, 40 40, 20 40, 10 20, 30 10))');
    INSERT INTO geography_test VALUES ('multipolygon',          'SRID=4326;MULTIPOLYGON (((40 40, 20 45, 45 30, 40 40)), ((20 35, 10 30, 10 10, 30 5, 45 20, 20 35), (30 20, 20 15, 20 25, 30 20)))');
    INSERT INTO geography_test VALUES ('geometry_collection',   'SRID=4326;GEOMETRYCOLLECTION (POINT (40 10), LINESTRING (10 10, 20 20, 10 40), POLYGON ((40 40, 20 45, 45 30, 40 40)))');

    COMMIT;
    "##;

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut client = Client::connect(&database_url, NoTls)?;

    client.batch_execute(INITIAL_DB_STATE)?;
    Ok(())
}

#[test]
fn read_geometry() -> Result<(), Box<dyn Error>> {
    use super::PostgisGeometry;
    use crate::ewkb::{Geometry, LineString, Point, Polygon};
    use diesel::prelude::*;
    use diesel::sql_types::*;
    use diesel::{Connection, PgConnection};

    #[derive(Queryable, QueryableByName, Debug, PartialEq)]
    struct NamedGeometry {
        #[sql_type = "Varchar"]
        pub name: String,
        #[sql_type = "PostgisGeometry"]
        pub obj: Geometry,
    }

    db_setup()?;

    let database_url = env::var("DATABASE_URL")?;
    let connection = PgConnection::establish(&database_url)?;

    let rows: Vec<NamedGeometry> =
        diesel::sql_query("SELECT * FROM geometry_test").load(&connection)?;

    assert_eq!(rows.len(), 7);

    let row: &NamedGeometry = rows.get(4).unwrap();
    assert_eq!(row.name, "polygon");
    assert_eq!(
        row.obj,
        Geometry::Polygon(Polygon {
            rings: vec![LineString {
                points: vec![
                    Point {
                        x: 30.0,
                        y: 10.0,
                        srid: Some(4326)
                    },
                    Point {
                        x: 40.0,
                        y: 40.0,
                        srid: Some(4326)
                    },
                    Point {
                        x: 20.0,
                        y: 40.0,
                        srid: Some(4326)
                    },
                    Point {
                        x: 10.0,
                        y: 20.0,
                        srid: Some(4326)
                    },
                    Point {
                        x: 30.0,
                        y: 10.0,
                        srid: Some(4326)
                    }
                ],
                srid: Some(4326)
            }],
            srid: Some(4326),
        })
    );

    Ok(())
}

#[test]
fn read_linestring() -> Result<(), Box<dyn Error>> {
    use super::PostgisGeometry;
    use crate::ewkb::{LineString, Point};
    use diesel::prelude::*;
    use diesel::sql_types::*;
    use diesel::{Connection, PgConnection};

    #[derive(Queryable, QueryableByName, Debug, PartialEq)]
    struct NamedLineString {
        #[sql_type = "Varchar"]
        pub name: String,
        #[sql_type = "PostgisGeometry"]
        pub obj: LineString,
    }

    db_setup()?;

    let database_url = env::var("DATABASE_URL")?;
    let connection = PgConnection::establish(&database_url)?;

    let rows: Vec<NamedLineString> =
        diesel::sql_query("SELECT * FROM geometry_test WHERE GeometryType(obj) = 'LINESTRING'")
            .load(&connection)?;

    assert_eq!(rows.len(), 1);

    let row: &NamedLineString = rows.get(0).unwrap();
    assert_eq!(row.name, "linestring");
    assert_eq!(
        row.obj,
        LineString {
            points: vec![
                Point {
                    x: 30.0,
                    y: 10.0,
                    srid: Some(4326)
                },
                Point {
                    x: 10.0,
                    y: 30.0,
                    srid: Some(4326)
                },
                Point {
                    x: 40.0,
                    y: 40.0,
                    srid: Some(4326)
                }
            ],
            srid: Some(4326),
        }
    );

    Ok(())
}

// #[ignore]
#[test]
fn write_linestring() -> Result<(), Box<dyn Error>> {
    use crate::ewkb::Point;
    use diesel::prelude::*;
    use diesel::{Connection, PgConnection};

    pub mod schema {
        table! {
            use diesel::sql_types::Varchar;
            use crate::diesel_shim::PostgisGeometry;

            geometry_test (name) {
                name -> Varchar,
                obj -> PostgisGeometry,
            }
        }
    }

    use schema::geometry_test;

    #[derive(Insertable, Queryable, QueryableByName, Debug, PartialEq)]
    #[table_name = "geometry_test"]
    struct NamedPoint {
        pub name: String,
        pub obj: Point,
    }

    db_setup()?;

    let database_url = env::var("DATABASE_URL")?;
    let connection = PgConnection::establish(&database_url)?;

    let mut rows: Vec<NamedPoint>;

    rows = diesel::sql_query("SELECT * FROM geometry_test WHERE GeometryType(obj) = 'POINT'")
        .load(&connection)?;

    assert_eq!(rows.len(), 1);

    let data = NamedPoint {
        name: "new_point".into(),
        obj: Point {
            x: 0.0,
            y: 0.0,
            srid: None,
        },
    };

    let inserted: Vec<NamedPoint> = diesel::insert_into(schema::geometry_test::table)
        .values(&data)
        .get_results(&connection)?;

    assert_eq!(data, inserted[0]);

    rows = diesel::sql_query("SELECT * FROM geometry_test WHERE GeometryType(obj) = 'POINT'")
        .load(&connection)?;

    assert_eq!(rows.len(), 2);

    Ok(())
}
