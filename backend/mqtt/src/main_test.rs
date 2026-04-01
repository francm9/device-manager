use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("MQTT Service loading...");

    mqtt::connect("localhost", 1883, "prueba");

    let _ = mqtt::subscribe("sensors").await;
    let _ = mqtt::publish("status", "online").await;

    loop {
        let messages = mqtt::get_new_topic_messages("sensors").await;
        info!("Size -> {}", messages.len());
        for msg in messages {
            info!("Message received: {:?}", msg);
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}
