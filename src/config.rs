#![allow(dead_code)]

extern crate confy;
extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MqttSettings {
    pub address: String,
    pub port: u16,
    pub(crate) client_id: String,
    pub mqtt_topic: String,
    pub mqtt_qos: u8,
    pub(crate) will_message: String,
    pub(crate) will_topic: String,
    pub(crate) user: String,
    pub(crate) pwd: String,
}

impl Default for MqttSettings {
    fn default() -> MqttSettings {
        MqttSettings {
            address: "ssl://test.branchviewer.com".to_string(),
            port: 1883,
            client_id: "test_client".to_string(),
            mqtt_topic: "test".to_string(),
            mqtt_qos: 0,
            will_message: "Bridge node has failed".to_string(),
            will_topic: "test/dead".to_string(),
            user: "test2".to_string(),
            pwd: "test".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GraphdbSettings {
    pub(crate) address: String,
    pub(crate) user: String,
    pub(crate) pass: String,
}

impl GraphdbSettings {
    fn new<A: Into<String>, B: Into<String>, C: Into<String>>(
        address: A,
        user: B,
        pass: C,
    ) -> GraphdbSettings {
        GraphdbSettings {
            address: address.into(),
            user: user.into(),
            pass: pass.into(),
        }
    }

    pub(crate) fn default() -> GraphdbSettings {
        GraphdbSettings {
            address: "127.0.0.1:7687".to_string(),
            user: "neo4j".to_string(),
            pass: "test".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct StateLisenerSettings {
    pub mqtt_settings: MqttSettings,
    pub graphdb_settings: GraphdbSettings,
}

impl Default for StateLisenerSettings {
    fn default() -> Self {
        StateLisenerSettings {
            mqtt_settings: MqttSettings::default(),
            graphdb_settings: GraphdbSettings::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{GraphdbSettings, MqttSettings, StateLisenerSettings};

    #[test]
    fn test_parse() {
        let t: StateLisenerSettings = confy::load_path("default/StateListener.config").unwrap();
    }
    //     let t = StatsSettings {
    //         mqtt_settings: MqttSettings {
    //             address: "tcp://127.0.0.1:1883".to_string(),
    //             client_id: "test_client".to_string(),
    //             mqtt_topic: vec!["*".to_string()],
    //             mqtt_qos: vec![1],
    //             will_message: "Bridge node has failed".to_string(),
    //             will_topic: "bridge/dead".to_string(),
    //             user: "mqttAdmin".to_string(),
    //             pwd: "super".to_string(),
    //         },
    //     };
    // }
}
