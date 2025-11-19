pub mod config;
pub mod handlers;
pub mod middleware;

use axum::{Router, routing::get};

use crate::api_models::{ContentType, to_response};
use crate::app::middleware::{LogLayer, default_log_settings};
use crate::app::{config::Config, handlers::handle_info};

#[derive(Default, Debug, Clone)]
pub(crate) struct App {
    cfg: Config,
}

impl App {
    pub(crate) async fn build(&self) -> Result<Router<()>, ()> {
        let r = Router::<()>::new();
        self.configure_logging(self.build_routes(r))
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

    fn configure_logging(&self, r: Router<()>) -> Result<Router<()>, ()> {
        let Config { log_level, .. } = self.cfg;
        let log_settings = default_log_settings();
        match log_settings.apply() {
            Ok(_) => Ok(r.layer(LogLayer::new(log_level))),
            Err(e) => {
                println!("log setup failed:{e}");
                Err(())
            }
        }
    }
}
