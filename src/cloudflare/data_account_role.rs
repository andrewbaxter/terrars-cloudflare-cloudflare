#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataAccountRoleData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    role_id: terrars::PrimField<std::string::String>,
}

struct DataAccountRole_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAccountRoleData_>,
}

#[derive(Clone)]
pub struct DataAccountRole(std::rc::Rc<DataAccountRole_>);

impl DataAccountRole {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of role's permissions."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nRole identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRole name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> DataAccountRole_PermissionsRef {
        DataAccountRole_PermissionsRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_id` after provisioning.\nRole identifier tag."]
    pub fn role_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataAccountRole {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAccountRole { }

impl terrars::ToListMappable for DataAccountRole {
    type O = terrars::ListRef<DataAccountRoleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAccountRole_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_account_role".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccountRole {
    pub tf_id: std::string::String,
    #[doc = "Account identifier tag."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Role identifier tag."]
    pub role_id: terrars::PrimField<std::string::String>,
}

impl BuildDataAccountRole {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAccountRole {
        let out = DataAccountRole(std::rc::Rc::new(DataAccountRole_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAccountRoleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                role_id: self.role_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccountRoleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRoleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccountRoleRef {
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

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of role's permissions."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nRole identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRole name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> DataAccountRole_PermissionsRef {
        DataAccountRole_PermissionsRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_id` after provisioning.\nRole identifier tag."]
    pub fn role_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAccountRole_Permissions_Analytics {}

impl DataAccountRole_Permissions_Analytics { }

impl terrars::ToListMappable for DataAccountRole_Permissions_Analytics {
    type O = terrars::BlockAssignable<DataAccountRole_Permissions_Analytics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRole_Permissions_Analytics {}

impl BuildDataAccountRole_Permissions_Analytics {
    pub fn build(self) -> DataAccountRole_Permissions_Analytics {
        DataAccountRole_Permissions_Analytics {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRole_Permissions_AnalyticsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRole_Permissions_AnalyticsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRole_Permissions_AnalyticsRef {
        DataAccountRole_Permissions_AnalyticsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRole_Permissions_AnalyticsRef {
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
pub struct DataAccountRole_Permissions_Billing {}

impl DataAccountRole_Permissions_Billing { }

impl terrars::ToListMappable for DataAccountRole_Permissions_Billing {
    type O = terrars::BlockAssignable<DataAccountRole_Permissions_Billing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRole_Permissions_Billing {}

impl BuildDataAccountRole_Permissions_Billing {
    pub fn build(self) -> DataAccountRole_Permissions_Billing {
        DataAccountRole_Permissions_Billing {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRole_Permissions_BillingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRole_Permissions_BillingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRole_Permissions_BillingRef {
        DataAccountRole_Permissions_BillingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRole_Permissions_BillingRef {
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
pub struct DataAccountRole_Permissions_CachePurge {}

impl DataAccountRole_Permissions_CachePurge { }

impl terrars::ToListMappable for DataAccountRole_Permissions_CachePurge {
    type O = terrars::BlockAssignable<DataAccountRole_Permissions_CachePurge>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRole_Permissions_CachePurge {}

impl BuildDataAccountRole_Permissions_CachePurge {
    pub fn build(self) -> DataAccountRole_Permissions_CachePurge {
        DataAccountRole_Permissions_CachePurge {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRole_Permissions_CachePurgeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRole_Permissions_CachePurgeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRole_Permissions_CachePurgeRef {
        DataAccountRole_Permissions_CachePurgeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRole_Permissions_CachePurgeRef {
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
pub struct DataAccountRole_Permissions_Dns {}

impl DataAccountRole_Permissions_Dns { }

impl terrars::ToListMappable for DataAccountRole_Permissions_Dns {
    type O = terrars::BlockAssignable<DataAccountRole_Permissions_Dns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRole_Permissions_Dns {}

impl BuildDataAccountRole_Permissions_Dns {
    pub fn build(self) -> DataAccountRole_Permissions_Dns {
        DataAccountRole_Permissions_Dns {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRole_Permissions_DnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRole_Permissions_DnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRole_Permissions_DnsRef {
        DataAccountRole_Permissions_DnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRole_Permissions_DnsRef {
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
pub struct DataAccountRole_Permissions_DnsRecords {}

impl DataAccountRole_Permissions_DnsRecords { }

impl terrars::ToListMappable for DataAccountRole_Permissions_DnsRecords {
    type O = terrars::BlockAssignable<DataAccountRole_Permissions_DnsRecords>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRole_Permissions_DnsRecords {}

impl BuildDataAccountRole_Permissions_DnsRecords {
    pub fn build(self) -> DataAccountRole_Permissions_DnsRecords {
        DataAccountRole_Permissions_DnsRecords {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRole_Permissions_DnsRecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRole_Permissions_DnsRecordsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRole_Permissions_DnsRecordsRef {
        DataAccountRole_Permissions_DnsRecordsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRole_Permissions_DnsRecordsRef {
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
pub struct DataAccountRole_Permissions_Lb {}

impl DataAccountRole_Permissions_Lb { }

impl terrars::ToListMappable for DataAccountRole_Permissions_Lb {
    type O = terrars::BlockAssignable<DataAccountRole_Permissions_Lb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRole_Permissions_Lb {}

impl BuildDataAccountRole_Permissions_Lb {
    pub fn build(self) -> DataAccountRole_Permissions_Lb {
        DataAccountRole_Permissions_Lb {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRole_Permissions_LbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRole_Permissions_LbRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRole_Permissions_LbRef {
        DataAccountRole_Permissions_LbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRole_Permissions_LbRef {
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
pub struct DataAccountRole_Permissions_Logs {}

impl DataAccountRole_Permissions_Logs { }

impl terrars::ToListMappable for DataAccountRole_Permissions_Logs {
    type O = terrars::BlockAssignable<DataAccountRole_Permissions_Logs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRole_Permissions_Logs {}

impl BuildDataAccountRole_Permissions_Logs {
    pub fn build(self) -> DataAccountRole_Permissions_Logs {
        DataAccountRole_Permissions_Logs {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRole_Permissions_LogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRole_Permissions_LogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRole_Permissions_LogsRef {
        DataAccountRole_Permissions_LogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRole_Permissions_LogsRef {
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
pub struct DataAccountRole_Permissions_Organization {}

impl DataAccountRole_Permissions_Organization { }

impl terrars::ToListMappable for DataAccountRole_Permissions_Organization {
    type O = terrars::BlockAssignable<DataAccountRole_Permissions_Organization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRole_Permissions_Organization {}

impl BuildDataAccountRole_Permissions_Organization {
    pub fn build(self) -> DataAccountRole_Permissions_Organization {
        DataAccountRole_Permissions_Organization {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRole_Permissions_OrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRole_Permissions_OrganizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRole_Permissions_OrganizationRef {
        DataAccountRole_Permissions_OrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRole_Permissions_OrganizationRef {
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
pub struct DataAccountRole_Permissions_Ssl {}

impl DataAccountRole_Permissions_Ssl { }

impl terrars::ToListMappable for DataAccountRole_Permissions_Ssl {
    type O = terrars::BlockAssignable<DataAccountRole_Permissions_Ssl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRole_Permissions_Ssl {}

impl BuildDataAccountRole_Permissions_Ssl {
    pub fn build(self) -> DataAccountRole_Permissions_Ssl {
        DataAccountRole_Permissions_Ssl {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRole_Permissions_SslRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRole_Permissions_SslRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRole_Permissions_SslRef {
        DataAccountRole_Permissions_SslRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRole_Permissions_SslRef {
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
pub struct DataAccountRole_Permissions_Waf {}

impl DataAccountRole_Permissions_Waf { }

impl terrars::ToListMappable for DataAccountRole_Permissions_Waf {
    type O = terrars::BlockAssignable<DataAccountRole_Permissions_Waf>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRole_Permissions_Waf {}

impl BuildDataAccountRole_Permissions_Waf {
    pub fn build(self) -> DataAccountRole_Permissions_Waf {
        DataAccountRole_Permissions_Waf {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRole_Permissions_WafRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRole_Permissions_WafRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRole_Permissions_WafRef {
        DataAccountRole_Permissions_WafRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRole_Permissions_WafRef {
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
pub struct DataAccountRole_Permissions_ZoneSettings {}

impl DataAccountRole_Permissions_ZoneSettings { }

impl terrars::ToListMappable for DataAccountRole_Permissions_ZoneSettings {
    type O = terrars::BlockAssignable<DataAccountRole_Permissions_ZoneSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRole_Permissions_ZoneSettings {}

impl BuildDataAccountRole_Permissions_ZoneSettings {
    pub fn build(self) -> DataAccountRole_Permissions_ZoneSettings {
        DataAccountRole_Permissions_ZoneSettings {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRole_Permissions_ZoneSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRole_Permissions_ZoneSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRole_Permissions_ZoneSettingsRef {
        DataAccountRole_Permissions_ZoneSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRole_Permissions_ZoneSettingsRef {
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
pub struct DataAccountRole_Permissions_Zones {}

impl DataAccountRole_Permissions_Zones { }

impl terrars::ToListMappable for DataAccountRole_Permissions_Zones {
    type O = terrars::BlockAssignable<DataAccountRole_Permissions_Zones>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRole_Permissions_Zones {}

impl BuildDataAccountRole_Permissions_Zones {
    pub fn build(self) -> DataAccountRole_Permissions_Zones {
        DataAccountRole_Permissions_Zones {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRole_Permissions_ZonesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRole_Permissions_ZonesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRole_Permissions_ZonesRef {
        DataAccountRole_Permissions_ZonesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRole_Permissions_ZonesRef {
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
pub struct DataAccountRole_Permissions {}

impl DataAccountRole_Permissions { }

impl terrars::ToListMappable for DataAccountRole_Permissions {
    type O = terrars::BlockAssignable<DataAccountRole_Permissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRole_Permissions {}

impl BuildDataAccountRole_Permissions {
    pub fn build(self) -> DataAccountRole_Permissions {
        DataAccountRole_Permissions {}
    }
}

#[allow(dead_code)]
pub struct DataAccountRole_PermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRole_PermissionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRole_PermissionsRef {
        DataAccountRole_PermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRole_PermissionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `analytics` after provisioning.\n"]
    pub fn analytics(&self) -> DataAccountRole_Permissions_AnalyticsRef {
        DataAccountRole_Permissions_AnalyticsRef::new(self.shared().clone(), format!("{}.analytics", self.base))
    }

    #[doc = "Get a reference to the value of field `billing` after provisioning.\n"]
    pub fn billing(&self) -> DataAccountRole_Permissions_BillingRef {
        DataAccountRole_Permissions_BillingRef::new(self.shared().clone(), format!("{}.billing", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_purge` after provisioning.\n"]
    pub fn cache_purge(&self) -> DataAccountRole_Permissions_CachePurgeRef {
        DataAccountRole_Permissions_CachePurgeRef::new(self.shared().clone(), format!("{}.cache_purge", self.base))
    }

    #[doc = "Get a reference to the value of field `dns` after provisioning.\n"]
    pub fn dns(&self) -> DataAccountRole_Permissions_DnsRef {
        DataAccountRole_Permissions_DnsRef::new(self.shared().clone(), format!("{}.dns", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_records` after provisioning.\n"]
    pub fn dns_records(&self) -> DataAccountRole_Permissions_DnsRecordsRef {
        DataAccountRole_Permissions_DnsRecordsRef::new(self.shared().clone(), format!("{}.dns_records", self.base))
    }

    #[doc = "Get a reference to the value of field `lb` after provisioning.\n"]
    pub fn lb(&self) -> DataAccountRole_Permissions_LbRef {
        DataAccountRole_Permissions_LbRef::new(self.shared().clone(), format!("{}.lb", self.base))
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\n"]
    pub fn logs(&self) -> DataAccountRole_Permissions_LogsRef {
        DataAccountRole_Permissions_LogsRef::new(self.shared().clone(), format!("{}.logs", self.base))
    }

    #[doc = "Get a reference to the value of field `organization` after provisioning.\n"]
    pub fn organization(&self) -> DataAccountRole_Permissions_OrganizationRef {
        DataAccountRole_Permissions_OrganizationRef::new(
            self.shared().clone(),
            format!("{}.organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> DataAccountRole_Permissions_SslRef {
        DataAccountRole_Permissions_SslRef::new(self.shared().clone(), format!("{}.ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `waf` after provisioning.\n"]
    pub fn waf(&self) -> DataAccountRole_Permissions_WafRef {
        DataAccountRole_Permissions_WafRef::new(self.shared().clone(), format!("{}.waf", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_settings` after provisioning.\n"]
    pub fn zone_settings(&self) -> DataAccountRole_Permissions_ZoneSettingsRef {
        DataAccountRole_Permissions_ZoneSettingsRef::new(
            self.shared().clone(),
            format!("{}.zone_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `zones` after provisioning.\n"]
    pub fn zones(&self) -> DataAccountRole_Permissions_ZonesRef {
        DataAccountRole_Permissions_ZonesRef::new(self.shared().clone(), format!("{}.zones", self.base))
    }
}
