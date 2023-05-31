use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AccessApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_idps: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_launcher_visible: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_redirect_to_identity: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_deny_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_deny_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_binding_cookie: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_only_cookie_attribute: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logo_url: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    same_site_cookie_attribute: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_auth_401_redirect: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_interstitial: Option<PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cors_headers: Option<Vec<AccessApplicationCorsHeadersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saas_app: Option<Vec<AccessApplicationSaasAppEl>>,
    dynamic: AccessApplicationDynamic,
}

struct AccessApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessApplicationData>,
}

#[derive(Clone)]
pub struct AccessApplication(Rc<AccessApplication_>);

impl AccessApplication {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `account_id`.\nThe account identifier to target for the resource. Conflicts with `zone_id`."]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }

    #[doc= "Set the field `allowed_idps`.\nThe identity providers selected for the application."]
    pub fn set_allowed_idps(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().allowed_idps = Some(v.into());
        self
    }

    #[doc= "Set the field `app_launcher_visible`.\nOption to show/hide applications in App Launcher. Defaults to `true`."]
    pub fn set_app_launcher_visible(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().app_launcher_visible = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_redirect_to_identity`.\nOption to skip identity provider selection if only one is configured in `allowed_idps`. Defaults to `false`."]
    pub fn set_auto_redirect_to_identity(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_redirect_to_identity = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_deny_message`.\nOption that returns a custom error message when a user is denied access to the application."]
    pub fn set_custom_deny_message(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_deny_message = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_deny_url`.\nOption that redirects to a custom URL when a user is denied access to the application."]
    pub fn set_custom_deny_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_deny_url = Some(v.into());
        self
    }

    #[doc= "Set the field `domain`.\nThe complete URL of the asset you wish to put Cloudflare Access in front of. Can include subdomains or paths. Or both."]
    pub fn set_domain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_binding_cookie`.\nOption to provide increased security against compromised authorization tokens and CSRF attacks by requiring an additional \"binding\" cookie on requests. Defaults to `false`."]
    pub fn set_enable_binding_cookie(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_binding_cookie = Some(v.into());
        self
    }

    #[doc= "Set the field `http_only_cookie_attribute`.\nOption to add the `HttpOnly` cookie flag to access tokens."]
    pub fn set_http_only_cookie_attribute(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().http_only_cookie_attribute = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `logo_url`.\nImage URL for the logo shown in the app launcher dashboard."]
    pub fn set_logo_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().logo_url = Some(v.into());
        self
    }

    #[doc= "Set the field `same_site_cookie_attribute`.\nDefines the same-site cookie setting for access tokens. Available values: `none`, `lax`, `strict`."]
    pub fn set_same_site_cookie_attribute(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().same_site_cookie_attribute = Some(v.into());
        self
    }

    #[doc= "Set the field `service_auth_401_redirect`.\nOption to return a 401 status code in service authentication rules on failed requests. Defaults to `false`."]
    pub fn set_service_auth_401_redirect(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().service_auth_401_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `session_duration`.\nHow often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`. Defaults to `24h`."]
    pub fn set_session_duration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().session_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `skip_interstitial`.\nOption to skip the authorization interstitial when using the CLI. Defaults to `false`."]
    pub fn set_skip_interstitial(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_interstitial = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nThe application type. Available values: `app_launcher`, `bookmark`, `biso`, `dash_sso`, `saas`, `self_hosted`, `ssh`, `vnc`, `warp`. Defaults to `self_hosted`."]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `zone_id`.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn set_zone_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone_id = Some(v.into());
        self
    }

    #[doc= "Set the field `cors_headers`.\n"]
    pub fn set_cors_headers(self, v: impl Into<BlockAssignable<AccessApplicationCorsHeadersEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cors_headers = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cors_headers = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `saas_app`.\n"]
    pub fn set_saas_app(self, v: impl Into<BlockAssignable<AccessApplicationSaasAppEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().saas_app = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.saas_app = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Conflicts with `zone_id`."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allowed_idps` after provisioning.\nThe identity providers selected for the application."]
    pub fn allowed_idps(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_idps", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `app_launcher_visible` after provisioning.\nOption to show/hide applications in App Launcher. Defaults to `true`."]
    pub fn app_launcher_visible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_launcher_visible", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `aud` after provisioning.\nApplication Audience (AUD) Tag of the application."]
    pub fn aud(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aud", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_redirect_to_identity` after provisioning.\nOption to skip identity provider selection if only one is configured in `allowed_idps`. Defaults to `false`."]
    pub fn auto_redirect_to_identity(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_redirect_to_identity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_deny_message` after provisioning.\nOption that returns a custom error message when a user is denied access to the application."]
    pub fn custom_deny_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_deny_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_deny_url` after provisioning.\nOption that redirects to a custom URL when a user is denied access to the application."]
    pub fn custom_deny_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_deny_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\nThe complete URL of the asset you wish to put Cloudflare Access in front of. Can include subdomains or paths. Or both."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_binding_cookie` after provisioning.\nOption to provide increased security against compromised authorization tokens and CSRF attacks by requiring an additional \"binding\" cookie on requests. Defaults to `false`."]
    pub fn enable_binding_cookie(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_binding_cookie", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `http_only_cookie_attribute` after provisioning.\nOption to add the `HttpOnly` cookie flag to access tokens."]
    pub fn http_only_cookie_attribute(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_only_cookie_attribute", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logo_url` after provisioning.\nImage URL for the logo shown in the app launcher dashboard."]
    pub fn logo_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logo_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nFriendly name of the Access Application."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `same_site_cookie_attribute` after provisioning.\nDefines the same-site cookie setting for access tokens. Available values: `none`, `lax`, `strict`."]
    pub fn same_site_cookie_attribute(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.same_site_cookie_attribute", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_auth_401_redirect` after provisioning.\nOption to return a 401 status code in service authentication rules on failed requests. Defaults to `false`."]
    pub fn service_auth_401_redirect(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_auth_401_redirect", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `session_duration` after provisioning.\nHow often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`. Defaults to `24h`."]
    pub fn session_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `skip_interstitial` after provisioning.\nOption to skip the authorization interstitial when using the CLI. Defaults to `false`."]
    pub fn skip_interstitial(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_interstitial", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe application type. Available values: `app_launcher`, `bookmark`, `biso`, `dash_sso`, `saas`, `self_hosted`, `ssh`, `vnc`, `warp`. Defaults to `self_hosted`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cors_headers` after provisioning.\n"]
    pub fn cors_headers(&self) -> ListRef<AccessApplicationCorsHeadersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_headers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `saas_app` after provisioning.\n"]
    pub fn saas_app(&self) -> ListRef<AccessApplicationSaasAppElRef> {
        ListRef::new(self.shared().clone(), format!("{}.saas_app", self.extract_ref()))
    }
}

