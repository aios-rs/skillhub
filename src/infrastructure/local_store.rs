use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// 已安装技能信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledSkill {
    pub namespace: String,
    pub slug: String,
    pub version: String,
    pub installed_at: String,
    pub install_path: PathBuf,
    pub manifest_path: PathBuf,
    pub skill_type: SkillType,
}

/// 技能类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillType {
    /// JavaScript/Node.js 项目
    JavaScript,
    /// Rust 项目
    Rust,
    /// Python 项目
    Python,
    /// 通用压缩包
    Archive,
}

/// 本地存储管理器
pub struct LocalStore {
    config_dir: PathBuf,
    skills_dir: PathBuf,
    installed_file: PathBuf,
}

impl LocalStore {
    /// 创建新的本地存储实例
    pub fn new() -> io::Result<Self> {
        let config_dir = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?
            .join(".skillhub");

        // 创建必要的目录
        fs::create_dir_all(&config_dir)?;
        let skills_dir = config_dir.join("skills");
        fs::create_dir_all(&skills_dir)?;

        let installed_file = config_dir.join("installed.json");

        Ok(Self {
            config_dir,
            skills_dir,
            installed_file,
        })
    }

    /// 获取配置目录
    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    /// 获取技能目录
    pub fn skills_dir(&self) -> &Path {
        &self.skills_dir
    }

    /// 安装技能
    pub async fn install(
        &mut self,
        namespace: &str,
        slug: &str,
        version: &str,
        data: Vec<u8>,
    ) -> io::Result<PathBuf> {
        let skill_dir = self.skills_dir.join(namespace).join(slug);
        fs::create_dir_all(&skill_dir)?;

        // 解压到临时目录
        let temp_zip = skill_dir.join("temp.zip");
        fs::write(&temp_zip, &data)?;

        // 使用 unzip 或内置解压
        self.extract_zip(&temp_zip, &skill_dir)?;
        fs::remove_file(&temp_zip)?;

        // 检测技能类型
        let skill_type = self.detect_skill_type(&skill_dir)?;
        let manifest_path = self.find_manifest_path(&skill_dir, &skill_type)?;

        // 添加到已安装列表
        let installed_skill = InstalledSkill {
            namespace: namespace.to_string(),
            slug: slug.to_string(),
            version: version.to_string(),
            installed_at: chrono::Utc::now().to_rfc3339(),
            install_path: skill_dir.clone(),
            manifest_path: manifest_path.clone(),
            skill_type,
        };

        self.add_to_installed(installed_skill)?;

        Ok(skill_dir)
    }

    /// 卸载技能
    pub async fn uninstall(&mut self, namespace: &str, slug: &str) -> io::Result<()> {
        let skill_dir = self.skills_dir.join(namespace).join(slug);

        // 删除目录
        if skill_dir.exists() {
            fs::remove_dir_all(&skill_dir)?;
        }

        // 从已安装列表中移除
        self.remove_from_installed(namespace, slug)?;

        Ok(())
    }

    /// 列出已安装的技能
    pub fn list(&self) -> io::Result<Vec<InstalledSkill>> {
        if !self.installed_file.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.installed_file)?;
        let installed: Vec<InstalledSkill> = serde_json::from_str(&content)
            .unwrap_or_default();

