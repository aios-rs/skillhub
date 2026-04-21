#[derive(Debug, Clone)]
pub struct GetSkillDetailQuery {
    pub namespace: String,
    pub slug: String,
}

#[derive(Debug, Clone)]
pub struct ListVersionsQuery {
    pub namespace: String,
    pub slug: String,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Clone)]
pub struct ListFilesQuery {
    pub namespace: String,
    pub slug: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct DownloadBundleQuery {
    pub namespace: String,
    pub slug: String,
    pub version: String,
}
