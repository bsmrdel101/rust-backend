use std::env;

mod controllers;


#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    controllers::tasks::register(&mut app);

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("127.0.0.1:{}", port);
    println!("Server listening on http://{}", address);
    app.listen(address).await?;
    Ok(())
}
