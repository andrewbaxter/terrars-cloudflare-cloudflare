use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DeviceDexTestData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    description: PrimField<String>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    interval: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<DeviceDexTestDataEl>>,
    dynamic: DeviceDexTestDynamic,
}

struct DeviceDexTest_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DeviceDexTestData>,
}

#[derive(Clone)]
pub struct DeviceDexTest(Rc<DeviceDexTest_>);

impl DeviceDexTest {
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

    #[doc= "Set the field `data`.\n"]
    pub fn set_data(self, v: impl Into<BlockAssignable<DeviceDexTestDataEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTimestamp of when the Dex Test was created."]
    pub fn created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAdditional details about the test."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nDetermines whether or not the test is active."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interval` after provisioning.\nHow often the test will run."]
    pub fn interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Device Dex Test. Must be unique."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated` after provisioning.\nTimestamp of when the Dex Test was last updated."]
    pub fn updated(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> ListRef<DeviceDexTestDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data", self.extract_ref()))
    }
}

impl Referable for DeviceDexTest {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DeviceDexTest { }

impl ToListMappable for DeviceDexTest {
    type O = ListRef<DeviceDexTestRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DeviceDexTest_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_device_dex_test".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDeviceDexTest {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub account_id: PrimField<String>,
    #[doc= "Additional details about the test."]
    pub description: PrimField<String>,
    #[doc= "Determines whether or not the test is active."]
    pub enabled: PrimField<bool>,
    #[doc= "How often the test will run."]
    pub interval: PrimField<String>,
    #[doc= "The name of the Device Dex Test. Must be unique."]
    pub name: PrimField<String>,
}

impl BuildDeviceDexTest {
    pub fn build(self, stack: &mut Stack) -> DeviceDexTest {
        let out = DeviceDexTest(Rc::new(DeviceDexTest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DeviceDexTestData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                description: self.description,
                enabled: self.enabled,
                id: core::default::Default::default(),
                interval: self.interval,
                name: self.name,
                data: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DeviceDexTestRef {
    shared: StackShared,
    base: String,
}

impl Ref for DeviceDexTestRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DeviceDexTestRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTimestamp of when the Dex Test was created."]
    pub fn created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAdditional details about the test."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nDetermines whether or not the test is active."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interval` after provisioning.\nHow often the test will run."]
    pub fn interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Device Dex Test. Must be unique."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated` after provisioning.\nTimestamp of when the Dex Test was last updated."]
    pub fn updated(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> ListRef<DeviceDexTestDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DeviceDexTestDataEl {
    host: PrimField<String>,
    kind: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
}

impl DeviceDexTestDataEl {
    #[doc= "Set the field `method`.\nThe http request method. Available values: `GET`."]
    pub fn set_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.method = Some(v.into());
        self
    }
}

impl ToListMappable for DeviceDexTestDataEl {
    type O = BlockAssignable<DeviceDexTestDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeviceDexTestDataEl {
    #[doc= "The host URL for `http` test `kind`. For `traceroute`, it must be a valid hostname or IP address."]
    pub host: PrimField<String>,
    #[doc= "The type of Device Dex Test. Available values: `http`, `traceroute`."]
    pub kind: PrimField<String>,
}

impl BuildDeviceDexTestDataEl {
    pub fn build(self) -> DeviceDexTestDataEl {
        DeviceDexTestDataEl {
            host: self.host,
            kind: self.kind,
            method: core::default::Default::default(),
        }
    }
}

pub struct DeviceDexTestDataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DeviceDexTestDataElRef {
    fn new(shared: StackShared, base: String) -> DeviceDexTestDataElRef {
        DeviceDexTestDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeviceDexTestDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\nThe host URL for `http` test `kind`. For `traceroute`, it must be a valid hostname or IP address."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc= "Get a reference to the value of field `kind` after provisioning.\nThe type of Device Dex Test. Available values: `http`, `traceroute`."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc= "Get a reference to the value of field `method` after provisioning.\nThe http request method. Available values: `GET`."]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }
}

#[derive(Serialize, Default)]
struct DeviceDexTestDynamic {
    data: Option<DynamicBlock<DeviceDexTestDataEl>>,
}
