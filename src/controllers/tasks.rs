use tide::{prelude::*, Body, Request, Response, StatusCode};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Task {
  id: i16,
  name: String,
}


type DbPool = Arc<PgPool>;

pub fn register(app: &mut tide::Server<DbPool>) {
  app.at("/api/tasks").get(get_all_tasks);
}

async fn get_all_tasks(_: Request<DbPool>) -> tide::Result {
  let tasks = vec![
    Task { id: 1, name: "Break the dishes".into() },
    Task { id: 2, name: "Sweep the lawn".into() },
  ];

  let mut res = Response::new(StatusCode::Ok);
  res.set_body(Body::from_json(&tasks)?);
  Ok(res)
}
