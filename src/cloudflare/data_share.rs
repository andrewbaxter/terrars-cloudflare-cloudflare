#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataShareData_ {
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
    filter: std::option::Option<DataShare_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_recipient_counts: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_resources: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataShare_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataShareData_>,
}

#[derive(Clone)]
pub struct DataShare(std::rc::Rc<DataShare_>);

impl DataShare {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataShare_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_recipient_counts`.\nInclude recipient counts in the response."]
    pub fn set_include_recipient_counts(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_recipient_counts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_resources`.\nInclude resources in the response."]
    pub fn set_include_resources(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `share_id`.\nShare identifier tag."]
    pub fn set_share_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().share_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_name` after provisioning.\nThe display name of an account."]
    pub fn account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `associated_recipient_count` after provisioning.\nThe number of recipients in the 'associated' state. This field is only included when requested via the 'include_recipient_counts' parameter."]
    pub fn associated_recipient_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associated_recipient_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `associating_recipient_count` after provisioning.\nThe number of recipients in the 'associating' state. This field is only included when requested via the 'include_recipient_counts' parameter."]
    pub fn associating_recipient_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associating_recipient_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\nWhen the share was created."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disassociated_recipient_count` after provisioning.\nThe number of recipients in the 'disassociated' state. This field is only included when requested via the 'include_recipient_counts' parameter."]
    pub fn disassociated_recipient_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disassociated_recipient_count", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `disassociating_recipient_count` after provisioning.\nThe number of recipients in the 'disassociating' state. This field is only included when requested via the 'include_recipient_counts' parameter."]
    pub fn disassociating_recipient_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disassociating_recipient_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataShare_FilterRef {
        DataShare_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nShare identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_recipient_counts` after provisioning.\nInclude recipient counts in the response."]
    pub fn include_recipient_counts(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_recipient_counts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_resources` after provisioning.\nInclude resources in the response."]
    pub fn include_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_resources", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nAvailable values: \"sent\", \"received\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified` after provisioning.\nWhen the share was modified."]
    pub fn modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the share."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\nOrganization identifier."]
    pub fn organization_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resources that are part of the share. This field is only included when requested via the 'include_resources' parameter."]
    pub fn resources(&self) -> terrars::ListRef<DataShare_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `share_id` after provisioning.\nShare identifier tag."]
    pub fn share_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nAvailable values: \"active\", \"deleting\", \"deleted\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_type` after provisioning.\nAvailable values: \"account\", \"organization\"."]
    pub fn target_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_type", self.extract_ref()))
    }
}

