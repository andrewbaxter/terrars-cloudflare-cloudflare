#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct CustomSslData_ {
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
    bundle_method: std::option::Option<terrars::PrimField<std::string::String>>,
    certificate: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_csr_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo_restrictions: std::option::Option<CustomSsl_GeoRestrictions>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct CustomSsl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CustomSslData_>,
}

#[derive(Clone)]
pub struct CustomSsl(std::rc::Rc<CustomSsl_>);

impl CustomSsl {
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
        "Set the field `bundle_method`.\nA ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it.\nAvailable values: \"ubiquitous\", \"optimal\", \"force\"."]
    pub fn set_bundle_method(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().bundle_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_csr_id`.\nThe identifier for the Custom CSR that was used."]
    pub fn set_custom_csr_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().custom_csr_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deploy`.\nThe environment to deploy the certificate to.\nAvailable values: \"staging\", \"production\"."]
    pub fn set_deploy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deploy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `geo_restrictions`.\nSpecify the region where your private key can be held locally for optimal TLS performance. HTTPS connections to any excluded data center will still be fully encrypted, but will incur some latency while Keyless SSL is used to complete the handshake with the nearest allowed data center. Options allow distribution to only to U.S. data centers, only to E.U. data centers, or only to highest security data centers. Default distribution is to all Cloudflare datacenters, for optimal performance."]
    pub fn set_geo_restrictions(self, v: impl std::convert::Into<CustomSsl_GeoRestrictions>) -> Self {
        self.0.data.borrow_mut().geo_restrictions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `policy`.\nSpecify the policy that determines the region where your private key will be held locally. HTTPS connections to any excluded data center will still be fully encrypted, but will incur some latency while Keyless SSL is used to complete the handshake with the nearest allowed data center. Any combination of countries, specified by their two letter country code (https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2#Officially_assigned_code_elements) can be chosen, such as 'country: IN', as well as 'region: EU' which refers to the EU region. If there are too few data centers satisfying the policy, it will be rejected.\nNote: The API accepts this field as either \"policy\" or \"policy_restrictions\" in requests. Responses return this field as \"policy_restrictions\"."]
    pub fn set_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_key`.\nThe zone's private key. Not required if custom_csr_id is provided, in which case the private key is retrieved from the CSR record held by Cloudflare."]
    pub fn set_private_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().private_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe type 'legacy_custom' enables support for legacy clients which do not include SNI in the TLS handshake.\nAvailable values: \"legacy_custom\", \"sni_custom\"."]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `bundle_method` after provisioning.\nA ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it.\nAvailable values: \"ubiquitous\", \"optimal\", \"force\"."]
    pub fn bundle_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate` after provisioning.\nThe zone's SSL certificate or certificate and the intermediate(s)."]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_csr_id` after provisioning.\nThe identifier for the Custom CSR that was used."]
    pub fn custom_csr_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_csr_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deploy` after provisioning.\nThe environment to deploy the certificate to.\nAvailable values: \"staging\", \"production\"."]
    pub fn deploy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expires_on` after provisioning.\nWhen the certificate from the authority expires."]
    pub fn expires_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `geo_restrictions` after provisioning.\nSpecify the region where your private key can be held locally for optimal TLS performance. HTTPS connections to any excluded data center will still be fully encrypted, but will incur some latency while Keyless SSL is used to complete the handshake with the nearest allowed data center. Options allow distribution to only to U.S. data centers, only to E.U. data centers, or only to highest security data centers. Default distribution is to all Cloudflare datacenters, for optimal performance."]
    pub fn geo_restrictions(&self) -> CustomSsl_GeoRestrictionsRef {
        CustomSsl_GeoRestrictionsRef::new(self.shared().clone(), format!("{}.geo_restrictions", self.extract_ref()))
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
    pub fn keyless_server(&self) -> CustomSsl_KeylessServerRef {
        CustomSsl_KeylessServerRef::new(self.shared().clone(), format!("{}.keyless_server", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the certificate was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy` after provisioning.\nSpecify the policy that determines the region where your private key will be held locally. HTTPS connections to any excluded data center will still be fully encrypted, but will incur some latency while Keyless SSL is used to complete the handshake with the nearest allowed data center. Any combination of countries, specified by their two letter country code (https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2#Officially_assigned_code_elements) can be chosen, such as 'country: IN', as well as 'region: EU' which refers to the EU region. If there are too few data centers satisfying the policy, it will be rejected.\nNote: The API accepts this field as either \"policy\" or \"policy_restrictions\" in requests. Responses return this field as \"policy_restrictions\"."]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.extract_ref()))
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
        "Get a reference to the value of field `private_key` after provisioning.\nThe zone's private key. Not required if custom_csr_id is provided, in which case the private key is retrieved from the CSR record held by Cloudflare."]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.extract_ref()))
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
        "Get a reference to the value of field `type_` after provisioning.\nThe type 'legacy_custom' enables support for legacy clients which do not include SNI in the TLS handshake.\nAvailable values: \"legacy_custom\", \"sni_custom\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
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

impl terrars::Referable for CustomSsl {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CustomSsl { }

impl terrars::ToListMappable for CustomSsl {
    type O = terrars::ListRef<CustomSslRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CustomSsl_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_custom_ssl".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCustomSsl {
    pub tf_id: std::string::String,
    #[doc = "The zone's SSL certificate or certificate and the intermediate(s)."]
    pub certificate: terrars::PrimField<std::string::String>,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildCustomSsl {
    pub fn build(self, stack: &mut terrars::Stack) -> CustomSsl {
        let out = CustomSsl(std::rc::Rc::new(CustomSsl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CustomSslData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bundle_method: core::default::Default::default(),
                certificate: self.certificate,
                custom_csr_id: core::default::Default::default(),
                deploy: core::default::Default::default(),
                geo_restrictions: core::default::Default::default(),
                policy: core::default::Default::default(),
                private_key: core::default::Default::default(),
                type_: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CustomSslRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomSslRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CustomSslRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bundle_method` after provisioning.\nA ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it.\nAvailable values: \"ubiquitous\", \"optimal\", \"force\"."]
    pub fn bundle_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate` after provisioning.\nThe zone's SSL certificate or certificate and the intermediate(s)."]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_csr_id` after provisioning.\nThe identifier for the Custom CSR that was used."]
    pub fn custom_csr_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_csr_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deploy` after provisioning.\nThe environment to deploy the certificate to.\nAvailable values: \"staging\", \"production\"."]
    pub fn deploy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expires_on` after provisioning.\nWhen the certificate from the authority expires."]
    pub fn expires_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `geo_restrictions` after provisioning.\nSpecify the region where your private key can be held locally for optimal TLS performance. HTTPS connections to any excluded data center will still be fully encrypted, but will incur some latency while Keyless SSL is used to complete the handshake with the nearest allowed data center. Options allow distribution to only to U.S. data centers, only to E.U. data centers, or only to highest security data centers. Default distribution is to all Cloudflare datacenters, for optimal performance."]
    pub fn geo_restrictions(&self) -> CustomSsl_GeoRestrictionsRef {
        CustomSsl_GeoRestrictionsRef::new(self.shared().clone(), format!("{}.geo_restrictions", self.extract_ref()))
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
    pub fn keyless_server(&self) -> CustomSsl_KeylessServerRef {
        CustomSsl_KeylessServerRef::new(self.shared().clone(), format!("{}.keyless_server", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the certificate was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy` after provisioning.\nSpecify the policy that determines the region where your private key will be held locally. HTTPS connections to any excluded data center will still be fully encrypted, but will incur some latency while Keyless SSL is used to complete the handshake with the nearest allowed data center. Any combination of countries, specified by their two letter country code (https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2#Officially_assigned_code_elements) can be chosen, such as 'country: IN', as well as 'region: EU' which refers to the EU region. If there are too few data centers satisfying the policy, it will be rejected.\nNote: The API accepts this field as either \"policy\" or \"policy_restrictions\" in requests. Responses return this field as \"policy_restrictions\"."]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.extract_ref()))
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
        "Get a reference to the value of field `private_key` after provisioning.\nThe zone's private key. Not required if custom_csr_id is provided, in which case the private key is retrieved from the CSR record held by Cloudflare."]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.extract_ref()))
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
        "Get a reference to the value of field `type_` after provisioning.\nThe type 'legacy_custom' enables support for legacy clients which do not include SNI in the TLS handshake.\nAvailable values: \"legacy_custom\", \"sni_custom\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
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
pub struct CustomSsl_GeoRestrictions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CustomSsl_GeoRestrictions {
    #[doc = "Set the field `label`.\nAvailable values: \"us\", \"eu\", \"highest_security\"."]
    pub fn set_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomSsl_GeoRestrictions {
    type O = terrars::BlockAssignable<CustomSsl_GeoRestrictions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomSsl_GeoRestrictions {}

impl BuildCustomSsl_GeoRestrictions {
    pub fn build(self) -> CustomSsl_GeoRestrictions {
        CustomSsl_GeoRestrictions { label: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CustomSsl_GeoRestrictionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomSsl_GeoRestrictionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomSsl_GeoRestrictionsRef {
        CustomSsl_GeoRestrictionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomSsl_GeoRestrictionsRef {
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
pub struct CustomSsl_KeylessServer_Tunnel {}

impl CustomSsl_KeylessServer_Tunnel { }

impl terrars::ToListMappable for CustomSsl_KeylessServer_Tunnel {
    type O = terrars::BlockAssignable<CustomSsl_KeylessServer_Tunnel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomSsl_KeylessServer_Tunnel {}

impl BuildCustomSsl_KeylessServer_Tunnel {
    pub fn build(self) -> CustomSsl_KeylessServer_Tunnel {
        CustomSsl_KeylessServer_Tunnel {}
    }
}

#[allow(dead_code)]
pub struct CustomSsl_KeylessServer_TunnelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomSsl_KeylessServer_TunnelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomSsl_KeylessServer_TunnelRef {
        CustomSsl_KeylessServer_TunnelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomSsl_KeylessServer_TunnelRef {
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
pub struct CustomSsl_KeylessServer {}

impl CustomSsl_KeylessServer { }

impl terrars::ToListMappable for CustomSsl_KeylessServer {
    type O = terrars::BlockAssignable<CustomSsl_KeylessServer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomSsl_KeylessServer {}

impl BuildCustomSsl_KeylessServer {
    pub fn build(self) -> CustomSsl_KeylessServer {
        CustomSsl_KeylessServer {}
    }
}

#[allow(dead_code)]
pub struct CustomSsl_KeylessServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomSsl_KeylessServerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomSsl_KeylessServerRef {
        CustomSsl_KeylessServerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomSsl_KeylessServerRef {
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
    pub fn tunnel(&self) -> CustomSsl_KeylessServer_TunnelRef {
        CustomSsl_KeylessServer_TunnelRef::new(self.shared().clone(), format!("{}.tunnel", self.base))
    }
}
