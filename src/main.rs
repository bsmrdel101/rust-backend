use std::env;
use std::sync::Arc;

mod controllers;
mod modules {
    pub mod db;
}

use modules::db;


#[async_std::main]
async fn main() -> tide::Result<()> {
    let pool = db::init_pool().await?;
    let pool = Arc::new(pool);
    let mut app = tide::with_state(pool.clone());

    controllers::tasks::register(&mut app);

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("127.0.0.1:{}", port);
    println!("Server listening on http://{}", address);
    app.listen(address).await?;
    Ok(())
}
