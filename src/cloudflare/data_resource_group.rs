#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataResourceGroupData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    resource_group_id: terrars::PrimField<std::string::String>,
}

struct DataResourceGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataResourceGroupData_>,
}

#[derive(Clone)]
pub struct DataResourceGroup(std::rc::Rc<DataResourceGroup_>);

impl DataResourceGroup {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier of the resource group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `meta` after provisioning.\nAttributes associated to the resource group."]
    pub fn meta(&self) -> DataResourceGroup_MetaRef {
        DataResourceGroup_MetaRef::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_group_id` after provisioning.\nResource Group identifier tag."]
    pub fn resource_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_group_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe scope associated to the resource group"]
    pub fn scope(&self) -> terrars::ListRef<DataResourceGroup_ScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }
}

impl terrars::Referable for DataResourceGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataResourceGroup { }

impl terrars::ToListMappable for DataResourceGroup {
    type O = terrars::ListRef<DataResourceGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataResourceGroup_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_resource_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataResourceGroup {
    pub tf_id: std::string::String,
    #[doc = "Account identifier tag."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Resource Group identifier tag."]
    pub resource_group_id: terrars::PrimField<std::string::String>,
}

impl BuildDataResourceGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> DataResourceGroup {
        let out = DataResourceGroup(std::rc::Rc::new(DataResourceGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataResourceGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                resource_group_id: self.resource_group_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataResourceGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResourceGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataResourceGroupRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier of the resource group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `meta` after provisioning.\nAttributes associated to the resource group."]
    pub fn meta(&self) -> DataResourceGroup_MetaRef {
        DataResourceGroup_MetaRef::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_group_id` after provisioning.\nResource Group identifier tag."]
    pub fn resource_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_group_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe scope associated to the resource group"]
    pub fn scope(&self) -> terrars::ListRef<DataResourceGroup_ScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataResourceGroup_Meta {}

impl DataResourceGroup_Meta { }

impl terrars::ToListMappable for DataResourceGroup_Meta {
    type O = terrars::BlockAssignable<DataResourceGroup_Meta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResourceGroup_Meta {}

impl BuildDataResourceGroup_Meta {
    pub fn build(self) -> DataResourceGroup_Meta {
        DataResourceGroup_Meta {}
    }
}

#[allow(dead_code)]
pub struct DataResourceGroup_MetaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResourceGroup_MetaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataResourceGroup_MetaRef {
        DataResourceGroup_MetaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResourceGroup_MetaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataResourceGroup_Scope_Objects {}

impl DataResourceGroup_Scope_Objects { }

impl terrars::ToListMappable for DataResourceGroup_Scope_Objects {
    type O = terrars::BlockAssignable<DataResourceGroup_Scope_Objects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResourceGroup_Scope_Objects {}

impl BuildDataResourceGroup_Scope_Objects {
    pub fn build(self) -> DataResourceGroup_Scope_Objects {
        DataResourceGroup_Scope_Objects {}
    }
}

#[allow(dead_code)]
pub struct DataResourceGroup_Scope_ObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResourceGroup_Scope_ObjectsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataResourceGroup_Scope_ObjectsRef {
        DataResourceGroup_Scope_ObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResourceGroup_Scope_ObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nThis is a combination of pre-defined resource name and identifier (like Zone ID etc.)"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataResourceGroup_Scope {}

impl DataResourceGroup_Scope { }

impl terrars::ToListMappable for DataResourceGroup_Scope {
    type O = terrars::BlockAssignable<DataResourceGroup_Scope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResourceGroup_Scope {}

impl BuildDataResourceGroup_Scope {
    pub fn build(self) -> DataResourceGroup_Scope {
        DataResourceGroup_Scope {}
    }
}

#[allow(dead_code)]
pub struct DataResourceGroup_ScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResourceGroup_ScopeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataResourceGroup_ScopeRef {
        DataResourceGroup_ScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResourceGroup_ScopeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nThis is a combination of pre-defined resource name and identifier (like Account ID etc.)"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `objects` after provisioning.\nA list of scope objects for additional context."]
    pub fn objects(&self) -> terrars::ListRef<DataResourceGroup_Scope_ObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.objects", self.base))
    }
}
