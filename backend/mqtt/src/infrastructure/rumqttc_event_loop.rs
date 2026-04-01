use crate::domain::mqtt_event_handler::MqttEventHandler;
use rumqttc::{Event, EventLoop, Packet};
use std::sync::Arc;
use tracing::{error, info};

pub struct RumqttcEventLoop {
    event_loop: EventLoop,
    handler: Arc<dyn MqttEventHandler>,
}

impl RumqttcEventLoop {
    pub fn new(event_loop: EventLoop, handler: Arc<dyn MqttEventHandler>) -> Self {
        Self {
            event_loop,
            handler,
        }
    }

    pub async fn run(mut self) {
        loop {
            match self.event_loop.poll().await {
                // --- Incoming ---
                Ok(Event::Incoming(Packet::Publish(p))) => {
                    self.handler.on_message(&p.topic, &p.payload).await;
                }
                Ok(Event::Incoming(Packet::ConnAck(ack))) => {
                    info!("Connected to broker, code: {:?}", ack.code);
                    self.handler.on_connect();
                }
                Ok(Event::Incoming(Packet::Disconnect)) => {
                    info!("Broker sent disconnect");
                    self.handler.on_disconnect();
                    break;
                }
                Ok(Event::Incoming(Packet::SubAck(ack))) => {
                    info!("Subscription acknowledged, pkid: {}", ack.pkid);
                }
                Ok(Event::Incoming(Packet::UnsubAck(ack))) => {
                    info!("Unsubscription acknowledged, pkid: {}", ack.pkid);
                }
                Ok(Event::Incoming(Packet::PubAck(ack))) => {
                    info!("Publish acknowledged (QoS 1), pkid: {}", ack.pkid);
                }
                Ok(Event::Incoming(Packet::PubRec(rec))) => {
                    info!("Publish received (QoS 2 step 1), pkid: {}", rec.pkid);
                }
                Ok(Event::Incoming(Packet::PubRel(rel))) => {
                    info!("Publish released (QoS 2 step 2), pkid: {}", rel.pkid);
                }
                Ok(Event::Incoming(Packet::PubComp(comp))) => {
                    info!("Publish complete (QoS 2 step 3), pkid: {}", comp.pkid);
                }
                Ok(Event::Incoming(Packet::PingResp)) => {
                    info!("Ping response received (keepalive ok)");
                }
                Ok(Event::Incoming(Packet::PingReq)) => {
                    info!("Ping request received");
                }
                Ok(Event::Incoming(Packet::Connect(_))) => {
                    info!("Connect packet received (unexpected in client)");
                }
                Ok(Event::Incoming(Packet::Unsubscribe(_))) => {
                    info!("Unsubscribe packet received");
                }
                Ok(Event::Incoming(Packet::Subscribe(_))) => {
                    info!("Subscribe packet received");
                }

                // --- Outgoing ---
                Ok(Event::Outgoing(outgoing)) => {
                    info!("Outgoing packet: {:?}", outgoing);
                }

                // --- Error ---
                Err(e) => {
                    error!("MQTT event loop error: {e}");
                }
            }
        }
    }
}
