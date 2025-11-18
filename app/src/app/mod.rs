pub mod config;
pub mod handlers;
pub mod middleware;

use axum::{Router, routing::get};

use crate::api_models::{ContentType, to_response};
use crate::app::middleware::{LogLayer, setup_logger};
use crate::app::{config::Config, handlers::handle_info};

#[derive(Default, Debug, Clone)]
pub(crate) struct App {
    cfg: Config,
}

impl App {
    pub(crate) async fn build(&self) -> Result<Router<()>, ()> {
        setup_logger();
        let Config { log_level, .. } = self.cfg;
        let r = Router::<()>::new();
        Ok(self.build_routes(r).layer(LogLayer::new(log_level)))
    }

    fn build_routes(&self, r: Router<()>) -> Router {
        r.route(
            "/info",
            get(|| async { to_response(handle_info().await, ContentType::Json) }),
        )
    }

    pub(crate) async fn run(&self) {
        let r = self.build().await.unwrap();
        let listener = tokio::net::TcpListener::bind(self.cfg.interface)
            .await
            .unwrap();
        axum::serve(listener, r).await.unwrap()
    }
}
