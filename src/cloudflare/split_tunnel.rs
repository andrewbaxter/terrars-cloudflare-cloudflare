use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct SplitTunnelData {
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
    mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tunnels: Option<Vec<SplitTunnelTunnelsEl>>,
    dynamic: SplitTunnelDynamic,
}

struct SplitTunnel_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SplitTunnelData>,
}

#[derive(Clone)]
pub struct SplitTunnel(Rc<SplitTunnel_>);

impl SplitTunnel {
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

    #[doc= "Set the field `policy_id`.\nThe settings policy for which to configure this split tunnel policy."]
    pub fn set_policy_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy_id = Some(v.into());
        self
    }

    #[doc= "Set the field `tunnels`.\n"]
    pub fn set_tunnels(self, v: impl Into<BlockAssignable<SplitTunnelTunnelsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tunnels = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tunnels = Some(d);
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

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nThe mode of the split tunnel policy. Available values: `include`, `exclude`."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `policy_id` after provisioning.\nThe settings policy for which to configure this split tunnel policy."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }
}

impl Referable for SplitTunnel {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SplitTunnel { }

impl ToListMappable for SplitTunnel {
    type O = ListRef<SplitTunnelRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SplitTunnel_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_split_tunnel".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSplitTunnel {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
    #[doc= "The mode of the split tunnel policy. Available values: `include`, `exclude`."]
    pub mode: PrimField<String>,
}

impl BuildSplitTunnel {
    pub fn build(self, stack: &mut Stack) -> SplitTunnel {
        let out = SplitTunnel(Rc::new(SplitTunnel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SplitTunnelData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                id: core::default::Default::default(),
                mode: self.mode,
                policy_id: core::default::Default::default(),
                tunnels: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SplitTunnelRef {
    shared: StackShared,
    base: String,
}

impl Ref for SplitTunnelRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SplitTunnelRef {
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

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nThe mode of the split tunnel policy. Available values: `include`, `exclude`."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `policy_id` after provisioning.\nThe settings policy for which to configure this split tunnel policy."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SplitTunnelTunnelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
}

impl SplitTunnelTunnelsEl {
    #[doc= "Set the field `address`.\nThe address for the tunnel."]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nA description for the tunnel."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `host`.\nThe domain name for the tunnel."]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
}

impl ToListMappable for SplitTunnelTunnelsEl {
    type O = BlockAssignable<SplitTunnelTunnelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSplitTunnelTunnelsEl {}

impl BuildSplitTunnelTunnelsEl {
    pub fn build(self) -> SplitTunnelTunnelsEl {
        SplitTunnelTunnelsEl {
            address: core::default::Default::default(),
            description: core::default::Default::default(),
            host: core::default::Default::default(),
        }
    }
}

pub struct SplitTunnelTunnelsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SplitTunnelTunnelsElRef {
    fn new(shared: StackShared, base: String) -> SplitTunnelTunnelsElRef {
        SplitTunnelTunnelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SplitTunnelTunnelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `address` after provisioning.\nThe address for the tunnel."]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description for the tunnel."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\nThe domain name for the tunnel."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
}

#[derive(Serialize, Default)]
struct SplitTunnelDynamic {
    tunnels: Option<DynamicBlock<SplitTunnelTunnelsEl>>,
}
