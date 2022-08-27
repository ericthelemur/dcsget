use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create()
            .table(Game::Table).if_not_exists()
            .col(ColumnDef::new(Game::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(Game::Title).string().not_null())
            .col(ColumnDef::new(Game::Description).string().not_null())
            .col(ColumnDef::new(Game::Version).string().not_null())
            .col(ColumnDef::new(Game::ImageURL).string())
            .col(ColumnDef::new(Game::Archive).string().not_null())
            .to_owned(),
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Game::Table).to_owned()).await
    }
}

#[derive(Iden)]
enum Game {
    Table,
    Id,
    Title,
    Description,
    Version,
    #[iden="image_url"]
    ImageURL,
    Archive,
}
