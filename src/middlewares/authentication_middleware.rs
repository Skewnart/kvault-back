use crate::errors::app_request_error::AppRequestError;
use crate::models::config::jwt_config::JwtConfig;
use crate::models::token::Token;
use actix_web::body::EitherBody;
use actix_web::web::Data;
use actix_web::{
    Error, HttpMessage, HttpResponse, ResponseError,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures::future::Ready;
use futures_util::future::{LocalBoxFuture, ready};
use log::{error, info};
use std::rc::Rc;

pub struct AuthenticationMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthenticationMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthenticationMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthenticationMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        Box::pin(async move {
            let jwt_keys = match req.request().app_data::<Data<JwtConfig>>() {
                Some(jwt_keys) => jwt_keys,
                None => {
                    return {
                        Ok(req.into_response(
                            HttpResponse::InternalServerError()
                                .json(serde_json::json!({ "error": "Error from RSA keys" }))
                                .map_into_right_body(),
                        ))
                    };
                }
            };

            let authorization = req
                .headers()
                .get("authorization")
                .and_then(|h| h.to_str().ok());

            match Token::extract_bearer(authorization)
                .and_then(|token| {
                    Token::decode(token, jwt_keys.pk.clone())
                        .map_err(|_err| AppRequestError::InternalTokenError(_err.to_string()))
                })
                .and_then(Token::verify)
            {
                Ok(token) => {
                    // Store auth info in request extensions for generic use
                    req.extensions_mut().insert(token);

                    info!("MIDDLEWARE AUTHENTICATION OK");
                    let res = service.call(req).await?;
                    Ok(res.map_into_left_body())
                }
                Err(e) => {
                    error!("MIDDLEWARE AUTHENTICATION KO");
                    Ok(req.into_response(e.error_response().map_into_right_body()))
                }
            }
        })
    }
}