impl Referable for AccessApplication {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AccessApplication { }

impl ToListMappable for AccessApplication {
    type O = ListRef<AccessApplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AccessApplication_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_access_application".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessApplication {
    pub tf_id: String,
    #[doc= "Friendly name of the Access Application."]
    pub name: PrimField<String>,
}

impl BuildAccessApplication {
    pub fn build(self, stack: &mut Stack) -> AccessApplication {
        let out = AccessApplication(Rc::new(AccessApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AccessApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                allowed_idps: core::default::Default::default(),
                app_launcher_visible: core::default::Default::default(),
                auto_redirect_to_identity: core::default::Default::default(),
                custom_deny_message: core::default::Default::default(),
                custom_deny_url: core::default::Default::default(),
                domain: core::default::Default::default(),
                enable_binding_cookie: core::default::Default::default(),
                http_only_cookie_attribute: core::default::Default::default(),
                id: core::default::Default::default(),
                logo_url: core::default::Default::default(),
                name: self.name,
                same_site_cookie_attribute: core::default::Default::default(),
                service_auth_401_redirect: core::default::Default::default(),
                session_duration: core::default::Default::default(),
                skip_interstitial: core::default::Default::default(),
                type_: core::default::Default::default(),
                zone_id: core::default::Default::default(),
                cors_headers: core::default::Default::default(),
                saas_app: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessApplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessApplicationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Conflicts with `zone_id`."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allowed_idps` after provisioning.\nThe identity providers selected for the application."]
    pub fn allowed_idps(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_idps", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `app_launcher_visible` after provisioning.\nOption to show/hide applications in App Launcher. Defaults to `true`."]
    pub fn app_launcher_visible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_launcher_visible", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `aud` after provisioning.\nApplication Audience (AUD) Tag of the application."]
    pub fn aud(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aud", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_redirect_to_identity` after provisioning.\nOption to skip identity provider selection if only one is configured in `allowed_idps`. Defaults to `false`."]
    pub fn auto_redirect_to_identity(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_redirect_to_identity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_deny_message` after provisioning.\nOption that returns a custom error message when a user is denied access to the application."]
    pub fn custom_deny_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_deny_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_deny_url` after provisioning.\nOption that redirects to a custom URL when a user is denied access to the application."]
    pub fn custom_deny_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_deny_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\nThe complete URL of the asset you wish to put Cloudflare Access in front of. Can include subdomains or paths. Or both."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_binding_cookie` after provisioning.\nOption to provide increased security against compromised authorization tokens and CSRF attacks by requiring an additional \"binding\" cookie on requests. Defaults to `false`."]
    pub fn enable_binding_cookie(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_binding_cookie", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `http_only_cookie_attribute` after provisioning.\nOption to add the `HttpOnly` cookie flag to access tokens."]
    pub fn http_only_cookie_attribute(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_only_cookie_attribute", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logo_url` after provisioning.\nImage URL for the logo shown in the app launcher dashboard."]
    pub fn logo_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logo_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nFriendly name of the Access Application."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `same_site_cookie_attribute` after provisioning.\nDefines the same-site cookie setting for access tokens. Available values: `none`, `lax`, `strict`."]
    pub fn same_site_cookie_attribute(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.same_site_cookie_attribute", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_auth_401_redirect` after provisioning.\nOption to return a 401 status code in service authentication rules on failed requests. Defaults to `false`."]
    pub fn service_auth_401_redirect(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_auth_401_redirect", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `session_duration` after provisioning.\nHow often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`. Defaults to `24h`."]
    pub fn session_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `skip_interstitial` after provisioning.\nOption to skip the authorization interstitial when using the CLI. Defaults to `false`."]
    pub fn skip_interstitial(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_interstitial", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe application type. Available values: `app_launcher`, `bookmark`, `biso`, `dash_sso`, `saas`, `self_hosted`, `ssh`, `vnc`, `warp`. Defaults to `self_hosted`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cors_headers` after provisioning.\n"]
    pub fn cors_headers(&self) -> ListRef<AccessApplicationCorsHeadersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_headers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `saas_app` after provisioning.\n"]
    pub fn saas_app(&self) -> ListRef<AccessApplicationSaasAppElRef> {
        ListRef::new(self.shared().clone(), format!("{}.saas_app", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AccessApplicationCorsHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_all_headers: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_all_methods: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_all_origins: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_credentials: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_methods: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_origins: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<PrimField<f64>>,
}

impl AccessApplicationCorsHeadersEl {
    #[doc= "Set the field `allow_all_headers`.\nValue to determine whether all HTTP headers are exposed."]
    pub fn set_allow_all_headers(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_all_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_all_methods`.\nValue to determine whether all methods are exposed."]
    pub fn set_allow_all_methods(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_all_methods = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_all_origins`.\nValue to determine whether all origins are permitted to make CORS requests."]
    pub fn set_allow_all_origins(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_all_origins = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_credentials`.\nValue to determine if credentials (cookies, authorization headers, or TLS client certificates) are included with requests."]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }

    #[doc= "Set the field `allowed_headers`.\nList of HTTP headers to expose via CORS."]
    pub fn set_allowed_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `allowed_methods`.\nList of methods to expose via CORS."]
    pub fn set_allowed_methods(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_methods = Some(v.into());
        self
    }

    #[doc= "Set the field `allowed_origins`.\nList of origins permitted to make CORS requests."]
    pub fn set_allowed_origins(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_origins = Some(v.into());
        self
    }

    #[doc= "Set the field `max_age`.\nThe maximum time a preflight request will be cached."]
    pub fn set_max_age(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age = Some(v.into());
        self
    }
}

impl ToListMappable for AccessApplicationCorsHeadersEl {
    type O = BlockAssignable<AccessApplicationCorsHeadersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessApplicationCorsHeadersEl {}

impl BuildAccessApplicationCorsHeadersEl {
    pub fn build(self) -> AccessApplicationCorsHeadersEl {
        AccessApplicationCorsHeadersEl {
            allow_all_headers: core::default::Default::default(),
            allow_all_methods: core::default::Default::default(),
            allow_all_origins: core::default::Default::default(),
            allow_credentials: core::default::Default::default(),
            allowed_headers: core::default::Default::default(),
            allowed_methods: core::default::Default::default(),
            allowed_origins: core::default::Default::default(),
            max_age: core::default::Default::default(),
        }
    }
}

pub struct AccessApplicationCorsHeadersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessApplicationCorsHeadersElRef {
    fn new(shared: StackShared, base: String) -> AccessApplicationCorsHeadersElRef {
        AccessApplicationCorsHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessApplicationCorsHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_all_headers` after provisioning.\nValue to determine whether all HTTP headers are exposed."]
    pub fn allow_all_headers(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_all_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_all_methods` after provisioning.\nValue to determine whether all methods are exposed."]
    pub fn allow_all_methods(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_all_methods", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_all_origins` after provisioning.\nValue to determine whether all origins are permitted to make CORS requests."]
    pub fn allow_all_origins(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_all_origins", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_credentials` after provisioning.\nValue to determine if credentials (cookies, authorization headers, or TLS client certificates) are included with requests."]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc= "Get a reference to the value of field `allowed_headers` after provisioning.\nList of HTTP headers to expose via CORS."]
    pub fn allowed_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allowed_methods` after provisioning.\nList of methods to expose via CORS."]
    pub fn allowed_methods(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_methods", self.base))
    }

    #[doc= "Get a reference to the value of field `allowed_origins` after provisioning.\nList of origins permitted to make CORS requests."]
    pub fn allowed_origins(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_origins", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age` after provisioning.\nThe maximum time a preflight request will be cached."]
    pub fn max_age(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessApplicationSaasAppEl {
    consumer_service_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_id_format: Option<PrimField<String>>,
    sp_entity_id: PrimField<String>,
}

impl AccessApplicationSaasAppEl {
    #[doc= "Set the field `name_id_format`.\nThe format of the name identifier sent to the SaaS application. Defaults to `email`."]
    pub fn set_name_id_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_id_format = Some(v.into());
        self
    }
}

impl ToListMappable for AccessApplicationSaasAppEl {
    type O = BlockAssignable<AccessApplicationSaasAppEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessApplicationSaasAppEl {
    #[doc= "The service provider's endpoint that is responsible for receiving and parsing a SAML assertion."]
    pub consumer_service_url: PrimField<String>,
    #[doc= "A globally unique name for an identity or service provider."]
    pub sp_entity_id: PrimField<String>,
}

impl BuildAccessApplicationSaasAppEl {
    pub fn build(self) -> AccessApplicationSaasAppEl {
        AccessApplicationSaasAppEl {
            consumer_service_url: self.consumer_service_url,
            name_id_format: core::default::Default::default(),
            sp_entity_id: self.sp_entity_id,
        }
    }
}

pub struct AccessApplicationSaasAppElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessApplicationSaasAppElRef {
    fn new(shared: StackShared, base: String) -> AccessApplicationSaasAppElRef {
        AccessApplicationSaasAppElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessApplicationSaasAppElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `consumer_service_url` after provisioning.\nThe service provider's endpoint that is responsible for receiving and parsing a SAML assertion."]
    pub fn consumer_service_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.consumer_service_url", self.base))
    }

    #[doc= "Get a reference to the value of field `name_id_format` after provisioning.\nThe format of the name identifier sent to the SaaS application. Defaults to `email`."]
    pub fn name_id_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_id_format", self.base))
    }

    #[doc= "Get a reference to the value of field `sp_entity_id` after provisioning.\nA globally unique name for an identity or service provider."]
    pub fn sp_entity_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sp_entity_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessApplicationDynamic {
    cors_headers: Option<DynamicBlock<AccessApplicationCorsHeadersEl>>,
    saas_app: Option<DynamicBlock<AccessApplicationSaasAppEl>>,
}
