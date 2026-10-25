#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ShareData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_recipient_counts: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_resources: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    recipients: std::vec::Vec<Share_Recipients>,
    resources: std::vec::Vec<Share_Resources>,
}

struct Share_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ShareData_>,
}

#[derive(Clone)]
pub struct Share(std::rc::Rc<Share_>);

impl Share {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc = "Get a reference to the value of field `recipients` after provisioning.\n"]
    pub fn recipients(&self) -> terrars::ListRef<Share_RecipientsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recipients", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<Share_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
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

impl terrars::Referable for Share {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Share { }

impl terrars::ToListMappable for Share {
    type O = terrars::ListRef<ShareRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Share_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_share".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildShare {
    pub tf_id: std::string::String,
    #[doc = "Account identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the share."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub recipients: std::vec::Vec<Share_Recipients>,
    #[doc = ""]
    pub resources: std::vec::Vec<Share_Resources>,
}

impl BuildShare {
    pub fn build(self, stack: &mut terrars::Stack) -> Share {
        let out = Share(std::rc::Rc::new(Share_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ShareData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                include_recipient_counts: core::default::Default::default(),
                include_resources: core::default::Default::default(),
                name: self.name,
                recipients: self.recipients,
                resources: self.resources,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ShareRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ShareRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ShareRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `recipients` after provisioning.\n"]
    pub fn recipients(&self) -> terrars::ListRef<Share_RecipientsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recipients", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<Share_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
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
pub struct Share_Recipients {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recipient_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Share_Recipients {
    #[doc = "Set the field `organization_id`.\nOrganization identifier."]
    pub fn set_organization_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.organization_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recipient_account_id`.\nThe account that will receive the share."]
    pub fn set_recipient_account_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.recipient_account_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Share_Recipients {
    type O = terrars::BlockAssignable<Share_Recipients>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildShare_Recipients {}

impl BuildShare_Recipients {
    pub fn build(self) -> Share_Recipients {
        Share_Recipients {
            organization_id: core::default::Default::default(),
            recipient_account_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Share_RecipientsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Share_RecipientsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Share_RecipientsRef {
        Share_RecipientsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Share_RecipientsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\nOrganization identifier."]
    pub fn organization_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `recipient_account_id` after provisioning.\nThe account that will receive the share."]
    pub fn recipient_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recipient_account_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Share_Resources {
    meta: terrars::PrimField<std::string::String>,
    resource_account_id: terrars::PrimField<std::string::String>,
    resource_id: terrars::PrimField<std::string::String>,
    resource_type: terrars::PrimField<std::string::String>,
}

impl Share_Resources { }

impl terrars::ToListMappable for Share_Resources {
    type O = terrars::BlockAssignable<Share_Resources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildShare_Resources {
    #[doc = "Resource Metadata."]
    pub meta: terrars::PrimField<std::string::String>,
    #[doc = "Account identifier."]
    pub resource_account_id: terrars::PrimField<std::string::String>,
    #[doc = "Share Resource identifier."]
    pub resource_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource Type.\nAvailable values: \"custom-ruleset\", \"gateway-policy\", \"gateway-destination-ip\", \"gateway-block-page-settings\", \"gateway-extended-email-matching\", \"idp-federation-grant\", \"trust-grant\"."]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildShare_Resources {
    pub fn build(self) -> Share_Resources {
        Share_Resources {
            meta: self.meta,
            resource_account_id: self.resource_account_id,
            resource_id: self.resource_id,
            resource_type: self.resource_type,
        }
    }
}

#[allow(dead_code)]
pub struct Share_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Share_ResourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Share_ResourcesRef {
        Share_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Share_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `meta` after provisioning.\nResource Metadata."]
    pub fn meta(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.meta", self.base))
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
}
