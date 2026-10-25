#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustAccessApplicationData_ {
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
    app_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataZeroTrustAccessApplication_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustAccessApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustAccessApplicationData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustAccessApplication(std::rc::Rc<DataZeroTrustAccessApplication_>);

impl DataZeroTrustAccessApplication {
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

    #[doc = "Set the field `app_id`.\nIdentifier."]
    pub fn set_app_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().app_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataZeroTrustAccessApplication_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
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

    #[doc =
        "Get a reference to the value of field `allow_authenticate_via_warp` after provisioning.\nWhen set to true, users can authenticate to this application using their WARP session.  When set to false this application will always require direct IdP authentication. This setting always overrides the organization setting for WARP authentication."]
    pub fn allow_authenticate_via_warp(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_authenticate_via_warp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_iframe` after provisioning.\nEnables loading application content in an iFrame."]
    pub fn allow_iframe(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_iframe", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed_idps` after provisioning.\nThe identity providers your users can select when connecting to this application. Defaults to all IdPs configured in your account."]
    pub fn allowed_idps(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_idps", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nIdentifier."]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_launcher_logo_url` after provisioning.\nThe image URL of the logo shown in the App Launcher header."]
    pub fn app_launcher_logo_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_launcher_logo_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_launcher_visible` after provisioning.\nDisplays the application in the App Launcher."]
    pub fn app_launcher_visible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_launcher_visible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aud` after provisioning.\nAudience tag."]
    pub fn aud(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aud", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_redirect_to_identity` after provisioning.\nWhen set to `true`, users skip the identity provider selection step during login. You must specify only one identity provider in allowed_idps."]
    pub fn auto_redirect_to_identity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_redirect_to_identity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bg_color` after provisioning.\nThe background color of the App Launcher page."]
    pub fn bg_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bg_color", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors_headers` after provisioning.\n"]
    pub fn cors_headers(&self) -> DataZeroTrustAccessApplication_CorsHeadersRef {
        DataZeroTrustAccessApplication_CorsHeadersRef::new(
            self.shared().clone(),
            format!("{}.cors_headers", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `custom_deny_message` after provisioning.\nThe custom error message shown to a user when they are denied access to the application."]
    pub fn custom_deny_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_deny_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_deny_url` after provisioning.\nThe custom URL a user is redirected to when they are denied access to the application when failing identity-based rules."]
    pub fn custom_deny_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_deny_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_non_identity_deny_url` after provisioning.\nThe custom URL a user is redirected to when they are denied access to the application when failing non-identity rules."]
    pub fn custom_non_identity_deny_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_non_identity_deny_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_pages` after provisioning.\nThe custom pages that will be displayed when applicable for this application"]
    pub fn custom_pages(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_pages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `destinations` after provisioning.\nList of destinations secured by Access. This supersedes `self_hosted_domains` to allow for more flexibility in defining different types of domains. If `destinations` are provided, then `self_hosted_domains` will be ignored."]
    pub fn destinations(&self) -> terrars::ListRef<DataZeroTrustAccessApplication_DestinationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destinations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nThe primary hostname and path secured by Access. This domain will be displayed if the app is visible in the App Launcher."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_binding_cookie` after provisioning.\nEnables the binding cookie, which increases security against compromised authorization tokens and CSRF attacks."]
    pub fn enable_binding_cookie(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_binding_cookie", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustAccessApplication_FilterRef {
        DataZeroTrustAccessApplication_FilterRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `footer_links` after provisioning.\nThe links in the App Launcher footer."]
    pub fn footer_links(&self) -> terrars::ListRef<DataZeroTrustAccessApplication_FooterLinksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.footer_links", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `header_bg_color` after provisioning.\nThe background color of the App Launcher header."]
    pub fn header_bg_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_bg_color", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `http_only_cookie_attribute` after provisioning.\nEnables the HttpOnly cookie attribute, which increases security against XSS attacks."]
    pub fn http_only_cookie_attribute(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_only_cookie_attribute", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `landing_page_design` after provisioning.\nThe design of the App Launcher landing page shown to users when they log in."]
    pub fn landing_page_design(&self) -> DataZeroTrustAccessApplication_LandingPageDesignRef {
        DataZeroTrustAccessApplication_LandingPageDesignRef::new(
            self.shared().clone(),
            format!("{}.landing_page_design", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `logo_url` after provisioning.\nThe image URL for the logo shown in the App Launcher dashboard."]
    pub fn logo_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logo_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mfa_config` after provisioning.\nConfigures multi-factor authentication (MFA) settings for the application. Only valid for self_hosted, ssh, vnc, and rdp application types."]
    pub fn mfa_config(&self) -> DataZeroTrustAccessApplication_MfaConfigRef {
        DataZeroTrustAccessApplication_MfaConfigRef::new(
            self.shared().clone(),
            format!("{}.mfa_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the application."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `options_preflight_bypass` after provisioning.\nAllows options preflight requests to bypass Access authentication and go directly to the origin. Cannot turn on if cors_headers is set."]
    pub fn options_preflight_bypass(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.options_preflight_bypass", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `path_cookie_attribute` after provisioning.\nEnables cookie paths to scope an application's JWT to the application path. If disabled, the JWT will scope to the hostname by default"]
    pub fn path_cookie_attribute(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_cookie_attribute", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policies` after provisioning.\n"]
    pub fn policies(&self) -> terrars::ListRef<DataZeroTrustAccessApplication_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_service_tokens_from_header` after provisioning.\nAllows matching Access Service Tokens passed HTTP in a single header with this name.\nThis works as an alternative to the (CF-Access-Client-Id, CF-Access-Client-Secret) pair of headers.\nThe header value will be interpreted as a json object similar to: \n  {\n    \"cf-access-client-id\": \"88bf3b6d86161464f6509f7219099e57.access.example.com\",\n    \"cf-access-client-secret\": \"bdd31cbc4dec990953e39163fbbb194c93313ca9f0a6e420346af9d326b1d2a5\"\n  }"]
    pub fn read_service_tokens_from_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_service_tokens_from_header", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `saas_app` after provisioning.\n"]
    pub fn saas_app(&self) -> DataZeroTrustAccessApplication_SaasAppRef {
        DataZeroTrustAccessApplication_SaasAppRef::new(
            self.shared().clone(),
            format!("{}.saas_app", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `same_site_cookie_attribute` after provisioning.\nSets the SameSite cookie setting, which provides increased security against CSRF attacks."]
    pub fn same_site_cookie_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.same_site_cookie_attribute", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scim_config` after provisioning.\nConfiguration for provisioning to this application via SCIM. This is currently in closed beta."]
    pub fn scim_config(&self) -> DataZeroTrustAccessApplication_ScimConfigRef {
        DataZeroTrustAccessApplication_ScimConfigRef::new(
            self.shared().clone(),
            format!("{}.scim_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `self_hosted_domains` after provisioning.\nList of public domains that Access will secure. This field is deprecated in favor of `destinations` and will be supported until **November 21, 2025.** If `destinations` are provided, then `self_hosted_domains` will be ignored."]
    pub fn self_hosted_domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_hosted_domains", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_auth_401_redirect` after provisioning.\nReturns a 401 status code when the request is blocked by a Service Auth policy."]
    pub fn service_auth_401_redirect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_auth_401_redirect", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nThe amount of time that tokens issued for this application will be valid. Must be in the format `300ms` or `2h45m`. Valid time units are: ns, us (or µs), ms, s, m, h. For infrastructure applications, this is the maximum connection duration; connections are terminated once it elapses. Infrastructure durations must use s, m, or h and resolve to whole seconds. Defaults to `24h` for supported application types."]
    pub fn session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_app_launcher_login_page` after provisioning.\nDetermines when to skip the App Launcher landing page."]
    pub fn skip_app_launcher_login_page(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_app_launcher_login_page", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_interstitial` after provisioning.\nEnables automatic authentication through cloudflared."]
    pub fn skip_interstitial(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_interstitial", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nThe tags you want assigned to an application. Tags are used to filter applications in the App Launcher dashboard."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_criteria` after provisioning.\n"]
    pub fn target_criteria(&self) -> terrars::ListRef<DataZeroTrustAccessApplication_TargetCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_criteria", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe application type.\nAvailable values: \"self_hosted\", \"saas\", \"ssh\", \"vnc\", \"app_launcher\", \"warp\", \"biso\", \"bookmark\", \"dash_sso\", \"infrastructure\", \"rdp\", \"mcp\", \"mcp_portal\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustAccessApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustAccessApplication { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication {
    type O = terrars::ListRef<DataZeroTrustAccessApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustAccessApplication_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_access_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustAccessApplication {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustAccessApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustAccessApplication {
        let out = DataZeroTrustAccessApplication(std::rc::Rc::new(DataZeroTrustAccessApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustAccessApplicationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                app_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustAccessApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustAccessApplicationRef {
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

    #[doc =
        "Get a reference to the value of field `allow_authenticate_via_warp` after provisioning.\nWhen set to true, users can authenticate to this application using their WARP session.  When set to false this application will always require direct IdP authentication. This setting always overrides the organization setting for WARP authentication."]
    pub fn allow_authenticate_via_warp(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_authenticate_via_warp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_iframe` after provisioning.\nEnables loading application content in an iFrame."]
    pub fn allow_iframe(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_iframe", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed_idps` after provisioning.\nThe identity providers your users can select when connecting to this application. Defaults to all IdPs configured in your account."]
    pub fn allowed_idps(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_idps", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nIdentifier."]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_launcher_logo_url` after provisioning.\nThe image URL of the logo shown in the App Launcher header."]
    pub fn app_launcher_logo_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_launcher_logo_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_launcher_visible` after provisioning.\nDisplays the application in the App Launcher."]
    pub fn app_launcher_visible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_launcher_visible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aud` after provisioning.\nAudience tag."]
    pub fn aud(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aud", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_redirect_to_identity` after provisioning.\nWhen set to `true`, users skip the identity provider selection step during login. You must specify only one identity provider in allowed_idps."]
    pub fn auto_redirect_to_identity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_redirect_to_identity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bg_color` after provisioning.\nThe background color of the App Launcher page."]
    pub fn bg_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bg_color", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors_headers` after provisioning.\n"]
    pub fn cors_headers(&self) -> DataZeroTrustAccessApplication_CorsHeadersRef {
        DataZeroTrustAccessApplication_CorsHeadersRef::new(
            self.shared().clone(),
            format!("{}.cors_headers", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `custom_deny_message` after provisioning.\nThe custom error message shown to a user when they are denied access to the application."]
    pub fn custom_deny_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_deny_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_deny_url` after provisioning.\nThe custom URL a user is redirected to when they are denied access to the application when failing identity-based rules."]
    pub fn custom_deny_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_deny_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_non_identity_deny_url` after provisioning.\nThe custom URL a user is redirected to when they are denied access to the application when failing non-identity rules."]
    pub fn custom_non_identity_deny_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_non_identity_deny_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_pages` after provisioning.\nThe custom pages that will be displayed when applicable for this application"]
    pub fn custom_pages(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_pages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `destinations` after provisioning.\nList of destinations secured by Access. This supersedes `self_hosted_domains` to allow for more flexibility in defining different types of domains. If `destinations` are provided, then `self_hosted_domains` will be ignored."]
    pub fn destinations(&self) -> terrars::ListRef<DataZeroTrustAccessApplication_DestinationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destinations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nThe primary hostname and path secured by Access. This domain will be displayed if the app is visible in the App Launcher."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_binding_cookie` after provisioning.\nEnables the binding cookie, which increases security against compromised authorization tokens and CSRF attacks."]
    pub fn enable_binding_cookie(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_binding_cookie", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustAccessApplication_FilterRef {
        DataZeroTrustAccessApplication_FilterRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `footer_links` after provisioning.\nThe links in the App Launcher footer."]
    pub fn footer_links(&self) -> terrars::ListRef<DataZeroTrustAccessApplication_FooterLinksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.footer_links", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `header_bg_color` after provisioning.\nThe background color of the App Launcher header."]
    pub fn header_bg_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_bg_color", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `http_only_cookie_attribute` after provisioning.\nEnables the HttpOnly cookie attribute, which increases security against XSS attacks."]
    pub fn http_only_cookie_attribute(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_only_cookie_attribute", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `landing_page_design` after provisioning.\nThe design of the App Launcher landing page shown to users when they log in."]
    pub fn landing_page_design(&self) -> DataZeroTrustAccessApplication_LandingPageDesignRef {
        DataZeroTrustAccessApplication_LandingPageDesignRef::new(
            self.shared().clone(),
            format!("{}.landing_page_design", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `logo_url` after provisioning.\nThe image URL for the logo shown in the App Launcher dashboard."]
    pub fn logo_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logo_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mfa_config` after provisioning.\nConfigures multi-factor authentication (MFA) settings for the application. Only valid for self_hosted, ssh, vnc, and rdp application types."]
    pub fn mfa_config(&self) -> DataZeroTrustAccessApplication_MfaConfigRef {
        DataZeroTrustAccessApplication_MfaConfigRef::new(
            self.shared().clone(),
            format!("{}.mfa_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the application."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `options_preflight_bypass` after provisioning.\nAllows options preflight requests to bypass Access authentication and go directly to the origin. Cannot turn on if cors_headers is set."]
    pub fn options_preflight_bypass(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.options_preflight_bypass", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `path_cookie_attribute` after provisioning.\nEnables cookie paths to scope an application's JWT to the application path. If disabled, the JWT will scope to the hostname by default"]
    pub fn path_cookie_attribute(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_cookie_attribute", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policies` after provisioning.\n"]
    pub fn policies(&self) -> terrars::ListRef<DataZeroTrustAccessApplication_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_service_tokens_from_header` after provisioning.\nAllows matching Access Service Tokens passed HTTP in a single header with this name.\nThis works as an alternative to the (CF-Access-Client-Id, CF-Access-Client-Secret) pair of headers.\nThe header value will be interpreted as a json object similar to: \n  {\n    \"cf-access-client-id\": \"88bf3b6d86161464f6509f7219099e57.access.example.com\",\n    \"cf-access-client-secret\": \"bdd31cbc4dec990953e39163fbbb194c93313ca9f0a6e420346af9d326b1d2a5\"\n  }"]
    pub fn read_service_tokens_from_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_service_tokens_from_header", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `saas_app` after provisioning.\n"]
    pub fn saas_app(&self) -> DataZeroTrustAccessApplication_SaasAppRef {
        DataZeroTrustAccessApplication_SaasAppRef::new(
            self.shared().clone(),
            format!("{}.saas_app", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `same_site_cookie_attribute` after provisioning.\nSets the SameSite cookie setting, which provides increased security against CSRF attacks."]
    pub fn same_site_cookie_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.same_site_cookie_attribute", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scim_config` after provisioning.\nConfiguration for provisioning to this application via SCIM. This is currently in closed beta."]
    pub fn scim_config(&self) -> DataZeroTrustAccessApplication_ScimConfigRef {
        DataZeroTrustAccessApplication_ScimConfigRef::new(
            self.shared().clone(),
            format!("{}.scim_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `self_hosted_domains` after provisioning.\nList of public domains that Access will secure. This field is deprecated in favor of `destinations` and will be supported until **November 21, 2025.** If `destinations` are provided, then `self_hosted_domains` will be ignored."]
    pub fn self_hosted_domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_hosted_domains", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_auth_401_redirect` after provisioning.\nReturns a 401 status code when the request is blocked by a Service Auth policy."]
    pub fn service_auth_401_redirect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_auth_401_redirect", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nThe amount of time that tokens issued for this application will be valid. Must be in the format `300ms` or `2h45m`. Valid time units are: ns, us (or µs), ms, s, m, h. For infrastructure applications, this is the maximum connection duration; connections are terminated once it elapses. Infrastructure durations must use s, m, or h and resolve to whole seconds. Defaults to `24h` for supported application types."]
    pub fn session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_app_launcher_login_page` after provisioning.\nDetermines when to skip the App Launcher landing page."]
    pub fn skip_app_launcher_login_page(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_app_launcher_login_page", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_interstitial` after provisioning.\nEnables automatic authentication through cloudflared."]
    pub fn skip_interstitial(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_interstitial", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nThe tags you want assigned to an application. Tags are used to filter applications in the App Launcher dashboard."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_criteria` after provisioning.\n"]
    pub fn target_criteria(&self) -> terrars::ListRef<DataZeroTrustAccessApplication_TargetCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_criteria", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe application type.\nAvailable values: \"self_hosted\", \"saas\", \"ssh\", \"vnc\", \"app_launcher\", \"warp\", \"biso\", \"bookmark\", \"dash_sso\", \"infrastructure\", \"rdp\", \"mcp\", \"mcp_portal\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_CorsHeaders {}

impl DataZeroTrustAccessApplication_CorsHeaders { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_CorsHeaders {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_CorsHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_CorsHeaders {}

impl BuildDataZeroTrustAccessApplication_CorsHeaders {
    pub fn build(self) -> DataZeroTrustAccessApplication_CorsHeaders {
        DataZeroTrustAccessApplication_CorsHeaders {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_CorsHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_CorsHeadersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessApplication_CorsHeadersRef {
        DataZeroTrustAccessApplication_CorsHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_CorsHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_all_headers` after provisioning.\nAllows all HTTP request headers."]
    pub fn allow_all_headers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_all_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_all_methods` after provisioning.\nAllows all HTTP request methods."]
    pub fn allow_all_methods(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_all_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_all_origins` after provisioning.\nAllows all origins."]
    pub fn allow_all_origins(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_all_origins", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_credentials` after provisioning.\nWhen set to `true`, includes credentials (cookies, authorization headers, or TLS client certificates) with requests."]
    pub fn allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowed_headers` after provisioning.\nAllowed HTTP request headers."]
    pub fn allowed_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowed_methods` after provisioning.\nAllowed HTTP request methods."]
    pub fn allowed_methods(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_origins` after provisioning.\nAllowed origins."]
    pub fn allowed_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_origins", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_age` after provisioning.\nThe maximum number of seconds the results of a preflight request can be cached."]
    pub fn max_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Destinations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcp_server_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataZeroTrustAccessApplication_Destinations {
    #[doc =
        "Set the field `mcp_server_id`.\nA MCP server id configured in ai-controls. Access will secure the MCP server if accessed through a MCP portal."]
    pub fn set_mcp_server_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mcp_server_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Destinations {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Destinations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Destinations {}

impl BuildDataZeroTrustAccessApplication_Destinations {
    pub fn build(self) -> DataZeroTrustAccessApplication_Destinations {
        DataZeroTrustAccessApplication_Destinations { mcp_server_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_DestinationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_DestinationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_DestinationsRef {
        DataZeroTrustAccessApplication_DestinationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_DestinationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cidr` after provisioning.\nThe CIDR range of the destination. Single IPs will be computed as /32."]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nThe hostname of the destination. Matches a valid SNI served by an HTTPS origin."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `l4_protocol` after provisioning.\nThe L4 protocol of the destination. When omitted, both UDP and TCP traffic will match.\nAvailable values: \"tcp\", \"udp\"."]
    pub fn l4_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.l4_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mcp_server_id` after provisioning.\nA MCP server id configured in ai-controls. Access will secure the MCP server if accessed through a MCP portal."]
    pub fn mcp_server_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mcp_server_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_range` after provisioning.\nThe port range of the destination. Can be a single port or a range of ports. When omitted, all ports will match."]
    pub fn port_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_range", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"public\", \"private\", \"via_mcp_server_portal\", \"worker\", \"preview_worker\", \"all_workers\", \"all_preview_workers\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nThe URI of the destination. Public destinations' URIs can include a domain and path with [wildcards](https://developers.cloudflare.com/cloudflare-one/policies/access/app-paths/)."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vnet_id` after provisioning.\nThe VNET ID to match the destination. When omitted, all VNETs will match."]
    pub fn vnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vnet_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `worker_id` after provisioning.\nThe ID of the Cloudflare Worker to protect with Access. Required when type is `worker` or `preview_worker`."]
    pub fn worker_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aud: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataZeroTrustAccessApplication_Filter {
    #[doc = "Set the field `aud`.\nThe aud of the app."]
    pub fn set_aud(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aud = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\nThe domain of the app."]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exact`.\nTrue for only exact string matches against passed name/domain query parameters."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe name of the app."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\nSearch for apps by other listed query parameters."]
    pub fn set_search(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.search = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Filter {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Filter {}

impl BuildDataZeroTrustAccessApplication_Filter {
    pub fn build(self) -> DataZeroTrustAccessApplication_Filter {
        DataZeroTrustAccessApplication_Filter {
            aud: core::default::Default::default(),
            domain: core::default::Default::default(),
            exact: core::default::Default::default(),
            name: core::default::Default::default(),
            search: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessApplication_FilterRef {
        DataZeroTrustAccessApplication_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aud` after provisioning.\nThe aud of the app."]
    pub fn aud(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aud", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe domain of the app."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exact` after provisioning.\nTrue for only exact string matches against passed name/domain query parameters."]
    pub fn exact(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the app."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nSearch for apps by other listed query parameters."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_FooterLinks {}

impl DataZeroTrustAccessApplication_FooterLinks { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_FooterLinks {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_FooterLinks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_FooterLinks {}

impl BuildDataZeroTrustAccessApplication_FooterLinks {
    pub fn build(self) -> DataZeroTrustAccessApplication_FooterLinks {
        DataZeroTrustAccessApplication_FooterLinks {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_FooterLinksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_FooterLinksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessApplication_FooterLinksRef {
        DataZeroTrustAccessApplication_FooterLinksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_FooterLinksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe hypertext in the footer link."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nthe hyperlink in the footer link."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_LandingPageDesign {}

impl DataZeroTrustAccessApplication_LandingPageDesign { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_LandingPageDesign {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_LandingPageDesign>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_LandingPageDesign {}

impl BuildDataZeroTrustAccessApplication_LandingPageDesign {
    pub fn build(self) -> DataZeroTrustAccessApplication_LandingPageDesign {
        DataZeroTrustAccessApplication_LandingPageDesign {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_LandingPageDesignRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_LandingPageDesignRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_LandingPageDesignRef {
        DataZeroTrustAccessApplication_LandingPageDesignRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_LandingPageDesignRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `button_color` after provisioning.\nThe background color of the log in button on the landing page."]
    pub fn button_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.button_color", self.base))
    }

    #[doc =
        "Get a reference to the value of field `button_text_color` after provisioning.\nThe color of the text in the log in button on the landing page."]
    pub fn button_text_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.button_text_color", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image_url` after provisioning.\nThe URL of the image shown on the landing page."]
    pub fn image_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `message` after provisioning.\nThe message shown on the landing page."]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe title shown on the landing page."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_MfaConfig {}

impl DataZeroTrustAccessApplication_MfaConfig { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_MfaConfig {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_MfaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_MfaConfig {}

impl BuildDataZeroTrustAccessApplication_MfaConfig {
    pub fn build(self) -> DataZeroTrustAccessApplication_MfaConfig {
        DataZeroTrustAccessApplication_MfaConfig {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_MfaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_MfaConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessApplication_MfaConfigRef {
        DataZeroTrustAccessApplication_MfaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_MfaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_authenticators` after provisioning.\nThe authenticators allowed for MFA.\nAvailable values: \"totp\", \"biometrics\", \"security_key\"."]
    pub fn allowed_authenticators(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_authenticators", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mfa_disabled` after provisioning.\nWhether MFA is disabled for this application."]
    pub fn mfa_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mfa_disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nHow often a user will be forced to re-authenticate with MFA."]
    pub fn session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_ApprovalGroups {}

impl DataZeroTrustAccessApplication_Policies_ApprovalGroups { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_ApprovalGroups {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_ApprovalGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_ApprovalGroups {}

impl BuildDataZeroTrustAccessApplication_Policies_ApprovalGroups {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_ApprovalGroups {
        DataZeroTrustAccessApplication_Policies_ApprovalGroups {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_ApprovalGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_ApprovalGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_ApprovalGroupsRef {
        DataZeroTrustAccessApplication_Policies_ApprovalGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_ApprovalGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `approvals_needed` after provisioning.\nThe number of approvals needed to obtain access."]
    pub fn approvals_needed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approvals_needed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email_addresses` after provisioning.\nA list of emails that can approve the access request."]
    pub fn email_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email_list_uuid` after provisioning.\nThe UUID of an re-usable email list."]
    pub fn email_list_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_list_uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_ConnectionRules_Ssh {}

impl DataZeroTrustAccessApplication_Policies_ConnectionRules_Ssh { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_ConnectionRules_Ssh {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_ConnectionRules_Ssh>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_ConnectionRules_Ssh {}

impl BuildDataZeroTrustAccessApplication_Policies_ConnectionRules_Ssh {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_ConnectionRules_Ssh {
        DataZeroTrustAccessApplication_Policies_ConnectionRules_Ssh {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_ConnectionRules_SshRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_ConnectionRules_SshRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_ConnectionRules_SshRef {
        DataZeroTrustAccessApplication_Policies_ConnectionRules_SshRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_ConnectionRules_SshRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_email_alias` after provisioning.\nEnables using Identity Provider email alias as SSH username."]
    pub fn allow_email_alias(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_email_alias", self.base))
    }

    #[doc =
        "Get a reference to the value of field `usernames` after provisioning.\nContains the Unix usernames that may be used when connecting over SSH."]
    pub fn usernames(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.usernames", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_ConnectionRules {}

impl DataZeroTrustAccessApplication_Policies_ConnectionRules { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_ConnectionRules {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_ConnectionRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_ConnectionRules {}

impl BuildDataZeroTrustAccessApplication_Policies_ConnectionRules {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_ConnectionRules {
        DataZeroTrustAccessApplication_Policies_ConnectionRules {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_ConnectionRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_ConnectionRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_ConnectionRulesRef {
        DataZeroTrustAccessApplication_Policies_ConnectionRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_ConnectionRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ssh` after provisioning.\nThe SSH-specific rules that define how users may connect to the targets secured by your application."]
    pub fn ssh(&self) -> DataZeroTrustAccessApplication_Policies_ConnectionRules_SshRef {
        DataZeroTrustAccessApplication_Policies_ConnectionRules_SshRef::new(
            self.shared().clone(),
            format!("{}.ssh", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_AnyValidServiceToken {}

impl DataZeroTrustAccessApplication_Policies_Exclude_AnyValidServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_AnyValidServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_AnyValidServiceToken {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_AnyValidServiceToken {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_AnyValidServiceToken {
        DataZeroTrustAccessApplication_Policies_Exclude_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_AnyValidServiceTokenRef {
        DataZeroTrustAccessApplication_Policies_Exclude_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_AuthContext {}

impl DataZeroTrustAccessApplication_Policies_Exclude_AuthContext { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_AuthContext {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_AuthContext {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_AuthContext {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_AuthContext {
        DataZeroTrustAccessApplication_Policies_Exclude_AuthContext {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_AuthContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_AuthContextRef {
        DataZeroTrustAccessApplication_Policies_Exclude_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_AuthContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ac_id` after provisioning.\nThe ACID of an Authentication context."]
    pub fn ac_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ac_id", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an Authentication context."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Azure identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_AuthMethod {}

impl DataZeroTrustAccessApplication_Policies_Exclude_AuthMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_AuthMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_AuthMethod {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_AuthMethod {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_AuthMethod {
        DataZeroTrustAccessApplication_Policies_Exclude_AuthMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_AuthMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_AuthMethodRef {
        DataZeroTrustAccessApplication_Policies_Exclude_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_AuthMethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_method` after provisioning.\nThe type of authentication method https://datatracker.ietf.org/doc/html/rfc8176#section-2."]
    pub fn auth_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_AzureAd {}

impl DataZeroTrustAccessApplication_Policies_Exclude_AzureAd { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_AzureAd {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_AzureAd {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_AzureAd {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_AzureAd {
        DataZeroTrustAccessApplication_Policies_Exclude_AzureAd {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_AzureAdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_AzureAdRef {
        DataZeroTrustAccessApplication_Policies_Exclude_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_AzureAdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an Azure group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Azure identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_Certificate {}

impl DataZeroTrustAccessApplication_Policies_Exclude_Certificate { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_Certificate {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_Certificate {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_Certificate {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_Certificate {
        DataZeroTrustAccessApplication_Policies_Exclude_Certificate {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_CertificateRef {
        DataZeroTrustAccessApplication_Policies_Exclude_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_CommonName {}

impl DataZeroTrustAccessApplication_Policies_Exclude_CommonName { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_CommonName {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_CommonName {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_CommonName {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_CommonName {
        DataZeroTrustAccessApplication_Policies_Exclude_CommonName {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_CommonNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_CommonNameRef {
        DataZeroTrustAccessApplication_Policies_Exclude_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_CommonNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\nThe common name to match."]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_DevicePosture {}

impl DataZeroTrustAccessApplication_Policies_Exclude_DevicePosture { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_DevicePosture {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_DevicePosture {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_DevicePosture {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_DevicePosture {
        DataZeroTrustAccessApplication_Policies_Exclude_DevicePosture {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_DevicePostureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_DevicePostureRef {
        DataZeroTrustAccessApplication_Policies_Exclude_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_DevicePostureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `integration_uid` after provisioning.\nThe ID of a device posture integration."]
    pub fn integration_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_uid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_Email {}

impl DataZeroTrustAccessApplication_Policies_Exclude_Email { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_Email {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_Email {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_Email {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_Email {
        DataZeroTrustAccessApplication_Policies_Exclude_Email {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_EmailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_EmailRef {
        DataZeroTrustAccessApplication_Policies_Exclude_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_EmailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe email of the user."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_EmailDomain {}

impl DataZeroTrustAccessApplication_Policies_Exclude_EmailDomain { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_EmailDomain {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_EmailDomain {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_EmailDomain {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_EmailDomain {
        DataZeroTrustAccessApplication_Policies_Exclude_EmailDomain {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_EmailDomainRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_EmailDomainRef {
        DataZeroTrustAccessApplication_Policies_Exclude_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_EmailDomainRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe email domain to match."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_EmailList {}

impl DataZeroTrustAccessApplication_Policies_Exclude_EmailList { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_EmailList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_EmailList {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_EmailList {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_EmailList {
        DataZeroTrustAccessApplication_Policies_Exclude_EmailList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_EmailListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_EmailListRef {
        DataZeroTrustAccessApplication_Policies_Exclude_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_EmailListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created email list."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_Everyone {}

impl DataZeroTrustAccessApplication_Policies_Exclude_Everyone { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_Everyone {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_Everyone {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_Everyone {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_Everyone {
        DataZeroTrustAccessApplication_Policies_Exclude_Everyone {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_EveryoneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_EveryoneRef {
        DataZeroTrustAccessApplication_Policies_Exclude_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_ExternalEvaluation {}

impl DataZeroTrustAccessApplication_Policies_Exclude_ExternalEvaluation { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_ExternalEvaluation {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_ExternalEvaluation {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_ExternalEvaluation {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_ExternalEvaluation {
        DataZeroTrustAccessApplication_Policies_Exclude_ExternalEvaluation {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_ExternalEvaluationRef {
        DataZeroTrustAccessApplication_Policies_Exclude_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_ExternalEvaluationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `evaluate_url` after provisioning.\nThe API endpoint containing your business logic."]
    pub fn evaluate_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluate_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keys_url` after provisioning.\nThe API endpoint containing the key that Access uses to verify that the response came from your API."]
    pub fn keys_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keys_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_Geo {}

impl DataZeroTrustAccessApplication_Policies_Exclude_Geo { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_Geo {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_Geo {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_Geo {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_Geo {
        DataZeroTrustAccessApplication_Policies_Exclude_Geo {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_GeoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_GeoRef {
        DataZeroTrustAccessApplication_Policies_Exclude_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_GeoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `country_code` after provisioning.\nThe country code that should be matched."]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_GithubOrganization {}

impl DataZeroTrustAccessApplication_Policies_Exclude_GithubOrganization { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_GithubOrganization {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_GithubOrganization {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_GithubOrganization {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_GithubOrganization {
        DataZeroTrustAccessApplication_Policies_Exclude_GithubOrganization {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_GithubOrganizationRef {
        DataZeroTrustAccessApplication_Policies_Exclude_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_GithubOrganizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Github identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the organization."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `team` after provisioning.\nThe name of the team"]
    pub fn team(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.team", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_Group {}

impl DataZeroTrustAccessApplication_Policies_Exclude_Group { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_Group {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_Group {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_Group {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_Group {
        DataZeroTrustAccessApplication_Policies_Exclude_Group {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_GroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_GroupRef {
        DataZeroTrustAccessApplication_Policies_Exclude_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_GroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created Access group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_Gsuite {}

impl DataZeroTrustAccessApplication_Policies_Exclude_Gsuite { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_Gsuite {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_Gsuite {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_Gsuite {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_Gsuite {
        DataZeroTrustAccessApplication_Policies_Exclude_Gsuite {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_GsuiteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_GsuiteRef {
        DataZeroTrustAccessApplication_Policies_Exclude_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_GsuiteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nThe email of the Google Workspace group."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Google Workspace identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_Ip {}

impl DataZeroTrustAccessApplication_Policies_Exclude_Ip { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_Ip {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_Ip {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_Ip {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_Ip {
        DataZeroTrustAccessApplication_Policies_Exclude_Ip {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_IpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_IpRef {
        DataZeroTrustAccessApplication_Policies_Exclude_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_IpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\nAn IPv4 or IPv6 CIDR block."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_IpList {}

impl DataZeroTrustAccessApplication_Policies_Exclude_IpList { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_IpList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_IpList {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_IpList {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_IpList {
        DataZeroTrustAccessApplication_Policies_Exclude_IpList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_IpListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_IpListRef {
        DataZeroTrustAccessApplication_Policies_Exclude_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_IpListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created IP list."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_LinkedAppToken {}

impl DataZeroTrustAccessApplication_Policies_Exclude_LinkedAppToken { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_LinkedAppToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_LinkedAppToken {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_LinkedAppToken {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_LinkedAppToken {
        DataZeroTrustAccessApplication_Policies_Exclude_LinkedAppToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_LinkedAppTokenRef {
        DataZeroTrustAccessApplication_Policies_Exclude_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_LinkedAppTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_uid` after provisioning.\nThe ID of an Access OIDC SaaS application"]
    pub fn app_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_uid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_LoginMethod {}

impl DataZeroTrustAccessApplication_Policies_Exclude_LoginMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_LoginMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_LoginMethod {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_LoginMethod {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_LoginMethod {
        DataZeroTrustAccessApplication_Policies_Exclude_LoginMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_LoginMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_LoginMethodRef {
        DataZeroTrustAccessApplication_Policies_Exclude_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_LoginMethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an identity provider."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_Oidc {}

impl DataZeroTrustAccessApplication_Policies_Exclude_Oidc { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_Oidc {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_Oidc {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_Oidc {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_Oidc {
        DataZeroTrustAccessApplication_Policies_Exclude_Oidc {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_OidcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_OidcRef {
        DataZeroTrustAccessApplication_Policies_Exclude_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_OidcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim_name` after provisioning.\nThe name of the OIDC claim."]
    pub fn claim_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `claim_value` after provisioning.\nThe OIDC claim value to look for."]
    pub fn claim_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your OIDC identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_Okta {}

impl DataZeroTrustAccessApplication_Policies_Exclude_Okta { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_Okta {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_Okta {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_Okta {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_Okta {
        DataZeroTrustAccessApplication_Policies_Exclude_Okta {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_OktaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_OktaRef {
        DataZeroTrustAccessApplication_Policies_Exclude_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_OktaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Okta identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Okta group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_Saml {}

impl DataZeroTrustAccessApplication_Policies_Exclude_Saml { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_Saml {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_Saml {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_Saml {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_Saml {
        DataZeroTrustAccessApplication_Policies_Exclude_Saml {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_SamlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_SamlRef {
        DataZeroTrustAccessApplication_Policies_Exclude_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_SamlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute_name` after provisioning.\nThe name of the SAML attribute."]
    pub fn attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `attribute_value` after provisioning.\nThe SAML attribute value to look for."]
    pub fn attribute_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your SAML identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_ServiceToken {}

impl DataZeroTrustAccessApplication_Policies_Exclude_ServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude_ServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude_ServiceToken {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude_ServiceToken {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude_ServiceToken {
        DataZeroTrustAccessApplication_Policies_Exclude_ServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Exclude_ServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Exclude_ServiceTokenRef {
        DataZeroTrustAccessApplication_Policies_Exclude_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Exclude_ServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token_id` after provisioning.\nThe ID of a Service Token."]
    pub fn token_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Exclude {}

impl DataZeroTrustAccessApplication_Policies_Exclude { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Exclude {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Exclude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Exclude {}

impl BuildDataZeroTrustAccessApplication_Policies_Exclude {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Exclude {
        DataZeroTrustAccessApplication_Policies_Exclude {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_ExcludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_ExcludeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_ExcludeRef {
        DataZeroTrustAccessApplication_Policies_ExcludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_ExcludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_AnyValidServiceTokenRef {
        DataZeroTrustAccessApplication_Policies_Exclude_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_AuthContextRef {
        DataZeroTrustAccessApplication_Policies_Exclude_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_AuthMethodRef {
        DataZeroTrustAccessApplication_Policies_Exclude_AuthMethodRef::new(
            self.shared().clone(),
            format!("{}.auth_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_AzureAdRef {
        DataZeroTrustAccessApplication_Policies_Exclude_AzureAdRef::new(
            self.shared().clone(),
            format!("{}.azure_ad", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_CertificateRef {
        DataZeroTrustAccessApplication_Policies_Exclude_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_CommonNameRef {
        DataZeroTrustAccessApplication_Policies_Exclude_CommonNameRef::new(
            self.shared().clone(),
            format!("{}.common_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_DevicePostureRef {
        DataZeroTrustAccessApplication_Policies_Exclude_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_EmailRef {
        DataZeroTrustAccessApplication_Policies_Exclude_EmailRef::new(
            self.shared().clone(),
            format!("{}.email", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_EmailDomainRef {
        DataZeroTrustAccessApplication_Policies_Exclude_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_EmailListRef {
        DataZeroTrustAccessApplication_Policies_Exclude_EmailListRef::new(
            self.shared().clone(),
            format!("{}.email_list", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_EveryoneRef {
        DataZeroTrustAccessApplication_Policies_Exclude_EveryoneRef::new(
            self.shared().clone(),
            format!("{}.everyone", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_ExternalEvaluationRef {
        DataZeroTrustAccessApplication_Policies_Exclude_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_GeoRef {
        DataZeroTrustAccessApplication_Policies_Exclude_GeoRef::new(
            self.shared().clone(),
            format!("{}.geo", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_GithubOrganizationRef {
        DataZeroTrustAccessApplication_Policies_Exclude_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_GroupRef {
        DataZeroTrustAccessApplication_Policies_Exclude_GroupRef::new(
            self.shared().clone(),
            format!("{}.group", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_GsuiteRef {
        DataZeroTrustAccessApplication_Policies_Exclude_GsuiteRef::new(
            self.shared().clone(),
            format!("{}.gsuite", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_IpRef {
        DataZeroTrustAccessApplication_Policies_Exclude_IpRef::new(
            self.shared().clone(),
            format!("{}.ip", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_IpListRef {
        DataZeroTrustAccessApplication_Policies_Exclude_IpListRef::new(
            self.shared().clone(),
            format!("{}.ip_list", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_LinkedAppTokenRef {
        DataZeroTrustAccessApplication_Policies_Exclude_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_LoginMethodRef {
        DataZeroTrustAccessApplication_Policies_Exclude_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_OidcRef {
        DataZeroTrustAccessApplication_Policies_Exclude_OidcRef::new(
            self.shared().clone(),
            format!("{}.oidc", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_OktaRef {
        DataZeroTrustAccessApplication_Policies_Exclude_OktaRef::new(
            self.shared().clone(),
            format!("{}.okta", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_SamlRef {
        DataZeroTrustAccessApplication_Policies_Exclude_SamlRef::new(
            self.shared().clone(),
            format!("{}.saml", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> DataZeroTrustAccessApplication_Policies_Exclude_ServiceTokenRef {
        DataZeroTrustAccessApplication_Policies_Exclude_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_AnyValidServiceToken {}

impl DataZeroTrustAccessApplication_Policies_Include_AnyValidServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_AnyValidServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_AnyValidServiceToken {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_AnyValidServiceToken {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_AnyValidServiceToken {
        DataZeroTrustAccessApplication_Policies_Include_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_AnyValidServiceTokenRef {
        DataZeroTrustAccessApplication_Policies_Include_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_AuthContext {}

impl DataZeroTrustAccessApplication_Policies_Include_AuthContext { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_AuthContext {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_AuthContext {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_AuthContext {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_AuthContext {
        DataZeroTrustAccessApplication_Policies_Include_AuthContext {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_AuthContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_AuthContextRef {
        DataZeroTrustAccessApplication_Policies_Include_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_AuthContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ac_id` after provisioning.\nThe ACID of an Authentication context."]
    pub fn ac_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ac_id", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an Authentication context."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Azure identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_AuthMethod {}

impl DataZeroTrustAccessApplication_Policies_Include_AuthMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_AuthMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_AuthMethod {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_AuthMethod {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_AuthMethod {
        DataZeroTrustAccessApplication_Policies_Include_AuthMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_AuthMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_AuthMethodRef {
        DataZeroTrustAccessApplication_Policies_Include_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_AuthMethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_method` after provisioning.\nThe type of authentication method https://datatracker.ietf.org/doc/html/rfc8176#section-2."]
    pub fn auth_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_AzureAd {}

impl DataZeroTrustAccessApplication_Policies_Include_AzureAd { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_AzureAd {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_AzureAd {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_AzureAd {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_AzureAd {
        DataZeroTrustAccessApplication_Policies_Include_AzureAd {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_AzureAdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_AzureAdRef {
        DataZeroTrustAccessApplication_Policies_Include_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_AzureAdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an Azure group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Azure identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_Certificate {}

impl DataZeroTrustAccessApplication_Policies_Include_Certificate { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_Certificate {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_Certificate {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_Certificate {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_Certificate {
        DataZeroTrustAccessApplication_Policies_Include_Certificate {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_CertificateRef {
        DataZeroTrustAccessApplication_Policies_Include_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_CommonName {}

impl DataZeroTrustAccessApplication_Policies_Include_CommonName { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_CommonName {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_CommonName {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_CommonName {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_CommonName {
        DataZeroTrustAccessApplication_Policies_Include_CommonName {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_CommonNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_CommonNameRef {
        DataZeroTrustAccessApplication_Policies_Include_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_CommonNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\nThe common name to match."]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_DevicePosture {}

impl DataZeroTrustAccessApplication_Policies_Include_DevicePosture { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_DevicePosture {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_DevicePosture {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_DevicePosture {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_DevicePosture {
        DataZeroTrustAccessApplication_Policies_Include_DevicePosture {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_DevicePostureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_DevicePostureRef {
        DataZeroTrustAccessApplication_Policies_Include_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_DevicePostureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `integration_uid` after provisioning.\nThe ID of a device posture integration."]
    pub fn integration_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_uid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_Email {}

impl DataZeroTrustAccessApplication_Policies_Include_Email { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_Email {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_Email {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_Email {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_Email {
        DataZeroTrustAccessApplication_Policies_Include_Email {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_EmailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_EmailRef {
        DataZeroTrustAccessApplication_Policies_Include_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_EmailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe email of the user."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_EmailDomain {}

impl DataZeroTrustAccessApplication_Policies_Include_EmailDomain { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_EmailDomain {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_EmailDomain {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_EmailDomain {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_EmailDomain {
        DataZeroTrustAccessApplication_Policies_Include_EmailDomain {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_EmailDomainRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_EmailDomainRef {
        DataZeroTrustAccessApplication_Policies_Include_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_EmailDomainRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe email domain to match."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_EmailList {}

impl DataZeroTrustAccessApplication_Policies_Include_EmailList { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_EmailList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_EmailList {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_EmailList {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_EmailList {
        DataZeroTrustAccessApplication_Policies_Include_EmailList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_EmailListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_EmailListRef {
        DataZeroTrustAccessApplication_Policies_Include_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_EmailListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created email list."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_Everyone {}

impl DataZeroTrustAccessApplication_Policies_Include_Everyone { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_Everyone {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_Everyone {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_Everyone {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_Everyone {
        DataZeroTrustAccessApplication_Policies_Include_Everyone {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_EveryoneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_EveryoneRef {
        DataZeroTrustAccessApplication_Policies_Include_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_ExternalEvaluation {}

impl DataZeroTrustAccessApplication_Policies_Include_ExternalEvaluation { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_ExternalEvaluation {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_ExternalEvaluation {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_ExternalEvaluation {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_ExternalEvaluation {
        DataZeroTrustAccessApplication_Policies_Include_ExternalEvaluation {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_ExternalEvaluationRef {
        DataZeroTrustAccessApplication_Policies_Include_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_ExternalEvaluationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `evaluate_url` after provisioning.\nThe API endpoint containing your business logic."]
    pub fn evaluate_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluate_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keys_url` after provisioning.\nThe API endpoint containing the key that Access uses to verify that the response came from your API."]
    pub fn keys_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keys_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_Geo {}

impl DataZeroTrustAccessApplication_Policies_Include_Geo { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_Geo {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_Geo {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_Geo {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_Geo {
        DataZeroTrustAccessApplication_Policies_Include_Geo {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_GeoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_GeoRef {
        DataZeroTrustAccessApplication_Policies_Include_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_GeoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `country_code` after provisioning.\nThe country code that should be matched."]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_GithubOrganization {}

impl DataZeroTrustAccessApplication_Policies_Include_GithubOrganization { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_GithubOrganization {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_GithubOrganization {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_GithubOrganization {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_GithubOrganization {
        DataZeroTrustAccessApplication_Policies_Include_GithubOrganization {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_GithubOrganizationRef {
        DataZeroTrustAccessApplication_Policies_Include_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_GithubOrganizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Github identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the organization."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `team` after provisioning.\nThe name of the team"]
    pub fn team(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.team", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_Group {}

impl DataZeroTrustAccessApplication_Policies_Include_Group { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_Group {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_Group {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_Group {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_Group {
        DataZeroTrustAccessApplication_Policies_Include_Group {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_GroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_GroupRef {
        DataZeroTrustAccessApplication_Policies_Include_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_GroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created Access group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_Gsuite {}

impl DataZeroTrustAccessApplication_Policies_Include_Gsuite { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_Gsuite {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_Gsuite {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_Gsuite {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_Gsuite {
        DataZeroTrustAccessApplication_Policies_Include_Gsuite {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_GsuiteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_GsuiteRef {
        DataZeroTrustAccessApplication_Policies_Include_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_GsuiteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nThe email of the Google Workspace group."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Google Workspace identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_Ip {}

impl DataZeroTrustAccessApplication_Policies_Include_Ip { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_Ip {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_Ip {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_Ip {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_Ip {
        DataZeroTrustAccessApplication_Policies_Include_Ip {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_IpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_IpRef {
        DataZeroTrustAccessApplication_Policies_Include_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_IpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\nAn IPv4 or IPv6 CIDR block."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_IpList {}

impl DataZeroTrustAccessApplication_Policies_Include_IpList { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_IpList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_IpList {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_IpList {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_IpList {
        DataZeroTrustAccessApplication_Policies_Include_IpList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_IpListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_IpListRef {
        DataZeroTrustAccessApplication_Policies_Include_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_IpListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created IP list."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_LinkedAppToken {}

impl DataZeroTrustAccessApplication_Policies_Include_LinkedAppToken { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_LinkedAppToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_LinkedAppToken {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_LinkedAppToken {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_LinkedAppToken {
        DataZeroTrustAccessApplication_Policies_Include_LinkedAppToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_LinkedAppTokenRef {
        DataZeroTrustAccessApplication_Policies_Include_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_LinkedAppTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_uid` after provisioning.\nThe ID of an Access OIDC SaaS application"]
    pub fn app_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_uid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_LoginMethod {}

impl DataZeroTrustAccessApplication_Policies_Include_LoginMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_LoginMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_LoginMethod {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_LoginMethod {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_LoginMethod {
        DataZeroTrustAccessApplication_Policies_Include_LoginMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_LoginMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_LoginMethodRef {
        DataZeroTrustAccessApplication_Policies_Include_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_LoginMethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an identity provider."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_Oidc {}

impl DataZeroTrustAccessApplication_Policies_Include_Oidc { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_Oidc {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_Oidc {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_Oidc {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_Oidc {
        DataZeroTrustAccessApplication_Policies_Include_Oidc {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_OidcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_OidcRef {
        DataZeroTrustAccessApplication_Policies_Include_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_OidcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim_name` after provisioning.\nThe name of the OIDC claim."]
    pub fn claim_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `claim_value` after provisioning.\nThe OIDC claim value to look for."]
    pub fn claim_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your OIDC identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_Okta {}

impl DataZeroTrustAccessApplication_Policies_Include_Okta { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_Okta {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_Okta {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_Okta {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_Okta {
        DataZeroTrustAccessApplication_Policies_Include_Okta {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_OktaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_OktaRef {
        DataZeroTrustAccessApplication_Policies_Include_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_OktaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Okta identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Okta group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_Saml {}

impl DataZeroTrustAccessApplication_Policies_Include_Saml { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_Saml {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_Saml {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_Saml {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_Saml {
        DataZeroTrustAccessApplication_Policies_Include_Saml {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_SamlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_SamlRef {
        DataZeroTrustAccessApplication_Policies_Include_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_SamlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute_name` after provisioning.\nThe name of the SAML attribute."]
    pub fn attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `attribute_value` after provisioning.\nThe SAML attribute value to look for."]
    pub fn attribute_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your SAML identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include_ServiceToken {}

impl DataZeroTrustAccessApplication_Policies_Include_ServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include_ServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include_ServiceToken {}

impl BuildDataZeroTrustAccessApplication_Policies_Include_ServiceToken {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include_ServiceToken {
        DataZeroTrustAccessApplication_Policies_Include_ServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Include_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Include_ServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Include_ServiceTokenRef {
        DataZeroTrustAccessApplication_Policies_Include_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Include_ServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token_id` after provisioning.\nThe ID of a Service Token."]
    pub fn token_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Include {}

impl DataZeroTrustAccessApplication_Policies_Include { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Include {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Include>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Include {}

impl BuildDataZeroTrustAccessApplication_Policies_Include {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Include {
        DataZeroTrustAccessApplication_Policies_Include {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_IncludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_IncludeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_IncludeRef {
        DataZeroTrustAccessApplication_Policies_IncludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_IncludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> DataZeroTrustAccessApplication_Policies_Include_AnyValidServiceTokenRef {
        DataZeroTrustAccessApplication_Policies_Include_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> DataZeroTrustAccessApplication_Policies_Include_AuthContextRef {
        DataZeroTrustAccessApplication_Policies_Include_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> DataZeroTrustAccessApplication_Policies_Include_AuthMethodRef {
        DataZeroTrustAccessApplication_Policies_Include_AuthMethodRef::new(
            self.shared().clone(),
            format!("{}.auth_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> DataZeroTrustAccessApplication_Policies_Include_AzureAdRef {
        DataZeroTrustAccessApplication_Policies_Include_AzureAdRef::new(
            self.shared().clone(),
            format!("{}.azure_ad", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> DataZeroTrustAccessApplication_Policies_Include_CertificateRef {
        DataZeroTrustAccessApplication_Policies_Include_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> DataZeroTrustAccessApplication_Policies_Include_CommonNameRef {
        DataZeroTrustAccessApplication_Policies_Include_CommonNameRef::new(
            self.shared().clone(),
            format!("{}.common_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> DataZeroTrustAccessApplication_Policies_Include_DevicePostureRef {
        DataZeroTrustAccessApplication_Policies_Include_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> DataZeroTrustAccessApplication_Policies_Include_EmailRef {
        DataZeroTrustAccessApplication_Policies_Include_EmailRef::new(
            self.shared().clone(),
            format!("{}.email", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> DataZeroTrustAccessApplication_Policies_Include_EmailDomainRef {
        DataZeroTrustAccessApplication_Policies_Include_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> DataZeroTrustAccessApplication_Policies_Include_EmailListRef {
        DataZeroTrustAccessApplication_Policies_Include_EmailListRef::new(
            self.shared().clone(),
            format!("{}.email_list", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> DataZeroTrustAccessApplication_Policies_Include_EveryoneRef {
        DataZeroTrustAccessApplication_Policies_Include_EveryoneRef::new(
            self.shared().clone(),
            format!("{}.everyone", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> DataZeroTrustAccessApplication_Policies_Include_ExternalEvaluationRef {
        DataZeroTrustAccessApplication_Policies_Include_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> DataZeroTrustAccessApplication_Policies_Include_GeoRef {
        DataZeroTrustAccessApplication_Policies_Include_GeoRef::new(
            self.shared().clone(),
            format!("{}.geo", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> DataZeroTrustAccessApplication_Policies_Include_GithubOrganizationRef {
        DataZeroTrustAccessApplication_Policies_Include_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> DataZeroTrustAccessApplication_Policies_Include_GroupRef {
        DataZeroTrustAccessApplication_Policies_Include_GroupRef::new(
            self.shared().clone(),
            format!("{}.group", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> DataZeroTrustAccessApplication_Policies_Include_GsuiteRef {
        DataZeroTrustAccessApplication_Policies_Include_GsuiteRef::new(
            self.shared().clone(),
            format!("{}.gsuite", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> DataZeroTrustAccessApplication_Policies_Include_IpRef {
        DataZeroTrustAccessApplication_Policies_Include_IpRef::new(
            self.shared().clone(),
            format!("{}.ip", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> DataZeroTrustAccessApplication_Policies_Include_IpListRef {
        DataZeroTrustAccessApplication_Policies_Include_IpListRef::new(
            self.shared().clone(),
            format!("{}.ip_list", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> DataZeroTrustAccessApplication_Policies_Include_LinkedAppTokenRef {
        DataZeroTrustAccessApplication_Policies_Include_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> DataZeroTrustAccessApplication_Policies_Include_LoginMethodRef {
        DataZeroTrustAccessApplication_Policies_Include_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> DataZeroTrustAccessApplication_Policies_Include_OidcRef {
        DataZeroTrustAccessApplication_Policies_Include_OidcRef::new(
            self.shared().clone(),
            format!("{}.oidc", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> DataZeroTrustAccessApplication_Policies_Include_OktaRef {
        DataZeroTrustAccessApplication_Policies_Include_OktaRef::new(
            self.shared().clone(),
            format!("{}.okta", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> DataZeroTrustAccessApplication_Policies_Include_SamlRef {
        DataZeroTrustAccessApplication_Policies_Include_SamlRef::new(
            self.shared().clone(),
            format!("{}.saml", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> DataZeroTrustAccessApplication_Policies_Include_ServiceTokenRef {
        DataZeroTrustAccessApplication_Policies_Include_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_MfaConfig {}

impl DataZeroTrustAccessApplication_Policies_MfaConfig { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_MfaConfig {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_MfaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_MfaConfig {}

impl BuildDataZeroTrustAccessApplication_Policies_MfaConfig {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_MfaConfig {
        DataZeroTrustAccessApplication_Policies_MfaConfig {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_MfaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_MfaConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_MfaConfigRef {
        DataZeroTrustAccessApplication_Policies_MfaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_MfaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_authenticators` after provisioning.\nThe authenticators allowed for MFA.\nAvailable values: \"totp\", \"biometrics\", \"security_key\", \"ssh_piv_key\"."]
    pub fn allowed_authenticators(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_authenticators", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mfa_disabled` after provisioning.\nWhether MFA is disabled for this policy."]
    pub fn mfa_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mfa_disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nHow often a user will be forced to re-authenticate with MFA."]
    pub fn session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_AnyValidServiceToken {}

impl DataZeroTrustAccessApplication_Policies_Require_AnyValidServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_AnyValidServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_AnyValidServiceToken {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_AnyValidServiceToken {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_AnyValidServiceToken {
        DataZeroTrustAccessApplication_Policies_Require_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_AnyValidServiceTokenRef {
        DataZeroTrustAccessApplication_Policies_Require_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_AuthContext {}

impl DataZeroTrustAccessApplication_Policies_Require_AuthContext { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_AuthContext {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_AuthContext {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_AuthContext {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_AuthContext {
        DataZeroTrustAccessApplication_Policies_Require_AuthContext {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_AuthContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_AuthContextRef {
        DataZeroTrustAccessApplication_Policies_Require_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_AuthContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ac_id` after provisioning.\nThe ACID of an Authentication context."]
    pub fn ac_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ac_id", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an Authentication context."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Azure identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_AuthMethod {}

impl DataZeroTrustAccessApplication_Policies_Require_AuthMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_AuthMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_AuthMethod {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_AuthMethod {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_AuthMethod {
        DataZeroTrustAccessApplication_Policies_Require_AuthMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_AuthMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_AuthMethodRef {
        DataZeroTrustAccessApplication_Policies_Require_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_AuthMethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_method` after provisioning.\nThe type of authentication method https://datatracker.ietf.org/doc/html/rfc8176#section-2."]
    pub fn auth_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_AzureAd {}

impl DataZeroTrustAccessApplication_Policies_Require_AzureAd { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_AzureAd {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_AzureAd {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_AzureAd {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_AzureAd {
        DataZeroTrustAccessApplication_Policies_Require_AzureAd {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_AzureAdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_AzureAdRef {
        DataZeroTrustAccessApplication_Policies_Require_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_AzureAdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an Azure group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Azure identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_Certificate {}

impl DataZeroTrustAccessApplication_Policies_Require_Certificate { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_Certificate {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_Certificate {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_Certificate {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_Certificate {
        DataZeroTrustAccessApplication_Policies_Require_Certificate {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_CertificateRef {
        DataZeroTrustAccessApplication_Policies_Require_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_CommonName {}

impl DataZeroTrustAccessApplication_Policies_Require_CommonName { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_CommonName {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_CommonName {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_CommonName {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_CommonName {
        DataZeroTrustAccessApplication_Policies_Require_CommonName {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_CommonNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_CommonNameRef {
        DataZeroTrustAccessApplication_Policies_Require_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_CommonNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\nThe common name to match."]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_DevicePosture {}

impl DataZeroTrustAccessApplication_Policies_Require_DevicePosture { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_DevicePosture {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_DevicePosture {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_DevicePosture {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_DevicePosture {
        DataZeroTrustAccessApplication_Policies_Require_DevicePosture {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_DevicePostureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_DevicePostureRef {
        DataZeroTrustAccessApplication_Policies_Require_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_DevicePostureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `integration_uid` after provisioning.\nThe ID of a device posture integration."]
    pub fn integration_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_uid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_Email {}

impl DataZeroTrustAccessApplication_Policies_Require_Email { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_Email {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_Email {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_Email {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_Email {
        DataZeroTrustAccessApplication_Policies_Require_Email {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_EmailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_EmailRef {
        DataZeroTrustAccessApplication_Policies_Require_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_EmailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe email of the user."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_EmailDomain {}

impl DataZeroTrustAccessApplication_Policies_Require_EmailDomain { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_EmailDomain {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_EmailDomain {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_EmailDomain {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_EmailDomain {
        DataZeroTrustAccessApplication_Policies_Require_EmailDomain {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_EmailDomainRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_EmailDomainRef {
        DataZeroTrustAccessApplication_Policies_Require_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_EmailDomainRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe email domain to match."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_EmailList {}

impl DataZeroTrustAccessApplication_Policies_Require_EmailList { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_EmailList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_EmailList {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_EmailList {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_EmailList {
        DataZeroTrustAccessApplication_Policies_Require_EmailList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_EmailListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_EmailListRef {
        DataZeroTrustAccessApplication_Policies_Require_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_EmailListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created email list."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_Everyone {}

impl DataZeroTrustAccessApplication_Policies_Require_Everyone { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_Everyone {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_Everyone {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_Everyone {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_Everyone {
        DataZeroTrustAccessApplication_Policies_Require_Everyone {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_EveryoneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_EveryoneRef {
        DataZeroTrustAccessApplication_Policies_Require_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_ExternalEvaluation {}

impl DataZeroTrustAccessApplication_Policies_Require_ExternalEvaluation { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_ExternalEvaluation {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_ExternalEvaluation {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_ExternalEvaluation {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_ExternalEvaluation {
        DataZeroTrustAccessApplication_Policies_Require_ExternalEvaluation {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_ExternalEvaluationRef {
        DataZeroTrustAccessApplication_Policies_Require_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_ExternalEvaluationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `evaluate_url` after provisioning.\nThe API endpoint containing your business logic."]
    pub fn evaluate_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluate_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keys_url` after provisioning.\nThe API endpoint containing the key that Access uses to verify that the response came from your API."]
    pub fn keys_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keys_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_Geo {}

impl DataZeroTrustAccessApplication_Policies_Require_Geo { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_Geo {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_Geo {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_Geo {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_Geo {
        DataZeroTrustAccessApplication_Policies_Require_Geo {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_GeoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_GeoRef {
        DataZeroTrustAccessApplication_Policies_Require_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_GeoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `country_code` after provisioning.\nThe country code that should be matched."]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_GithubOrganization {}

impl DataZeroTrustAccessApplication_Policies_Require_GithubOrganization { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_GithubOrganization {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_GithubOrganization {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_GithubOrganization {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_GithubOrganization {
        DataZeroTrustAccessApplication_Policies_Require_GithubOrganization {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_GithubOrganizationRef {
        DataZeroTrustAccessApplication_Policies_Require_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_GithubOrganizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Github identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the organization."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `team` after provisioning.\nThe name of the team"]
    pub fn team(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.team", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_Group {}

impl DataZeroTrustAccessApplication_Policies_Require_Group { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_Group {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_Group {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_Group {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_Group {
        DataZeroTrustAccessApplication_Policies_Require_Group {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_GroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_GroupRef {
        DataZeroTrustAccessApplication_Policies_Require_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_GroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created Access group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_Gsuite {}

impl DataZeroTrustAccessApplication_Policies_Require_Gsuite { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_Gsuite {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_Gsuite {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_Gsuite {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_Gsuite {
        DataZeroTrustAccessApplication_Policies_Require_Gsuite {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_GsuiteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_GsuiteRef {
        DataZeroTrustAccessApplication_Policies_Require_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_GsuiteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nThe email of the Google Workspace group."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Google Workspace identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_Ip {}

impl DataZeroTrustAccessApplication_Policies_Require_Ip { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_Ip {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_Ip {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_Ip {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_Ip {
        DataZeroTrustAccessApplication_Policies_Require_Ip {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_IpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_IpRef {
        DataZeroTrustAccessApplication_Policies_Require_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_IpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\nAn IPv4 or IPv6 CIDR block."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_IpList {}

impl DataZeroTrustAccessApplication_Policies_Require_IpList { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_IpList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_IpList {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_IpList {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_IpList {
        DataZeroTrustAccessApplication_Policies_Require_IpList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_IpListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_IpListRef {
        DataZeroTrustAccessApplication_Policies_Require_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_IpListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created IP list."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_LinkedAppToken {}

impl DataZeroTrustAccessApplication_Policies_Require_LinkedAppToken { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_LinkedAppToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_LinkedAppToken {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_LinkedAppToken {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_LinkedAppToken {
        DataZeroTrustAccessApplication_Policies_Require_LinkedAppToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_LinkedAppTokenRef {
        DataZeroTrustAccessApplication_Policies_Require_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_LinkedAppTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_uid` after provisioning.\nThe ID of an Access OIDC SaaS application"]
    pub fn app_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_uid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_LoginMethod {}

impl DataZeroTrustAccessApplication_Policies_Require_LoginMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_LoginMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_LoginMethod {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_LoginMethod {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_LoginMethod {
        DataZeroTrustAccessApplication_Policies_Require_LoginMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_LoginMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_LoginMethodRef {
        DataZeroTrustAccessApplication_Policies_Require_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_LoginMethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an identity provider."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_Oidc {}

impl DataZeroTrustAccessApplication_Policies_Require_Oidc { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_Oidc {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_Oidc {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_Oidc {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_Oidc {
        DataZeroTrustAccessApplication_Policies_Require_Oidc {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_OidcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_OidcRef {
        DataZeroTrustAccessApplication_Policies_Require_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_OidcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim_name` after provisioning.\nThe name of the OIDC claim."]
    pub fn claim_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `claim_value` after provisioning.\nThe OIDC claim value to look for."]
    pub fn claim_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your OIDC identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_Okta {}

impl DataZeroTrustAccessApplication_Policies_Require_Okta { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_Okta {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_Okta {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_Okta {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_Okta {
        DataZeroTrustAccessApplication_Policies_Require_Okta {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_OktaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_OktaRef {
        DataZeroTrustAccessApplication_Policies_Require_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_OktaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Okta identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Okta group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_Saml {}

impl DataZeroTrustAccessApplication_Policies_Require_Saml { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_Saml {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_Saml {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_Saml {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_Saml {
        DataZeroTrustAccessApplication_Policies_Require_Saml {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_SamlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_SamlRef {
        DataZeroTrustAccessApplication_Policies_Require_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_SamlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute_name` after provisioning.\nThe name of the SAML attribute."]
    pub fn attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `attribute_value` after provisioning.\nThe SAML attribute value to look for."]
    pub fn attribute_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your SAML identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require_ServiceToken {}

impl DataZeroTrustAccessApplication_Policies_Require_ServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require_ServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require_ServiceToken {}

impl BuildDataZeroTrustAccessApplication_Policies_Require_ServiceToken {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require_ServiceToken {
        DataZeroTrustAccessApplication_Policies_Require_ServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_Require_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_Require_ServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_Require_ServiceTokenRef {
        DataZeroTrustAccessApplication_Policies_Require_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_Require_ServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token_id` after provisioning.\nThe ID of a Service Token."]
    pub fn token_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies_Require {}

impl DataZeroTrustAccessApplication_Policies_Require { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies_Require {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies_Require>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies_Require {}

impl BuildDataZeroTrustAccessApplication_Policies_Require {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies_Require {
        DataZeroTrustAccessApplication_Policies_Require {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_Policies_RequireRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_Policies_RequireRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_Policies_RequireRef {
        DataZeroTrustAccessApplication_Policies_RequireRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_Policies_RequireRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> DataZeroTrustAccessApplication_Policies_Require_AnyValidServiceTokenRef {
        DataZeroTrustAccessApplication_Policies_Require_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> DataZeroTrustAccessApplication_Policies_Require_AuthContextRef {
        DataZeroTrustAccessApplication_Policies_Require_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> DataZeroTrustAccessApplication_Policies_Require_AuthMethodRef {
        DataZeroTrustAccessApplication_Policies_Require_AuthMethodRef::new(
            self.shared().clone(),
            format!("{}.auth_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> DataZeroTrustAccessApplication_Policies_Require_AzureAdRef {
        DataZeroTrustAccessApplication_Policies_Require_AzureAdRef::new(
            self.shared().clone(),
            format!("{}.azure_ad", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> DataZeroTrustAccessApplication_Policies_Require_CertificateRef {
        DataZeroTrustAccessApplication_Policies_Require_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> DataZeroTrustAccessApplication_Policies_Require_CommonNameRef {
        DataZeroTrustAccessApplication_Policies_Require_CommonNameRef::new(
            self.shared().clone(),
            format!("{}.common_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> DataZeroTrustAccessApplication_Policies_Require_DevicePostureRef {
        DataZeroTrustAccessApplication_Policies_Require_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> DataZeroTrustAccessApplication_Policies_Require_EmailRef {
        DataZeroTrustAccessApplication_Policies_Require_EmailRef::new(
            self.shared().clone(),
            format!("{}.email", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> DataZeroTrustAccessApplication_Policies_Require_EmailDomainRef {
        DataZeroTrustAccessApplication_Policies_Require_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> DataZeroTrustAccessApplication_Policies_Require_EmailListRef {
        DataZeroTrustAccessApplication_Policies_Require_EmailListRef::new(
            self.shared().clone(),
            format!("{}.email_list", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> DataZeroTrustAccessApplication_Policies_Require_EveryoneRef {
        DataZeroTrustAccessApplication_Policies_Require_EveryoneRef::new(
            self.shared().clone(),
            format!("{}.everyone", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> DataZeroTrustAccessApplication_Policies_Require_ExternalEvaluationRef {
        DataZeroTrustAccessApplication_Policies_Require_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> DataZeroTrustAccessApplication_Policies_Require_GeoRef {
        DataZeroTrustAccessApplication_Policies_Require_GeoRef::new(
            self.shared().clone(),
            format!("{}.geo", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> DataZeroTrustAccessApplication_Policies_Require_GithubOrganizationRef {
        DataZeroTrustAccessApplication_Policies_Require_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> DataZeroTrustAccessApplication_Policies_Require_GroupRef {
        DataZeroTrustAccessApplication_Policies_Require_GroupRef::new(
            self.shared().clone(),
            format!("{}.group", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> DataZeroTrustAccessApplication_Policies_Require_GsuiteRef {
        DataZeroTrustAccessApplication_Policies_Require_GsuiteRef::new(
            self.shared().clone(),
            format!("{}.gsuite", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> DataZeroTrustAccessApplication_Policies_Require_IpRef {
        DataZeroTrustAccessApplication_Policies_Require_IpRef::new(
            self.shared().clone(),
            format!("{}.ip", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> DataZeroTrustAccessApplication_Policies_Require_IpListRef {
        DataZeroTrustAccessApplication_Policies_Require_IpListRef::new(
            self.shared().clone(),
            format!("{}.ip_list", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> DataZeroTrustAccessApplication_Policies_Require_LinkedAppTokenRef {
        DataZeroTrustAccessApplication_Policies_Require_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> DataZeroTrustAccessApplication_Policies_Require_LoginMethodRef {
        DataZeroTrustAccessApplication_Policies_Require_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> DataZeroTrustAccessApplication_Policies_Require_OidcRef {
        DataZeroTrustAccessApplication_Policies_Require_OidcRef::new(
            self.shared().clone(),
            format!("{}.oidc", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> DataZeroTrustAccessApplication_Policies_Require_OktaRef {
        DataZeroTrustAccessApplication_Policies_Require_OktaRef::new(
            self.shared().clone(),
            format!("{}.okta", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> DataZeroTrustAccessApplication_Policies_Require_SamlRef {
        DataZeroTrustAccessApplication_Policies_Require_SamlRef::new(
            self.shared().clone(),
            format!("{}.saml", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> DataZeroTrustAccessApplication_Policies_Require_ServiceTokenRef {
        DataZeroTrustAccessApplication_Policies_Require_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_Policies {}

impl DataZeroTrustAccessApplication_Policies { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_Policies {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_Policies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_Policies {}

impl BuildDataZeroTrustAccessApplication_Policies {
    pub fn build(self) -> DataZeroTrustAccessApplication_Policies {
        DataZeroTrustAccessApplication_Policies {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_PoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_PoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessApplication_PoliciesRef {
        DataZeroTrustAccessApplication_PoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_PoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `approval_groups` after provisioning.\nAdministrators who can approve a temporary authentication request."]
    pub fn approval_groups(&self) -> terrars::SetRef<DataZeroTrustAccessApplication_Policies_ApprovalGroupsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.approval_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `approval_required` after provisioning.\nRequires the user to request access from an administrator at the start of each session."]
    pub fn approval_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approval_required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connection_rules` after provisioning.\nThe rules that define how users may connect to the targets secured by your application."]
    pub fn connection_rules(&self) -> DataZeroTrustAccessApplication_Policies_ConnectionRulesRef {
        DataZeroTrustAccessApplication_Policies_ConnectionRulesRef::new(
            self.shared().clone(),
            format!("{}.connection_rules", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `decision` after provisioning.\nThe action Access will take if a user matches this policy. Infrastructure application policies can only use the Allow action.\nAvailable values: \"allow\", \"deny\", \"non_identity\", \"bypass\"."]
    pub fn decision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.decision", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nRules evaluated with a NOT logical operator. To match the policy, a user cannot meet any of the Exclude rules."]
    pub fn exclude(&self) -> terrars::SetRef<DataZeroTrustAccessApplication_Policies_ExcludeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe UUID of the policy"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nRules evaluated with an OR logical operator. A user needs to meet only one of the Include rules."]
    pub fn include(&self) -> terrars::SetRef<DataZeroTrustAccessApplication_Policies_IncludeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.include", self.base))
    }

    #[doc =
        "Get a reference to the value of field `isolation_required` after provisioning.\nRequire this application to be served in an isolated browser for users matching this policy. 'Client Web Isolation' must be on for the account in order to use this feature."]
    pub fn isolation_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.isolation_required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mfa_config` after provisioning.\nConfigures multi-factor authentication (MFA) settings for this policy."]
    pub fn mfa_config(&self) -> DataZeroTrustAccessApplication_Policies_MfaConfigRef {
        DataZeroTrustAccessApplication_Policies_MfaConfigRef::new(
            self.shared().clone(),
            format!("{}.mfa_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Access policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `precedence` after provisioning.\nThe order of execution for this policy. Must be unique for each policy within an app."]
    pub fn precedence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precedence", self.base))
    }

    #[doc =
        "Get a reference to the value of field `purpose_justification_prompt` after provisioning.\nA custom message that will appear on the purpose justification screen."]
    pub fn purpose_justification_prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.purpose_justification_prompt", self.base))
    }

    #[doc =
        "Get a reference to the value of field `purpose_justification_required` after provisioning.\nRequire users to enter a justification when they log in to the application."]
    pub fn purpose_justification_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.purpose_justification_required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require` after provisioning.\nRules evaluated with an AND logical operator. To match the policy, a user must meet all of the Require rules."]
    pub fn require(&self) -> terrars::SetRef<DataZeroTrustAccessApplication_Policies_RequireRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.require", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nThe amount of time that tokens issued for the application will be valid. Must be in the format `300ms` or `2h45m`. Valid time units are: ns, us (or µs), ms, s, m, h."]
    pub fn session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source_NameByIdp {}

impl DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source_NameByIdp { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source_NameByIdp {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source_NameByIdp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source_NameByIdp {}

impl BuildDataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source_NameByIdp {
    pub fn build(self) -> DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source_NameByIdp {
        DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source_NameByIdp {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source_NameByIdpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source_NameByIdpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source_NameByIdpRef {
        DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source_NameByIdpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source_NameByIdpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idp_id` after provisioning.\nThe UID of the IdP."]
    pub fn idp_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idp_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_name` after provisioning.\nThe name of the IdP provided attribute."]
    pub fn source_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source {}

impl DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source {}

impl BuildDataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source {
    pub fn build(self) -> DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source {
        DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_SaasApp_CustomAttributes_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_SaasApp_CustomAttributes_SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_SaasApp_CustomAttributes_SourceRef {
        DataZeroTrustAccessApplication_SaasApp_CustomAttributes_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_SaasApp_CustomAttributes_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the IdP attribute."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name_by_idp` after provisioning.\nA mapping from IdP ID to attribute name."]
    pub fn name_by_idp(
        &self,
    ) -> terrars::ListRef<DataZeroTrustAccessApplication_SaasApp_CustomAttributes_Source_NameByIdpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name_by_idp", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_SaasApp_CustomAttributes {}

impl DataZeroTrustAccessApplication_SaasApp_CustomAttributes { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_SaasApp_CustomAttributes {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_SaasApp_CustomAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_SaasApp_CustomAttributes {}

impl BuildDataZeroTrustAccessApplication_SaasApp_CustomAttributes {
    pub fn build(self) -> DataZeroTrustAccessApplication_SaasApp_CustomAttributes {
        DataZeroTrustAccessApplication_SaasApp_CustomAttributes {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_SaasApp_CustomAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_SaasApp_CustomAttributesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_SaasApp_CustomAttributesRef {
        DataZeroTrustAccessApplication_SaasApp_CustomAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_SaasApp_CustomAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `friendly_name` after provisioning.\nThe SAML FriendlyName of the attribute."]
    pub fn friendly_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.friendly_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the attribute."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name_format` after provisioning.\nA globally unique name for an identity or service provider.\nAvailable values: \"urn:oasis:names:tc:SAML:2.0:attrname-format:unspecified\", \"urn:oasis:names:tc:SAML:2.0:attrname-format:basic\", \"urn:oasis:names:tc:SAML:2.0:attrname-format:uri\"."]
    pub fn name_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `required` after provisioning.\nIf the attribute is required when building a SAML assertion."]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> DataZeroTrustAccessApplication_SaasApp_CustomAttributes_SourceRef {
        DataZeroTrustAccessApplication_SaasApp_CustomAttributes_SourceRef::new(
            self.shared().clone(),
            format!("{}.source", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_SaasApp_CustomClaims_Source {}

impl DataZeroTrustAccessApplication_SaasApp_CustomClaims_Source { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_SaasApp_CustomClaims_Source {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_SaasApp_CustomClaims_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_SaasApp_CustomClaims_Source {}

impl BuildDataZeroTrustAccessApplication_SaasApp_CustomClaims_Source {
    pub fn build(self) -> DataZeroTrustAccessApplication_SaasApp_CustomClaims_Source {
        DataZeroTrustAccessApplication_SaasApp_CustomClaims_Source {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_SaasApp_CustomClaims_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_SaasApp_CustomClaims_SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_SaasApp_CustomClaims_SourceRef {
        DataZeroTrustAccessApplication_SaasApp_CustomClaims_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_SaasApp_CustomClaims_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the IdP claim."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name_by_idp` after provisioning.\nA mapping from IdP ID to claim name."]
    pub fn name_by_idp(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.name_by_idp", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_SaasApp_CustomClaims {}

impl DataZeroTrustAccessApplication_SaasApp_CustomClaims { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_SaasApp_CustomClaims {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_SaasApp_CustomClaims>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_SaasApp_CustomClaims {}

impl BuildDataZeroTrustAccessApplication_SaasApp_CustomClaims {
    pub fn build(self) -> DataZeroTrustAccessApplication_SaasApp_CustomClaims {
        DataZeroTrustAccessApplication_SaasApp_CustomClaims {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_SaasApp_CustomClaimsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_SaasApp_CustomClaimsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_SaasApp_CustomClaimsRef {
        DataZeroTrustAccessApplication_SaasApp_CustomClaimsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_SaasApp_CustomClaimsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the claim."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `required` after provisioning.\nIf the claim is required when building an OIDC token."]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe scope of the claim.\nAvailable values: \"groups\", \"profile\", \"email\", \"openid\"."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> DataZeroTrustAccessApplication_SaasApp_CustomClaims_SourceRef {
        DataZeroTrustAccessApplication_SaasApp_CustomClaims_SourceRef::new(
            self.shared().clone(),
            format!("{}.source", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_SaasApp_HybridAndImplicitOptions {}

impl DataZeroTrustAccessApplication_SaasApp_HybridAndImplicitOptions { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_SaasApp_HybridAndImplicitOptions {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_SaasApp_HybridAndImplicitOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_SaasApp_HybridAndImplicitOptions {}

impl BuildDataZeroTrustAccessApplication_SaasApp_HybridAndImplicitOptions {
    pub fn build(self) -> DataZeroTrustAccessApplication_SaasApp_HybridAndImplicitOptions {
        DataZeroTrustAccessApplication_SaasApp_HybridAndImplicitOptions {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_SaasApp_HybridAndImplicitOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_SaasApp_HybridAndImplicitOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_SaasApp_HybridAndImplicitOptionsRef {
        DataZeroTrustAccessApplication_SaasApp_HybridAndImplicitOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_SaasApp_HybridAndImplicitOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `return_access_token_from_authorization_endpoint` after provisioning.\nIf an Access Token should be returned from the OIDC Authorization endpoint"]
    pub fn return_access_token_from_authorization_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.return_access_token_from_authorization_endpoint", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `return_id_token_from_authorization_endpoint` after provisioning.\nIf an ID Token should be returned from the OIDC Authorization endpoint"]
    pub fn return_id_token_from_authorization_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.return_id_token_from_authorization_endpoint", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_SaasApp_RefreshTokenOptions {}

impl DataZeroTrustAccessApplication_SaasApp_RefreshTokenOptions { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_SaasApp_RefreshTokenOptions {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_SaasApp_RefreshTokenOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_SaasApp_RefreshTokenOptions {}

impl BuildDataZeroTrustAccessApplication_SaasApp_RefreshTokenOptions {
    pub fn build(self) -> DataZeroTrustAccessApplication_SaasApp_RefreshTokenOptions {
        DataZeroTrustAccessApplication_SaasApp_RefreshTokenOptions {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_SaasApp_RefreshTokenOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_SaasApp_RefreshTokenOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_SaasApp_RefreshTokenOptionsRef {
        DataZeroTrustAccessApplication_SaasApp_RefreshTokenOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_SaasApp_RefreshTokenOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `lifetime` after provisioning.\nHow long a refresh token will be valid for after creation. Valid units are m,h,d. Must be longer than 1m."]
    pub fn lifetime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifetime", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_SaasApp {}

impl DataZeroTrustAccessApplication_SaasApp { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_SaasApp {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_SaasApp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_SaasApp {}

impl BuildDataZeroTrustAccessApplication_SaasApp {
    pub fn build(self) -> DataZeroTrustAccessApplication_SaasApp {
        DataZeroTrustAccessApplication_SaasApp {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_SaasAppRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_SaasAppRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessApplication_SaasAppRef {
        DataZeroTrustAccessApplication_SaasAppRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_SaasAppRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_token_lifetime` after provisioning.\nThe lifetime of the OIDC Access Token after creation. Valid units are m,h. Must be greater than or equal to 1m and less than or equal to 24h."]
    pub fn access_token_lifetime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token_lifetime", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_pkce_without_client_secret` after provisioning.\nIf client secret should be required on the token endpoint when authorization_code_with_pkce grant is used."]
    pub fn allow_pkce_without_client_secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_pkce_without_client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `app_launcher_url` after provisioning.\nThe URL where this applications tile redirects users"]
    pub fn app_launcher_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_launcher_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auth_type` after provisioning.\nOptional identifier indicating the authentication protocol used for the saas app. Required for OIDC. Default if unset is \"saml\"\nAvailable values: \"saml\", \"oidc\"."]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe application client id"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nThe application client secret, only returned on POST request."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `consumer_service_url` after provisioning.\nThe service provider's endpoint that is responsible for receiving and parsing a SAML assertion."]
    pub fn consumer_service_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_service_url", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_attributes` after provisioning.\n"]
    pub fn custom_attributes(&self) -> terrars::ListRef<DataZeroTrustAccessApplication_SaasApp_CustomAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_claims` after provisioning.\n"]
    pub fn custom_claims(&self) -> terrars::ListRef<DataZeroTrustAccessApplication_SaasApp_CustomClaimsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_claims", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_relay_state` after provisioning.\nThe URL that the user will be redirected to after a successful login for IDP initiated logins."]
    pub fn default_relay_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_relay_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `grant_types` after provisioning.\nThe OIDC flows supported by this application"]
    pub fn grant_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grant_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `group_filter_regex` after provisioning.\nA regex to filter Cloudflare groups returned in ID token and userinfo endpoint"]
    pub fn group_filter_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_filter_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `hybrid_and_implicit_options` after provisioning.\n"]
    pub fn hybrid_and_implicit_options(&self) -> DataZeroTrustAccessApplication_SaasApp_HybridAndImplicitOptionsRef {
        DataZeroTrustAccessApplication_SaasApp_HybridAndImplicitOptionsRef::new(
            self.shared().clone(),
            format!("{}.hybrid_and_implicit_options", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `idp_entity_id` after provisioning.\nThe unique identifier for your SaaS application."]
    pub fn idp_entity_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idp_entity_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name_id_format` after provisioning.\nThe format of the name identifier sent to the SaaS application.\nAvailable values: \"id\", \"email\"."]
    pub fn name_id_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_id_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name_id_transform_jsonata` after provisioning.\nA [JSONata](https://jsonata.org/) expression that transforms an application's user identities into a NameID value for its SAML assertion. This expression should evaluate to a singular string. The output of this expression can override the `name_id_format` setting."]
    pub fn name_id_transform_jsonata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_id_transform_jsonata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `public_key` after provisioning.\nThe Access public certificate that will be used to verify your identity."]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redirect_uris` after provisioning.\nThe permitted URL's for Cloudflare to return Authorization codes and Access/ID tokens"]
    pub fn redirect_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect_uris", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_token_options` after provisioning.\n"]
    pub fn refresh_token_options(&self) -> DataZeroTrustAccessApplication_SaasApp_RefreshTokenOptionsRef {
        DataZeroTrustAccessApplication_SaasApp_RefreshTokenOptionsRef::new(
            self.shared().clone(),
            format!("{}.refresh_token_options", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `saml_attribute_transform_jsonata` after provisioning.\nA [JSONata] (https://jsonata.org/) expression that transforms an application's user identities into attribute assertions in the SAML response. The expression can transform id, email, name, and groups values. It can also transform fields listed in the saml_attributes or oidc_fields of the identity provider used to authenticate. The output of this expression must be a JSON object."]
    pub fn saml_attribute_transform_jsonata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.saml_attribute_transform_jsonata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nDefine the user information shared with access, \"offline_access\" scope will be automatically enabled if refresh tokens are enabled"]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sp_entity_id` after provisioning.\nA globally unique name for an identity or service provider."]
    pub fn sp_entity_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sp_entity_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sso_endpoint` after provisioning.\nThe endpoint where your SaaS application will send login requests."]
    pub fn sso_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sso_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_ScimConfig_Authentication {}

impl DataZeroTrustAccessApplication_ScimConfig_Authentication { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_ScimConfig_Authentication {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_ScimConfig_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_ScimConfig_Authentication {}

impl BuildDataZeroTrustAccessApplication_ScimConfig_Authentication {
    pub fn build(self) -> DataZeroTrustAccessApplication_ScimConfig_Authentication {
        DataZeroTrustAccessApplication_ScimConfig_Authentication {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_ScimConfig_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_ScimConfig_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_ScimConfig_AuthenticationRef {
        DataZeroTrustAccessApplication_ScimConfig_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_ScimConfig_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authorization_url` after provisioning.\nURL used to generate the auth code used during token generation."]
    pub fn authorization_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nClient ID used to authenticate when generating a token for authenticating with the remote SCIM service."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nSecret used to authenticate when generating a token for authenticating with the remove SCIM service."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nPassword used to authenticate with the remote SCIM service."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scheme` after provisioning.\nThe authentication scheme to use when making SCIM requests to this application.\nAvailable values: \"httpbasic\", \"oauthbearertoken\", \"oauth2\", \"access_service_token\"."]
    pub fn scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nThe authorization scopes to request when generating the token used to authenticate with the remove SCIM service."]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token` after provisioning.\nToken used to authenticate with the remote SCIM service."]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_url` after provisioning.\nURL used to generate the token used to authenticate with the remote SCIM service."]
    pub fn token_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user` after provisioning.\nUser name used to authenticate with the remote SCIM service."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_ScimConfig_Mappings_Operations {}

impl DataZeroTrustAccessApplication_ScimConfig_Mappings_Operations { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_ScimConfig_Mappings_Operations {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_ScimConfig_Mappings_Operations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_ScimConfig_Mappings_Operations {}

impl BuildDataZeroTrustAccessApplication_ScimConfig_Mappings_Operations {
    pub fn build(self) -> DataZeroTrustAccessApplication_ScimConfig_Mappings_Operations {
        DataZeroTrustAccessApplication_ScimConfig_Mappings_Operations {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_ScimConfig_Mappings_OperationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_ScimConfig_Mappings_OperationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_ScimConfig_Mappings_OperationsRef {
        DataZeroTrustAccessApplication_ScimConfig_Mappings_OperationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_ScimConfig_Mappings_OperationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nWhether or not this mapping applies to create (POST) operations."]
    pub fn create(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nWhether or not this mapping applies to DELETE operations."]
    pub fn delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nWhether or not this mapping applies to update (PATCH/PUT) operations."]
    pub fn update(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_ScimConfig_Mappings {}

impl DataZeroTrustAccessApplication_ScimConfig_Mappings { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_ScimConfig_Mappings {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_ScimConfig_Mappings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_ScimConfig_Mappings {}

impl BuildDataZeroTrustAccessApplication_ScimConfig_Mappings {
    pub fn build(self) -> DataZeroTrustAccessApplication_ScimConfig_Mappings {
        DataZeroTrustAccessApplication_ScimConfig_Mappings {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_ScimConfig_MappingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_ScimConfig_MappingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_ScimConfig_MappingsRef {
        DataZeroTrustAccessApplication_ScimConfig_MappingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_ScimConfig_MappingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether or not this mapping is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nA [SCIM filter expression](https://datatracker.ietf.org/doc/html/rfc7644#section-3.4.2.2) that matches resources that should be provisioned to this application."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operations` after provisioning.\nWhether or not this mapping applies to creates, updates, or deletes."]
    pub fn operations(&self) -> DataZeroTrustAccessApplication_ScimConfig_Mappings_OperationsRef {
        DataZeroTrustAccessApplication_ScimConfig_Mappings_OperationsRef::new(
            self.shared().clone(),
            format!("{}.operations", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `schema` after provisioning.\nWhich SCIM resource type this mapping applies to."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strictness` after provisioning.\nThe level of adherence to outbound resource schemas when provisioning to this mapping. ‘Strict’ removes unknown values, while ‘passthrough’ passes unknown values to the target.\nAvailable values: \"strict\", \"passthrough\"."]
    pub fn strictness(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strictness", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transform_jsonata` after provisioning.\nA [JSONata](https://jsonata.org/) expression that transforms the resource before provisioning it in the application."]
    pub fn transform_jsonata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transform_jsonata", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_ScimConfig {}

impl DataZeroTrustAccessApplication_ScimConfig { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_ScimConfig {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_ScimConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_ScimConfig {}

impl BuildDataZeroTrustAccessApplication_ScimConfig {
    pub fn build(self) -> DataZeroTrustAccessApplication_ScimConfig {
        DataZeroTrustAccessApplication_ScimConfig {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_ScimConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_ScimConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessApplication_ScimConfigRef {
        DataZeroTrustAccessApplication_ScimConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_ScimConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authentication` after provisioning.\nAttributes for configuring HTTP Basic authentication scheme for SCIM provisioning to an application."]
    pub fn authentication(&self) -> DataZeroTrustAccessApplication_ScimConfig_AuthenticationRef {
        DataZeroTrustAccessApplication_ScimConfig_AuthenticationRef::new(
            self.shared().clone(),
            format!("{}.authentication", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `deactivate_on_delete` after provisioning.\nIf false, propagates DELETE requests to the target application for SCIM resources. If true, sets 'active' to false on the SCIM resource. Note: Some targets do not support DELETE operations."]
    pub fn deactivate_on_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deactivate_on_delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether SCIM provisioning is turned on for this application."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `idp_uid` after provisioning.\nThe UID of the IdP to use as the source for SCIM resources to provision to this application."]
    pub fn idp_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idp_uid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mappings` after provisioning.\nA list of mappings to apply to SCIM resources before provisioning them in this application. These can transform or filter the resources to be provisioned."]
    pub fn mappings(&self) -> terrars::ListRef<DataZeroTrustAccessApplication_ScimConfig_MappingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mappings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `remote_uri` after provisioning.\nThe base URI for the application's SCIM-compatible API."]
    pub fn remote_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_TargetCriteria_Exclude_TargetAttributes {}

impl DataZeroTrustAccessApplication_TargetCriteria_Exclude_TargetAttributes { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_TargetCriteria_Exclude_TargetAttributes {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_TargetCriteria_Exclude_TargetAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_TargetCriteria_Exclude_TargetAttributes {}

impl BuildDataZeroTrustAccessApplication_TargetCriteria_Exclude_TargetAttributes {
    pub fn build(self) -> DataZeroTrustAccessApplication_TargetCriteria_Exclude_TargetAttributes {
        DataZeroTrustAccessApplication_TargetCriteria_Exclude_TargetAttributes {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_TargetCriteria_Exclude_TargetAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_TargetCriteria_Exclude_TargetAttributesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_TargetCriteria_Exclude_TargetAttributesRef {
        DataZeroTrustAccessApplication_TargetCriteria_Exclude_TargetAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_TargetCriteria_Exclude_TargetAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_TargetCriteria_Exclude {}

impl DataZeroTrustAccessApplication_TargetCriteria_Exclude { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_TargetCriteria_Exclude {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_TargetCriteria_Exclude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_TargetCriteria_Exclude {}

impl BuildDataZeroTrustAccessApplication_TargetCriteria_Exclude {
    pub fn build(self) -> DataZeroTrustAccessApplication_TargetCriteria_Exclude {
        DataZeroTrustAccessApplication_TargetCriteria_Exclude {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_TargetCriteria_ExcludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_TargetCriteria_ExcludeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_TargetCriteria_ExcludeRef {
        DataZeroTrustAccessApplication_TargetCriteria_ExcludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_TargetCriteria_ExcludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nMap of target tag keys to values. Values within a key are OR'd."]
    pub fn tags(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_attributes` after provisioning.\nHostname selector map. Only the hostname key is supported."]
    pub fn target_attributes(&self) -> DataZeroTrustAccessApplication_TargetCriteria_Exclude_TargetAttributesRef {
        DataZeroTrustAccessApplication_TargetCriteria_Exclude_TargetAttributesRef::new(
            self.shared().clone(),
            format!("{}.target_attributes", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_TargetCriteria_Include_TargetAttributes {}

impl DataZeroTrustAccessApplication_TargetCriteria_Include_TargetAttributes { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_TargetCriteria_Include_TargetAttributes {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_TargetCriteria_Include_TargetAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_TargetCriteria_Include_TargetAttributes {}

impl BuildDataZeroTrustAccessApplication_TargetCriteria_Include_TargetAttributes {
    pub fn build(self) -> DataZeroTrustAccessApplication_TargetCriteria_Include_TargetAttributes {
        DataZeroTrustAccessApplication_TargetCriteria_Include_TargetAttributes {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_TargetCriteria_Include_TargetAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_TargetCriteria_Include_TargetAttributesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_TargetCriteria_Include_TargetAttributesRef {
        DataZeroTrustAccessApplication_TargetCriteria_Include_TargetAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_TargetCriteria_Include_TargetAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_TargetCriteria_Include {}

impl DataZeroTrustAccessApplication_TargetCriteria_Include { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_TargetCriteria_Include {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_TargetCriteria_Include>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_TargetCriteria_Include {}

impl BuildDataZeroTrustAccessApplication_TargetCriteria_Include {
    pub fn build(self) -> DataZeroTrustAccessApplication_TargetCriteria_Include {
        DataZeroTrustAccessApplication_TargetCriteria_Include {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_TargetCriteria_IncludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_TargetCriteria_IncludeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_TargetCriteria_IncludeRef {
        DataZeroTrustAccessApplication_TargetCriteria_IncludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_TargetCriteria_IncludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nMap of target tag keys to values. Values within a key are OR'd."]
    pub fn tags(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_attributes` after provisioning.\nHostname selector map. Only the hostname key is supported."]
    pub fn target_attributes(&self) -> DataZeroTrustAccessApplication_TargetCriteria_Include_TargetAttributesRef {
        DataZeroTrustAccessApplication_TargetCriteria_Include_TargetAttributesRef::new(
            self.shared().clone(),
            format!("{}.target_attributes", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_TargetCriteria_Require_TargetAttributes {}

impl DataZeroTrustAccessApplication_TargetCriteria_Require_TargetAttributes { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_TargetCriteria_Require_TargetAttributes {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_TargetCriteria_Require_TargetAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_TargetCriteria_Require_TargetAttributes {}

impl BuildDataZeroTrustAccessApplication_TargetCriteria_Require_TargetAttributes {
    pub fn build(self) -> DataZeroTrustAccessApplication_TargetCriteria_Require_TargetAttributes {
        DataZeroTrustAccessApplication_TargetCriteria_Require_TargetAttributes {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_TargetCriteria_Require_TargetAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_TargetCriteria_Require_TargetAttributesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_TargetCriteria_Require_TargetAttributesRef {
        DataZeroTrustAccessApplication_TargetCriteria_Require_TargetAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_TargetCriteria_Require_TargetAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_TargetCriteria_Require {}

impl DataZeroTrustAccessApplication_TargetCriteria_Require { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_TargetCriteria_Require {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_TargetCriteria_Require>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_TargetCriteria_Require {}

impl BuildDataZeroTrustAccessApplication_TargetCriteria_Require {
    pub fn build(self) -> DataZeroTrustAccessApplication_TargetCriteria_Require {
        DataZeroTrustAccessApplication_TargetCriteria_Require {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_TargetCriteria_RequireRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_TargetCriteria_RequireRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_TargetCriteria_RequireRef {
        DataZeroTrustAccessApplication_TargetCriteria_RequireRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_TargetCriteria_RequireRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nMap of target tag keys to values. Values within a key are OR'd."]
    pub fn tags(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_attributes` after provisioning.\nHostname selector map. Only the hostname key is supported."]
    pub fn target_attributes(&self) -> DataZeroTrustAccessApplication_TargetCriteria_Require_TargetAttributesRef {
        DataZeroTrustAccessApplication_TargetCriteria_Require_TargetAttributesRef::new(
            self.shared().clone(),
            format!("{}.target_attributes", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessApplication_TargetCriteria {}

impl DataZeroTrustAccessApplication_TargetCriteria { }

impl terrars::ToListMappable for DataZeroTrustAccessApplication_TargetCriteria {
    type O = terrars::BlockAssignable<DataZeroTrustAccessApplication_TargetCriteria>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessApplication_TargetCriteria {}

impl BuildDataZeroTrustAccessApplication_TargetCriteria {
    pub fn build(self) -> DataZeroTrustAccessApplication_TargetCriteria {
        DataZeroTrustAccessApplication_TargetCriteria {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessApplication_TargetCriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessApplication_TargetCriteriaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessApplication_TargetCriteriaRef {
        DataZeroTrustAccessApplication_TargetCriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessApplication_TargetCriteriaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nTarget is excluded when any selector in this rule matches."]
    pub fn exclude(&self) -> DataZeroTrustAccessApplication_TargetCriteria_ExcludeRef {
        DataZeroTrustAccessApplication_TargetCriteria_ExcludeRef::new(
            self.shared().clone(),
            format!("{}.exclude", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nTarget matches when any selector in this rule matches."]
    pub fn include(&self) -> DataZeroTrustAccessApplication_TargetCriteria_IncludeRef {
        DataZeroTrustAccessApplication_TargetCriteria_IncludeRef::new(
            self.shared().clone(),
            format!("{}.include", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port that the targets use for the chosen communication protocol. A port cannot be assigned to multiple protocols."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nThe communication protocol your application secures.\nAvailable values: \"SSH\", \"RDP\"."]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require` after provisioning.\nTarget matches only when every selector in this rule matches."]
    pub fn require(&self) -> DataZeroTrustAccessApplication_TargetCriteria_RequireRef {
        DataZeroTrustAccessApplication_TargetCriteria_RequireRef::new(
            self.shared().clone(),
            format!("{}.require", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `target_attributes` after provisioning.\nDeprecated: use include.target_attributes for hostname selectors and rule tags for tag-based matching. New or changed maps may contain only hostname. The API accepts unchanged legacy maps on update, including non-hostname keys."]
    pub fn target_attributes(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.target_attributes", self.base))
    }
}
