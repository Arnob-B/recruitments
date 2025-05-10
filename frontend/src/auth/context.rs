use yew::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AuthContext {
    pub token: Option<String>,
    pub username: Option<String>,
}

impl Default for AuthContext {
    fn default() -> Self {
        if let Ok(auth) = LocalStorage::get::<Self>("auth") {
            auth
        } else {
            Self {
                token: None,
                username: None,
            }
        }
    }
}

#[derive(Debug)]
pub enum AuthAction {
    Login(String, String),
    Logout,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthContextHandle {
    pub inner: UseReducerHandle<AuthContext>,
}

impl AuthContextHandle {
    pub fn dispatch(&self, action: AuthAction) {
        match action {
            AuthAction::Login(token, username) => {
                let auth = AuthContext {
                    token: Some(token),
                    username: Some(username),
                };
                LocalStorage::set("auth", &auth).unwrap();
                self.inner.dispatch(auth);
            }
            AuthAction::Logout => {
                LocalStorage::delete("auth");
                self.inner.dispatch(AuthContext::default());
            }
        }
    }
    
    pub fn token(&self) -> Option<String> {
        self.inner.token.clone()
    }
    
    pub fn is_authenticated(&self) -> bool {
        self.inner.token.is_some()
    }
}