use awc::Client;
use awc::http::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ClientError(pub String);

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ClientError {}

pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Clone)]
pub struct HttpClient {
    pub base_url: String,
    client: Client,
}

impl HttpClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::default(),
        }
    }

    pub async fn get(&self, path: &str, bearer_token: Option<&str>) -> ClientResult<(u16, String)> {
        let url = join_url(&self.base_url, path);
        let mut req = self.client.get(url);
        if let Some(token) = bearer_token {
            req = req.insert_header((AUTHORIZATION, format!("Bearer {}", token)));
        }
        let mut res = req.send().await.map_err(|e| ClientError(e.to_string()))?;
        let status = res.status().as_u16();
        let body = res.body().await.map_err(|e| ClientError(e.to_string()))?;
        let text = String::from_utf8(body.to_vec()).unwrap_or_default();
        Ok((status, text))
    }

    pub async fn post_json<T: Serialize>(
        &self,
        path: &str,
        body: &T,
        bearer_token: Option<&str>,
    ) -> ClientResult<(u16, String)> {
        let url = join_url(&self.base_url, path);
        let mut req = self
            .client
            .post(url)
            .insert_header((CONTENT_TYPE, "application/json"));
        if let Some(token) = bearer_token {
            req = req.insert_header((AUTHORIZATION, format!("Bearer {}", token)));
        }
        let mut res = req
            .send_json(body)
            .await
            .map_err(|e| ClientError(e.to_string()))?;
        let status = res.status().as_u16();
        let body = res.body().await.map_err(|e| ClientError(e.to_string()))?;
        let text = String::from_utf8(body.to_vec()).unwrap_or_default();
        Ok((status, text))
    }

    pub async fn post_json_typed<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
        bearer_token: Option<&str>,
    ) -> ClientResult<T> {
        let (status, text) = self.post_json(path, body, bearer_token).await?;
        if (200..300).contains(&status) {
            serde_json::from_str::<T>(&text).map_err(|e| ClientError(e.to_string()))
        } else {
            Err(ClientError(format!("{} {}", status, text)))
        }
    }

    pub async fn put_json<T: Serialize>(
        &self,
        path: &str,
        body: &T,
        bearer_token: Option<&str>,
    ) -> ClientResult<(u16, String)> {
        let url = join_url(&self.base_url, path);
        let mut req = self
            .client
            .put(url)
            .insert_header((CONTENT_TYPE, "application/json"));
        if let Some(token) = bearer_token {
            req = req.insert_header((AUTHORIZATION, format!("Bearer {}", token)));
        }
        let mut res = req
            .send_json(body)
            .await
            .map_err(|e| ClientError(e.to_string()))?;
        let status = res.status().as_u16();
        let body = res.body().await.map_err(|e| ClientError(e.to_string()))?;
        let text = String::from_utf8(body.to_vec()).unwrap_or_default();
        Ok((status, text))
    }

    pub async fn put_json_typed<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
        bearer_token: Option<&str>,
    ) -> ClientResult<T> {
        let (status, text) = self.put_json(path, body, bearer_token).await?;
        if (200..300).contains(&status) {
            serde_json::from_str::<T>(&text).map_err(|e| ClientError(e.to_string()))
        } else {
            Err(ClientError(format!("{} {}", status, text)))
        }
    }

    pub async fn delete(
        &self,
        path: &str,
        bearer_token: Option<&str>,
    ) -> ClientResult<(u16, String)> {
        let url = join_url(&self.base_url, path);
        let mut req = self.client.delete(url);
        if let Some(token) = bearer_token {
            req = req.insert_header((AUTHORIZATION, format!("Bearer {}", token)));
        }
        let mut res = req.send().await.map_err(|e| ClientError(e.to_string()))?;
        let status = res.status().as_u16();
        let body = res.body().await.map_err(|e| ClientError(e.to_string()))?;
        let text = String::from_utf8(body.to_vec()).unwrap_or_default();
        Ok((status, text))
    }

    pub async fn get_json<T: DeserializeOwned>(
        &self,
        path: &str,
        bearer_token: Option<&str>,
    ) -> ClientResult<T> {
        let (status, text) = self.get(path, bearer_token).await?;
        if (200..300).contains(&status) {
            serde_json::from_str::<T>(&text).map_err(|e| ClientError(e.to_string()))
        } else {
            Err(ClientError(format!("{} {}", status, text)))
        }
    }
}

fn join_url(base: &str, path: &str) -> String {
    format!(
        "{}/{}",
        base.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}
