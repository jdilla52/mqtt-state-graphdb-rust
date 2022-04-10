use rumqttc::{MqttOptions, AsyncClient, QoS, Event, Incoming, EventLoop};
use tokio::{task, time};
use std::time::Duration;
use std::error::Error;
use log::debug;
use tokio::task::JoinHandle;
use crate::config::MqttSettings;


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
    pub(crate) async fn new(settings:MqttSettings) ->MqttConnection{
        let (client, eventloop) = MqttConnection::connection_rumqtt(settings).await;
        MqttConnection{
            client, eventloop
        }
    }

    async fn connection_rumqtt(settings:MqttSettings) -> (AsyncClient, EventLoop) {
        let mut mqttoptions = MqttOptions::new(settings.client_id, settings.address, settings.port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        let qos: QoS = qos_from_u8(settings.mqtt_qos);
        client.subscribe(settings.mqtt_topic, qos).await.unwrap_or_else(|e|{
            println!("failed to subscribe {:?}", e);
        });

        (client, eventloop)
    }

    pub async fn listen(&mut self) {
        loop {
            match self.eventloop.poll().await {
                Ok(Event::Incoming(Incoming::Publish(p))) => {
                    debug!("Topic: {}, Payload: {:?}", p.topic, p.payload);
                }
                Ok(Event::Incoming(i)) => {
                    debug!("Incoming = {:?}", i);
                }
                Ok(Event::Outgoing(o)) => debug!("Outgoing = {:?}", o),
                Err(e) => {
                    debug!("Error = {:?}", e);
                }
            }
        }
    }
}


#[cfg(test)]
mod test_eval{
    use super::*;

    #[tokio::test]
    async fn test_control_construct() {
        // connection_rumqtt().await;
        env_logger::init();

        let mut client = MqttConnection::new(
           MqttSettings::default()
        ).await;
        client.listen().await;
    }
}