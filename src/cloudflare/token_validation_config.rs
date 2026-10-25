#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct TokenValidationConfigData_ {
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
    credentials: TokenValidationConfig_Credentials,
    description: terrars::PrimField<std::string::String>,
    title: terrars::PrimField<std::string::String>,
    token_sources: terrars::ListField<terrars::PrimField<std::string::String>>,
    token_type: terrars::PrimField<std::string::String>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct TokenValidationConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TokenValidationConfigData_>,
}

#[derive(Clone)]
pub struct TokenValidationConfig(std::rc::Rc<TokenValidationConfig_>);

impl TokenValidationConfig {
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

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `credentials` after provisioning.\nRequest payload for create and PUT credentials operations. Provided keys define the complete stored key set. Key identities (`{alg,kid}`) must be unique."]
    pub fn credentials(&self) -> TokenValidationConfig_CredentialsRef {
        TokenValidationConfig_CredentialsRef::new(
            self.shared().clone(),
            format!("{}.credentials", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_sources` after provisioning.\n"]
    pub fn token_sources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_type` after provisioning.\nAvailable values: \"JWT\"."]
    pub fn token_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for TokenValidationConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TokenValidationConfig { }

impl terrars::ToListMappable for TokenValidationConfig {
    type O = terrars::ListRef<TokenValidationConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TokenValidationConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_token_validation_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTokenValidationConfig {
    pub tf_id: std::string::String,
    #[doc =
        "Request payload for create and PUT credentials operations. Provided keys define the complete stored key set. Key identities (`{alg,kid}`) must be unique."]
    pub credentials: TokenValidationConfig_Credentials,
    #[doc = ""]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub title: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub token_sources: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "Available values: \"JWT\"."]
    pub token_type: terrars::PrimField<std::string::String>,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildTokenValidationConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> TokenValidationConfig {
        let out = TokenValidationConfig(std::rc::Rc::new(TokenValidationConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TokenValidationConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                credentials: self.credentials,
                description: self.description,
                title: self.title,
                token_sources: self.token_sources,
                token_type: self.token_type,
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TokenValidationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TokenValidationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TokenValidationConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `credentials` after provisioning.\nRequest payload for create and PUT credentials operations. Provided keys define the complete stored key set. Key identities (`{alg,kid}`) must be unique."]
    pub fn credentials(&self) -> TokenValidationConfig_CredentialsRef {
        TokenValidationConfig_CredentialsRef::new(
            self.shared().clone(),
            format!("{}.credentials", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_sources` after provisioning.\n"]
    pub fn token_sources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_type` after provisioning.\nAvailable values: \"JWT\"."]
    pub fn token_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TokenValidationConfig_Credentials_Keys {
    alg: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crv: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    e: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    k: std::option::Option<terrars::PrimField<std::string::String>>,
    kid: terrars::PrimField<std::string::String>,
    kty: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    n: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    y: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TokenValidationConfig_Credentials_Keys {
    #[doc = "Set the field `crv`.\nCurve\nAvailable values: \"P-256\", \"P-384\"."]
    pub fn set_crv(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.crv = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `e`.\nRSA exponent"]
    pub fn set_e(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.e = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `k`.\nSymmetric key material. Required for create and PUT update requests."]
    pub fn set_k(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.k = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `n`.\nRSA modulus"]
    pub fn set_n(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.n = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `x`.\nX EC coordinate"]
    pub fn set_x(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.x = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `y`.\nY EC coordinate"]
    pub fn set_y(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.y = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TokenValidationConfig_Credentials_Keys {
    type O = terrars::BlockAssignable<TokenValidationConfig_Credentials_Keys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTokenValidationConfig_Credentials_Keys {
    #[doc =
        "Algorithm\nAvailable values: \"RS256\", \"RS384\", \"RS512\", \"PS256\", \"PS384\", \"PS512\", \"ES256\", \"ES384\", \"HS256\", \"HS384\", \"HS512\"."]
    pub alg: terrars::PrimField<std::string::String>,
    #[doc = "Key ID"]
    pub kid: terrars::PrimField<std::string::String>,
    #[doc = "Key Type\nAvailable values: \"RSA\", \"EC\", \"oct\"."]
    pub kty: terrars::PrimField<std::string::String>,
}

impl BuildTokenValidationConfig_Credentials_Keys {
    pub fn build(self) -> TokenValidationConfig_Credentials_Keys {
        TokenValidationConfig_Credentials_Keys {
            alg: self.alg,
            crv: core::default::Default::default(),
            e: core::default::Default::default(),
            k: core::default::Default::default(),
            kid: self.kid,
            kty: self.kty,
            n: core::default::Default::default(),
            x: core::default::Default::default(),
            y: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TokenValidationConfig_Credentials_KeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TokenValidationConfig_Credentials_KeysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TokenValidationConfig_Credentials_KeysRef {
        TokenValidationConfig_Credentials_KeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TokenValidationConfig_Credentials_KeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alg` after provisioning.\nAlgorithm\nAvailable values: \"RS256\", \"RS384\", \"RS512\", \"PS256\", \"PS384\", \"PS512\", \"ES256\", \"ES384\", \"HS256\", \"HS384\", \"HS512\"."]
    pub fn alg(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alg", self.base))
    }

    #[doc =
        "Get a reference to the value of field `crv` after provisioning.\nCurve\nAvailable values: \"P-256\", \"P-384\"."]
    pub fn crv(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crv", self.base))
    }

    #[doc = "Get a reference to the value of field `e` after provisioning.\nRSA exponent"]
    pub fn e(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.e", self.base))
    }

    #[doc =
        "Get a reference to the value of field `k` after provisioning.\nSymmetric key material. Required for create and PUT update requests."]
    pub fn k(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.k", self.base))
    }

    #[doc = "Get a reference to the value of field `kid` after provisioning.\nKey ID"]
    pub fn kid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kty` after provisioning.\nKey Type\nAvailable values: \"RSA\", \"EC\", \"oct\"."]
    pub fn kty(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kty", self.base))
    }

    #[doc = "Get a reference to the value of field `n` after provisioning.\nRSA modulus"]
    pub fn n(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.n", self.base))
    }

    #[doc = "Get a reference to the value of field `x` after provisioning.\nX EC coordinate"]
    pub fn x(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.x", self.base))
    }

    #[doc = "Get a reference to the value of field `y` after provisioning.\nY EC coordinate"]
    pub fn y(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.y", self.base))
    }
}

#[derive(Serialize)]
pub struct TokenValidationConfig_Credentials {
    keys: std::vec::Vec<TokenValidationConfig_Credentials_Keys>,
}

impl TokenValidationConfig_Credentials { }

impl terrars::ToListMappable for TokenValidationConfig_Credentials {
    type O = terrars::BlockAssignable<TokenValidationConfig_Credentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTokenValidationConfig_Credentials {
    #[doc = ""]
    pub keys: std::vec::Vec<TokenValidationConfig_Credentials_Keys>,
}

impl BuildTokenValidationConfig_Credentials {
    pub fn build(self) -> TokenValidationConfig_Credentials {
        TokenValidationConfig_Credentials { keys: self.keys }
    }
}

#[allow(dead_code)]
pub struct TokenValidationConfig_CredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TokenValidationConfig_CredentialsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TokenValidationConfig_CredentialsRef {
        TokenValidationConfig_CredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TokenValidationConfig_CredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `keys` after provisioning.\n"]
    pub fn keys(&self) -> terrars::ListRef<TokenValidationConfig_Credentials_KeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.keys", self.base))
    }
}
