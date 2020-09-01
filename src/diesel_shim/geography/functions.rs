use super::PostgisGeography;
use diesel::sql_types::*;

sql_function!(
    #[sql_name="ST_Area"] 
    fn st_area(g1: PostgisGeography, use_spheroid: Bool) -> Float
);

sql_function!(
    #[sql_name="ST_Azimuth"]
    fn st_azimuth(point_a: PostgisGeography, point_b: PostgisGeography) -> Float
);

sql_function!(
    #[sql_name="ST_Centroid"] 
    fn st_centroid(g1: PostgisGeography, use_spheroid: Bool) -> PostgisGeography
);

sql_function!(
    #[sql_name="ST_CoveredBy"]
    fn st_covered_by(geog_a: PostgisGeography, geog_b: PostgisGeography) -> Bool
);
sql_function!(
    #[sql_name="ST_Covers"]
    fn st_covers(geogpoly_a: PostgisGeography, geogpoint_b: PostgisGeography) -> Bool
);

sql_function!(
    #[sql_name="ST_DWithin"]
    fn st_d_within(gg1: PostgisGeography, gg2: PostgisGeography, distance_meters: Double, use_spheroid: Bool) -> Bool
);

sql_function!(
    #[sql_name="ST_Distance"]
    fn st_distance(geog_1: PostgisGeography, geog_2: PostgisGeography, use_spheroid: Bool) -> Float
);

sql_function!(
    #[sql_name="ST_Intersection"]
    fn st_intersection(geog_a: PostgisGeography, geog_b: PostgisGeography) -> PostgisGeography
);

sql_function!(
    #[sql_name="ST_Intersects"]
    fn st_intersects(geog_a: PostgisGeography, geog_b: PostgisGeography) -> Bool
);

sql_function!(
    #[sql_name="ST_Length"]
    fn st_length(geog: PostgisGeography, use_spheroid: Bool) -> Float
);

sql_function!(
    #[sql_name="ST_Perimeter"]
    fn st_perimeter(geog: PostgisGeography, use_spheroid: Bool) -> Float
);

sql_function!(
    #[sql_name="ST_Project"]
    fn st_project(g1: PostgisGeography, distance: Float, azimuth: Float) -> PostgisGeography
);

sql_function!(
    #[sql_name="ST_Segmentize"]
    fn st_segmentize(geog: PostgisGeography, max_segment_length: Float) -> PostgisGeography
);

sql_function!(
    #[sql_name="ST_Summary"]
    fn st_summary(g: PostgisGeography) -> Text
);
