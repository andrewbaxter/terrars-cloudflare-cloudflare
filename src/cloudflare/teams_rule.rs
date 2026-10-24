use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct TeamsRuleData {
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
    action: terrars::PrimField<std::string::String>,
    description: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_posture: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filters: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    precedence: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_settings: std::option::Option<std::vec::Vec<TeamsRuleRuleSettingsEl>>,
    dynamic: TeamsRuleDynamic,
}

struct TeamsRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TeamsRuleData>,
}

#[derive(Clone)]
pub struct TeamsRule(std::rc::Rc<TeamsRule_>);

impl TeamsRule {
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

    #[doc = "Set the field `device_posture`.\nThe wirefilter expression to be used for device_posture check matching."]
    pub fn set_device_posture(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().device_posture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nIndicator of rule enablement."]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filters`.\nThe protocol or layer to evaluate the traffic and identity expressions."]
    pub fn set_filters(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().filters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity`.\nThe wirefilter expression to be used for identity matching."]
    pub fn set_identity(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().identity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `traffic`.\nThe wirefilter expression to be used for traffic matching."]
    pub fn set_traffic(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().traffic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_settings`.\n"]
    pub fn set_rule_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TeamsRuleRuleSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe action executed by matched teams rule. Available values: `allow`, `block`, `safesearch`, `ytrestricted`, `on`, `off`, `scan`, `noscan`, `isolate`, `noisolate`, `override`, `l4_override`, `egress`, `audit_ssh`."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the teams rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `device_posture` after provisioning.\nThe wirefilter expression to be used for device_posture check matching."]
    pub fn device_posture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_posture", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nIndicator of rule enablement."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filters` after provisioning.\nThe protocol or layer to evaluate the traffic and identity expressions."]
    pub fn filters(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identity` after provisioning.\nThe wirefilter expression to be used for identity matching."]
    pub fn identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the teams rule."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `precedence` after provisioning.\nThe evaluation precedence of the teams rule."]
    pub fn precedence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precedence", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `traffic` after provisioning.\nThe wirefilter expression to be used for traffic matching."]
    pub fn traffic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_settings` after provisioning.\n"]
    pub fn rule_settings(&self) -> terrars::ListRef<TeamsRuleRuleSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_settings", self.extract_ref()))
    }
}

impl terrars::Referable for TeamsRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TeamsRule { }

impl terrars::ToListMappable for TeamsRule {
    type O = terrars::ListRef<TeamsRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TeamsRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_teams_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTeamsRule {
    pub tf_id: std::string::String,
    #[doc = "The account identifier to target for the resource."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The action executed by matched teams rule. Available values: `allow`, `block`, `safesearch`, `ytrestricted`, `on`, `off`, `scan`, `noscan`, `isolate`, `noisolate`, `override`, `l4_override`, `egress`, `audit_ssh`."]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = "The description of the teams rule."]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = "The name of the teams rule."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The evaluation precedence of the teams rule."]
    pub precedence: terrars::PrimField<f64>,
}

impl BuildTeamsRule {
    pub fn build(self, stack: &mut terrars::Stack) -> TeamsRule {
        let out = TeamsRule(std::rc::Rc::new(TeamsRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TeamsRuleData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TeamsRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TeamsRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TeamsRuleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe action executed by matched teams rule. Available values: `allow`, `block`, `safesearch`, `ytrestricted`, `on`, `off`, `scan`, `noscan`, `isolate`, `noisolate`, `override`, `l4_override`, `egress`, `audit_ssh`."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the teams rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `device_posture` after provisioning.\nThe wirefilter expression to be used for device_posture check matching."]
    pub fn device_posture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_posture", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nIndicator of rule enablement."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filters` after provisioning.\nThe protocol or layer to evaluate the traffic and identity expressions."]
    pub fn filters(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identity` after provisioning.\nThe wirefilter expression to be used for identity matching."]
    pub fn identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the teams rule."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `precedence` after provisioning.\nThe evaluation precedence of the teams rule."]
    pub fn precedence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precedence", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `traffic` after provisioning.\nThe wirefilter expression to be used for traffic matching."]
    pub fn traffic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_settings` after provisioning.\n"]
    pub fn rule_settings(&self) -> terrars::ListRef<TeamsRuleRuleSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TeamsRuleRuleSettingsElAuditSshEl {
    command_logging: terrars::PrimField<bool>,
}

impl TeamsRuleRuleSettingsElAuditSshEl { }

impl terrars::ToListMappable for TeamsRuleRuleSettingsElAuditSshEl {
    type O = terrars::BlockAssignable<TeamsRuleRuleSettingsElAuditSshEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsRuleRuleSettingsElAuditSshEl {
    #[doc = "Log all SSH commands."]
    pub command_logging: terrars::PrimField<bool>,
}

impl BuildTeamsRuleRuleSettingsElAuditSshEl {
    pub fn build(self) -> TeamsRuleRuleSettingsElAuditSshEl {
        TeamsRuleRuleSettingsElAuditSshEl { command_logging: self.command_logging }
    }
}

pub struct TeamsRuleRuleSettingsElAuditSshElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TeamsRuleRuleSettingsElAuditSshElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TeamsRuleRuleSettingsElAuditSshElRef {
        TeamsRuleRuleSettingsElAuditSshElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsRuleRuleSettingsElAuditSshElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `command_logging` after provisioning.\nLog all SSH commands."]
    pub fn command_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.command_logging", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsRuleRuleSettingsElBisoAdminControlsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_copy_paste: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_download: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_keyboard: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_printing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_upload: std::option::Option<terrars::PrimField<bool>>,
}

impl TeamsRuleRuleSettingsElBisoAdminControlsEl {
    #[doc = "Set the field `disable_copy_paste`.\nDisable copy-paste."]
    pub fn set_disable_copy_paste(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_copy_paste = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_download`.\nDisable download."]
    pub fn set_disable_download(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_download = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_keyboard`.\nDisable keyboard usage."]
    pub fn set_disable_keyboard(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_keyboard = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_printing`.\nDisable printing."]
    pub fn set_disable_printing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_printing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_upload`.\nDisable upload."]
    pub fn set_disable_upload(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_upload = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TeamsRuleRuleSettingsElBisoAdminControlsEl {
    type O = terrars::BlockAssignable<TeamsRuleRuleSettingsElBisoAdminControlsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TeamsRuleRuleSettingsElBisoAdminControlsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TeamsRuleRuleSettingsElBisoAdminControlsElRef {
        TeamsRuleRuleSettingsElBisoAdminControlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsRuleRuleSettingsElBisoAdminControlsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_copy_paste` after provisioning.\nDisable copy-paste."]
    pub fn disable_copy_paste(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_copy_paste", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_download` after provisioning.\nDisable download."]
    pub fn disable_download(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_download", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_keyboard` after provisioning.\nDisable keyboard usage."]
    pub fn disable_keyboard(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_keyboard", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_printing` after provisioning.\nDisable printing."]
    pub fn disable_printing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_printing", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_upload` after provisioning.\nDisable upload."]
    pub fn disable_upload(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_upload", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsRuleRuleSettingsElCheckSessionEl {
    duration: terrars::PrimField<std::string::String>,
    enforce: terrars::PrimField<bool>,
}

impl TeamsRuleRuleSettingsElCheckSessionEl { }

impl terrars::ToListMappable for TeamsRuleRuleSettingsElCheckSessionEl {
    type O = terrars::BlockAssignable<TeamsRuleRuleSettingsElCheckSessionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsRuleRuleSettingsElCheckSessionEl {
    #[doc = "Configure how fresh the session needs to be to be considered valid."]
    pub duration: terrars::PrimField<std::string::String>,
    #[doc = "Enable session enforcement for this rule."]
    pub enforce: terrars::PrimField<bool>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TeamsRuleRuleSettingsElCheckSessionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TeamsRuleRuleSettingsElCheckSessionElRef {
        TeamsRuleRuleSettingsElCheckSessionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsRuleRuleSettingsElCheckSessionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nConfigure how fresh the session needs to be to be considered valid."]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforce` after provisioning.\nEnable session enforcement for this rule."]
    pub fn enforce(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsRuleRuleSettingsElEgressEl {
    ipv4: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_fallback: std::option::Option<terrars::PrimField<std::string::String>>,
    ipv6: terrars::PrimField<std::string::String>,
}

impl TeamsRuleRuleSettingsElEgressEl {
    #[doc =
        "Set the field `ipv4_fallback`.\nThe IPv4 address to be used for egress in the event of an error egressing with the primary IPv4. Can be '0.0.0.0' to indicate local egreass via Warp IPs."]
    pub fn set_ipv4_fallback(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv4_fallback = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TeamsRuleRuleSettingsElEgressEl {
    type O = terrars::BlockAssignable<TeamsRuleRuleSettingsElEgressEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsRuleRuleSettingsElEgressEl {
    #[doc = "The IPv4 address to be used for egress."]
    pub ipv4: terrars::PrimField<std::string::String>,
    #[doc = "The IPv6 range to be used for egress."]
    pub ipv6: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TeamsRuleRuleSettingsElEgressElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TeamsRuleRuleSettingsElEgressElRef {
        TeamsRuleRuleSettingsElEgressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsRuleRuleSettingsElEgressElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv4` after provisioning.\nThe IPv4 address to be used for egress."]
    pub fn ipv4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_fallback` after provisioning.\nThe IPv4 address to be used for egress in the event of an error egressing with the primary IPv4. Can be '0.0.0.0' to indicate local egreass via Warp IPs."]
    pub fn ipv4_fallback(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_fallback", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\nThe IPv6 range to be used for egress."]
    pub fn ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsRuleRuleSettingsElL4overrideEl {
    ip: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl TeamsRuleRuleSettingsElL4overrideEl { }

impl terrars::ToListMappable for TeamsRuleRuleSettingsElL4overrideEl {
    type O = terrars::BlockAssignable<TeamsRuleRuleSettingsElL4overrideEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsRuleRuleSettingsElL4overrideEl {
    #[doc = "Override IP to forward traffic to."]
    pub ip: terrars::PrimField<std::string::String>,
    #[doc = "Override Port to forward traffic to."]
    pub port: terrars::PrimField<f64>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TeamsRuleRuleSettingsElL4overrideElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TeamsRuleRuleSettingsElL4overrideElRef {
        TeamsRuleRuleSettingsElL4overrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsRuleRuleSettingsElL4overrideElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\nOverride IP to forward traffic to."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nOverride Port to forward traffic to."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsRuleRuleSettingsElPayloadLogEl {
    enabled: terrars::PrimField<bool>,
}

impl TeamsRuleRuleSettingsElPayloadLogEl { }

impl terrars::ToListMappable for TeamsRuleRuleSettingsElPayloadLogEl {
    type O = terrars::BlockAssignable<TeamsRuleRuleSettingsElPayloadLogEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsRuleRuleSettingsElPayloadLogEl {
    #[doc = "Enable or disable DLP Payload Logging for this rule."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildTeamsRuleRuleSettingsElPayloadLogEl {
    pub fn build(self) -> TeamsRuleRuleSettingsElPayloadLogEl {
        TeamsRuleRuleSettingsElPayloadLogEl { enabled: self.enabled }
    }
}

pub struct TeamsRuleRuleSettingsElPayloadLogElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TeamsRuleRuleSettingsElPayloadLogElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TeamsRuleRuleSettingsElPayloadLogElRef {
        TeamsRuleRuleSettingsElPayloadLogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsRuleRuleSettingsElPayloadLogElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnable or disable DLP Payload Logging for this rule."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsRuleRuleSettingsElUntrustedCertEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TeamsRuleRuleSettingsElUntrustedCertEl {
    #[doc =
        "Set the field `action`.\nAction to be taken when the SSL certificate of upstream is invalid. Available values: `pass_through`, `block`, `error`."]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TeamsRuleRuleSettingsElUntrustedCertEl {
    type O = terrars::BlockAssignable<TeamsRuleRuleSettingsElUntrustedCertEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TeamsRuleRuleSettingsElUntrustedCertElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TeamsRuleRuleSettingsElUntrustedCertElRef {
        TeamsRuleRuleSettingsElUntrustedCertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsRuleRuleSettingsElUntrustedCertElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAction to be taken when the SSL certificate of upstream is invalid. Available values: `pass_through`, `block`, `error`."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize, Default)]
struct TeamsRuleRuleSettingsElDynamic {
    audit_ssh: std::option::Option<terrars::DynamicBlock<TeamsRuleRuleSettingsElAuditSshEl>>,
    biso_admin_controls: std::option::Option<terrars::DynamicBlock<TeamsRuleRuleSettingsElBisoAdminControlsEl>>,
    check_session: std::option::Option<terrars::DynamicBlock<TeamsRuleRuleSettingsElCheckSessionEl>>,
    egress: std::option::Option<terrars::DynamicBlock<TeamsRuleRuleSettingsElEgressEl>>,
    l4override: std::option::Option<terrars::DynamicBlock<TeamsRuleRuleSettingsElL4overrideEl>>,
    payload_log: std::option::Option<terrars::DynamicBlock<TeamsRuleRuleSettingsElPayloadLogEl>>,
    untrusted_cert: std::option::Option<terrars::DynamicBlock<TeamsRuleRuleSettingsElUntrustedCertEl>>,
}

#[derive(Serialize)]
pub struct TeamsRuleRuleSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add_headers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_child_bypass: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_page_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_page_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bypass_parent_rule: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insecure_disable_dnssec_validation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_categories: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_ips: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit_ssh: std::option::Option<std::vec::Vec<TeamsRuleRuleSettingsElAuditSshEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    biso_admin_controls: std::option::Option<std::vec::Vec<TeamsRuleRuleSettingsElBisoAdminControlsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_session: std::option::Option<std::vec::Vec<TeamsRuleRuleSettingsElCheckSessionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<std::vec::Vec<TeamsRuleRuleSettingsElEgressEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    l4override: std::option::Option<std::vec::Vec<TeamsRuleRuleSettingsElL4overrideEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload_log: std::option::Option<std::vec::Vec<TeamsRuleRuleSettingsElPayloadLogEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    untrusted_cert: std::option::Option<std::vec::Vec<TeamsRuleRuleSettingsElUntrustedCertEl>>,
    dynamic: TeamsRuleRuleSettingsElDynamic,
}

impl TeamsRuleRuleSettingsEl {
    #[doc = "Set the field `add_headers`.\nAdd custom headers to allowed requests in the form of key-value pairs."]
    pub fn set_add_headers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.add_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_child_bypass`.\nAllow parent MSP accounts to enable bypass their children's rules."]
    pub fn set_allow_child_bypass(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_child_bypass = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_page_enabled`.\nIndicator of block page enablement."]
    pub fn set_block_page_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.block_page_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_page_reason`.\nThe displayed reason for a user being blocked."]
    pub fn set_block_page_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.block_page_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bypass_parent_rule`.\nAllow child MSP accounts to bypass their parent's rule."]
    pub fn set_bypass_parent_rule(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bypass_parent_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `insecure_disable_dnssec_validation`.\nDisable DNSSEC validation (must be Allow rule)."]
    pub fn set_insecure_disable_dnssec_validation(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.insecure_disable_dnssec_validation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_categories`.\nTurns on IP category based filter on dns if the rule contains dns category checks."]
    pub fn set_ip_categories(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ip_categories = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `override_host`.\nThe host to override matching DNS queries with."]
    pub fn set_override_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.override_host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `override_ips`.\nThe IPs to override matching DNS queries with."]
    pub fn set_override_ips(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.override_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audit_ssh`.\n"]
    pub fn set_audit_ssh(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TeamsRuleRuleSettingsElAuditSshEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audit_ssh = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audit_ssh = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `biso_admin_controls`.\n"]
    pub fn set_biso_admin_controls(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TeamsRuleRuleSettingsElBisoAdminControlsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.biso_admin_controls = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.biso_admin_controls = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `check_session`.\n"]
    pub fn set_check_session(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TeamsRuleRuleSettingsElCheckSessionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.check_session = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.check_session = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `egress`.\n"]
    pub fn set_egress(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TeamsRuleRuleSettingsElEgressEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `l4override`.\n"]
    pub fn set_l4override(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TeamsRuleRuleSettingsElL4overrideEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.l4override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.l4override = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `payload_log`.\n"]
    pub fn set_payload_log(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TeamsRuleRuleSettingsElPayloadLogEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.payload_log = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.payload_log = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `untrusted_cert`.\n"]
    pub fn set_untrusted_cert(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TeamsRuleRuleSettingsElUntrustedCertEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.untrusted_cert = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.untrusted_cert = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TeamsRuleRuleSettingsEl {
    type O = terrars::BlockAssignable<TeamsRuleRuleSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct TeamsRuleRuleSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TeamsRuleRuleSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TeamsRuleRuleSettingsElRef {
        TeamsRuleRuleSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsRuleRuleSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `add_headers` after provisioning.\nAdd custom headers to allowed requests in the form of key-value pairs."]
    pub fn add_headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.add_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_child_bypass` after provisioning.\nAllow parent MSP accounts to enable bypass their children's rules."]
    pub fn allow_child_bypass(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_child_bypass", self.base))
    }

    #[doc =
        "Get a reference to the value of field `block_page_enabled` after provisioning.\nIndicator of block page enablement."]
    pub fn block_page_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_page_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `block_page_reason` after provisioning.\nThe displayed reason for a user being blocked."]
    pub fn block_page_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_page_reason", self.base))
    }

    #[doc =
        "Get a reference to the value of field `bypass_parent_rule` after provisioning.\nAllow child MSP accounts to bypass their parent's rule."]
    pub fn bypass_parent_rule(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bypass_parent_rule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `insecure_disable_dnssec_validation` after provisioning.\nDisable DNSSEC validation (must be Allow rule)."]
    pub fn insecure_disable_dnssec_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.insecure_disable_dnssec_validation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_categories` after provisioning.\nTurns on IP category based filter on dns if the rule contains dns category checks."]
    pub fn ip_categories(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_categories", self.base))
    }

    #[doc =
        "Get a reference to the value of field `override_host` after provisioning.\nThe host to override matching DNS queries with."]
    pub fn override_host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `override_ips` after provisioning.\nThe IPs to override matching DNS queries with."]
    pub fn override_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `audit_ssh` after provisioning.\n"]
    pub fn audit_ssh(&self) -> terrars::ListRef<TeamsRuleRuleSettingsElAuditSshElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audit_ssh", self.base))
    }

    #[doc = "Get a reference to the value of field `biso_admin_controls` after provisioning.\n"]
    pub fn biso_admin_controls(&self) -> terrars::ListRef<TeamsRuleRuleSettingsElBisoAdminControlsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.biso_admin_controls", self.base))
    }

    #[doc = "Get a reference to the value of field `check_session` after provisioning.\n"]
    pub fn check_session(&self) -> terrars::ListRef<TeamsRuleRuleSettingsElCheckSessionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_session", self.base))
    }

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> terrars::ListRef<TeamsRuleRuleSettingsElEgressElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress", self.base))
    }

    #[doc = "Get a reference to the value of field `l4override` after provisioning.\n"]
    pub fn l4override(&self) -> terrars::ListRef<TeamsRuleRuleSettingsElL4overrideElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.l4override", self.base))
    }

    #[doc = "Get a reference to the value of field `payload_log` after provisioning.\n"]
    pub fn payload_log(&self) -> terrars::ListRef<TeamsRuleRuleSettingsElPayloadLogElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.payload_log", self.base))
    }

    #[doc = "Get a reference to the value of field `untrusted_cert` after provisioning.\n"]
    pub fn untrusted_cert(&self) -> terrars::ListRef<TeamsRuleRuleSettingsElUntrustedCertElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.untrusted_cert", self.base))
    }
}

#[derive(Serialize, Default)]
struct TeamsRuleDynamic {
    rule_settings: std::option::Option<terrars::DynamicBlock<TeamsRuleRuleSettingsEl>>,
}
