#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZoneDnssecData_ {
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
    dnssec_multi_signer: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dnssec_presigned: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dnssec_use_nsec3: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct ZoneDnssec_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZoneDnssecData_>,
}

#[derive(Clone)]
pub struct ZoneDnssec(std::rc::Rc<ZoneDnssec_>);

impl ZoneDnssec {
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
        "Set the field `dnssec_multi_signer`.\nIf true, multi-signer DNSSEC is enabled on the zone, allowing multiple\nproviders to serve a DNSSEC-signed zone at the same time.\nThis is required for DNSKEY records (except those automatically\ngenerated by Cloudflare) to be added to the zone.\n\nSee [Multi-signer DNSSEC](https://developers.cloudflare.com/dns/dnssec/multi-signer-dnssec/) for details."]
    pub fn set_dnssec_multi_signer(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().dnssec_multi_signer = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dnssec_presigned`.\nIf true, allows Cloudflare to transfer in a DNSSEC-signed zone\nincluding signatures from an external provider, without requiring\nCloudflare to sign any records on the fly.\n\nNote that this feature has some limitations.\nSee [Cloudflare as Secondary](https://developers.cloudflare.com/dns/zone-setups/zone-transfers/cloudflare-as-secondary/setup/#dnssec) for details."]
    pub fn set_dnssec_presigned(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().dnssec_presigned = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dnssec_use_nsec3`.\nIf true, enables the use of NSEC3 together with DNSSEC on the zone.\nCombined with setting dnssec_presigned to true, this enables the use of\nNSEC3 records when transferring in from an external provider.\nIf dnssec_presigned is instead set to false (default), NSEC3 records will be\ngenerated and signed at request time.\n\nSee [DNSSEC with NSEC3](https://developers.cloudflare.com/dns/dnssec/enable-nsec3/) for details."]
    pub fn set_dnssec_use_nsec3(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().dnssec_use_nsec3 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nStatus of DNSSEC, based on user-desired state and presence of necessary records.\nAvailable values: \"active\", \"disabled\"."]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\nAlgorithm key code."]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `digest` after provisioning.\nDigest hash."]
    pub fn digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `digest_algorithm` after provisioning.\nType of digest algorithm."]
    pub fn digest_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest_algorithm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `digest_type` after provisioning.\nCoded type for digest algorithm."]
    pub fn digest_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dnssec_multi_signer` after provisioning.\nIf true, multi-signer DNSSEC is enabled on the zone, allowing multiple\nproviders to serve a DNSSEC-signed zone at the same time.\nThis is required for DNSKEY records (except those automatically\ngenerated by Cloudflare) to be added to the zone.\n\nSee [Multi-signer DNSSEC](https://developers.cloudflare.com/dns/dnssec/multi-signer-dnssec/) for details."]
    pub fn dnssec_multi_signer(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dnssec_multi_signer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dnssec_presigned` after provisioning.\nIf true, allows Cloudflare to transfer in a DNSSEC-signed zone\nincluding signatures from an external provider, without requiring\nCloudflare to sign any records on the fly.\n\nNote that this feature has some limitations.\nSee [Cloudflare as Secondary](https://developers.cloudflare.com/dns/zone-setups/zone-transfers/cloudflare-as-secondary/setup/#dnssec) for details."]
    pub fn dnssec_presigned(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dnssec_presigned", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dnssec_use_nsec3` after provisioning.\nIf true, enables the use of NSEC3 together with DNSSEC on the zone.\nCombined with setting dnssec_presigned to true, this enables the use of\nNSEC3 records when transferring in from an external provider.\nIf dnssec_presigned is instead set to false (default), NSEC3 records will be\ngenerated and signed at request time.\n\nSee [DNSSEC with NSEC3](https://developers.cloudflare.com/dns/dnssec/enable-nsec3/) for details."]
    pub fn dnssec_use_nsec3(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dnssec_use_nsec3", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ds` after provisioning.\nFull DS record."]
    pub fn ds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `flags` after provisioning.\nFlag for DNSSEC record."]
    pub fn flags(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_tag` after provisioning.\nCode for key tag."]
    pub fn key_tag(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_type` after provisioning.\nAlgorithm key type."]
    pub fn key_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\nWhen DNSSEC was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_key` after provisioning.\nPublic key for DS record."]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of DNSSEC, based on user-desired state and presence of necessary records.\nAvailable values: \"active\", \"disabled\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for ZoneDnssec {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZoneDnssec { }

impl terrars::ToListMappable for ZoneDnssec {
    type O = terrars::ListRef<ZoneDnssecRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZoneDnssec_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zone_dnssec".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZoneDnssec {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildZoneDnssec {
    pub fn build(self, stack: &mut terrars::Stack) -> ZoneDnssec {
        let out = ZoneDnssec(std::rc::Rc::new(ZoneDnssec_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZoneDnssecData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                dnssec_multi_signer: core::default::Default::default(),
                dnssec_presigned: core::default::Default::default(),
                dnssec_use_nsec3: core::default::Default::default(),
                status: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZoneDnssecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneDnssecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZoneDnssecRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\nAlgorithm key code."]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `digest` after provisioning.\nDigest hash."]
    pub fn digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `digest_algorithm` after provisioning.\nType of digest algorithm."]
    pub fn digest_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest_algorithm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `digest_type` after provisioning.\nCoded type for digest algorithm."]
    pub fn digest_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dnssec_multi_signer` after provisioning.\nIf true, multi-signer DNSSEC is enabled on the zone, allowing multiple\nproviders to serve a DNSSEC-signed zone at the same time.\nThis is required for DNSKEY records (except those automatically\ngenerated by Cloudflare) to be added to the zone.\n\nSee [Multi-signer DNSSEC](https://developers.cloudflare.com/dns/dnssec/multi-signer-dnssec/) for details."]
    pub fn dnssec_multi_signer(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dnssec_multi_signer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dnssec_presigned` after provisioning.\nIf true, allows Cloudflare to transfer in a DNSSEC-signed zone\nincluding signatures from an external provider, without requiring\nCloudflare to sign any records on the fly.\n\nNote that this feature has some limitations.\nSee [Cloudflare as Secondary](https://developers.cloudflare.com/dns/zone-setups/zone-transfers/cloudflare-as-secondary/setup/#dnssec) for details."]
    pub fn dnssec_presigned(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dnssec_presigned", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dnssec_use_nsec3` after provisioning.\nIf true, enables the use of NSEC3 together with DNSSEC on the zone.\nCombined with setting dnssec_presigned to true, this enables the use of\nNSEC3 records when transferring in from an external provider.\nIf dnssec_presigned is instead set to false (default), NSEC3 records will be\ngenerated and signed at request time.\n\nSee [DNSSEC with NSEC3](https://developers.cloudflare.com/dns/dnssec/enable-nsec3/) for details."]
    pub fn dnssec_use_nsec3(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dnssec_use_nsec3", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ds` after provisioning.\nFull DS record."]
    pub fn ds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `flags` after provisioning.\nFlag for DNSSEC record."]
    pub fn flags(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_tag` after provisioning.\nCode for key tag."]
    pub fn key_tag(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_type` after provisioning.\nAlgorithm key type."]
    pub fn key_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\nWhen DNSSEC was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_key` after provisioning.\nPublic key for DS record."]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of DNSSEC, based on user-desired state and presence of necessary records.\nAvailable values: \"active\", \"disabled\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}
