use super::PostgisGeometry;
use diesel::sql_types::*;

sql_function!(
    #[sql_name="GeometryType"] 
    fn geometry_type(x: PostgisGeometry) -> Text
);

// SQL/MM 3 functions
sql_function!(
    #[sql_name="ST_AddEdgeModFace"]
    fn st_add_edge_mod_face(topology: Varchar, node: Integer, other_node: Integer, curve: PostgisGeometry) -> Integer
);

sql_function!(
    #[sql_name="ST_AddEdgeNewFaces"]
    fn st_add_edge_new_faces(topology: Varchar, node: Integer, other_node: Integer, curve: PostgisGeometry) -> Integer
);

sql_function!(
    #[sql_name="ST_AddIsoEdge"]
    fn st_add_iso_edge(topology: Varchar, node: Integer, other_node: Integer, linestring: PostgisGeometry) -> Integer
);

sql_function!(
    #[sql_name="ST_AddIsoNode"]
    fn st_add_iso_node(topology: Varchar, face: Integer, point: PostgisGeometry) -> Integer
);

sql_function!(
    #[sql_name="ST_Area"]
    fn st_area(g1: PostgisGeometry) -> Float
);

sql_function!(
    #[sql_name="ST_Boundary"]
    fn st_boundary(geom_a: PostgisGeometry) -> Nullable<PostgisGeometry>
);

sql_function!(
    #[sql_name="ST_Buffer"]
    fn st_buffer(geom_a: PostgisGeometry) -> PostgisGeometry
);

sql_function!(
    #[sql_name="ST_Buffer"]
    fn st_buffer_with_segments(geom_a: PostgisGeometry, num_seg_quarter_circle: Integer) -> PostgisGeometry
);

sql_function!(
    #[sql_name="ST_Buffer"]
    fn st_buffer_with_style(geom_a: PostgisGeometry, buffer_style_parameters: Text) -> PostgisGeometry
);

sql_function!(
    #[sql_name="ST_Centroid"]
    fn st_centroid(g1: PostgisGeometry) -> PostgisGeometry
);

sql_function!(
    #[sql_name="ST_ChangeEdgeGeom"]
    fn st_change_edge_geom(topology: Varchar, edge: Integer, curve: PostgisGeometry) -> Integer
);

sql_function!(
    #[sql_name="ST_Contains"]
    fn st_contains(geom_a: PostgisGeometry, geom_b: PostgisGeometry) -> Bool
);

sql_function!(
    #[sql_name="ST_ConvexHull"]
    fn st_convex_hull(geom_a: PostgisGeometry) -> PostgisGeometry
);

sql_function!(
    #[sql_name="ST_CoordDim"]
    fn st_coord_dim(geom_a: PostgisGeometry) -> Integer
);

sql_function!(
    #[sql_name="ST_CreateTopoGeo"]
    fn st_create_topo_geo(topology: Varchar, collection: PostgisGeometry) -> Text
);

sql_function!(
    #[sql_name="ST_Crosses"]
    fn st_crosses(g1: PostgisGeometry, g2: PostgisGeometry) -> Bool
);

sql_function!(
    #[sql_name="ST_CurveToLine"]
    fn st_curve_to_line(curve_geom: PostgisGeometry, tolerance: Float, tolerance_type: Integer, flags: Integer) -> PostgisGeometry
);

sql_function!(
    #[sql_name="ST_Difference"]
    fn st_difference(geom_a: PostgisGeometry, geom_b: PostgisGeometry) -> PostgisGeometry
);

sql_function!(
    #[sql_name="ST_Dimension"]
    fn st_dimension(g: PostgisGeometry) -> Integer
);

sql_function!(
    #[sql_name="ST_Disjoint"]
    fn st_disjoint(a: PostgisGeometry, b: PostgisGeometry) -> Bool
);

sql_function!(
    #[sql_name="ST_Distance"]
    fn st_distance(g1: PostgisGeometry, g2: PostgisGeometry) -> Float
);

sql_function!(
    #[sql_name="ST_EndPoint"]
    fn st_end_point(g: PostgisGeometry) -> Nullable<PostgisGeometry>
);

sql_function!(
    #[sql_name="ST_Envelope"]
    fn st_envelope(g1: PostgisGeometry) -> PostgisGeometry
);

sql_function!(
    #[sql_name="ST_Equals"]
    fn st_equals(a: PostgisGeometry, b: PostgisGeometry) -> Bool
);

sql_function!(
    #[sql_name="ST_ExteriorRing"]
    fn st_exterior_ring(polygon: PostgisGeometry) -> Nullable<PostgisGeometry>
);

sql_function!(
    #[sql_name="ST_GeometryN"]
    fn st_geometry_n(geom_a: PostgisGeometry, n: Integer) -> Nullable<PostgisGeometry>
);

sql_function!(
    #[sql_name="ST_GeometryType"]
    fn st_geometry_type(g1: PostgisGeometry) -> Text
);

sql_function!(
    #[sql_name="ST_GetFaceEdges"]
    fn st_get_face_edges(topology: Varchar, face: Integer) -> PostgisGeometry
);

sql_function!(
    #[sql_name="ST_GetFaceGeometry"]
    fn st_get_face_geometry(topology: Varchar, face: Integer) -> PostgisGeometry
);

