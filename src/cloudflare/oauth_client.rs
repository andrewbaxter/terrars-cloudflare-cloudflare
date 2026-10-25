#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct OauthClientData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_cors_origins: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    client_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    grant_types: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logo_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optional_scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_logout_redirect_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    redirect_uris: terrars::ListField<terrars::PrimField<std::string::String>>,
    response_types: terrars::ListField<terrars::PrimField<std::string::String>>,
    scopes: terrars::ListField<terrars::PrimField<std::string::String>>,
    token_endpoint_auth_method: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tos_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visibility: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct OauthClient_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OauthClientData_>,
}

#[derive(Clone)]
pub struct OauthClient(std::rc::Rc<OauthClient_>);

impl OauthClient {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc = "Set the field `allowed_cors_origins`.\nArray of allowed CORS origins."]
    pub fn set_allowed_cors_origins(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().allowed_cors_origins = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_uri`.\nURL of the home page of the client."]
    pub fn set_client_uri(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().client_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logo_uri`.\nURL of the client's logo."]
    pub fn set_logo_uri(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().logo_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_client_id`.\nThe unique identifier for an OAuth client."]
    pub fn set_oauth_client_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().oauth_client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `optional_scopes`.\nScopes that the authorizing user may decline during consent. Each value must also appear in `scopes`. The scopes `openid`, `offline`, and `offline_access` cannot be optional."]
    pub fn set_optional_scopes(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().optional_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_uri`.\nURL that points to a privacy policy document."]
    pub fn set_policy_uri(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().policy_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `post_logout_redirect_uris`.\nArray of allowed post-logout redirect URIs."]
    pub fn set_post_logout_redirect_uris(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().post_logout_redirect_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tos_uri`.\nURL that points to a terms of service document."]
    pub fn set_tos_uri(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tos_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `visibility`.\nPromote the OAuth client from private to public visibility. Only `public` is accepted; demotion to `private` is not supported. Promotion requires a non-empty client name, logo URI, verified client URI host, and at least one non-identity scope.\nAvailable values: \"public\"."]
    pub fn set_visibility(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().visibility = std::option::Option::Some(v.into());
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
        "Get a reference to the value of field `client_secret` after provisioning.\nThe client secret. This is the only time the secret is returned in a response."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_uri` after provisioning.\nURL of the home page of the client."]
    pub fn client_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_uri_verification` after provisioning.\nClient URI domain control verification state."]
    pub fn client_uri_verification(&self) -> OauthClient_ClientUriVerificationRef {
        OauthClient_ClientUriVerificationRef::new(
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
        "Get a reference to the value of field `visibility` after provisioning.\nPromote the OAuth client from private to public visibility. Only `public` is accepted; demotion to `private` is not supported. Promotion requires a non-empty client name, logo URI, verified client URI host, and at least one non-identity scope.\nAvailable values: \"public\"."]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }
}

impl terrars::Referable for OauthClient {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OauthClient { }

impl terrars::ToListMappable for OauthClient {
    type O = terrars::ListRef<OauthClientRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OauthClient_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_oauth_client".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOauthClient {
    pub tf_id: std::string::String,
    #[doc = "Account identifier tag."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Human-readable name of the OAuth client."]
    pub client_name: terrars::PrimField<std::string::String>,
    #[doc =
        "Array of OAuth grant types the client is allowed to use. `authorization_code` is required; `refresh_token` may be included optionally."]
    pub grant_types: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "Array of allowed redirect URIs for the client."]
    pub redirect_uris: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "Array of OAuth response types the client is allowed to use."]
    pub response_types: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "Array of OAuth scopes the client is allowed to request. Colon-delimited scopes are not accepted. Dot-delimited scopes are validated against available OAuth API scopes; simple identity scopes are allowed. Protocol scopes `offline_access` and `openid` are added or removed automatically based on `grant_types` and `response_types`."]
    pub scopes: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "The authentication method the client uses at the token endpoint.\nAvailable values: \"none\", \"client_secret_basic\", \"client_secret_post\"."]
    pub token_endpoint_auth_method: terrars::PrimField<std::string::String>,
}

impl BuildOauthClient {
    pub fn build(self, stack: &mut terrars::Stack) -> OauthClient {
        let out = OauthClient(std::rc::Rc::new(OauthClient_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OauthClientData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                allowed_cors_origins: core::default::Default::default(),
                client_name: self.client_name,
                client_uri: core::default::Default::default(),
                grant_types: self.grant_types,
                logo_uri: core::default::Default::default(),
                oauth_client_id: core::default::Default::default(),
                optional_scopes: core::default::Default::default(),
                policy_uri: core::default::Default::default(),
                post_logout_redirect_uris: core::default::Default::default(),
                redirect_uris: self.redirect_uris,
                response_types: self.response_types,
                scopes: self.scopes,
                token_endpoint_auth_method: self.token_endpoint_auth_method,
                tos_uri: core::default::Default::default(),
                visibility: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OauthClientRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OauthClientRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OauthClientRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
        "Get a reference to the value of field `client_secret` after provisioning.\nThe client secret. This is the only time the secret is returned in a response."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_uri` after provisioning.\nURL of the home page of the client."]
    pub fn client_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_uri_verification` after provisioning.\nClient URI domain control verification state."]
    pub fn client_uri_verification(&self) -> OauthClient_ClientUriVerificationRef {
        OauthClient_ClientUriVerificationRef::new(
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
        "Get a reference to the value of field `visibility` after provisioning.\nPromote the OAuth client from private to public visibility. Only `public` is accepted; demotion to `private` is not supported. Promotion requires a non-empty client name, logo URI, verified client URI host, and at least one non-identity scope.\nAvailable values: \"public\"."]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OauthClient_ClientUriVerification {}

impl OauthClient_ClientUriVerification { }

impl terrars::ToListMappable for OauthClient_ClientUriVerification {
    type O = terrars::BlockAssignable<OauthClient_ClientUriVerification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOauthClient_ClientUriVerification {}

impl BuildOauthClient_ClientUriVerification {
    pub fn build(self) -> OauthClient_ClientUriVerification {
        OauthClient_ClientUriVerification {}
    }
}

#[allow(dead_code)]
pub struct OauthClient_ClientUriVerificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OauthClient_ClientUriVerificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OauthClient_ClientUriVerificationRef {
        OauthClient_ClientUriVerificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OauthClient_ClientUriVerificationRef {
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
