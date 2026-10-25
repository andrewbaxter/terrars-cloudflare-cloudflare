#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataCustomSslData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_certificate_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataCustomSsl_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCustomSsl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCustomSslData_>,
}

#[derive(Clone)]
pub struct DataCustomSsl(std::rc::Rc<DataCustomSsl_>);

impl DataCustomSsl {
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

    #[doc = "Set the field `custom_certificate_id`.\nCustom certificate identifier tag."]
    pub fn set_custom_certificate_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().custom_certificate_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataCustomSsl_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `bundle_method` after provisioning.\nA ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it.\nAvailable values: \"ubiquitous\", \"optimal\", \"force\"."]
    pub fn bundle_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_certificate_id` after provisioning.\nCustom certificate identifier tag."]
    pub fn custom_certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_certificate_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_csr_id` after provisioning.\nThe identifier for the Custom CSR that was used."]
    pub fn custom_csr_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_csr_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expires_on` after provisioning.\nWhen the certificate from the authority expires."]
    pub fn expires_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataCustomSsl_FilterRef {
        DataCustomSsl_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `geo_restrictions` after provisioning.\nSpecify the region where your private key can be held locally for optimal TLS performance. HTTPS connections to any excluded data center will still be fully encrypted, but will incur some latency while Keyless SSL is used to complete the handshake with the nearest allowed data center. Options allow distribution to only to U.S. data centers, only to E.U. data centers, or only to highest security data centers. Default distribution is to all Cloudflare datacenters, for optimal performance."]
    pub fn geo_restrictions(&self) -> DataCustomSsl_GeoRestrictionsRef {
        DataCustomSsl_GeoRestrictionsRef::new(
            self.shared().clone(),
            format!("{}.geo_restrictions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hosts` after provisioning.\n"]
    pub fn hosts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nCustom certificate identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `issuer` after provisioning.\nThe certificate authority that issued the certificate."]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `keyless_server` after provisioning.\n"]
    pub fn keyless_server(&self) -> DataCustomSsl_KeylessServerRef {
        DataCustomSsl_KeylessServerRef::new(self.shared().clone(), format!("{}.keyless_server", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the certificate was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_restrictions` after provisioning.\nThe policy restrictions returned by the API. This field is returned in responses\nwhen a policy has been set. The API accepts the \"policy\" field in requests but\nreturns this field as \"policy_restrictions\" in responses.\n\nSpecifies the region(s) where your private key can be held locally for optimal\nTLS performance. Format is a boolean expression, for example:\n\"(country: US) or (region: EU)\""]
    pub fn policy_restrictions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_restrictions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nThe order/priority in which the certificate will be used in a request. The higher priority will break ties across overlapping 'legacy_custom' certificates, but 'legacy_custom' certificates will always supercede 'sni_custom' certificates."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `signature` after provisioning.\nThe type of hash used for the certificate."]
    pub fn signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the zone's custom SSL.\nAvailable values: \"active\", \"expired\", \"deleted\", \"pending\", \"initializing\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uploaded_on` after provisioning.\nWhen the certificate was uploaded to Cloudflare."]
    pub fn uploaded_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uploaded_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataCustomSsl {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCustomSsl { }

impl terrars::ToListMappable for DataCustomSsl {
    type O = terrars::ListRef<DataCustomSslRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCustomSsl_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_custom_ssl".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCustomSsl {
    pub tf_id: std::string::String,
}

impl BuildDataCustomSsl {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCustomSsl {
        let out = DataCustomSsl(std::rc::Rc::new(DataCustomSsl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCustomSslData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                custom_certificate_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCustomSslRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCustomSslRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCustomSslRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `bundle_method` after provisioning.\nA ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it.\nAvailable values: \"ubiquitous\", \"optimal\", \"force\"."]
    pub fn bundle_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_certificate_id` after provisioning.\nCustom certificate identifier tag."]
    pub fn custom_certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_certificate_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_csr_id` after provisioning.\nThe identifier for the Custom CSR that was used."]
    pub fn custom_csr_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_csr_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expires_on` after provisioning.\nWhen the certificate from the authority expires."]
    pub fn expires_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataCustomSsl_FilterRef {
        DataCustomSsl_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `geo_restrictions` after provisioning.\nSpecify the region where your private key can be held locally for optimal TLS performance. HTTPS connections to any excluded data center will still be fully encrypted, but will incur some latency while Keyless SSL is used to complete the handshake with the nearest allowed data center. Options allow distribution to only to U.S. data centers, only to E.U. data centers, or only to highest security data centers. Default distribution is to all Cloudflare datacenters, for optimal performance."]
    pub fn geo_restrictions(&self) -> DataCustomSsl_GeoRestrictionsRef {
        DataCustomSsl_GeoRestrictionsRef::new(
            self.shared().clone(),
            format!("{}.geo_restrictions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hosts` after provisioning.\n"]
    pub fn hosts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nCustom certificate identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `issuer` after provisioning.\nThe certificate authority that issued the certificate."]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `keyless_server` after provisioning.\n"]
    pub fn keyless_server(&self) -> DataCustomSsl_KeylessServerRef {
        DataCustomSsl_KeylessServerRef::new(self.shared().clone(), format!("{}.keyless_server", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the certificate was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_restrictions` after provisioning.\nThe policy restrictions returned by the API. This field is returned in responses\nwhen a policy has been set. The API accepts the \"policy\" field in requests but\nreturns this field as \"policy_restrictions\" in responses.\n\nSpecifies the region(s) where your private key can be held locally for optimal\nTLS performance. Format is a boolean expression, for example:\n\"(country: US) or (region: EU)\""]
    pub fn policy_restrictions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_restrictions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nThe order/priority in which the certificate will be used in a request. The higher priority will break ties across overlapping 'legacy_custom' certificates, but 'legacy_custom' certificates will always supercede 'sni_custom' certificates."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `signature` after provisioning.\nThe type of hash used for the certificate."]
    pub fn signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the zone's custom SSL.\nAvailable values: \"active\", \"expired\", \"deleted\", \"pending\", \"initializing\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uploaded_on` after provisioning.\nWhen the certificate was uploaded to Cloudflare."]
    pub fn uploaded_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uploaded_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCustomSsl_Filter {
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCustomSsl_Filter {
    #[doc =
        "Set the field `match_`.\nWhether to match all search requirements or at least one (any).\nAvailable values: \"any\", \"all\"."]
    pub fn set_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nStatus of the zone's custom SSL.\nAvailable values: \"active\", \"expired\", \"deleted\", \"pending\", \"initializing\"."]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCustomSsl_Filter {
    type O = terrars::BlockAssignable<DataCustomSsl_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCustomSsl_Filter {}

impl BuildDataCustomSsl_Filter {
    pub fn build(self) -> DataCustomSsl_Filter {
        DataCustomSsl_Filter {
            match_: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCustomSsl_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCustomSsl_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCustomSsl_FilterRef {
        DataCustomSsl_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCustomSsl_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nWhether to match all search requirements or at least one (any).\nAvailable values: \"any\", \"all\"."]
    pub fn match_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the zone's custom SSL.\nAvailable values: \"active\", \"expired\", \"deleted\", \"pending\", \"initializing\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCustomSsl_GeoRestrictions {}

impl DataCustomSsl_GeoRestrictions { }

impl terrars::ToListMappable for DataCustomSsl_GeoRestrictions {
    type O = terrars::BlockAssignable<DataCustomSsl_GeoRestrictions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCustomSsl_GeoRestrictions {}

impl BuildDataCustomSsl_GeoRestrictions {
    pub fn build(self) -> DataCustomSsl_GeoRestrictions {
        DataCustomSsl_GeoRestrictions {}
    }
}

#[allow(dead_code)]
pub struct DataCustomSsl_GeoRestrictionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCustomSsl_GeoRestrictionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCustomSsl_GeoRestrictionsRef {
        DataCustomSsl_GeoRestrictionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCustomSsl_GeoRestrictionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `label` after provisioning.\nAvailable values: \"us\", \"eu\", \"highest_security\"."]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCustomSsl_KeylessServer_Tunnel {}

impl DataCustomSsl_KeylessServer_Tunnel { }

impl terrars::ToListMappable for DataCustomSsl_KeylessServer_Tunnel {
    type O = terrars::BlockAssignable<DataCustomSsl_KeylessServer_Tunnel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCustomSsl_KeylessServer_Tunnel {}

impl BuildDataCustomSsl_KeylessServer_Tunnel {
    pub fn build(self) -> DataCustomSsl_KeylessServer_Tunnel {
        DataCustomSsl_KeylessServer_Tunnel {}
    }
}

#[allow(dead_code)]
pub struct DataCustomSsl_KeylessServer_TunnelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCustomSsl_KeylessServer_TunnelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCustomSsl_KeylessServer_TunnelRef {
        DataCustomSsl_KeylessServer_TunnelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCustomSsl_KeylessServer_TunnelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `private_ip` after provisioning.\nPrivate IP of the Key Server Host."]
    pub fn private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vnet_id` after provisioning.\nCloudflare Tunnel Virtual Network ID."]
    pub fn vnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCustomSsl_KeylessServer {}

impl DataCustomSsl_KeylessServer { }

impl terrars::ToListMappable for DataCustomSsl_KeylessServer {
    type O = terrars::BlockAssignable<DataCustomSsl_KeylessServer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCustomSsl_KeylessServer {}

impl BuildDataCustomSsl_KeylessServer {
    pub fn build(self) -> DataCustomSsl_KeylessServer {
        DataCustomSsl_KeylessServer {}
    }
}

#[allow(dead_code)]
pub struct DataCustomSsl_KeylessServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCustomSsl_KeylessServerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCustomSsl_KeylessServerRef {
        DataCustomSsl_KeylessServerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCustomSsl_KeylessServerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the Keyless SSL was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether or not the Keyless SSL is on or off."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe keyless SSL name."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nKeyless certificate identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the Keyless SSL was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe keyless SSL name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permissions` after provisioning.\nAvailable permissions for the Keyless SSL for the current user requesting the item."]
    pub fn permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe keyless SSL port used to communicate between Cloudflare and the client's Keyless SSL server."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the Keyless SSL.\nAvailable values: \"active\", \"deleted\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tunnel` after provisioning.\nConfiguration for using Keyless SSL through a Cloudflare Tunnel."]
    pub fn tunnel(&self) -> DataCustomSsl_KeylessServer_TunnelRef {
        DataCustomSsl_KeylessServer_TunnelRef::new(self.shared().clone(), format!("{}.tunnel", self.base))
    }
}
