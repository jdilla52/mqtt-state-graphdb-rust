fn main() {
    println!("Hello, world!");
    let mut mqttoptions = MqttOptions::new("doesn'tmatter", settings.address, settings.port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    let qos: QoS = qos_from_u8(settings.mqtt_qos);
    client.subscribe(settings.mqtt_topic, qos).await.unwrap_or_else(|e|{
        println!("failed to subscribe {:?}", e);
        panic!("failed to subscribe");
    });
    
    
    
    
}
