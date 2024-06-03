mod app;
mod modules;

use std::time::Duration;

use nidrs::externs::axum::{
    error_handling::HandleErrorLayer,
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    BoxError,
};
use nidrs::externs::tower::timeout::TimeoutLayer;
pub use nidrs::AppError;
pub use nidrs::AppResult;

#[nidrs::main]
fn main() {
    let app = nidrs::NidrsFactory::create(app::AppModule);

    let app = app.default_prefix("/api/{version}");
    let app = app.default_version("v1");

    let mut app = app.listen(3000);

    app.router = app.router.layer(
        nidrs::externs::tower::ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|error: BoxError| async move {
                if error.is::<nidrs::externs::tower::timeout::error::Elapsed>() {
                    Ok(StatusCode::REQUEST_TIMEOUT)
                } else {
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {error}"),
                    ))
                }
            }))
            .layer(TimeoutLayer::new(Duration::from_secs(5)))
            .layer(middleware::from_fn(auth)),
    );

    app.block();
}

#[derive(Clone, Debug)]
struct TestData {
    pub id: u64,
    pub username: String,
}

async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    // let auth_header = req.headers().get(http::header::AUTHORIZATION).and_then(|header| header.to_str().ok());

    // let auth_header = if let Some(auth_header) = auth_header {
    //     auth_header
    // } else {
    //     return Err(StatusCode::UNAUTHORIZED);
    // };

    println!("auth {:?}", req);

    req.extensions_mut().insert(TestData {
        id: 1,
        username: "foo".to_string(),
    });
    Ok(next.run(req).await)
}
