use crate::Publisher;
use sensor_model::RawMessage;
use yew::Callback;

pub struct HttpPublisher {
    url: reqwest::Url,
    username: String,
    password: String,
    command: Callback<RawMessage>,
}

impl HttpPublisher {
    pub fn new(
        url: reqwest::Url,
        username: String,
        password: String,
        command: Callback<RawMessage>,
    ) -> Self {
        Self {
            url,
            username,
            password,
            command,
        }
    }

    async fn do_send(
        payload: String,
        url: reqwest::Url,
        username: String,
        password: String,
        on_command: Callback<RawMessage>,
    ) -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let response = client
            .post(url)
            // .query(&["ct", &format!("{}", timeout)])
            .basic_auth(username, Some(password))
            .body(payload)
            .send()
            .await;

        if let Ok(response) = response {
            if let Ok(response) = response.json::<RawMessage>().await {
                on_command.emit(response);
            }
        }

        Ok(())
    }
}

impl Publisher for HttpPublisher {
    fn send(&self, payload: String) -> anyhow::Result<()> {
        let url = self.url.clone();
        let username = self.username.clone();
        let password = self.password.clone();
        let on_command = self.command.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Err(err) = Self::do_send(payload, url, username, password, on_command).await {
                log::warn!("Failed to publish data: {}", err);
            }
        });

        Ok(())
    }
}
