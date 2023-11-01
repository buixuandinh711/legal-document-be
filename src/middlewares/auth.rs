use actix_identity::Identity;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::HeaderName,
    Error, HttpResponse,
};
use actix_web_lab::middleware::Next;

pub async fn auth(
    identity: Option<Identity>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // if identity.is_none() {
    //     return Ok(ServiceResponse::from(
    //         HttpResponse::Unauthorized().body("Identity not exist"),
    //     ));
    // }
    // let officer_id = identity.as_ref().unwrap().id();
    // if officer_id.is_err() {
    //     return Err(HttpResponse::InternalServerError().body("Unable to get identity info"));
    // }
    // let officer_id: i64 = officer_id.unwrap().parse().unwrap();
    let headers = req.headers();
    if let Some(auth_header_value) = headers.get(HeaderName::from_static("division-id")) {
        log::info!("Get the header {}", auth_header_value.to_str().unwrap());
    } else {
        // custom auth header not found
    }
    next.call(req).await
}
