use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AccessPolicyData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    application_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approval_required: std::option::Option<terrars::PrimField<bool>>,
    decision: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    isolation_required: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    precedence: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    purpose_justification_prompt: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    purpose_justification_required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approval_group: std::option::Option<std::vec::Vec<AccessPolicyApprovalGroupEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude: std::option::Option<std::vec::Vec<AccessPolicyExcludeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<std::vec::Vec<AccessPolicyIncludeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require: std::option::Option<std::vec::Vec<AccessPolicyRequireEl>>,
    dynamic: AccessPolicyDynamic,
}

struct AccessPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccessPolicyData>,
}

#[derive(Clone)]
pub struct AccessPolicy(std::rc::Rc<AccessPolicy_>);

impl AccessPolicy {
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

    #[doc = "Set the field `account_id`.\nThe account identifier to target for the resource. Conflicts with `zone_id`."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `approval_required`.\n"]
    pub fn set_approval_required(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().approval_required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `isolation_required`.\nRequire this application to be served in an isolated browser for users matching this policy."]
    pub fn set_isolation_required(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().isolation_required = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `purpose_justification_prompt`.\nThe prompt to display to the user for a justification for accessing the resource. Required when using `purpose_justification_required`."]
    pub fn set_purpose_justification_prompt(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().purpose_justification_prompt = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `purpose_justification_required`.\nWhether to prompt the user for a justification for accessing the resource."]
    pub fn set_purpose_justification_required(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().purpose_justification_required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `approval_group`.\n"]
    pub fn set_approval_group(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyApprovalGroupEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().approval_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.approval_group = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exclude`.\n"]
    pub fn set_exclude(self, v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyExcludeEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclude = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.exclude = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `include`.\n"]
    pub fn set_include(self, v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyIncludeEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().include = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.include = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `require`.\n"]
    pub fn set_require(self, v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyRequireEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().require = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.require = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Conflicts with `zone_id`."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_id` after provisioning.\nThe ID of the application the policy is associated with."]
    pub fn application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `approval_required` after provisioning.\n"]
    pub fn approval_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approval_required", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `decision` after provisioning.\nDefines the action Access will take if the policy matches the user. Available values: `allow`, `deny`, `non_identity`, `bypass`."]
    pub fn decision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.decision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `isolation_required` after provisioning.\nRequire this application to be served in an isolated browser for users matching this policy."]
    pub fn isolation_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.isolation_required", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nFriendly name of the Access Policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `precedence` after provisioning.\nThe unique precedence for policies on a single application."]
    pub fn precedence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precedence", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purpose_justification_prompt` after provisioning.\nThe prompt to display to the user for a justification for accessing the resource. Required when using `purpose_justification_required`."]
    pub fn purpose_justification_prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.purpose_justification_prompt", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purpose_justification_required` after provisioning.\nWhether to prompt the user for a justification for accessing the resource."]
    pub fn purpose_justification_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.purpose_justification_required", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `approval_group` after provisioning.\n"]
    pub fn approval_group(&self) -> terrars::ListRef<AccessPolicyApprovalGroupElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude` after provisioning.\n"]
    pub fn exclude(&self) -> terrars::ListRef<AccessPolicyExcludeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> terrars::ListRef<AccessPolicyIncludeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require` after provisioning.\n"]
    pub fn require(&self) -> terrars::ListRef<AccessPolicyRequireElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.require", self.extract_ref()))
    }
}

impl terrars::Referable for AccessPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AccessPolicy { }

