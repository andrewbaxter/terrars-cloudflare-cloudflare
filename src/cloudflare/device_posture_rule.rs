use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DevicePostureRuleData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std::option::Option<std::vec::Vec<DevicePostureRuleInputEl>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<DevicePostureRuleMatchEl>>,
    dynamic: DevicePostureRuleDynamic,
}

struct DevicePostureRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DevicePostureRuleData>,
}

#[derive(Clone)]
pub struct DevicePostureRule(std::rc::Rc<DevicePostureRule_>);

impl DevicePostureRule {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expiration`.\nExpire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`."]
    pub fn set_expiration(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().expiration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the device posture rule."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schedule`.\nTells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`."]
    pub fn set_schedule(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input`.\n"]
    pub fn set_input(self, v: impl std::convert::Into<terrars::BlockAssignable<DevicePostureRuleInputEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(self, v: impl std::convert::Into<terrars::BlockAssignable<DevicePostureRuleMatchEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expiration` after provisioning.\nExpire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`."]
    pub fn expiration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the device posture rule."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nTells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`."]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> terrars::ListRef<DevicePostureRuleInputElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DevicePostureRuleMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }
}

impl terrars::Referable for DevicePostureRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DevicePostureRule { }

impl terrars::ToListMappable for DevicePostureRule {
    type O = terrars::ListRef<DevicePostureRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DevicePostureRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_device_posture_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDevicePostureRule {
    pub tf_id: std::string::String,
    #[doc = "The account identifier to target for the resource."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDevicePostureRule {
    pub fn build(self, stack: &mut terrars::Stack) -> DevicePostureRule {
        let out = DevicePostureRule(std::rc::Rc::new(DevicePostureRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DevicePostureRuleData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                description: core::default::Default::default(),
                expiration: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                schedule: core::default::Default::default(),
                type_: self.type_,
                input: core::default::Default::default(),
                match_: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DevicePostureRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DevicePostureRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DevicePostureRuleRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expiration` after provisioning.\nExpire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`."]
    pub fn expiration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the device posture rule."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nTells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`."]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> terrars::ListRef<DevicePostureRuleInputElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DevicePostureRuleMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DevicePostureRuleInputEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_disks: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exists: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os_distro_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os_distro_revision: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overall: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    running: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensor_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha256: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thumbprint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_operator: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DevicePostureRuleInputEl {
    #[doc = "Set the field `check_disks`.\nSpecific volume(s) to check for encryption."]
    pub fn set_check_disks(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.check_disks = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `compliance_status`.\nThe workspace one device compliance status. Available values: `compliant`, `noncompliant`."]
    pub fn set_compliance_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compliance_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_id`.\nThe workspace one connection id."]
    pub fn set_connection_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\nThe domain that the client must join."]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nTrue if the firewall must be enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exists`.\nChecks if the file should exist."]
    pub fn set_exists(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exists = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nThe Teams List id."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operator`.\nThe version comparison operator. Available values: `>`, `>=`, `<`, `<=`, `==`."]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `os`.\nOS signal score from Crowdstrike. Value must be between 1 and 100."]
    pub fn set_os(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.os = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `os_distro_name`.\nThe operating system excluding version information."]
    pub fn set_os_distro_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.os_distro_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `os_distro_revision`.\nThe operating system version excluding OS name information or release name."]
    pub fn set_os_distro_revision(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.os_distro_revision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `overall`.\nOverall ZTA score from Crowdstrike. Value must be between 1 and 100."]
    pub fn set_overall(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.overall = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\nThe path to the file."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_all`.\nTrue if all drives must be encrypted."]
    pub fn set_require_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `running`.\nChecks if the application should be running."]
    pub fn set_running(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.running = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensor_config`.\nSensor signal score from Crowdstrike. Value must be between 1 and 100."]
    pub fn set_sensor_config(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sensor_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sha256`.\nThe sha256 hash of the file."]
    pub fn set_sha256(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha256 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thumbprint`.\nThe thumbprint of the file certificate."]
    pub fn set_thumbprint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.thumbprint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nThe operating system semantic version."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version_operator`.\nThe version comparison operator for crowdstrike. Available values: `>`, `>=`, `<`, `<=`, `==`."]
    pub fn set_version_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version_operator = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DevicePostureRuleInputEl {
    type O = terrars::BlockAssignable<DevicePostureRuleInputEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevicePostureRuleInputEl {}

impl BuildDevicePostureRuleInputEl {
    pub fn build(self) -> DevicePostureRuleInputEl {
        DevicePostureRuleInputEl {
            check_disks: core::default::Default::default(),
            compliance_status: core::default::Default::default(),
            connection_id: core::default::Default::default(),
            domain: core::default::Default::default(),
            enabled: core::default::Default::default(),
            exists: core::default::Default::default(),
            id: core::default::Default::default(),
            operator: core::default::Default::default(),
            os: core::default::Default::default(),
            os_distro_name: core::default::Default::default(),
            os_distro_revision: core::default::Default::default(),
            overall: core::default::Default::default(),
            path: core::default::Default::default(),
            require_all: core::default::Default::default(),
            running: core::default::Default::default(),
            sensor_config: core::default::Default::default(),
            sha256: core::default::Default::default(),
            thumbprint: core::default::Default::default(),
            version: core::default::Default::default(),
            version_operator: core::default::Default::default(),
        }
    }
}

pub struct DevicePostureRuleInputElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DevicePostureRuleInputElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DevicePostureRuleInputElRef {
        DevicePostureRuleInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevicePostureRuleInputElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `check_disks` after provisioning.\nSpecific volume(s) to check for encryption."]
    pub fn check_disks(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.check_disks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `compliance_status` after provisioning.\nThe workspace one device compliance status. Available values: `compliant`, `noncompliant`."]
    pub fn compliance_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compliance_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connection_id` after provisioning.\nThe workspace one connection id."]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe domain that the client must join."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nTrue if the firewall must be enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `exists` after provisioning.\nChecks if the file should exist."]
    pub fn exists(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exists", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe Teams List id."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nThe version comparison operator. Available values: `>`, `>=`, `<`, `<=`, `==`."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `os` after provisioning.\nOS signal score from Crowdstrike. Value must be between 1 and 100."]
    pub fn os(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os", self.base))
    }

    #[doc =
        "Get a reference to the value of field `os_distro_name` after provisioning.\nThe operating system excluding version information."]
    pub fn os_distro_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_distro_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `os_distro_revision` after provisioning.\nThe operating system version excluding OS name information or release name."]
    pub fn os_distro_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_distro_revision", self.base))
    }

    #[doc =
        "Get a reference to the value of field `overall` after provisioning.\nOverall ZTA score from Crowdstrike. Value must be between 1 and 100."]
    pub fn overall(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overall", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe path to the file."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_all` after provisioning.\nTrue if all drives must be encrypted."]
    pub fn require_all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `running` after provisioning.\nChecks if the application should be running."]
    pub fn running(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.running", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sensor_config` after provisioning.\nSensor signal score from Crowdstrike. Value must be between 1 and 100."]
    pub fn sensor_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensor_config", self.base))
    }

    #[doc = "Get a reference to the value of field `sha256` after provisioning.\nThe sha256 hash of the file."]
    pub fn sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }

    #[doc =
        "Get a reference to the value of field `thumbprint` after provisioning.\nThe thumbprint of the file certificate."]
    pub fn thumbprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thumbprint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe operating system semantic version."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version_operator` after provisioning.\nThe version comparison operator for crowdstrike. Available values: `>`, `>=`, `<`, `<=`, `==`."]
    pub fn version_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_operator", self.base))
    }
}

#[derive(Serialize)]
pub struct DevicePostureRuleMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DevicePostureRuleMatchEl {
    #[doc =
        "Set the field `platform`.\nThe platform of the device. Available values: `windows`, `mac`, `linux`, `android`, `ios`, `chromeos`."]
    pub fn set_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.platform = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DevicePostureRuleMatchEl {
    type O = terrars::BlockAssignable<DevicePostureRuleMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevicePostureRuleMatchEl {}

impl BuildDevicePostureRuleMatchEl {
    pub fn build(self) -> DevicePostureRuleMatchEl {
        DevicePostureRuleMatchEl { platform: core::default::Default::default() }
    }
}

pub struct DevicePostureRuleMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DevicePostureRuleMatchElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DevicePostureRuleMatchElRef {
        DevicePostureRuleMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevicePostureRuleMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `platform` after provisioning.\nThe platform of the device. Available values: `windows`, `mac`, `linux`, `android`, `ios`, `chromeos`."]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.base))
    }
}

#[derive(Serialize, Default)]
struct DevicePostureRuleDynamic {
    input: std::option::Option<terrars::DynamicBlock<DevicePostureRuleInputEl>>,
    match_: std::option::Option<terrars::DynamicBlock<DevicePostureRuleMatchEl>>,
}
