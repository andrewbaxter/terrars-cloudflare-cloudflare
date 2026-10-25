#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataLoadBalancerMonitorGroupData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    monitor_group_id: terrars::PrimField<std::string::String>,
}

struct DataLoadBalancerMonitorGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLoadBalancerMonitorGroupData_>,
}

#[derive(Clone)]
pub struct DataLoadBalancerMonitorGroup(std::rc::Rc<DataLoadBalancerMonitorGroup_>);

impl DataLoadBalancerMonitorGroup {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `members` after provisioning.\nList of monitors in this group"]
    pub fn members(&self) -> terrars::SetRef<DataLoadBalancerMonitorGroup_MembersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.members", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe timestamp of when the monitor group was last updated"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitor_group_id` after provisioning.\n"]
    pub fn monitor_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor_group_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataLoadBalancerMonitorGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLoadBalancerMonitorGroup { }

impl terrars::ToListMappable for DataLoadBalancerMonitorGroup {
    type O = terrars::ListRef<DataLoadBalancerMonitorGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLoadBalancerMonitorGroup_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_load_balancer_monitor_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLoadBalancerMonitorGroup {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub monitor_group_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLoadBalancerMonitorGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLoadBalancerMonitorGroup {
        let out = DataLoadBalancerMonitorGroup(std::rc::Rc::new(DataLoadBalancerMonitorGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLoadBalancerMonitorGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                monitor_group_id: self.monitor_group_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLoadBalancerMonitorGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerMonitorGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLoadBalancerMonitorGroupRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `members` after provisioning.\nList of monitors in this group"]
    pub fn members(&self) -> terrars::SetRef<DataLoadBalancerMonitorGroup_MembersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.members", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe timestamp of when the monitor group was last updated"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitor_group_id` after provisioning.\n"]
    pub fn monitor_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor_group_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancerMonitorGroup_Members {}

impl DataLoadBalancerMonitorGroup_Members { }

impl terrars::ToListMappable for DataLoadBalancerMonitorGroup_Members {
    type O = terrars::BlockAssignable<DataLoadBalancerMonitorGroup_Members>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancerMonitorGroup_Members {}

impl BuildDataLoadBalancerMonitorGroup_Members {
    pub fn build(self) -> DataLoadBalancerMonitorGroup_Members {
        DataLoadBalancerMonitorGroup_Members {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancerMonitorGroup_MembersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerMonitorGroup_MembersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadBalancerMonitorGroup_MembersRef {
        DataLoadBalancerMonitorGroup_MembersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerMonitorGroup_MembersRef {
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
