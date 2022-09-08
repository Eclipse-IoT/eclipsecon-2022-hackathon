use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

pub struct HealthServer {
    port: u16,
}
impl HealthServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn run(&mut self) -> Result<(), anyhow::Error> {
        let addr = ([0, 0, 0, 0], self.port).into();
        let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(healthz)) });

        let server = Server::bind(&addr).serve(service);

        log::info!("Listening on http://{}", addr);

        server.await?;
        Ok(())
    }
}

async fn healthz(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (_, _) => Ok(Response::new(Body::from("{\"status\": \"OK\"}"))),
    }
}
