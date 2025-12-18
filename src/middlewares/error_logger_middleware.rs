use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use futures_util::future::{LocalBoxFuture, Ready, ok};
use log::error;
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct ErrorLoggerMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ErrorLoggerMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: actix_web::body::MessageBody + std::fmt::Debug + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = ErrorLoggerMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ErrorLoggerMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct ErrorLoggerMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ErrorLoggerMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: actix_web::body::MessageBody + std::fmt::Debug + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        Box::pin(async move {
            let res = srv.call(req).await?;
            let status = res.status();
            if status.is_client_error() || status.is_server_error() {
                error!("HTTP error {}: {:?}", status, res.response().body());
            }
            Ok(res)
        })
    }
}
