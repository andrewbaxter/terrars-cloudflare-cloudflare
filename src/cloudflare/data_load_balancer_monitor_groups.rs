#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataLoadBalancerMonitorGroupsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
}

struct DataLoadBalancerMonitorGroups_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLoadBalancerMonitorGroupsData_>,
}

#[derive(Clone)]
pub struct DataLoadBalancerMonitorGroups(std::rc::Rc<DataLoadBalancerMonitorGroups_>);

impl DataLoadBalancerMonitorGroups {
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

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataLoadBalancerMonitorGroups_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

impl terrars::Referable for DataLoadBalancerMonitorGroups {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLoadBalancerMonitorGroups { }

impl terrars::ToListMappable for DataLoadBalancerMonitorGroups {
    type O = terrars::ListRef<DataLoadBalancerMonitorGroupsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLoadBalancerMonitorGroups_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_load_balancer_monitor_groups".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLoadBalancerMonitorGroups {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLoadBalancerMonitorGroups {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLoadBalancerMonitorGroups {
        let out = DataLoadBalancerMonitorGroups(std::rc::Rc::new(DataLoadBalancerMonitorGroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLoadBalancerMonitorGroupsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                max_items: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLoadBalancerMonitorGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerMonitorGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLoadBalancerMonitorGroupsRef {
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

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataLoadBalancerMonitorGroups_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancerMonitorGroups_Result_Members {}

impl DataLoadBalancerMonitorGroups_Result_Members { }

impl terrars::ToListMappable for DataLoadBalancerMonitorGroups_Result_Members {
    type O = terrars::BlockAssignable<DataLoadBalancerMonitorGroups_Result_Members>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancerMonitorGroups_Result_Members {}

impl BuildDataLoadBalancerMonitorGroups_Result_Members {
    pub fn build(self) -> DataLoadBalancerMonitorGroups_Result_Members {
        DataLoadBalancerMonitorGroups_Result_Members {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancerMonitorGroups_Result_MembersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerMonitorGroups_Result_MembersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLoadBalancerMonitorGroups_Result_MembersRef {
        DataLoadBalancerMonitorGroups_Result_MembersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerMonitorGroups_Result_MembersRef {
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

#[derive(Serialize)]
pub struct DataLoadBalancerMonitorGroups_Result {}

impl DataLoadBalancerMonitorGroups_Result { }

impl terrars::ToListMappable for DataLoadBalancerMonitorGroups_Result {
    type O = terrars::BlockAssignable<DataLoadBalancerMonitorGroups_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancerMonitorGroups_Result {}

impl BuildDataLoadBalancerMonitorGroups_Result {
    pub fn build(self) -> DataLoadBalancerMonitorGroups_Result {
        DataLoadBalancerMonitorGroups_Result {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancerMonitorGroups_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerMonitorGroups_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadBalancerMonitorGroups_ResultRef {
        DataLoadBalancerMonitorGroups_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerMonitorGroups_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe timestamp of when the monitor group was created"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA short description of the monitor group"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of the Monitor Group to use for checking the health of origins within this pool."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `members` after provisioning.\nList of monitors in this group"]
    pub fn members(&self) -> terrars::SetRef<DataLoadBalancerMonitorGroups_Result_MembersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.members", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe timestamp of when the monitor group was last updated"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }
}
