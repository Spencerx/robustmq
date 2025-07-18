// Copyright 2023 RobustMQ Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_base::utils::time_util::timestamp_to_local_datetime;
use protocol::broker_mqtt::broker_mqtt_admin::MqttSubscribeRaw;
use protocol::mqtt::common::{Filter, MqttProtocol, SubscribeProperties};
use serde::{Deserialize, Serialize};

pub const SHARE_SUB_PREFIX: &str = "$share";
pub const QUEUE_SUB_PREFIX: &str = "$queue";

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct MqttSubscribe {
    pub client_id: String,
    pub path: String,
    pub cluster_name: String,
    pub broker_id: u64,
    pub protocol: MqttProtocol,
    pub filter: Filter,
    pub pkid: u16,
    pub subscribe_properties: Option<SubscribeProperties>,
    pub create_time: u64,
}

impl MqttSubscribe {
    pub fn encode(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}

impl From<MqttSubscribe> for MqttSubscribeRaw {
    fn from(sub: MqttSubscribe) -> Self {
        Self {
            broker_id: sub.broker_id,
            client_id: sub.client_id,
            create_time: timestamp_to_local_datetime(sub.create_time as i64),
            no_local: if sub.filter.nolocal { 1 } else { 0 },
            path: sub.path.clone(),
            pk_id: sub.pkid as u32,
            preserve_retain: if sub.filter.preserve_retain { 1 } else { 0 },
            properties: serde_json::to_string(&sub.subscribe_properties).unwrap(),
            protocol: format!("{:?}", sub.protocol),
            qos: format!("{:?}", sub.filter.qos),
            retain_handling: format!("{:?}", sub.filter.retain_handling),
            is_share_sub: is_mqtt_share_subscribe(&sub.path),
        }
    }
}

pub fn is_mqtt_share_subscribe(sub_name: &str) -> bool {
    is_mqtt_share_sub(sub_name) || is_mqtt_queue_sub(sub_name)
}

pub fn is_mqtt_share_sub(sub_name: &str) -> bool {
    sub_name.starts_with(SHARE_SUB_PREFIX)
}

pub fn is_mqtt_queue_sub(sub_name: &str) -> bool {
    sub_name.starts_with(QUEUE_SUB_PREFIX)
}

#[cfg(test)]
mod tests {
    use crate::mqtt::subscribe_data::{is_mqtt_share_sub, is_mqtt_share_subscribe};

    #[test]
    fn is_mqtt_share_subscribe_test() {
        assert!(is_mqtt_share_sub("$share/g1/test/hello"));
        assert!(is_mqtt_share_subscribe("$share/g1/test/hello"));
        assert!(!is_mqtt_share_subscribe("/test/hello"));
    }
}
