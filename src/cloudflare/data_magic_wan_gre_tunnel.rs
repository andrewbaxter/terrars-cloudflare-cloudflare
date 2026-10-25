#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataMagicWanGreTunnelData_ {
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
    gre_tunnel_id: terrars::PrimField<std::string::String>,
}

struct DataMagicWanGreTunnel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMagicWanGreTunnelData_>,
}

#[derive(Clone)]
pub struct DataMagicWanGreTunnel(std::rc::Rc<DataMagicWanGreTunnel_>);

impl DataMagicWanGreTunnel {
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

    #[doc = "Get a reference to the value of field `gre_tunnel` after provisioning.\n"]
    pub fn gre_tunnel(&self) -> DataMagicWanGreTunnel_GreTunnelRef {
        DataMagicWanGreTunnel_GreTunnelRef::new(self.shared().clone(), format!("{}.gre_tunnel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gre_tunnel_id` after provisioning.\nIdentifier"]
    pub fn gre_tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gre_tunnel_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl terrars::Referable for DataMagicWanGreTunnel {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMagicWanGreTunnel { }

impl terrars::ToListMappable for DataMagicWanGreTunnel {
    type O = terrars::ListRef<DataMagicWanGreTunnelRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMagicWanGreTunnel_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_magic_wan_gre_tunnel".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMagicWanGreTunnel {
    pub tf_id: std::string::String,
    #[doc = "Identifier"]
    pub gre_tunnel_id: terrars::PrimField<std::string::String>,
}

impl BuildDataMagicWanGreTunnel {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMagicWanGreTunnel {
        let out = DataMagicWanGreTunnel(std::rc::Rc::new(DataMagicWanGreTunnel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMagicWanGreTunnelData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                gre_tunnel_id: self.gre_tunnel_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMagicWanGreTunnelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanGreTunnelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMagicWanGreTunnelRef {
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

    #[doc = "Get a reference to the value of field `gre_tunnel` after provisioning.\n"]
    pub fn gre_tunnel(&self) -> DataMagicWanGreTunnel_GreTunnelRef {
        DataMagicWanGreTunnel_GreTunnelRef::new(self.shared().clone(), format!("{}.gre_tunnel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gre_tunnel_id` after provisioning.\nIdentifier"]
    pub fn gre_tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gre_tunnel_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMagicWanGreTunnel_GreTunnel_Bgp {}

impl DataMagicWanGreTunnel_GreTunnel_Bgp { }

impl terrars::ToListMappable for DataMagicWanGreTunnel_GreTunnel_Bgp {
    type O = terrars::BlockAssignable<DataMagicWanGreTunnel_GreTunnel_Bgp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicWanGreTunnel_GreTunnel_Bgp {}

impl BuildDataMagicWanGreTunnel_GreTunnel_Bgp {
    pub fn build(self) -> DataMagicWanGreTunnel_GreTunnel_Bgp {
        DataMagicWanGreTunnel_GreTunnel_Bgp {}
    }
}

#[allow(dead_code)]
pub struct DataMagicWanGreTunnel_GreTunnel_BgpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanGreTunnel_GreTunnel_BgpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicWanGreTunnel_GreTunnel_BgpRef {
        DataMagicWanGreTunnel_GreTunnel_BgpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicWanGreTunnel_GreTunnel_BgpRef {
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
pub struct DataMagicWanGreTunnel_GreTunnel_BgpStatus {}

impl DataMagicWanGreTunnel_GreTunnel_BgpStatus { }

impl terrars::ToListMappable for DataMagicWanGreTunnel_GreTunnel_BgpStatus {
    type O = terrars::BlockAssignable<DataMagicWanGreTunnel_GreTunnel_BgpStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicWanGreTunnel_GreTunnel_BgpStatus {}

impl BuildDataMagicWanGreTunnel_GreTunnel_BgpStatus {
    pub fn build(self) -> DataMagicWanGreTunnel_GreTunnel_BgpStatus {
        DataMagicWanGreTunnel_GreTunnel_BgpStatus {}
    }
}

#[allow(dead_code)]
pub struct DataMagicWanGreTunnel_GreTunnel_BgpStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanGreTunnel_GreTunnel_BgpStatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicWanGreTunnel_GreTunnel_BgpStatusRef {
        DataMagicWanGreTunnel_GreTunnel_BgpStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicWanGreTunnel_GreTunnel_BgpStatusRef {
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
pub struct DataMagicWanGreTunnel_GreTunnel_HealthCheck_Target {}

impl DataMagicWanGreTunnel_GreTunnel_HealthCheck_Target { }

impl terrars::ToListMappable for DataMagicWanGreTunnel_GreTunnel_HealthCheck_Target {
    type O = terrars::BlockAssignable<DataMagicWanGreTunnel_GreTunnel_HealthCheck_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicWanGreTunnel_GreTunnel_HealthCheck_Target {}

impl BuildDataMagicWanGreTunnel_GreTunnel_HealthCheck_Target {
    pub fn build(self) -> DataMagicWanGreTunnel_GreTunnel_HealthCheck_Target {
        DataMagicWanGreTunnel_GreTunnel_HealthCheck_Target {}
    }
}

#[allow(dead_code)]
pub struct DataMagicWanGreTunnel_GreTunnel_HealthCheck_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanGreTunnel_GreTunnel_HealthCheck_TargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMagicWanGreTunnel_GreTunnel_HealthCheck_TargetRef {
        DataMagicWanGreTunnel_GreTunnel_HealthCheck_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicWanGreTunnel_GreTunnel_HealthCheck_TargetRef {
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
pub struct DataMagicWanGreTunnel_GreTunnel_HealthCheck {}

impl DataMagicWanGreTunnel_GreTunnel_HealthCheck { }

impl terrars::ToListMappable for DataMagicWanGreTunnel_GreTunnel_HealthCheck {
    type O = terrars::BlockAssignable<DataMagicWanGreTunnel_GreTunnel_HealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicWanGreTunnel_GreTunnel_HealthCheck {}

impl BuildDataMagicWanGreTunnel_GreTunnel_HealthCheck {
    pub fn build(self) -> DataMagicWanGreTunnel_GreTunnel_HealthCheck {
        DataMagicWanGreTunnel_GreTunnel_HealthCheck {}
    }
}

#[allow(dead_code)]
pub struct DataMagicWanGreTunnel_GreTunnel_HealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanGreTunnel_GreTunnel_HealthCheckRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMagicWanGreTunnel_GreTunnel_HealthCheckRef {
        DataMagicWanGreTunnel_GreTunnel_HealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicWanGreTunnel_GreTunnel_HealthCheckRef {
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
    pub fn target(&self) -> DataMagicWanGreTunnel_GreTunnel_HealthCheck_TargetRef {
        DataMagicWanGreTunnel_GreTunnel_HealthCheck_TargetRef::new(
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
pub struct DataMagicWanGreTunnel_GreTunnel {}

impl DataMagicWanGreTunnel_GreTunnel { }

impl terrars::ToListMappable for DataMagicWanGreTunnel_GreTunnel {
    type O = terrars::BlockAssignable<DataMagicWanGreTunnel_GreTunnel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicWanGreTunnel_GreTunnel {}

impl BuildDataMagicWanGreTunnel_GreTunnel {
    pub fn build(self) -> DataMagicWanGreTunnel_GreTunnel {
        DataMagicWanGreTunnel_GreTunnel {}
    }
}

#[allow(dead_code)]
pub struct DataMagicWanGreTunnel_GreTunnelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanGreTunnel_GreTunnelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicWanGreTunnel_GreTunnelRef {
        DataMagicWanGreTunnel_GreTunnelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicWanGreTunnel_GreTunnelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `automatic_return_routing` after provisioning.\nTrue if automatic stateful return routing should be enabled for a tunnel, false otherwise. Requires the `coupler_integration` account flag to be enabled; requests setting this to `true` without that flag will be rejected."]
    pub fn automatic_return_routing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_return_routing", self.base))
    }

    #[doc = "Get a reference to the value of field `bgp` after provisioning.\n"]
    pub fn bgp(&self) -> DataMagicWanGreTunnel_GreTunnel_BgpRef {
        DataMagicWanGreTunnel_GreTunnel_BgpRef::new(self.shared().clone(), format!("{}.bgp", self.base))
    }

    #[doc = "Get a reference to the value of field `bgp_status` after provisioning.\n"]
    pub fn bgp_status(&self) -> DataMagicWanGreTunnel_GreTunnel_BgpStatusRef {
        DataMagicWanGreTunnel_GreTunnel_BgpStatusRef::new(self.shared().clone(), format!("{}.bgp_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_gre_endpoint` after provisioning.\nThe IP address assigned to the Cloudflare side of the GRE tunnel."]
    pub fn cloudflare_gre_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_gre_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe date and time the tunnel was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `customer_gre_endpoint` after provisioning.\nThe IP address assigned to the customer side of the GRE tunnel."]
    pub fn customer_gre_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_gre_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of the GRE tunnel."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> DataMagicWanGreTunnel_GreTunnel_HealthCheckRef {
        DataMagicWanGreTunnel_GreTunnel_HealthCheckRef::new(
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
        "Get a reference to the value of field `mtu` after provisioning.\nMaximum Transmission Unit (MTU) in bytes for the GRE tunnel. The minimum value is 576."]
    pub fn mtu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mtu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the tunnel. The name cannot contain spaces or special characters, must be 15 characters or less, and cannot share a name with another GRE tunnel."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nTime To Live (TTL) in number of hops of the GRE tunnel."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}