impl terrars::Referable for DataShare {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataShare { }

impl terrars::ToListMappable for DataShare {
    type O = terrars::ListRef<DataShareRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataShare_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_share".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataShare {
    pub tf_id: std::string::String,
    #[doc = "Account identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildDataShare {
    pub fn build(self, stack: &mut terrars::Stack) -> DataShare {
        let out = DataShare(std::rc::Rc::new(DataShare_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataShareData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                filter: core::default::Default::default(),
                include_recipient_counts: core::default::Default::default(),
                include_resources: core::default::Default::default(),
                share_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataShareRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataShareRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataShareRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_name` after provisioning.\nThe display name of an account."]
    pub fn account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `associated_recipient_count` after provisioning.\nThe number of recipients in the 'associated' state. This field is only included when requested via the 'include_recipient_counts' parameter."]
    pub fn associated_recipient_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associated_recipient_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `associating_recipient_count` after provisioning.\nThe number of recipients in the 'associating' state. This field is only included when requested via the 'include_recipient_counts' parameter."]
    pub fn associating_recipient_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associating_recipient_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\nWhen the share was created."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disassociated_recipient_count` after provisioning.\nThe number of recipients in the 'disassociated' state. This field is only included when requested via the 'include_recipient_counts' parameter."]
    pub fn disassociated_recipient_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disassociated_recipient_count", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `disassociating_recipient_count` after provisioning.\nThe number of recipients in the 'disassociating' state. This field is only included when requested via the 'include_recipient_counts' parameter."]
    pub fn disassociating_recipient_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disassociating_recipient_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataShare_FilterRef {
        DataShare_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nShare identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_recipient_counts` after provisioning.\nInclude recipient counts in the response."]
    pub fn include_recipient_counts(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_recipient_counts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_resources` after provisioning.\nInclude resources in the response."]
    pub fn include_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_resources", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nAvailable values: \"sent\", \"received\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified` after provisioning.\nWhen the share was modified."]
    pub fn modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the share."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\nOrganization identifier."]
    pub fn organization_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resources that are part of the share. This field is only included when requested via the 'include_resources' parameter."]
    pub fn resources(&self) -> terrars::ListRef<DataShare_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `share_id` after provisioning.\nShare identifier tag."]
    pub fn share_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nAvailable values: \"active\", \"deleting\", \"deleted\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_type` after provisioning.\nAvailable values: \"account\", \"organization\"."]
    pub fn target_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataShare_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kind: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataShare_Filter {
    #[doc = "Set the field `direction`.\nDirection to sort objects.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kind`.\nFilter shares by kind.\nAvailable values: \"sent\", \"received\"."]
    pub fn set_kind(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kind = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order`.\nOrder shares by values in the given field.\nAvailable values: \"name\", \"created\"."]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_types`.\nFilter share resources by resource_types."]
    pub fn set_resource_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nFilter shares by status.\nAvailable values: \"active\", \"deleting\", \"deleted\"."]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag`.\nFilter shares by tag. Each value is either `key=value` (matches shares whose tags contain that key/value pair) or `key` alone (matches shares that have any value for that key). May be repeated; multiple `tag` parameters are ANDed together. Maximum 20 `tag` parameters per request."]
    pub fn set_tag(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_type`.\nFilter shares by target_type.\nAvailable values: \"account\", \"organization\"."]
    pub fn set_target_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataShare_Filter {
    type O = terrars::BlockAssignable<DataShare_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataShare_Filter {}

impl BuildDataShare_Filter {
    pub fn build(self) -> DataShare_Filter {
        DataShare_Filter {
            direction: core::default::Default::default(),
            kind: core::default::Default::default(),
            order: core::default::Default::default(),
            resource_types: core::default::Default::default(),
            status: core::default::Default::default(),
            tag: core::default::Default::default(),
            target_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataShare_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataShare_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataShare_FilterRef {
        DataShare_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataShare_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to sort objects.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nFilter shares by kind.\nAvailable values: \"sent\", \"received\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nOrder shares by values in the given field.\nAvailable values: \"name\", \"created\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_types` after provisioning.\nFilter share resources by resource_types."]
    pub fn resource_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nFilter shares by status.\nAvailable values: \"active\", \"deleting\", \"deleted\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nFilter shares by tag. Each value is either `key=value` (matches shares whose tags contain that key/value pair) or `key` alone (matches shares that have any value for that key). May be repeated; multiple `tag` parameters are ANDed together. Maximum 20 `tag` parameters per request."]
    pub fn tag(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_type` after provisioning.\nFilter shares by target_type.\nAvailable values: \"account\", \"organization\"."]
    pub fn target_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataShare_Resources {}

impl DataShare_Resources { }

impl terrars::ToListMappable for DataShare_Resources {
    type O = terrars::BlockAssignable<DataShare_Resources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataShare_Resources {}

impl BuildDataShare_Resources {
    pub fn build(self) -> DataShare_Resources {
        DataShare_Resources {}
    }
}

#[allow(dead_code)]
pub struct DataShare_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataShare_ResourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataShare_ResourcesRef {
        DataShare_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataShare_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\nWhen the share was created."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nShare Resource identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `meta` after provisioning.\nResource Metadata."]
    pub fn meta(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.meta", self.base))
    }

    #[doc = "Get a reference to the value of field `modified` after provisioning.\nWhen the share was modified."]
    pub fn modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_account_id` after provisioning.\nAccount identifier."]
    pub fn resource_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\nShare Resource identifier."]
    pub fn resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_type` after provisioning.\nResource Type.\nAvailable values: \"custom-ruleset\", \"gateway-policy\", \"gateway-destination-ip\", \"gateway-block-page-settings\", \"gateway-extended-email-matching\", \"idp-federation-grant\", \"trust-grant\"."]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_version` after provisioning.\nResource Version."]
    pub fn resource_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nResource Status.\nAvailable values: \"active\", \"deleting\", \"deleted\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
