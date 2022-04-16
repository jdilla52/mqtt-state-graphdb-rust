use crate::config::StateLisenerSettings;
use crate::mqtt::MqttConnection;

pub struct StateListener {
    settings: StateLisenerSettings,
}

impl StateListener {
    fn new(settings: StateLisenerSettings) -> StateListener {
        StateListener { settings }
    }
    fn run(&self) {
        let connection = MqttConnection::new(self.settings.mqtt_settings.clone());
    }
}
