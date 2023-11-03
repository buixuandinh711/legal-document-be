use std::{
    future::{ready, Future},
    pin::Pin,
};

use actix_identity::Identity;
use actix_web::{
    dev::Payload,
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized},
    http::header::HeaderName,
    web, Error, FromRequest, HttpRequest,
};
use serde::Serialize;

use crate::{
    app_config::AppState,
    models::{
        officier_model::{self, PositionRole},
        ModelError,
    },
};

const HEADER_DIVISION_ID_KEY: HeaderName = HeaderName::from_static("division-id");
const HEADER_POSITION_INDEX_KEY: HeaderName = HeaderName::from_static("position-index");

#[derive(Serialize)]
pub struct AuthenticatedOfficer {
    pub address: String,
    pub division_id: String,
    pub position_index: i16,
    pub position_role: PositionRole,
}

impl FromRequest for AuthenticatedOfficer {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<AuthenticatedOfficer, Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let identity = Identity::from_request(req, pl).into_inner();
        if identity.is_err() {
            return Box::pin(ready(Err(ErrorBadRequest("Failed to extract identity"))));
        }
        let identity_id = identity.unwrap().id();
        if identity_id.is_err() {
            return Box::pin(ready(Err(ErrorInternalServerError("Failed to get id"))));
        }
        let officer_address = identity_id.unwrap();

        let headers = req.headers();
        if !headers.contains_key(HEADER_DIVISION_ID_KEY)
            || !headers.contains_key(HEADER_POSITION_INDEX_KEY)
        {
            return Box::pin(ready(Err(ErrorBadRequest(
                "Failed to get header auth info",
            ))));
        }

        let division_id = headers
            .get(HEADER_DIVISION_ID_KEY)
            .and_then(|div_id| div_id.to_str().ok());
        let position_index: Option<i16> = headers
            .get(HEADER_POSITION_INDEX_KEY)
            .and_then(|div_id| div_id.to_str().ok())
            .and_then(|str_div_id| str_div_id.parse().ok());
        if division_id.is_none() || position_index.is_none() {
            return Box::pin(ready(Err(ErrorBadRequest(
                "Failed to get header auth info",
            ))));
        }
        let division_id = division_id.unwrap().to_owned();
        let position_index = position_index.unwrap();

        let app_state: Result<web::Data<AppState>, Error> =
            web::Data::from_request(req, pl).into_inner();
        if app_state.is_err() {
            return Box::pin(ready(Err(ErrorInternalServerError(
                "Failed to get app state",
            ))));
        }

        Box::pin(async move {
            let client = app_state
                .unwrap()
                .db_pool
                .get()
                .await
                .map_err(|_err| ErrorInternalServerError("Failed to get db client"))?;

            let position_role = officier_model::validate_and_get_role(
                &client,
                &officer_address,
                &division_id,
                position_index,
            )
            .await;

            match position_role {
                Ok(role) => Ok(AuthenticatedOfficer {
                    address: officer_address,
                    division_id,
                    position_index,
                    position_role: role,
                }),
                Err(err) => match err {
                    ModelError::AuthError => Err(ErrorUnauthorized("Invalid authentication info")),
                    _ => Err(ErrorInternalServerError(
                        "Failed to validate authentication info",
                    )),
                },
            }
        })

        // let role = officier_model::validate_and_get_role(client, officer_address, division_onchain_id, position_index)
    }
}
