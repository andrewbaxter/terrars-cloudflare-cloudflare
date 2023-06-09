use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct IpsecTunnelData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_null_cipher: Option<PrimField<bool>>,
    cloudflare_endpoint: PrimField<String>,
    customer_endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fqdn_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hex_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    interface_address: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psk: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<String>>,
}

struct IpsecTunnel_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IpsecTunnelData>,
}

#[derive(Clone)]
pub struct IpsecTunnel(Rc<IpsecTunnel_>);

impl IpsecTunnel {
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

    #[doc= "Set the field `account_id`.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_null_cipher`.\nSpecifies if this tunnel may use a null cipher (ENCR_NULL) in Phase 2. Defaults to `false`."]
    pub fn set_allow_null_cipher(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_null_cipher = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nAn optional description of the IPsec tunnel."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `fqdn_id`.\n`remote_id` in the form of a fqdn. This value is generated by cloudflare."]
    pub fn set_fqdn_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fqdn_id = Some(v.into());
        self
    }

    #[doc= "Set the field `health_check_enabled`.\nSpecifies if ICMP tunnel health checks are enabled. Default: `true`."]
    pub fn set_health_check_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().health_check_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `health_check_target`.\nThe IP address of the customer endpoint that will receive tunnel health checks. Default: `<customer_gre_endpoint>`."]
    pub fn set_health_check_target(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().health_check_target = Some(v.into());
        self
    }

    #[doc= "Set the field `health_check_type`.\nSpecifies the ICMP echo type for the health check (`request` or `reply`). Available values: `request`, `reply` Default: `reply`."]
    pub fn set_health_check_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().health_check_type = Some(v.into());
        self
    }

    #[doc= "Set the field `hex_id`.\n`remote_id` as a hex string. This value is generated by cloudflare."]
    pub fn set_hex_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().hex_id = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `psk`.\nPre shared key to be used with the IPsec tunnel. If left unset, it will be autogenerated."]
    pub fn set_psk(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().psk = Some(v.into());
        self
    }

    #[doc= "Set the field `remote_id`.\nID to be used while setting up the IPsec tunnel. This value is generated by cloudflare."]
    pub fn set_remote_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().remote_id = Some(v.into());
        self
    }

    #[doc= "Set the field `user_id`.\n`remote_id` in the form of an email address. This value is generated by cloudflare."]
    pub fn set_user_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_null_cipher` after provisioning.\nSpecifies if this tunnel may use a null cipher (ENCR_NULL) in Phase 2. Defaults to `false`."]
    pub fn allow_null_cipher(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_null_cipher", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloudflare_endpoint` after provisioning.\nIP address assigned to the Cloudflare side of the IPsec tunnel."]
    pub fn cloudflare_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer_endpoint` after provisioning.\nIP address assigned to the customer side of the IPsec tunnel."]
    pub fn customer_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of the IPsec tunnel."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fqdn_id` after provisioning.\n`remote_id` in the form of a fqdn. This value is generated by cloudflare."]
    pub fn fqdn_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fqdn_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_enabled` after provisioning.\nSpecifies if ICMP tunnel health checks are enabled. Default: `true`."]
    pub fn health_check_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_target` after provisioning.\nThe IP address of the customer endpoint that will receive tunnel health checks. Default: `<customer_gre_endpoint>`."]
    pub fn health_check_target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_target", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_type` after provisioning.\nSpecifies the ICMP echo type for the health check (`request` or `reply`). Available values: `request`, `reply` Default: `reply`."]
    pub fn health_check_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hex_id` after provisioning.\n`remote_id` as a hex string. This value is generated by cloudflare."]
    pub fn hex_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hex_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interface_address` after provisioning.\n31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel."]
    pub fn interface_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interface_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the IPsec tunnel."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `psk` after provisioning.\nPre shared key to be used with the IPsec tunnel. If left unset, it will be autogenerated."]
    pub fn psk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.psk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remote_id` after provisioning.\nID to be used while setting up the IPsec tunnel. This value is generated by cloudflare."]
    pub fn remote_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.remote_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\n`remote_id` in the form of an email address. This value is generated by cloudflare."]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }
}

impl Referable for IpsecTunnel {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for IpsecTunnel { }

impl ToListMappable for IpsecTunnel {
    type O = ListRef<IpsecTunnelRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IpsecTunnel_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_ipsec_tunnel".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIpsecTunnel {
    pub tf_id: String,
    #[doc= "IP address assigned to the Cloudflare side of the IPsec tunnel."]
    pub cloudflare_endpoint: PrimField<String>,
    #[doc= "IP address assigned to the customer side of the IPsec tunnel."]
    pub customer_endpoint: PrimField<String>,
    #[doc= "31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel."]
    pub interface_address: PrimField<String>,
    #[doc= "Name of the IPsec tunnel."]
    pub name: PrimField<String>,
}

impl BuildIpsecTunnel {
    pub fn build(self, stack: &mut Stack) -> IpsecTunnel {
        let out = IpsecTunnel(Rc::new(IpsecTunnel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IpsecTunnelData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                allow_null_cipher: core::default::Default::default(),
                cloudflare_endpoint: self.cloudflare_endpoint,
                customer_endpoint: self.customer_endpoint,
                description: core::default::Default::default(),
                fqdn_id: core::default::Default::default(),
                health_check_enabled: core::default::Default::default(),
                health_check_target: core::default::Default::default(),
                health_check_type: core::default::Default::default(),
                hex_id: core::default::Default::default(),
                id: core::default::Default::default(),
                interface_address: self.interface_address,
                name: self.name,
                psk: core::default::Default::default(),
                remote_id: core::default::Default::default(),
                user_id: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IpsecTunnelRef {
    shared: StackShared,
    base: String,
}

impl Ref for IpsecTunnelRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IpsecTunnelRef {
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

    #[doc= "Get a reference to the value of field `allow_null_cipher` after provisioning.\nSpecifies if this tunnel may use a null cipher (ENCR_NULL) in Phase 2. Defaults to `false`."]
    pub fn allow_null_cipher(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_null_cipher", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloudflare_endpoint` after provisioning.\nIP address assigned to the Cloudflare side of the IPsec tunnel."]
    pub fn cloudflare_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer_endpoint` after provisioning.\nIP address assigned to the customer side of the IPsec tunnel."]
    pub fn customer_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of the IPsec tunnel."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fqdn_id` after provisioning.\n`remote_id` in the form of a fqdn. This value is generated by cloudflare."]
    pub fn fqdn_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fqdn_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_enabled` after provisioning.\nSpecifies if ICMP tunnel health checks are enabled. Default: `true`."]
    pub fn health_check_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_target` after provisioning.\nThe IP address of the customer endpoint that will receive tunnel health checks. Default: `<customer_gre_endpoint>`."]
    pub fn health_check_target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_target", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_type` after provisioning.\nSpecifies the ICMP echo type for the health check (`request` or `reply`). Available values: `request`, `reply` Default: `reply`."]
    pub fn health_check_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hex_id` after provisioning.\n`remote_id` as a hex string. This value is generated by cloudflare."]
    pub fn hex_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hex_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interface_address` after provisioning.\n31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel."]
    pub fn interface_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interface_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the IPsec tunnel."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `psk` after provisioning.\nPre shared key to be used with the IPsec tunnel. If left unset, it will be autogenerated."]
    pub fn psk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.psk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remote_id` after provisioning.\nID to be used while setting up the IPsec tunnel. This value is generated by cloudflare."]
    pub fn remote_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.remote_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\n`remote_id` in the form of an email address. This value is generated by cloudflare."]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }
}
