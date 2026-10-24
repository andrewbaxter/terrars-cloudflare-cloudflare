use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct IpsecTunnelData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_null_cipher: std::option::Option<terrars::PrimField<bool>>,
    cloudflare_endpoint: terrars::PrimField<std::string::String>,
    customer_endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fqdn_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hex_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    interface_address: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psk: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct IpsecTunnel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IpsecTunnelData>,
}

#[derive(Clone)]
pub struct IpsecTunnel(std::rc::Rc<IpsecTunnel_>);

impl IpsecTunnel {
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

    #[doc =
        "Set the field `account_id`.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_null_cipher`.\nSpecifies if this tunnel may use a null cipher (ENCR_NULL) in Phase 2. Defaults to `false`."]
    pub fn set_allow_null_cipher(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_null_cipher = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nAn optional description of the IPsec tunnel."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fqdn_id`.\n`remote_id` in the form of a fqdn. This value is generated by cloudflare."]
    pub fn set_fqdn_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().fqdn_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `health_check_enabled`.\nSpecifies if ICMP tunnel health checks are enabled. Default: `true`."]
    pub fn set_health_check_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().health_check_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `health_check_target`.\nThe IP address of the customer endpoint that will receive tunnel health checks. Default: `<customer_gre_endpoint>`."]
    pub fn set_health_check_target(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().health_check_target = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `health_check_type`.\nSpecifies the ICMP echo type for the health check (`request` or `reply`). Available values: `request`, `reply` Default: `reply`."]
    pub fn set_health_check_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().health_check_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hex_id`.\n`remote_id` as a hex string. This value is generated by cloudflare."]
    pub fn set_hex_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hex_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `psk`.\nPre shared key to be used with the IPsec tunnel. If left unset, it will be autogenerated."]
    pub fn set_psk(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().psk = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `remote_id`.\nID to be used while setting up the IPsec tunnel. This value is generated by cloudflare."]
    pub fn set_remote_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().remote_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user_id`.\n`remote_id` in the form of an email address. This value is generated by cloudflare."]
    pub fn set_user_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_null_cipher` after provisioning.\nSpecifies if this tunnel may use a null cipher (ENCR_NULL) in Phase 2. Defaults to `false`."]
    pub fn allow_null_cipher(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_null_cipher", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_endpoint` after provisioning.\nIP address assigned to the Cloudflare side of the IPsec tunnel."]
    pub fn cloudflare_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_endpoint` after provisioning.\nIP address assigned to the customer side of the IPsec tunnel."]
    pub fn customer_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of the IPsec tunnel."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fqdn_id` after provisioning.\n`remote_id` in the form of a fqdn. This value is generated by cloudflare."]
    pub fn fqdn_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fqdn_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_check_enabled` after provisioning.\nSpecifies if ICMP tunnel health checks are enabled. Default: `true`."]
    pub fn health_check_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_check_target` after provisioning.\nThe IP address of the customer endpoint that will receive tunnel health checks. Default: `<customer_gre_endpoint>`."]
    pub fn health_check_target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_target", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_check_type` after provisioning.\nSpecifies the ICMP echo type for the health check (`request` or `reply`). Available values: `request`, `reply` Default: `reply`."]
    pub fn health_check_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hex_id` after provisioning.\n`remote_id` as a hex string. This value is generated by cloudflare."]
    pub fn hex_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hex_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interface_address` after provisioning.\n31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel."]
    pub fn interface_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the IPsec tunnel."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psk` after provisioning.\nPre shared key to be used with the IPsec tunnel. If left unset, it will be autogenerated."]
    pub fn psk(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psk", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remote_id` after provisioning.\nID to be used while setting up the IPsec tunnel. This value is generated by cloudflare."]
    pub fn remote_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_id` after provisioning.\n`remote_id` in the form of an email address. This value is generated by cloudflare."]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }
}

impl terrars::Referable for IpsecTunnel {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IpsecTunnel { }

impl terrars::ToListMappable for IpsecTunnel {
    type O = terrars::ListRef<IpsecTunnelRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IpsecTunnel_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_ipsec_tunnel".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIpsecTunnel {
    pub tf_id: std::string::String,
    #[doc = "IP address assigned to the Cloudflare side of the IPsec tunnel."]
    pub cloudflare_endpoint: terrars::PrimField<std::string::String>,
    #[doc = "IP address assigned to the customer side of the IPsec tunnel."]
    pub customer_endpoint: terrars::PrimField<std::string::String>,
    #[doc = "31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel."]
    pub interface_address: terrars::PrimField<std::string::String>,
    #[doc = "Name of the IPsec tunnel."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildIpsecTunnel {
    pub fn build(self, stack: &mut terrars::Stack) -> IpsecTunnel {
        let out = IpsecTunnel(std::rc::Rc::new(IpsecTunnel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IpsecTunnelData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IpsecTunnelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IpsecTunnelRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_null_cipher` after provisioning.\nSpecifies if this tunnel may use a null cipher (ENCR_NULL) in Phase 2. Defaults to `false`."]
    pub fn allow_null_cipher(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_null_cipher", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_endpoint` after provisioning.\nIP address assigned to the Cloudflare side of the IPsec tunnel."]
    pub fn cloudflare_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_endpoint` after provisioning.\nIP address assigned to the customer side of the IPsec tunnel."]
    pub fn customer_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of the IPsec tunnel."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fqdn_id` after provisioning.\n`remote_id` in the form of a fqdn. This value is generated by cloudflare."]
    pub fn fqdn_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fqdn_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_check_enabled` after provisioning.\nSpecifies if ICMP tunnel health checks are enabled. Default: `true`."]
    pub fn health_check_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_check_target` after provisioning.\nThe IP address of the customer endpoint that will receive tunnel health checks. Default: `<customer_gre_endpoint>`."]
    pub fn health_check_target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_target", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_check_type` after provisioning.\nSpecifies the ICMP echo type for the health check (`request` or `reply`). Available values: `request`, `reply` Default: `reply`."]
    pub fn health_check_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hex_id` after provisioning.\n`remote_id` as a hex string. This value is generated by cloudflare."]
    pub fn hex_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hex_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interface_address` after provisioning.\n31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel."]
    pub fn interface_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the IPsec tunnel."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psk` after provisioning.\nPre shared key to be used with the IPsec tunnel. If left unset, it will be autogenerated."]
    pub fn psk(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psk", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remote_id` after provisioning.\nID to be used while setting up the IPsec tunnel. This value is generated by cloudflare."]
    pub fn remote_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_id` after provisioning.\n`remote_id` in the form of an email address. This value is generated by cloudflare."]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }
}
