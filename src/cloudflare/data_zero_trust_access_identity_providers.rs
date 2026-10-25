#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustAccessIdentityProvidersData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scim_enabled: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustAccessIdentityProviders_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustAccessIdentityProvidersData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustAccessIdentityProviders(std::rc::Rc<DataZeroTrustAccessIdentityProviders_>);

impl DataZeroTrustAccessIdentityProviders {
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

    #[doc =
        "Set the field `account_id`.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scim_enabled`.\nIndicates to Access to only retrieve identity providers that have the System for Cross-Domain Identity Management (SCIM) enabled."]
    pub fn set_scim_enabled(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().scim_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustAccessIdentityProviders_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scim_enabled` after provisioning.\nIndicates to Access to only retrieve identity providers that have the System for Cross-Domain Identity Management (SCIM) enabled."]
    pub fn scim_enabled(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scim_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustAccessIdentityProviders {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustAccessIdentityProviders { }

impl terrars::ToListMappable for DataZeroTrustAccessIdentityProviders {
    type O = terrars::ListRef<DataZeroTrustAccessIdentityProvidersRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustAccessIdentityProviders_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_access_identity_providers".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustAccessIdentityProviders {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustAccessIdentityProviders {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustAccessIdentityProviders {
        let out = DataZeroTrustAccessIdentityProviders(std::rc::Rc::new(DataZeroTrustAccessIdentityProviders_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustAccessIdentityProvidersData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                scim_enabled: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustAccessIdentityProvidersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessIdentityProvidersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustAccessIdentityProvidersRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustAccessIdentityProviders_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scim_enabled` after provisioning.\nIndicates to Access to only retrieve identity providers that have the System for Cross-Domain Identity Management (SCIM) enabled."]
    pub fn scim_enabled(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scim_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessIdentityProviders_Result_Config_HeaderAttributes {}

impl DataZeroTrustAccessIdentityProviders_Result_Config_HeaderAttributes { }

impl terrars::ToListMappable for DataZeroTrustAccessIdentityProviders_Result_Config_HeaderAttributes {
    type O = terrars::BlockAssignable<DataZeroTrustAccessIdentityProviders_Result_Config_HeaderAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessIdentityProviders_Result_Config_HeaderAttributes {}

impl BuildDataZeroTrustAccessIdentityProviders_Result_Config_HeaderAttributes {
    pub fn build(self) -> DataZeroTrustAccessIdentityProviders_Result_Config_HeaderAttributes {
        DataZeroTrustAccessIdentityProviders_Result_Config_HeaderAttributes {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessIdentityProviders_Result_Config_HeaderAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessIdentityProviders_Result_Config_HeaderAttributesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessIdentityProviders_Result_Config_HeaderAttributesRef {
        DataZeroTrustAccessIdentityProviders_Result_Config_HeaderAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessIdentityProviders_Result_Config_HeaderAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_name` after provisioning.\nattribute name from the IDP"]
    pub fn attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `header_name` after provisioning.\nheader that will be added on the request to the origin"]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessIdentityProviders_Result_Config {}

impl DataZeroTrustAccessIdentityProviders_Result_Config { }

impl terrars::ToListMappable for DataZeroTrustAccessIdentityProviders_Result_Config {
    type O = terrars::BlockAssignable<DataZeroTrustAccessIdentityProviders_Result_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessIdentityProviders_Result_Config {}

impl BuildDataZeroTrustAccessIdentityProviders_Result_Config {
    pub fn build(self) -> DataZeroTrustAccessIdentityProviders_Result_Config {
        DataZeroTrustAccessIdentityProviders_Result_Config {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessIdentityProviders_Result_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessIdentityProviders_Result_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessIdentityProviders_Result_ConfigRef {
        DataZeroTrustAccessIdentityProviders_Result_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessIdentityProviders_Result_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apps_domain` after provisioning.\nYour companies TLD"]
    pub fn apps_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apps_domain", self.base))
    }

    #[doc =
        "Get a reference to the value of field `attributes` after provisioning.\nA list of SAML attribute names that will be added to your signed JWT token and can be used in SAML policy rules."]
    pub fn attributes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auth_url` after provisioning.\nThe authorization_endpoint URL of your IdP"]
    pub fn auth_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `authorization_server_id` after provisioning.\nYour okta authorization server id"]
    pub fn authorization_server_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_server_id", self.base))
    }

    #[doc = "Get a reference to the value of field `centrify_account` after provisioning.\nYour centrify account url"]
    pub fn centrify_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.centrify_account", self.base))
    }

    #[doc = "Get a reference to the value of field `centrify_app_id` after provisioning.\nYour centrify app id"]
    pub fn centrify_app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.centrify_app_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `certs_url` after provisioning.\nThe jwks_uri endpoint of your IdP to allow the IdP keys to sign the tokens"]
    pub fn certs_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certs_url", self.base))
    }

    #[doc = "Get a reference to the value of field `claims` after provisioning.\nCustom claims"]
    pub fn claims(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.claims", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nYour OAuth Client ID"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nYour OAuth Client Secret"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `conditional_access_enabled` after provisioning.\nShould Cloudflare try to load authentication contexts from your account"]
    pub fn conditional_access_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conditional_access_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\nYour Azure directory uuid"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email_attribute_name` after provisioning.\nThe attribute name for email in the SAML response."]
    pub fn email_attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_attribute_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email_claim_name` after provisioning.\nThe claim name for email in the id_token response."]
    pub fn email_claim_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_claim_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_encryption` after provisioning.\nEnable SAML assertion encryption. When enabled, the Identity Provider will encrypt \nSAML assertions using the certificate from the assigned certificate set.\n\nTo enable encryption:\n1. Create a certificate set via POST to `/identity_providers/{id}/saml_certificate`\n2. Set this field to `true` and include `saml_certificate_set_id` in the PUT request\n3. Configure the public certificate in your external Identity Provider\n\nNote: Requires `saml_certificate_set_id` to be set when `true`."]
    pub fn enable_encryption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_encryption", self.base))
    }

    #[doc =
        "Get a reference to the value of field `force_authn` after provisioning.\nAsks the IdP to reauthenticate the user for each SAML authentication request."]
    pub fn force_authn(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_authn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `header_attributes` after provisioning.\nAdd a list of attribute names that will be returned in the response header from the Access callback."]
    pub fn header_attributes(
        &self,
    ) -> terrars::ListRef<DataZeroTrustAccessIdentityProviders_Result_Config_HeaderAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_attributes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `idp_public_certs` after provisioning.\nX509 certificate to verify the signature in the SAML authentication response"]
    pub fn idp_public_certs(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idp_public_certs", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer_url` after provisioning.\nIdP Entity ID or Issuer URL"]
    pub fn issuer_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `login_page_auto_prompt` after provisioning.\nWhen enabled, the Access login page automatically prompts the user to authenticate with a passkey."]
    pub fn login_page_auto_prompt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login_page_auto_prompt", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_sso_url_length` after provisioning.\nThe maximum URL length the IdP accepts for the SSO redirect URL.\nWhen the constructed SSO URL would exceed this length, the RelayState\nis stored server-side and a short nonce is passed to the IdP instead.\nSet this if your IdP enforces a URL length limit."]
    pub fn max_sso_url_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_sso_url_length", self.base))
    }

    #[doc = "Get a reference to the value of field `okta_account` after provisioning.\nYour okta account url"]
    pub fn okta_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.okta_account", self.base))
    }

    #[doc = "Get a reference to the value of field `onelogin_account` after provisioning.\nYour OneLogin account url"]
    pub fn onelogin_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.onelogin_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ping_env_id` after provisioning.\nYour PingOne environment identifier"]
    pub fn ping_env_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ping_env_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pkce_enabled` after provisioning.\nEnable Proof Key for Code Exchange (PKCE)"]
    pub fn pkce_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pkce_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prompt` after provisioning.\nIndicates the type of user interaction that is required. prompt=login forces the user to enter their credentials on that request, negating single-sign on. prompt=none is the opposite. It ensures that the user isn't presented with any interactive prompt. If the request can't be completed silently by using single-sign on, the Microsoft identity platform returns an interaction_required error. prompt=select_account interrupts single sign-on providing account selection experience listing all the accounts either in session or any remembered account or an option to choose to use a different account altogether.\nAvailable values: \"login\", \"select_account\", \"none\", \"consent\"."]
    pub fn prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect_url` after provisioning.\n"]
    pub fn redirect_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `restrict_to_account_members` after provisioning.\nWhen enabled, only users who are members of your Cloudflare account can authenticate through this identity provider. When disabled, any user with a Cloudflare account can authenticate, subject to your Access policies."]
    pub fn restrict_to_account_members(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restrict_to_account_members", self.base))
    }

    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nOAuth scopes"]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sign_request` after provisioning.\nSign the SAML authentication request with Access credentials. To verify the signature, use the public key from the Access certs endpoints."]
    pub fn sign_request(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sign_request", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sso_target_url` after provisioning.\nURL to send the SAML authentication requests to"]
    pub fn sso_target_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sso_target_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `support_groups` after provisioning.\nShould Cloudflare try to load groups from your account"]
    pub fn support_groups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `token_url` after provisioning.\nThe token_endpoint URL of your IdP"]
    pub fn token_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_login_hint` after provisioning.\nWhether to use a previously authenticated Access email as a Google login hint when exactly one email matches the Workspace domain."]
    pub fn use_login_hint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_login_hint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet_CurrentCertificate {}

impl DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet_CurrentCertificate { }

impl terrars::ToListMappable for DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet_CurrentCertificate {
    type O =
        terrars::BlockAssignable<DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet_CurrentCertificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet_CurrentCertificate {}

impl BuildDataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet_CurrentCertificate {
    pub fn build(self) -> DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet_CurrentCertificate {
        DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet_CurrentCertificate {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet_CurrentCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet_CurrentCertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet_CurrentCertificateRef {
        DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet_CurrentCertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet_CurrentCertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `is_current` after provisioning.\nIndicates whether this is the currently active certificate"]
    pub fn is_current(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_current", self.base))
    }

    #[doc =
        "Get a reference to the value of field `not_after` after provisioning.\nCertificate expiration date. Certificates are automatically rotated 30 days before expiration."]
    pub fn not_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_after", self.base))
    }

    #[doc =
        "Get a reference to the value of field `public_certificate` after provisioning.\nPEM-encoded X.509 certificate containing the public key.\nConfigure this certificate in your external SAML Identity Provider to enable encryption."]
    pub fn public_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier for the certificate"]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet {}

impl DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet { }

impl terrars::ToListMappable for DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet {
    type O = terrars::BlockAssignable<DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet {}

impl BuildDataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet {
    pub fn build(self) -> DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet {
        DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSetRef {
        DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp when the certificate set was created"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `current_certificate` after provisioning.\nThe currently active certificate used for encrypting SAML assertions"]
    pub fn current_certificate(
        &self,
    ) -> DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet_CurrentCertificateRef {
        DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSet_CurrentCertificateRef::new(
            self.shared().clone(),
            format!("{}.current_certificate", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `previous_certificate` after provisioning.\nThe previous certificate, maintained during rotation to ensure continuity. Null if no rotation has occurred. Mirrors the structure of `saml_certificate`."]
    pub fn previous_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.previous_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nUnique identifier for the certificate set"]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the certificate set was last updated (e.g., during rotation)"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessIdentityProviders_Result_ScimConfig {}

impl DataZeroTrustAccessIdentityProviders_Result_ScimConfig { }

impl terrars::ToListMappable for DataZeroTrustAccessIdentityProviders_Result_ScimConfig {
    type O = terrars::BlockAssignable<DataZeroTrustAccessIdentityProviders_Result_ScimConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessIdentityProviders_Result_ScimConfig {}

impl BuildDataZeroTrustAccessIdentityProviders_Result_ScimConfig {
    pub fn build(self) -> DataZeroTrustAccessIdentityProviders_Result_ScimConfig {
        DataZeroTrustAccessIdentityProviders_Result_ScimConfig {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessIdentityProviders_Result_ScimConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessIdentityProviders_Result_ScimConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessIdentityProviders_Result_ScimConfigRef {
        DataZeroTrustAccessIdentityProviders_Result_ScimConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessIdentityProviders_Result_ScimConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nA flag to enable or disable SCIM for the identity provider."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_update_behavior` after provisioning.\nIndicates how a SCIM event updates a user identity used for policy evaluation. Use \"automatic\" to automatically update a user's identity and augment it with fields from the SCIM user resource. Use \"reauth\" to force re-authentication on group membership updates, user identity update will only occur after successful re-authentication. With \"reauth\" identities will not contain fields from the SCIM user resource. With \"no_action\" identities will not be changed by SCIM updates in any way and users will not be prompted to reauthenticate.\nAvailable values: \"automatic\", \"reauth\", \"no_action\"."]
    pub fn identity_update_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_update_behavior", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scim_base_url` after provisioning.\nThe base URL of Cloudflare's SCIM V2.0 API endpoint."]
    pub fn scim_base_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scim_base_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seat_deprovision` after provisioning.\nA flag to remove a user's seat in Zero Trust when they have been deprovisioned in the Identity Provider.  This cannot be enabled unless user_deprovision is also enabled."]
    pub fn seat_deprovision(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seat_deprovision", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret` after provisioning.\nA read-only token generated when the SCIM integration is enabled for the first time.  It is redacted on subsequent requests.  If you lose this you will need to refresh it at /access/identity_providers/:idpID/refresh_scim_secret."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user_deprovision` after provisioning.\nA flag to enable revoking a user's session in Access and Gateway when they have been deprovisioned in the Identity Provider."]
    pub fn user_deprovision(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_deprovision", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessIdentityProviders_Result {}

impl DataZeroTrustAccessIdentityProviders_Result { }

impl terrars::ToListMappable for DataZeroTrustAccessIdentityProviders_Result {
    type O = terrars::BlockAssignable<DataZeroTrustAccessIdentityProviders_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessIdentityProviders_Result {}

impl BuildDataZeroTrustAccessIdentityProviders_Result {
    pub fn build(self) -> DataZeroTrustAccessIdentityProviders_Result {
        DataZeroTrustAccessIdentityProviders_Result {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessIdentityProviders_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessIdentityProviders_ResultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessIdentityProviders_ResultRef {
        DataZeroTrustAccessIdentityProviders_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessIdentityProviders_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nThe configuration parameters for the identity provider. To view the required parameters for a specific provider, refer to our [developer documentation](https://developers.cloudflare.com/cloudflare-one/identity/idp-integration/)."]
    pub fn config(&self) -> DataZeroTrustAccessIdentityProviders_Result_ConfigRef {
        DataZeroTrustAccessIdentityProviders_Result_ConfigRef::new(
            self.shared().clone(),
            format!("{}.config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the identity provider, shown to users on the login page."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nIndicates that the identity provider is immutable and cannot be updated or deleted via the API."]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `saml_certificate_set` after provisioning.\nThe SAML encryption certificate set details, including current and previous certificates.\nOnly present for SAML identity providers with a certificate set assigned."]
    pub fn saml_certificate_set(&self) -> DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSetRef {
        DataZeroTrustAccessIdentityProviders_Result_SamlCertificateSetRef::new(
            self.shared().clone(),
            format!("{}.saml_certificate_set", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `saml_certificate_set_id` after provisioning.\nThe UID of the SAML encryption certificate set assigned to this Identity Provider.\nOnly present for SAML identity providers with encryption configured.\nCreate a certificate set via POST to `/identity_providers/{id}/saml_certificate`."]
    pub fn saml_certificate_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.saml_certificate_set_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scim_config` after provisioning.\nThe configuration settings for enabling a System for Cross-Domain Identity Management (SCIM) with the identity provider."]
    pub fn scim_config(&self) -> DataZeroTrustAccessIdentityProviders_Result_ScimConfigRef {
        DataZeroTrustAccessIdentityProviders_Result_ScimConfigRef::new(
            self.shared().clone(),
            format!("{}.scim_config", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of identity provider. To determine the value for a specific provider, refer to our [developer documentation](https://developers.cloudflare.com/cloudflare-one/identity/idp-integration/).\nAvailable values: \"onetimepin\", \"azureAD\", \"saml\", \"centrify\", \"facebook\", \"github\", \"google-apps\", \"google\", \"linkedin\", \"oidc\", \"okta\", \"onelogin\", \"pingone\", \"yandex\", \"cloudflare\", \"passkeys\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
