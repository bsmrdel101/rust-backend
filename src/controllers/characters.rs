use tide::{Body, Request, Response, StatusCode};
use sqlx::{FromRow, PgPool};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, FromRow)]
struct Character {
  id: i16,
  name: String,
}


type DbPool = Arc<PgPool>;

pub fn register(app: &mut tide::Server<DbPool>) {
  app.at("/api/characters").get(get_all_user_characters);
}

async fn get_all_user_characters(req: Request<DbPool>) -> tide::Result {
  let pool = req.state();
  let response: Vec<Character> = sqlx::query_as::<_, Character>("SELECT * FROM characters")
    .fetch_all(pool.as_ref())
    .await?;

  let mut res = Response::new(StatusCode::Ok);
  res.set_body(Body::from_json(&response)?);
  Ok(res)
}
