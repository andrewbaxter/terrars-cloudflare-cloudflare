#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustOrganizationData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_authenticate_via_warp: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_redirect_to_identity: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_pages: std::option::Option<ZeroTrustOrganization_CustomPages>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny_unmatched_requests: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny_unmatched_requests_exempted_zone_names: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_ui_read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_design: std::option::Option<ZeroTrustOrganization_LoginDesign>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mfa_config: std::option::Option<ZeroTrustOrganization_MfaConfig>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mfa_configuration_allowed: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mfa_piv_key_requirements: std::option::Option<ZeroTrustOrganization_MfaPivKeyRequirements>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mfa_required_for_all_apps: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_token_inactivity: std::option::Option<ZeroTrustOrganization_ServiceTokenInactivity>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strict_service_token_auth: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ui_read_only_toggle_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_seat_expiration_inactive_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warp_auth_non_browser_401: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warp_auth_session_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ZeroTrustOrganization_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustOrganizationData_>,
}

#[derive(Clone)]
pub struct ZeroTrustOrganization(std::rc::Rc<ZeroTrustOrganization_>);

impl ZeroTrustOrganization {
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

    #[doc =
        "Set the field `account_id`.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_authenticate_via_warp`.\nWhen set to true, users can authenticate via WARP for any application in your organization. Application settings will take precedence over this value."]
    pub fn set_allow_authenticate_via_warp(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_authenticate_via_warp = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `auth_domain`.\nThe unique subdomain assigned to your Zero Trust organization. If omitted on creation, a unique subdomain is auto-generated in the format `adjective-noun-hex4` (e.g. `frosty-moon-7a3b.cloudflareaccess.com`)."]
    pub fn set_auth_domain(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().auth_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `auto_redirect_to_identity`.\nWhen set to `true`, users skip the identity provider selection step during login."]
    pub fn set_auto_redirect_to_identity(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_redirect_to_identity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_pages`.\n"]
    pub fn set_custom_pages(self, v: impl std::convert::Into<ZeroTrustOrganization_CustomPages>) -> Self {
        self.0.data.borrow_mut().custom_pages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deny_unmatched_requests`.\nDetermines whether to deny all requests to Cloudflare-protected resources that lack an associated Access application. If enabled, you must explicitly configure an Access application and policy to allow traffic to your Cloudflare-protected resources. For domains you want to be public across all subdomains, add the domain to the `deny_unmatched_requests_exempted_zone_names` array."]
    pub fn set_deny_unmatched_requests(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deny_unmatched_requests = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deny_unmatched_requests_exempted_zone_names`.\nContains zone names to exempt from the `deny_unmatched_requests` feature. Requests to a subdomain in an exempted zone will block unauthenticated traffic by default if there is a configured Access application and policy that matches the request."]
    pub fn set_deny_unmatched_requests_exempted_zone_names(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().deny_unmatched_requests_exempted_zone_names = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_ui_read_only`.\nLock all settings as Read-Only in the Dashboard, regardless of user permission. Updates may only be made via the API or Terraform for this account when enabled."]
    pub fn set_is_ui_read_only(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_ui_read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `login_design`.\n"]
    pub fn set_login_design(self, v: impl std::convert::Into<ZeroTrustOrganization_LoginDesign>) -> Self {
        self.0.data.borrow_mut().login_design = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mfa_config`.\nConfigures multi-factor authentication (MFA) settings for an organization."]
    pub fn set_mfa_config(self, v: impl std::convert::Into<ZeroTrustOrganization_MfaConfig>) -> Self {
        self.0.data.borrow_mut().mfa_config = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mfa_configuration_allowed`.\nIndicates if this organization can enforce multi-factor authentication (MFA) requirements at the application and policy level."]
    pub fn set_mfa_configuration_allowed(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().mfa_configuration_allowed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mfa_piv_key_requirements`.\nConfigures PIV key requirements for MFA using hardware security keys."]
    pub fn set_mfa_piv_key_requirements(
        self,
        v: impl std::convert::Into<ZeroTrustOrganization_MfaPivKeyRequirements>,
    ) -> Self {
        self.0.data.borrow_mut().mfa_piv_key_requirements = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mfa_required_for_all_apps`.\nDetermines whether global MFA settings apply to applications by default. The organization must have MFA enabled with at least one authentication method and a session duration configured. Note: 'allowed_authenticators' cannot contain only the infrastructure SSH authenticators ('piv_key' and 'ssh_fido2_key') if the organization has any non-infrastructure applications."]
    pub fn set_mfa_required_for_all_apps(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().mfa_required_for_all_apps = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nThe name of your Zero Trust organization. When omitted on creation, defaults to the provided auth_domain; when both are omitted, defaults to the auto-generated subdomain slug (e.g. frosty-moon-7a3b)."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_token_inactivity`.\nConfigures automatic enforcement for inactive service tokens. A service token is inactive if no policy references it, and it has not successfully authenticated with an Access application during the selected inactivity period. This setting applies to every service token in your Zero Trust account."]
    pub fn set_service_token_inactivity(
        self,
        v: impl std::convert::Into<ZeroTrustOrganization_ServiceTokenInactivity>,
    ) -> Self {
        self.0.data.borrow_mut().service_token_inactivity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_duration`.\nThe amount of time that tokens issued for applications will be valid. Must be in the format `300ms` or `2h45m`. Valid time units are: ns, us (or µs), ms, s, m, h."]
    pub fn set_session_duration(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().session_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `strict_service_token_auth`.\nEnables new behaviors for requests made with Access service tokens. Unauthorized requests emit audit logs, and return a 401 or 403 status code in the response instead of redirecting to the login page. Successful requests no longer receive a CF_Authorization cookie in the response. Zero Trust organizations created on or after October 5, 2026 will have this setting enabled by default, and cannot disable it."]
    pub fn set_strict_service_token_auth(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().strict_service_token_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ui_read_only_toggle_reason`.\nA description of the reason why the UI read only field is being toggled."]
    pub fn set_ui_read_only_toggle_reason(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().ui_read_only_toggle_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user_seat_expiration_inactive_time`.\nThe amount of time a user seat is inactive before it expires. When the user seat exceeds the set time of inactivity, the user is removed as an active seat and no longer counts against your Teams seat count.  Minimum value for this setting is 1 month (730h). Must be in the format `300ms` or `2h45m`. Valid time units are: `ns`, `us` (or `µs`), `ms`, `s`, `m`, `h`."]
    pub fn set_user_seat_expiration_inactive_time(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().user_seat_expiration_inactive_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `warp_auth_non_browser_401`.\nWhen enabled, unsuccessful WARP authentication requests with a non-HTML Accept header return a 401 response instead of redirecting to the login page."]
    pub fn set_warp_auth_non_browser_401(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().warp_auth_non_browser_401 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `warp_auth_session_duration`.\nThe amount of time that tokens issued for applications will be valid. Must be in the format `30m` or `2h45m`. Valid time units are: m, h."]
    pub fn set_warp_auth_session_duration(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().warp_auth_session_duration = std::option::Option::Some(v.into());
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
    pub fn custom_pages(&self) -> ZeroTrustOrganization_CustomPagesRef {
        ZeroTrustOrganization_CustomPagesRef::new(
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
    pub fn login_design(&self) -> ZeroTrustOrganization_LoginDesignRef {
        ZeroTrustOrganization_LoginDesignRef::new(
            self.shared().clone(),
            format!("{}.login_design", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `mfa_config` after provisioning.\nConfigures multi-factor authentication (MFA) settings for an organization."]
    pub fn mfa_config(&self) -> ZeroTrustOrganization_MfaConfigRef {
        ZeroTrustOrganization_MfaConfigRef::new(self.shared().clone(), format!("{}.mfa_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mfa_configuration_allowed` after provisioning.\nIndicates if this organization can enforce multi-factor authentication (MFA) requirements at the application and policy level."]
    pub fn mfa_configuration_allowed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mfa_configuration_allowed", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mfa_piv_key_requirements` after provisioning.\nConfigures PIV key requirements for MFA using hardware security keys."]
    pub fn mfa_piv_key_requirements(&self) -> ZeroTrustOrganization_MfaPivKeyRequirementsRef {
        ZeroTrustOrganization_MfaPivKeyRequirementsRef::new(
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
    pub fn service_token_inactivity(&self) -> ZeroTrustOrganization_ServiceTokenInactivityRef {
        ZeroTrustOrganization_ServiceTokenInactivityRef::new(
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

impl terrars::Referable for ZeroTrustOrganization {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustOrganization { }

impl terrars::ToListMappable for ZeroTrustOrganization {
    type O = terrars::ListRef<ZeroTrustOrganizationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustOrganization_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_organization".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustOrganization {
    pub tf_id: std::string::String,
}

impl BuildZeroTrustOrganization {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustOrganization {
        let out = ZeroTrustOrganization(std::rc::Rc::new(ZeroTrustOrganization_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustOrganizationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                allow_authenticate_via_warp: core::default::Default::default(),
                auth_domain: core::default::Default::default(),
                auto_redirect_to_identity: core::default::Default::default(),
                custom_pages: core::default::Default::default(),
                deny_unmatched_requests: core::default::Default::default(),
                deny_unmatched_requests_exempted_zone_names: core::default::Default::default(),
                is_ui_read_only: core::default::Default::default(),
                login_design: core::default::Default::default(),
                mfa_config: core::default::Default::default(),
                mfa_configuration_allowed: core::default::Default::default(),
                mfa_piv_key_requirements: core::default::Default::default(),
                mfa_required_for_all_apps: core::default::Default::default(),
                name: core::default::Default::default(),
                service_token_inactivity: core::default::Default::default(),
                session_duration: core::default::Default::default(),
                strict_service_token_auth: core::default::Default::default(),
                ui_read_only_toggle_reason: core::default::Default::default(),
                user_seat_expiration_inactive_time: core::default::Default::default(),
                warp_auth_non_browser_401: core::default::Default::default(),
                warp_auth_session_duration: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustOrganizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustOrganizationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
    pub fn custom_pages(&self) -> ZeroTrustOrganization_CustomPagesRef {
        ZeroTrustOrganization_CustomPagesRef::new(
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
    pub fn login_design(&self) -> ZeroTrustOrganization_LoginDesignRef {
        ZeroTrustOrganization_LoginDesignRef::new(
            self.shared().clone(),
            format!("{}.login_design", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `mfa_config` after provisioning.\nConfigures multi-factor authentication (MFA) settings for an organization."]
    pub fn mfa_config(&self) -> ZeroTrustOrganization_MfaConfigRef {
        ZeroTrustOrganization_MfaConfigRef::new(self.shared().clone(), format!("{}.mfa_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mfa_configuration_allowed` after provisioning.\nIndicates if this organization can enforce multi-factor authentication (MFA) requirements at the application and policy level."]
    pub fn mfa_configuration_allowed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mfa_configuration_allowed", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mfa_piv_key_requirements` after provisioning.\nConfigures PIV key requirements for MFA using hardware security keys."]
    pub fn mfa_piv_key_requirements(&self) -> ZeroTrustOrganization_MfaPivKeyRequirementsRef {
        ZeroTrustOrganization_MfaPivKeyRequirementsRef::new(
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
    pub fn service_token_inactivity(&self) -> ZeroTrustOrganization_ServiceTokenInactivityRef {
        ZeroTrustOrganization_ServiceTokenInactivityRef::new(
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
pub struct ZeroTrustOrganization_CustomPages {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forbidden: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_denied: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustOrganization_CustomPages {
    #[doc =
        "Set the field `forbidden`.\nThe uid of the custom page to use when a user is denied access after failing a non-identity rule."]
    pub fn set_forbidden(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.forbidden = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_denied`.\nThe uid of the custom page to use when a user is denied access."]
    pub fn set_identity_denied(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_denied = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustOrganization_CustomPages {
    type O = terrars::BlockAssignable<ZeroTrustOrganization_CustomPages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustOrganization_CustomPages {}

impl BuildZeroTrustOrganization_CustomPages {
    pub fn build(self) -> ZeroTrustOrganization_CustomPages {
        ZeroTrustOrganization_CustomPages {
            forbidden: core::default::Default::default(),
            identity_denied: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustOrganization_CustomPagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustOrganization_CustomPagesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustOrganization_CustomPagesRef {
        ZeroTrustOrganization_CustomPagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustOrganization_CustomPagesRef {
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
pub struct ZeroTrustOrganization_LoginDesign {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    background_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    footer_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logo_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_color: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustOrganization_LoginDesign {
    #[doc = "Set the field `background_color`.\nThe background color on your login page."]
    pub fn set_background_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.background_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `footer_text`.\nThe text at the bottom of your login page."]
    pub fn set_footer_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.footer_text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_text`.\nThe text at the top of your login page."]
    pub fn set_header_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logo_path`.\nThe URL of the logo on your login page."]
    pub fn set_logo_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logo_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text_color`.\nThe text color on your login page."]
    pub fn set_text_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text_color = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustOrganization_LoginDesign {
    type O = terrars::BlockAssignable<ZeroTrustOrganization_LoginDesign>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustOrganization_LoginDesign {}

impl BuildZeroTrustOrganization_LoginDesign {
    pub fn build(self) -> ZeroTrustOrganization_LoginDesign {
        ZeroTrustOrganization_LoginDesign {
            background_color: core::default::Default::default(),
            footer_text: core::default::Default::default(),
            header_text: core::default::Default::default(),
            logo_path: core::default::Default::default(),
            text_color: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustOrganization_LoginDesignRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustOrganization_LoginDesignRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustOrganization_LoginDesignRef {
        ZeroTrustOrganization_LoginDesignRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustOrganization_LoginDesignRef {
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
pub struct ZeroTrustOrganization_MfaConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_authenticators: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amr_matching_session_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required_aaguids: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustOrganization_MfaConfig {
    #[doc =
        "Set the field `allowed_authenticators`.\nLists the MFA methods that users can authenticate with. The `piv_key` and `ssh_fido2_key` values are supported only for infrastructure applications."]
    pub fn set_allowed_authenticators(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_authenticators = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `amr_matching_session_duration`.\nAllows a user to skip MFA via Authentication Method Reference (AMR) matching when the AMR claim provided by the IdP the user used to authenticate contains \"mfa\". Must be in minutes (m) or hours (h). Minimum: 0m. Maximum: 720h (30 days)."]
    pub fn set_amr_matching_session_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.amr_matching_session_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `required_aaguids`.\nSpecifies a Cloudflare List of required FIDO2 authenticator device AAGUIDs."]
    pub fn set_required_aaguids(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.required_aaguids = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_duration`.\nDefines the duration of an MFA session. Must be in minutes (m) or hours (h). Minimum: 0m. Maximum: 720h (30 days). Examples:`5m` or `24h`."]
    pub fn set_session_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.session_duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustOrganization_MfaConfig {
    type O = terrars::BlockAssignable<ZeroTrustOrganization_MfaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustOrganization_MfaConfig {}

impl BuildZeroTrustOrganization_MfaConfig {
    pub fn build(self) -> ZeroTrustOrganization_MfaConfig {
        ZeroTrustOrganization_MfaConfig {
            allowed_authenticators: core::default::Default::default(),
            amr_matching_session_duration: core::default::Default::default(),
            required_aaguids: core::default::Default::default(),
            session_duration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustOrganization_MfaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustOrganization_MfaConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustOrganization_MfaConfigRef {
        ZeroTrustOrganization_MfaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustOrganization_MfaConfigRef {
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
pub struct ZeroTrustOrganization_MfaPivKeyRequirements {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pin_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_fips_device: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_key_size: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_key_type: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    touch_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustOrganization_MfaPivKeyRequirements {
    #[doc =
        "Set the field `pin_policy`.\nDefines when a PIN is required to use the SSH key. Valid values: `never` (no PIN required), `once` (PIN required once per session), `always` (PIN required for each use).\nAvailable values: \"never\", \"once\", \"always\"."]
    pub fn set_pin_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pin_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `require_fips_device`.\nRequires the PIV key to be stored on a FIPS 140-2 Level 1 or higher validated device."]
    pub fn set_require_fips_device(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_fips_device = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssh_key_size`.\nSpecifies the allowed SSH key sizes in bits. Valid sizes depend on key type. Ed25519 has a fixed key size and does not accept this parameter."]
    pub fn set_ssh_key_size(mut self, v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>) -> Self {
        self.ssh_key_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssh_key_type`.\nSpecifies the allowed SSH key types. Valid values are `ecdsa`, `ed25519`, and `rsa`."]
    pub fn set_ssh_key_type(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ssh_key_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `touch_policy`.\nDefines when physical touch is required to use the SSH key. Valid values: `never` (no touch required), `always` (touch required for each use), `cached` (touch cached for 15 seconds).\nAvailable values: \"never\", \"always\", \"cached\"."]
    pub fn set_touch_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.touch_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustOrganization_MfaPivKeyRequirements {
    type O = terrars::BlockAssignable<ZeroTrustOrganization_MfaPivKeyRequirements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustOrganization_MfaPivKeyRequirements {}

impl BuildZeroTrustOrganization_MfaPivKeyRequirements {
    pub fn build(self) -> ZeroTrustOrganization_MfaPivKeyRequirements {
        ZeroTrustOrganization_MfaPivKeyRequirements {
            pin_policy: core::default::Default::default(),
            require_fips_device: core::default::Default::default(),
            ssh_key_size: core::default::Default::default(),
            ssh_key_type: core::default::Default::default(),
            touch_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustOrganization_MfaPivKeyRequirementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustOrganization_MfaPivKeyRequirementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustOrganization_MfaPivKeyRequirementsRef {
        ZeroTrustOrganization_MfaPivKeyRequirementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustOrganization_MfaPivKeyRequirementsRef {
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
pub struct ZeroTrustOrganization_ServiceTokenInactivity {
    action: terrars::PrimField<std::string::String>,
    enabled: terrars::PrimField<bool>,
    inactivity_threshold_days: terrars::PrimField<f64>,
}

impl ZeroTrustOrganization_ServiceTokenInactivity { }

impl terrars::ToListMappable for ZeroTrustOrganization_ServiceTokenInactivity {
    type O = terrars::BlockAssignable<ZeroTrustOrganization_ServiceTokenInactivity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustOrganization_ServiceTokenInactivity {
    #[doc = "The action applied to an inactive service token.\nAvailable values: \"disable\", \"delete\"."]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = "Whether automatic enforcement for inactive service tokens is enabled."]
    pub enabled: terrars::PrimField<bool>,
    #[doc = "The number of days a service token must be inactive before the configured action is applied."]
    pub inactivity_threshold_days: terrars::PrimField<f64>,
}

impl BuildZeroTrustOrganization_ServiceTokenInactivity {
    pub fn build(self) -> ZeroTrustOrganization_ServiceTokenInactivity {
        ZeroTrustOrganization_ServiceTokenInactivity {
            action: self.action,
            enabled: self.enabled,
            inactivity_threshold_days: self.inactivity_threshold_days,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustOrganization_ServiceTokenInactivityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustOrganization_ServiceTokenInactivityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustOrganization_ServiceTokenInactivityRef {
        ZeroTrustOrganization_ServiceTokenInactivityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustOrganization_ServiceTokenInactivityRef {
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
