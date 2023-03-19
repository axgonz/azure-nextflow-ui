use crate::env::*;

use openidconnect::{
    AuthorizationCode,
    ClientId,
    ClientSecret,
    CsrfToken,
    Nonce,
    IssuerUrl,
    PkceCodeChallenge,
    PkceCodeVerifier,
    RedirectUrl,
    OAuth2TokenResponse, 
    reqwest::async_http_client,
    AccessToken,
    RefreshToken,
    Scope,
};

use openidconnect::core::{
  CoreAuthenticationFlow,
  CoreClient,
  CoreProviderMetadata,
};

use anyhow::anyhow;

use leptos::log;

use serde::{
    Serialize,
    Deserialize
};

use web_sys::{
    window,
    UrlSearchParams
};

const AUTH_STATE_SS_KEY: &str = "authState";
const AUTH_NONCE_SS_KEY: &str = "authNonce";
const AUTH_PKCE_SS_KEY: &str = "authPkce";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth {
    pub auth_url: Option<String>,
    pub access_token: Option<AccessToken>,
    pub refresh_token: Option<RefreshToken>,
    auth_state: Option<String>,
    auth_nonce: Option<String>,
    auth_pkce: Option<String>
}

/// https://docs.rs/openidconnect/2.5.1/openidconnect/
impl Auth {
    pub fn cont() -> Self {
        let mut me = Self {
            auth_url: None,
            access_token: None,
            refresh_token: None,
            auth_state: None,
            auth_nonce: None,
            auth_pkce: None,
        };
        me.get_proof();
        Self::remove_proof();
        return me
    }

    pub async fn begin(
        client_id: String, 
        client_secret: Option<String>,
        issuer_url: String,
        redirect_url: String,
    ) -> Self {
        // Use OpenID Connect Discovery to fetch the provider metadata.
        let provider_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new(issuer_url.clone()).unwrap(),
            async_http_client,
        )
        .await.unwrap();

