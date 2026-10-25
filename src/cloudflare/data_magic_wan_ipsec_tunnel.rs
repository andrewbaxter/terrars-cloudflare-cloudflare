#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataMagicWanIpsecTunnelData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    ipsec_tunnel_id: terrars::PrimField<std::string::String>,
}

struct DataMagicWanIpsecTunnel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMagicWanIpsecTunnelData_>,
}

#[derive(Clone)]
pub struct DataMagicWanIpsecTunnel(std::rc::Rc<DataMagicWanIpsecTunnel_>);

impl DataMagicWanIpsecTunnel {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderCloudflare) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `account_id`.\nIdentifier"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipsec_tunnel` after provisioning.\n"]
    pub fn ipsec_tunnel(&self) -> DataMagicWanIpsecTunnel_IpsecTunnelRef {
        DataMagicWanIpsecTunnel_IpsecTunnelRef::new(
            self.shared().clone(),
            format!("{}.ipsec_tunnel", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipsec_tunnel_id` after provisioning.\nIdentifier"]
    pub fn ipsec_tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipsec_tunnel_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataMagicWanIpsecTunnel {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMagicWanIpsecTunnel { }

impl terrars::ToListMappable for DataMagicWanIpsecTunnel {
    type O = terrars::ListRef<DataMagicWanIpsecTunnelRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMagicWanIpsecTunnel_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_magic_wan_ipsec_tunnel".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMagicWanIpsecTunnel {
    pub tf_id: std::string::String,
    #[doc = "Identifier"]
    pub ipsec_tunnel_id: terrars::PrimField<std::string::String>,
}

impl BuildDataMagicWanIpsecTunnel {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMagicWanIpsecTunnel {
        let out = DataMagicWanIpsecTunnel(std::rc::Rc::new(DataMagicWanIpsecTunnel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMagicWanIpsecTunnelData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                ipsec_tunnel_id: self.ipsec_tunnel_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMagicWanIpsecTunnelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanIpsecTunnelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMagicWanIpsecTunnelRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipsec_tunnel` after provisioning.\n"]
    pub fn ipsec_tunnel(&self) -> DataMagicWanIpsecTunnel_IpsecTunnelRef {
        DataMagicWanIpsecTunnel_IpsecTunnelRef::new(
            self.shared().clone(),
            format!("{}.ipsec_tunnel", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipsec_tunnel_id` after provisioning.\nIdentifier"]
    pub fn ipsec_tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipsec_tunnel_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMagicWanIpsecTunnel_IpsecTunnel_Bgp {}

impl DataMagicWanIpsecTunnel_IpsecTunnel_Bgp { }

impl terrars::ToListMappable for DataMagicWanIpsecTunnel_IpsecTunnel_Bgp {
    type O = terrars::BlockAssignable<DataMagicWanIpsecTunnel_IpsecTunnel_Bgp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicWanIpsecTunnel_IpsecTunnel_Bgp {}

impl BuildDataMagicWanIpsecTunnel_IpsecTunnel_Bgp {
    pub fn build(self) -> DataMagicWanIpsecTunnel_IpsecTunnel_Bgp {
        DataMagicWanIpsecTunnel_IpsecTunnel_Bgp {}
    }
}

#[allow(dead_code)]
pub struct DataMagicWanIpsecTunnel_IpsecTunnel_BgpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanIpsecTunnel_IpsecTunnel_BgpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicWanIpsecTunnel_IpsecTunnel_BgpRef {
        DataMagicWanIpsecTunnel_IpsecTunnel_BgpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicWanIpsecTunnel_IpsecTunnel_BgpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `customer_asn` after provisioning.\nASN used on the customer end of the BGP session"]
    pub fn customer_asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_asn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `export_filter_id` after provisioning.\nID of the BGP filter profile applied to routes advertised to the customer."]
    pub fn export_filter_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_filter_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `extra_prefixes` after provisioning.\nPrefixes in this list will be advertised to the customer device, in addition to the routes in the Magic routing table."]
    pub fn extra_prefixes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extra_prefixes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `import_filter_id` after provisioning.\nID of the BGP filter profile applied to routes received from the customer."]
    pub fn import_filter_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.import_filter_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `md5_key` after provisioning.\nMD5 key to use for session authentication.\n\nNote that *this is not a security measure*. MD5 is not a valid security mechanism, and the\nkey is not treated as a secret value. This is *only* supported for preventing\nmisconfiguration, not for defending against malicious attacks.\n\nThe MD5 key, if set, must be of non-zero length and consist only of the following types of\ncharacter:\n\n* ASCII alphanumerics: `[a-zA-Z0-9]`\n* Special characters in the set `'!@#$%^&*()+[]{}<>/.,;:_-~`= \\|`\n\nIn other words, MD5 keys may contain any printable ASCII character aside from newline (0x0A),\nquotation mark (`\"`), vertical tab (0x0B), carriage return (0x0D), tab (0x09), form feed\n(0x0C), and the question mark (`?`). Requests specifying an MD5 key with one or more of\nthese disallowed characters will be rejected."]
    pub fn md5_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.md5_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicWanIpsecTunnel_IpsecTunnel_BgpStatus {}

impl DataMagicWanIpsecTunnel_IpsecTunnel_BgpStatus { }

impl terrars::ToListMappable for DataMagicWanIpsecTunnel_IpsecTunnel_BgpStatus {
    type O = terrars::BlockAssignable<DataMagicWanIpsecTunnel_IpsecTunnel_BgpStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicWanIpsecTunnel_IpsecTunnel_BgpStatus {}

impl BuildDataMagicWanIpsecTunnel_IpsecTunnel_BgpStatus {
    pub fn build(self) -> DataMagicWanIpsecTunnel_IpsecTunnel_BgpStatus {
        DataMagicWanIpsecTunnel_IpsecTunnel_BgpStatus {}
    }
}

#[allow(dead_code)]
pub struct DataMagicWanIpsecTunnel_IpsecTunnel_BgpStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanIpsecTunnel_IpsecTunnel_BgpStatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMagicWanIpsecTunnel_IpsecTunnel_BgpStatusRef {
        DataMagicWanIpsecTunnel_IpsecTunnel_BgpStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicWanIpsecTunnel_IpsecTunnel_BgpStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bgp_state` after provisioning.\n"]
    pub fn bgp_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgp_state", self.base))
    }

    #[doc = "Get a reference to the value of field `cf_speaker_ip` after provisioning.\n"]
    pub fn cf_speaker_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cf_speaker_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `cf_speaker_port` after provisioning.\n"]
    pub fn cf_speaker_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cf_speaker_port", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_speaker_ip` after provisioning.\n"]
    pub fn customer_speaker_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_speaker_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_speaker_port` after provisioning.\n"]
    pub fn customer_speaker_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_speaker_port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nAvailable values: \"BGP_DOWN\", \"BGP_UP\", \"BGP_ESTABLISHING\"."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_established` after provisioning.\n"]
    pub fn tcp_established(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_established", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicWanIpsecTunnel_IpsecTunnel_CustomRemoteIdentities {}

impl DataMagicWanIpsecTunnel_IpsecTunnel_CustomRemoteIdentities { }

impl terrars::ToListMappable for DataMagicWanIpsecTunnel_IpsecTunnel_CustomRemoteIdentities {
    type O = terrars::BlockAssignable<DataMagicWanIpsecTunnel_IpsecTunnel_CustomRemoteIdentities>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicWanIpsecTunnel_IpsecTunnel_CustomRemoteIdentities {}

impl BuildDataMagicWanIpsecTunnel_IpsecTunnel_CustomRemoteIdentities {
    pub fn build(self) -> DataMagicWanIpsecTunnel_IpsecTunnel_CustomRemoteIdentities {
        DataMagicWanIpsecTunnel_IpsecTunnel_CustomRemoteIdentities {}
    }
}

#[allow(dead_code)]
pub struct DataMagicWanIpsecTunnel_IpsecTunnel_CustomRemoteIdentitiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanIpsecTunnel_IpsecTunnel_CustomRemoteIdentitiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMagicWanIpsecTunnel_IpsecTunnel_CustomRemoteIdentitiesRef {
        DataMagicWanIpsecTunnel_IpsecTunnel_CustomRemoteIdentitiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicWanIpsecTunnel_IpsecTunnel_CustomRemoteIdentitiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fqdn_id` after provisioning.\nA custom IKE ID of type FQDN that may be used to identity the IPsec tunnel. The\ngenerated IKE IDs can still be used even if this custom value is specified.\n\nMust be of the form `<custom label>.<account ID>.custom.ipsec.cloudflare.com`.\n\nThis custom ID does not need to be unique. Two IPsec tunnels may have the same custom\nfqdn_id. However, if another IPsec tunnel has the same value then the two tunnels\ncannot have the same cloudflare_endpoint."]
    pub fn fqdn_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fqdn_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck_Target {}

impl DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck_Target { }

impl terrars::ToListMappable for DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck_Target {
    type O = terrars::BlockAssignable<DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck_Target {}

impl BuildDataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck_Target {
    pub fn build(self) -> DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck_Target {
        DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck_Target {}
    }
}

#[allow(dead_code)]
pub struct DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck_TargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck_TargetRef {
        DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck_TargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effective` after provisioning.\nThe effective health check target. If 'saved' is empty, then this field will be populated with the calculated default value on GET requests. Ignored in POST, PUT, and PATCH requests."]
    pub fn effective(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective", self.base))
    }

    #[doc =
        "Get a reference to the value of field `saved` after provisioning.\nThe saved health check target. Setting the value to the empty string indicates that the calculated default value will be used."]
    pub fn saved(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.saved", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck {}

impl DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck { }

impl terrars::ToListMappable for DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck {
    type O = terrars::BlockAssignable<DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck {}

impl BuildDataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck {
    pub fn build(self) -> DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck {
        DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck {}
    }
}

#[allow(dead_code)]
pub struct DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheckRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheckRef {
        DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe direction of the flow of the healthcheck. Either unidirectional, where the probe comes to you via the tunnel and the result comes back to Cloudflare via the open Internet, or bidirectional where both the probe and result come and go via the tunnel.\nAvailable values: \"unidirectional\", \"bidirectional\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nDetermines whether to run healthchecks for a tunnel."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rate` after provisioning.\nHow frequent the health check is run. The default value is `mid`.\nAvailable values: \"low\", \"mid\", \"high\"."]
    pub fn rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nThe destination address in a request type health check. After the healthcheck is decapsulated at the customer end of the tunnel, the ICMP echo will be forwarded to this address. This field defaults to `customer_gre_endpoint address`. This field is ignored for bidirectional healthchecks as the interface_address (not assigned to the Cloudflare side of the tunnel) is used as the target. Must be in object form if the x-magic-new-hc-target header is set to true and string form if x-magic-new-hc-target is absent or set to false."]
    pub fn target(&self) -> DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck_TargetRef {
        DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheck_TargetRef::new(
            self.shared().clone(),
            format!("{}.target", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of healthcheck to run, reply or request. The default value is `reply`.\nAvailable values: \"reply\", \"request\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicWanIpsecTunnel_IpsecTunnel_PskMetadata {}

impl DataMagicWanIpsecTunnel_IpsecTunnel_PskMetadata { }

impl terrars::ToListMappable for DataMagicWanIpsecTunnel_IpsecTunnel_PskMetadata {
    type O = terrars::BlockAssignable<DataMagicWanIpsecTunnel_IpsecTunnel_PskMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicWanIpsecTunnel_IpsecTunnel_PskMetadata {}

impl BuildDataMagicWanIpsecTunnel_IpsecTunnel_PskMetadata {
    pub fn build(self) -> DataMagicWanIpsecTunnel_IpsecTunnel_PskMetadata {
        DataMagicWanIpsecTunnel_IpsecTunnel_PskMetadata {}
    }
}

#[allow(dead_code)]
pub struct DataMagicWanIpsecTunnel_IpsecTunnel_PskMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanIpsecTunnel_IpsecTunnel_PskMetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMagicWanIpsecTunnel_IpsecTunnel_PskMetadataRef {
        DataMagicWanIpsecTunnel_IpsecTunnel_PskMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicWanIpsecTunnel_IpsecTunnel_PskMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `last_generated_on` after provisioning.\nThe date and time the tunnel was last modified."]
    pub fn last_generated_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_generated_on", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicWanIpsecTunnel_IpsecTunnel {}

impl DataMagicWanIpsecTunnel_IpsecTunnel { }

impl terrars::ToListMappable for DataMagicWanIpsecTunnel_IpsecTunnel {
    type O = terrars::BlockAssignable<DataMagicWanIpsecTunnel_IpsecTunnel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicWanIpsecTunnel_IpsecTunnel {}

impl BuildDataMagicWanIpsecTunnel_IpsecTunnel {
    pub fn build(self) -> DataMagicWanIpsecTunnel_IpsecTunnel {
        DataMagicWanIpsecTunnel_IpsecTunnel {}
    }
}

#[allow(dead_code)]
pub struct DataMagicWanIpsecTunnel_IpsecTunnelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanIpsecTunnel_IpsecTunnelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicWanIpsecTunnel_IpsecTunnelRef {
        DataMagicWanIpsecTunnel_IpsecTunnelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicWanIpsecTunnel_IpsecTunnelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_null_cipher` after provisioning.\nWhen `true`, the tunnel can use a null-cipher (`ENCR_NULL`) in the ESP tunnel (Phase 2)."]
    pub fn allow_null_cipher(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_null_cipher", self.base))
    }

    #[doc =
        "Get a reference to the value of field `automatic_return_routing` after provisioning.\nTrue if automatic stateful return routing should be enabled for a tunnel, false otherwise. Requires the `coupler_integration` account flag to be enabled; requests setting this to `true` without that flag will be rejected."]
    pub fn automatic_return_routing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_return_routing", self.base))
    }

    #[doc = "Get a reference to the value of field `bgp` after provisioning.\n"]
    pub fn bgp(&self) -> DataMagicWanIpsecTunnel_IpsecTunnel_BgpRef {
        DataMagicWanIpsecTunnel_IpsecTunnel_BgpRef::new(self.shared().clone(), format!("{}.bgp", self.base))
    }

    #[doc = "Get a reference to the value of field `bgp_status` after provisioning.\n"]
    pub fn bgp_status(&self) -> DataMagicWanIpsecTunnel_IpsecTunnel_BgpStatusRef {
        DataMagicWanIpsecTunnel_IpsecTunnel_BgpStatusRef::new(
            self.shared().clone(),
            format!("{}.bgp_status", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_endpoint` after provisioning.\nThe IP address assigned to the Cloudflare side of the IPsec tunnel."]
    pub fn cloudflare_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe date and time the tunnel was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_remote_identities` after provisioning.\n"]
    pub fn custom_remote_identities(&self) -> DataMagicWanIpsecTunnel_IpsecTunnel_CustomRemoteIdentitiesRef {
        DataMagicWanIpsecTunnel_IpsecTunnel_CustomRemoteIdentitiesRef::new(
            self.shared().clone(),
            format!("{}.custom_remote_identities", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_endpoint` after provisioning.\nThe IP address assigned to the customer side of the IPsec tunnel. Not required, but must be set for proactive traceroutes to work."]
    pub fn customer_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description forthe IPsec tunnel."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheckRef {
        DataMagicWanIpsecTunnel_IpsecTunnel_HealthCheckRef::new(
            self.shared().clone(),
            format!("{}.health_check", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interface_address` after provisioning.\nA 31-bit prefix (/31 in CIDR notation) supporting two hosts, one for each side of the tunnel. Select the subnet from the following private IP space: 10.0.0.0–10.255.255.255, 172.16.0.0–172.31.255.255, 192.168.0.0–192.168.255.255."]
    pub fn interface_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interface_address6` after provisioning.\nA 127 bit IPV6 prefix from within the virtual_subnet6 prefix space with the address being the first IP of the subnet and not same as the address of virtual_subnet6. Eg if virtual_subnet6 is 2606:54c1:7:0:a9fe:12d2::/127 , interface_address6 could be 2606:54c1:7:0:a9fe:12d2:1:200/127"]
    pub fn interface_address6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_address6", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe date and time the tunnel was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the IPsec tunnel. The name cannot share a name with other tunnels."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `psk_metadata` after provisioning.\nThe PSK metadata that includes when the PSK was generated."]
    pub fn psk_metadata(&self) -> DataMagicWanIpsecTunnel_IpsecTunnel_PskMetadataRef {
        DataMagicWanIpsecTunnel_IpsecTunnel_PskMetadataRef::new(
            self.shared().clone(),
            format!("{}.psk_metadata", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `replay_protection` after provisioning.\nIf `true`, then IPsec replay protection will be supported in the Cloudflare-to-customer direction."]
    pub fn replay_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replay_protection", self.base))
    }
}
