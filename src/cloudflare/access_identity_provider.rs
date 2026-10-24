use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AccessIdentityProviderData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<AccessIdentityProviderConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scim_config: std::option::Option<std::vec::Vec<AccessIdentityProviderScimConfigEl>>,
    dynamic: AccessIdentityProviderDynamic,
}

struct AccessIdentityProvider_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccessIdentityProviderData>,
}

#[derive(Clone)]
pub struct AccessIdentityProvider(std::rc::Rc<AccessIdentityProvider_>);

impl AccessIdentityProvider {
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
        "Set the field `account_id`.\nThe account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zone_id`.\nThe zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessIdentityProviderConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scim_config`.\n"]
    pub fn set_scim_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessIdentityProviderScimConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scim_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scim_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFriendly name of the Access Identity Provider configuration."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe provider type to use. Available values: `centrify`, `facebook`, `google-apps`, `oidc`, `github`, `google`, `saml`, `linkedin`, `azureAD`, `okta`, `onetimepin`, `onelogin`, `yandex`."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<AccessIdentityProviderConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scim_config` after provisioning.\n"]
    pub fn scim_config(&self) -> terrars::ListRef<AccessIdentityProviderScimConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scim_config", self.extract_ref()))
    }
}

impl terrars::Referable for AccessIdentityProvider {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AccessIdentityProvider { }

impl terrars::ToListMappable for AccessIdentityProvider {
    type O = terrars::ListRef<AccessIdentityProviderRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AccessIdentityProvider_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_access_identity_provider".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessIdentityProvider {
    pub tf_id: std::string::String,
    #[doc = "Friendly name of the Access Identity Provider configuration."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The provider type to use. Available values: `centrify`, `facebook`, `google-apps`, `oidc`, `github`, `google`, `saml`, `linkedin`, `azureAD`, `okta`, `onetimepin`, `onelogin`, `yandex`."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildAccessIdentityProvider {
    pub fn build(self, stack: &mut terrars::Stack) -> AccessIdentityProvider {
        let out = AccessIdentityProvider(std::rc::Rc::new(AccessIdentityProvider_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AccessIdentityProviderData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                type_: self.type_,
                zone_id: core::default::Default::default(),
                config: core::default::Default::default(),
                scim_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessIdentityProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessIdentityProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessIdentityProviderRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFriendly name of the Access Identity Provider configuration."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe provider type to use. Available values: `centrify`, `facebook`, `google-apps`, `oidc`, `github`, `google`, `saml`, `linkedin`, `azureAD`, `okta`, `onetimepin`, `onelogin`, `yandex`."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<AccessIdentityProviderConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scim_config` after provisioning.\n"]
    pub fn scim_config(&self) -> terrars::ListRef<AccessIdentityProviderScimConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scim_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AccessIdentityProviderConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apps_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    centrify_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    centrify_app_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certs_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    claims: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    directory_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_attribute_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idp_public_cert: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    okta_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    onelogin_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pkce_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sign_request: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sso_target_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    support_groups: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessIdentityProviderConfigEl {
    #[doc = "Set the field `api_token`.\n"]
    pub fn set_api_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `apps_domain`.\n"]
    pub fn set_apps_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.apps_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_url`.\n"]
    pub fn set_auth_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `centrify_account`.\n"]
    pub fn set_centrify_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.centrify_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `centrify_app_id`.\n"]
    pub fn set_centrify_app_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.centrify_app_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certs_url`.\n"]
    pub fn set_certs_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certs_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `claims`.\n"]
    pub fn set_claims(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.claims = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `directory_id`.\n"]
    pub fn set_directory_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.directory_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_attribute_name`.\n"]
    pub fn set_email_attribute_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.email_attribute_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idp_public_cert`.\n"]
    pub fn set_idp_public_cert(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.idp_public_cert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer_url`.\n"]
    pub fn set_issuer_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `okta_account`.\n"]
    pub fn set_okta_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.okta_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `onelogin_account`.\n"]
    pub fn set_onelogin_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.onelogin_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pkce_enabled`.\n"]
    pub fn set_pkce_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.pkce_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redirect_url`.\n"]
    pub fn set_redirect_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.redirect_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scopes`.\n"]
    pub fn set_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sign_request`.\n"]
    pub fn set_sign_request(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.sign_request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sso_target_url`.\n"]
    pub fn set_sso_target_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sso_target_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `support_groups`.\n"]
    pub fn set_support_groups(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.support_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_url`.\n"]
    pub fn set_token_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessIdentityProviderConfigEl {
    type O = terrars::BlockAssignable<AccessIdentityProviderConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessIdentityProviderConfigEl {}

impl BuildAccessIdentityProviderConfigEl {
    pub fn build(self) -> AccessIdentityProviderConfigEl {
        AccessIdentityProviderConfigEl {
            api_token: core::default::Default::default(),
            apps_domain: core::default::Default::default(),
            attributes: core::default::Default::default(),
            auth_url: core::default::Default::default(),
            centrify_account: core::default::Default::default(),
            centrify_app_id: core::default::Default::default(),
            certs_url: core::default::Default::default(),
            claims: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            directory_id: core::default::Default::default(),
            email_attribute_name: core::default::Default::default(),
            idp_public_cert: core::default::Default::default(),
            issuer_url: core::default::Default::default(),
            okta_account: core::default::Default::default(),
            onelogin_account: core::default::Default::default(),
            pkce_enabled: core::default::Default::default(),
            redirect_url: core::default::Default::default(),
            scopes: core::default::Default::default(),
            sign_request: core::default::Default::default(),
            sso_target_url: core::default::Default::default(),
            support_groups: core::default::Default::default(),
            token_url: core::default::Default::default(),
        }
    }
}

pub struct AccessIdentityProviderConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessIdentityProviderConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessIdentityProviderConfigElRef {
        AccessIdentityProviderConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessIdentityProviderConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_token` after provisioning.\n"]
    pub fn api_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_token", self.base))
    }

    #[doc = "Get a reference to the value of field `apps_domain` after provisioning.\n"]
    pub fn apps_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apps_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_url` after provisioning.\n"]
    pub fn auth_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_url", self.base))
    }

    #[doc = "Get a reference to the value of field `centrify_account` after provisioning.\n"]
    pub fn centrify_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.centrify_account", self.base))
    }

    #[doc = "Get a reference to the value of field `centrify_app_id` after provisioning.\n"]
    pub fn centrify_app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.centrify_app_id", self.base))
    }

    #[doc = "Get a reference to the value of field `certs_url` after provisioning.\n"]
    pub fn certs_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certs_url", self.base))
    }

    #[doc = "Get a reference to the value of field `claims` after provisioning.\n"]
    pub fn claims(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.claims", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.base))
    }

    #[doc = "Get a reference to the value of field `email_attribute_name` after provisioning.\n"]
    pub fn email_attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_attribute_name", self.base))
    }

    #[doc = "Get a reference to the value of field `idp_public_cert` after provisioning.\n"]
    pub fn idp_public_cert(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idp_public_cert", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer_url` after provisioning.\n"]
    pub fn issuer_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer_url", self.base))
    }

    #[doc = "Get a reference to the value of field `okta_account` after provisioning.\n"]
    pub fn okta_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.okta_account", self.base))
    }

    #[doc = "Get a reference to the value of field `onelogin_account` after provisioning.\n"]
    pub fn onelogin_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.onelogin_account", self.base))
    }

    #[doc = "Get a reference to the value of field `pkce_enabled` after provisioning.\n"]
    pub fn pkce_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pkce_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect_url` after provisioning.\n"]
    pub fn redirect_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_url", self.base))
    }

    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc = "Get a reference to the value of field `sign_request` after provisioning.\n"]
    pub fn sign_request(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sign_request", self.base))
    }

    #[doc = "Get a reference to the value of field `sso_target_url` after provisioning.\n"]
    pub fn sso_target_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sso_target_url", self.base))
    }

    #[doc = "Get a reference to the value of field `support_groups` after provisioning.\n"]
    pub fn support_groups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `token_url` after provisioning.\n"]
    pub fn token_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessIdentityProviderScimConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_member_deprovision: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seat_deprovision: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_deprovision: std::option::Option<terrars::PrimField<bool>>,
}

impl AccessIdentityProviderScimConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_member_deprovision`.\n"]
    pub fn set_group_member_deprovision(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.group_member_deprovision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seat_deprovision`.\n"]
    pub fn set_seat_deprovision(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.seat_deprovision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_deprovision`.\n"]
    pub fn set_user_deprovision(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.user_deprovision = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessIdentityProviderScimConfigEl {
    type O = terrars::BlockAssignable<AccessIdentityProviderScimConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessIdentityProviderScimConfigEl {}

impl BuildAccessIdentityProviderScimConfigEl {
    pub fn build(self) -> AccessIdentityProviderScimConfigEl {
        AccessIdentityProviderScimConfigEl {
            enabled: core::default::Default::default(),
            group_member_deprovision: core::default::Default::default(),
            seat_deprovision: core::default::Default::default(),
            secret: core::default::Default::default(),
            user_deprovision: core::default::Default::default(),
        }
    }
}

pub struct AccessIdentityProviderScimConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessIdentityProviderScimConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessIdentityProviderScimConfigElRef {
        AccessIdentityProviderScimConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessIdentityProviderScimConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `group_member_deprovision` after provisioning.\n"]
    pub fn group_member_deprovision(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_member_deprovision", self.base))
    }

    #[doc = "Get a reference to the value of field `seat_deprovision` after provisioning.\n"]
    pub fn seat_deprovision(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seat_deprovision", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc = "Get a reference to the value of field `user_deprovision` after provisioning.\n"]
    pub fn user_deprovision(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_deprovision", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessIdentityProviderDynamic {
    config: std::option::Option<terrars::DynamicBlock<AccessIdentityProviderConfigEl>>,
    scim_config: std::option::Option<terrars::DynamicBlock<AccessIdentityProviderScimConfigEl>>,
}
