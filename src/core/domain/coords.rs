
pub struct Coords {
    pub lng: f64,
    pub lat: f64,
    pub alt: f64,
}

impl Coords {
    pub fn new(lng: f64, lat: f64, alt: f64) -> Self {
        Self { lng, lat, alt }
    }
}

