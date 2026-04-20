use crate::error::{CliError, Result};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> Reply<T> {
    pub fn into_result(self) -> Result<T> {
        if self.code == 0 {
            self.data.ok_or_else(|| CliError::Api(self.code, "Empty response data".to_string()))
        } else {
            Err(CliError::Api(self.code, self.msg))
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SkillSummary {
    #[serde(default)]
    pub id: Uuid,
    pub name: String,
    #[serde(default)]
    pub display_name: String,
    pub description: String,
    pub version: String,
    pub downloads: i64,
    #[serde(default)]
    pub avg_rating: f64,
    pub namespace: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub items: Vec<SkillSummary>,
    pub total: i64,
}

pub struct ApiClient {
    client: Client,
    base_url: String,
    token: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: String, token: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url,
            token,
        }
    }

    fn require_auth(&self) -> Result<()> {
        if self.token.is_none() {
            return Err(CliError::NotAuthenticated);
        }
        Ok(())
    }

    async fn post<T: Serialize, R: for<'de> Deserialize<'de>>(&self, path: &str, body: &T) -> Result<R> {
        let mut req = self.client.post(format!("{}{}", self.base_url, path));
        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }
        let resp = req.json(body).send().await?;
        self.handle_response(resp).await
    }

    async fn get<R: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<R> {
        let mut req = self.client.get(format!("{}{}", self.base_url, path));
        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }
        let resp = req.send().await?;
        self.handle_response(resp).await
    }

    async fn post_multipart(&self, path: &str, form: reqwest::multipart::Form) -> Result<Reply<serde_json::Value>> {
        self.require_auth()?;
        let req = self
            .client
            .post(format!("{}{}", self.base_url, path))
            .header("Authorization", format!("Bearer {}", self.token.as_ref().unwrap()))
            .multipart(form);
        let resp = req.send().await?;
        self.handle_response(resp).await
    }

    async fn download(&self, path: &str) -> Result<bytes::Bytes> {
        self.require_auth()?;
        let req = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .header("Authorization", format!("Bearer {}", self.token.as_ref().unwrap()));
        let resp = req.send().await?;
        let status = resp.status();
        if status != StatusCode::OK {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api(status.as_u16() as i32, text));
        }
        Ok(resp.bytes().await?)
    }

    async fn handle_response<R: for<'de> Deserialize<'de>>(&self, resp: reqwest::Response) -> Result<R> {
        let status = resp.status();
        let text = resp.text().await?;
        if status != StatusCode::OK {
            return Err(CliError::Api(status.as_u16() as i32, text));
        }
        let reply: Reply<R> = serde_json::from_str(&text)
            .map_err(|e| CliError::Parse(format!("Failed to parse response: {}", e)))?;
        reply.into_result()
    }

    pub async fn login(&mut self, username: &str, password: &str) -> Result<LoginResponse> {
        let req = LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        };
        let resp: LoginResponse = self.post("/api/auth/login", &req).await?;
        self.token = Some(resp.token.clone());
        Ok(resp)
    }

    pub async fn search(&self, query: &str, page: i32, page_size: i32, sort: Option<&str>) -> Result<SearchResponse> {
        let mut path = format!(
            "/api/skill-hub/search?q={}&page={}&page_size={}",
            urlencoding::encode(query),
            page,
            page_size
        );
        if let Some(s) = sort {
            path.push_str(&format!("&sort={}", s));
        }
        self.get(&path).await
    }

    pub async fn publish(&self, namespace: &str, zip_path: &std::path::Path) -> Result<serde_json::Value> {
        self.require_auth()?;
        let zip_bytes = std::fs::read(zip_path)?;
        let part = reqwest::multipart::Part::bytes(zip_bytes)
            .file_name(zip_path.file_name().unwrap().to_string_lossy().to_string())
            .mime_str("application/zip")?;
        let form = reqwest::multipart::Form::new().part("file", part);
        let reply = self.post_multipart(&format!("/api/skill-hub/{}/publish", namespace), form).await?;
        reply.into_result()
    }

    pub async fn download_bundle(&self, namespace: &str, slug: &str, version: &str) -> Result<bytes::Bytes> {
        self.download(&format!(
            "/api/skill-hub/{}/{}/versions/{}/download",
            namespace, slug, version
        ))
        .await
    }

    pub async fn get_skill_detail(&self, namespace: &str, slug: &str) -> Result<serde_json::Value> {
        self.get(&format!("/api/skill-hub/{}/{}", namespace, slug)).await
    }
}
