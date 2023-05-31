use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DeviceManagedNetworksData {
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
    name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<DeviceManagedNetworksConfigEl>>,
    dynamic: DeviceManagedNetworksDynamic,
}

struct DeviceManagedNetworks_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DeviceManagedNetworksData>,
}

#[derive(Clone)]
pub struct DeviceManagedNetworks(Rc<DeviceManagedNetworks_>);

impl DeviceManagedNetworks {
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

    #[doc= "Set the field `config`.\n"]
    pub fn set_config(self, v: impl Into<BlockAssignable<DeviceManagedNetworksConfigEl>>) -> Self {
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

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Device Managed Network. Must be unique."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of Device Managed Network. Available values: `tls`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<DeviceManagedNetworksConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }
}

impl Referable for DeviceManagedNetworks {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DeviceManagedNetworks { }

impl ToListMappable for DeviceManagedNetworks {
    type O = ListRef<DeviceManagedNetworksRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DeviceManagedNetworks_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_device_managed_networks".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDeviceManagedNetworks {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
    #[doc= "The name of the Device Managed Network. Must be unique."]
    pub name: PrimField<String>,
    #[doc= "The type of Device Managed Network. Available values: `tls`."]
    pub type_: PrimField<String>,
}

impl BuildDeviceManagedNetworks {
    pub fn build(self, stack: &mut Stack) -> DeviceManagedNetworks {
        let out = DeviceManagedNetworks(Rc::new(DeviceManagedNetworks_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DeviceManagedNetworksData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                id: core::default::Default::default(),
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

pub struct DeviceManagedNetworksRef {
    shared: StackShared,
    base: String,
}

impl Ref for DeviceManagedNetworksRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DeviceManagedNetworksRef {
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

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Device Managed Network. Must be unique."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of Device Managed Network. Available values: `tls`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<DeviceManagedNetworksConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DeviceManagedNetworksConfigEl {
    sha256: PrimField<String>,
    tls_sockaddr: PrimField<String>,
}

impl DeviceManagedNetworksConfigEl { }

impl ToListMappable for DeviceManagedNetworksConfigEl {
    type O = BlockAssignable<DeviceManagedNetworksConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeviceManagedNetworksConfigEl {
    #[doc= "The SHA-256 hash of the TLS certificate presented by the host found at tls_sockaddr. If absent, regular certificate verification (trusted roots, valid timestamp, etc) will be used to validate the certificate."]
    pub sha256: PrimField<String>,
    #[doc= "A network address of the form \"host:port\" that the WARP client will use to detect the presence of a TLS host."]
    pub tls_sockaddr: PrimField<String>,
}

impl BuildDeviceManagedNetworksConfigEl {
    pub fn build(self) -> DeviceManagedNetworksConfigEl {
        DeviceManagedNetworksConfigEl {
            sha256: self.sha256,
            tls_sockaddr: self.tls_sockaddr,
        }
    }
}

pub struct DeviceManagedNetworksConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DeviceManagedNetworksConfigElRef {
    fn new(shared: StackShared, base: String) -> DeviceManagedNetworksConfigElRef {
        DeviceManagedNetworksConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeviceManagedNetworksConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `sha256` after provisioning.\nThe SHA-256 hash of the TLS certificate presented by the host found at tls_sockaddr. If absent, regular certificate verification (trusted roots, valid timestamp, etc) will be used to validate the certificate."]
    pub fn sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }

    #[doc= "Get a reference to the value of field `tls_sockaddr` after provisioning.\nA network address of the form \"host:port\" that the WARP client will use to detect the presence of a TLS host."]
    pub fn tls_sockaddr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_sockaddr", self.base))
    }
}

#[derive(Serialize, Default)]
struct DeviceManagedNetworksDynamic {
    config: Option<DynamicBlock<DeviceManagedNetworksConfigEl>>,
}
