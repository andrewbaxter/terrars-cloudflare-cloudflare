#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustListData_ {
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
    filter: std::option::Option<DataZeroTrustList_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    list_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustList_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustListData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustList(std::rc::Rc<DataZeroTrustList_>);

impl DataZeroTrustList {
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

    #[doc = "Set the field `account_id`.\nSpecify the Cloudflare account identifier."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataZeroTrustList_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `list_id`.\nIdentify the API resource with a UUID."]
    pub fn set_list_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().list_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecify the Cloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nProvide the list description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustList_FilterRef {
        DataZeroTrustList_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentify the API resource with a UUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\nProvide the list items."]
    pub fn items(&self) -> terrars::SetRef<DataZeroTrustList_ItemsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_count` after provisioning.\nIndicate the number of items in the list."]
    pub fn list_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.list_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_id` after provisioning.\nIdentify the API resource with a UUID."]
    pub fn list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.list_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nSpecify the list name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nSpecify the list type.\nAvailable values: \"SERIAL\", \"URL\", \"DOMAIN\", \"EMAIL\", \"IP\", \"CATEGORY\", \"LOCATION\", \"DEVICE\", \"AAGUID\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustList {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustList { }

impl terrars::ToListMappable for DataZeroTrustList {
    type O = terrars::ListRef<DataZeroTrustListRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustList_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_list".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustList {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustList {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustList {
        let out = DataZeroTrustList(std::rc::Rc::new(DataZeroTrustList_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustListData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                list_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustListRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecify the Cloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nProvide the list description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustList_FilterRef {
        DataZeroTrustList_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentify the API resource with a UUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\nProvide the list items."]
    pub fn items(&self) -> terrars::SetRef<DataZeroTrustList_ItemsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_count` after provisioning.\nIndicate the number of items in the list."]
    pub fn list_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.list_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_id` after provisioning.\nIdentify the API resource with a UUID."]
    pub fn list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.list_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nSpecify the list name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nSpecify the list type.\nAvailable values: \"SERIAL\", \"URL\", \"DOMAIN\", \"EMAIL\", \"IP\", \"CATEGORY\", \"LOCATION\", \"DEVICE\", \"AAGUID\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustList_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataZeroTrustList_Filter {
    #[doc =
        "Set the field `direction`.\nSort direction. Applies to the field named in `order_by`; when `order_by`\nis omitted it applies to the default `created_at` ordering. When\n`direction` is omitted the default is field-specific: explicitly choosing\n`created_at` or `updated_at` defaults to descending (newest first); `name`\nand `item_count` default to ascending; and the default `created_at`\nordering used when `order_by` is omitted is ascending (for backwards\ncompatibility).\n  * `asc` — ascending.\n  * `desc` — descending.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter`.\nFilter the returned lists by one or more `field:value` pairs.\nRepeat the parameter to apply multiple filters; they are combined with\nlogical AND (a list must satisfy every filter to be returned).\n\nSupported fields and their matching behaviour:\n  * `name` — case-insensitive substring match on the list name.\n  * `id` — substring match on the list ID (UUID), with or without dashes.\n  * `type` — exact match on the list type. Supersedes the legacy `type` query\n    parameter when both are supplied. Must be one of the valid type values.\n  * `item_count` — exact integer match on the number of items in the list.\n\nEach entry must match one of the per-field patterns below: the field must be\none of `name`, `id`, `type`, or `item_count`; `name`/`id` accept any value,\n`type` is restricted to the valid list type values, and `item_count` must be\na non-negative integer."]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order_by`.\nField to sort the returned lists by. When omitted, results are ordered by\n`created_at` in ascending order (i.e. creation order) for backwards\ncompatibility. Supported values:\n  * `name` — sort alphabetically by list name.\n  * `created_at` — sort by creation time; defaults to descending unless `direction` is set.\n  * `updated_at` — sort by last-modified time; defaults to descending unless `direction` is set.\n  * `item_count` — sort by number of items in the list.\nAvailable values: \"name\", \"created_at\", \"updated_at\", \"item_count\"."]
    pub fn set_order_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order_by = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `search`.\nCase-insensitive substring match on the list name or description. When\ncombined with `filter`, both must match (logical AND)."]
    pub fn set_search(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.search = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nSpecify the list type.\nAvailable values: \"SERIAL\", \"URL\", \"DOMAIN\", \"EMAIL\", \"IP\", \"CATEGORY\", \"LOCATION\", \"DEVICE\", \"AAGUID\"."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataZeroTrustList_Filter {
    type O = terrars::BlockAssignable<DataZeroTrustList_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustList_Filter {}

impl BuildDataZeroTrustList_Filter {
    pub fn build(self) -> DataZeroTrustList_Filter {
        DataZeroTrustList_Filter {
            direction: core::default::Default::default(),
            filter: core::default::Default::default(),
            order_by: core::default::Default::default(),
            search: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustList_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustList_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustList_FilterRef {
        DataZeroTrustList_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustList_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nSort direction. Applies to the field named in `order_by`; when `order_by`\nis omitted it applies to the default `created_at` ordering. When\n`direction` is omitted the default is field-specific: explicitly choosing\n`created_at` or `updated_at` defaults to descending (newest first); `name`\nand `item_count` default to ascending; and the default `created_at`\nordering used when `order_by` is omitted is ascending (for backwards\ncompatibility).\n  * `asc` — ascending.\n  * `desc` — descending.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter the returned lists by one or more `field:value` pairs.\nRepeat the parameter to apply multiple filters; they are combined with\nlogical AND (a list must satisfy every filter to be returned).\n\nSupported fields and their matching behaviour:\n  * `name` — case-insensitive substring match on the list name.\n  * `id` — substring match on the list ID (UUID), with or without dashes.\n  * `type` — exact match on the list type. Supersedes the legacy `type` query\n    parameter when both are supplied. Must be one of the valid type values.\n  * `item_count` — exact integer match on the number of items in the list.\n\nEach entry must match one of the per-field patterns below: the field must be\none of `name`, `id`, `type`, or `item_count`; `name`/`id` accept any value,\n`type` is restricted to the valid list type values, and `item_count` must be\na non-negative integer."]
    pub fn filter(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order_by` after provisioning.\nField to sort the returned lists by. When omitted, results are ordered by\n`created_at` in ascending order (i.e. creation order) for backwards\ncompatibility. Supported values:\n  * `name` — sort alphabetically by list name.\n  * `created_at` — sort by creation time; defaults to descending unless `direction` is set.\n  * `updated_at` — sort by last-modified time; defaults to descending unless `direction` is set.\n  * `item_count` — sort by number of items in the list.\nAvailable values: \"name\", \"created_at\", \"updated_at\", \"item_count\"."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.base))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nCase-insensitive substring match on the list name or description. When\ncombined with `filter`, both must match (logical AND)."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nSpecify the list type.\nAvailable values: \"SERIAL\", \"URL\", \"DOMAIN\", \"EMAIL\", \"IP\", \"CATEGORY\", \"LOCATION\", \"DEVICE\", \"AAGUID\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustList_Items {}

impl DataZeroTrustList_Items { }

impl terrars::ToListMappable for DataZeroTrustList_Items {
    type O = terrars::BlockAssignable<DataZeroTrustList_Items>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustList_Items {}

impl BuildDataZeroTrustList_Items {
    pub fn build(self) -> DataZeroTrustList_Items {
        DataZeroTrustList_Items {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustList_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustList_ItemsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustList_ItemsRef {
        DataZeroTrustList_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustList_ItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nProvide the list item description (optional)."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nSpecify the item value."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
