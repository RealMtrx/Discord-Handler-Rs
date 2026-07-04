use mongodb::bson::{doc, Document};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub points: i64,
}

pub async fn get_user(db: &Database, user_id: &str) -> Result<Option<User>, String> {
    let col = db.collection::<User>("users");
    let filter = doc! { "userId": user_id };
    let user = col
        .find_one(filter)
        .await
        .map_err(|e| e.to_string())?;
    Ok(user)
}

pub async fn create_user(db: &Database, user_id: &str) -> Result<User, String> {
    let col = db.collection::<User>("users");
    let user = User {
        user_id: user_id.to_string(),
        points: 0,
    };
    col.insert_one(&user)
        .await
        .map_err(|e| e.to_string())?;
    Ok(user)
}
