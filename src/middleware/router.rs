use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::{http, web};


pub struct Https;

impl<S> Middleware<S> for Https {
    type Service = HttpsMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        HttpsMiddleware { service }
    }
}

pub struct HttpsMiddleware<S> {
    service: S,
}

impl<S, Err> Service<web::WebRequest<Err>> for HttpsMiddleware<S>
where
    S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
    Err: web::ErrorRenderer,
{
    type Response = web::WebResponse;
    type Error = web::Error;

    ntex::forward_ready!(service);

    async fn call(&self, request: web::WebRequest<Err>, context: ServiceCtx<'_, Self>) -> Result<Self::Response, Self::Error> {

        let connection_info = request.connection_info().clone();
        let req_type = connection_info.scheme();
        let host : &str = connection_info.host();
        let uri : String = request.uri().to_string();

        println!("{req_type}");

        return match req_type {
            "http" => {
                let new_route = format!("https://{}{}", host, uri);
                    Ok(request.into_response(web::HttpResponse::MovedPermanently().header(http::header::LOCATION, new_route).finish()))
            },
            _ => Ok(context.call(&self.service, request).await?)
        };

    }
}
