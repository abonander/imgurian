use anterofit::{Adapter, JsonAdapter};

use std::fmt::{self, Display, Formatter, Write};

pub struct AuthUrl<'i, S> {
    resp_type: ResponseType,
    client_id: &'i str,
    state: Option<S>,
}

impl<'i, S: Display> Display for AuthUrl<'i, S> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(state) = self.state {
            write!(f,
                   "https://api.imgur.com/oauth2/authorize?client_id={}&response_type={}&state={}",
                   self.client_id, self.resp_type, state)
        } else {
            write!(f, "https://api.imgur.com/oauth2/authorize?client_id={}&response_type={}",
                   self.client_id, self.resp_type)
        }
    }
}

impl<'i, S> AuthUrl<'i, S> {
    pub fn with_state<S: Display>(self, state: S) -> AuthUrl<'i, S> {
        AuthUrlWithState {
            resp_type: self.resp_type,
            client_id: self.client_id,
            state: Some(state),
        }
    }
}

pub enum ResponseType {
    Code,
    Token,
    Pin
}

impl Display for ResponseType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::ResponseType::*;

        match *self {
            Code => "code",
            Token => "token",
            Pin => "pin"
        }
    }
}

pub enum NoState {}

impl Display for NoState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {}
    }
}

pub fn auth_url<'i, I>(auth_type: ResponseType, client_id: I) -> AuthUrl<'i, NoState>
where I: Into<Cow<'i, str>> {
    AuthUrl {
        resp_type: auth_type,
        client_id: client_id.into(),
        state: None
    }
}