sql_function!(
    #[sql_name="ST_InteriorRingN"]
    fn st_interior_ring_n(polygon: PostgisGeometry, n: Integer) -> Nullable<PostgisGeometry>
);

sql_function!(
    #[sql_name="ST_Intersection"]
    fn st_intersection(geom_a: PostgisGeometry, geom_b: PostgisGeometry) -> PostgisGeometry
);

sql_function!(
    #[sql_name="ST_Intersects"]
    fn st_intersects(geom_a: PostgisGeometry, geom_b: PostgisGeometry) -> Bool
);

sql_function!(
    #[sql_name="ST_IsClosed"]
    fn st_is_closed(g: Nullable<PostgisGeometry>) -> Bool
);

sql_function!(
    #[sql_name="ST_IsEmpty"]
    fn st_is_empty(g: Nullable<PostgisGeometry>) -> Nullable<Bool>
);

sql_function!(
    #[sql_name="ST_IsRing"]
    fn st_is_ring(g: Nullable<PostgisGeometry>) -> Nullable<Bool>
);

sql_function!(
    #[sql_name="ST_IsSimple"]
    fn st_is_simple(geom_a: Nullable<PostgisGeometry>) -> Nullable<Bool>
);

sql_function!(
    #[sql_name="ST_IsValid"]
    fn st_is_valid(g: Nullable<PostgisGeometry>) -> Nullable<Bool>
);

sql_function!(
    #[sql_name="ST_Length"]
    fn st_length(a_2d_line_string: PostgisGeometry) -> Float
);

sql_function!(
    #[sql_name="ST_M"]
    fn st_m(point: PostgisGeometry) -> Nullable<Float>
);

sql_function!(
    #[sql_name="ST_ModEdgeHeal"]
    fn st_mod_edge_heal(topology: Varchar, edge: Integer, another_edge: Integer) -> Integer
);

sql_function!(
    #[sql_name="ST_ModEdgeSplit"]
    fn st_mod_edge_split(topology: Varchar, edge: Integer, point: PostgisGeometry) -> Integer
);

sql_function!(
    #[sql_name="ST_NumGeometries"]
    fn st_num_geometries(geom: PostgisGeometry) -> Nullable<Integer>
);

sql_function!(
    #[sql_name="ST_NumInteriorRings"]
    fn st_num_interior_rings(polygon: PostgisGeometry) -> Nullable<Integer>
);

sql_function!(
    #[sql_name="ST_NumPoints"]
    fn st_num_points(g1: PostgisGeometry) -> Integer
);

sql_function!(
    #[sql_name="ST_SRID"]
    fn st_srid(g1: PostgisGeometry) -> Integer
);

sql_function!(
    #[sql_name="ST_StartPoint"]
    fn st_start_point(geom_a: PostgisGeometry) -> Nullable<PostgisGeometry>
);

sql_function!(
    #[sql_name="ST_Touches"]
    fn st_touches(g1: PostgisGeometry, g2: PostgisGeometry) -> Bool
);

sql_function!(
    #[sql_name="ST_Within"]
    fn st_within(a: PostgisGeometry, b: PostgisGeometry) -> Bool
);

// PostGIS Functions that support 3D

sql_function!(
    #[sql_name="ST_GeometricMedian"]
    fn st_geometric_median(g: PostgisGeometry, tolerance: Float, max_iter: Integer, fail_if_not_converged: Bool) -> PostgisGeometry
);

// no_arg_sql_function doesn't support sql_name
no_arg_sql_function!(PostGIS_Full_Version, Text, "Reports full postgis version and build configuration infos.");
no_arg_sql_function!(PostGIS_GEOS_Version, Nullable<Text>, "Returns the version number of the GEOS library, or NULL if GEOS support is not enabled.");
no_arg_sql_function!(PostGIS_HasBBox, Bool, "Returns TRUE if the bbox of this geometry is cached, FALSE otherwise.");
no_arg_sql_function!(PostGIS_LibXML_Version, Text, "Returns the version number of the LibXML2 library.");
no_arg_sql_function!(PostGIS_Lib_Build_Date, Text, "Returns build date of the PostGIS library.");
no_arg_sql_function!(PostGIS_Lib_Version, Text, "Returns the version number of the PostGIS library.");
no_arg_sql_function!(PostGIS_Liblwgeom_Version, Text, "Returns the version number of the liblwgeom library.");
no_arg_sql_function!(PostGIS_PROJ_Version, Nullable<Text>, "Returns the version number of the PROJ4 library, or NULL if PROJ4 support is not enabled.");
no_arg_sql_function!(PostGIS_Scripts_Build_Date, Text, "Returns build date of the PostGIS scripts.");
no_arg_sql_function!(PostGIS_Scripts_Installed, Text, "Returns version of the postgis scripts installed in this database.");
no_arg_sql_function!(PostGIS_Scripts_Released, Text, "Returns the version number of the postgis.sql script released with the installed postgis lib.");
no_arg_sql_function!(PostGIS_Version, Text, "Returns PostGIS version number and compile-time options.");
no_arg_sql_function!(PostGIS_Wagyu_Version, Nullable<Text>, "Returns the version number of the internal Wagyu library, or NULL if Wagyu support is not enabled.");
