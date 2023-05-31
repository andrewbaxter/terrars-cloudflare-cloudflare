use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DevicePostureIntegrationData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<DevicePostureIntegrationConfigEl>>,
    dynamic: DevicePostureIntegrationDynamic,
}

struct DevicePostureIntegration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DevicePostureIntegrationData>,
}

#[derive(Clone)]
pub struct DevicePostureIntegration(Rc<DevicePostureIntegration_>);

impl DevicePostureIntegration {
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

    #[doc= "Set the field `identifier`.\n"]
    pub fn set_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().identifier = Some(v.into());
        self
    }

    #[doc= "Set the field `interval`.\nIndicates the frequency with which to poll the third-party API. Must be in the format `1h` or `30m`."]
    pub fn set_interval(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().interval = Some(v.into());
        self
    }

    #[doc= "Set the field `config`.\n"]
    pub fn set_config(self, v: impl Into<BlockAssignable<DevicePostureIntegrationConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interval` after provisioning.\nIndicates the frequency with which to poll the third-party API. Must be in the format `1h` or `30m`."]
    pub fn interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the device posture integration."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe device posture integration type. Available values: `workspace_one`, `uptycs`, `crowdstrike_s2s`, `intune`, `kolide`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<DevicePostureIntegrationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }
}

impl Referable for DevicePostureIntegration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DevicePostureIntegration { }

impl ToListMappable for DevicePostureIntegration {
    type O = ListRef<DevicePostureIntegrationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DevicePostureIntegration_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_device_posture_integration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDevicePostureIntegration {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
    #[doc= "Name of the device posture integration."]
    pub name: PrimField<String>,
    #[doc= "The device posture integration type. Available values: `workspace_one`, `uptycs`, `crowdstrike_s2s`, `intune`, `kolide`."]
    pub type_: PrimField<String>,
}

impl BuildDevicePostureIntegration {
    pub fn build(self, stack: &mut Stack) -> DevicePostureIntegration {
        let out = DevicePostureIntegration(Rc::new(DevicePostureIntegration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DevicePostureIntegrationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                id: core::default::Default::default(),
                identifier: core::default::Default::default(),
                interval: core::default::Default::default(),
                name: self.name,
                type_: self.type_,
                config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DevicePostureIntegrationRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevicePostureIntegrationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DevicePostureIntegrationRef {
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

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interval` after provisioning.\nIndicates the frequency with which to poll the third-party API. Must be in the format `1h` or `30m`."]
    pub fn interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the device posture integration."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe device posture integration type. Available values: `workspace_one`, `uptycs`, `crowdstrike_s2s`, `intune`, `kolide`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<DevicePostureIntegrationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DevicePostureIntegrationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_id: Option<PrimField<String>>,
}

impl DevicePostureIntegrationConfigEl {
    #[doc= "Set the field `api_url`.\nThe third-party API's URL."]
    pub fn set_api_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_url = Some(v.into());
        self
    }

    #[doc= "Set the field `auth_url`.\nThe third-party authorization API URL."]
    pub fn set_auth_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_url = Some(v.into());
        self
    }

    #[doc= "Set the field `client_id`.\nThe client identifier for authenticating API calls."]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }

    #[doc= "Set the field `client_key`.\nThe client key for authenticating API calls."]
    pub fn set_client_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_key = Some(v.into());
        self
    }

    #[doc= "Set the field `client_secret`.\nThe client secret for authenticating API calls."]
    pub fn set_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret = Some(v.into());
        self
    }

    #[doc= "Set the field `customer_id`.\nThe customer identifier for authenticating API calls."]
    pub fn set_customer_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.customer_id = Some(v.into());
        self
    }
}

impl ToListMappable for DevicePostureIntegrationConfigEl {
    type O = BlockAssignable<DevicePostureIntegrationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevicePostureIntegrationConfigEl {}

impl BuildDevicePostureIntegrationConfigEl {
    pub fn build(self) -> DevicePostureIntegrationConfigEl {
        DevicePostureIntegrationConfigEl {
            api_url: core::default::Default::default(),
            auth_url: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_key: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            customer_id: core::default::Default::default(),
        }
    }
}

pub struct DevicePostureIntegrationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevicePostureIntegrationConfigElRef {
    fn new(shared: StackShared, base: String) -> DevicePostureIntegrationConfigElRef {
        DevicePostureIntegrationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevicePostureIntegrationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `api_url` after provisioning.\nThe third-party API's URL."]
    pub fn api_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_url", self.base))
    }

    #[doc= "Get a reference to the value of field `auth_url` after provisioning.\nThe third-party authorization API URL."]
    pub fn auth_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_url", self.base))
    }

    #[doc= "Get a reference to the value of field `client_id` after provisioning.\nThe client identifier for authenticating API calls."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc= "Get a reference to the value of field `client_key` after provisioning.\nThe client key for authenticating API calls."]
    pub fn client_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc= "Get a reference to the value of field `client_secret` after provisioning.\nThe client secret for authenticating API calls."]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc= "Get a reference to the value of field `customer_id` after provisioning.\nThe customer identifier for authenticating API calls."]
    pub fn customer_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DevicePostureIntegrationDynamic {
    config: Option<DynamicBlock<DevicePostureIntegrationConfigEl>>,
}
