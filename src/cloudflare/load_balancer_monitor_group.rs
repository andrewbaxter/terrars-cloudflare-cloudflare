#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct LoadBalancerMonitorGroupData_ {
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
    description: terrars::PrimField<std::string::String>,
    members: std::vec::Vec<LoadBalancerMonitorGroup_Members>,
}

struct LoadBalancerMonitorGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LoadBalancerMonitorGroupData_>,
}

#[derive(Clone)]
pub struct LoadBalancerMonitorGroup(std::rc::Rc<LoadBalancerMonitorGroup_>);

impl LoadBalancerMonitorGroup {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe timestamp of when the monitor group was created"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA short description of the monitor group"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of the Monitor Group to use for checking the health of origins within this pool."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `members` after provisioning.\nList of monitors in this group"]
    pub fn members(&self) -> terrars::SetRef<LoadBalancerMonitorGroup_MembersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.members", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe timestamp of when the monitor group was last updated"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }
}

impl terrars::Referable for LoadBalancerMonitorGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LoadBalancerMonitorGroup { }

impl terrars::ToListMappable for LoadBalancerMonitorGroup {
    type O = terrars::ListRef<LoadBalancerMonitorGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LoadBalancerMonitorGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_load_balancer_monitor_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLoadBalancerMonitorGroup {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "A short description of the monitor group"]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = "List of monitors in this group"]
    pub members: std::vec::Vec<LoadBalancerMonitorGroup_Members>,
}

impl BuildLoadBalancerMonitorGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> LoadBalancerMonitorGroup {
        let out = LoadBalancerMonitorGroup(std::rc::Rc::new(LoadBalancerMonitorGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LoadBalancerMonitorGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                description: self.description,
                members: self.members,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LoadBalancerMonitorGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerMonitorGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LoadBalancerMonitorGroupRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe timestamp of when the monitor group was created"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA short description of the monitor group"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of the Monitor Group to use for checking the health of origins within this pool."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `members` after provisioning.\nList of monitors in this group"]
    pub fn members(&self) -> terrars::SetRef<LoadBalancerMonitorGroup_MembersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.members", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe timestamp of when the monitor group was last updated"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerMonitorGroup_Members {
    enabled: terrars::PrimField<bool>,
    monitor_id: terrars::PrimField<std::string::String>,
    monitoring_only: terrars::PrimField<bool>,
    must_be_healthy: terrars::PrimField<bool>,
}

impl LoadBalancerMonitorGroup_Members { }

impl terrars::ToListMappable for LoadBalancerMonitorGroup_Members {
    type O = terrars::BlockAssignable<LoadBalancerMonitorGroup_Members>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerMonitorGroup_Members {
    #[doc = "Whether this monitor is enabled in the group"]
    pub enabled: terrars::PrimField<bool>,
    #[doc = "The ID of the Monitor to use for checking the health of origins within this pool."]
    pub monitor_id: terrars::PrimField<std::string::String>,
    #[doc = "Whether this monitor is used for monitoring only (does not affect pool health)"]
    pub monitoring_only: terrars::PrimField<bool>,
    #[doc = "Whether this monitor must be healthy for the pool to be considered healthy"]
    pub must_be_healthy: terrars::PrimField<bool>,
}

impl BuildLoadBalancerMonitorGroup_Members {
    pub fn build(self) -> LoadBalancerMonitorGroup_Members {
        LoadBalancerMonitorGroup_Members {
            enabled: self.enabled,
            monitor_id: self.monitor_id,
            monitoring_only: self.monitoring_only,
            must_be_healthy: self.must_be_healthy,
        }
    }
}

#[allow(dead_code)]
pub struct LoadBalancerMonitorGroup_MembersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerMonitorGroup_MembersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerMonitorGroup_MembersRef {
        LoadBalancerMonitorGroup_MembersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerMonitorGroup_MembersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe timestamp of when the monitor was added to the group"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether this monitor is enabled in the group"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `monitor_id` after provisioning.\nThe ID of the Monitor to use for checking the health of origins within this pool."]
    pub fn monitor_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `monitoring_only` after provisioning.\nWhether this monitor is used for monitoring only (does not affect pool health)"]
    pub fn monitoring_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `must_be_healthy` after provisioning.\nWhether this monitor must be healthy for the pool to be considered healthy"]
    pub fn must_be_healthy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.must_be_healthy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nThe timestamp of when the monitor group member was last updated"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}
