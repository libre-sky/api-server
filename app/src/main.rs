use crate::app::App;

pub(crate) mod api_models;
mod app;
mod db;
pub(crate) mod models;
#[tokio::main]
async fn main() {
    let a = App::default();
    a.run().await;

    println!("{:?}", a);
}