        // Create an OpenID Connect client by specifying the client ID, client secret, authorization URL
        // and token URL. Set the URL the user will be redirected to after the authorization process.
        let client = CoreClient::from_provider_metadata(
            provider_metadata, 
            ClientId::new(client_id.clone()), 
            if client_secret.is_some() {
                Some(ClientSecret::new(client_secret.unwrap()))
            }
            else {
                None
            }
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url.clone()).unwrap());

        // Generate a PKCE challenge.
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate the full authorization URL. Set the desired scopes. Set the PKCE code challenge.
        let (auth_url, csrf_token, nonce) = client.authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .add_scope(Scope::new(API_SCOPE.to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        Self {
            auth_url: Some(auth_url.to_string()),
            access_token: None,
            refresh_token: None,
            auth_state: Some(serde_json::to_string(&csrf_token).unwrap()),
            auth_nonce: Some(serde_json::to_string(&nonce).unwrap()),
            auth_pkce: Some(serde_json::to_string(&pkce_verifier).unwrap()),
        } 
    }

    pub async fn complete(
        client_id: String, 
        client_secret: Option<String>,
        issuer_url: String,
        redirect_url: String,
    ) -> Result<Self, anyhow::Error> {
        let me = Self::cont();

        if me.auth_state.is_none() || me.auth_nonce.is_none() || me.auth_pkce.is_none() {
            log!("No verifiers");
            return Err(anyhow!("No verifiers"));
        }
    
        let csrf_token: CsrfToken = serde_json::from_str(&me.auth_state.clone().unwrap()).unwrap();
        let _nonce: Nonce = serde_json::from_str(&me.auth_nonce.clone().unwrap()).unwrap();
        let pkce_verifier: PkceCodeVerifier = serde_json::from_str(&me.auth_pkce.clone().unwrap()).unwrap();      

        if Self::code().is_none() || Self::state().is_none() {
            log!("Missing code or state");
            return Err(anyhow!("Missing code or state"));
        }

        let code = Self::code().unwrap();
        let state = Self::state().unwrap();

        let provider_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new(issuer_url.clone()).unwrap(),
            async_http_client,
        )
        .await.unwrap();

        let client = CoreClient::from_provider_metadata(
            provider_metadata, 
            ClientId::new(client_id.clone()),
            if client_secret.is_some() {
                Some(ClientSecret::new(client_secret.unwrap()))
            }
            else {
                None
            }
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url.clone()).unwrap());

        if Self::validate_state(csrf_token, state) {
            let token_response = client
                .exchange_code(AuthorizationCode::new(code))
                .set_pkce_verifier(pkce_verifier)
                .request_async(async_http_client)
                .await;
            
            match token_response {
                Ok(token_response) => {
                    /* WARNING

                        Code in this block will not work in wasm until crate openidconnect 
                         is at version "3.0.0". As a result this is here for the future and not 
                         used. It successfully compiles with version "2.5.1" but there are 
                         missing env imports when running in wasm.

                    // Extract the ID token claims after verifying its authenticity and nonce.
                    let id_token = token_response
                        .id_token()
                        .ok_or_else(|| anyhow!("Server did not return an ID token"))
                        .unwrap();                         
                         
                    let claims = id_token          
                        .claims(&client.id_token_verifier(), &nonce)
                        .unwrap();

                    // Verify the access token hash to ensure that the access token hasn't been substituted for
                    // another user's.
                    if let Some(expected_access_token_hash) = claims.access_token_hash() {
                        let actual_access_token_hash = AccessTokenHash::from_token(
                            token_response.access_token(),
                            &id_token.signing_alg().unwrap()
                        ).unwrap();
                        if actual_access_token_hash != *expected_access_token_hash {
                            log!("Invalid access token");
                            return Err(anyhow!("Invalid access token"))
                        }
                        else {
                            // The authenticated user's identity is now available. See the IdTokenClaims struct for a
                            // complete listing of the available claims.
                            log!(
                                "User {} with e-mail address {} has authenticated successfully",
                                claims.subject().as_str(),
                                claims.email().map(|email| email.as_str()).unwrap_or("<not provided>"),
                            );

                            log!("claims:\n{:#?}", claims);

                            return Ok(())
                        }
                    }
                    else {
                        log!("Bad token hash");
                        return Err(anyhow!("Bad token hash"));
                    } 
                    */  

                    return Ok(Self {
                        auth_url: None,
                        access_token: Some(token_response.access_token().clone()),
                        refresh_token: token_response.refresh_token().cloned(),
                        auth_state: None,
                        auth_nonce: None,
                        auth_pkce: None,
                    })
                }
                Err(error) => {
                    log!("Bad token result:\n{:#?}", error);
                    return Err(anyhow!("Bad token result"));
                }
            }         
        }
        else {
            log!("Invalid state");
            return Err(anyhow!("Invalid state"));
        }
    }

    fn validate_state(csrf_token: CsrfToken, state: String) -> bool {        
        if csrf_token.secret() == &state {
            log!("state and csrf_token match");
            return true
        }
        else {
            log!("state and csrf_token are different");
            return false
        }
    }

    pub fn set_proof(&self) {
        if let Ok(Some(storage)) = window().unwrap().session_storage() {
            // auth_state
            if storage.set_item(AUTH_STATE_SS_KEY, &self.auth_state.clone().unwrap()).is_err() {
                log!("Error while trying to set item in session storage");
            }
            // auth_nonce
            if storage.set_item(AUTH_NONCE_SS_KEY, &self.auth_nonce.clone().unwrap()).is_err() {
                log!("Error while trying to set item in session storage");
            }
            // auth_pkce
            if storage.set_item(AUTH_PKCE_SS_KEY, &self.auth_pkce.clone().unwrap()).is_err() {
                log!("Error while trying to set item in session storage");
            }
        }
        else {
            log!("Error while trying to set item from session storage");
        }
    }

    fn get_proof(&mut self) {
        if let Ok(Some(storage)) = window().unwrap().session_storage() {
            // auth_state
            match storage.get_item(AUTH_STATE_SS_KEY) {
                Ok(value) => self.auth_state = value,
                Err(_) => log!("Error while trying to get item from session storage")
            }
            // auth_nonce
            match storage.get_item(AUTH_NONCE_SS_KEY) {
                Ok(value) => self.auth_nonce = value,
                Err(_) => log!("Error while trying to get item from session storage")
            }
            // auth_pkce
            match storage.get_item(AUTH_PKCE_SS_KEY) {
                Ok(value) => self.auth_pkce = value,
                Err(_) => log!("Error while trying to get item from session storage")
            }                        
        } else {
            log!("Error while trying to get item from session storage");
        }
    }    

    pub fn remove_proof() {
        if let Ok(Some(storage)) = window().unwrap().session_storage() {
            // auth_state
            if storage.remove_item(AUTH_STATE_SS_KEY).is_err() {
                log!("Error while trying to remove item in session storage");
            }
            // auth_nonce
            if storage.remove_item(AUTH_NONCE_SS_KEY).is_err() {
                log!("Error while trying to remove item in session storage");
            }
            // auth_pkce
            if storage.remove_item(AUTH_PKCE_SS_KEY).is_err() {
                log!("Error while trying to remove item in session storage");
            }
        }
        else {
            log!("Error while trying to remove item from session storage");
        }
    }       

    fn code() -> Option<String> {
        if let Ok(search) = window().unwrap().location().search() {
            let params = UrlSearchParams::new_with_str(&search);
            return params.unwrap().get("code")
        };
        return None
    }  

    #[allow(dead_code)]
    pub fn has_code() -> bool {
        if Self::code().is_some() {
            return true
        }
        else {
            return false
        }
    }

    fn state() -> Option<String> {
        if let Ok(search) = window().unwrap().location().search() {
            let params = UrlSearchParams::new_with_str(&search);
            return params.unwrap().get("state")
        };
        return None
    }      

    #[allow(dead_code)]
    pub fn has_state() -> bool {
        if Self::state().is_some() {
            return true
        }
        else {
            return false
        }
    } 

    pub fn clear_access_token(&mut self) {
        self.access_token = None;
    }

    pub fn clear_refresh_token(&mut self) {
        self.refresh_token = None;
    }
}
