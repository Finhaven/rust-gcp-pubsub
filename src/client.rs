use crate::{Error, BAD_TOKEN};
use crate::Topic;
use goauth::auth::JwtClaims;
use goauth::credentials::Credentials;
use goauth::scopes::Scope;
use smpl_jwt::Jwt;
use surf::http::Method;
use surf::{Url, RequestBuilder};
use goauth::GoErr;

#[derive(Clone, Debug)]
pub struct Client {
    credentials: Credentials,
    access_token: Option<String>,
}

impl Client {
    pub fn new(credentials: Credentials) -> Self {
        Client {
            access_token: None,
            credentials,
        }
    }

    pub fn base_request(&self, method: Method, url: &str) -> Result<RequestBuilder, Error> {
        let parsed_url = Url::parse(url).unwrap();
        let bearer_token = format!("Bearer {}", self.access_token.clone().unwrap_or_else(|| BAD_TOKEN.into()));
        if bearer_token == *BAD_TOKEN {
            return Err(Error::PubSub(BAD_TOKEN.to_string()));
        }
        Ok(surf::Request::builder(method, parsed_url).header("Authorization", bearer_token))
    }

    pub fn topic(&self, name: &str) -> Topic {
        Topic::new(self.clone(), name)
    }

    pub fn project(&self) -> String {
        self.credentials.project()
    }

    pub async fn refresh_token(&mut self) -> Result<(), Error> {
        match self.get_token().await {
            Ok(token) => {
                self.access_token = Some(token.access_token().to_owned());
                Ok(())
            }
            Err(e) => {
                Err(Error::from(e))
            },
        }
    }

    async fn get_token(&mut self) -> Result<goauth::auth::Token, GoErr> {
        let claims = JwtClaims::new(
            self.credentials.iss(),
            &Scope::PubSub,
            self.credentials.token_uri(),
            None,
            None,
        );
        let jwt = Jwt::new(claims, self.credentials.rsa_key().unwrap(), None);
        goauth::get_token(&jwt, &self.credentials).await
    }
}
