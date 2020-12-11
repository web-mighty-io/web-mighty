use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{http, HttpResponse};
use futures::future::{ok, Either, Ready};
use futures::task::{Context, Poll};

/// Redirects to https when http request is incoming.
///
/// # Examples
///
/// ```no_run
/// use actix_web::{get, App, HttpServer, Responder};
/// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
/// use server::https::RedirectHttps;
///
/// #[get("/")]
/// async fn index() -> impl Responder {
///     format!("Hello, World!")
/// }
///
/// #[cfg(feature = "https")]
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
///     builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
///     builder.set_certificate_chain_file("cert.pem").unwrap();
///
///     HttpServer::new(move || App::new().wrap(RedirectHttps::new(8080, 8443)).service(index))
///         .bind("0.0.0.0:8080")?
///         .bind_openssl("0.0.0.0:8443", builder)?
///         .run()
///         .await
/// }
/// ```
pub struct RedirectHttps {
    http_port: u16,
    https_port: u16,
}

impl RedirectHttps {
    pub fn new(http_port: u16, https_port: u16) -> RedirectHttps {
        RedirectHttps { http_port, https_port }
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
        })
    }
}

pub struct RedirectHttpsMiddleware<S> {
    service: S,
    http_port: String,
    https_port: String,
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
        if req.connection_info().scheme() == "https" {
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
                    .header(http::header::LOCATION, url)
                    .finish()
                    .into_body(),
            )))
        }
    }
}
