#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataOauthClientData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    oauth_client_id: terrars::PrimField<std::string::String>,
}

struct DataOauthClient_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOauthClientData_>,
}

#[derive(Clone)]
pub struct DataOauthClient(std::rc::Rc<DataOauthClient_>);

impl DataOauthClient {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderCloudflare) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed_cors_origins` after provisioning.\nArray of allowed CORS origins."]
    pub fn allowed_cors_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_cors_origins", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe unique identifier for an OAuth client."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_name` after provisioning.\nHuman-readable name of the OAuth client."]
    pub fn client_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_uri` after provisioning.\nURL of the home page of the client."]
    pub fn client_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_uri_verification` after provisioning.\nClient URI domain control verification state."]
    pub fn client_uri_verification(&self) -> DataOauthClient_ClientUriVerificationRef {
        DataOauthClient_ClientUriVerificationRef::new(
            self.shared().clone(),
            format!("{}.client_uri_verification", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp when the OAuth client was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `grant_types` after provisioning.\nArray of OAuth grant types the client is allowed to use. `authorization_code` is required; `refresh_token` may be included optionally."]
    pub fn grant_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grant_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `has_rotated_secret` after provisioning.\nIndicates whether the client has a rotated secret that has not yet been deleted."]
    pub fn has_rotated_secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_rotated_secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logo_uri` after provisioning.\nURL of the client's logo."]
    pub fn logo_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logo_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oauth_client_id` after provisioning.\nThe unique identifier for an OAuth client."]
    pub fn oauth_client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth_client_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `optional_scopes` after provisioning.\nScopes that the authorizing user may decline during consent. Each value must also appear in `scopes`. The scopes `openid`, `offline`, and `offline_access` cannot be optional."]
    pub fn optional_scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.optional_scopes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_uri` after provisioning.\nURL that points to a privacy policy document."]
    pub fn policy_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `post_logout_redirect_uris` after provisioning.\nArray of allowed post-logout redirect URIs."]
    pub fn post_logout_redirect_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.post_logout_redirect_uris", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `promoted_at` after provisioning.\nTimestamp when the OAuth client was promoted to public visibility."]
    pub fn promoted_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.promoted_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redirect_uris` after provisioning.\nArray of allowed redirect URIs for the client."]
    pub fn redirect_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect_uris", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `response_types` after provisioning.\nArray of OAuth response types the client is allowed to use."]
    pub fn response_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nArray of OAuth scopes the client is allowed to request. Colon-delimited scopes are not accepted. Dot-delimited scopes are validated against available OAuth API scopes; simple identity scopes are allowed. Protocol scopes `offline_access` and `openid` are added or removed automatically based on `grant_types` and `response_types`."]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `token_endpoint_auth_method` after provisioning.\nThe authentication method the client uses at the token endpoint.\nAvailable values: \"none\", \"client_secret_basic\", \"client_secret_post\"."]
    pub fn token_endpoint_auth_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint_auth_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tos_uri` after provisioning.\nURL that points to a terms of service document."]
    pub fn tos_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tos_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the OAuth client was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `visibility` after provisioning.\nVisibility of the OAuth client.\nAvailable values: \"public\", \"private\"."]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }
}

impl terrars::Referable for DataOauthClient {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOauthClient { }

impl terrars::ToListMappable for DataOauthClient {
    type O = terrars::ListRef<DataOauthClientRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOauthClient_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_oauth_client".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOauthClient {
    pub tf_id: std::string::String,
    #[doc = "Account identifier tag."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "The unique identifier for an OAuth client."]
    pub oauth_client_id: terrars::PrimField<std::string::String>,
}

impl BuildDataOauthClient {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOauthClient {
        let out = DataOauthClient(std::rc::Rc::new(DataOauthClient_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOauthClientData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                oauth_client_id: self.oauth_client_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOauthClientRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOauthClientRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOauthClientRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed_cors_origins` after provisioning.\nArray of allowed CORS origins."]
    pub fn allowed_cors_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_cors_origins", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe unique identifier for an OAuth client."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_name` after provisioning.\nHuman-readable name of the OAuth client."]
    pub fn client_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_uri` after provisioning.\nURL of the home page of the client."]
    pub fn client_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_uri_verification` after provisioning.\nClient URI domain control verification state."]
    pub fn client_uri_verification(&self) -> DataOauthClient_ClientUriVerificationRef {
        DataOauthClient_ClientUriVerificationRef::new(
            self.shared().clone(),
            format!("{}.client_uri_verification", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp when the OAuth client was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `grant_types` after provisioning.\nArray of OAuth grant types the client is allowed to use. `authorization_code` is required; `refresh_token` may be included optionally."]
    pub fn grant_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grant_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `has_rotated_secret` after provisioning.\nIndicates whether the client has a rotated secret that has not yet been deleted."]
    pub fn has_rotated_secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_rotated_secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logo_uri` after provisioning.\nURL of the client's logo."]
    pub fn logo_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logo_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oauth_client_id` after provisioning.\nThe unique identifier for an OAuth client."]
    pub fn oauth_client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth_client_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `optional_scopes` after provisioning.\nScopes that the authorizing user may decline during consent. Each value must also appear in `scopes`. The scopes `openid`, `offline`, and `offline_access` cannot be optional."]
    pub fn optional_scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.optional_scopes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_uri` after provisioning.\nURL that points to a privacy policy document."]
    pub fn policy_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `post_logout_redirect_uris` after provisioning.\nArray of allowed post-logout redirect URIs."]
    pub fn post_logout_redirect_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.post_logout_redirect_uris", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `promoted_at` after provisioning.\nTimestamp when the OAuth client was promoted to public visibility."]
    pub fn promoted_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.promoted_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redirect_uris` after provisioning.\nArray of allowed redirect URIs for the client."]
    pub fn redirect_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect_uris", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `response_types` after provisioning.\nArray of OAuth response types the client is allowed to use."]
    pub fn response_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nArray of OAuth scopes the client is allowed to request. Colon-delimited scopes are not accepted. Dot-delimited scopes are validated against available OAuth API scopes; simple identity scopes are allowed. Protocol scopes `offline_access` and `openid` are added or removed automatically based on `grant_types` and `response_types`."]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `token_endpoint_auth_method` after provisioning.\nThe authentication method the client uses at the token endpoint.\nAvailable values: \"none\", \"client_secret_basic\", \"client_secret_post\"."]
    pub fn token_endpoint_auth_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint_auth_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tos_uri` after provisioning.\nURL that points to a terms of service document."]
    pub fn tos_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tos_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the OAuth client was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `visibility` after provisioning.\nVisibility of the OAuth client.\nAvailable values: \"public\", \"private\"."]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOauthClient_ClientUriVerification {}

impl DataOauthClient_ClientUriVerification { }

impl terrars::ToListMappable for DataOauthClient_ClientUriVerification {
    type O = terrars::BlockAssignable<DataOauthClient_ClientUriVerification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOauthClient_ClientUriVerification {}

impl BuildDataOauthClient_ClientUriVerification {
    pub fn build(self) -> DataOauthClient_ClientUriVerification {
        DataOauthClient_ClientUriVerification {}
    }
}

#[allow(dead_code)]
pub struct DataOauthClient_ClientUriVerificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOauthClient_ClientUriVerificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOauthClient_ClientUriVerificationRef {
        DataOauthClient_ClientUriVerificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOauthClient_ClientUriVerificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nCurrent verification status for the client URI host.\nAvailable values: \"pending\", \"in_progress\", \"verified\", \"failed\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `text` after provisioning.\nExact TXT record value that must be added to DNS to prove ownership of the client URI host."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
