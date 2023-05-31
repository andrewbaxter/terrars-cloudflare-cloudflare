use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct TeamsRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    action: PrimField<String>,
    description: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_posture: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity: Option<PrimField<String>>,
    name: PrimField<String>,
    precedence: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traffic: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_settings: Option<Vec<TeamsRuleRuleSettingsEl>>,
    dynamic: TeamsRuleDynamic,
}

struct TeamsRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TeamsRuleData>,
}

#[derive(Clone)]
pub struct TeamsRule(Rc<TeamsRule_>);

impl TeamsRule {
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

    #[doc= "Set the field `device_posture`.\nThe wirefilter expression to be used for device_posture check matching."]
    pub fn set_device_posture(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().device_posture = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\nIndicator of rule enablement."]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `filters`.\nThe protocol or layer to evaluate the traffic and identity expressions."]
    pub fn set_filters(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().filters = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `identity`.\nThe wirefilter expression to be used for identity matching."]
    pub fn set_identity(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().identity = Some(v.into());
        self
    }

    #[doc= "Set the field `traffic`.\nThe wirefilter expression to be used for traffic matching."]
    pub fn set_traffic(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().traffic = Some(v.into());
        self
    }

    #[doc= "Set the field `rule_settings`.\n"]
    pub fn set_rule_settings(self, v: impl Into<BlockAssignable<TeamsRuleRuleSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\nThe action executed by matched teams rule. Available values: `allow`, `block`, `safesearch`, `ytrestricted`, `on`, `off`, `scan`, `noscan`, `isolate`, `noisolate`, `override`, `l4_override`, `egress`, `audit_ssh`."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of the teams rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `device_posture` after provisioning.\nThe wirefilter expression to be used for device_posture check matching."]
    pub fn device_posture(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_posture", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nIndicator of rule enablement."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filters` after provisioning.\nThe protocol or layer to evaluate the traffic and identity expressions."]
    pub fn filters(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `identity` after provisioning.\nThe wirefilter expression to be used for identity matching."]
    pub fn identity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the teams rule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `precedence` after provisioning.\nThe evaluation precedence of the teams rule."]
    pub fn precedence(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.precedence", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `traffic` after provisioning.\nThe wirefilter expression to be used for traffic matching."]
    pub fn traffic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.traffic", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rule_settings` after provisioning.\n"]
    pub fn rule_settings(&self) -> ListRef<TeamsRuleRuleSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule_settings", self.extract_ref()))
    }
}

impl Referable for TeamsRule {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for TeamsRule { }

impl ToListMappable for TeamsRule {
    type O = ListRef<TeamsRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TeamsRule_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_teams_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTeamsRule {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
    #[doc= "The action executed by matched teams rule. Available values: `allow`, `block`, `safesearch`, `ytrestricted`, `on`, `off`, `scan`, `noscan`, `isolate`, `noisolate`, `override`, `l4_override`, `egress`, `audit_ssh`."]
    pub action: PrimField<String>,
    #[doc= "The description of the teams rule."]
    pub description: PrimField<String>,
    #[doc= "The name of the teams rule."]
    pub name: PrimField<String>,
    #[doc= "The evaluation precedence of the teams rule."]
    pub precedence: PrimField<f64>,
}

impl BuildTeamsRule {
    pub fn build(self, stack: &mut Stack) -> TeamsRule {
        let out = TeamsRule(Rc::new(TeamsRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TeamsRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                action: self.action,
                description: self.description,
                device_posture: core::default::Default::default(),
                enabled: core::default::Default::default(),
                filters: core::default::Default::default(),
                id: core::default::Default::default(),
                identity: core::default::Default::default(),
                name: self.name,
                precedence: self.precedence,
                traffic: core::default::Default::default(),
                rule_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TeamsRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TeamsRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\nThe action executed by matched teams rule. Available values: `allow`, `block`, `safesearch`, `ytrestricted`, `on`, `off`, `scan`, `noscan`, `isolate`, `noisolate`, `override`, `l4_override`, `egress`, `audit_ssh`."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of the teams rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `device_posture` after provisioning.\nThe wirefilter expression to be used for device_posture check matching."]
    pub fn device_posture(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_posture", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nIndicator of rule enablement."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filters` after provisioning.\nThe protocol or layer to evaluate the traffic and identity expressions."]
    pub fn filters(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `identity` after provisioning.\nThe wirefilter expression to be used for identity matching."]
    pub fn identity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the teams rule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `precedence` after provisioning.\nThe evaluation precedence of the teams rule."]
    pub fn precedence(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.precedence", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `traffic` after provisioning.\nThe wirefilter expression to be used for traffic matching."]
    pub fn traffic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.traffic", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rule_settings` after provisioning.\n"]
    pub fn rule_settings(&self) -> ListRef<TeamsRuleRuleSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TeamsRuleRuleSettingsElAuditSshEl {
    command_logging: PrimField<bool>,
}

impl TeamsRuleRuleSettingsElAuditSshEl { }

impl ToListMappable for TeamsRuleRuleSettingsElAuditSshEl {
    type O = BlockAssignable<TeamsRuleRuleSettingsElAuditSshEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsRuleRuleSettingsElAuditSshEl {
    #[doc= "Log all SSH commands."]
    pub command_logging: PrimField<bool>,
}

impl BuildTeamsRuleRuleSettingsElAuditSshEl {
    pub fn build(self) -> TeamsRuleRuleSettingsElAuditSshEl {
        TeamsRuleRuleSettingsElAuditSshEl { command_logging: self.command_logging }
    }
}

pub struct TeamsRuleRuleSettingsElAuditSshElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsRuleRuleSettingsElAuditSshElRef {
    fn new(shared: StackShared, base: String) -> TeamsRuleRuleSettingsElAuditSshElRef {
        TeamsRuleRuleSettingsElAuditSshElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsRuleRuleSettingsElAuditSshElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `command_logging` after provisioning.\nLog all SSH commands."]
    pub fn command_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.command_logging", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsRuleRuleSettingsElBisoAdminControlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_copy_paste: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_download: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_keyboard: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_printing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_upload: Option<PrimField<bool>>,
}

impl TeamsRuleRuleSettingsElBisoAdminControlsEl {
    #[doc= "Set the field `disable_copy_paste`.\nDisable copy-paste."]
    pub fn set_disable_copy_paste(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_copy_paste = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_download`.\nDisable download."]
    pub fn set_disable_download(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_download = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_keyboard`.\nDisable keyboard usage."]
    pub fn set_disable_keyboard(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_keyboard = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_printing`.\nDisable printing."]
    pub fn set_disable_printing(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_printing = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_upload`.\nDisable upload."]
    pub fn set_disable_upload(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_upload = Some(v.into());
        self
    }
}

impl ToListMappable for TeamsRuleRuleSettingsElBisoAdminControlsEl {
    type O = BlockAssignable<TeamsRuleRuleSettingsElBisoAdminControlsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsRuleRuleSettingsElBisoAdminControlsEl {}

impl BuildTeamsRuleRuleSettingsElBisoAdminControlsEl {
    pub fn build(self) -> TeamsRuleRuleSettingsElBisoAdminControlsEl {
        TeamsRuleRuleSettingsElBisoAdminControlsEl {
            disable_copy_paste: core::default::Default::default(),
            disable_download: core::default::Default::default(),
            disable_keyboard: core::default::Default::default(),
            disable_printing: core::default::Default::default(),
            disable_upload: core::default::Default::default(),
        }
    }
}

pub struct TeamsRuleRuleSettingsElBisoAdminControlsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsRuleRuleSettingsElBisoAdminControlsElRef {
    fn new(shared: StackShared, base: String) -> TeamsRuleRuleSettingsElBisoAdminControlsElRef {
        TeamsRuleRuleSettingsElBisoAdminControlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsRuleRuleSettingsElBisoAdminControlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disable_copy_paste` after provisioning.\nDisable copy-paste."]
    pub fn disable_copy_paste(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_copy_paste", self.base))
    }

    #[doc= "Get a reference to the value of field `disable_download` after provisioning.\nDisable download."]
    pub fn disable_download(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_download", self.base))
    }

    #[doc= "Get a reference to the value of field `disable_keyboard` after provisioning.\nDisable keyboard usage."]
    pub fn disable_keyboard(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_keyboard", self.base))
    }

    #[doc= "Get a reference to the value of field `disable_printing` after provisioning.\nDisable printing."]
    pub fn disable_printing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_printing", self.base))
    }

    #[doc= "Get a reference to the value of field `disable_upload` after provisioning.\nDisable upload."]
    pub fn disable_upload(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_upload", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsRuleRuleSettingsElCheckSessionEl {
    duration: PrimField<String>,
    enforce: PrimField<bool>,
}

impl TeamsRuleRuleSettingsElCheckSessionEl { }

impl ToListMappable for TeamsRuleRuleSettingsElCheckSessionEl {
    type O = BlockAssignable<TeamsRuleRuleSettingsElCheckSessionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsRuleRuleSettingsElCheckSessionEl {
    #[doc= "Configure how fresh the session needs to be to be considered valid."]
    pub duration: PrimField<String>,
    #[doc= "Enable session enforcement for this rule."]
    pub enforce: PrimField<bool>,
}

impl BuildTeamsRuleRuleSettingsElCheckSessionEl {
    pub fn build(self) -> TeamsRuleRuleSettingsElCheckSessionEl {
        TeamsRuleRuleSettingsElCheckSessionEl {
            duration: self.duration,
            enforce: self.enforce,
        }
    }
}

pub struct TeamsRuleRuleSettingsElCheckSessionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsRuleRuleSettingsElCheckSessionElRef {
    fn new(shared: StackShared, base: String) -> TeamsRuleRuleSettingsElCheckSessionElRef {
        TeamsRuleRuleSettingsElCheckSessionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsRuleRuleSettingsElCheckSessionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `duration` after provisioning.\nConfigure how fresh the session needs to be to be considered valid."]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc= "Get a reference to the value of field `enforce` after provisioning.\nEnable session enforcement for this rule."]
    pub fn enforce(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsRuleRuleSettingsElEgressEl {
    ipv4: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_fallback: Option<PrimField<String>>,
    ipv6: PrimField<String>,
}

impl TeamsRuleRuleSettingsElEgressEl {
    #[doc= "Set the field `ipv4_fallback`.\nThe IPv4 address to be used for egress in the event of an error egressing with the primary IPv4. Can be '0.0.0.0' to indicate local egreass via Warp IPs."]
    pub fn set_ipv4_fallback(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv4_fallback = Some(v.into());
        self
    }
}

impl ToListMappable for TeamsRuleRuleSettingsElEgressEl {
    type O = BlockAssignable<TeamsRuleRuleSettingsElEgressEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsRuleRuleSettingsElEgressEl {
    #[doc= "The IPv4 address to be used for egress."]
    pub ipv4: PrimField<String>,
    #[doc= "The IPv6 range to be used for egress."]
    pub ipv6: PrimField<String>,
}

impl BuildTeamsRuleRuleSettingsElEgressEl {
    pub fn build(self) -> TeamsRuleRuleSettingsElEgressEl {
        TeamsRuleRuleSettingsElEgressEl {
            ipv4: self.ipv4,
            ipv4_fallback: core::default::Default::default(),
            ipv6: self.ipv6,
        }
    }
}

pub struct TeamsRuleRuleSettingsElEgressElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsRuleRuleSettingsElEgressElRef {
    fn new(shared: StackShared, base: String) -> TeamsRuleRuleSettingsElEgressElRef {
        TeamsRuleRuleSettingsElEgressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsRuleRuleSettingsElEgressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ipv4` after provisioning.\nThe IPv4 address to be used for egress."]
    pub fn ipv4(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv4_fallback` after provisioning.\nThe IPv4 address to be used for egress in the event of an error egressing with the primary IPv4. Can be '0.0.0.0' to indicate local egreass via Warp IPs."]
    pub fn ipv4_fallback(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_fallback", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6` after provisioning.\nThe IPv6 range to be used for egress."]
    pub fn ipv6(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsRuleRuleSettingsElL4overrideEl {
    ip: PrimField<String>,
    port: PrimField<f64>,
}

impl TeamsRuleRuleSettingsElL4overrideEl { }

impl ToListMappable for TeamsRuleRuleSettingsElL4overrideEl {
    type O = BlockAssignable<TeamsRuleRuleSettingsElL4overrideEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsRuleRuleSettingsElL4overrideEl {
    #[doc= "Override IP to forward traffic to."]
    pub ip: PrimField<String>,
    #[doc= "Override Port to forward traffic to."]
    pub port: PrimField<f64>,
}

impl BuildTeamsRuleRuleSettingsElL4overrideEl {
    pub fn build(self) -> TeamsRuleRuleSettingsElL4overrideEl {
        TeamsRuleRuleSettingsElL4overrideEl {
            ip: self.ip,
            port: self.port,
        }
    }
}

pub struct TeamsRuleRuleSettingsElL4overrideElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsRuleRuleSettingsElL4overrideElRef {
    fn new(shared: StackShared, base: String) -> TeamsRuleRuleSettingsElL4overrideElRef {
        TeamsRuleRuleSettingsElL4overrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsRuleRuleSettingsElL4overrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\nOverride IP to forward traffic to."]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\nOverride Port to forward traffic to."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsRuleRuleSettingsElPayloadLogEl {
    enabled: PrimField<bool>,
}

impl TeamsRuleRuleSettingsElPayloadLogEl { }

impl ToListMappable for TeamsRuleRuleSettingsElPayloadLogEl {
    type O = BlockAssignable<TeamsRuleRuleSettingsElPayloadLogEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsRuleRuleSettingsElPayloadLogEl {
    #[doc= "Enable or disable DLP Payload Logging for this rule."]
    pub enabled: PrimField<bool>,
}

impl BuildTeamsRuleRuleSettingsElPayloadLogEl {
    pub fn build(self) -> TeamsRuleRuleSettingsElPayloadLogEl {
        TeamsRuleRuleSettingsElPayloadLogEl { enabled: self.enabled }
    }
}

pub struct TeamsRuleRuleSettingsElPayloadLogElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsRuleRuleSettingsElPayloadLogElRef {
    fn new(shared: StackShared, base: String) -> TeamsRuleRuleSettingsElPayloadLogElRef {
        TeamsRuleRuleSettingsElPayloadLogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsRuleRuleSettingsElPayloadLogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nEnable or disable DLP Payload Logging for this rule."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsRuleRuleSettingsElUntrustedCertEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
}

impl TeamsRuleRuleSettingsElUntrustedCertEl {
    #[doc= "Set the field `action`.\nAction to be taken when the SSL certificate of upstream is invalid. Available values: `pass_through`, `block`, `error`."]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }
}

impl ToListMappable for TeamsRuleRuleSettingsElUntrustedCertEl {
    type O = BlockAssignable<TeamsRuleRuleSettingsElUntrustedCertEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsRuleRuleSettingsElUntrustedCertEl {}

impl BuildTeamsRuleRuleSettingsElUntrustedCertEl {
    pub fn build(self) -> TeamsRuleRuleSettingsElUntrustedCertEl {
        TeamsRuleRuleSettingsElUntrustedCertEl { action: core::default::Default::default() }
    }
}

pub struct TeamsRuleRuleSettingsElUntrustedCertElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsRuleRuleSettingsElUntrustedCertElRef {
    fn new(shared: StackShared, base: String) -> TeamsRuleRuleSettingsElUntrustedCertElRef {
        TeamsRuleRuleSettingsElUntrustedCertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsRuleRuleSettingsElUntrustedCertElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\nAction to be taken when the SSL certificate of upstream is invalid. Available values: `pass_through`, `block`, `error`."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize, Default)]
struct TeamsRuleRuleSettingsElDynamic {
    audit_ssh: Option<DynamicBlock<TeamsRuleRuleSettingsElAuditSshEl>>,
    biso_admin_controls: Option<DynamicBlock<TeamsRuleRuleSettingsElBisoAdminControlsEl>>,
    check_session: Option<DynamicBlock<TeamsRuleRuleSettingsElCheckSessionEl>>,
    egress: Option<DynamicBlock<TeamsRuleRuleSettingsElEgressEl>>,
    l4override: Option<DynamicBlock<TeamsRuleRuleSettingsElL4overrideEl>>,
    payload_log: Option<DynamicBlock<TeamsRuleRuleSettingsElPayloadLogEl>>,
    untrusted_cert: Option<DynamicBlock<TeamsRuleRuleSettingsElUntrustedCertEl>>,
}

#[derive(Serialize)]
pub struct TeamsRuleRuleSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    add_headers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_child_bypass: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_page_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_page_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_parent_rule: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure_disable_dnssec_validation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_categories: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_ips: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audit_ssh: Option<Vec<TeamsRuleRuleSettingsElAuditSshEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    biso_admin_controls: Option<Vec<TeamsRuleRuleSettingsElBisoAdminControlsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    check_session: Option<Vec<TeamsRuleRuleSettingsElCheckSessionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress: Option<Vec<TeamsRuleRuleSettingsElEgressEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    l4override: Option<Vec<TeamsRuleRuleSettingsElL4overrideEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload_log: Option<Vec<TeamsRuleRuleSettingsElPayloadLogEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    untrusted_cert: Option<Vec<TeamsRuleRuleSettingsElUntrustedCertEl>>,
    dynamic: TeamsRuleRuleSettingsElDynamic,
}

impl TeamsRuleRuleSettingsEl {
    #[doc= "Set the field `add_headers`.\nAdd custom headers to allowed requests in the form of key-value pairs."]
    pub fn set_add_headers(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.add_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_child_bypass`.\nAllow parent MSP accounts to enable bypass their children's rules."]
    pub fn set_allow_child_bypass(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_child_bypass = Some(v.into());
        self
    }

    #[doc= "Set the field `block_page_enabled`.\nIndicator of block page enablement."]
    pub fn set_block_page_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.block_page_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `block_page_reason`.\nThe displayed reason for a user being blocked."]
    pub fn set_block_page_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.block_page_reason = Some(v.into());
        self
    }

    #[doc= "Set the field `bypass_parent_rule`.\nAllow child MSP accounts to bypass their parent's rule."]
    pub fn set_bypass_parent_rule(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.bypass_parent_rule = Some(v.into());
        self
    }

    #[doc= "Set the field `insecure_disable_dnssec_validation`.\nDisable DNSSEC validation (must be Allow rule)."]
    pub fn set_insecure_disable_dnssec_validation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.insecure_disable_dnssec_validation = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_categories`.\nTurns on IP category based filter on dns if the rule contains dns category checks."]
    pub fn set_ip_categories(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ip_categories = Some(v.into());
        self
    }

    #[doc= "Set the field `override_host`.\nThe host to override matching DNS queries with."]
    pub fn set_override_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.override_host = Some(v.into());
        self
    }

    #[doc= "Set the field `override_ips`.\nThe IPs to override matching DNS queries with."]
    pub fn set_override_ips(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.override_ips = Some(v.into());
        self
    }

    #[doc= "Set the field `audit_ssh`.\n"]
    pub fn set_audit_ssh(mut self, v: impl Into<BlockAssignable<TeamsRuleRuleSettingsElAuditSshEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audit_ssh = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audit_ssh = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `biso_admin_controls`.\n"]
    pub fn set_biso_admin_controls(
        mut self,
        v: impl Into<BlockAssignable<TeamsRuleRuleSettingsElBisoAdminControlsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.biso_admin_controls = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.biso_admin_controls = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `check_session`.\n"]
    pub fn set_check_session(mut self, v: impl Into<BlockAssignable<TeamsRuleRuleSettingsElCheckSessionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.check_session = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.check_session = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `egress`.\n"]
    pub fn set_egress(mut self, v: impl Into<BlockAssignable<TeamsRuleRuleSettingsElEgressEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.egress = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.egress = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `l4override`.\n"]
    pub fn set_l4override(mut self, v: impl Into<BlockAssignable<TeamsRuleRuleSettingsElL4overrideEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.l4override = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.l4override = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `payload_log`.\n"]
    pub fn set_payload_log(mut self, v: impl Into<BlockAssignable<TeamsRuleRuleSettingsElPayloadLogEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.payload_log = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.payload_log = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `untrusted_cert`.\n"]
    pub fn set_untrusted_cert(mut self, v: impl Into<BlockAssignable<TeamsRuleRuleSettingsElUntrustedCertEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.untrusted_cert = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.untrusted_cert = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for TeamsRuleRuleSettingsEl {
    type O = BlockAssignable<TeamsRuleRuleSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsRuleRuleSettingsEl {}

impl BuildTeamsRuleRuleSettingsEl {
    pub fn build(self) -> TeamsRuleRuleSettingsEl {
        TeamsRuleRuleSettingsEl {
            add_headers: core::default::Default::default(),
            allow_child_bypass: core::default::Default::default(),
            block_page_enabled: core::default::Default::default(),
            block_page_reason: core::default::Default::default(),
            bypass_parent_rule: core::default::Default::default(),
            insecure_disable_dnssec_validation: core::default::Default::default(),
            ip_categories: core::default::Default::default(),
            override_host: core::default::Default::default(),
            override_ips: core::default::Default::default(),
            audit_ssh: core::default::Default::default(),
            biso_admin_controls: core::default::Default::default(),
            check_session: core::default::Default::default(),
            egress: core::default::Default::default(),
            l4override: core::default::Default::default(),
            payload_log: core::default::Default::default(),
            untrusted_cert: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TeamsRuleRuleSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsRuleRuleSettingsElRef {
    fn new(shared: StackShared, base: String) -> TeamsRuleRuleSettingsElRef {
        TeamsRuleRuleSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsRuleRuleSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `add_headers` after provisioning.\nAdd custom headers to allowed requests in the form of key-value pairs."]
    pub fn add_headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.add_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_child_bypass` after provisioning.\nAllow parent MSP accounts to enable bypass their children's rules."]
    pub fn allow_child_bypass(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_child_bypass", self.base))
    }

    #[doc= "Get a reference to the value of field `block_page_enabled` after provisioning.\nIndicator of block page enablement."]
    pub fn block_page_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_page_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `block_page_reason` after provisioning.\nThe displayed reason for a user being blocked."]
    pub fn block_page_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_page_reason", self.base))
    }

    #[doc= "Get a reference to the value of field `bypass_parent_rule` after provisioning.\nAllow child MSP accounts to bypass their parent's rule."]
    pub fn bypass_parent_rule(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bypass_parent_rule", self.base))
    }

    #[doc= "Get a reference to the value of field `insecure_disable_dnssec_validation` after provisioning.\nDisable DNSSEC validation (must be Allow rule)."]
    pub fn insecure_disable_dnssec_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.insecure_disable_dnssec_validation", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_categories` after provisioning.\nTurns on IP category based filter on dns if the rule contains dns category checks."]
    pub fn ip_categories(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_categories", self.base))
    }

    #[doc= "Get a reference to the value of field `override_host` after provisioning.\nThe host to override matching DNS queries with."]
    pub fn override_host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.override_host", self.base))
    }

    #[doc= "Get a reference to the value of field `override_ips` after provisioning.\nThe IPs to override matching DNS queries with."]
    pub fn override_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.override_ips", self.base))
    }

    #[doc= "Get a reference to the value of field `audit_ssh` after provisioning.\n"]
    pub fn audit_ssh(&self) -> ListRef<TeamsRuleRuleSettingsElAuditSshElRef> {
        ListRef::new(self.shared().clone(), format!("{}.audit_ssh", self.base))
    }

    #[doc= "Get a reference to the value of field `biso_admin_controls` after provisioning.\n"]
    pub fn biso_admin_controls(&self) -> ListRef<TeamsRuleRuleSettingsElBisoAdminControlsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.biso_admin_controls", self.base))
    }

    #[doc= "Get a reference to the value of field `check_session` after provisioning.\n"]
    pub fn check_session(&self) -> ListRef<TeamsRuleRuleSettingsElCheckSessionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.check_session", self.base))
    }

    #[doc= "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> ListRef<TeamsRuleRuleSettingsElEgressElRef> {
        ListRef::new(self.shared().clone(), format!("{}.egress", self.base))
    }

    #[doc= "Get a reference to the value of field `l4override` after provisioning.\n"]
    pub fn l4override(&self) -> ListRef<TeamsRuleRuleSettingsElL4overrideElRef> {
        ListRef::new(self.shared().clone(), format!("{}.l4override", self.base))
    }

    #[doc= "Get a reference to the value of field `payload_log` after provisioning.\n"]
    pub fn payload_log(&self) -> ListRef<TeamsRuleRuleSettingsElPayloadLogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.payload_log", self.base))
    }

    #[doc= "Get a reference to the value of field `untrusted_cert` after provisioning.\n"]
    pub fn untrusted_cert(&self) -> ListRef<TeamsRuleRuleSettingsElUntrustedCertElRef> {
        ListRef::new(self.shared().clone(), format!("{}.untrusted_cert", self.base))
    }
}

#[derive(Serialize, Default)]
struct TeamsRuleDynamic {
    rule_settings: Option<DynamicBlock<TeamsRuleRuleSettingsEl>>,
}
