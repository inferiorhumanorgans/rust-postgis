use std::io::Write;

use diesel::deserialize;
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::types::FromSql;

use crate::ewkb::{Geometry, LineString, Point, Polygon};
use crate::ewkb::{MultiLineString, MultiPoint, MultiPolygon};

#[derive(SqlType, QueryId)]
#[postgres(type_name = "geometry")]
pub struct PostgisGeometry;

impl FromSql<PostgisGeometry, Pg> for Geometry {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        use crate::ewkb::EwkbRead;
        use std::io::Cursor;
        let bytes = not_none!(bytes);
        let mut rdr = Cursor::new(bytes);
        Ok(Geometry::read_ewkb(&mut rdr)?.into())
    }
}

impl ToSql<PostgisGeometry, Pg> for Geometry {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        use crate::ewkb::{AsEwkbGeometry, EwkbWrite};
        Geometry::from(self.clone()).as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}

impl FromSql<PostgisGeometry, Pg> for Point {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        use crate::ewkb::EwkbRead;
        use std::io::Cursor;
        let bytes = not_none!(bytes);
        let mut rdr = Cursor::new(bytes);
        Ok(Point::read_ewkb(&mut rdr)?.into())
    }
}

impl ToSql<PostgisGeometry, Pg> for Point {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        use crate::ewkb::{AsEwkbPoint, EwkbWrite};
        Point::from(*self).as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}

impl FromSql<PostgisGeometry, Pg> for MultiPoint {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        use crate::ewkb::EwkbRead;
        use std::io::Cursor;
        let bytes = not_none!(bytes);
        let mut rdr = Cursor::new(bytes);
        Ok(MultiPoint::read_ewkb(&mut rdr)?.into())
    }
}

impl ToSql<PostgisGeometry, Pg> for MultiPoint {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        use crate::ewkb::{AsEwkbMultiPoint, EwkbWrite};
        MultiPoint::from(self.clone()).as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}

impl FromSql<PostgisGeometry, Pg> for LineString {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        use crate::ewkb::EwkbRead;
        use std::io::Cursor;
        let bytes = not_none!(bytes);
        let mut rdr = Cursor::new(bytes);
        Ok(LineString::read_ewkb(&mut rdr)?.into())
    }
}

impl ToSql<PostgisGeometry, Pg> for LineString {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        use crate::ewkb::{AsEwkbLineString, EwkbWrite};
        LineString::from(self.clone()).as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}

impl FromSql<PostgisGeometry, Pg> for MultiLineString {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        use crate::ewkb::EwkbRead;
        use std::io::Cursor;
        let bytes = not_none!(bytes);
        let mut rdr = Cursor::new(bytes);
        Ok(MultiLineString::read_ewkb(&mut rdr)?.into())
    }
}

impl ToSql<PostgisGeometry, Pg> for MultiLineString {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        use crate::ewkb::{AsEwkbMultiLineString, EwkbWrite};
        MultiLineString::from(self.clone())
            .as_ewkb()
            .write_ewkb(out)?;
        Ok(IsNull::No)
    }
}

impl FromSql<PostgisGeometry, Pg> for Polygon {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        use crate::ewkb::EwkbRead;
        use std::io::Cursor;
        let bytes = not_none!(bytes);
        let mut rdr = Cursor::new(bytes);
        Ok(Polygon::read_ewkb(&mut rdr)?.into())
    }
}

impl ToSql<PostgisGeometry, Pg> for Polygon {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        use crate::ewkb::{AsEwkbPolygon, EwkbWrite};
        Polygon::from(self.clone()).as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}

impl FromSql<PostgisGeometry, Pg> for MultiPolygon {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        use crate::ewkb::EwkbRead;
        use std::io::Cursor;
        let bytes = not_none!(bytes);
        let mut rdr = Cursor::new(bytes);
        Ok(MultiPolygon::read_ewkb(&mut rdr)?.into())
    }
}

impl ToSql<PostgisGeometry, Pg> for MultiPolygon {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        use crate::ewkb::{AsEwkbMultiPolygon, EwkbWrite};
        MultiPolygon::from(self.clone()).as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}

pub mod functions;

pub mod dsl {
    pub use super::functions::*;
}

#[cfg(test)]
mod tests;
