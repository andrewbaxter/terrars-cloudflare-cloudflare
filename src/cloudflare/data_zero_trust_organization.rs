#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustOrganizationData_ {
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
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustOrganization_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustOrganizationData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustOrganization(std::rc::Rc<DataZeroTrustOrganization_>);

impl DataZeroTrustOrganization {
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
        "Get a reference to the value of field `allow_authenticate_via_warp` after provisioning.\nWhen set to true, users can authenticate via WARP for any application in your organization. Application settings will take precedence over this value."]
    pub fn allow_authenticate_via_warp(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_authenticate_via_warp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auth_domain` after provisioning.\nThe unique subdomain assigned to your Zero Trust organization. If omitted on creation, a unique subdomain is auto-generated in the format `adjective-noun-hex4` (e.g. `frosty-moon-7a3b.cloudflareaccess.com`)."]
    pub fn auth_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_redirect_to_identity` after provisioning.\nWhen set to `true`, users skip the identity provider selection step during login."]
    pub fn auto_redirect_to_identity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_redirect_to_identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_pages` after provisioning.\n"]
    pub fn custom_pages(&self) -> DataZeroTrustOrganization_CustomPagesRef {
        DataZeroTrustOrganization_CustomPagesRef::new(
            self.shared().clone(),
            format!("{}.custom_pages", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `deny_unmatched_requests` after provisioning.\nDetermines whether to deny all requests to Cloudflare-protected resources that lack an associated Access application. If enabled, you must explicitly configure an Access application and policy to allow traffic to your Cloudflare-protected resources. For domains you want to be public across all subdomains, add the domain to the `deny_unmatched_requests_exempted_zone_names` array."]
    pub fn deny_unmatched_requests(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deny_unmatched_requests", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deny_unmatched_requests_exempted_zone_names` after provisioning.\nContains zone names to exempt from the `deny_unmatched_requests` feature. Requests to a subdomain in an exempted zone will block unauthenticated traffic by default if there is a configured Access application and policy that matches the request."]
    pub fn deny_unmatched_requests_exempted_zone_names(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.deny_unmatched_requests_exempted_zone_names", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `is_ui_read_only` after provisioning.\nLock all settings as Read-Only in the Dashboard, regardless of user permission. Updates may only be made via the API or Terraform for this account when enabled."]
    pub fn is_ui_read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_ui_read_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `login_design` after provisioning.\n"]
    pub fn login_design(&self) -> DataZeroTrustOrganization_LoginDesignRef {
        DataZeroTrustOrganization_LoginDesignRef::new(
            self.shared().clone(),
            format!("{}.login_design", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `mfa_config` after provisioning.\nConfigures multi-factor authentication (MFA) settings for an organization."]
    pub fn mfa_config(&self) -> DataZeroTrustOrganization_MfaConfigRef {
        DataZeroTrustOrganization_MfaConfigRef::new(
            self.shared().clone(),
            format!("{}.mfa_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `mfa_piv_key_requirements` after provisioning.\nConfigures PIV key requirements for MFA using hardware security keys."]
    pub fn mfa_piv_key_requirements(&self) -> DataZeroTrustOrganization_MfaPivKeyRequirementsRef {
        DataZeroTrustOrganization_MfaPivKeyRequirementsRef::new(
            self.shared().clone(),
            format!("{}.mfa_piv_key_requirements", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `mfa_required_for_all_apps` after provisioning.\nDetermines whether global MFA settings apply to applications by default. The organization must have MFA enabled with at least one authentication method and a session duration configured. Note: 'allowed_authenticators' cannot contain only the infrastructure SSH authenticators ('piv_key' and 'ssh_fido2_key') if the organization has any non-infrastructure applications."]
    pub fn mfa_required_for_all_apps(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mfa_required_for_all_apps", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of your Zero Trust organization. When omitted on creation, defaults to the provided auth_domain; when both are omitted, defaults to the auto-generated subdomain slug (e.g. frosty-moon-7a3b)."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_token_inactivity` after provisioning.\nConfigures automatic enforcement for inactive service tokens. A service token is inactive if no policy references it, and it has not successfully authenticated with an Access application during the selected inactivity period. This setting applies to every service token in your Zero Trust account."]
    pub fn service_token_inactivity(&self) -> DataZeroTrustOrganization_ServiceTokenInactivityRef {
        DataZeroTrustOrganization_ServiceTokenInactivityRef::new(
            self.shared().clone(),
            format!("{}.service_token_inactivity", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nThe amount of time that tokens issued for applications will be valid. Must be in the format `300ms` or `2h45m`. Valid time units are: ns, us (or µs), ms, s, m, h."]
    pub fn session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `strict_service_token_auth` after provisioning.\nEnables new behaviors for requests made with Access service tokens. Unauthorized requests emit audit logs, and return a 401 or 403 status code in the response instead of redirecting to the login page. Successful requests no longer receive a CF_Authorization cookie in the response. Zero Trust organizations created on or after October 5, 2026 will have this setting enabled by default, and cannot disable it."]
    pub fn strict_service_token_auth(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strict_service_token_auth", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trusted_accounts` after provisioning.\nThe account tags of organizations trusted by this organization for policy and device posture sharing."]
    pub fn trusted_accounts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_accounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ui_read_only_toggle_reason` after provisioning.\nA description of the reason why the UI read only field is being toggled."]
    pub fn ui_read_only_toggle_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ui_read_only_toggle_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_seat_expiration_inactive_time` after provisioning.\nThe amount of time a user seat is inactive before it expires. When the user seat exceeds the set time of inactivity, the user is removed as an active seat and no longer counts against your Teams seat count.  Minimum value for this setting is 1 month (730h). Must be in the format `300ms` or `2h45m`. Valid time units are: `ns`, `us` (or `µs`), `ms`, `s`, `m`, `h`."]
    pub fn user_seat_expiration_inactive_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_seat_expiration_inactive_time", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `warp_auth_non_browser_401` after provisioning.\nWhen enabled, unsuccessful WARP authentication requests with a non-HTML Accept header return a 401 response instead of redirecting to the login page."]
    pub fn warp_auth_non_browser_401(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warp_auth_non_browser_401", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `warp_auth_session_duration` after provisioning.\nThe amount of time that tokens issued for applications will be valid. Must be in the format `30m` or `2h45m`. Valid time units are: m, h."]
    pub fn warp_auth_session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warp_auth_session_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustOrganization {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustOrganization { }

impl terrars::ToListMappable for DataZeroTrustOrganization {
    type O = terrars::ListRef<DataZeroTrustOrganizationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustOrganization_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_organization".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustOrganization {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustOrganization {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustOrganization {
        let out = DataZeroTrustOrganization(std::rc::Rc::new(DataZeroTrustOrganization_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustOrganizationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustOrganizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustOrganizationRef {
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
        "Get a reference to the value of field `allow_authenticate_via_warp` after provisioning.\nWhen set to true, users can authenticate via WARP for any application in your organization. Application settings will take precedence over this value."]
    pub fn allow_authenticate_via_warp(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_authenticate_via_warp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auth_domain` after provisioning.\nThe unique subdomain assigned to your Zero Trust organization. If omitted on creation, a unique subdomain is auto-generated in the format `adjective-noun-hex4` (e.g. `frosty-moon-7a3b.cloudflareaccess.com`)."]
    pub fn auth_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_redirect_to_identity` after provisioning.\nWhen set to `true`, users skip the identity provider selection step during login."]
    pub fn auto_redirect_to_identity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_redirect_to_identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_pages` after provisioning.\n"]
    pub fn custom_pages(&self) -> DataZeroTrustOrganization_CustomPagesRef {
        DataZeroTrustOrganization_CustomPagesRef::new(
            self.shared().clone(),
            format!("{}.custom_pages", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `deny_unmatched_requests` after provisioning.\nDetermines whether to deny all requests to Cloudflare-protected resources that lack an associated Access application. If enabled, you must explicitly configure an Access application and policy to allow traffic to your Cloudflare-protected resources. For domains you want to be public across all subdomains, add the domain to the `deny_unmatched_requests_exempted_zone_names` array."]
    pub fn deny_unmatched_requests(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deny_unmatched_requests", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deny_unmatched_requests_exempted_zone_names` after provisioning.\nContains zone names to exempt from the `deny_unmatched_requests` feature. Requests to a subdomain in an exempted zone will block unauthenticated traffic by default if there is a configured Access application and policy that matches the request."]
    pub fn deny_unmatched_requests_exempted_zone_names(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.deny_unmatched_requests_exempted_zone_names", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `is_ui_read_only` after provisioning.\nLock all settings as Read-Only in the Dashboard, regardless of user permission. Updates may only be made via the API or Terraform for this account when enabled."]
    pub fn is_ui_read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_ui_read_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `login_design` after provisioning.\n"]
    pub fn login_design(&self) -> DataZeroTrustOrganization_LoginDesignRef {
        DataZeroTrustOrganization_LoginDesignRef::new(
            self.shared().clone(),
            format!("{}.login_design", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `mfa_config` after provisioning.\nConfigures multi-factor authentication (MFA) settings for an organization."]
    pub fn mfa_config(&self) -> DataZeroTrustOrganization_MfaConfigRef {
        DataZeroTrustOrganization_MfaConfigRef::new(
            self.shared().clone(),
            format!("{}.mfa_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `mfa_piv_key_requirements` after provisioning.\nConfigures PIV key requirements for MFA using hardware security keys."]
    pub fn mfa_piv_key_requirements(&self) -> DataZeroTrustOrganization_MfaPivKeyRequirementsRef {
        DataZeroTrustOrganization_MfaPivKeyRequirementsRef::new(
            self.shared().clone(),
            format!("{}.mfa_piv_key_requirements", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `mfa_required_for_all_apps` after provisioning.\nDetermines whether global MFA settings apply to applications by default. The organization must have MFA enabled with at least one authentication method and a session duration configured. Note: 'allowed_authenticators' cannot contain only the infrastructure SSH authenticators ('piv_key' and 'ssh_fido2_key') if the organization has any non-infrastructure applications."]
    pub fn mfa_required_for_all_apps(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mfa_required_for_all_apps", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of your Zero Trust organization. When omitted on creation, defaults to the provided auth_domain; when both are omitted, defaults to the auto-generated subdomain slug (e.g. frosty-moon-7a3b)."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_token_inactivity` after provisioning.\nConfigures automatic enforcement for inactive service tokens. A service token is inactive if no policy references it, and it has not successfully authenticated with an Access application during the selected inactivity period. This setting applies to every service token in your Zero Trust account."]
    pub fn service_token_inactivity(&self) -> DataZeroTrustOrganization_ServiceTokenInactivityRef {
        DataZeroTrustOrganization_ServiceTokenInactivityRef::new(
            self.shared().clone(),
            format!("{}.service_token_inactivity", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nThe amount of time that tokens issued for applications will be valid. Must be in the format `300ms` or `2h45m`. Valid time units are: ns, us (or µs), ms, s, m, h."]
    pub fn session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `strict_service_token_auth` after provisioning.\nEnables new behaviors for requests made with Access service tokens. Unauthorized requests emit audit logs, and return a 401 or 403 status code in the response instead of redirecting to the login page. Successful requests no longer receive a CF_Authorization cookie in the response. Zero Trust organizations created on or after October 5, 2026 will have this setting enabled by default, and cannot disable it."]
    pub fn strict_service_token_auth(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strict_service_token_auth", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trusted_accounts` after provisioning.\nThe account tags of organizations trusted by this organization for policy and device posture sharing."]
    pub fn trusted_accounts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_accounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ui_read_only_toggle_reason` after provisioning.\nA description of the reason why the UI read only field is being toggled."]
    pub fn ui_read_only_toggle_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ui_read_only_toggle_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_seat_expiration_inactive_time` after provisioning.\nThe amount of time a user seat is inactive before it expires. When the user seat exceeds the set time of inactivity, the user is removed as an active seat and no longer counts against your Teams seat count.  Minimum value for this setting is 1 month (730h). Must be in the format `300ms` or `2h45m`. Valid time units are: `ns`, `us` (or `µs`), `ms`, `s`, `m`, `h`."]
    pub fn user_seat_expiration_inactive_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_seat_expiration_inactive_time", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `warp_auth_non_browser_401` after provisioning.\nWhen enabled, unsuccessful WARP authentication requests with a non-HTML Accept header return a 401 response instead of redirecting to the login page."]
    pub fn warp_auth_non_browser_401(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warp_auth_non_browser_401", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `warp_auth_session_duration` after provisioning.\nThe amount of time that tokens issued for applications will be valid. Must be in the format `30m` or `2h45m`. Valid time units are: m, h."]
    pub fn warp_auth_session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warp_auth_session_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustOrganization_CustomPages {}

impl DataZeroTrustOrganization_CustomPages { }

impl terrars::ToListMappable for DataZeroTrustOrganization_CustomPages {
    type O = terrars::BlockAssignable<DataZeroTrustOrganization_CustomPages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustOrganization_CustomPages {}

impl BuildDataZeroTrustOrganization_CustomPages {
    pub fn build(self) -> DataZeroTrustOrganization_CustomPages {
        DataZeroTrustOrganization_CustomPages {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustOrganization_CustomPagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustOrganization_CustomPagesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustOrganization_CustomPagesRef {
        DataZeroTrustOrganization_CustomPagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustOrganization_CustomPagesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `forbidden` after provisioning.\nThe uid of the custom page to use when a user is denied access after failing a non-identity rule."]
    pub fn forbidden(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forbidden", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_denied` after provisioning.\nThe uid of the custom page to use when a user is denied access."]
    pub fn identity_denied(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_denied", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustOrganization_LoginDesign {}

impl DataZeroTrustOrganization_LoginDesign { }

impl terrars::ToListMappable for DataZeroTrustOrganization_LoginDesign {
    type O = terrars::BlockAssignable<DataZeroTrustOrganization_LoginDesign>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustOrganization_LoginDesign {}

impl BuildDataZeroTrustOrganization_LoginDesign {
    pub fn build(self) -> DataZeroTrustOrganization_LoginDesign {
        DataZeroTrustOrganization_LoginDesign {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustOrganization_LoginDesignRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustOrganization_LoginDesignRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustOrganization_LoginDesignRef {
        DataZeroTrustOrganization_LoginDesignRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustOrganization_LoginDesignRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `background_color` after provisioning.\nThe background color on your login page."]
    pub fn background_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.background_color", self.base))
    }

    #[doc =
        "Get a reference to the value of field `footer_text` after provisioning.\nThe text at the bottom of your login page."]
    pub fn footer_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.footer_text", self.base))
    }

    #[doc =
        "Get a reference to the value of field `header_text` after provisioning.\nThe text at the top of your login page."]
    pub fn header_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_text", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logo_path` after provisioning.\nThe URL of the logo on your login page."]
    pub fn logo_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logo_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `text_color` after provisioning.\nThe text color on your login page."]
    pub fn text_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_color", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustOrganization_MfaConfig {}

impl DataZeroTrustOrganization_MfaConfig { }

impl terrars::ToListMappable for DataZeroTrustOrganization_MfaConfig {
    type O = terrars::BlockAssignable<DataZeroTrustOrganization_MfaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustOrganization_MfaConfig {}

impl BuildDataZeroTrustOrganization_MfaConfig {
    pub fn build(self) -> DataZeroTrustOrganization_MfaConfig {
        DataZeroTrustOrganization_MfaConfig {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustOrganization_MfaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustOrganization_MfaConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustOrganization_MfaConfigRef {
        DataZeroTrustOrganization_MfaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustOrganization_MfaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_authenticators` after provisioning.\nLists the MFA methods that users can authenticate with. The `piv_key` and `ssh_fido2_key` values are supported only for infrastructure applications."]
    pub fn allowed_authenticators(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_authenticators", self.base))
    }

    #[doc =
        "Get a reference to the value of field `amr_matching_session_duration` after provisioning.\nAllows a user to skip MFA via Authentication Method Reference (AMR) matching when the AMR claim provided by the IdP the user used to authenticate contains \"mfa\". Must be in minutes (m) or hours (h). Minimum: 0m. Maximum: 720h (30 days)."]
    pub fn amr_matching_session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amr_matching_session_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `required_aaguids` after provisioning.\nSpecifies a Cloudflare List of required FIDO2 authenticator device AAGUIDs."]
    pub fn required_aaguids(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required_aaguids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nDefines the duration of an MFA session. Must be in minutes (m) or hours (h). Minimum: 0m. Maximum: 720h (30 days). Examples:`5m` or `24h`."]
    pub fn session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustOrganization_MfaPivKeyRequirements {}

impl DataZeroTrustOrganization_MfaPivKeyRequirements { }

impl terrars::ToListMappable for DataZeroTrustOrganization_MfaPivKeyRequirements {
    type O = terrars::BlockAssignable<DataZeroTrustOrganization_MfaPivKeyRequirements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustOrganization_MfaPivKeyRequirements {}

impl BuildDataZeroTrustOrganization_MfaPivKeyRequirements {
    pub fn build(self) -> DataZeroTrustOrganization_MfaPivKeyRequirements {
        DataZeroTrustOrganization_MfaPivKeyRequirements {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustOrganization_MfaPivKeyRequirementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustOrganization_MfaPivKeyRequirementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustOrganization_MfaPivKeyRequirementsRef {
        DataZeroTrustOrganization_MfaPivKeyRequirementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustOrganization_MfaPivKeyRequirementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pin_policy` after provisioning.\nDefines when a PIN is required to use the SSH key. Valid values: `never` (no PIN required), `once` (PIN required once per session), `always` (PIN required for each use).\nAvailable values: \"never\", \"once\", \"always\"."]
    pub fn pin_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pin_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_fips_device` after provisioning.\nRequires the PIV key to be stored on a FIPS 140-2 Level 1 or higher validated device."]
    pub fn require_fips_device(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_fips_device", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssh_key_size` after provisioning.\nSpecifies the allowed SSH key sizes in bits. Valid sizes depend on key type. Ed25519 has a fixed key size and does not accept this parameter."]
    pub fn ssh_key_size(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_key_size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssh_key_type` after provisioning.\nSpecifies the allowed SSH key types. Valid values are `ecdsa`, `ed25519`, and `rsa`."]
    pub fn ssh_key_type(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_key_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `touch_policy` after provisioning.\nDefines when physical touch is required to use the SSH key. Valid values: `never` (no touch required), `always` (touch required for each use), `cached` (touch cached for 15 seconds).\nAvailable values: \"never\", \"always\", \"cached\"."]
    pub fn touch_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.touch_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustOrganization_ServiceTokenInactivity {}

impl DataZeroTrustOrganization_ServiceTokenInactivity { }

impl terrars::ToListMappable for DataZeroTrustOrganization_ServiceTokenInactivity {
    type O = terrars::BlockAssignable<DataZeroTrustOrganization_ServiceTokenInactivity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustOrganization_ServiceTokenInactivity {}

impl BuildDataZeroTrustOrganization_ServiceTokenInactivity {
    pub fn build(self) -> DataZeroTrustOrganization_ServiceTokenInactivity {
        DataZeroTrustOrganization_ServiceTokenInactivity {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustOrganization_ServiceTokenInactivityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustOrganization_ServiceTokenInactivityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustOrganization_ServiceTokenInactivityRef {
        DataZeroTrustOrganization_ServiceTokenInactivityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustOrganization_ServiceTokenInactivityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe action applied to an inactive service token.\nAvailable values: \"disable\", \"delete\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether automatic enforcement for inactive service tokens is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `inactivity_threshold_days` after provisioning.\nThe number of days a service token must be inactive before the configured action is applied."]
    pub fn inactivity_threshold_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inactivity_threshold_days", self.base))
    }
}
