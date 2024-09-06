use bevy_ecs::prelude::*;
use geo::{Contains, Coord, Polygon as GeoPolygon};
use kml::Kml;

#[derive(Debug, Clone, Copy, Default)]
pub struct Coordinates {
    pub longitude: f64,
    pub latitude: f64,
}

pub struct Perimeter {
    pub points: Vec<Coordinates>,
}

#[derive(Clone, Copy)]
pub struct Bounds {
    pub north_west_corner: Coordinates,
    pub size_degrees: f64,
}

impl Bounds {
    pub fn center(&self) -> Coordinates {
        Coordinates {
            latitude: self.north_west_corner.latitude - (self.size_degrees / 2.0),
            longitude: self.north_west_corner.longitude + (self.size_degrees / 2.0),
        }
    }
}

#[derive(Resource)]
pub struct EnvironmentResource {
    pub perimeter: Perimeter,
    pub cells: Vec<Bounds>,
}

impl EnvironmentResource {
    pub fn new(kml: Kml, cell_size: f64) -> Self {
        if let Kml::Polygon(polygon) = kml {
            // Convert the KML polygon to a Perimeter struct
            let perimeter = Perimeter {
                points: polygon
                    .outer
                    .coords
                    .iter()
                    .map(|coord| Coordinates {
                        latitude: coord.y,
                        longitude: coord.x,
                    })
                    .collect(),
            };

            // Convert the perimeter to geo_types::Polygon
            let geo_polygon = GeoPolygon::new(
                perimeter
                    .points
                    .iter()
                    .map(|coord| Coord {
                        x: coord.longitude,
                        y: coord.latitude,
                    })
                    .collect(),
                vec![], // No interior rings (holes) in the polygon
            );

            // Find the minimum and maximum latitude and longitude in the perimeter
            let (min_latitude, max_latitude, min_longitude, max_longitude) =
                perimeter.points.iter().fold(
                    (f64::MAX, f64::MIN, f64::MAX, f64::MIN),
                    |(min_lat, max_lat, min_lon, max_lon), coord| {
                        (
                            min_lat.min(coord.latitude),
                            max_lat.max(coord.latitude),
                            min_lon.min(coord.longitude),
                            max_lon.max(coord.longitude),
                        )
                    },
                );

            // Floor the latitude and longitude to the nearest cell_size
            let min_latitude = (min_latitude / cell_size).floor() * cell_size;
            let max_latitude = (max_latitude / cell_size).ceil() * cell_size;
            let min_longitude = (min_longitude / cell_size).floor() * cell_size;
            let max_longitude = (max_longitude / cell_size).ceil() * cell_size;

            let mut cells = Vec::new();

            // Create the cells by iterating over the latitude and longitude ranges
            let mut latitude = min_latitude;
            while latitude < max_latitude {
                let mut longitude = min_longitude;
                while longitude < max_longitude {
                    // Calculate the center of the cell
                    let cell_center = Coordinates {
                        latitude: latitude + cell_size / 2.0,
                        longitude: longitude + cell_size / 2.0,
                    };

                    // Check if the cell center is inside the polygon
                    if geo_polygon.contains(&Coord {
                        x: cell_center.longitude,
                        y: cell_center.latitude,
                    }) {
                        // Add the cell if the center is inside the polygon
                        cells.push(Bounds {
                            north_west_corner: Coordinates {
                                latitude,
                                longitude,
                            },
                            size_degrees: cell_size,
                        });
                    }

                    longitude += cell_size;
                }
                latitude += cell_size;
            }

            EnvironmentResource { perimeter, cells }
        } else {
            tracing::error!("Expected kml of polygon type");
            panic!("Expected kml of Polygon type!");
        }
    }
}
