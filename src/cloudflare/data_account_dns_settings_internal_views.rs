#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataAccountDnsSettingsInternalViewsData_ {
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
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<DataAccountDnsSettingsInternalViews_Name>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataAccountDnsSettingsInternalViews_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAccountDnsSettingsInternalViewsData_>,
}

#[derive(Clone)]
pub struct DataAccountDnsSettingsInternalViews(std::rc::Rc<DataAccountDnsSettingsInternalViews_>);

impl DataAccountDnsSettingsInternalViews {
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

    #[doc = "Set the field `direction`.\nDirection to order DNS views in.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `match_`.\nWhether to match all search requirements or at least one (any). If set to `all`, acts like a logical AND between filters. If set to `any`, acts like a logical OR instead.\nAvailable values: \"any\", \"all\"."]
    pub fn set_match(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<DataAccountDnsSettingsInternalViews_Name>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order`.\nField to order DNS views by.\nAvailable values: \"name\", \"created_on\", \"modified_on\"."]
    pub fn set_order(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nA zone ID that exists in the zones list for the view."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_name`.\nA zone name that exists in the zones list for the view."]
    pub fn set_zone_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order DNS views in.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nWhether to match all search requirements or at least one (any). If set to `all`, acts like a logical AND between filters. If set to `any`, acts like a logical OR instead.\nAvailable values: \"any\", \"all\"."]
    pub fn match_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> DataAccountDnsSettingsInternalViews_NameRef {
        DataAccountDnsSettingsInternalViews_NameRef::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to order DNS views by.\nAvailable values: \"name\", \"created_on\", \"modified_on\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataAccountDnsSettingsInternalViews_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nA zone ID that exists in the zones list for the view."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_name` after provisioning.\nA zone name that exists in the zones list for the view."]
    pub fn zone_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_name", self.extract_ref()))
    }
}

impl terrars::Referable for DataAccountDnsSettingsInternalViews {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAccountDnsSettingsInternalViews { }

impl terrars::ToListMappable for DataAccountDnsSettingsInternalViews {
    type O = terrars::ListRef<DataAccountDnsSettingsInternalViewsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAccountDnsSettingsInternalViews_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_account_dns_settings_internal_views".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccountDnsSettingsInternalViews {
    pub tf_id: std::string::String,
}

impl BuildDataAccountDnsSettingsInternalViews {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAccountDnsSettingsInternalViews {
        let out = DataAccountDnsSettingsInternalViews(std::rc::Rc::new(DataAccountDnsSettingsInternalViews_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAccountDnsSettingsInternalViewsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                direction: core::default::Default::default(),
                match_: core::default::Default::default(),
                max_items: core::default::Default::default(),
                name: core::default::Default::default(),
                order: core::default::Default::default(),
                zone_id: core::default::Default::default(),
                zone_name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccountDnsSettingsInternalViewsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountDnsSettingsInternalViewsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccountDnsSettingsInternalViewsRef {
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
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order DNS views in.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nWhether to match all search requirements or at least one (any). If set to `all`, acts like a logical AND between filters. If set to `any`, acts like a logical OR instead.\nAvailable values: \"any\", \"all\"."]
    pub fn match_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> DataAccountDnsSettingsInternalViews_NameRef {
        DataAccountDnsSettingsInternalViews_NameRef::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to order DNS views by.\nAvailable values: \"name\", \"created_on\", \"modified_on\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataAccountDnsSettingsInternalViews_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nA zone ID that exists in the zones list for the view."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_name` after provisioning.\nA zone name that exists in the zones list for the view."]
    pub fn zone_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAccountDnsSettingsInternalViews_Name {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endswith: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startswith: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAccountDnsSettingsInternalViews_Name {
    #[doc = "Set the field `contains`.\nSubstring of the DNS view name."]
    pub fn set_contains(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endswith`.\nSuffix of the DNS view name."]
    pub fn set_endswith(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endswith = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exact`.\nExact value of the DNS view name."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `startswith`.\nPrefix of the DNS view name."]
    pub fn set_startswith(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.startswith = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAccountDnsSettingsInternalViews_Name {
    type O = terrars::BlockAssignable<DataAccountDnsSettingsInternalViews_Name>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountDnsSettingsInternalViews_Name {}

impl BuildDataAccountDnsSettingsInternalViews_Name {
    pub fn build(self) -> DataAccountDnsSettingsInternalViews_Name {
        DataAccountDnsSettingsInternalViews_Name {
            contains: core::default::Default::default(),
            endswith: core::default::Default::default(),
            exact: core::default::Default::default(),
            startswith: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAccountDnsSettingsInternalViews_NameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountDnsSettingsInternalViews_NameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountDnsSettingsInternalViews_NameRef {
        DataAccountDnsSettingsInternalViews_NameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountDnsSettingsInternalViews_NameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contains` after provisioning.\nSubstring of the DNS view name."]
    pub fn contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc = "Get a reference to the value of field `endswith` after provisioning.\nSuffix of the DNS view name."]
    pub fn endswith(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endswith", self.base))
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\nExact value of the DNS view name."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `startswith` after provisioning.\nPrefix of the DNS view name."]
    pub fn startswith(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.startswith", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountDnsSettingsInternalViews_Result {}

impl DataAccountDnsSettingsInternalViews_Result { }

impl terrars::ToListMappable for DataAccountDnsSettingsInternalViews_Result {
    type O = terrars::BlockAssignable<DataAccountDnsSettingsInternalViews_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountDnsSettingsInternalViews_Result {}

impl BuildDataAccountDnsSettingsInternalViews_Result {
    pub fn build(self) -> DataAccountDnsSettingsInternalViews_Result {
        DataAccountDnsSettingsInternalViews_Result {}
    }
}

#[allow(dead_code)]
pub struct DataAccountDnsSettingsInternalViews_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountDnsSettingsInternalViews_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountDnsSettingsInternalViews_ResultRef {
        DataAccountDnsSettingsInternalViews_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountDnsSettingsInternalViews_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\nWhen the view was created."]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modified_time` after provisioning.\nWhen the view was last modified."]
    pub fn modified_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_time", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the view."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `zones` after provisioning.\nThe list of zones linked to this view."]
    pub fn zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.zones", self.base))
    }
}
