use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct UserAgentBlockingRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    description: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    mode: PrimField<String>,
    paused: PrimField<bool>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<UserAgentBlockingRuleConfigurationEl>>,
    dynamic: UserAgentBlockingRuleDynamic,
}

struct UserAgentBlockingRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<UserAgentBlockingRuleData>,
}

#[derive(Clone)]
pub struct UserAgentBlockingRule(Rc<UserAgentBlockingRule_>);

impl UserAgentBlockingRule {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `configuration`.\n"]
    pub fn set_configuration(self, v: impl Into<BlockAssignable<UserAgentBlockingRuleConfigurationEl>>) -> Self {
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

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn informative summary of the rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nThe action to apply to a matched request. Available values: `block`, `challenge`, `js_challenge`, `managed_challenge`."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `paused` after provisioning.\nWhen true, indicates that the rule is currently paused."]
    pub fn paused(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<UserAgentBlockingRuleConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

impl Referable for UserAgentBlockingRule {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for UserAgentBlockingRule { }

impl ToListMappable for UserAgentBlockingRule {
    type O = ListRef<UserAgentBlockingRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for UserAgentBlockingRule_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_user_agent_blocking_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildUserAgentBlockingRule {
    pub tf_id: String,
    #[doc= "An informative summary of the rule."]
    pub description: PrimField<String>,
    #[doc= "The action to apply to a matched request. Available values: `block`, `challenge`, `js_challenge`, `managed_challenge`."]
    pub mode: PrimField<String>,
    #[doc= "When true, indicates that the rule is currently paused."]
    pub paused: PrimField<bool>,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildUserAgentBlockingRule {
    pub fn build(self, stack: &mut Stack) -> UserAgentBlockingRule {
        let out = UserAgentBlockingRule(Rc::new(UserAgentBlockingRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(UserAgentBlockingRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: self.description,
                id: core::default::Default::default(),
                mode: self.mode,
                paused: self.paused,
                zone_id: self.zone_id,
                configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct UserAgentBlockingRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for UserAgentBlockingRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl UserAgentBlockingRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn informative summary of the rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nThe action to apply to a matched request. Available values: `block`, `challenge`, `js_challenge`, `managed_challenge`."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `paused` after provisioning.\nWhen true, indicates that the rule is currently paused."]
    pub fn paused(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<UserAgentBlockingRuleConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct UserAgentBlockingRuleConfigurationEl {
    target: PrimField<String>,
    value: PrimField<String>,
}

impl UserAgentBlockingRuleConfigurationEl { }

impl ToListMappable for UserAgentBlockingRuleConfigurationEl {
    type O = BlockAssignable<UserAgentBlockingRuleConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildUserAgentBlockingRuleConfigurationEl {
    #[doc= "The configuration target for this rule. You must set the target to ua for User Agent Blocking rules."]
    pub target: PrimField<String>,
    #[doc= "The exact user agent string to match. This value will be compared to the received User-Agent HTTP header value."]
    pub value: PrimField<String>,
}

impl BuildUserAgentBlockingRuleConfigurationEl {
    pub fn build(self) -> UserAgentBlockingRuleConfigurationEl {
        UserAgentBlockingRuleConfigurationEl {
            target: self.target,
            value: self.value,
        }
    }
}

pub struct UserAgentBlockingRuleConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for UserAgentBlockingRuleConfigurationElRef {
    fn new(shared: StackShared, base: String) -> UserAgentBlockingRuleConfigurationElRef {
        UserAgentBlockingRuleConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl UserAgentBlockingRuleConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\nThe configuration target for this rule. You must set the target to ua for User Agent Blocking rules."]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe exact user agent string to match. This value will be compared to the received User-Agent HTTP header value."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct UserAgentBlockingRuleDynamic {
    configuration: Option<DynamicBlock<UserAgentBlockingRuleConfigurationEl>>,
}