        Ok(installed)
    }

    /// 更新技能
    pub async fn update(
        &mut self,
        namespace: &str,
        slug: &str,
        new_version: &str,
        data: Vec<u8>,
    ) -> io::Result<()> {
        // 先卸载旧版本
        self.uninstall(namespace, slug).await?;

        // 安装新版本
        self.install(namespace, slug, new_version, data).await?;

        Ok(())
    }

    /// 获取已安装的技能信息
    pub fn get_installed(&self, namespace: &str, slug: &str) -> Option<InstalledSkill> {
        self.list().ok()?.into_iter().find(|s| s.namespace == namespace && s.slug == slug)
    }

    /// 检测技能类型
    fn detect_skill_type(&self, dir: &Path) -> io::Result<SkillType> {
        // 检测 Rust 项目
        if dir.join("Cargo.toml").exists() {
            return Ok(SkillType::Rust);
        }

        // 检测 Node.js 项目
        if dir.join("package.json").exists() {
            return Ok(SkillType::JavaScript);
        }

        // 检测 Python 项目
        if dir.join("setup.py").exists() || dir.join("pyproject.toml").exists() {
            return Ok(SkillType::Python);
        }

        Ok(SkillType::Archive)
    }

    /// 查找清单文件路径
    fn find_manifest_path(&self, dir: &Path, skill_type: &SkillType) -> io::Result<PathBuf> {
        let path = match skill_type {
            SkillType::Rust => dir.join("Cargo.toml"),
            SkillType::JavaScript => dir.join("package.json"),
            SkillType::Python => {
                if dir.join("pyproject.toml").exists() {
                    dir.join("pyproject.toml")
                } else {
                    dir.join("setup.py")
                }
            }
            SkillType::Archive => dir.join("skill.json"),
        };

        if path.exists() {
            Ok(path)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Manifest file not found: {:?}", path),
            ))
        }
    }

    /// 解压 ZIP 文件
    fn extract_zip(&self, zip_path: &Path, dest_dir: &Path) -> io::Result<()> {
        // 检查 unzip 命令是否可用
        if std::process::Command::new("unzip")
            .arg("-q")
            .arg("-o")
            .arg(zip_path)
            .arg("-d")
            .arg(dest_dir)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return Ok(());
        }

        // 回退到内置解压（需要添加 zip 依赖）
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "unzip command not available. Please install unzip: apt install unzip or brew install unzip",
        ))
    }

    /// 添加到已安装列表
    fn add_to_installed(&mut self, skill: InstalledSkill) -> io::Result<()> {
        let mut installed = self.list().unwrap_or_default();

        // 移除同名技能（如果存在）
        installed.retain(|s| s.namespace != skill.namespace || s.slug != skill.slug);

        // 添加新安装的技能
        installed.push(skill);

        self.save_installed(&installed)
    }

    /// 从已安装列表中移除
    fn remove_from_installed(&mut self, namespace: &str, slug: &str) -> io::Result<()> {
        let mut installed = self.list().unwrap_or_default();
        installed.retain(|s| s.namespace != namespace || s.slug != slug);
        self.save_installed(&installed)
    }

    /// 保存已安装列表
    fn save_installed(&self, installed: &[InstalledSkill]) -> io::Result<()> {
        let content = serde_json::to_string_pretty(installed)?;
        fs::write(&self.installed_file, content)?;
        Ok(())
    }

    /// 检查 Rust 环境是否可用
    pub fn check_rust_environment(&self) -> io::Result<RustEnvironment> {
        let cargo = std::process::Command::new("cargo")
            .arg("--version")
            .output();

        let rustc = std::process::Command::new("rustc")
            .arg("--version")
            .output();

        Ok(RustEnvironment {
            cargo_available: cargo.as_ref().map(|o| o.status.success()).unwrap_or(false),
            cargo_version: cargo.ok().and_then(|o| {
                String::from_utf8(o.stdout).ok()
            }).and_then(|s| s.split_whitespace().nth(1).map(String::from)),
            rustc_available: rustc.as_ref().map(|o| o.status.success()).unwrap_or(false),
            rustc_version: rustc.ok().and_then(|o| {
                String::from_utf8(o.stdout).ok()
            }).and_then(|s| s.split_whitespace().nth(1).map(String::from)),
        })
    }

    /// 构建 Rust 项目
    pub fn build_rust_project(&self, skill_dir: &Path) -> io::Result<BuildResult> {
        use colored::Colorize;

        println!("{} {}", "🔍".cyan(), "Checking Rust environment...".bold());

        let env = self.check_rust_environment()?;

        if !env.cargo_available {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Cargo not found. Please install Rust: https://rustup.rs/"
            ));
        }

        if let Some(v) = &env.cargo_version {
            println!("  {} Cargo version: {}", "✅".green(), v);
        } else {
            println!("  {} Cargo available (version unknown)", "✅".green());
        }

        println!();
        println!("{} {}", "🔨".cyan(), "Building Rust project...".bold());
        println!("  {}", "This may take a few minutes...".dimmed());
        println!();

        let output = std::process::Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(skill_dir)
            .output()?;

        let success = output.status.success();

        if success {
            println!();
            println!("{} {}", "✅".green(), "Build successful!".bold());
            Ok(BuildResult {
                success: true,
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            })
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!();
            eprintln!("{} {}", "❌".red(), "Build failed!".bold());
            eprintln!("{}", stderr);
            Ok(BuildResult {
                success: false,
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: stderr.to_string(),
            })
        }
    }

    /// 配置国内镜像（Rust）
    pub fn setup_china_mirror(&self) -> io::Result<bool> {
        use colored::Colorize;

        let cargo_config = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?
            .join(".cargo/config");

        let mut needs_update = false;

        // 创建 .cargo 目录（如果不存在）
        if let Some(parent) = cargo_config.parent() {
            fs::create_dir_all(parent)?;
        }

        // 检查是否已配置
        let existing_config = cargo_config.exists()
            .then(|| fs::read_to_string(&cargo_config))
            .transpose()?;

        let has_mirror = existing_config
            .as_ref()
            .map(|c| c.contains("rsproxy.cn"))
            .unwrap_or(false);

        if !has_mirror {
            println!("{} {}", "🇨🇳".cyan(), "Detected China region, setting up Rust mirror...");

            let config_content = r#"[source.crates-io]
replace-with = 'rsproxy-sparse'

[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[net]
git-fetch-with-cli = true
"#;

            fs::write(&cargo_config, config_content)?;
            needs_update = true;

            println!("  {} Cargo mirror configured (rsproxy.cn)", "✅".green());
            println!("  {} Config file: {}", "📝".cyan(), cargo_config.display());
        }

        Ok(needs_update)
    }
}

/// Rust 环境信息
#[derive(Debug, Clone)]
pub struct RustEnvironment {
    pub cargo_available: bool,
    pub cargo_version: Option<String>,
    pub rustc_available: bool,
    pub rustc_version: Option<String>,
}

/// 构建结果
#[derive(Debug, Clone)]
pub struct BuildResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}
