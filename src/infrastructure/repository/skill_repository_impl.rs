use crate::domain::{
    entity::skill::{
        HubStats, PublishResult, Skill, SkillFile, SkillLabel, SkillLifecycleMutationResponse, SkillVersion,
    },
    error::DomainResult,
    repository::skill_repository::{SearchResult, SkillRepository, SearchParams},
    value_object::visibility::Visibility,
};
use crate::infrastructure::repository::ClientRef;
use async_trait::async_trait;

pub struct SkillRepositoryImpl {
    client: ClientRef,
}

impl SkillRepositoryImpl {
    pub fn new(client: ClientRef) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SkillRepository for SkillRepositoryImpl {
    async fn search(&self, params: SearchParams) -> DomainResult<SearchResult> {
        let dto = self
            .client
            .search(
                params.query.as_deref(),
                params.namespace.as_deref(),
                &params.labels,
                &params.sort.to_string(),
                params.page as i32,
                params.page_size as i32,
            )
            .await?;

        let skills = dto
            .skills
            .into_iter()
            .map(|d| convert_skill_dto(d, &params))
            .collect();

        Ok(SearchResult {
            skills,
            total: dto.total,
            page: dto.page as u32,
            page_size: dto.page_size as u32,
        })
    }

    async fn get_detail(&self, namespace: &str, slug: &str) -> DomainResult<Option<Skill>> {
        let dto = self.client.get_skill_detail(namespace, slug).await?;
        Ok(dto.map(|d| convert_skill_dto(d, &SearchParams::default())))
    }

    async fn list_versions(
        &self,
        namespace: &str,
        slug: &str,
        page: u32,
        page_size: u32,
    ) -> DomainResult<Vec<SkillVersion>> {
        let dtos = self
            .client
            .list_versions(namespace, slug, page as i32, page_size as i32)
            .await?;

        Ok(dtos
            .into_iter()
            .map(|d| SkillVersion {
                id: d.id,
                skill_id: d.skill_id,
                version: d.version,
                status: d.status,
                changelog: d.changelog,
                file_count: d.file_count,
                total_size: d.total_size,
                published_at: d.published_at,
                bundle_ready: d.bundle_ready,
                download_ready: d.download_ready,
                create_by: d.create_by,
                create_at: d.create_at,
            })
            .collect())
    }

    async fn get_version(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
    ) -> DomainResult<Option<SkillVersion>> {
        let dto = self.client.get_version(namespace, slug, version).await?;
        Ok(dto.map(|d| SkillVersion {
            id: d.id,
            skill_id: d.skill_id,
            version: d.version,
            status: d.status,
            changelog: d.changelog,
            file_count: d.file_count,
            total_size: d.total_size,
            published_at: d.published_at,
            bundle_ready: d.bundle_ready,
            download_ready: d.download_ready,
            create_by: d.create_by,
            create_at: d.create_at,
        }))
    }

    async fn list_files(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
    ) -> DomainResult<Vec<SkillFile>> {
        let dtos = self.client.list_files(namespace, slug, version).await?;

        Ok(dtos
            .into_iter()
            .map(|d| SkillFile {
                id: d.id,
                version_id: d.version_id,
                file_path: d.file_path,
                file_size: d.file_size,
                content_type: d.content_type,
            })
            .collect())
    }

    async fn get_file_content(
        &self,
        _namespace: &str,
        _slug: &str,
        _version: &str,
        _path: &str,
    ) -> DomainResult<String> {
        // TODO: Implement via GET /api/skill-hub/{ns}/{slug}/versions/{v}/file?path=...
        Ok(String::new())
    }

    async fn publish(
        &self,
        namespace: &str,
        file_data: Vec<u8>,
        visibility: Visibility,
    ) -> DomainResult<PublishResult> {
        let dto = self
            .client
            .publish(namespace, file_data, "skill.zip", visibility.to_string())
            .await?;

        Ok(PublishResult {
            skill_id: dto.skill_id,
            namespace: dto.namespace,
            slug: dto.slug,
            version: dto.version,
            status: dto.status,
            file_count: dto.file_count,
            total_size: dto.total_size,
        })
    }

    async fn download_bundle(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
    ) -> DomainResult<Vec<u8>> {
        self.client.download_bundle(namespace, slug, version).await
    }

    async fn download_latest(&self, namespace: &str, slug: &str) -> DomainResult<Vec<u8>> {
        self.client.download_latest(namespace, slug).await
    }

    async fn star(&self, skill_id: &str) -> DomainResult<()> {
        self.client.star_skill(skill_id).await
    }

    async fn unstar(&self, skill_id: &str) -> DomainResult<()> {
        self.client.unstar_skill(skill_id).await
    }

    async fn rate(&self, skill_id: &str, score: i16) -> DomainResult<()> {
        self.client.rate_skill(skill_id, score).await
    }

    async fn get_stats(&self) -> DomainResult<HubStats> {
        let dto = self.client.get_stats().await?;
        Ok(HubStats {
            total_skills: dto.total_skills,
            total_downloads: dto.total_downloads,
            total_namespaces: dto.total_namespaces,
            total_ratings: dto.total_ratings,
        })
    }

    async fn list_my_skills(&self, page: u32, page_size: u32) -> DomainResult<SearchResult> {
        let (dtos, total) = self
            .client
            .list_my_skills(page as i32, page_size as i32)
            .await?;

        let skills: Vec<Skill> = dtos
            .into_iter()
            .map(|d| Skill {
                id: d.id,
                tenant_id: String::new(),
                namespace_id: String::new(),
                namespace_slug: d.namespace_slug,
                slug: d.slug,
                display_name: d.display_name,
                summary: d.summary,
                owner_name: String::new(),
                owner_id: String::new(),
                visibility: Visibility::try_from(d.visibility.as_str()).unwrap_or(Visibility::Private),
                status: d.status,
                download_count: d.download_count,
                star_count: d.star_count,
                rating_avg: d.rating_avg,
                rating_count: d.rating_count,
                hidden: false,
                tags: Vec::new(),
                labels: Vec::new(),
                can_manage_lifecycle: false,
                can_submit_promotion: false,
                can_interact: false,
                can_report: false,
                created_at: String::new(),
                updated_at: d.updated_at,
                latest_version: d.latest_version,
            })
            .collect();

        Ok(SearchResult {
            skills,
            total,
            page,
            page_size,
        })
    }

    async fn list_my_stars(&self, page: u32, page_size: u32) -> DomainResult<SearchResult> {
        let (dtos, total) = self
            .client
            .list_my_stars(page as i32, page_size as i32)
            .await?;

        let skills: Vec<Skill> = dtos
            .into_iter()
            .map(|d| Skill {
                id: d.id,
                tenant_id: String::new(),
                namespace_id: String::new(),
                namespace_slug: d.namespace_slug,
                slug: d.slug,
                display_name: d.display_name,
                summary: d.summary,
                owner_name: String::new(),
                owner_id: String::new(),
                visibility: Visibility::try_from(d.visibility.as_str()).unwrap_or(Visibility::Private),
                status: d.status,
                download_count: d.download_count,
                star_count: d.star_count,
                rating_avg: d.rating_avg,
                rating_count: d.rating_count,
                hidden: false,
                tags: Vec::new(),
                labels: Vec::new(),
                can_manage_lifecycle: false,
                can_submit_promotion: false,
                can_interact: false,
                can_report: false,
                created_at: String::new(),
                updated_at: d.updated_at,
                latest_version: d.latest_version,
            })
            .collect();

        Ok(SearchResult {
            skills,
            total,
            page,
            page_size,
        })
    }

    async fn archive(
        &self,
        _namespace: &str,
        _slug: &str,
    ) -> DomainResult<SkillLifecycleMutationResponse> {
        // TODO: Implement
        Ok(SkillLifecycleMutationResponse {
            skill_id: String::new(),
            version_id: None,
            action: String::new(),
            new_status: String::new(),
        })
    }

    async fn unarchive(
        &self,
        _namespace: &str,
        _slug: &str,
    ) -> DomainResult<SkillLifecycleMutationResponse> {
        // TODO: Implement
        Ok(SkillLifecycleMutationResponse {
            skill_id: String::new(),
            version_id: None,
            action: String::new(),
            new_status: String::new(),
        })
    }

    async fn yank_version(
        &self,
        _namespace: &str,
        _slug: &str,
        _version: &str,
        _reason: &str,
    ) -> DomainResult<SkillLifecycleMutationResponse> {
        // TODO: Implement
        Ok(SkillLifecycleMutationResponse {
            skill_id: String::new(),
            version_id: None,
            action: String::new(),
            new_status: String::new(),
        })
    }

    async fn rerelease_version(
        &self,
        _namespace: &str,
        _slug: &str,
        _version: &str,
    ) -> DomainResult<SkillLifecycleMutationResponse> {
        // TODO: Implement
        Ok(SkillLifecycleMutationResponse {
            skill_id: String::new(),
            version_id: None,
            action: String::new(),
            new_status: String::new(),
        })
    }

    async fn delete_version(
        &self,
        _namespace: &str,
        _slug: &str,
        _version: &str,
    ) -> DomainResult<()> {
        // TODO: Implement
        Ok(())
    }

    async fn submit_review(
        &self,
        _namespace: &str,
        _slug: &str,
        _version: &str,
        _comment: Option<&str>,
    ) -> DomainResult<SkillLifecycleMutationResponse> {
        // TODO: Implement
        Ok(SkillLifecycleMutationResponse {
            skill_id: String::new(),
            version_id: None,
            action: String::new(),
            new_status: String::new(),
        })
    }

    async fn withdraw_review(
        &self,
        _namespace: &str,
        _slug: &str,
        _version: &str,
    ) -> DomainResult<SkillLifecycleMutationResponse> {
        // TODO: Implement
        Ok(SkillLifecycleMutationResponse {
            skill_id: String::new(),
            version_id: None,
            action: String::new(),
            new_status: String::new(),
        })
    }

    async fn confirm_publish(
        &self,
        _namespace: &str,
        _slug: &str,
        _version: &str,
    ) -> DomainResult<SkillLifecycleMutationResponse> {
        // TODO: Implement
        Ok(SkillLifecycleMutationResponse {
            skill_id: String::new(),
            version_id: None,
            action: String::new(),
            new_status: String::new(),
        })
    }

    async fn get_skill_labels(&self, _skill_id: &str) -> DomainResult<Vec<String>> {
        // TODO: Implement
        Ok(Vec::new())
    }

    async fn set_skill_labels(&self, _skill_id: &str, _label_ids: Vec<i64>) -> DomainResult<()> {
        // TODO: Implement
        Ok(())
    }

    async fn remove_label_from_skill(&self, _skill_id: &str, _label_id: i64) -> DomainResult<()> {
        // TODO: Implement
        Ok(())
    }
}

fn convert_skill_dto(dto: crate::application::dto::HubSkillDto, _params: &SearchParams) -> Skill {
    Skill {
        id: dto.id,
        tenant_id: dto.tenant_id,
        namespace_id: dto.namespace_id,
        namespace_slug: dto.namespace_slug,
        slug: dto.slug,
        display_name: dto.display_name,
        summary: dto.summary,
        owner_name: dto.owner_name,
        owner_id: dto.owner_id,
        visibility: Visibility::try_from(dto.visibility.as_str()).unwrap_or(Visibility::Private),
        status: dto.status,
        download_count: dto.download_count,
        star_count: dto.star_count,
        rating_avg: dto.rating_avg,
        rating_count: dto.rating_count,
        hidden: dto.hidden,
        tags: dto.tags,
        labels: dto
            .labels
            .into_iter()
            .map(|l| SkillLabel {
                id: l.id,
                slug: l.slug,
                display_name: l.display_name,
            })
            .collect(),
        can_manage_lifecycle: dto.can_manage_lifecycle,
        can_submit_promotion: dto.can_submit_promotion,
        can_interact: dto.can_interact,
        can_report: dto.can_report,
        created_at: dto.created_at,
        updated_at: dto.updated_at,
        latest_version: dto.latest_version,
    }
}
