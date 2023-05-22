use async_trait::async_trait;
use mongodb::{
    results::{DeleteResult, InsertOneResult},
    Database,
};

#[async_trait]
pub trait Entity {
    type Output;

    async fn insert(&self, db: &Database) -> Result<InsertOneResult, mongodb::error::Error>;
    async fn delete(self, db: &Database) -> Result<DeleteResult, mongodb::error::Error>;

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
