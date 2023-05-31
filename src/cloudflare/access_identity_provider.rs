use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AccessIdentityProviderData {
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
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<AccessIdentityProviderConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scim_config: Option<Vec<AccessIdentityProviderScimConfigEl>>,
    dynamic: AccessIdentityProviderDynamic,
}

struct AccessIdentityProvider_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessIdentityProviderData>,
}

#[derive(Clone)]
pub struct AccessIdentityProvider(Rc<AccessIdentityProvider_>);

impl AccessIdentityProvider {
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

    #[doc= "Set the field `account_id`.\nThe account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `zone_id`.\nThe zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_zone_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone_id = Some(v.into());
        self
    }

    #[doc= "Set the field `config`.\n"]
    pub fn set_config(self, v: impl Into<BlockAssignable<AccessIdentityProviderConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `scim_config`.\n"]
    pub fn set_scim_config(self, v: impl Into<BlockAssignable<AccessIdentityProviderScimConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scim_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scim_config = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nFriendly name of the Access Identity Provider configuration."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe provider type to use. Available values: `centrify`, `facebook`, `google-apps`, `oidc`, `github`, `google`, `saml`, `linkedin`, `azureAD`, `okta`, `onetimepin`, `onelogin`, `yandex`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<AccessIdentityProviderConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scim_config` after provisioning.\n"]
    pub fn scim_config(&self) -> ListRef<AccessIdentityProviderScimConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scim_config", self.extract_ref()))
    }
}

impl Referable for AccessIdentityProvider {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AccessIdentityProvider { }

impl ToListMappable for AccessIdentityProvider {
    type O = ListRef<AccessIdentityProviderRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AccessIdentityProvider_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_access_identity_provider".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessIdentityProvider {
    pub tf_id: String,
    #[doc= "Friendly name of the Access Identity Provider configuration."]
    pub name: PrimField<String>,
    #[doc= "The provider type to use. Available values: `centrify`, `facebook`, `google-apps`, `oidc`, `github`, `google`, `saml`, `linkedin`, `azureAD`, `okta`, `onetimepin`, `onelogin`, `yandex`."]
    pub type_: PrimField<String>,
}

impl BuildAccessIdentityProvider {
    pub fn build(self, stack: &mut Stack) -> AccessIdentityProvider {
        let out = AccessIdentityProvider(Rc::new(AccessIdentityProvider_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AccessIdentityProviderData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                type_: self.type_,
                zone_id: core::default::Default::default(),
                config: core::default::Default::default(),
                scim_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessIdentityProviderRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessIdentityProviderRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessIdentityProviderRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nFriendly name of the Access Identity Provider configuration."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe provider type to use. Available values: `centrify`, `facebook`, `google-apps`, `oidc`, `github`, `google`, `saml`, `linkedin`, `azureAD`, `okta`, `onetimepin`, `onelogin`, `yandex`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<AccessIdentityProviderConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scim_config` after provisioning.\n"]
    pub fn scim_config(&self) -> ListRef<AccessIdentityProviderScimConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scim_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AccessIdentityProviderConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apps_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    centrify_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    centrify_app_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certs_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    claims: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    directory_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_attribute_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idp_public_cert: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    okta_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    onelogin_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pkce_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sign_request: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sso_target_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    support_groups: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_url: Option<PrimField<String>>,
}

impl AccessIdentityProviderConfigEl {
    #[doc= "Set the field `api_token`.\n"]
    pub fn set_api_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_token = Some(v.into());
        self
    }

    #[doc= "Set the field `apps_domain`.\n"]
    pub fn set_apps_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.apps_domain = Some(v.into());
        self
    }

    #[doc= "Set the field `attributes`.\n"]
    pub fn set_attributes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.attributes = Some(v.into());
        self
    }

    #[doc= "Set the field `auth_url`.\n"]
    pub fn set_auth_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_url = Some(v.into());
        self
    }

    #[doc= "Set the field `centrify_account`.\n"]
    pub fn set_centrify_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.centrify_account = Some(v.into());
        self
    }

    #[doc= "Set the field `centrify_app_id`.\n"]
    pub fn set_centrify_app_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.centrify_app_id = Some(v.into());
        self
    }

    #[doc= "Set the field `certs_url`.\n"]
    pub fn set_certs_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certs_url = Some(v.into());
        self
    }

    #[doc= "Set the field `claims`.\n"]
    pub fn set_claims(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.claims = Some(v.into());
        self
    }

    #[doc= "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }

    #[doc= "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret = Some(v.into());
        self
    }

    #[doc= "Set the field `directory_id`.\n"]
    pub fn set_directory_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.directory_id = Some(v.into());
        self
    }

    #[doc= "Set the field `email_attribute_name`.\n"]
    pub fn set_email_attribute_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email_attribute_name = Some(v.into());
        self
    }

    #[doc= "Set the field `idp_public_cert`.\n"]
    pub fn set_idp_public_cert(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.idp_public_cert = Some(v.into());
        self
    }

    #[doc= "Set the field `issuer_url`.\n"]
    pub fn set_issuer_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer_url = Some(v.into());
        self
    }

    #[doc= "Set the field `okta_account`.\n"]
    pub fn set_okta_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.okta_account = Some(v.into());
        self
    }

    #[doc= "Set the field `onelogin_account`.\n"]
    pub fn set_onelogin_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.onelogin_account = Some(v.into());
        self
    }

    #[doc= "Set the field `pkce_enabled`.\n"]
    pub fn set_pkce_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.pkce_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `redirect_url`.\n"]
    pub fn set_redirect_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redirect_url = Some(v.into());
        self
    }

    #[doc= "Set the field `scopes`.\n"]
    pub fn set_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scopes = Some(v.into());
        self
    }

    #[doc= "Set the field `sign_request`.\n"]
    pub fn set_sign_request(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.sign_request = Some(v.into());
        self
    }

    #[doc= "Set the field `sso_target_url`.\n"]
    pub fn set_sso_target_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sso_target_url = Some(v.into());
        self
    }

    #[doc= "Set the field `support_groups`.\n"]
    pub fn set_support_groups(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.support_groups = Some(v.into());
        self
    }

    #[doc= "Set the field `token_url`.\n"]
    pub fn set_token_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token_url = Some(v.into());
        self
    }
}

