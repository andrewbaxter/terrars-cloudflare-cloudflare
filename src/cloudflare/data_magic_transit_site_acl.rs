#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataMagicTransitSiteAclData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    acl_id: terrars::PrimField<std::string::String>,
    site_id: terrars::PrimField<std::string::String>,
}

struct DataMagicTransitSiteAcl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMagicTransitSiteAclData_>,
}

#[derive(Clone)]
pub struct DataMagicTransitSiteAcl(std::rc::Rc<DataMagicTransitSiteAcl_>);

impl DataMagicTransitSiteAcl {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `acl_id` after provisioning.\nIdentifier"]
    pub fn acl_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription for the ACL."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `forward_locally` after provisioning.\nThe desired forwarding action for this ACL policy. If set to \"false\", the policy will forward traffic to Cloudflare. If set to \"true\", the policy will forward traffic locally on the Magic Connector. If not included in request, will default to false."]
    pub fn forward_locally(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forward_locally", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lan_1` after provisioning.\n"]
    pub fn lan_1(&self) -> DataMagicTransitSiteAcl_Lan1Ref {
        DataMagicTransitSiteAcl_Lan1Ref::new(self.shared().clone(), format!("{}.lan_1", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lan_2` after provisioning.\n"]
    pub fn lan_2(&self) -> DataMagicTransitSiteAcl_Lan2Ref {
        DataMagicTransitSiteAcl_Lan2Ref::new(self.shared().clone(), format!("{}.lan_2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the ACL."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocols", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unidirectional` after provisioning.\nThe desired traffic direction for this ACL policy. If set to \"false\", the policy will allow bidirectional traffic. If set to \"true\", the policy will only allow traffic in one direction. If not included in request, will default to false."]
    pub fn unidirectional(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unidirectional", self.extract_ref()))
    }
}

impl terrars::Referable for DataMagicTransitSiteAcl {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMagicTransitSiteAcl { }

impl terrars::ToListMappable for DataMagicTransitSiteAcl {
    type O = terrars::ListRef<DataMagicTransitSiteAclRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMagicTransitSiteAcl_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_magic_transit_site_acl".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMagicTransitSiteAcl {
    pub tf_id: std::string::String,
    #[doc = "Identifier"]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Identifier"]
    pub acl_id: terrars::PrimField<std::string::String>,
    #[doc = "Identifier"]
    pub site_id: terrars::PrimField<std::string::String>,
}

impl BuildDataMagicTransitSiteAcl {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMagicTransitSiteAcl {
        let out = DataMagicTransitSiteAcl(std::rc::Rc::new(DataMagicTransitSiteAcl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMagicTransitSiteAclData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                acl_id: self.acl_id,
                site_id: self.site_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMagicTransitSiteAclRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteAclRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMagicTransitSiteAclRef {
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

    #[doc = "Get a reference to the value of field `acl_id` after provisioning.\nIdentifier"]
    pub fn acl_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription for the ACL."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `forward_locally` after provisioning.\nThe desired forwarding action for this ACL policy. If set to \"false\", the policy will forward traffic to Cloudflare. If set to \"true\", the policy will forward traffic locally on the Magic Connector. If not included in request, will default to false."]
    pub fn forward_locally(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forward_locally", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lan_1` after provisioning.\n"]
    pub fn lan_1(&self) -> DataMagicTransitSiteAcl_Lan1Ref {
        DataMagicTransitSiteAcl_Lan1Ref::new(self.shared().clone(), format!("{}.lan_1", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lan_2` after provisioning.\n"]
    pub fn lan_2(&self) -> DataMagicTransitSiteAcl_Lan2Ref {
        DataMagicTransitSiteAcl_Lan2Ref::new(self.shared().clone(), format!("{}.lan_2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the ACL."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocols", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unidirectional` after provisioning.\nThe desired traffic direction for this ACL policy. If set to \"false\", the policy will allow bidirectional traffic. If set to \"true\", the policy will only allow traffic in one direction. If not included in request, will default to false."]
    pub fn unidirectional(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unidirectional", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMagicTransitSiteAcl_Lan1 {}

impl DataMagicTransitSiteAcl_Lan1 { }

impl terrars::ToListMappable for DataMagicTransitSiteAcl_Lan1 {
    type O = terrars::BlockAssignable<DataMagicTransitSiteAcl_Lan1>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitSiteAcl_Lan1 {}

impl BuildDataMagicTransitSiteAcl_Lan1 {
    pub fn build(self) -> DataMagicTransitSiteAcl_Lan1 {
        DataMagicTransitSiteAcl_Lan1 {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitSiteAcl_Lan1Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteAcl_Lan1Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicTransitSiteAcl_Lan1Ref {
        DataMagicTransitSiteAcl_Lan1Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitSiteAcl_Lan1Ref {
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
pub struct DataMagicTransitSiteAcl_Lan2 {}

impl DataMagicTransitSiteAcl_Lan2 { }

impl terrars::ToListMappable for DataMagicTransitSiteAcl_Lan2 {
    type O = terrars::BlockAssignable<DataMagicTransitSiteAcl_Lan2>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitSiteAcl_Lan2 {}

impl BuildDataMagicTransitSiteAcl_Lan2 {
    pub fn build(self) -> DataMagicTransitSiteAcl_Lan2 {
        DataMagicTransitSiteAcl_Lan2 {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitSiteAcl_Lan2Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteAcl_Lan2Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicTransitSiteAcl_Lan2Ref {
        DataMagicTransitSiteAcl_Lan2Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitSiteAcl_Lan2Ref {
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
