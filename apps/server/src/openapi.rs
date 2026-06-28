use utoipa::OpenApi;

use iam::presentation::web::openapi::IamApiDoc;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path="/api/v1/iam", api=IamApiDoc),
    ),
    info(
        title="Golden Ocean API",
        version="1.0.0"
    )
)]
pub struct ApiDoc;
