#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataDlsPrefixBindingData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    binding_id: terrars::PrimField<std::string::String>,
}

struct DataDlsPrefixBinding_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDlsPrefixBindingData_>,
}

#[derive(Clone)]
pub struct DataDlsPrefixBinding(std::rc::Rc<DataDlsPrefixBinding_>);

impl DataDlsPrefixBinding {
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

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nIdentifier of a Cloudflare account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `binding_id` after provisioning.\nUnique identifier for the prefix binding."]
    pub fn binding_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.binding_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\nThe CIDR that is bound."]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the prefix binding."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prefix_id` after provisioning.\nThe ID of the parent prefix."]
    pub fn prefix_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region_key` after provisioning.\nThe region key used for the binding."]
    pub fn region_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region_key", self.extract_ref()))
    }
}

impl terrars::Referable for DataDlsPrefixBinding {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDlsPrefixBinding { }

impl terrars::ToListMappable for DataDlsPrefixBinding {
    type O = terrars::ListRef<DataDlsPrefixBindingRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDlsPrefixBinding_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_dls_prefix_binding".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDlsPrefixBinding {
    pub tf_id: std::string::String,
    #[doc = "Identifier of a Cloudflare account."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Unique identifier for the prefix binding."]
    pub binding_id: terrars::PrimField<std::string::String>,
}

impl BuildDataDlsPrefixBinding {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDlsPrefixBinding {
        let out = DataDlsPrefixBinding(std::rc::Rc::new(DataDlsPrefixBinding_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDlsPrefixBindingData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                binding_id: self.binding_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDlsPrefixBindingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDlsPrefixBindingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDlsPrefixBindingRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nIdentifier of a Cloudflare account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `binding_id` after provisioning.\nUnique identifier for the prefix binding."]
    pub fn binding_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.binding_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\nThe CIDR that is bound."]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the prefix binding."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prefix_id` after provisioning.\nThe ID of the parent prefix."]
    pub fn prefix_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region_key` after provisioning.\nThe region key used for the binding."]
    pub fn region_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region_key", self.extract_ref()))
    }
}
