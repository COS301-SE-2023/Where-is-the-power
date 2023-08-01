use async_trait::async_trait;
use bson::Document;
use mongodb::{
    options::{FindOptions, UpdateModifications},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Cursor, Database,
};

#[async_trait]
pub trait Entity {
    async fn insert(&self, db: &Database) -> Result<InsertOneResult, mongodb::error::Error>;
    async fn delete(self, db: &Database) -> Result<DeleteResult, mongodb::error::Error>;

    async fn find(
        filter: Document,
        db: &Database,
        options: Option<FindOptions>,
    ) -> Result<Vec<Box<Self>>, mongodb::error::Error>;

    async fn find_one(
        filter: Document,
        db: &Database,
        options: Option<FindOptions>,
    ) -> Option<Box<Self>>;

    #[deprecated(note = "Use `Entity::find` instead.")]
    async fn query(filter: Document, db: &Database) -> Result<Cursor<Self>, mongodb::error::Error>
    where
        Self: Sized;

    async fn update(
        &mut self,
        update: UpdateModifications,
        db: &Database,
    ) -> Result<UpdateResult, mongodb::error::Error>;

    // TODO: update method

    //     async fn find(
    //         db: &Database,
    //         filter: impl Into<Option<Document>>,
    //     ) -> Result<Cursor<Self::Output>, mongodb::error::Error>;
    //
    //     async fn find_one(
    //         db: &Database,
    //         filter: impl Into<Option<Document>>,
    //     ) -> Result<Option<Self::Output>, mongodb::error::Error>;
}
