pub mod geospatial_computations {
    use rocket::form::validate::Len;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Degrees(pub f64);

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Meters(pub f64);

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Coord {
        pub lat: Degrees,
        pub lon: Degrees,
        pub alt: Option<Meters>,
    }

    pub struct GeospatialComputationError {
        pub code: u32,
        pub field: String,
        pub message: String
    }

    pub struct RayCastingAlgorithmResponse {
        
    }

    pub struct CrossProductForPointOrientation {

    }

    pub async fn ray_casting_algorithm(point: Coord, polygon: Vec<Coord>) -> Result<RayCastingAlgorithmResponse, GeospatialComputationError>{
        let polygon_size = polygon.len();

        if polygon_size < 3 {
            return Err(GeospatialComputationError{ 
                code: 400,
                field: "polygon".to_string(),
                message: "size of polygon less than three".to_string()
            });
        }

        return Ok(RayCastingAlgorithmResponse {  
            
        });
    }

    pub async fn cross_product_for_point_orientation(point: Coord, line: Vec<Coord>) -> Result<CrossProductForPointOrientation, GeospatialComputationError> {
        return Ok(CrossProductForPointOrientation {  
            
        });
    }
}