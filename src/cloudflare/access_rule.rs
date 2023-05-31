use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AccessRuleData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<AccessRuleConfigurationEl>>,
    dynamic: AccessRuleDynamic,
}

struct AccessRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessRuleData>,
}

#[derive(Clone)]
pub struct AccessRule(Rc<AccessRule_>);

impl AccessRule {
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

    #[doc= "Set the field `account_id`.\nThe account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `notes`.\nA personal note about the rule. Typically used as a reminder or explanation for the rule."]
    pub fn set_notes(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().notes = Some(v.into());
        self
    }

    #[doc= "Set the field `zone_id`.\nThe zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_zone_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone_id = Some(v.into());
        self
    }

    #[doc= "Set the field `configuration`.\n"]
    pub fn set_configuration(self, v: impl Into<BlockAssignable<AccessRuleConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nThe action to apply to a matched request. Available values: `block`, `challenge`, `whitelist`, `js_challenge`, `managed_challenge`."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notes` after provisioning.\nA personal note about the rule. Typically used as a reminder or explanation for the rule."]
    pub fn notes(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.notes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<AccessRuleConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

impl Referable for AccessRule {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AccessRule { }

impl ToListMappable for AccessRule {
    type O = ListRef<AccessRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AccessRule_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_access_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessRule {
    pub tf_id: String,
    #[doc= "The action to apply to a matched request. Available values: `block`, `challenge`, `whitelist`, `js_challenge`, `managed_challenge`."]
    pub mode: PrimField<String>,
}

impl BuildAccessRule {
    pub fn build(self, stack: &mut Stack) -> AccessRule {
        let out = AccessRule(Rc::new(AccessRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AccessRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                mode: self.mode,
                notes: core::default::Default::default(),
                zone_id: core::default::Default::default(),
                configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nThe action to apply to a matched request. Available values: `block`, `challenge`, `whitelist`, `js_challenge`, `managed_challenge`."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notes` after provisioning.\nA personal note about the rule. Typically used as a reminder or explanation for the rule."]
    pub fn notes(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.notes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<AccessRuleConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AccessRuleConfigurationEl {
    target: PrimField<String>,
    value: PrimField<String>,
}

impl AccessRuleConfigurationEl { }

impl ToListMappable for AccessRuleConfigurationEl {
    type O = BlockAssignable<AccessRuleConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessRuleConfigurationEl {
    #[doc= "The request property to target. Available values: `ip`, `ip6`, `ip_range`, `asn`, `country`. **Modifying this attribute will force creation of a new resource.**"]
    pub target: PrimField<String>,
    #[doc= "The value to target. Depends on target's type. **Modifying this attribute will force creation of a new resource.**"]
    pub value: PrimField<String>,
}

impl BuildAccessRuleConfigurationEl {
    pub fn build(self) -> AccessRuleConfigurationEl {
        AccessRuleConfigurationEl {
            target: self.target,
            value: self.value,
        }
    }
}

pub struct AccessRuleConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessRuleConfigurationElRef {
    fn new(shared: StackShared, base: String) -> AccessRuleConfigurationElRef {
        AccessRuleConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessRuleConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\nThe request property to target. Available values: `ip`, `ip6`, `ip_range`, `asn`, `country`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe value to target. Depends on target's type. **Modifying this attribute will force creation of a new resource.**"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessRuleDynamic {
    configuration: Option<DynamicBlock<AccessRuleConfigurationEl>>,
}
