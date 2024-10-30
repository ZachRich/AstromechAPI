mod i2c_manager;

use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use crate::i2c_manager::I2cManager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create I2cManager wrapped in Arc<Mutex<...>> for thread-safe access
    let i2c_manager = Arc::new(Mutex::new(I2cManager::new()));

    HttpServer::new(move || {
        let i2c_manager = Arc::clone(&i2c_manager); // Clone the Arc

        App::new()
            .app_data(web::Data::new(i2c_manager))
            .route("/servos/list", web::get().to(get_servos)) // New route to list servos
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

// Handler Functions

async fn get_servos(i2c_manager: web::Data<Arc<Mutex<I2cManager>>>) -> String {
    let manager = i2c_manager.lock().unwrap();
    manager.list_servos();
    "Servos list".to_string()
}
