#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataShareRecipientsData_ {
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
    include_resources: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    share_id: terrars::PrimField<std::string::String>,
}

struct DataShareRecipients_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataShareRecipientsData_>,
}

#[derive(Clone)]
pub struct DataShareRecipients(std::rc::Rc<DataShareRecipients_>);

impl DataShareRecipients {
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

    #[doc = "Set the field `include_resources`.\nInclude resources in the response."]
    pub fn set_include_resources(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_resources` after provisioning.\nInclude resources in the response."]
    pub fn include_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataShareRecipients_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `share_id` after provisioning.\nShare identifier tag."]
    pub fn share_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataShareRecipients {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataShareRecipients { }

impl terrars::ToListMappable for DataShareRecipients {
    type O = terrars::ListRef<DataShareRecipientsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataShareRecipients_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_share_recipients".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataShareRecipients {
    pub tf_id: std::string::String,
    #[doc = "Account identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Share identifier tag."]
    pub share_id: terrars::PrimField<std::string::String>,
}

impl BuildDataShareRecipients {
    pub fn build(self, stack: &mut terrars::Stack) -> DataShareRecipients {
        let out = DataShareRecipients(std::rc::Rc::new(DataShareRecipients_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataShareRecipientsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                include_resources: core::default::Default::default(),
                max_items: core::default::Default::default(),
                share_id: self.share_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataShareRecipientsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataShareRecipientsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataShareRecipientsRef {
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

    #[doc =
        "Get a reference to the value of field `include_resources` after provisioning.\nInclude resources in the response."]
    pub fn include_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataShareRecipients_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `share_id` after provisioning.\nShare identifier tag."]
    pub fn share_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataShareRecipients_Result_Resources {}

impl DataShareRecipients_Result_Resources { }

impl terrars::ToListMappable for DataShareRecipients_Result_Resources {
    type O = terrars::BlockAssignable<DataShareRecipients_Result_Resources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataShareRecipients_Result_Resources {}

impl BuildDataShareRecipients_Result_Resources {
    pub fn build(self) -> DataShareRecipients_Result_Resources {
        DataShareRecipients_Result_Resources {}
    }
}

#[allow(dead_code)]
pub struct DataShareRecipients_Result_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataShareRecipients_Result_ResourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataShareRecipients_Result_ResourcesRef {
        DataShareRecipients_Result_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataShareRecipients_Result_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\nShare Recipient error message."]
    pub fn error(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\nShare Resource identifier."]
    pub fn resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_version` after provisioning.\nResource Version."]
    pub fn resource_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `terminal` after provisioning.\nWhether the error is terminal or will be continually retried."]
    pub fn terminal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.terminal", self.base))
    }
}

#[derive(Serialize)]
pub struct DataShareRecipients_Result {}

impl DataShareRecipients_Result { }

impl terrars::ToListMappable for DataShareRecipients_Result {
    type O = terrars::BlockAssignable<DataShareRecipients_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataShareRecipients_Result {}

impl BuildDataShareRecipients_Result {
    pub fn build(self) -> DataShareRecipients_Result {
        DataShareRecipients_Result {}
    }
}

#[allow(dead_code)]
pub struct DataShareRecipients_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataShareRecipients_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataShareRecipients_ResultRef {
        DataShareRecipients_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataShareRecipients_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `association_status` after provisioning.\nThe current state of the recipient relative to the share. The\n`desired_association_status` (not exposed in the response) tracks the\ntarget state set by the API; the background reconciliation workflow\ndrives `current_association_status` toward it.\n\n- `associating` — The recipient was recently added; the workflow is\n  pushing shared resources into the recipient account.\n- `associated` — Shared resources have been successfully applied to\n  the recipient account.\n- `disassociating` — The recipient was removed (via DELETE or PUT\n  replacement); the workflow is removing shared resources from the\n  recipient account.\n- `disassociated` — Shared resources have been removed from the\n  recipient account. The recipient record remains in the database.\nAvailable values: \"associating\", \"associated\", \"disassociating\", \"disassociated\"."]
    pub fn association_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.association_status", self.base))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\nWhen the share was created."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nShare Recipient identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `modified` after provisioning.\nWhen the share was modified."]
    pub fn modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<DataShareRecipients_Result_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}
