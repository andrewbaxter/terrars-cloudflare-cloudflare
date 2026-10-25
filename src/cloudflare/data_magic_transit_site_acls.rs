#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataMagicTransitSiteAclsData_ {
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
    site_id: terrars::PrimField<std::string::String>,
}

struct DataMagicTransitSiteAcls_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMagicTransitSiteAclsData_>,
}

#[derive(Clone)]
pub struct DataMagicTransitSiteAcls(std::rc::Rc<DataMagicTransitSiteAcls_>);

impl DataMagicTransitSiteAcls {
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

    #[doc = "Set the field `account_id`.\nIdentifier"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataMagicTransitSiteAcls_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataMagicTransitSiteAcls {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMagicTransitSiteAcls { }

impl terrars::ToListMappable for DataMagicTransitSiteAcls {
    type O = terrars::ListRef<DataMagicTransitSiteAclsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMagicTransitSiteAcls_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_magic_transit_site_acls".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMagicTransitSiteAcls {
    pub tf_id: std::string::String,
    #[doc = "Identifier"]
    pub site_id: terrars::PrimField<std::string::String>,
}

impl BuildDataMagicTransitSiteAcls {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMagicTransitSiteAcls {
        let out = DataMagicTransitSiteAcls(std::rc::Rc::new(DataMagicTransitSiteAcls_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMagicTransitSiteAclsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                site_id: self.site_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMagicTransitSiteAclsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteAclsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMagicTransitSiteAclsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataMagicTransitSiteAcls_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMagicTransitSiteAcls_Result_Lan1 {}

impl DataMagicTransitSiteAcls_Result_Lan1 { }

impl terrars::ToListMappable for DataMagicTransitSiteAcls_Result_Lan1 {
    type O = terrars::BlockAssignable<DataMagicTransitSiteAcls_Result_Lan1>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitSiteAcls_Result_Lan1 {}

impl BuildDataMagicTransitSiteAcls_Result_Lan1 {
    pub fn build(self) -> DataMagicTransitSiteAcls_Result_Lan1 {
        DataMagicTransitSiteAcls_Result_Lan1 {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitSiteAcls_Result_Lan1Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteAcls_Result_Lan1Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicTransitSiteAcls_Result_Lan1Ref {
        DataMagicTransitSiteAcls_Result_Lan1Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitSiteAcls_Result_Lan1Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `lan_id` after provisioning.\nThe identifier for the LAN you want to create an ACL policy with."]
    pub fn lan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lan_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lan_name` after provisioning.\nThe name of the LAN based on the provided lan_id."]
    pub fn lan_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lan_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_ranges` after provisioning.\nArray of port ranges on the provided LAN that will be included in the ACL. If no ports or port rangess are provided, communication on any port on this LAN is allowed."]
    pub fn port_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ports` after provisioning.\nArray of ports on the provided LAN that will be included in the ACL. If no ports or port ranges are provided, communication on any port on this LAN is allowed."]
    pub fn ports(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnets` after provisioning.\nArray of subnet IPs within the LAN that will be included in the ACL. If no subnets are provided, communication on any subnets on this LAN are allowed."]
    pub fn subnets(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicTransitSiteAcls_Result_Lan2 {}

impl DataMagicTransitSiteAcls_Result_Lan2 { }

impl terrars::ToListMappable for DataMagicTransitSiteAcls_Result_Lan2 {
    type O = terrars::BlockAssignable<DataMagicTransitSiteAcls_Result_Lan2>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitSiteAcls_Result_Lan2 {}

impl BuildDataMagicTransitSiteAcls_Result_Lan2 {
    pub fn build(self) -> DataMagicTransitSiteAcls_Result_Lan2 {
        DataMagicTransitSiteAcls_Result_Lan2 {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitSiteAcls_Result_Lan2Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteAcls_Result_Lan2Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicTransitSiteAcls_Result_Lan2Ref {
        DataMagicTransitSiteAcls_Result_Lan2Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitSiteAcls_Result_Lan2Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `lan_id` after provisioning.\nThe identifier for the LAN you want to create an ACL policy with."]
    pub fn lan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lan_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lan_name` after provisioning.\nThe name of the LAN based on the provided lan_id."]
    pub fn lan_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lan_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_ranges` after provisioning.\nArray of port ranges on the provided LAN that will be included in the ACL. If no ports or port rangess are provided, communication on any port on this LAN is allowed."]
    pub fn port_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ports` after provisioning.\nArray of ports on the provided LAN that will be included in the ACL. If no ports or port ranges are provided, communication on any port on this LAN is allowed."]
    pub fn ports(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnets` after provisioning.\nArray of subnet IPs within the LAN that will be included in the ACL. If no subnets are provided, communication on any subnets on this LAN are allowed."]
    pub fn subnets(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicTransitSiteAcls_Result {}

impl DataMagicTransitSiteAcls_Result { }

impl terrars::ToListMappable for DataMagicTransitSiteAcls_Result {
    type O = terrars::BlockAssignable<DataMagicTransitSiteAcls_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitSiteAcls_Result {}

impl BuildDataMagicTransitSiteAcls_Result {
    pub fn build(self) -> DataMagicTransitSiteAcls_Result {
        DataMagicTransitSiteAcls_Result {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitSiteAcls_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteAcls_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicTransitSiteAcls_ResultRef {
        DataMagicTransitSiteAcls_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitSiteAcls_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription for the ACL."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `forward_locally` after provisioning.\nThe desired forwarding action for this ACL policy. If set to \"false\", the policy will forward traffic to Cloudflare. If set to \"true\", the policy will forward traffic locally on the Magic Connector. If not included in request, will default to false."]
    pub fn forward_locally(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forward_locally", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `lan_1` after provisioning.\n"]
    pub fn lan_1(&self) -> DataMagicTransitSiteAcls_Result_Lan1Ref {
        DataMagicTransitSiteAcls_Result_Lan1Ref::new(self.shared().clone(), format!("{}.lan_1", self.base))
    }

    #[doc = "Get a reference to the value of field `lan_2` after provisioning.\n"]
    pub fn lan_2(&self) -> DataMagicTransitSiteAcls_Result_Lan2Ref {
        DataMagicTransitSiteAcls_Result_Lan2Ref::new(self.shared().clone(), format!("{}.lan_2", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the ACL."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocols", self.base))
    }

    #[doc =
        "Get a reference to the value of field `unidirectional` after provisioning.\nThe desired traffic direction for this ACL policy. If set to \"false\", the policy will allow bidirectional traffic. If set to \"true\", the policy will only allow traffic in one direction. If not included in request, will default to false."]
    pub fn unidirectional(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unidirectional", self.base))
    }
}
