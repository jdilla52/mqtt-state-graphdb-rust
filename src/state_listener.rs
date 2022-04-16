use crate::config::StateLisenerSettings;
use crate::graph_db::Graphdb;
use crate::mqtt::MqttConnection;
use std::io::Bytes;

pub struct StateListener {
    settings: StateLisenerSettings,
}

impl StateListener {
    fn new(settings: StateLisenerSettings) -> StateListener {
        StateListener { settings }
    }
    async fn run(&self) {
        let mut connection = MqttConnection::new(self.settings.mqtt_settings.clone()).await;
        let gdb = Graphdb::new(self.settings.graphdb_settings.clone()).await;
        connection.listen(gdb).await;
    }
}
