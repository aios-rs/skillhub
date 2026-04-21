pub mod auth_repository;
pub mod label_repository;
pub mod namespace_repository;
pub mod notification_repository;
pub mod promotion_repository;
pub mod report_repository;
pub mod review_repository;
pub mod skill_repository;
pub mod token_repository;
pub mod user_repository;

// Re-export traits for convenience
pub use auth_repository::AuthRepository;
pub use label_repository::LabelRepository;
pub use namespace_repository::NamespaceRepository;
pub use notification_repository::NotificationRepository;
pub use promotion_repository::PromotionRepository;
pub use report_repository::ReportRepository;
pub use review_repository::ReviewRepository;
pub use skill_repository::SkillRepository;
pub use token_repository::TokenRepository;
pub use user_repository::UserRepository;
