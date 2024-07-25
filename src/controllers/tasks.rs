use tide::{prelude::*, Body};
use tide::{Request, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Task {
    id: i16,
    name: String,
}


pub fn register(app: &mut tide::Server<()>) {
    app.at("/api/tasks")
        .get(get_all_tasks);
}

async fn get_all_tasks(_req: Request<()>) -> tide::Result {
    let tasks = vec![
        Task { id: 1, name: "Break the dishes".into() },
        Task { id: 2, name: "Sweap the lawn".into() },
    ];
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&tasks)?);
    Ok(res)
}
