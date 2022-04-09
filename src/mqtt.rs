use rumqttc::{MqttOptions, AsyncClient, QoS, Event, Incoming};
use tokio::{task, time};
use std::time::Duration;
use std::error::Error;



struct MqttConnection {
    client_id: String,
    host: String,
    port: u16,
    topic: String,
}


impl MqttConnection {
    async fn connection_rumqtt(&self){
        let mut mqttoptions = MqttOptions::new(&self.client_id, &self.host, self.port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
        client.subscribe(&self.topic, QoS::AtMostOnce).await.unwrap_or_else(|e|{
            println!("failed to subscribe {:?}", e);
        });

        // task::spawn(async move {
        //     for i in 0..10 {
        //         client.publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize]).await.unwrap();
        //         time::sleep(Duration::from_millis(100)).await;
        //     }
        // });

        loop {
            let event = eventloop.poll().await;
            println!("{:?}", event);
            match eventloop.poll().await {
                Ok(Event::Incoming(Incoming::Publish(p))) => {
                    println!("Topic: {}, Payload: {:?}", p.topic, p.payload);
                }
                Ok(Event::Incoming(i)) => {
                    println!("Incoming = {:?}", i);
                }
                Ok(Event::Outgoing(o)) => println!("Outgoing = {:?}", o),
                Err(e) => {
                    println!("Error = {:?}", e);
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
        let client = MqttConnection{
            client_id: "test_client".to_string(),
            host: "localhost".to_string(),
            port: 1883,
            topic: "test".to_string()
        };
        client.connection_rumqtt().await;
    }
}