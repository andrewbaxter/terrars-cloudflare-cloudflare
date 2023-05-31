use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct TunnelData {
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
    config_src: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    secret: PrimField<String>,
}

struct Tunnel_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TunnelData>,
}

#[derive(Clone)]
pub struct Tunnel(Rc<Tunnel_>);

impl Tunnel {
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

    #[doc= "Set the field `config_src`.\nIndicates if this is a locally or remotely configured tunnel. If `local`, manage the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the tunnel on the Zero Trust dashboard or using tunnel_config, tunnel_route or tunnel_virtual_network resources. Available values: `local`, `cloudflare`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_config_src(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().config_src = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cname` after provisioning.\nUsable CNAME for accessing the Tunnel."]
    pub fn cname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `config_src` after provisioning.\nIndicates if this is a locally or remotely configured tunnel. If `local`, manage the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the tunnel on the Zero Trust dashboard or using tunnel_config, tunnel_route or tunnel_virtual_network resources. Available values: `local`, `cloudflare`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn config_src(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.config_src", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA user-friendly name chosen when the tunnel is created. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret` after provisioning.\n32 or more bytes, encoded as a base64 string. The Create Argo Tunnel endpoint sets this as the tunnel's password. Anyone wishing to run the tunnel needs this password. **Modifying this attribute will force creation of a new resource.**"]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tunnel_token` after provisioning.\nToken used by a connector to authenticate and run the tunnel."]
    pub fn tunnel_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel_token", self.extract_ref()))
    }
}

impl Referable for Tunnel {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Tunnel { }

impl ToListMappable for Tunnel {
    type O = ListRef<TunnelRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Tunnel_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_tunnel".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTunnel {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub account_id: PrimField<String>,
    #[doc= "A user-friendly name chosen when the tunnel is created. **Modifying this attribute will force creation of a new resource.**"]
    pub name: PrimField<String>,
    #[doc= "32 or more bytes, encoded as a base64 string. The Create Argo Tunnel endpoint sets this as the tunnel's password. Anyone wishing to run the tunnel needs this password. **Modifying this attribute will force creation of a new resource.**"]
    pub secret: PrimField<String>,
}

impl BuildTunnel {
    pub fn build(self, stack: &mut Stack) -> Tunnel {
        let out = Tunnel(Rc::new(Tunnel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TunnelData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                config_src: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                secret: self.secret,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TunnelRef {
    shared: StackShared,
    base: String,
}

impl Ref for TunnelRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TunnelRef {
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

    #[doc= "Get a reference to the value of field `cname` after provisioning.\nUsable CNAME for accessing the Tunnel."]
    pub fn cname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `config_src` after provisioning.\nIndicates if this is a locally or remotely configured tunnel. If `local`, manage the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the tunnel on the Zero Trust dashboard or using tunnel_config, tunnel_route or tunnel_virtual_network resources. Available values: `local`, `cloudflare`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn config_src(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.config_src", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA user-friendly name chosen when the tunnel is created. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret` after provisioning.\n32 or more bytes, encoded as a base64 string. The Create Argo Tunnel endpoint sets this as the tunnel's password. Anyone wishing to run the tunnel needs this password. **Modifying this attribute will force creation of a new resource.**"]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tunnel_token` after provisioning.\nToken used by a connector to authenticate and run the tunnel."]
    pub fn tunnel_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel_token", self.extract_ref()))
    }
}
