use std::time::Duration;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use tokio::time;
use mqtt_state_graph_db_rust::config::{MqttSettings, StateLisenerSettings};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
extern crate serde;

use serde::{Deserialize, Serialize};

fn qos_from_u8(orig: u8) -> QoS {
    match orig {
        0x0 => QoS::ExactlyOnce,
        0x1 => QoS::ExactlyOnce,
        0x2 => QoS::ExactlyOnce,
        _ => panic!("failed parse qos"),
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let global: StateLisenerSettings = confy::load_path("../default/StateListener.config").unwrap();
    let settings = global.mqtt_settings;
    let mut mqttoptions = MqttOptions::new("doesn'tmatter", settings.address, settings.port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    let qos: QoS = qos_from_u8(settings.mqtt_qos);

    let mut rng = rand::thread_rng();
    for i in 0..20 {
        let j = rng.gen_range(0..12);
        let mut topic: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        for i in 0..j {
            let sub_path: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(4)
                .map(char::from)
                .collect();

            topic = topic + "/" + sub_path.as_str();
        }

        client
            .publish(topic, qos, false, vec![1; i])
            .await
            .unwrap();

        time::sleep(Duration::from_millis(1)).await;
    }

    loop {
        let event = eventloop.poll().await;
        println!("{:?}", event.unwrap());
    }
    
    
}
