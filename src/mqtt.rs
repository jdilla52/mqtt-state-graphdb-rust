use crate::config::MqttSettings;
use log::{debug, error};
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};
use std::error::Error;
use std::str::Bytes;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::{task, time};
use crate::graph_db::Graphdb;

pub struct MqttConnection {
    client: AsyncClient,
    eventloop: EventLoop,
}

fn qos_from_u8(orig: u8) -> QoS {
    match orig {
        0x0 => QoS::ExactlyOnce,
        0x1 => QoS::ExactlyOnce,
        0x2 => QoS::ExactlyOnce,
        _ => panic!("failed parse qos"),
    }
}
impl MqttConnection {
    pub(crate) async fn new(settings: MqttSettings) -> MqttConnection {
        let (client, eventloop) = MqttConnection::connection_rumqtt(settings).await;
        MqttConnection { client, eventloop }
    }

    async fn connection_rumqtt(settings: MqttSettings) -> (AsyncClient, EventLoop) {
        let mut mqttoptions = MqttOptions::new(settings.client_id, settings.address, settings.port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        let qos: QoS = qos_from_u8(settings.mqtt_qos);
        client
            .subscribe(settings.mqtt_topic, qos)
            .await
            .unwrap_or_else(|e| {
                println!("failed to subscribe {:?}", e);
                panic!("failed to subscribe");
            });

        (client, eventloop)
    }

    pub async fn listen(&mut self, action: Graphdb) {
        loop {
            match self.eventloop.poll().await {
                Ok(Event::Incoming(Incoming::Publish(p))) => {
                    debug!("Topic: {}, Payload: {:?}", p.topic, p.payload);
                    match action.create_path(p.topic, p.payload.to_vec()).await {
                        Ok(t)=>debug!("successful"),
                        Err(e)=> error!("failed to insert: {:?}", e)
                    }
                }
                Ok(Event::Incoming(i)) => {
                    debug!("Incoming = {:?}", i);
                }
                Ok(Event::Outgoing(o)) => {
                    debug!("Outgoing = {:?}", o);
                }
                Err(e) => {
                    let err = format!("error: {:?}", e);
                    debug!("Error = {:?}", e);
                }
            }
        }
    }
}

#[cfg(test)]
mod test_eval {
    use crate::config::GraphdbSettings;
    use super::*;

    #[tokio::test]
    async fn test_control_construct() {
        // connection_rumqtt().await;
        env_logger::init();
        let gdb = Graphdb::new(GraphdbSettings::default()).await;
        let mut client = MqttConnection::new(MqttSettings::default()).await;
        client.listen(gdb).await;
    }
}
