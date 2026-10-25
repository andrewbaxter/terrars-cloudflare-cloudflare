#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataEmailSecurityImpersonationRegistriesData_ {
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
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provenance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataEmailSecurityImpersonationRegistries_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEmailSecurityImpersonationRegistriesData_>,
}

#[derive(Clone)]
pub struct DataEmailSecurityImpersonationRegistries(std::rc::Rc<DataEmailSecurityImpersonationRegistries_>);

impl DataEmailSecurityImpersonationRegistries {
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

    #[doc = "Set the field `account_id`.\nIdentifier."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\nThe sorting direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `order`.\nField to sort by.\nAvailable values: \"name\", \"email\", \"created_at\"."]
    pub fn set_order(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `provenance`.\nAvailable values: \"A1S_INTERNAL\", \"SNOOPY-CASB_OFFICE_365\", \"SNOOPY-OFFICE_365\", \"SNOOPY-GOOGLE_DIRECTORY\"."]
    pub fn set_provenance(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().provenance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\nSearch term for filtering records. Behavior may change."]
    pub fn set_search(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().search = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe sorting direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to sort by.\nAvailable values: \"name\", \"email\", \"created_at\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provenance` after provisioning.\nAvailable values: \"A1S_INTERNAL\", \"SNOOPY-CASB_OFFICE_365\", \"SNOOPY-OFFICE_365\", \"SNOOPY-GOOGLE_DIRECTORY\"."]
    pub fn provenance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provenance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataEmailSecurityImpersonationRegistries_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nSearch term for filtering records. Behavior may change."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }
}

impl terrars::Referable for DataEmailSecurityImpersonationRegistries {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEmailSecurityImpersonationRegistries { }

impl terrars::ToListMappable for DataEmailSecurityImpersonationRegistries {
    type O = terrars::ListRef<DataEmailSecurityImpersonationRegistriesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEmailSecurityImpersonationRegistries_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_email_security_impersonation_registries".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEmailSecurityImpersonationRegistries {
    pub tf_id: std::string::String,
}

impl BuildDataEmailSecurityImpersonationRegistries {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEmailSecurityImpersonationRegistries {
        let out =
            DataEmailSecurityImpersonationRegistries(std::rc::Rc::new(DataEmailSecurityImpersonationRegistries_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(DataEmailSecurityImpersonationRegistriesData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    account_id: core::default::Default::default(),
                    direction: core::default::Default::default(),
                    max_items: core::default::Default::default(),
                    order: core::default::Default::default(),
                    provenance: core::default::Default::default(),
                    search: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEmailSecurityImpersonationRegistriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailSecurityImpersonationRegistriesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEmailSecurityImpersonationRegistriesRef {
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
        "Get a reference to the value of field `direction` after provisioning.\nThe sorting direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to sort by.\nAvailable values: \"name\", \"email\", \"created_at\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provenance` after provisioning.\nAvailable values: \"A1S_INTERNAL\", \"SNOOPY-CASB_OFFICE_365\", \"SNOOPY-OFFICE_365\", \"SNOOPY-GOOGLE_DIRECTORY\"."]
    pub fn provenance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provenance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataEmailSecurityImpersonationRegistries_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nSearch term for filtering records. Behavior may change."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEmailSecurityImpersonationRegistries_Result {}

impl DataEmailSecurityImpersonationRegistries_Result { }

impl terrars::ToListMappable for DataEmailSecurityImpersonationRegistries_Result {
    type O = terrars::BlockAssignable<DataEmailSecurityImpersonationRegistries_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmailSecurityImpersonationRegistries_Result {}

impl BuildDataEmailSecurityImpersonationRegistries_Result {
    pub fn build(self) -> DataEmailSecurityImpersonationRegistries_Result {
        DataEmailSecurityImpersonationRegistries_Result {}
    }
}

#[allow(dead_code)]
pub struct DataEmailSecurityImpersonationRegistries_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailSecurityImpersonationRegistries_ResultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEmailSecurityImpersonationRegistries_ResultRef {
        DataEmailSecurityImpersonationRegistries_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmailSecurityImpersonationRegistries_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comments` after provisioning.\nOptional note describing the entry."]
    pub fn comments(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comments", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `directory_id` after provisioning.\nIdentifier of the directory the entry was synced from, when directory-synced."]
    pub fn directory_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `directory_node_id` after provisioning.\nIdentifier of the directory node the entry was synced from, when directory-synced."]
    pub fn directory_node_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_node_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nEmail address (or pattern) of the protected identity."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `external_directory_node_id` after provisioning.\nDeprecated. External identifier of the directory node."]
    pub fn external_directory_node_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_directory_node_id", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nImpersonation registry entry identifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_email_regex` after provisioning.\nWhether `email` is a regular expression instead of a literal address."]
    pub fn is_email_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_email_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `last_modified` after provisioning.\nDeprecated, use `modified_at` instead. End of life: November 1, 2026."]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.base))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDisplay name of the protected identity."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provenance` after provisioning.\nSource the entry was created from.\nAvailable values: \"A1S_INTERNAL\", \"SNOOPY-CASB_OFFICE_365\", \"SNOOPY-OFFICE_365\", \"SNOOPY-GOOGLE_DIRECTORY\"."]
    pub fn provenance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provenance", self.base))
    }
}
