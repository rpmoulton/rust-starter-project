pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users;
mod m20250725_153600_create_roles;
mod m20250725_153700_create_abilities;
mod m20250725_153800_create_user_roles;
mod m20250725_153900_create_role_abilities;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users::Migration),
            Box::new(m20250725_153600_create_roles::Migration),
            Box::new(m20250725_153700_create_abilities::Migration),
            Box::new(m20250725_153800_create_user_roles::Migration),
            Box::new(m20250725_153900_create_role_abilities::Migration),
        ]
    }
}
