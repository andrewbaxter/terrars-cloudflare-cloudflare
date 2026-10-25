#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct HyperdriveConfigData_ {
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
    caching: std::option::Option<HyperdriveConfig_Caching>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mtls: std::option::Option<HyperdriveConfig_Mtls>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<HyperdriveConfig_Origin>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_connection_limit: std::option::Option<terrars::PrimField<f64>>,
}

struct HyperdriveConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<HyperdriveConfigData_>,
}

#[derive(Clone)]
pub struct HyperdriveConfig(std::rc::Rc<HyperdriveConfig_>);

impl HyperdriveConfig {
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

    #[doc = "Set the field `caching`.\n"]
    pub fn set_caching(self, v: impl std::convert::Into<HyperdriveConfig_Caching>) -> Self {
        self.0.data.borrow_mut().caching = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `integration`.\n"]
    pub fn set_integration(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().integration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mtls`.\nmTLS configuration for the origin connection. Cannot be used with VPC Service origins; TLS must be managed on the VPC Service."]
    pub fn set_mtls(self, v: impl std::convert::Into<HyperdriveConfig_Mtls>) -> Self {
        self.0.data.borrow_mut().mtls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin`.\nCombines database connection fields with exactly one supported network location."]
    pub fn set_origin(self, v: impl std::convert::Into<HyperdriveConfig_Origin>) -> Self {
        self.0.data.borrow_mut().origin = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin_connection_limit`.\nThe (soft) maximum number of connections the Hyperdrive is allowed to make to the origin database.\n\nMaximum allowed: 20 for free tier accounts, 100 for paid tier accounts.\nIf not specified, defaults to 20 for free tier and 60 for paid tier.\nCertain Cloudflare-managed origins may be permitted a higher limit.\nContact Cloudflare if you need a higher limit."]
    pub fn set_origin_connection_limit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().origin_connection_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nDefine configurations using a unique string identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `caching` after provisioning.\n"]
    pub fn caching(&self) -> HyperdriveConfig_CachingRef {
        HyperdriveConfig_CachingRef::new(self.shared().clone(), format!("{}.caching", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nDefines the creation time of the Hyperdrive configuration."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nDefine configurations using a unique string identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration` after provisioning.\n"]
    pub fn integration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nDefines the last modified time of the Hyperdrive configuration."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mtls` after provisioning.\nmTLS configuration for the origin connection. Cannot be used with VPC Service origins; TLS must be managed on the VPC Service."]
    pub fn mtls(&self) -> HyperdriveConfig_MtlsRef {
        HyperdriveConfig_MtlsRef::new(self.shared().clone(), format!("{}.mtls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Hyperdrive configuration. Used to identify the configuration in the Cloudflare dashboard and API."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin` after provisioning.\nCombines database connection fields with exactly one supported network location."]
    pub fn origin(&self) -> HyperdriveConfig_OriginRef {
        HyperdriveConfig_OriginRef::new(self.shared().clone(), format!("{}.origin", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_connection_limit` after provisioning.\nThe (soft) maximum number of connections the Hyperdrive is allowed to make to the origin database.\n\nMaximum allowed: 20 for free tier accounts, 100 for paid tier accounts.\nIf not specified, defaults to 20 for free tier and 60 for paid tier.\nCertain Cloudflare-managed origins may be permitted a higher limit.\nContact Cloudflare if you need a higher limit."]
    pub fn origin_connection_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_connection_limit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restarted_on` after provisioning.\nDefines the last time the Hyperdrive connection pool was explicitly restarted via the restart endpoint. Omitted if the pool has never been explicitly restarted."]
    pub fn restarted_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restarted_on", self.extract_ref()))
    }
}

impl terrars::Referable for HyperdriveConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for HyperdriveConfig { }

impl terrars::ToListMappable for HyperdriveConfig {
    type O = terrars::ListRef<HyperdriveConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for HyperdriveConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_hyperdrive_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildHyperdriveConfig {
    pub tf_id: std::string::String,
    #[doc = "Define configurations using a unique string identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the Hyperdrive configuration. Used to identify the configuration in the Cloudflare dashboard and API."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildHyperdriveConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> HyperdriveConfig {
        let out = HyperdriveConfig(std::rc::Rc::new(HyperdriveConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(HyperdriveConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                caching: core::default::Default::default(),
                integration: core::default::Default::default(),
                mtls: core::default::Default::default(),
                name: self.name,
                origin: core::default::Default::default(),
                origin_connection_limit: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct HyperdriveConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HyperdriveConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl HyperdriveConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nDefine configurations using a unique string identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `caching` after provisioning.\n"]
    pub fn caching(&self) -> HyperdriveConfig_CachingRef {
        HyperdriveConfig_CachingRef::new(self.shared().clone(), format!("{}.caching", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nDefines the creation time of the Hyperdrive configuration."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nDefine configurations using a unique string identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration` after provisioning.\n"]
    pub fn integration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nDefines the last modified time of the Hyperdrive configuration."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mtls` after provisioning.\nmTLS configuration for the origin connection. Cannot be used with VPC Service origins; TLS must be managed on the VPC Service."]
    pub fn mtls(&self) -> HyperdriveConfig_MtlsRef {
        HyperdriveConfig_MtlsRef::new(self.shared().clone(), format!("{}.mtls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Hyperdrive configuration. Used to identify the configuration in the Cloudflare dashboard and API."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin` after provisioning.\nCombines database connection fields with exactly one supported network location."]
    pub fn origin(&self) -> HyperdriveConfig_OriginRef {
        HyperdriveConfig_OriginRef::new(self.shared().clone(), format!("{}.origin", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_connection_limit` after provisioning.\nThe (soft) maximum number of connections the Hyperdrive is allowed to make to the origin database.\n\nMaximum allowed: 20 for free tier accounts, 100 for paid tier accounts.\nIf not specified, defaults to 20 for free tier and 60 for paid tier.\nCertain Cloudflare-managed origins may be permitted a higher limit.\nContact Cloudflare if you need a higher limit."]
    pub fn origin_connection_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_connection_limit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restarted_on` after provisioning.\nDefines the last time the Hyperdrive connection pool was explicitly restarted via the restart endpoint. Omitted if the pool has never been explicitly restarted."]
    pub fn restarted_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restarted_on", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct HyperdriveConfig_Caching {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stale_while_revalidate: std::option::Option<terrars::PrimField<f64>>,
}

impl HyperdriveConfig_Caching {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age`.\n"]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stale_while_revalidate`.\n"]
    pub fn set_stale_while_revalidate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.stale_while_revalidate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HyperdriveConfig_Caching {
    type O = terrars::BlockAssignable<HyperdriveConfig_Caching>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHyperdriveConfig_Caching {}

impl BuildHyperdriveConfig_Caching {
    pub fn build(self) -> HyperdriveConfig_Caching {
        HyperdriveConfig_Caching {
            disabled: core::default::Default::default(),
            max_age: core::default::Default::default(),
            stale_while_revalidate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HyperdriveConfig_CachingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HyperdriveConfig_CachingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HyperdriveConfig_CachingRef {
        HyperdriveConfig_CachingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HyperdriveConfig_CachingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc = "Get a reference to the value of field `stale_while_revalidate` after provisioning.\n"]
    pub fn stale_while_revalidate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stale_while_revalidate", self.base))
    }
}

#[derive(Serialize)]
pub struct HyperdriveConfig_Mtls {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certificate_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mtls_certificate_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sslmode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl HyperdriveConfig_Mtls {
    #[doc = "Set the field `ca_certificate_id`.\nDefine CA certificate ID obtained after uploading CA cert."]
    pub fn set_ca_certificate_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ca_certificate_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mtls_certificate_id`.\nDefine mTLS certificate ID obtained after uploading client cert."]
    pub fn set_mtls_certificate_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.mtls_certificate_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sslmode`.\nPostgreSQL accepts `require`, `verify-ca`, and `verify-full`. MySQL accepts `REQUIRED`, `VERIFY_CA`, and `VERIFY_IDENTITY`. The verify modes require a CA certificate; the require modes cannot be used with a CA certificate."]
    pub fn set_sslmode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sslmode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HyperdriveConfig_Mtls {
    type O = terrars::BlockAssignable<HyperdriveConfig_Mtls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHyperdriveConfig_Mtls {}

impl BuildHyperdriveConfig_Mtls {
    pub fn build(self) -> HyperdriveConfig_Mtls {
        HyperdriveConfig_Mtls {
            ca_certificate_id: core::default::Default::default(),
            mtls_certificate_id: core::default::Default::default(),
            sslmode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HyperdriveConfig_MtlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HyperdriveConfig_MtlsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HyperdriveConfig_MtlsRef {
        HyperdriveConfig_MtlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HyperdriveConfig_MtlsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ca_certificate_id` after provisioning.\nDefine CA certificate ID obtained after uploading CA cert."]
    pub fn ca_certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mtls_certificate_id` after provisioning.\nDefine mTLS certificate ID obtained after uploading client cert."]
    pub fn mtls_certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mtls_certificate_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sslmode` after provisioning.\nPostgreSQL accepts `require`, `verify-ca`, and `verify-full`. MySQL accepts `REQUIRED`, `VERIFY_CA`, and `VERIFY_IDENTITY`. The verify modes require a CA certificate; the require modes cannot be used with a CA certificate."]
    pub fn sslmode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sslmode", self.base))
    }
}

#[derive(Serialize)]
pub struct HyperdriveConfig_Origin {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    database: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    password: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    scheme: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_id: std::option::Option<terrars::PrimField<std::string::String>>,
    user: terrars::PrimField<std::string::String>,
}

impl HyperdriveConfig_Origin {
    #[doc =
        "Set the field `access_client_id`.\nDefines the Client ID of the Access token to use when connecting to the origin database."]
    pub fn set_access_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `access_client_secret`.\nDefines the Client Secret of the Access Token to use when connecting to the origin database. The API never returns this write-only value."]
    pub fn set_access_client_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.access_client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `host`.\nDefines the publicly reachable hostname or IP of your origin database. Private, loopback, and link-local IP addresses are not allowed."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nDefines the port of your origin database. Defaults to 5432 for PostgreSQL or 3306 for MySQL if not specified."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_id`.\nThe identifier of the Workers VPC Service to connect through. Hyperdrive will egress through the specified VPC Service to reach the origin database."]
    pub fn set_service_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HyperdriveConfig_Origin {
    type O = terrars::BlockAssignable<HyperdriveConfig_Origin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHyperdriveConfig_Origin {
    #[doc = "Set the name of your origin database."]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = "Set the password needed to access your origin database. The API never returns this write-only value."]
    pub password: terrars::PrimField<std::string::String>,
    #[doc =
        "Specifies the URL scheme used to connect to your origin database.\nAvailable values: \"postgres\", \"postgresql\", \"mysql\"."]
    pub scheme: terrars::PrimField<std::string::String>,
    #[doc = "Set the user of your origin database."]
    pub user: terrars::PrimField<std::string::String>,
}

impl BuildHyperdriveConfig_Origin {
    pub fn build(self) -> HyperdriveConfig_Origin {
        HyperdriveConfig_Origin {
            access_client_id: core::default::Default::default(),
            access_client_secret: core::default::Default::default(),
            database: self.database,
            host: core::default::Default::default(),
            password: self.password,
            port: core::default::Default::default(),
            scheme: self.scheme,
            service_id: core::default::Default::default(),
            user: self.user,
        }
    }
}

#[allow(dead_code)]
pub struct HyperdriveConfig_OriginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HyperdriveConfig_OriginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HyperdriveConfig_OriginRef {
        HyperdriveConfig_OriginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HyperdriveConfig_OriginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_client_id` after provisioning.\nDefines the Client ID of the Access token to use when connecting to the origin database."]
    pub fn access_client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `access_client_secret` after provisioning.\nDefines the Client Secret of the Access Token to use when connecting to the origin database. The API never returns this write-only value."]
    pub fn access_client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nSet the name of your origin database."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nDefines the publicly reachable hostname or IP of your origin database. Private, loopback, and link-local IP addresses are not allowed."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nSet the password needed to access your origin database. The API never returns this write-only value."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nDefines the port of your origin database. Defaults to 5432 for PostgreSQL or 3306 for MySQL if not specified."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scheme` after provisioning.\nSpecifies the URL scheme used to connect to your origin database.\nAvailable values: \"postgres\", \"postgresql\", \"mysql\"."]
    pub fn scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_id` after provisioning.\nThe identifier of the Workers VPC Service to connect through. Hyperdrive will egress through the specified VPC Service to reach the origin database."]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\nSet the user of your origin database."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}
