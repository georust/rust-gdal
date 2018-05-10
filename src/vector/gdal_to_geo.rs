use vector::Geometry;
use geo;
use geo_types;
use gdal_sys::{self, OGRwkbGeometryType};

impl geo::ToGeo<f64> for Geometry {
    fn to_geo(&self) -> geo_types::Geometry<f64> {
        let geometry_type = unsafe { gdal_sys::OGR_G_GetGeometryType(self.c_geometry()) };

        let ring = |n: usize| {
            let ring = unsafe { self._get_geometry(n) };
            match ring.to_geo() {
                geo_types::Geometry::LineString(r) => r,
                _ => panic!("Expected to get a LineString")
            }
        };

        match geometry_type {
            OGRwkbGeometryType::wkbPoint => {
                let (x, y, _) = self.get_point(0);
                geo_types::Geometry::Point(geo_types::Point(geo_types::Coordinate{x, y}))
            },
            OGRwkbGeometryType::wkbMultiPoint => {
                let point_count = unsafe { gdal_sys::OGR_G_GetGeometryCount(self.c_geometry()) } as usize;
                let coords = (0..point_count)
                    .map(|n| {
                        match unsafe { self._get_geometry(n) }.to_geo() {
                            geo_types::Geometry::Point(p) => p,
                            _ => panic!("Expected to get a Point")
                        }
                    })
                    .collect();
                geo_types::Geometry::MultiPoint(geo_types::MultiPoint(coords))
            },
            OGRwkbGeometryType::wkbLineString => {
                let coords = self.get_point_vec().iter()
                    .map(|&(x, y, _)| geo_types::Point(geo_types::Coordinate{x, y}))
                    .collect();
                geo_types::Geometry::LineString(geo_types::LineString(coords))
            },
            OGRwkbGeometryType::wkbMultiLineString => {
                let string_count = unsafe { gdal_sys::OGR_G_GetGeometryCount(self.c_geometry()) } as usize;
                let strings = (0..string_count)
                    .map(|n| {
                        match unsafe { self._get_geometry(n) }.to_geo() {
                            geo_types::Geometry::LineString(s) => s,
                            _ => panic!("Expected to get a LineString")
                        }
                    })
                    .collect();
                geo_types::Geometry::MultiLineString(geo_types::MultiLineString(strings))
            },
            OGRwkbGeometryType::wkbPolygon => {
                let ring_count = unsafe { gdal_sys::OGR_G_GetGeometryCount(self.c_geometry()) } as usize;
                let outer = ring(0);
                let holes = (1..ring_count).map(ring).collect();
                geo_types::Geometry::Polygon(geo_types::Polygon::new(outer, holes))
            },
            OGRwkbGeometryType::wkbMultiPolygon => {
                let string_count = unsafe { gdal_sys::OGR_G_GetGeometryCount(self.c_geometry()) } as usize;
                let strings = (0..string_count)
                    .map(|n| {
                        match unsafe { self._get_geometry(n) }.to_geo() {
                            geo_types::Geometry::Polygon(s) => s,
                            _ => panic!("Expected to get a Polygon")
                        }
                    })
                    .collect();
                geo_types::Geometry::MultiPolygon(geo_types::MultiPolygon(strings))
            },
            OGRwkbGeometryType::wkbGeometryCollection => {
                let item_count = unsafe { gdal_sys::OGR_G_GetGeometryCount(self.c_geometry()) } as usize;
                let geometry_list = (0..item_count)
                    .map(|n| unsafe { self._get_geometry(n) }.to_geo())
                    .collect();
                geo_types::Geometry::GeometryCollection(geo_types::GeometryCollection(geometry_list))
            }
            _ => panic!("Unknown geometry type")
        }
    }
}
