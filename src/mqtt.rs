use rumqttc::{MqttOptions, AsyncClient, QoS, Event, Incoming, EventLoop};
use tokio::{task, time};
use std::time::Duration;
use std::error::Error;



struct MqttConnection {
    client: AsyncClient,
    eventloop: EventLoop,
}


impl MqttConnection {
    async fn new(client_id: String, host:String, port: u16, topic:String)->MqttConnection{
        let (client, eventloop) = MqttConnection::connection_rumqtt(client_id, host, port, topic).await;
        MqttConnection{
            client, eventloop
        }
    }

    async fn connection_rumqtt(client_id:String, host:String, port:u16, topic: String) -> (AsyncClient, EventLoop) {
        let mut mqttoptions = MqttOptions::new(client_id, host, port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
        client.subscribe(topic, QoS::AtMostOnce).await.unwrap_or_else(|e|{
            println!("failed to subscribe {:?}", e);
        });

        (client, eventloop)
    }

    pub async fn listen(&mut self){
        loop {
            let event = self.eventloop.poll().await;
            println!("{:?}", event);
            match self.eventloop.poll().await {
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
        let mut client = MqttConnection::new(
            "test_client".to_string(),
            "localhost".to_string(),
            1883,
            "test".to_string()
        ).await;
        client.listen().await;
    }
}