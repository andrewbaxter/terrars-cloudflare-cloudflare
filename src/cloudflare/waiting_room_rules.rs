use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct WaitingRoomRulesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    waiting_room_id: PrimField<String>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<WaitingRoomRulesRulesEl>>,
    dynamic: WaitingRoomRulesDynamic,
}

struct WaitingRoomRules_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WaitingRoomRulesData>,
}

#[derive(Clone)]
pub struct WaitingRoomRules(Rc<WaitingRoomRules_>);

impl WaitingRoomRules {
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

    #[doc= "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl Into<BlockAssignable<WaitingRoomRulesRulesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `waiting_room_id` after provisioning.\nThe Waiting Room ID the rules should apply to. **Modifying this attribute will force creation of a new resource.**"]
    pub fn waiting_room_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.waiting_room_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<WaitingRoomRulesRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

impl Referable for WaitingRoomRules {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for WaitingRoomRules { }

impl ToListMappable for WaitingRoomRules {
    type O = ListRef<WaitingRoomRulesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for WaitingRoomRules_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_waiting_room_rules".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWaitingRoomRules {
    pub tf_id: String,
    #[doc= "The Waiting Room ID the rules should apply to. **Modifying this attribute will force creation of a new resource.**"]
    pub waiting_room_id: PrimField<String>,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildWaitingRoomRules {
    pub fn build(self, stack: &mut Stack) -> WaitingRoomRules {
        let out = WaitingRoomRules(Rc::new(WaitingRoomRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WaitingRoomRulesData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                waiting_room_id: self.waiting_room_id,
                zone_id: self.zone_id,
                rules: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WaitingRoomRulesRef {
    shared: StackShared,
    base: String,
}

impl Ref for WaitingRoomRulesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WaitingRoomRulesRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `waiting_room_id` after provisioning.\nThe Waiting Room ID the rules should apply to. **Modifying this attribute will force creation of a new resource.**"]
    pub fn waiting_room_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.waiting_room_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<WaitingRoomRulesRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WaitingRoomRulesRulesEl {
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl WaitingRoomRulesRulesEl {
    #[doc= "Set the field `description`.\nBrief summary of the waiting room rule and its intended use."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\nWhether the rule is enabled or disabled. Available values: `enabled`, `disabled`."]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for WaitingRoomRulesRulesEl {
    type O = BlockAssignable<WaitingRoomRulesRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWaitingRoomRulesRulesEl {
    #[doc= "Action to perform in the ruleset rule. Available values: `bypass_waiting_room`."]
    pub action: PrimField<String>,
    #[doc= "Criteria for an HTTP request to trigger the waiting room rule action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Waiting Room Rules Docs](https://developers.cloudflare.com/waiting-room/additional-options/waiting-room-rules/bypass-rules/)."]
    pub expression: PrimField<String>,
}

impl BuildWaitingRoomRulesRulesEl {
    pub fn build(self) -> WaitingRoomRulesRulesEl {
        WaitingRoomRulesRulesEl {
            action: self.action,
            description: core::default::Default::default(),
            expression: self.expression,
            status: core::default::Default::default(),
        }
    }
}

pub struct WaitingRoomRulesRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WaitingRoomRulesRulesElRef {
    fn new(shared: StackShared, base: String) -> WaitingRoomRulesRulesElRef {
        WaitingRoomRulesRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WaitingRoomRulesRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\nAction to perform in the ruleset rule. Available values: `bypass_waiting_room`."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nBrief summary of the waiting room rule and its intended use."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `expression` after provisioning.\nCriteria for an HTTP request to trigger the waiting room rule action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Waiting Room Rules Docs](https://developers.cloudflare.com/waiting-room/additional-options/waiting-room-rules/bypass-rules/)."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique rule identifier."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nWhether the rule is enabled or disabled. Available values: `enabled`, `disabled`."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\nVersion of the waiting room rule."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct WaitingRoomRulesDynamic {
    rules: Option<DynamicBlock<WaitingRoomRulesRulesEl>>,
}
