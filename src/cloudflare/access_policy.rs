use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AccessPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    application_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approval_required: Option<PrimField<bool>>,
    decision: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isolation_required: Option<PrimField<bool>>,
    name: PrimField<String>,
    precedence: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purpose_justification_prompt: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purpose_justification_required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approval_group: Option<Vec<AccessPolicyApprovalGroupEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<Vec<AccessPolicyExcludeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<Vec<AccessPolicyIncludeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require: Option<Vec<AccessPolicyRequireEl>>,
    dynamic: AccessPolicyDynamic,
}

struct AccessPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessPolicyData>,
}

#[derive(Clone)]
pub struct AccessPolicy(Rc<AccessPolicy_>);

impl AccessPolicy {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `account_id`.\nThe account identifier to target for the resource. Conflicts with `zone_id`."]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }

    #[doc= "Set the field `approval_required`.\n"]
    pub fn set_approval_required(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().approval_required = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `isolation_required`.\nRequire this application to be served in an isolated browser for users matching this policy."]
    pub fn set_isolation_required(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().isolation_required = Some(v.into());
        self
    }

    #[doc= "Set the field `purpose_justification_prompt`.\nThe prompt to display to the user for a justification for accessing the resource. Required when using `purpose_justification_required`."]
    pub fn set_purpose_justification_prompt(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().purpose_justification_prompt = Some(v.into());
        self
    }

    #[doc= "Set the field `purpose_justification_required`.\nWhether to prompt the user for a justification for accessing the resource."]
    pub fn set_purpose_justification_required(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().purpose_justification_required = Some(v.into());
        self
    }

    #[doc= "Set the field `zone_id`.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn set_zone_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone_id = Some(v.into());
        self
    }

