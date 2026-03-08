use std::sync::Arc;

use self::application::connect_service::ConnectService;
use self::domain::connect_service_trait::ConnectServiceTrait;
use self::infrastructure::rumqttc_connection_service::RumqttcConnectionService;

pub mod application;
pub mod domain;
pub mod infrastructure;

fn main() {
    println!("Welcome to the MQTT Service!");

    let rumqttc_connection_service: Arc<dyn ConnectServiceTrait> =
        Arc::new(RumqttcConnectionService::new("prueba", "localhost", 1883));
    let connection_service = ConnectService::new(rumqttc_connection_service);

    let connect_result = connection_service.connect();
}
