#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustDeviceDefaultProfileLocalDomainFallbackData_ {
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
    domains: std::vec::Vec<ZeroTrustDeviceDefaultProfileLocalDomainFallback_Domains>,
}

struct ZeroTrustDeviceDefaultProfileLocalDomainFallback_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustDeviceDefaultProfileLocalDomainFallbackData_>,
}

#[derive(Clone)]
pub struct ZeroTrustDeviceDefaultProfileLocalDomainFallback(
    std::rc::Rc<ZeroTrustDeviceDefaultProfileLocalDomainFallback_>,
);

impl ZeroTrustDeviceDefaultProfileLocalDomainFallback {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> terrars::SetRef<ZeroTrustDeviceDefaultProfileLocalDomainFallback_DomainsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustDeviceDefaultProfileLocalDomainFallback {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustDeviceDefaultProfileLocalDomainFallback { }

impl terrars::ToListMappable for ZeroTrustDeviceDefaultProfileLocalDomainFallback {
    type O = terrars::ListRef<ZeroTrustDeviceDefaultProfileLocalDomainFallbackRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustDeviceDefaultProfileLocalDomainFallback_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_device_default_profile_local_domain_fallback".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustDeviceDefaultProfileLocalDomainFallback {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub domains: std::vec::Vec<ZeroTrustDeviceDefaultProfileLocalDomainFallback_Domains>,
}

impl BuildZeroTrustDeviceDefaultProfileLocalDomainFallback {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustDeviceDefaultProfileLocalDomainFallback {
        let out =
            ZeroTrustDeviceDefaultProfileLocalDomainFallback(
                std::rc::Rc::new(ZeroTrustDeviceDefaultProfileLocalDomainFallback_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(ZeroTrustDeviceDefaultProfileLocalDomainFallbackData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        provisioner: core::default::Default::default(),
                        account_id: self.account_id,
                        domains: self.domains,
                    }),
                }),
            );
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustDeviceDefaultProfileLocalDomainFallbackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDeviceDefaultProfileLocalDomainFallbackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustDeviceDefaultProfileLocalDomainFallbackRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> terrars::SetRef<ZeroTrustDeviceDefaultProfileLocalDomainFallback_DomainsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDeviceDefaultProfileLocalDomainFallback_Domains {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_server: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    suffix: terrars::PrimField<std::string::String>,
}

impl ZeroTrustDeviceDefaultProfileLocalDomainFallback_Domains {
    #[doc = "Set the field `description`.\nA description of the fallback domain, displayed in the client UI."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_server`.\nA list of IP addresses to handle domain resolution."]
    pub fn set_dns_server(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dns_server = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDeviceDefaultProfileLocalDomainFallback_Domains {
    type O = terrars::BlockAssignable<ZeroTrustDeviceDefaultProfileLocalDomainFallback_Domains>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDeviceDefaultProfileLocalDomainFallback_Domains {
    #[doc = "The domain suffix to match when resolving locally."]
    pub suffix: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDeviceDefaultProfileLocalDomainFallback_Domains {
    pub fn build(self) -> ZeroTrustDeviceDefaultProfileLocalDomainFallback_Domains {
        ZeroTrustDeviceDefaultProfileLocalDomainFallback_Domains {
            description: core::default::Default::default(),
            dns_server: core::default::Default::default(),
            suffix: self.suffix,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDeviceDefaultProfileLocalDomainFallback_DomainsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDeviceDefaultProfileLocalDomainFallback_DomainsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustDeviceDefaultProfileLocalDomainFallback_DomainsRef {
        ZeroTrustDeviceDefaultProfileLocalDomainFallback_DomainsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDeviceDefaultProfileLocalDomainFallback_DomainsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the fallback domain, displayed in the client UI."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dns_server` after provisioning.\nA list of IP addresses to handle domain resolution."]
    pub fn dns_server(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_server", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suffix` after provisioning.\nThe domain suffix to match when resolving locally."]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
