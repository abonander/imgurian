#[macro_use]
extern crate anterofit;

use anterofit::{Adapter, JsonAdapter};

use anterofit::net::intercept::AddHeader;

use anterofit::hyper::header::{Authorization, Bearer};

const ENDPOINT: &'static str = "https://api.imgur.com/3";

const AUTH_ENDPOINT: &'static str = "https://api.imgur.com/oauth2";

pub struct AnonymousClient {
    adapter: JsonAdapter,
}

pub struct AuthorizedClient {
    adapter: JsonAdapter,
}

pub fn anonymous<I: ToString>(client_id: I) -> AnonymousClient {
    let adapter = Adapter::builder()
        .base_url(ENDPOINT.parse().expect("BASE_URL malformed"))
        .serialize_json()
        .interceptor(AddHeader(Authorization(client_id.to_string())))
        .build();

    AnonymousClient {
        adapter: adapter,
    }
}

pub fn authorized<T: ToString>(token: T) -> AuthorizedClient {
    let adapter = Adapter::builder()
        .base_url(ENDPOINT.parse().expect("BASE_URL malformed"))
        .serialize_json()
        .interceptor(AddHeader(Authorization(Bearer { token: token.to_string() })))
        .build();

    AuthorizedClient {
        adapter: adapter
    }
}