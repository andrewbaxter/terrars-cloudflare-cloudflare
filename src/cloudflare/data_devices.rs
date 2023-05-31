use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataDevicesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataDevices_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDevicesData>,
}

#[derive(Clone)]
pub struct DataDevices(Rc<DataDevices_>);

impl DataDevices {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderCloudflare) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `devices` after provisioning.\n"]
    pub fn devices(&self) -> ListRef<DataDevicesDevicesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.devices", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl Referable for DataDevices {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataDevices { }

impl ToListMappable for DataDevices {
    type O = ListRef<DataDevicesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataDevices_ {
    fn extract_datasource_type(&self) -> String {
        "cloudflare_devices".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDevices {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
}

impl BuildDataDevices {
    pub fn build(self, stack: &mut Stack) -> DataDevices {
        let out = DataDevices(Rc::new(DataDevices_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDevicesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                account_id: self.account_id,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDevicesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDevicesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDevicesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `devices` after provisioning.\n"]
    pub fn devices(&self) -> ListRef<DataDevicesDevicesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.devices", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDevicesDevicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deleted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_seen: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mac_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_distro_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_distro_revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoked_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serial_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DataDevicesDevicesEl {
    #[doc= "Set the field `created`.\n"]
    pub fn set_created(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created = Some(v.into());
        self
    }

    #[doc= "Set the field `deleted`.\n"]
    pub fn set_deleted(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deleted = Some(v.into());
        self
    }

    #[doc= "Set the field `device_type`.\n"]
    pub fn set_device_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_type = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip = Some(v.into());
        self
    }

    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `last_seen`.\n"]
    pub fn set_last_seen(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_seen = Some(v.into());
        self
    }

    #[doc= "Set the field `mac_address`.\n"]
    pub fn set_mac_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mac_address = Some(v.into());
        self
    }

    #[doc= "Set the field `manufacturer`.\n"]
    pub fn set_manufacturer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.manufacturer = Some(v.into());
        self
    }

    #[doc= "Set the field `model`.\n"]
    pub fn set_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `os_distro_name`.\n"]
    pub fn set_os_distro_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.os_distro_name = Some(v.into());
        self
    }

    #[doc= "Set the field `os_distro_revision`.\n"]
    pub fn set_os_distro_revision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.os_distro_revision = Some(v.into());
        self
    }

    #[doc= "Set the field `os_version`.\n"]
    pub fn set_os_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.os_version = Some(v.into());
        self
    }

    #[doc= "Set the field `revoked_at`.\n"]
    pub fn set_revoked_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revoked_at = Some(v.into());
        self
    }

    #[doc= "Set the field `serial_number`.\n"]
    pub fn set_serial_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.serial_number = Some(v.into());
        self
    }

    #[doc= "Set the field `updated`.\n"]
    pub fn set_updated(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.updated = Some(v.into());
        self
    }

    #[doc= "Set the field `user_email`.\n"]
    pub fn set_user_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_email = Some(v.into());
        self
    }

    #[doc= "Set the field `user_id`.\n"]
    pub fn set_user_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_id = Some(v.into());
        self
    }

    #[doc= "Set the field `user_name`.\n"]
    pub fn set_user_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_name = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for DataDevicesDevicesEl {
    type O = BlockAssignable<DataDevicesDevicesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDevicesDevicesEl {}

impl BuildDataDevicesDevicesEl {
    pub fn build(self) -> DataDevicesDevicesEl {
        DataDevicesDevicesEl {
            created: core::default::Default::default(),
            deleted: core::default::Default::default(),
            device_type: core::default::Default::default(),
            id: core::default::Default::default(),
            ip: core::default::Default::default(),
            key: core::default::Default::default(),
            last_seen: core::default::Default::default(),
            mac_address: core::default::Default::default(),
            manufacturer: core::default::Default::default(),
            model: core::default::Default::default(),
            name: core::default::Default::default(),
            os_distro_name: core::default::Default::default(),
            os_distro_revision: core::default::Default::default(),
            os_version: core::default::Default::default(),
            revoked_at: core::default::Default::default(),
            serial_number: core::default::Default::default(),
            updated: core::default::Default::default(),
            user_email: core::default::Default::default(),
            user_id: core::default::Default::default(),
            user_name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataDevicesDevicesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDevicesDevicesElRef {
    fn new(shared: StackShared, base: String) -> DataDevicesDevicesElRef {
        DataDevicesDevicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDevicesDevicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc= "Get a reference to the value of field `deleted` after provisioning.\n"]
    pub fn deleted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deleted", self.base))
    }

    #[doc= "Get a reference to the value of field `device_type` after provisioning.\n"]
    pub fn device_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `last_seen` after provisioning.\n"]
    pub fn last_seen(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_seen", self.base))
    }

    #[doc= "Get a reference to the value of field `mac_address` after provisioning.\n"]
    pub fn mac_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mac_address", self.base))
    }

    #[doc= "Get a reference to the value of field `manufacturer` after provisioning.\n"]
    pub fn manufacturer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.manufacturer", self.base))
    }

    #[doc= "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `os_distro_name` after provisioning.\n"]
    pub fn os_distro_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os_distro_name", self.base))
    }

    #[doc= "Get a reference to the value of field `os_distro_revision` after provisioning.\n"]
    pub fn os_distro_revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os_distro_revision", self.base))
    }

    #[doc= "Get a reference to the value of field `os_version` after provisioning.\n"]
    pub fn os_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os_version", self.base))
    }

    #[doc= "Get a reference to the value of field `revoked_at` after provisioning.\n"]
    pub fn revoked_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revoked_at", self.base))
    }

    #[doc= "Get a reference to the value of field `serial_number` after provisioning.\n"]
    pub fn serial_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.base))
    }

    #[doc= "Get a reference to the value of field `updated` after provisioning.\n"]
    pub fn updated(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated", self.base))
    }

    #[doc= "Get a reference to the value of field `user_email` after provisioning.\n"]
    pub fn user_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_email", self.base))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }

    #[doc= "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_name", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
