use actix_cors::Cors;

pub struct CorsImpl {}

impl CorsImpl {
    pub fn get_default() -> Cors {
        Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DEL"])
            .max_age(3600)
    }
}