impl terrars::ToListMappable for AccessPolicy {
    type O = terrars::ListRef<AccessPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AccessPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_access_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessPolicy {
    pub tf_id: std::string::String,
    #[doc = "The ID of the application the policy is associated with."]
    pub application_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Defines the action Access will take if the policy matches the user. Available values: `allow`, `deny`, `non_identity`, `bypass`."]
    pub decision: terrars::PrimField<std::string::String>,
    #[doc = "Friendly name of the Access Policy."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The unique precedence for policies on a single application."]
    pub precedence: terrars::PrimField<f64>,
}

impl BuildAccessPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> AccessPolicy {
        let out = AccessPolicy(std::rc::Rc::new(AccessPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AccessPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                application_id: self.application_id,
                approval_required: core::default::Default::default(),
                decision: self.decision,
                id: core::default::Default::default(),
                isolation_required: core::default::Default::default(),
                name: self.name,
                precedence: self.precedence,
                purpose_justification_prompt: core::default::Default::default(),
                purpose_justification_required: core::default::Default::default(),
                zone_id: core::default::Default::default(),
                approval_group: core::default::Default::default(),
                exclude: core::default::Default::default(),
                include: core::default::Default::default(),
                require: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Conflicts with `zone_id`."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_id` after provisioning.\nThe ID of the application the policy is associated with."]
    pub fn application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `approval_required` after provisioning.\n"]
    pub fn approval_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approval_required", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `decision` after provisioning.\nDefines the action Access will take if the policy matches the user. Available values: `allow`, `deny`, `non_identity`, `bypass`."]
    pub fn decision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.decision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `isolation_required` after provisioning.\nRequire this application to be served in an isolated browser for users matching this policy."]
    pub fn isolation_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.isolation_required", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nFriendly name of the Access Policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `precedence` after provisioning.\nThe unique precedence for policies on a single application."]
    pub fn precedence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precedence", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purpose_justification_prompt` after provisioning.\nThe prompt to display to the user for a justification for accessing the resource. Required when using `purpose_justification_required`."]
    pub fn purpose_justification_prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.purpose_justification_prompt", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purpose_justification_required` after provisioning.\nWhether to prompt the user for a justification for accessing the resource."]
    pub fn purpose_justification_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.purpose_justification_required", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `approval_group` after provisioning.\n"]
    pub fn approval_group(&self) -> terrars::ListRef<AccessPolicyApprovalGroupElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude` after provisioning.\n"]
    pub fn exclude(&self) -> terrars::ListRef<AccessPolicyExcludeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> terrars::ListRef<AccessPolicyIncludeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require` after provisioning.\n"]
    pub fn require(&self) -> terrars::ListRef<AccessPolicyRequireElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.require", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyApprovalGroupEl {
    approvals_needed: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_list_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessPolicyApprovalGroupEl {
    #[doc = "Set the field `email_addresses`.\nList of emails to request approval from."]
    pub fn set_email_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.email_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_list_uuid`.\n"]
    pub fn set_email_list_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email_list_uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyApprovalGroupEl {
    type O = terrars::BlockAssignable<AccessPolicyApprovalGroupEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyApprovalGroupEl {
    #[doc = "Number of approvals needed."]
    pub approvals_needed: terrars::PrimField<f64>,
}

impl BuildAccessPolicyApprovalGroupEl {
    pub fn build(self) -> AccessPolicyApprovalGroupEl {
        AccessPolicyApprovalGroupEl {
            approvals_needed: self.approvals_needed,
            email_addresses: core::default::Default::default(),
            email_list_uuid: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyApprovalGroupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyApprovalGroupElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyApprovalGroupElRef {
        AccessPolicyApprovalGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyApprovalGroupElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `approvals_needed` after provisioning.\nNumber of approvals needed."]
    pub fn approvals_needed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approvals_needed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email_addresses` after provisioning.\nList of emails to request approval from."]
    pub fn email_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `email_list_uuid` after provisioning.\n"]
    pub fn email_list_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_list_uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyExcludeElAzureEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessPolicyExcludeElAzureEl {
    #[doc = "Set the field `id`.\nThe ID of the Azure group or user."]
    pub fn set_id(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_provider_id`.\nThe ID of the Azure Identity provider."]
    pub fn set_identity_provider_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_provider_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyExcludeElAzureEl {
    type O = terrars::BlockAssignable<AccessPolicyExcludeElAzureEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyExcludeElAzureEl {}

impl BuildAccessPolicyExcludeElAzureEl {
    pub fn build(self) -> AccessPolicyExcludeElAzureEl {
        AccessPolicyExcludeElAzureEl {
            id: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyExcludeElAzureElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyExcludeElAzureElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyExcludeElAzureElRef {
        AccessPolicyExcludeElAzureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyExcludeElAzureElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the Azure group or user."]
    pub fn id(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of the Azure Identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyExcludeElExternalEvaluationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keys_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessPolicyExcludeElExternalEvaluationEl {
    #[doc = "Set the field `evaluate_url`.\n"]
    pub fn set_evaluate_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.evaluate_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `keys_url`.\n"]
    pub fn set_keys_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.keys_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyExcludeElExternalEvaluationEl {
    type O = terrars::BlockAssignable<AccessPolicyExcludeElExternalEvaluationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyExcludeElExternalEvaluationEl {}

impl BuildAccessPolicyExcludeElExternalEvaluationEl {
    pub fn build(self) -> AccessPolicyExcludeElExternalEvaluationEl {
        AccessPolicyExcludeElExternalEvaluationEl {
            evaluate_url: core::default::Default::default(),
            keys_url: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyExcludeElExternalEvaluationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyExcludeElExternalEvaluationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyExcludeElExternalEvaluationElRef {
        AccessPolicyExcludeElExternalEvaluationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyExcludeElExternalEvaluationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `evaluate_url` after provisioning.\n"]
    pub fn evaluate_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluate_url", self.base))
    }

    #[doc = "Get a reference to the value of field `keys_url` after provisioning.\n"]
    pub fn keys_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keys_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyExcludeElGithubEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    teams: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AccessPolicyExcludeElGithubEl {
    #[doc = "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_provider_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `teams`.\n"]
    pub fn set_teams(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.teams = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyExcludeElGithubEl {
    type O = terrars::BlockAssignable<AccessPolicyExcludeElGithubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyExcludeElGithubEl {}

impl BuildAccessPolicyExcludeElGithubEl {
    pub fn build(self) -> AccessPolicyExcludeElGithubEl {
        AccessPolicyExcludeElGithubEl {
            identity_provider_id: core::default::Default::default(),
            name: core::default::Default::default(),
            teams: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyExcludeElGithubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyExcludeElGithubElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyExcludeElGithubElRef {
        AccessPolicyExcludeElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyExcludeElGithubElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `teams` after provisioning.\n"]
    pub fn teams(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.teams", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyExcludeElGsuiteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessPolicyExcludeElGsuiteEl {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_provider_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyExcludeElGsuiteEl {
    type O = terrars::BlockAssignable<AccessPolicyExcludeElGsuiteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyExcludeElGsuiteEl {}

impl BuildAccessPolicyExcludeElGsuiteEl {
    pub fn build(self) -> AccessPolicyExcludeElGsuiteEl {
        AccessPolicyExcludeElGsuiteEl {
            email: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyExcludeElGsuiteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyExcludeElGsuiteElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyExcludeElGsuiteElRef {
        AccessPolicyExcludeElGsuiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyExcludeElGsuiteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyExcludeElOktaEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AccessPolicyExcludeElOktaEl {
    #[doc = "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_provider_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyExcludeElOktaEl {
    type O = terrars::BlockAssignable<AccessPolicyExcludeElOktaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyExcludeElOktaEl {}

impl BuildAccessPolicyExcludeElOktaEl {
    pub fn build(self) -> AccessPolicyExcludeElOktaEl {
        AccessPolicyExcludeElOktaEl {
            identity_provider_id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyExcludeElOktaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyExcludeElOktaElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyExcludeElOktaElRef {
        AccessPolicyExcludeElOktaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyExcludeElOktaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyExcludeElSamlEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessPolicyExcludeElSamlEl {
    #[doc = "Set the field `attribute_name`.\n"]
    pub fn set_attribute_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attribute_value`.\n"]
    pub fn set_attribute_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_provider_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyExcludeElSamlEl {
    type O = terrars::BlockAssignable<AccessPolicyExcludeElSamlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyExcludeElSamlEl {}

impl BuildAccessPolicyExcludeElSamlEl {
    pub fn build(self) -> AccessPolicyExcludeElSamlEl {
        AccessPolicyExcludeElSamlEl {
            attribute_name: core::default::Default::default(),
            attribute_value: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyExcludeElSamlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyExcludeElSamlElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyExcludeElSamlElRef {
        AccessPolicyExcludeElSamlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyExcludeElSamlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_name` after provisioning.\n"]
    pub fn attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute_value` after provisioning.\n"]
    pub fn attribute_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessPolicyExcludeElDynamic {
    azure: std::option::Option<terrars::DynamicBlock<AccessPolicyExcludeElAzureEl>>,
    external_evaluation: std::option::Option<terrars::DynamicBlock<AccessPolicyExcludeElExternalEvaluationEl>>,
    github: std::option::Option<terrars::DynamicBlock<AccessPolicyExcludeElGithubEl>>,
    gsuite: std::option::Option<terrars::DynamicBlock<AccessPolicyExcludeElGsuiteEl>>,
    okta: std::option::Option<terrars::DynamicBlock<AccessPolicyExcludeElOktaEl>>,
    saml: std::option::Option<terrars::DynamicBlock<AccessPolicyExcludeElSamlEl>>,
}

#[derive(Serialize)]
pub struct AccessPolicyExcludeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_valid_service_token: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_posture: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_domain: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    everyone: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_list: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_method: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_token: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure: std::option::Option<std::vec::Vec<AccessPolicyExcludeElAzureEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_evaluation: std::option::Option<std::vec::Vec<AccessPolicyExcludeElExternalEvaluationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<std::vec::Vec<AccessPolicyExcludeElGithubEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gsuite: std::option::Option<std::vec::Vec<AccessPolicyExcludeElGsuiteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    okta: std::option::Option<std::vec::Vec<AccessPolicyExcludeElOktaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml: std::option::Option<std::vec::Vec<AccessPolicyExcludeElSamlEl>>,
    dynamic: AccessPolicyExcludeElDynamic,
}

impl AccessPolicyExcludeEl {
    #[doc = "Set the field `any_valid_service_token`.\n"]
    pub fn set_any_valid_service_token(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.any_valid_service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_method`.\n"]
    pub fn set_auth_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.common_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_posture`.\n"]
    pub fn set_device_posture(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.device_posture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_domain`.\n"]
    pub fn set_email_domain(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.email_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `everyone`.\n"]
    pub fn set_everyone(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.everyone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geo`.\n"]
    pub fn set_geo(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.geo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip`.\n"]
    pub fn set_ip(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_list`.\n"]
    pub fn set_ip_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `login_method`.\n"]
    pub fn set_login_method(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.login_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_token`.\n"]
    pub fn set_service_token(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `azure`.\n"]
    pub fn set_azure(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyExcludeElAzureEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `external_evaluation`.\n"]
    pub fn set_external_evaluation(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyExcludeElExternalEvaluationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.external_evaluation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.external_evaluation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyExcludeElGithubEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.github = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.github = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gsuite`.\n"]
    pub fn set_gsuite(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyExcludeElGsuiteEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gsuite = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gsuite = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `okta`.\n"]
    pub fn set_okta(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyExcludeElOktaEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.okta = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.okta = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `saml`.\n"]
    pub fn set_saml(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyExcludeElSamlEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.saml = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.saml = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessPolicyExcludeEl {
    type O = terrars::BlockAssignable<AccessPolicyExcludeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyExcludeEl {}

impl BuildAccessPolicyExcludeEl {
    pub fn build(self) -> AccessPolicyExcludeEl {
        AccessPolicyExcludeEl {
            any_valid_service_token: core::default::Default::default(),
            auth_method: core::default::Default::default(),
            certificate: core::default::Default::default(),
            common_name: core::default::Default::default(),
            device_posture: core::default::Default::default(),
            email: core::default::Default::default(),
            email_domain: core::default::Default::default(),
            everyone: core::default::Default::default(),
            geo: core::default::Default::default(),
            group: core::default::Default::default(),
            ip: core::default::Default::default(),
            ip_list: core::default::Default::default(),
            login_method: core::default::Default::default(),
            service_token: core::default::Default::default(),
            azure: core::default::Default::default(),
            external_evaluation: core::default::Default::default(),
            github: core::default::Default::default(),
            gsuite: core::default::Default::default(),
            okta: core::default::Default::default(),
            saml: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyExcludeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyExcludeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyExcludeElRef {
        AccessPolicyExcludeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyExcludeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `any_valid_service_token` after provisioning.\n"]
    pub fn any_valid_service_token(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.any_valid_service_token", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device_posture", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `everyone` after provisioning.\n"]
    pub fn everyone(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.login_method", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_token", self.base))
    }

    #[doc = "Get a reference to the value of field `azure` after provisioning.\n"]
    pub fn azure(&self) -> terrars::ListRef<AccessPolicyExcludeElAzureElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure", self.base))
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> terrars::ListRef<AccessPolicyExcludeElExternalEvaluationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_evaluation", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<AccessPolicyExcludeElGithubElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> terrars::ListRef<AccessPolicyExcludeElGsuiteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> terrars::ListRef<AccessPolicyExcludeElOktaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> terrars::ListRef<AccessPolicyExcludeElSamlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyIncludeElAzureEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessPolicyIncludeElAzureEl {
    #[doc = "Set the field `id`.\nThe ID of the Azure group or user."]
    pub fn set_id(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_provider_id`.\nThe ID of the Azure Identity provider."]
    pub fn set_identity_provider_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_provider_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyIncludeElAzureEl {
    type O = terrars::BlockAssignable<AccessPolicyIncludeElAzureEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyIncludeElAzureEl {}

impl BuildAccessPolicyIncludeElAzureEl {
    pub fn build(self) -> AccessPolicyIncludeElAzureEl {
        AccessPolicyIncludeElAzureEl {
            id: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyIncludeElAzureElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyIncludeElAzureElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyIncludeElAzureElRef {
        AccessPolicyIncludeElAzureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyIncludeElAzureElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the Azure group or user."]
    pub fn id(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of the Azure Identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyIncludeElExternalEvaluationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keys_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessPolicyIncludeElExternalEvaluationEl {
    #[doc = "Set the field `evaluate_url`.\n"]
    pub fn set_evaluate_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.evaluate_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `keys_url`.\n"]
    pub fn set_keys_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.keys_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyIncludeElExternalEvaluationEl {
    type O = terrars::BlockAssignable<AccessPolicyIncludeElExternalEvaluationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyIncludeElExternalEvaluationEl {}

impl BuildAccessPolicyIncludeElExternalEvaluationEl {
    pub fn build(self) -> AccessPolicyIncludeElExternalEvaluationEl {
        AccessPolicyIncludeElExternalEvaluationEl {
            evaluate_url: core::default::Default::default(),
            keys_url: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyIncludeElExternalEvaluationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyIncludeElExternalEvaluationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyIncludeElExternalEvaluationElRef {
        AccessPolicyIncludeElExternalEvaluationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyIncludeElExternalEvaluationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `evaluate_url` after provisioning.\n"]
    pub fn evaluate_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluate_url", self.base))
    }

    #[doc = "Get a reference to the value of field `keys_url` after provisioning.\n"]
    pub fn keys_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keys_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyIncludeElGithubEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    teams: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AccessPolicyIncludeElGithubEl {
    #[doc = "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_provider_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `teams`.\n"]
    pub fn set_teams(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.teams = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyIncludeElGithubEl {
    type O = terrars::BlockAssignable<AccessPolicyIncludeElGithubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyIncludeElGithubEl {}

impl BuildAccessPolicyIncludeElGithubEl {
    pub fn build(self) -> AccessPolicyIncludeElGithubEl {
        AccessPolicyIncludeElGithubEl {
            identity_provider_id: core::default::Default::default(),
            name: core::default::Default::default(),
            teams: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyIncludeElGithubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyIncludeElGithubElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyIncludeElGithubElRef {
        AccessPolicyIncludeElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyIncludeElGithubElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `teams` after provisioning.\n"]
    pub fn teams(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.teams", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyIncludeElGsuiteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessPolicyIncludeElGsuiteEl {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_provider_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyIncludeElGsuiteEl {
    type O = terrars::BlockAssignable<AccessPolicyIncludeElGsuiteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyIncludeElGsuiteEl {}

impl BuildAccessPolicyIncludeElGsuiteEl {
    pub fn build(self) -> AccessPolicyIncludeElGsuiteEl {
        AccessPolicyIncludeElGsuiteEl {
            email: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyIncludeElGsuiteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyIncludeElGsuiteElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyIncludeElGsuiteElRef {
        AccessPolicyIncludeElGsuiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyIncludeElGsuiteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyIncludeElOktaEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AccessPolicyIncludeElOktaEl {
    #[doc = "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_provider_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyIncludeElOktaEl {
    type O = terrars::BlockAssignable<AccessPolicyIncludeElOktaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyIncludeElOktaEl {}

impl BuildAccessPolicyIncludeElOktaEl {
    pub fn build(self) -> AccessPolicyIncludeElOktaEl {
        AccessPolicyIncludeElOktaEl {
            identity_provider_id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyIncludeElOktaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyIncludeElOktaElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyIncludeElOktaElRef {
        AccessPolicyIncludeElOktaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyIncludeElOktaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyIncludeElSamlEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessPolicyIncludeElSamlEl {
    #[doc = "Set the field `attribute_name`.\n"]
    pub fn set_attribute_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attribute_value`.\n"]
    pub fn set_attribute_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_provider_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyIncludeElSamlEl {
    type O = terrars::BlockAssignable<AccessPolicyIncludeElSamlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyIncludeElSamlEl {}

impl BuildAccessPolicyIncludeElSamlEl {
    pub fn build(self) -> AccessPolicyIncludeElSamlEl {
        AccessPolicyIncludeElSamlEl {
            attribute_name: core::default::Default::default(),
            attribute_value: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyIncludeElSamlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyIncludeElSamlElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyIncludeElSamlElRef {
        AccessPolicyIncludeElSamlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyIncludeElSamlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_name` after provisioning.\n"]
    pub fn attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute_value` after provisioning.\n"]
    pub fn attribute_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessPolicyIncludeElDynamic {
    azure: std::option::Option<terrars::DynamicBlock<AccessPolicyIncludeElAzureEl>>,
    external_evaluation: std::option::Option<terrars::DynamicBlock<AccessPolicyIncludeElExternalEvaluationEl>>,
    github: std::option::Option<terrars::DynamicBlock<AccessPolicyIncludeElGithubEl>>,
    gsuite: std::option::Option<terrars::DynamicBlock<AccessPolicyIncludeElGsuiteEl>>,
    okta: std::option::Option<terrars::DynamicBlock<AccessPolicyIncludeElOktaEl>>,
    saml: std::option::Option<terrars::DynamicBlock<AccessPolicyIncludeElSamlEl>>,
}

#[derive(Serialize)]
pub struct AccessPolicyIncludeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_valid_service_token: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_posture: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_domain: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    everyone: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_list: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_method: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_token: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure: std::option::Option<std::vec::Vec<AccessPolicyIncludeElAzureEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_evaluation: std::option::Option<std::vec::Vec<AccessPolicyIncludeElExternalEvaluationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<std::vec::Vec<AccessPolicyIncludeElGithubEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gsuite: std::option::Option<std::vec::Vec<AccessPolicyIncludeElGsuiteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    okta: std::option::Option<std::vec::Vec<AccessPolicyIncludeElOktaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml: std::option::Option<std::vec::Vec<AccessPolicyIncludeElSamlEl>>,
    dynamic: AccessPolicyIncludeElDynamic,
}

impl AccessPolicyIncludeEl {
    #[doc = "Set the field `any_valid_service_token`.\n"]
    pub fn set_any_valid_service_token(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.any_valid_service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_method`.\n"]
    pub fn set_auth_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.common_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_posture`.\n"]
    pub fn set_device_posture(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.device_posture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_domain`.\n"]
    pub fn set_email_domain(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.email_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `everyone`.\n"]
    pub fn set_everyone(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.everyone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geo`.\n"]
    pub fn set_geo(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.geo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip`.\n"]
    pub fn set_ip(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_list`.\n"]
    pub fn set_ip_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `login_method`.\n"]
    pub fn set_login_method(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.login_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_token`.\n"]
    pub fn set_service_token(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `azure`.\n"]
    pub fn set_azure(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyIncludeElAzureEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `external_evaluation`.\n"]
    pub fn set_external_evaluation(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyIncludeElExternalEvaluationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.external_evaluation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.external_evaluation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyIncludeElGithubEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.github = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.github = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gsuite`.\n"]
    pub fn set_gsuite(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyIncludeElGsuiteEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gsuite = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gsuite = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `okta`.\n"]
    pub fn set_okta(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyIncludeElOktaEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.okta = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.okta = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `saml`.\n"]
    pub fn set_saml(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyIncludeElSamlEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.saml = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.saml = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessPolicyIncludeEl {
    type O = terrars::BlockAssignable<AccessPolicyIncludeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyIncludeEl {}

impl BuildAccessPolicyIncludeEl {
    pub fn build(self) -> AccessPolicyIncludeEl {
        AccessPolicyIncludeEl {
            any_valid_service_token: core::default::Default::default(),
            auth_method: core::default::Default::default(),
            certificate: core::default::Default::default(),
            common_name: core::default::Default::default(),
            device_posture: core::default::Default::default(),
            email: core::default::Default::default(),
            email_domain: core::default::Default::default(),
            everyone: core::default::Default::default(),
            geo: core::default::Default::default(),
            group: core::default::Default::default(),
            ip: core::default::Default::default(),
            ip_list: core::default::Default::default(),
            login_method: core::default::Default::default(),
            service_token: core::default::Default::default(),
            azure: core::default::Default::default(),
            external_evaluation: core::default::Default::default(),
            github: core::default::Default::default(),
            gsuite: core::default::Default::default(),
            okta: core::default::Default::default(),
            saml: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyIncludeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyIncludeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyIncludeElRef {
        AccessPolicyIncludeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyIncludeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `any_valid_service_token` after provisioning.\n"]
    pub fn any_valid_service_token(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.any_valid_service_token", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device_posture", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `everyone` after provisioning.\n"]
    pub fn everyone(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.login_method", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_token", self.base))
    }

    #[doc = "Get a reference to the value of field `azure` after provisioning.\n"]
    pub fn azure(&self) -> terrars::ListRef<AccessPolicyIncludeElAzureElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure", self.base))
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> terrars::ListRef<AccessPolicyIncludeElExternalEvaluationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_evaluation", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<AccessPolicyIncludeElGithubElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> terrars::ListRef<AccessPolicyIncludeElGsuiteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> terrars::ListRef<AccessPolicyIncludeElOktaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> terrars::ListRef<AccessPolicyIncludeElSamlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyRequireElAzureEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessPolicyRequireElAzureEl {
    #[doc = "Set the field `id`.\nThe ID of the Azure group or user."]
    pub fn set_id(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_provider_id`.\nThe ID of the Azure Identity provider."]
    pub fn set_identity_provider_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_provider_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyRequireElAzureEl {
    type O = terrars::BlockAssignable<AccessPolicyRequireElAzureEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyRequireElAzureEl {}

impl BuildAccessPolicyRequireElAzureEl {
    pub fn build(self) -> AccessPolicyRequireElAzureEl {
        AccessPolicyRequireElAzureEl {
            id: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyRequireElAzureElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyRequireElAzureElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyRequireElAzureElRef {
        AccessPolicyRequireElAzureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyRequireElAzureElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the Azure group or user."]
    pub fn id(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of the Azure Identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyRequireElExternalEvaluationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keys_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessPolicyRequireElExternalEvaluationEl {
    #[doc = "Set the field `evaluate_url`.\n"]
    pub fn set_evaluate_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.evaluate_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `keys_url`.\n"]
    pub fn set_keys_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.keys_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyRequireElExternalEvaluationEl {
    type O = terrars::BlockAssignable<AccessPolicyRequireElExternalEvaluationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyRequireElExternalEvaluationEl {}

impl BuildAccessPolicyRequireElExternalEvaluationEl {
    pub fn build(self) -> AccessPolicyRequireElExternalEvaluationEl {
        AccessPolicyRequireElExternalEvaluationEl {
            evaluate_url: core::default::Default::default(),
            keys_url: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyRequireElExternalEvaluationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyRequireElExternalEvaluationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyRequireElExternalEvaluationElRef {
        AccessPolicyRequireElExternalEvaluationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyRequireElExternalEvaluationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `evaluate_url` after provisioning.\n"]
    pub fn evaluate_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluate_url", self.base))
    }

    #[doc = "Get a reference to the value of field `keys_url` after provisioning.\n"]
    pub fn keys_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keys_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyRequireElGithubEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    teams: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AccessPolicyRequireElGithubEl {
    #[doc = "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_provider_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `teams`.\n"]
    pub fn set_teams(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.teams = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyRequireElGithubEl {
    type O = terrars::BlockAssignable<AccessPolicyRequireElGithubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyRequireElGithubEl {}

impl BuildAccessPolicyRequireElGithubEl {
    pub fn build(self) -> AccessPolicyRequireElGithubEl {
        AccessPolicyRequireElGithubEl {
            identity_provider_id: core::default::Default::default(),
            name: core::default::Default::default(),
            teams: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyRequireElGithubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyRequireElGithubElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyRequireElGithubElRef {
        AccessPolicyRequireElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyRequireElGithubElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `teams` after provisioning.\n"]
    pub fn teams(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.teams", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyRequireElGsuiteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessPolicyRequireElGsuiteEl {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_provider_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyRequireElGsuiteEl {
    type O = terrars::BlockAssignable<AccessPolicyRequireElGsuiteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyRequireElGsuiteEl {}

impl BuildAccessPolicyRequireElGsuiteEl {
    pub fn build(self) -> AccessPolicyRequireElGsuiteEl {
        AccessPolicyRequireElGsuiteEl {
            email: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyRequireElGsuiteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyRequireElGsuiteElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyRequireElGsuiteElRef {
        AccessPolicyRequireElGsuiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyRequireElGsuiteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyRequireElOktaEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AccessPolicyRequireElOktaEl {
    #[doc = "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_provider_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyRequireElOktaEl {
    type O = terrars::BlockAssignable<AccessPolicyRequireElOktaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyRequireElOktaEl {}

impl BuildAccessPolicyRequireElOktaEl {
    pub fn build(self) -> AccessPolicyRequireElOktaEl {
        AccessPolicyRequireElOktaEl {
            identity_provider_id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyRequireElOktaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyRequireElOktaElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyRequireElOktaElRef {
        AccessPolicyRequireElOktaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyRequireElOktaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyRequireElSamlEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessPolicyRequireElSamlEl {
    #[doc = "Set the field `attribute_name`.\n"]
    pub fn set_attribute_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attribute_value`.\n"]
    pub fn set_attribute_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_provider_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessPolicyRequireElSamlEl {
    type O = terrars::BlockAssignable<AccessPolicyRequireElSamlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyRequireElSamlEl {}

impl BuildAccessPolicyRequireElSamlEl {
    pub fn build(self) -> AccessPolicyRequireElSamlEl {
        AccessPolicyRequireElSamlEl {
            attribute_name: core::default::Default::default(),
            attribute_value: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyRequireElSamlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyRequireElSamlElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyRequireElSamlElRef {
        AccessPolicyRequireElSamlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyRequireElSamlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_name` after provisioning.\n"]
    pub fn attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute_value` after provisioning.\n"]
    pub fn attribute_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessPolicyRequireElDynamic {
    azure: std::option::Option<terrars::DynamicBlock<AccessPolicyRequireElAzureEl>>,
    external_evaluation: std::option::Option<terrars::DynamicBlock<AccessPolicyRequireElExternalEvaluationEl>>,
    github: std::option::Option<terrars::DynamicBlock<AccessPolicyRequireElGithubEl>>,
    gsuite: std::option::Option<terrars::DynamicBlock<AccessPolicyRequireElGsuiteEl>>,
    okta: std::option::Option<terrars::DynamicBlock<AccessPolicyRequireElOktaEl>>,
    saml: std::option::Option<terrars::DynamicBlock<AccessPolicyRequireElSamlEl>>,
}

#[derive(Serialize)]
pub struct AccessPolicyRequireEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_valid_service_token: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_posture: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_domain: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    everyone: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_list: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_method: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_token: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure: std::option::Option<std::vec::Vec<AccessPolicyRequireElAzureEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_evaluation: std::option::Option<std::vec::Vec<AccessPolicyRequireElExternalEvaluationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<std::vec::Vec<AccessPolicyRequireElGithubEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gsuite: std::option::Option<std::vec::Vec<AccessPolicyRequireElGsuiteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    okta: std::option::Option<std::vec::Vec<AccessPolicyRequireElOktaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml: std::option::Option<std::vec::Vec<AccessPolicyRequireElSamlEl>>,
    dynamic: AccessPolicyRequireElDynamic,
}

impl AccessPolicyRequireEl {
    #[doc = "Set the field `any_valid_service_token`.\n"]
    pub fn set_any_valid_service_token(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.any_valid_service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_method`.\n"]
    pub fn set_auth_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.common_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_posture`.\n"]
    pub fn set_device_posture(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.device_posture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_domain`.\n"]
    pub fn set_email_domain(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.email_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `everyone`.\n"]
    pub fn set_everyone(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.everyone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geo`.\n"]
    pub fn set_geo(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.geo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip`.\n"]
    pub fn set_ip(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_list`.\n"]
    pub fn set_ip_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `login_method`.\n"]
    pub fn set_login_method(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.login_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_token`.\n"]
    pub fn set_service_token(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `azure`.\n"]
    pub fn set_azure(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyRequireElAzureEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `external_evaluation`.\n"]
    pub fn set_external_evaluation(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyRequireElExternalEvaluationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.external_evaluation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.external_evaluation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyRequireElGithubEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.github = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.github = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gsuite`.\n"]
    pub fn set_gsuite(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyRequireElGsuiteEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gsuite = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gsuite = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `okta`.\n"]
    pub fn set_okta(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyRequireElOktaEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.okta = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.okta = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `saml`.\n"]
    pub fn set_saml(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessPolicyRequireElSamlEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.saml = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.saml = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessPolicyRequireEl {
    type O = terrars::BlockAssignable<AccessPolicyRequireEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyRequireEl {}

impl BuildAccessPolicyRequireEl {
    pub fn build(self) -> AccessPolicyRequireEl {
        AccessPolicyRequireEl {
            any_valid_service_token: core::default::Default::default(),
            auth_method: core::default::Default::default(),
            certificate: core::default::Default::default(),
            common_name: core::default::Default::default(),
            device_posture: core::default::Default::default(),
            email: core::default::Default::default(),
            email_domain: core::default::Default::default(),
            everyone: core::default::Default::default(),
            geo: core::default::Default::default(),
            group: core::default::Default::default(),
            ip: core::default::Default::default(),
            ip_list: core::default::Default::default(),
            login_method: core::default::Default::default(),
            service_token: core::default::Default::default(),
            azure: core::default::Default::default(),
            external_evaluation: core::default::Default::default(),
            github: core::default::Default::default(),
            gsuite: core::default::Default::default(),
            okta: core::default::Default::default(),
            saml: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessPolicyRequireElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessPolicyRequireElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessPolicyRequireElRef {
        AccessPolicyRequireElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyRequireElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `any_valid_service_token` after provisioning.\n"]
    pub fn any_valid_service_token(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.any_valid_service_token", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device_posture", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `everyone` after provisioning.\n"]
    pub fn everyone(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.login_method", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_token", self.base))
    }

    #[doc = "Get a reference to the value of field `azure` after provisioning.\n"]
    pub fn azure(&self) -> terrars::ListRef<AccessPolicyRequireElAzureElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure", self.base))
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> terrars::ListRef<AccessPolicyRequireElExternalEvaluationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_evaluation", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<AccessPolicyRequireElGithubElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> terrars::ListRef<AccessPolicyRequireElGsuiteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> terrars::ListRef<AccessPolicyRequireElOktaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> terrars::ListRef<AccessPolicyRequireElSamlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessPolicyDynamic {
    approval_group: std::option::Option<terrars::DynamicBlock<AccessPolicyApprovalGroupEl>>,
    exclude: std::option::Option<terrars::DynamicBlock<AccessPolicyExcludeEl>>,
    include: std::option::Option<terrars::DynamicBlock<AccessPolicyIncludeEl>>,
    require: std::option::Option<terrars::DynamicBlock<AccessPolicyRequireEl>>,
}
