use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DevicePostureRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<Vec<DevicePostureRuleInputEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<DevicePostureRuleMatchEl>>,
    dynamic: DevicePostureRuleDynamic,
}

struct DevicePostureRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DevicePostureRuleData>,
}

#[derive(Clone)]
pub struct DevicePostureRule(Rc<DevicePostureRule_>);

impl DevicePostureRule {
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

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `expiration`.\nExpire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`."]
    pub fn set_expiration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expiration = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nName of the device posture rule."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `schedule`.\nTells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`."]
    pub fn set_schedule(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().schedule = Some(v.into());
        self
    }

    #[doc= "Set the field `input`.\n"]
    pub fn set_input(self, v: impl Into<BlockAssignable<DevicePostureRuleInputEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `match_`.\n"]
    pub fn set_match(self, v: impl Into<BlockAssignable<DevicePostureRuleMatchEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().match_ = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.match_ = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expiration` after provisioning.\nExpire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`."]
    pub fn expiration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expiration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the device posture rule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `schedule` after provisioning.\nTells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`."]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> ListRef<DevicePostureRuleInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<DevicePostureRuleMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }
}

impl Referable for DevicePostureRule {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DevicePostureRule { }

impl ToListMappable for DevicePostureRule {
    type O = ListRef<DevicePostureRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DevicePostureRule_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_device_posture_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDevicePostureRule {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
    #[doc= "The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`."]
    pub type_: PrimField<String>,
}

impl BuildDevicePostureRule {
    pub fn build(self, stack: &mut Stack) -> DevicePostureRule {
        let out = DevicePostureRule(Rc::new(DevicePostureRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DevicePostureRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                description: core::default::Default::default(),
                expiration: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                schedule: core::default::Default::default(),
                type_: self.type_,
                input: core::default::Default::default(),
                match_: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DevicePostureRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevicePostureRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DevicePostureRuleRef {
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

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expiration` after provisioning.\nExpire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`."]
    pub fn expiration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expiration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the device posture rule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `schedule` after provisioning.\nTells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`."]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> ListRef<DevicePostureRuleInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<DevicePostureRuleMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DevicePostureRuleInputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    check_disks: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compliance_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exists: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_distro_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_distro_revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overall: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_all: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    running: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensor_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbprint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_operator: Option<PrimField<String>>,
}

impl DevicePostureRuleInputEl {
    #[doc= "Set the field `check_disks`.\nSpecific volume(s) to check for encryption."]
    pub fn set_check_disks(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.check_disks = Some(v.into());
        self
    }

    #[doc= "Set the field `compliance_status`.\nThe workspace one device compliance status. Available values: `compliant`, `noncompliant`."]
    pub fn set_compliance_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compliance_status = Some(v.into());
        self
    }

    #[doc= "Set the field `connection_id`.\nThe workspace one connection id."]
    pub fn set_connection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_id = Some(v.into());
        self
    }

    #[doc= "Set the field `domain`.\nThe domain that the client must join."]
    pub fn set_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\nTrue if the firewall must be enabled."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `exists`.\nChecks if the file should exist."]
    pub fn set_exists(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.exists = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\nThe Teams List id."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `operator`.\nThe version comparison operator. Available values: `>`, `>=`, `<`, `<=`, `==`."]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }

    #[doc= "Set the field `os`.\nOS signal score from Crowdstrike. Value must be between 1 and 100."]
    pub fn set_os(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.os = Some(v.into());
        self
    }

    #[doc= "Set the field `os_distro_name`.\nThe operating system excluding version information."]
    pub fn set_os_distro_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.os_distro_name = Some(v.into());
        self
    }

    #[doc= "Set the field `os_distro_revision`.\nThe operating system version excluding OS name information or release name."]
    pub fn set_os_distro_revision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.os_distro_revision = Some(v.into());
        self
    }

    #[doc= "Set the field `overall`.\nOverall ZTA score from Crowdstrike. Value must be between 1 and 100."]
    pub fn set_overall(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.overall = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\nThe path to the file."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `require_all`.\nTrue if all drives must be encrypted."]
    pub fn set_require_all(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_all = Some(v.into());
        self
    }

    #[doc= "Set the field `running`.\nChecks if the application should be running."]
    pub fn set_running(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.running = Some(v.into());
        self
    }

    #[doc= "Set the field `sensor_config`.\nSensor signal score from Crowdstrike. Value must be between 1 and 100."]
    pub fn set_sensor_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sensor_config = Some(v.into());
        self
    }

    #[doc= "Set the field `sha256`.\nThe sha256 hash of the file."]
    pub fn set_sha256(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha256 = Some(v.into());
        self
    }

    #[doc= "Set the field `thumbprint`.\nThe thumbprint of the file certificate."]
    pub fn set_thumbprint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.thumbprint = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\nThe operating system semantic version."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }

    #[doc= "Set the field `version_operator`.\nThe version comparison operator for crowdstrike. Available values: `>`, `>=`, `<`, `<=`, `==`."]
    pub fn set_version_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version_operator = Some(v.into());
        self
    }
}

impl ToListMappable for DevicePostureRuleInputEl {
    type O = BlockAssignable<DevicePostureRuleInputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DevicePostureRuleInputElRef {
    fn new(shared: StackShared, base: String) -> DevicePostureRuleInputElRef {
        DevicePostureRuleInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevicePostureRuleInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `check_disks` after provisioning.\nSpecific volume(s) to check for encryption."]
    pub fn check_disks(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.check_disks", self.base))
    }

    #[doc= "Get a reference to the value of field `compliance_status` after provisioning.\nThe workspace one device compliance status. Available values: `compliant`, `noncompliant`."]
    pub fn compliance_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compliance_status", self.base))
    }

    #[doc= "Get a reference to the value of field `connection_id` after provisioning.\nThe workspace one connection id."]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.base))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\nThe domain that the client must join."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nTrue if the firewall must be enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `exists` after provisioning.\nChecks if the file should exist."]
    pub fn exists(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exists", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe Teams List id."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `operator` after provisioning.\nThe version comparison operator. Available values: `>`, `>=`, `<`, `<=`, `==`."]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc= "Get a reference to the value of field `os` after provisioning.\nOS signal score from Crowdstrike. Value must be between 1 and 100."]
    pub fn os(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os", self.base))
    }

    #[doc= "Get a reference to the value of field `os_distro_name` after provisioning.\nThe operating system excluding version information."]
    pub fn os_distro_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os_distro_name", self.base))
    }

    #[doc= "Get a reference to the value of field `os_distro_revision` after provisioning.\nThe operating system version excluding OS name information or release name."]
    pub fn os_distro_revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os_distro_revision", self.base))
    }

    #[doc= "Get a reference to the value of field `overall` after provisioning.\nOverall ZTA score from Crowdstrike. Value must be between 1 and 100."]
    pub fn overall(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.overall", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe path to the file."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `require_all` after provisioning.\nTrue if all drives must be encrypted."]
    pub fn require_all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_all", self.base))
    }

    #[doc= "Get a reference to the value of field `running` after provisioning.\nChecks if the application should be running."]
    pub fn running(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.running", self.base))
    }

    #[doc= "Get a reference to the value of field `sensor_config` after provisioning.\nSensor signal score from Crowdstrike. Value must be between 1 and 100."]
    pub fn sensor_config(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sensor_config", self.base))
    }

    #[doc= "Get a reference to the value of field `sha256` after provisioning.\nThe sha256 hash of the file."]
    pub fn sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }

    #[doc= "Get a reference to the value of field `thumbprint` after provisioning.\nThe thumbprint of the file certificate."]
    pub fn thumbprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.thumbprint", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\nThe operating system semantic version."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc= "Get a reference to the value of field `version_operator` after provisioning.\nThe version comparison operator for crowdstrike. Available values: `>`, `>=`, `<`, `<=`, `==`."]
    pub fn version_operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_operator", self.base))
    }
}

#[derive(Serialize)]
pub struct DevicePostureRuleMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<PrimField<String>>,
}

impl DevicePostureRuleMatchEl {
    #[doc= "Set the field `platform`.\nThe platform of the device. Available values: `windows`, `mac`, `linux`, `android`, `ios`, `chromeos`."]
    pub fn set_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.platform = Some(v.into());
        self
    }
}

impl ToListMappable for DevicePostureRuleMatchEl {
    type O = BlockAssignable<DevicePostureRuleMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DevicePostureRuleMatchElRef {
    fn new(shared: StackShared, base: String) -> DevicePostureRuleMatchElRef {
        DevicePostureRuleMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevicePostureRuleMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `platform` after provisioning.\nThe platform of the device. Available values: `windows`, `mac`, `linux`, `android`, `ios`, `chromeos`."]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.base))
    }
}

#[derive(Serialize, Default)]
struct DevicePostureRuleDynamic {
    input: Option<DynamicBlock<DevicePostureRuleInputEl>>,
    match_: Option<DynamicBlock<DevicePostureRuleMatchEl>>,
}
