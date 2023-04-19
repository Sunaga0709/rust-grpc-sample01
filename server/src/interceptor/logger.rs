use futures::{future::BoxFuture};
use std::task::{Context, Poll};
use tower::{Layer, Service};
use tonic::transport::NamedService;
use tracing::{info, error};

#[derive(Clone)]
pub struct Logger<S> {
    inner: S,
}

impl<S> Logger<S> {
    fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, Req> Service<Req> for Logger<S>
where
    S: Service<Req>,
    Req: std::fmt::Debug,
    S::Response: std::fmt::Debug,
	S::Error: std::fmt::Debug,
	S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Req) -> Self::Future {
		info!("{:?}", req);
		let fut = self.inner.call(req);
		Box::pin(async move {
            let result = fut.await;
            match &result {
                Ok(res) => {
                    info!("{:?}", res);
                }
                Err(err) => {
                    error!("{:?}", err);
                }
            }
            result
        })
	}
}

impl<S> NamedService for Logger<S>
where
    S: NamedService,
{
    const NAME: &'static str = S::NAME;
}

pub struct LoggingMiddlewareLayer;

impl<S> Layer<S> for LoggingMiddlewareLayer {
    type Service = Logger<S>;

    fn layer(&self, service: S) -> Self::Service {
        Logger::new(service)
    }
}