impl ToListMappable for AccessIdentityProviderConfigEl {
    type O = BlockAssignable<AccessIdentityProviderConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessIdentityProviderConfigElRef {
    fn new(shared: StackShared, base: String) -> AccessIdentityProviderConfigElRef {
        AccessIdentityProviderConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessIdentityProviderConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `api_token` after provisioning.\n"]
    pub fn api_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_token", self.base))
    }

    #[doc= "Get a reference to the value of field `apps_domain` after provisioning.\n"]
    pub fn apps_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.apps_domain", self.base))
    }

    #[doc= "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }

    #[doc= "Get a reference to the value of field `auth_url` after provisioning.\n"]
    pub fn auth_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_url", self.base))
    }

    #[doc= "Get a reference to the value of field `centrify_account` after provisioning.\n"]
    pub fn centrify_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.centrify_account", self.base))
    }

    #[doc= "Get a reference to the value of field `centrify_app_id` after provisioning.\n"]
    pub fn centrify_app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.centrify_app_id", self.base))
    }

    #[doc= "Get a reference to the value of field `certs_url` after provisioning.\n"]
    pub fn certs_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certs_url", self.base))
    }

    #[doc= "Get a reference to the value of field `claims` after provisioning.\n"]
    pub fn claims(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.claims", self.base))
    }

    #[doc= "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc= "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc= "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.base))
    }

    #[doc= "Get a reference to the value of field `email_attribute_name` after provisioning.\n"]
    pub fn email_attribute_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_attribute_name", self.base))
    }

    #[doc= "Get a reference to the value of field `idp_public_cert` after provisioning.\n"]
    pub fn idp_public_cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.idp_public_cert", self.base))
    }

    #[doc= "Get a reference to the value of field `issuer_url` after provisioning.\n"]
    pub fn issuer_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer_url", self.base))
    }

    #[doc= "Get a reference to the value of field `okta_account` after provisioning.\n"]
    pub fn okta_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.okta_account", self.base))
    }

    #[doc= "Get a reference to the value of field `onelogin_account` after provisioning.\n"]
    pub fn onelogin_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.onelogin_account", self.base))
    }

    #[doc= "Get a reference to the value of field `pkce_enabled` after provisioning.\n"]
    pub fn pkce_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pkce_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `redirect_url` after provisioning.\n"]
    pub fn redirect_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_url", self.base))
    }

    #[doc= "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc= "Get a reference to the value of field `sign_request` after provisioning.\n"]
    pub fn sign_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.sign_request", self.base))
    }

    #[doc= "Get a reference to the value of field `sso_target_url` after provisioning.\n"]
    pub fn sso_target_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sso_target_url", self.base))
    }

    #[doc= "Get a reference to the value of field `support_groups` after provisioning.\n"]
    pub fn support_groups(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.support_groups", self.base))
    }

    #[doc= "Get a reference to the value of field `token_url` after provisioning.\n"]
    pub fn token_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessIdentityProviderScimConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_member_deprovision: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seat_deprovision: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_deprovision: Option<PrimField<bool>>,
}

impl AccessIdentityProviderScimConfigEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `group_member_deprovision`.\n"]
    pub fn set_group_member_deprovision(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.group_member_deprovision = Some(v.into());
        self
    }

    #[doc= "Set the field `seat_deprovision`.\n"]
    pub fn set_seat_deprovision(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.seat_deprovision = Some(v.into());
        self
    }

    #[doc= "Set the field `secret`.\n"]
    pub fn set_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret = Some(v.into());
        self
    }

    #[doc= "Set the field `user_deprovision`.\n"]
    pub fn set_user_deprovision(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.user_deprovision = Some(v.into());
        self
    }
}

impl ToListMappable for AccessIdentityProviderScimConfigEl {
    type O = BlockAssignable<AccessIdentityProviderScimConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessIdentityProviderScimConfigElRef {
    fn new(shared: StackShared, base: String) -> AccessIdentityProviderScimConfigElRef {
        AccessIdentityProviderScimConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessIdentityProviderScimConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `group_member_deprovision` after provisioning.\n"]
    pub fn group_member_deprovision(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_member_deprovision", self.base))
    }

    #[doc= "Get a reference to the value of field `seat_deprovision` after provisioning.\n"]
    pub fn seat_deprovision(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.seat_deprovision", self.base))
    }

    #[doc= "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc= "Get a reference to the value of field `user_deprovision` after provisioning.\n"]
    pub fn user_deprovision(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_deprovision", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessIdentityProviderDynamic {
    config: Option<DynamicBlock<AccessIdentityProviderConfigEl>>,
    scim_config: Option<DynamicBlock<AccessIdentityProviderScimConfigEl>>,
}
