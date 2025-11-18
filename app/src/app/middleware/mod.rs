use axum::{extract::Request, response::Response};
use futures_util::future::BoxFuture;
use log::{Level, debug, info};
use std::task::{Context, Poll};
use tower::{Layer, Service};

pub(crate) mod colors;
pub(crate) use colors::setup_logger;
#[derive(Clone)]
pub(crate) struct LogLayer {
    log_level: Level,
}

impl<S> Layer<S> for LogLayer {
    type Service = LogMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        let Self { log_level } = self;
        let log_level = log_level.clone();
        LogMiddleware { inner, log_level }
    }
}

impl LogLayer {
    pub fn new(log_level: Level) -> Self {
        Self { log_level }
    }
}

#[derive(Clone)]
pub(crate) struct LogMiddleware<S> {
    log_level: Level,
    inner: S,
}

impl<S> Service<Request> for LogMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;

    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        self.log_request(&request);
        let future = self.inner.call(request);

        Box::pin(async move {
            let response = future.await?;
            //self.log_response(&response);
            Ok(response)
        })
    }
}

impl<S> LogMiddleware<S> {
    fn log_request(&self, request: &Request) {
        if self.log_level < Level::Info {
            return;
        }
        info!("request: {}\t{}", request.method(), request.uri().path());
    }

    #[allow(dead_code)]
    fn log_response(&self, response: &Response) {
        if self.log_level < Level::Debug {
            return;
        }
        debug!(
            "response: {} ({:?})",
            response.status(),
            response.headers().get("content-type")
        )
    }
}
