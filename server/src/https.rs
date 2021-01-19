//! Redirects to https when http request is incoming.

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header;
use actix_web::HttpResponse;
use futures::future::{ok, Either, Ready};
use futures::task::{Context, Poll};

pub struct RedirectHttps {
    http_port: u16,
    https_port: u16,
    redirect: bool,
}

impl RedirectHttps {
    pub fn new(http_port: u16, https_port: u16, redirect: bool) -> RedirectHttps {
        RedirectHttps {
            http_port,
            https_port,
            redirect,
        }
    }
}

impl<S, B> Transform<S> for RedirectHttps
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Transform = RedirectHttpsMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RedirectHttpsMiddleware {
            service,
            http_port: self.http_port.to_string(),
            https_port: self.https_port.to_string(),
            redirect: self.redirect,
        })
    }
}

pub struct RedirectHttpsMiddleware<S> {
    service: S,
    http_port: String,
    https_port: String,
    redirect: bool,
}

impl<S, B> Service for RedirectHttpsMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        if req.connection_info().scheme() == "https" || !self.redirect {
            Either::Left(self.service.call(req))
        } else {
            let host = req
                .connection_info()
                .host()
                .to_owned()
                .replacen(&*self.http_port, &*self.https_port, 1);
            let uri = req.uri().to_owned();
            let url = format!("https://{}{}", host, uri);
            Either::Right(ok(req.into_response(
                HttpResponse::MovedPermanently()
                    .header(header::LOCATION, url)
                    .finish()
                    .into_body(),
            )))
        }
    }
}
