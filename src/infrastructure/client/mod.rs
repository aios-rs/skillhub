use crate::domain::error::{DomainError, DomainResult};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::sync::{Arc, RwLock};

use crate::application::dto::{
    ApiReply, HubNamespaceDto, HubSearchResultDto, HubSkillDto, HubSkillFileDto,
    HubSkillVersionDto, HubStatsDto, LabelDto, NotificationDto, NotificationPreferenceDto,
    PublishResultDto, SkillSummaryDto,
};

pub struct SkillHubClient {
    client: Client,
    base_url: Arc<String>,
    token: Arc<RwLock<Option<String>>>,
}

impl SkillHubClient {
    pub fn new(base_url: String, token: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: Arc::new(base_url),
            token: Arc::new(RwLock::new(token)),
        }
    }

    pub fn set_token(&self, token: String) {
        if let Ok(mut guard) = self.token.write() {
            *guard = Some(token);
        }
    }

    pub fn has_token(&self) -> bool {
        self.token.read().map(|g| g.is_some()).unwrap_or(false)
    }

    fn require_auth(&self) -> DomainResult<()> {
        if self.token.read().map(|g| g.is_none()).unwrap_or(true) {
            return Err(DomainError::NotAuthenticated);
        }
        Ok(())
    }

    fn add_auth(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Ok(guard) = self.token.read() {
            if let Some(token) = guard.as_ref() {
                return req.header("Authorization", format!("Bearer {}", token));
            }
        }
        req
    }

    async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> DomainResult<T> {
        let url = format!("{}{}", self.base_url, path);
        let req = self.client.get(&url);
        let req = self.add_auth(req);
        let resp = req.send().await.map_err(|e| DomainError::Http(e.to_string()))?;
        self.handle_response(resp).await
    }

    async fn post<T: for<'de> Deserialize<'de>, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> DomainResult<T> {
        let url = format!("{}{}", self.base_url, path);
        let req = self.client.post(&url);
        let req = self.add_auth(req);
        let resp = req.json(body).send().await.map_err(|e| DomainError::Http(e.to_string()))?;
        self.handle_response(resp).await
    }

    async fn put<T: for<'de> Deserialize<'de>, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> DomainResult<T> {
        let url = format!("{}{}", self.base_url, path);
        let req = self.client.put(&url);
        let req = self.add_auth(req);
        let resp = req.json(body).send().await.map_err(|e| DomainError::Http(e.to_string()))?;
        self.handle_response(resp).await
    }

    async fn delete(&self, path: &str) -> DomainResult<()> {
        let url = format!("{}{}", self.base_url, path);
        let req = self.client.delete(&url);
        let req = self.add_auth(req);
        let resp = req.send().await.map_err(|e| DomainError::Http(e.to_string()))?;
        let status = resp.status();
        if status != StatusCode::OK {
            let text = resp.text().await.unwrap_or_default();
            return Err(DomainError::Api(status.as_u16() as i32, text));
        }
        Ok(())
    }

    async fn download(&self, path: &str) -> DomainResult<Vec<u8>> {
        self.require_auth()?;
        let url = format!("{}{}", self.base_url, path);
        let req = self.client.get(&url);
        let req = self.add_auth(req);
        let resp = req.send().await.map_err(|e| DomainError::Http(e.to_string()))?;
        let status = resp.status();
        if status != StatusCode::OK {
            let text = resp.text().await.unwrap_or_default();
            return Err(DomainError::Api(status.as_u16() as i32, text));
        }
        Ok(resp.bytes().await?.to_vec())
    }

    async fn post_multipart<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> DomainResult<T> {
        self.require_auth()?;
        let url = format!("{}{}", self.base_url, path);
        let req = self.client.post(&url);
        let req = self.add_auth(req);
        let resp = req.multipart(form).send().await.map_err(|e| DomainError::Http(e.to_string()))?;
        self.handle_response(resp).await
    }

    async fn handle_response<T: for<'de> Deserialize<'de>>(
        &self,
        resp: reqwest::Response,
    ) -> DomainResult<T> {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status != StatusCode::OK {
            return Err(DomainError::Api(status.as_u16() as i32, text));
        }
        let reply: ApiReply<T> = serde_json::from_str(&text)
            .map_err(|e| DomainError::Parse(format!("Failed to parse response: {}", e)))?;
        reply
            .into_result()
            .map_err(|msg| DomainError::Api(-1, msg))
    }

    // ===== Skill endpoints =====

    pub async fn search(
        &self,
        query: Option<&str>,
        namespace: Option<&str>,
        labels: &[String],
        sort: &str,
        page: i32,
        page_size: i32,
    ) -> DomainResult<HubSearchResultDto> {
        let mut path = format!(
            "/api/skill-hub/search?page={}&page_size={}&sort={}",
            page, page_size, sort
        );
        if let Some(q) = query {
            path.push_str(&format!("&q={}", urlencoding::encode(q)));
        }
        if let Some(ns) = namespace {
            path.push_str(&format!("&namespace={}", urlencoding::encode(ns)));
        }
        for label in labels {
            path.push_str(&format!("&labels={}", urlencoding::encode(label)));
        }
        self.get(&path).await
    }

    pub async fn get_stats(&self) -> DomainResult<HubStatsDto> {
        self.get("/api/skill-hub/stats").await
    }

    pub async fn get_skill_detail(&self, namespace: &str, slug: &str) -> DomainResult<Option<HubSkillDto>> {
        self.get(&format!("/api/skill-hub/{}/{}", namespace, slug)).await
    }

    pub async fn list_versions(
        &self,
        namespace: &str,
        slug: &str,
        page: i32,
        page_size: i32,
    ) -> DomainResult<Vec<HubSkillVersionDto>> {
        self.get(&format!(
            "/api/skill-hub/{}/{}/versions?page={}&page_size={}",
            namespace, slug, page, page_size
        ))
        .await
    }

    pub async fn get_version(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
    ) -> DomainResult<Option<HubSkillVersionDto>> {
        self.get(&format!(
            "/api/skill-hub/{}/{}/versions/{}",
            namespace, slug, version
        ))
        .await
    }

    pub async fn list_files(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
    ) -> DomainResult<Vec<HubSkillFileDto>> {
        self.get(&format!(
            "/api/skill-hub/{}/{}/versions/{}/files",
            namespace, slug, version
        ))
        .await
    }

    pub async fn publish(
        &self,
        namespace: &str,
        file_data: Vec<u8>,
        filename: &str,
        visibility: String,
    ) -> DomainResult<PublishResultDto> {
        let part = reqwest::multipart::Part::bytes(file_data)
            .file_name(filename.to_string())
            .mime_str("application/zip")?;
        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("visibility", visibility);
        self.post_multipart(&format!("/api/skill-hub/{}/publish", namespace), form)
            .await
    }

    pub async fn download_bundle(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
    ) -> DomainResult<Vec<u8>> {
        self.download(&format!(
            "/api/skill-hub/{}/{}/versions/{}/download",
            namespace, slug, version
        ))
        .await
    }

    pub async fn download_latest(&self, namespace: &str, slug: &str) -> DomainResult<Vec<u8>> {
        self.download(&format!("/api/skill-hub/{}/{}", namespace, slug))
            .await
    }

    pub async fn star_skill(&self, id: &str) -> DomainResult<()> {
        self.put(&format!("/api/skill-hub/skills/{}/star", id), &serde_json::json!({}))
            .await
    }

    pub async fn unstar_skill(&self, id: &str) -> DomainResult<()> {
        self.delete(&format!("/api/skill-hub/skills/{}/star", id))
            .await
    }

    pub async fn rate_skill(&self, id: &str, score: i16) -> DomainResult<()> {
        self.put(
            &format!("/api/skill-hub/skills/{}/rating", id),
            &serde_json::json!({ "score": score }),
        )
        .await
    }

    pub async fn list_my_skills(&self, page: i32, page_size: i32) -> DomainResult<Vec<SkillSummaryDto>> {
        self.get(&format!(
            "/api/skill-hub/me/skills?page={}&page_size={}",
            page, page_size
        ))
        .await
    }

    pub async fn list_my_stars(&self, page: i32, page_size: i32) -> DomainResult<Vec<SkillSummaryDto>> {
        self.get(&format!(
            "/api/skill-hub/me/stars?page={}&page_size={}",
            page, page_size
        ))
        .await
    }

    // ===== Namespace endpoints =====

    pub async fn list_namespaces(&self) -> DomainResult<Vec<HubNamespaceDto>> {
        self.get("/api/skill-hub/namespaces").await
    }

    // ===== Label endpoints =====

    pub async fn list_labels(&self) -> DomainResult<Vec<LabelDto>> {
        self.get("/api/skill-hub/labels").await
    }

    // ===== Notification endpoints =====

    pub async fn list_notifications(
        &self,
        page: i32,
        page_size: i32,
    ) -> DomainResult<Vec<NotificationDto>> {
        self.get(&format!(
            "/api/skill-hub/notifications?page={}&page_size={}",
            page, page_size
        ))
        .await
    }

    pub async fn get_unread_notification_count(&self) -> DomainResult<i64> {
        self.get("/api/skill-hub/notifications/unread-count")
            .await
    }

    pub async fn mark_notification_read(&self, id: &str) -> DomainResult<()> {
        self.post(&format!("/api/skill-hub/notifications/{}/read", id), &())
            .await
    }

    pub async fn mark_all_notifications_read(&self) -> DomainResult<()> {
        self.post("/api/skill-hub/notifications/read-all", &()).await
    }

    pub async fn delete_notification(&self, id: &str) -> DomainResult<()> {
        self.delete(&format!("/api/skill-hub/notifications/{}", id))
            .await
    }

    pub async fn get_notification_preferences(&self) -> DomainResult<Vec<NotificationPreferenceDto>> {
        self.get("/api/skill-hub/notifications/preferences").await
    }

    // ===== Token endpoints =====

    pub async fn list_tokens(&self, page: i32, page_size: i32) -> DomainResult<Vec<serde_json::Value>> {
        self.get(&format!("/api/skill-hub/tokens?page={}&page_size={}", page, page_size))
            .await
    }

    pub async fn delete_token(&self, id: &str) -> DomainResult<()> {
        self.delete(&format!("/api/skill-hub/tokens/{}", id)).await
    }

    // ===== User endpoints =====

    pub async fn get_user_profile(&self) -> DomainResult<serde_json::Value> {
        self.get("/api/skill-hub/user/profile").await
    }

    // ===== Auth endpoints =====

    pub async fn login(&self, username: &str, password: &str) -> DomainResult<String> {
        let url = format!("{}/api/auth/login", self.base_url);
        let body = serde_json::json!({
            "username": username,
            "password": password
        });
        let resp = self.client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| DomainError::Http(e.to_string()))?;

        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();

        if status != StatusCode::OK {
            return Err(DomainError::Api(status.as_u16() as i32, text));
        }

        // Parse response to get token
        let reply: ApiReply<serde_json::Value> = serde_json::from_str(&text)
            .map_err(|e| DomainError::Parse(format!("Failed to parse response: {}", e)))?;

        let data = reply.into_result()
            .map_err(|msg| DomainError::Api(-1, msg))?;

        let token = data.get("token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| DomainError::Parse("Missing token in response".to_string()))?;

        Ok(token.to_string())
    }

    pub async fn login_with_app(&self, app_id: &str, app_secret: &str) -> DomainResult<String> {
        let url = format!("{}/api/auth/app-login", self.base_url);
        let body = serde_json::json!({
            "app_id": app_id,
            "app_secret": app_secret
        });
        let resp = self.client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| DomainError::Http(e.to_string()))?;

        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();

        if status != StatusCode::OK {
            return Err(DomainError::Api(status.as_u16() as i32, text));
        }

        let reply: ApiReply<serde_json::Value> = serde_json::from_str(&text)
            .map_err(|e| DomainError::Parse(format!("Failed to parse response: {}", e)))?;

        let data = reply.into_result()
            .map_err(|msg| DomainError::Api(-1, msg))?;

        let token = data.get("token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| DomainError::Parse("Missing token in response".to_string()))?;

        Ok(token.to_string())
    }
}