    #[doc= "Set the field `approval_group`.\n"]
    pub fn set_approval_group(self, v: impl Into<BlockAssignable<AccessPolicyApprovalGroupEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().approval_group = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.approval_group = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `exclude`.\n"]
    pub fn set_exclude(self, v: impl Into<BlockAssignable<AccessPolicyExcludeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclude = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.exclude = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `include`.\n"]
    pub fn set_include(self, v: impl Into<BlockAssignable<AccessPolicyIncludeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().include = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.include = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `require`.\n"]
    pub fn set_require(self, v: impl Into<BlockAssignable<AccessPolicyRequireEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().require = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.require = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Conflicts with `zone_id`."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `application_id` after provisioning.\nThe ID of the application the policy is associated with."]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `approval_required` after provisioning.\n"]
    pub fn approval_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.approval_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `decision` after provisioning.\nDefines the action Access will take if the policy matches the user. Available values: `allow`, `deny`, `non_identity`, `bypass`."]
    pub fn decision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.decision", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `isolation_required` after provisioning.\nRequire this application to be served in an isolated browser for users matching this policy."]
    pub fn isolation_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.isolation_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nFriendly name of the Access Policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `precedence` after provisioning.\nThe unique precedence for policies on a single application."]
    pub fn precedence(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.precedence", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `purpose_justification_prompt` after provisioning.\nThe prompt to display to the user for a justification for accessing the resource. Required when using `purpose_justification_required`."]
    pub fn purpose_justification_prompt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.purpose_justification_prompt", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `purpose_justification_required` after provisioning.\nWhether to prompt the user for a justification for accessing the resource."]
    pub fn purpose_justification_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.purpose_justification_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `approval_group` after provisioning.\n"]
    pub fn approval_group(&self) -> ListRef<AccessPolicyApprovalGroupElRef> {
        ListRef::new(self.shared().clone(), format!("{}.approval_group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `exclude` after provisioning.\n"]
    pub fn exclude(&self) -> ListRef<AccessPolicyExcludeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclude", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> ListRef<AccessPolicyIncludeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.include", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require` after provisioning.\n"]
    pub fn require(&self) -> ListRef<AccessPolicyRequireElRef> {
        ListRef::new(self.shared().clone(), format!("{}.require", self.extract_ref()))
    }
}

impl Referable for AccessPolicy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AccessPolicy { }

impl ToListMappable for AccessPolicy {
    type O = ListRef<AccessPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AccessPolicy_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_access_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessPolicy {
    pub tf_id: String,
    #[doc= "The ID of the application the policy is associated with."]
    pub application_id: PrimField<String>,
    #[doc= "Defines the action Access will take if the policy matches the user. Available values: `allow`, `deny`, `non_identity`, `bypass`."]
    pub decision: PrimField<String>,
    #[doc= "Friendly name of the Access Policy."]
    pub name: PrimField<String>,
    #[doc= "The unique precedence for policies on a single application."]
    pub precedence: PrimField<f64>,
}

impl BuildAccessPolicy {
    pub fn build(self, stack: &mut Stack) -> AccessPolicy {
        let out = AccessPolicy(Rc::new(AccessPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AccessPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Conflicts with `zone_id`."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `application_id` after provisioning.\nThe ID of the application the policy is associated with."]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `approval_required` after provisioning.\n"]
    pub fn approval_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.approval_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `decision` after provisioning.\nDefines the action Access will take if the policy matches the user. Available values: `allow`, `deny`, `non_identity`, `bypass`."]
    pub fn decision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.decision", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `isolation_required` after provisioning.\nRequire this application to be served in an isolated browser for users matching this policy."]
    pub fn isolation_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.isolation_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nFriendly name of the Access Policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `precedence` after provisioning.\nThe unique precedence for policies on a single application."]
    pub fn precedence(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.precedence", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `purpose_justification_prompt` after provisioning.\nThe prompt to display to the user for a justification for accessing the resource. Required when using `purpose_justification_required`."]
    pub fn purpose_justification_prompt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.purpose_justification_prompt", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `purpose_justification_required` after provisioning.\nWhether to prompt the user for a justification for accessing the resource."]
    pub fn purpose_justification_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.purpose_justification_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `approval_group` after provisioning.\n"]
    pub fn approval_group(&self) -> ListRef<AccessPolicyApprovalGroupElRef> {
        ListRef::new(self.shared().clone(), format!("{}.approval_group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `exclude` after provisioning.\n"]
    pub fn exclude(&self) -> ListRef<AccessPolicyExcludeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclude", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> ListRef<AccessPolicyIncludeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.include", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require` after provisioning.\n"]
    pub fn require(&self) -> ListRef<AccessPolicyRequireElRef> {
        ListRef::new(self.shared().clone(), format!("{}.require", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyApprovalGroupEl {
    approvals_needed: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_list_uuid: Option<PrimField<String>>,
}

impl AccessPolicyApprovalGroupEl {
    #[doc= "Set the field `email_addresses`.\nList of emails to request approval from."]
    pub fn set_email_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email_addresses = Some(v.into());
        self
    }

    #[doc= "Set the field `email_list_uuid`.\n"]
    pub fn set_email_list_uuid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email_list_uuid = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyApprovalGroupEl {
    type O = BlockAssignable<AccessPolicyApprovalGroupEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessPolicyApprovalGroupEl {
    #[doc= "Number of approvals needed."]
    pub approvals_needed: PrimField<f64>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyApprovalGroupElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyApprovalGroupElRef {
        AccessPolicyApprovalGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyApprovalGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `approvals_needed` after provisioning.\nNumber of approvals needed."]
    pub fn approvals_needed(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.approvals_needed", self.base))
    }

    #[doc= "Get a reference to the value of field `email_addresses` after provisioning.\nList of emails to request approval from."]
    pub fn email_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email_addresses", self.base))
    }

    #[doc= "Get a reference to the value of field `email_list_uuid` after provisioning.\n"]
    pub fn email_list_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_list_uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyExcludeElAzureEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessPolicyExcludeElAzureEl {
    #[doc= "Set the field `id`.\nThe ID of the Azure group or user."]
    pub fn set_id(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\nThe ID of the Azure Identity provider."]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyExcludeElAzureEl {
    type O = BlockAssignable<AccessPolicyExcludeElAzureEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyExcludeElAzureElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyExcludeElAzureElRef {
        AccessPolicyExcludeElAzureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyExcludeElAzureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the Azure group or user."]
    pub fn id(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of the Azure Identity provider."]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyExcludeElExternalEvaluationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluate_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keys_url: Option<PrimField<String>>,
}

impl AccessPolicyExcludeElExternalEvaluationEl {
    #[doc= "Set the field `evaluate_url`.\n"]
    pub fn set_evaluate_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.evaluate_url = Some(v.into());
        self
    }

    #[doc= "Set the field `keys_url`.\n"]
    pub fn set_keys_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.keys_url = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyExcludeElExternalEvaluationEl {
    type O = BlockAssignable<AccessPolicyExcludeElExternalEvaluationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyExcludeElExternalEvaluationElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyExcludeElExternalEvaluationElRef {
        AccessPolicyExcludeElExternalEvaluationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyExcludeElExternalEvaluationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `evaluate_url` after provisioning.\n"]
    pub fn evaluate_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.evaluate_url", self.base))
    }

    #[doc= "Get a reference to the value of field `keys_url` after provisioning.\n"]
    pub fn keys_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.keys_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyExcludeElGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    teams: Option<ListField<PrimField<String>>>,
}

impl AccessPolicyExcludeElGithubEl {
    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `teams`.\n"]
    pub fn set_teams(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.teams = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyExcludeElGithubEl {
    type O = BlockAssignable<AccessPolicyExcludeElGithubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyExcludeElGithubElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyExcludeElGithubElRef {
        AccessPolicyExcludeElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyExcludeElGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `teams` after provisioning.\n"]
    pub fn teams(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.teams", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyExcludeElGsuiteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessPolicyExcludeElGsuiteEl {
    #[doc= "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyExcludeElGsuiteEl {
    type O = BlockAssignable<AccessPolicyExcludeElGsuiteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyExcludeElGsuiteElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyExcludeElGsuiteElRef {
        AccessPolicyExcludeElGsuiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyExcludeElGsuiteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyExcludeElOktaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<ListField<PrimField<String>>>,
}

impl AccessPolicyExcludeElOktaEl {
    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyExcludeElOktaEl {
    type O = BlockAssignable<AccessPolicyExcludeElOktaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyExcludeElOktaElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyExcludeElOktaElRef {
        AccessPolicyExcludeElOktaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyExcludeElOktaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyExcludeElSamlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessPolicyExcludeElSamlEl {
    #[doc= "Set the field `attribute_name`.\n"]
    pub fn set_attribute_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_name = Some(v.into());
        self
    }

    #[doc= "Set the field `attribute_value`.\n"]
    pub fn set_attribute_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_value = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyExcludeElSamlEl {
    type O = BlockAssignable<AccessPolicyExcludeElSamlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyExcludeElSamlElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyExcludeElSamlElRef {
        AccessPolicyExcludeElSamlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyExcludeElSamlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `attribute_name` after provisioning.\n"]
    pub fn attribute_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc= "Get a reference to the value of field `attribute_value` after provisioning.\n"]
    pub fn attribute_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessPolicyExcludeElDynamic {
    azure: Option<DynamicBlock<AccessPolicyExcludeElAzureEl>>,
    external_evaluation: Option<DynamicBlock<AccessPolicyExcludeElExternalEvaluationEl>>,
    github: Option<DynamicBlock<AccessPolicyExcludeElGithubEl>>,
    gsuite: Option<DynamicBlock<AccessPolicyExcludeElGsuiteEl>>,
    okta: Option<DynamicBlock<AccessPolicyExcludeElOktaEl>>,
    saml: Option<DynamicBlock<AccessPolicyExcludeElSamlEl>>,
}

#[derive(Serialize)]
pub struct AccessPolicyExcludeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    any_valid_service_token: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_posture: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_domain: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    everyone: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geo: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_list: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_method: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_token: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    azure: Option<Vec<AccessPolicyExcludeElAzureEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_evaluation: Option<Vec<AccessPolicyExcludeElExternalEvaluationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<Vec<AccessPolicyExcludeElGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gsuite: Option<Vec<AccessPolicyExcludeElGsuiteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    okta: Option<Vec<AccessPolicyExcludeElOktaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saml: Option<Vec<AccessPolicyExcludeElSamlEl>>,
    dynamic: AccessPolicyExcludeElDynamic,
}

impl AccessPolicyExcludeEl {
    #[doc= "Set the field `any_valid_service_token`.\n"]
    pub fn set_any_valid_service_token(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.any_valid_service_token = Some(v.into());
        self
    }

    #[doc= "Set the field `auth_method`.\n"]
    pub fn set_auth_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_method = Some(v.into());
        self
    }

    #[doc= "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.common_name = Some(v.into());
        self
    }

    #[doc= "Set the field `device_posture`.\n"]
    pub fn set_device_posture(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.device_posture = Some(v.into());
        self
    }

    #[doc= "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `email_domain`.\n"]
    pub fn set_email_domain(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email_domain = Some(v.into());
        self
    }

    #[doc= "Set the field `everyone`.\n"]
    pub fn set_everyone(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.everyone = Some(v.into());
        self
    }

    #[doc= "Set the field `geo`.\n"]
    pub fn set_geo(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.geo = Some(v.into());
        self
    }

    #[doc= "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.group = Some(v.into());
        self
    }

    #[doc= "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_list`.\n"]
    pub fn set_ip_list(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_list = Some(v.into());
        self
    }

    #[doc= "Set the field `login_method`.\n"]
    pub fn set_login_method(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.login_method = Some(v.into());
        self
    }

    #[doc= "Set the field `service_token`.\n"]
    pub fn set_service_token(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.service_token = Some(v.into());
        self
    }

    #[doc= "Set the field `azure`.\n"]
    pub fn set_azure(mut self, v: impl Into<BlockAssignable<AccessPolicyExcludeElAzureEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.azure = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.azure = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `external_evaluation`.\n"]
    pub fn set_external_evaluation(
        mut self,
        v: impl Into<BlockAssignable<AccessPolicyExcludeElExternalEvaluationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.external_evaluation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.external_evaluation = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl Into<BlockAssignable<AccessPolicyExcludeElGithubEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.github = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.github = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `gsuite`.\n"]
    pub fn set_gsuite(mut self, v: impl Into<BlockAssignable<AccessPolicyExcludeElGsuiteEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gsuite = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gsuite = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `okta`.\n"]
    pub fn set_okta(mut self, v: impl Into<BlockAssignable<AccessPolicyExcludeElOktaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.okta = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.okta = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `saml`.\n"]
    pub fn set_saml(mut self, v: impl Into<BlockAssignable<AccessPolicyExcludeElSamlEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.saml = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.saml = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AccessPolicyExcludeEl {
    type O = BlockAssignable<AccessPolicyExcludeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct AccessPolicyExcludeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyExcludeElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyExcludeElRef {
        AccessPolicyExcludeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyExcludeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `any_valid_service_token` after provisioning.\n"]
    pub fn any_valid_service_token(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.any_valid_service_token", self.base))
    }

    #[doc= "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc= "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc= "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.device_posture", self.base))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email_domain", self.base))
    }

    #[doc= "Get a reference to the value of field `everyone` after provisioning.\n"]
    pub fn everyone(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc= "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc= "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.login_method", self.base))
    }

    #[doc= "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.service_token", self.base))
    }

    #[doc= "Get a reference to the value of field `azure` after provisioning.\n"]
    pub fn azure(&self) -> ListRef<AccessPolicyExcludeElAzureElRef> {
        ListRef::new(self.shared().clone(), format!("{}.azure", self.base))
    }

    #[doc= "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> ListRef<AccessPolicyExcludeElExternalEvaluationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_evaluation", self.base))
    }

    #[doc= "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<AccessPolicyExcludeElGithubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc= "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> ListRef<AccessPolicyExcludeElGsuiteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc= "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> ListRef<AccessPolicyExcludeElOktaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc= "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> ListRef<AccessPolicyExcludeElSamlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyIncludeElAzureEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessPolicyIncludeElAzureEl {
    #[doc= "Set the field `id`.\nThe ID of the Azure group or user."]
    pub fn set_id(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\nThe ID of the Azure Identity provider."]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyIncludeElAzureEl {
    type O = BlockAssignable<AccessPolicyIncludeElAzureEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyIncludeElAzureElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyIncludeElAzureElRef {
        AccessPolicyIncludeElAzureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyIncludeElAzureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the Azure group or user."]
    pub fn id(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of the Azure Identity provider."]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyIncludeElExternalEvaluationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluate_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keys_url: Option<PrimField<String>>,
}

impl AccessPolicyIncludeElExternalEvaluationEl {
    #[doc= "Set the field `evaluate_url`.\n"]
    pub fn set_evaluate_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.evaluate_url = Some(v.into());
        self
    }

    #[doc= "Set the field `keys_url`.\n"]
    pub fn set_keys_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.keys_url = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyIncludeElExternalEvaluationEl {
    type O = BlockAssignable<AccessPolicyIncludeElExternalEvaluationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyIncludeElExternalEvaluationElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyIncludeElExternalEvaluationElRef {
        AccessPolicyIncludeElExternalEvaluationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyIncludeElExternalEvaluationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `evaluate_url` after provisioning.\n"]
    pub fn evaluate_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.evaluate_url", self.base))
    }

    #[doc= "Get a reference to the value of field `keys_url` after provisioning.\n"]
    pub fn keys_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.keys_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyIncludeElGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    teams: Option<ListField<PrimField<String>>>,
}

impl AccessPolicyIncludeElGithubEl {
    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `teams`.\n"]
    pub fn set_teams(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.teams = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyIncludeElGithubEl {
    type O = BlockAssignable<AccessPolicyIncludeElGithubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyIncludeElGithubElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyIncludeElGithubElRef {
        AccessPolicyIncludeElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyIncludeElGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `teams` after provisioning.\n"]
    pub fn teams(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.teams", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyIncludeElGsuiteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessPolicyIncludeElGsuiteEl {
    #[doc= "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyIncludeElGsuiteEl {
    type O = BlockAssignable<AccessPolicyIncludeElGsuiteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyIncludeElGsuiteElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyIncludeElGsuiteElRef {
        AccessPolicyIncludeElGsuiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyIncludeElGsuiteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyIncludeElOktaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<ListField<PrimField<String>>>,
}

impl AccessPolicyIncludeElOktaEl {
    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyIncludeElOktaEl {
    type O = BlockAssignable<AccessPolicyIncludeElOktaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyIncludeElOktaElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyIncludeElOktaElRef {
        AccessPolicyIncludeElOktaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyIncludeElOktaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyIncludeElSamlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessPolicyIncludeElSamlEl {
    #[doc= "Set the field `attribute_name`.\n"]
    pub fn set_attribute_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_name = Some(v.into());
        self
    }

    #[doc= "Set the field `attribute_value`.\n"]
    pub fn set_attribute_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_value = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyIncludeElSamlEl {
    type O = BlockAssignable<AccessPolicyIncludeElSamlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyIncludeElSamlElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyIncludeElSamlElRef {
        AccessPolicyIncludeElSamlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyIncludeElSamlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `attribute_name` after provisioning.\n"]
    pub fn attribute_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc= "Get a reference to the value of field `attribute_value` after provisioning.\n"]
    pub fn attribute_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessPolicyIncludeElDynamic {
    azure: Option<DynamicBlock<AccessPolicyIncludeElAzureEl>>,
    external_evaluation: Option<DynamicBlock<AccessPolicyIncludeElExternalEvaluationEl>>,
    github: Option<DynamicBlock<AccessPolicyIncludeElGithubEl>>,
    gsuite: Option<DynamicBlock<AccessPolicyIncludeElGsuiteEl>>,
    okta: Option<DynamicBlock<AccessPolicyIncludeElOktaEl>>,
    saml: Option<DynamicBlock<AccessPolicyIncludeElSamlEl>>,
}

#[derive(Serialize)]
pub struct AccessPolicyIncludeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    any_valid_service_token: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_posture: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_domain: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    everyone: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geo: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_list: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_method: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_token: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    azure: Option<Vec<AccessPolicyIncludeElAzureEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_evaluation: Option<Vec<AccessPolicyIncludeElExternalEvaluationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<Vec<AccessPolicyIncludeElGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gsuite: Option<Vec<AccessPolicyIncludeElGsuiteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    okta: Option<Vec<AccessPolicyIncludeElOktaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saml: Option<Vec<AccessPolicyIncludeElSamlEl>>,
    dynamic: AccessPolicyIncludeElDynamic,
}

impl AccessPolicyIncludeEl {
    #[doc= "Set the field `any_valid_service_token`.\n"]
    pub fn set_any_valid_service_token(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.any_valid_service_token = Some(v.into());
        self
    }

    #[doc= "Set the field `auth_method`.\n"]
    pub fn set_auth_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_method = Some(v.into());
        self
    }

    #[doc= "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.common_name = Some(v.into());
        self
    }

    #[doc= "Set the field `device_posture`.\n"]
    pub fn set_device_posture(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.device_posture = Some(v.into());
        self
    }

    #[doc= "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `email_domain`.\n"]
    pub fn set_email_domain(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email_domain = Some(v.into());
        self
    }

    #[doc= "Set the field `everyone`.\n"]
    pub fn set_everyone(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.everyone = Some(v.into());
        self
    }

    #[doc= "Set the field `geo`.\n"]
    pub fn set_geo(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.geo = Some(v.into());
        self
    }

    #[doc= "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.group = Some(v.into());
        self
    }

    #[doc= "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_list`.\n"]
    pub fn set_ip_list(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_list = Some(v.into());
        self
    }

    #[doc= "Set the field `login_method`.\n"]
    pub fn set_login_method(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.login_method = Some(v.into());
        self
    }

    #[doc= "Set the field `service_token`.\n"]
    pub fn set_service_token(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.service_token = Some(v.into());
        self
    }

    #[doc= "Set the field `azure`.\n"]
    pub fn set_azure(mut self, v: impl Into<BlockAssignable<AccessPolicyIncludeElAzureEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.azure = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.azure = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `external_evaluation`.\n"]
    pub fn set_external_evaluation(
        mut self,
        v: impl Into<BlockAssignable<AccessPolicyIncludeElExternalEvaluationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.external_evaluation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.external_evaluation = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl Into<BlockAssignable<AccessPolicyIncludeElGithubEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.github = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.github = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `gsuite`.\n"]
    pub fn set_gsuite(mut self, v: impl Into<BlockAssignable<AccessPolicyIncludeElGsuiteEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gsuite = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gsuite = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `okta`.\n"]
    pub fn set_okta(mut self, v: impl Into<BlockAssignable<AccessPolicyIncludeElOktaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.okta = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.okta = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `saml`.\n"]
    pub fn set_saml(mut self, v: impl Into<BlockAssignable<AccessPolicyIncludeElSamlEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.saml = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.saml = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AccessPolicyIncludeEl {
    type O = BlockAssignable<AccessPolicyIncludeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct AccessPolicyIncludeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyIncludeElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyIncludeElRef {
        AccessPolicyIncludeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyIncludeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `any_valid_service_token` after provisioning.\n"]
    pub fn any_valid_service_token(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.any_valid_service_token", self.base))
    }

    #[doc= "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc= "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc= "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.device_posture", self.base))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email_domain", self.base))
    }

    #[doc= "Get a reference to the value of field `everyone` after provisioning.\n"]
    pub fn everyone(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc= "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc= "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.login_method", self.base))
    }

    #[doc= "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.service_token", self.base))
    }

    #[doc= "Get a reference to the value of field `azure` after provisioning.\n"]
    pub fn azure(&self) -> ListRef<AccessPolicyIncludeElAzureElRef> {
        ListRef::new(self.shared().clone(), format!("{}.azure", self.base))
    }

    #[doc= "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> ListRef<AccessPolicyIncludeElExternalEvaluationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_evaluation", self.base))
    }

    #[doc= "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<AccessPolicyIncludeElGithubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc= "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> ListRef<AccessPolicyIncludeElGsuiteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc= "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> ListRef<AccessPolicyIncludeElOktaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc= "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> ListRef<AccessPolicyIncludeElSamlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyRequireElAzureEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessPolicyRequireElAzureEl {
    #[doc= "Set the field `id`.\nThe ID of the Azure group or user."]
    pub fn set_id(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\nThe ID of the Azure Identity provider."]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyRequireElAzureEl {
    type O = BlockAssignable<AccessPolicyRequireElAzureEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyRequireElAzureElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyRequireElAzureElRef {
        AccessPolicyRequireElAzureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyRequireElAzureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the Azure group or user."]
    pub fn id(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of the Azure Identity provider."]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyRequireElExternalEvaluationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluate_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keys_url: Option<PrimField<String>>,
}

impl AccessPolicyRequireElExternalEvaluationEl {
    #[doc= "Set the field `evaluate_url`.\n"]
    pub fn set_evaluate_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.evaluate_url = Some(v.into());
        self
    }

    #[doc= "Set the field `keys_url`.\n"]
    pub fn set_keys_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.keys_url = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyRequireElExternalEvaluationEl {
    type O = BlockAssignable<AccessPolicyRequireElExternalEvaluationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyRequireElExternalEvaluationElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyRequireElExternalEvaluationElRef {
        AccessPolicyRequireElExternalEvaluationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyRequireElExternalEvaluationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `evaluate_url` after provisioning.\n"]
    pub fn evaluate_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.evaluate_url", self.base))
    }

    #[doc= "Get a reference to the value of field `keys_url` after provisioning.\n"]
    pub fn keys_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.keys_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyRequireElGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    teams: Option<ListField<PrimField<String>>>,
}

impl AccessPolicyRequireElGithubEl {
    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `teams`.\n"]
    pub fn set_teams(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.teams = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyRequireElGithubEl {
    type O = BlockAssignable<AccessPolicyRequireElGithubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyRequireElGithubElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyRequireElGithubElRef {
        AccessPolicyRequireElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyRequireElGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `teams` after provisioning.\n"]
    pub fn teams(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.teams", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyRequireElGsuiteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessPolicyRequireElGsuiteEl {
    #[doc= "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyRequireElGsuiteEl {
    type O = BlockAssignable<AccessPolicyRequireElGsuiteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyRequireElGsuiteElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyRequireElGsuiteElRef {
        AccessPolicyRequireElGsuiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyRequireElGsuiteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyRequireElOktaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<ListField<PrimField<String>>>,
}

impl AccessPolicyRequireElOktaEl {
    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyRequireElOktaEl {
    type O = BlockAssignable<AccessPolicyRequireElOktaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyRequireElOktaElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyRequireElOktaElRef {
        AccessPolicyRequireElOktaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyRequireElOktaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessPolicyRequireElSamlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessPolicyRequireElSamlEl {
    #[doc= "Set the field `attribute_name`.\n"]
    pub fn set_attribute_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_name = Some(v.into());
        self
    }

    #[doc= "Set the field `attribute_value`.\n"]
    pub fn set_attribute_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_value = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessPolicyRequireElSamlEl {
    type O = BlockAssignable<AccessPolicyRequireElSamlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyRequireElSamlElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyRequireElSamlElRef {
        AccessPolicyRequireElSamlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyRequireElSamlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `attribute_name` after provisioning.\n"]
    pub fn attribute_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc= "Get a reference to the value of field `attribute_value` after provisioning.\n"]
    pub fn attribute_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessPolicyRequireElDynamic {
    azure: Option<DynamicBlock<AccessPolicyRequireElAzureEl>>,
    external_evaluation: Option<DynamicBlock<AccessPolicyRequireElExternalEvaluationEl>>,
    github: Option<DynamicBlock<AccessPolicyRequireElGithubEl>>,
    gsuite: Option<DynamicBlock<AccessPolicyRequireElGsuiteEl>>,
    okta: Option<DynamicBlock<AccessPolicyRequireElOktaEl>>,
    saml: Option<DynamicBlock<AccessPolicyRequireElSamlEl>>,
}

#[derive(Serialize)]
pub struct AccessPolicyRequireEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    any_valid_service_token: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_posture: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_domain: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    everyone: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geo: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_list: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_method: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_token: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    azure: Option<Vec<AccessPolicyRequireElAzureEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_evaluation: Option<Vec<AccessPolicyRequireElExternalEvaluationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<Vec<AccessPolicyRequireElGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gsuite: Option<Vec<AccessPolicyRequireElGsuiteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    okta: Option<Vec<AccessPolicyRequireElOktaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saml: Option<Vec<AccessPolicyRequireElSamlEl>>,
    dynamic: AccessPolicyRequireElDynamic,
}

impl AccessPolicyRequireEl {
    #[doc= "Set the field `any_valid_service_token`.\n"]
    pub fn set_any_valid_service_token(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.any_valid_service_token = Some(v.into());
        self
    }

    #[doc= "Set the field `auth_method`.\n"]
    pub fn set_auth_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_method = Some(v.into());
        self
    }

    #[doc= "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.common_name = Some(v.into());
        self
    }

    #[doc= "Set the field `device_posture`.\n"]
    pub fn set_device_posture(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.device_posture = Some(v.into());
        self
    }

    #[doc= "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `email_domain`.\n"]
    pub fn set_email_domain(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email_domain = Some(v.into());
        self
    }

    #[doc= "Set the field `everyone`.\n"]
    pub fn set_everyone(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.everyone = Some(v.into());
        self
    }

    #[doc= "Set the field `geo`.\n"]
    pub fn set_geo(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.geo = Some(v.into());
        self
    }

    #[doc= "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.group = Some(v.into());
        self
    }

    #[doc= "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_list`.\n"]
    pub fn set_ip_list(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_list = Some(v.into());
        self
    }

    #[doc= "Set the field `login_method`.\n"]
    pub fn set_login_method(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.login_method = Some(v.into());
        self
    }

    #[doc= "Set the field `service_token`.\n"]
    pub fn set_service_token(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.service_token = Some(v.into());
        self
    }

    #[doc= "Set the field `azure`.\n"]
    pub fn set_azure(mut self, v: impl Into<BlockAssignable<AccessPolicyRequireElAzureEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.azure = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.azure = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `external_evaluation`.\n"]
    pub fn set_external_evaluation(
        mut self,
        v: impl Into<BlockAssignable<AccessPolicyRequireElExternalEvaluationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.external_evaluation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.external_evaluation = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl Into<BlockAssignable<AccessPolicyRequireElGithubEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.github = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.github = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `gsuite`.\n"]
    pub fn set_gsuite(mut self, v: impl Into<BlockAssignable<AccessPolicyRequireElGsuiteEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gsuite = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gsuite = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `okta`.\n"]
    pub fn set_okta(mut self, v: impl Into<BlockAssignable<AccessPolicyRequireElOktaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.okta = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.okta = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `saml`.\n"]
    pub fn set_saml(mut self, v: impl Into<BlockAssignable<AccessPolicyRequireElSamlEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.saml = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.saml = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AccessPolicyRequireEl {
    type O = BlockAssignable<AccessPolicyRequireEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct AccessPolicyRequireElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessPolicyRequireElRef {
    fn new(shared: StackShared, base: String) -> AccessPolicyRequireElRef {
        AccessPolicyRequireElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessPolicyRequireElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `any_valid_service_token` after provisioning.\n"]
    pub fn any_valid_service_token(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.any_valid_service_token", self.base))
    }

    #[doc= "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc= "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc= "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.device_posture", self.base))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email_domain", self.base))
    }

    #[doc= "Get a reference to the value of field `everyone` after provisioning.\n"]
    pub fn everyone(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc= "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc= "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.login_method", self.base))
    }

    #[doc= "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.service_token", self.base))
    }

    #[doc= "Get a reference to the value of field `azure` after provisioning.\n"]
    pub fn azure(&self) -> ListRef<AccessPolicyRequireElAzureElRef> {
        ListRef::new(self.shared().clone(), format!("{}.azure", self.base))
    }

    #[doc= "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> ListRef<AccessPolicyRequireElExternalEvaluationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_evaluation", self.base))
    }

    #[doc= "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<AccessPolicyRequireElGithubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc= "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> ListRef<AccessPolicyRequireElGsuiteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc= "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> ListRef<AccessPolicyRequireElOktaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc= "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> ListRef<AccessPolicyRequireElSamlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessPolicyDynamic {
    approval_group: Option<DynamicBlock<AccessPolicyApprovalGroupEl>>,
    exclude: Option<DynamicBlock<AccessPolicyExcludeEl>>,
    include: Option<DynamicBlock<AccessPolicyIncludeEl>>,
    require: Option<DynamicBlock<AccessPolicyRequireEl>>,
}
