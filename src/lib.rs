use mosquitto_plugin::*;
use ron::from_str;

#[derive(Debug)]
pub struct MQTTInterceptor {
    disabled_payload: Option<Vec<u8>>,
}

// Required trait implementation
impl MosquittoPlugin for MQTTInterceptor {
    fn init(opts: std::collections::HashMap<&str, &str>) -> Self {
        let disabled_payload = opts
            .get("disabled_payload")
            .map(|x| from_str::<Vec<u8>>(x).expect("Failed to parse disabled_payload"));

        MQTTInterceptor { disabled_payload }
    }

    fn acl_check(
        &mut self,
        _client: &dyn MosquittoClientContext,
        level: AclCheckAccessLevel,
        msg: MosquittoMessage,
    ) -> Result<Success, mosquitto_plugin::Error> {
        mosquitto_debug!("disabled topics: {:?}", self.disabled_payload);
        mosquitto_debug!("topic: {}", msg.topic);
        mosquitto_debug!("level requested: {}", level);

        match &self.disabled_payload {
            Some(it) => {
                if it == msg.payload {
                    Err(Error::AclDenied)
                } else {
                    Ok(Success)
                }
            }
            None => Ok(Success),
        }
    }
}

// This generates the dynamic c bindings functions that are exported and usable by mosquitto
create_dynamic_library!(MQTTInterceptor);
