use mosquitto_plugin::*;
use ron::from_str;

#[derive(Debug)]
pub struct MQTTInterceptor {
    disabled_payload: Option<Vec<u8>>,
}

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
                let payload = msg.payload.to_vec();
                let reject_payload = payload.len() == it.len()
                    && payload.iter().enumerate().all(|(index, x)| it[index] == *x);

                if reject_payload {
                    Err(Error::AclDenied)
                } else {
                    Ok(Success)
                }
            }
            None => Ok(Success),
        }
    }
}

create_dynamic_library!(MQTTInterceptor);
