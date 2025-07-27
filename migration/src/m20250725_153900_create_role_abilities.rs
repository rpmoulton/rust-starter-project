use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RoleAbilities::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RoleAbilities::RoleId).uuid().not_null())
                    .col(ColumnDef::new(RoleAbilities::AbilityId).uuid().not_null())
                    .primary_key(Index::create().col(RoleAbilities::RoleId).col(RoleAbilities::AbilityId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-role_abilities-role_id")
                            .from(RoleAbilities::Table, RoleAbilities::RoleId)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-role_abilities-ability_id")
                            .from(RoleAbilities::Table, RoleAbilities::AbilityId)
                            .to(Abilities::Table, Abilities::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RoleAbilities::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Abilities {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum RoleAbilities {
    Table,
    RoleId,
    AbilityId,
}