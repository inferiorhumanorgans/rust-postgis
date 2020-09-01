// use std::io::Write;

// use diesel::deserialize;
// use diesel::pg::Pg;
// use diesel::serialize::{self, IsNull, Output, ToSql};
// use diesel::types::FromSql;

// use crate::ewkb::{Geometry, LineString, Point, Polygon};
// use crate::ewkb::{MultiLineString, MultiPoint, MultiPolygon};

#[derive(SqlType, QueryId)]
#[postgres(type_name = "geography")]
pub struct PostgisGeography;

pub mod functions;

pub mod dsl {
    pub use super::functions::*;
}

#[cfg(test)]
mod tests;
