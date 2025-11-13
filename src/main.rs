use crate::app::App;

pub(crate) mod api_models;
mod app;

#[tokio::main]
async fn main() {
    let a = App::default();
    a.run().await;

    println!("{:?}", a);
}
