use mosquitto_plugin::*;
use ron::from_str;

#[derive(Debug)]
pub struct MQTTInterceptor {
    username: String,
    password: String,
    disabled_payload: Option<Vec<u8>>,
}

impl MosquittoPlugin for MQTTInterceptor {
    fn init(opts: std::collections::HashMap<&str, &str>) -> Self {
        let username = from_str(opts.get("username").expect("Username option required"))
            .expect("Failed to parse username");
        let password = from_str(opts.get("password").expect("Password option required"))
            .expect("Failed to parse password");
        let disabled_payload = opts
            .get("disabled_payload")
            .map(|x| from_str::<Vec<u8>>(x).expect("Failed to parse disabled_payload"));

        MQTTInterceptor {
            username,
            password,
            disabled_payload,
        }
    }

    fn username_password(
        &mut self,
        _client: &dyn MosquittoClientContext,
        username: Option<&str>,
        password: Option<&str>,
    ) -> Result<Success, Error> {
        let (username, password) = username
            .and_then(|x| password.map(|y| (x, y)))
            .ok_or(Error::Auth)?;

        if username != self.username.as_str() || password != self.password.as_str() {
            return Err(Error::Auth);
        }

        Ok(Success)
    }

    fn acl_check(
        &mut self,
        client: &dyn MosquittoClientContext,
        _level: AclCheckAccessLevel,
        msg: MosquittoMessage,
    ) -> Result<Success, mosquitto_plugin::Error> {
        if self.username.as_str() != client.get_username() {
            return Err(Error::AclDenied);
        }

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
