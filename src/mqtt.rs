use rumqttc::{MqttOptions, AsyncClient, QoS};
use tokio::{task, time};
use std::time::Duration;
use std::error::Error;


async fn connection_rumqtt(){
    let mut mqttoptions = MqttOptions::new("rumqtt-test", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).await.unwrap_or_else(|e|{
        println!("failed to subscribe {:?}", e);
    });

    task::spawn(async move {
        for i in 0..10 {
            client.publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize]).await.unwrap();
            time::sleep(Duration::from_millis(100)).await;
        }
    });

    loop {
        let notification = eventloop.poll().await.unwrap();
        println!("Received = {:?}", notification);
    }

    // loop {
    //     let notification = eventloop.poll().await.unwrap();
    //     println!("Received = {:?}", notification);
    // }
}


#[cfg(test)]
mod test_eval{
    use super::*;

    #[tokio::test]
    async fn test_control_construct() {
        connection_rumqtt().await;
    }
}