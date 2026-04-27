use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::{web, http};


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

        let uri = request.path();

        println!("{uri}");


        if uri == "/" {
            let redirect : web::HttpResponse = web::HttpResponse::PermanentRedirect().header(http::header::LOCATION, "/static").finish();
            
            return Ok(request.into_response(redirect));    
        }

       return Ok(context.call(&self.service, request).await?);
    }
}
