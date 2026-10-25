#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataAccountRolesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
}

struct DataAccountRoles_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAccountRolesData_>,
}

#[derive(Clone)]
pub struct DataAccountRoles(std::rc::Rc<DataAccountRoles_>);

impl DataAccountRoles {
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

    #[doc = "Set the field `account_id`.\nAccount identifier tag."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataAccountRoles_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

impl terrars::Referable for DataAccountRoles {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAccountRoles { }

impl terrars::ToListMappable for DataAccountRoles {
    type O = terrars::ListRef<DataAccountRolesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAccountRoles_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_account_roles".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccountRoles {
    pub tf_id: std::string::String,
}

impl BuildDataAccountRoles {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAccountRoles {
        let out = DataAccountRoles(std::rc::Rc::new(DataAccountRoles_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAccountRolesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccountRolesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRolesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccountRolesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataAccountRoles_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAccountRoles_Result_Permissions_Analytics {}

impl DataAccountRoles_Result_Permissions_Analytics { }

impl terrars::ToListMappable for DataAccountRoles_Result_Permissions_Analytics {
    type O = terrars::BlockAssignable<DataAccountRoles_Result_Permissions_Analytics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRoles_Result_Permissions_Analytics {}

impl BuildDataAccountRoles_Result_Permissions_Analytics {
    pub fn build(self) -> DataAccountRoles_Result_Permissions_Analytics {
        DataAccountRoles_Result_Permissions_Analytics {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRoles_Result_Permissions_AnalyticsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRoles_Result_Permissions_AnalyticsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountRoles_Result_Permissions_AnalyticsRef {
        DataAccountRoles_Result_Permissions_AnalyticsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRoles_Result_Permissions_AnalyticsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountRoles_Result_Permissions_Billing {}

impl DataAccountRoles_Result_Permissions_Billing { }

impl terrars::ToListMappable for DataAccountRoles_Result_Permissions_Billing {
    type O = terrars::BlockAssignable<DataAccountRoles_Result_Permissions_Billing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRoles_Result_Permissions_Billing {}

impl BuildDataAccountRoles_Result_Permissions_Billing {
    pub fn build(self) -> DataAccountRoles_Result_Permissions_Billing {
        DataAccountRoles_Result_Permissions_Billing {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRoles_Result_Permissions_BillingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRoles_Result_Permissions_BillingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountRoles_Result_Permissions_BillingRef {
        DataAccountRoles_Result_Permissions_BillingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRoles_Result_Permissions_BillingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountRoles_Result_Permissions_CachePurge {}

impl DataAccountRoles_Result_Permissions_CachePurge { }

impl terrars::ToListMappable for DataAccountRoles_Result_Permissions_CachePurge {
    type O = terrars::BlockAssignable<DataAccountRoles_Result_Permissions_CachePurge>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRoles_Result_Permissions_CachePurge {}

impl BuildDataAccountRoles_Result_Permissions_CachePurge {
    pub fn build(self) -> DataAccountRoles_Result_Permissions_CachePurge {
        DataAccountRoles_Result_Permissions_CachePurge {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRoles_Result_Permissions_CachePurgeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRoles_Result_Permissions_CachePurgeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountRoles_Result_Permissions_CachePurgeRef {
        DataAccountRoles_Result_Permissions_CachePurgeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRoles_Result_Permissions_CachePurgeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountRoles_Result_Permissions_Dns {}

impl DataAccountRoles_Result_Permissions_Dns { }

impl terrars::ToListMappable for DataAccountRoles_Result_Permissions_Dns {
    type O = terrars::BlockAssignable<DataAccountRoles_Result_Permissions_Dns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRoles_Result_Permissions_Dns {}

impl BuildDataAccountRoles_Result_Permissions_Dns {
    pub fn build(self) -> DataAccountRoles_Result_Permissions_Dns {
        DataAccountRoles_Result_Permissions_Dns {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRoles_Result_Permissions_DnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRoles_Result_Permissions_DnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRoles_Result_Permissions_DnsRef {
        DataAccountRoles_Result_Permissions_DnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRoles_Result_Permissions_DnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountRoles_Result_Permissions_DnsRecords {}

impl DataAccountRoles_Result_Permissions_DnsRecords { }

impl terrars::ToListMappable for DataAccountRoles_Result_Permissions_DnsRecords {
    type O = terrars::BlockAssignable<DataAccountRoles_Result_Permissions_DnsRecords>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRoles_Result_Permissions_DnsRecords {}

impl BuildDataAccountRoles_Result_Permissions_DnsRecords {
    pub fn build(self) -> DataAccountRoles_Result_Permissions_DnsRecords {
        DataAccountRoles_Result_Permissions_DnsRecords {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRoles_Result_Permissions_DnsRecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRoles_Result_Permissions_DnsRecordsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountRoles_Result_Permissions_DnsRecordsRef {
        DataAccountRoles_Result_Permissions_DnsRecordsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRoles_Result_Permissions_DnsRecordsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountRoles_Result_Permissions_Lb {}

impl DataAccountRoles_Result_Permissions_Lb { }

impl terrars::ToListMappable for DataAccountRoles_Result_Permissions_Lb {
    type O = terrars::BlockAssignable<DataAccountRoles_Result_Permissions_Lb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRoles_Result_Permissions_Lb {}

impl BuildDataAccountRoles_Result_Permissions_Lb {
    pub fn build(self) -> DataAccountRoles_Result_Permissions_Lb {
        DataAccountRoles_Result_Permissions_Lb {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRoles_Result_Permissions_LbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRoles_Result_Permissions_LbRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRoles_Result_Permissions_LbRef {
        DataAccountRoles_Result_Permissions_LbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRoles_Result_Permissions_LbRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountRoles_Result_Permissions_Logs {}

impl DataAccountRoles_Result_Permissions_Logs { }

impl terrars::ToListMappable for DataAccountRoles_Result_Permissions_Logs {
    type O = terrars::BlockAssignable<DataAccountRoles_Result_Permissions_Logs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRoles_Result_Permissions_Logs {}

impl BuildDataAccountRoles_Result_Permissions_Logs {
    pub fn build(self) -> DataAccountRoles_Result_Permissions_Logs {
        DataAccountRoles_Result_Permissions_Logs {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRoles_Result_Permissions_LogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRoles_Result_Permissions_LogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRoles_Result_Permissions_LogsRef {
        DataAccountRoles_Result_Permissions_LogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRoles_Result_Permissions_LogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountRoles_Result_Permissions_Organization {}

impl DataAccountRoles_Result_Permissions_Organization { }

impl terrars::ToListMappable for DataAccountRoles_Result_Permissions_Organization {
    type O = terrars::BlockAssignable<DataAccountRoles_Result_Permissions_Organization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRoles_Result_Permissions_Organization {}

impl BuildDataAccountRoles_Result_Permissions_Organization {
    pub fn build(self) -> DataAccountRoles_Result_Permissions_Organization {
        DataAccountRoles_Result_Permissions_Organization {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRoles_Result_Permissions_OrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRoles_Result_Permissions_OrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountRoles_Result_Permissions_OrganizationRef {
        DataAccountRoles_Result_Permissions_OrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRoles_Result_Permissions_OrganizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountRoles_Result_Permissions_Ssl {}

impl DataAccountRoles_Result_Permissions_Ssl { }

impl terrars::ToListMappable for DataAccountRoles_Result_Permissions_Ssl {
    type O = terrars::BlockAssignable<DataAccountRoles_Result_Permissions_Ssl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRoles_Result_Permissions_Ssl {}

impl BuildDataAccountRoles_Result_Permissions_Ssl {
    pub fn build(self) -> DataAccountRoles_Result_Permissions_Ssl {
        DataAccountRoles_Result_Permissions_Ssl {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRoles_Result_Permissions_SslRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRoles_Result_Permissions_SslRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRoles_Result_Permissions_SslRef {
        DataAccountRoles_Result_Permissions_SslRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRoles_Result_Permissions_SslRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountRoles_Result_Permissions_Waf {}

impl DataAccountRoles_Result_Permissions_Waf { }

impl terrars::ToListMappable for DataAccountRoles_Result_Permissions_Waf {
    type O = terrars::BlockAssignable<DataAccountRoles_Result_Permissions_Waf>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRoles_Result_Permissions_Waf {}

impl BuildDataAccountRoles_Result_Permissions_Waf {
    pub fn build(self) -> DataAccountRoles_Result_Permissions_Waf {
        DataAccountRoles_Result_Permissions_Waf {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRoles_Result_Permissions_WafRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRoles_Result_Permissions_WafRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRoles_Result_Permissions_WafRef {
        DataAccountRoles_Result_Permissions_WafRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRoles_Result_Permissions_WafRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountRoles_Result_Permissions_ZoneSettings {}

impl DataAccountRoles_Result_Permissions_ZoneSettings { }

impl terrars::ToListMappable for DataAccountRoles_Result_Permissions_ZoneSettings {
    type O = terrars::BlockAssignable<DataAccountRoles_Result_Permissions_ZoneSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRoles_Result_Permissions_ZoneSettings {}

impl BuildDataAccountRoles_Result_Permissions_ZoneSettings {
    pub fn build(self) -> DataAccountRoles_Result_Permissions_ZoneSettings {
        DataAccountRoles_Result_Permissions_ZoneSettings {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRoles_Result_Permissions_ZoneSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRoles_Result_Permissions_ZoneSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountRoles_Result_Permissions_ZoneSettingsRef {
        DataAccountRoles_Result_Permissions_ZoneSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRoles_Result_Permissions_ZoneSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountRoles_Result_Permissions_Zones {}

impl DataAccountRoles_Result_Permissions_Zones { }

impl terrars::ToListMappable for DataAccountRoles_Result_Permissions_Zones {
    type O = terrars::BlockAssignable<DataAccountRoles_Result_Permissions_Zones>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRoles_Result_Permissions_Zones {}

impl BuildDataAccountRoles_Result_Permissions_Zones {
    pub fn build(self) -> DataAccountRoles_Result_Permissions_Zones {
        DataAccountRoles_Result_Permissions_Zones {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRoles_Result_Permissions_ZonesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRoles_Result_Permissions_ZonesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRoles_Result_Permissions_ZonesRef {
        DataAccountRoles_Result_Permissions_ZonesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRoles_Result_Permissions_ZonesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountRoles_Result_Permissions {}

impl DataAccountRoles_Result_Permissions { }

impl terrars::ToListMappable for DataAccountRoles_Result_Permissions {
    type O = terrars::BlockAssignable<DataAccountRoles_Result_Permissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRoles_Result_Permissions {}

impl BuildDataAccountRoles_Result_Permissions {
    pub fn build(self) -> DataAccountRoles_Result_Permissions {
        DataAccountRoles_Result_Permissions {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRoles_Result_PermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRoles_Result_PermissionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRoles_Result_PermissionsRef {
        DataAccountRoles_Result_PermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRoles_Result_PermissionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `analytics` after provisioning.\n"]
    pub fn analytics(&self) -> DataAccountRoles_Result_Permissions_AnalyticsRef {
        DataAccountRoles_Result_Permissions_AnalyticsRef::new(
            self.shared().clone(),
            format!("{}.analytics", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `billing` after provisioning.\n"]
    pub fn billing(&self) -> DataAccountRoles_Result_Permissions_BillingRef {
        DataAccountRoles_Result_Permissions_BillingRef::new(self.shared().clone(), format!("{}.billing", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_purge` after provisioning.\n"]
    pub fn cache_purge(&self) -> DataAccountRoles_Result_Permissions_CachePurgeRef {
        DataAccountRoles_Result_Permissions_CachePurgeRef::new(
            self.shared().clone(),
            format!("{}.cache_purge", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dns` after provisioning.\n"]
    pub fn dns(&self) -> DataAccountRoles_Result_Permissions_DnsRef {
        DataAccountRoles_Result_Permissions_DnsRef::new(self.shared().clone(), format!("{}.dns", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_records` after provisioning.\n"]
    pub fn dns_records(&self) -> DataAccountRoles_Result_Permissions_DnsRecordsRef {
        DataAccountRoles_Result_Permissions_DnsRecordsRef::new(
            self.shared().clone(),
            format!("{}.dns_records", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lb` after provisioning.\n"]
    pub fn lb(&self) -> DataAccountRoles_Result_Permissions_LbRef {
        DataAccountRoles_Result_Permissions_LbRef::new(self.shared().clone(), format!("{}.lb", self.base))
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\n"]
    pub fn logs(&self) -> DataAccountRoles_Result_Permissions_LogsRef {
        DataAccountRoles_Result_Permissions_LogsRef::new(self.shared().clone(), format!("{}.logs", self.base))
    }

    #[doc = "Get a reference to the value of field `organization` after provisioning.\n"]
    pub fn organization(&self) -> DataAccountRoles_Result_Permissions_OrganizationRef {
        DataAccountRoles_Result_Permissions_OrganizationRef::new(
            self.shared().clone(),
            format!("{}.organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> DataAccountRoles_Result_Permissions_SslRef {
        DataAccountRoles_Result_Permissions_SslRef::new(self.shared().clone(), format!("{}.ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `waf` after provisioning.\n"]
    pub fn waf(&self) -> DataAccountRoles_Result_Permissions_WafRef {
        DataAccountRoles_Result_Permissions_WafRef::new(self.shared().clone(), format!("{}.waf", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_settings` after provisioning.\n"]
    pub fn zone_settings(&self) -> DataAccountRoles_Result_Permissions_ZoneSettingsRef {
        DataAccountRoles_Result_Permissions_ZoneSettingsRef::new(
            self.shared().clone(),
            format!("{}.zone_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `zones` after provisioning.\n"]
    pub fn zones(&self) -> DataAccountRoles_Result_Permissions_ZonesRef {
        DataAccountRoles_Result_Permissions_ZonesRef::new(self.shared().clone(), format!("{}.zones", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountRoles_Result {}

impl DataAccountRoles_Result { }

impl terrars::ToListMappable for DataAccountRoles_Result {
    type O = terrars::BlockAssignable<DataAccountRoles_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRoles_Result {}

impl BuildDataAccountRoles_Result {
    pub fn build(self) -> DataAccountRoles_Result {
        DataAccountRoles_Result {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRoles_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRoles_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRoles_ResultRef {
        DataAccountRoles_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRoles_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of role's permissions."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nRole identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRole name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> DataAccountRoles_Result_PermissionsRef {
        DataAccountRoles_Result_PermissionsRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
}
