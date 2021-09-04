use hyper::{Body, Request, Response, server::{ Server, conn::AddrStream }};
use std::future::Future;
use std::net::SocketAddr;
use std::convert::Infallible;

pub struct Opts<H, ErrF> {
    pub addr: SocketAddr,
    pub handler: H,
    pub on_error: ErrF
}

/// Start a Hyper server to handle incoming requests
pub async fn start_server<H, F, E, ErrF>(opts: Opts<H, ErrF>) -> Result<(), hyper::Error>
where
    H: Clone + Send + Sync + 'static + FnMut(Request<Body>, SocketAddr) -> F,
    F: Send + 'static + Future<Output = Result<Response<Body>, E>>,
    ErrF: Clone + Send + Sync + 'static + FnMut(&E), 
    E: IntoResponse + std::fmt::Debug + Send + Sync + 'static
{
    let addr = opts.addr;
    let handler = opts.handler;
    let on_error = opts.on_error;

    // Every new connection leads to this outer closure being called:
    let service = hyper::service::make_service_fn(move |addr: &AddrStream| {
        let handler = handler.clone();
        let on_error = on_error.clone();
        let addr = addr.remote_addr();
        async move {
            // Every new request leads to this inner closure being called:
            Ok::<_, Infallible>(hyper::service::service_fn(move |r| {
                let mut handler = handler.clone();
                let mut on_error = on_error.clone();
                async move {
                    // Call our handler function. Errors are converted into responses
                    // and returned, but also passed to on_error so that we can react
                    // if we like.
                    match handler(r, addr).await {
                        Ok(r) => Ok::<_, Infallible>(r),
                        Err(e) => {
                            on_error(&e);
                            Ok(e.into_response())
                        }
                    }
                }
            })) 
        }
    });
    let server = Server::bind(&addr).serve(service);

    server.await?;
    Ok(())
}

/// Implement this for any error type you'd like to return, so that we know how
/// to respond to the user/battlesnake server when something goes wrong.
pub trait IntoResponse {
    fn into_response(self) -> Response<Body>;
}

impl IntoResponse for anyhow::Error {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(500)
            .body(Body::from("Whoops; something went wrong on my end!"))
            .unwrap()
    }
}