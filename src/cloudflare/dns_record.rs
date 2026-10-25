#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DnsRecordData_ {
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
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<DnsRecord_Data>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_shadow_metadata: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_routing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxied: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<DnsRecord_Settings>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ttl: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct DnsRecord_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DnsRecordData_>,
}

#[derive(Clone)]
pub struct DnsRecord(std::rc::Rc<DnsRecord_>);

impl DnsRecord {
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
        "Set the field `comment`.\nComments or notes about the DNS record. This field has no effect on DNS responses."]
    pub fn set_comment(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().comment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content`.\nA valid IPv4 address."]
    pub fn set_content(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data`.\nComponents of a MX record."]
    pub fn set_data(self, v: impl std::convert::Into<DnsRecord_Data>) -> Self {
        self.0.data.borrow_mut().data = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_shadow_metadata`.\nWhether to include shadow metadata in the `meta` field of each record in the response. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records)."]
    pub fn set_include_shadow_metadata(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_shadow_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `priority`.\nRequired for MX, SRV and URI records; unused by other record types. Records with lower priorities are preferred."]
    pub fn set_priority(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_routing`.\nEnables private network routing to the origin."]
    pub fn set_private_routing(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().private_routing = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxied`.\nWhether the record is receiving the performance and security benefits of Cloudflare."]
    pub fn set_proxied(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().proxied = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `settings`.\nSettings for the DNS record."]
    pub fn set_settings(self, v: impl std::convert::Into<DnsRecord_Settings>) -> Self {
        self.0.data.borrow_mut().settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\nCustom tags for the DNS record. This field has no effect on DNS responses."]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `comment` after provisioning.\nComments or notes about the DNS record. This field has no effect on DNS responses."]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `comment_modified_on` after provisioning.\nWhen the record comment was last modified. Omitted if there is no comment."]
    pub fn comment_modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment_modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\nA valid IPv4 address."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the record was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\nComponents of a MX record."]
    pub fn data(&self) -> DnsRecord_DataRef {
        DnsRecord_DataRef::new(self.shared().clone(), format!("{}.data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_shadow_metadata` after provisioning.\nWhether to include shadow metadata in the `meta` field of each record in the response. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records)."]
    pub fn include_shadow_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_shadow_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `meta` after provisioning.\nExtra Cloudflare-specific information about the record."]
    pub fn meta(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the record was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nDNS record name (or @ for the zone apex) in Punycode."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nRequired for MX, SRV and URI records; unused by other record types. Records with lower priorities are preferred."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_routing` after provisioning.\nEnables private network routing to the origin."]
    pub fn private_routing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_routing", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proxiable` after provisioning.\nWhether the record can be proxied by Cloudflare or not."]
    pub fn proxiable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxiable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proxied` after provisioning.\nWhether the record is receiving the performance and security benefits of Cloudflare."]
    pub fn proxied(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxied", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\nSettings for the DNS record."]
    pub fn settings(&self) -> DnsRecord_SettingsRef {
        DnsRecord_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nCustom tags for the DNS record. This field has no effect on DNS responses."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags_modified_on` after provisioning.\nWhen the record tags were last modified. Omitted if there are no tags."]
    pub fn tags_modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tags_modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nTime To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'. Value must be between 60 and 86400, with the minimum reduced to 30 for Enterprise zones."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nRecord type.\nAvailable values: \"A\", \"AAAA\", \"CNAME\", \"MX\", \"NS\", \"OPENPGPKEY\", \"PTR\", \"TXT\", \"CAA\", \"CERT\", \"DNSKEY\", \"DS\", \"HTTPS\", \"LOC\", \"NAPTR\", \"SMIMEA\", \"SRV\", \"SSHFP\", \"SVCB\", \"TLSA\", \"URI\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DnsRecord {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DnsRecord { }

impl terrars::ToListMappable for DnsRecord {
    type O = terrars::ListRef<DnsRecordRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DnsRecord_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_dns_record".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDnsRecord {
    pub tf_id: std::string::String,
    #[doc = "DNS record name (or @ for the zone apex) in Punycode."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'. Value must be between 60 and 86400, with the minimum reduced to 30 for Enterprise zones."]
    pub ttl: terrars::PrimField<f64>,
    #[doc =
        "Record type.\nAvailable values: \"A\", \"AAAA\", \"CNAME\", \"MX\", \"NS\", \"OPENPGPKEY\", \"PTR\", \"TXT\", \"CAA\", \"CERT\", \"DNSKEY\", \"DS\", \"HTTPS\", \"LOC\", \"NAPTR\", \"SMIMEA\", \"SRV\", \"SSHFP\", \"SVCB\", \"TLSA\", \"URI\"."]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildDnsRecord {
    pub fn build(self, stack: &mut terrars::Stack) -> DnsRecord {
        let out = DnsRecord(std::rc::Rc::new(DnsRecord_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DnsRecordData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                comment: core::default::Default::default(),
                content: core::default::Default::default(),
                data: core::default::Default::default(),
                include_shadow_metadata: core::default::Default::default(),
                name: self.name,
                priority: core::default::Default::default(),
                private_routing: core::default::Default::default(),
                proxied: core::default::Default::default(),
                settings: core::default::Default::default(),
                tags: core::default::Default::default(),
                ttl: self.ttl,
                type_: self.type_,
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DnsRecordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsRecordRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DnsRecordRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `comment` after provisioning.\nComments or notes about the DNS record. This field has no effect on DNS responses."]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `comment_modified_on` after provisioning.\nWhen the record comment was last modified. Omitted if there is no comment."]
    pub fn comment_modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment_modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\nA valid IPv4 address."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the record was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\nComponents of a MX record."]
    pub fn data(&self) -> DnsRecord_DataRef {
        DnsRecord_DataRef::new(self.shared().clone(), format!("{}.data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_shadow_metadata` after provisioning.\nWhether to include shadow metadata in the `meta` field of each record in the response. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records)."]
    pub fn include_shadow_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_shadow_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `meta` after provisioning.\nExtra Cloudflare-specific information about the record."]
    pub fn meta(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the record was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nDNS record name (or @ for the zone apex) in Punycode."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nRequired for MX, SRV and URI records; unused by other record types. Records with lower priorities are preferred."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_routing` after provisioning.\nEnables private network routing to the origin."]
    pub fn private_routing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_routing", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proxiable` after provisioning.\nWhether the record can be proxied by Cloudflare or not."]
    pub fn proxiable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxiable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proxied` after provisioning.\nWhether the record is receiving the performance and security benefits of Cloudflare."]
    pub fn proxied(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxied", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\nSettings for the DNS record."]
    pub fn settings(&self) -> DnsRecord_SettingsRef {
        DnsRecord_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nCustom tags for the DNS record. This field has no effect on DNS responses."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags_modified_on` after provisioning.\nWhen the record tags were last modified. Omitted if there are no tags."]
    pub fn tags_modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tags_modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nTime To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'. Value must be between 60 and 86400, with the minimum reduced to 30 for Enterprise zones."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nRecord type.\nAvailable values: \"A\", \"AAAA\", \"CNAME\", \"MX\", \"NS\", \"OPENPGPKEY\", \"PTR\", \"TXT\", \"CAA\", \"CERT\", \"DNSKEY\", \"DS\", \"HTTPS\", \"LOC\", \"NAPTR\", \"SMIMEA\", \"SRV\", \"SSHFP\", \"SVCB\", \"TLSA\", \"URI\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DnsRecord_Data {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    altitude: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digest_type: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fingerprint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flags: std::option::Option<terrars::PrimField<serde_json::Value>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_tag: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lat_degrees: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lat_direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lat_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lat_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    long_degrees: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    long_direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    long_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    long_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    matching_type: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    precision_horz: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    precision_vert: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preference: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replacement: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selector: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl DnsRecord_Data {
    #[doc = "Set the field `algorithm`.\nAlgorithm."]
    pub fn set_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `altitude`.\nAltitude of location in meters."]
    pub fn set_altitude(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.altitude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate`.\nCertificate."]
    pub fn set_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `digest`.\nDigest."]
    pub fn set_digest(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.digest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `digest_type`.\nDigest Type."]
    pub fn set_digest_type(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.digest_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fingerprint`.\nFingerprint."]
    pub fn set_fingerprint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fingerprint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flags`.\nFlags for the CAA record."]
    pub fn set_flags(mut self, v: impl std::convert::Into<terrars::PrimField<serde_json::Value>>) -> Self {
        self.flags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_tag`.\nKey Tag."]
    pub fn set_key_tag(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.key_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lat_degrees`.\nDegrees of latitude."]
    pub fn set_lat_degrees(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lat_degrees = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lat_direction`.\nLatitude direction.\nAvailable values: \"N\", \"S\"."]
    pub fn set_lat_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lat_direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lat_minutes`.\nMinutes of latitude."]
    pub fn set_lat_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lat_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lat_seconds`.\nSeconds of latitude."]
    pub fn set_lat_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lat_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `long_degrees`.\nDegrees of longitude."]
    pub fn set_long_degrees(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.long_degrees = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `long_direction`.\nLongitude direction.\nAvailable values: \"E\", \"W\"."]
    pub fn set_long_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.long_direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `long_minutes`.\nMinutes of longitude."]
    pub fn set_long_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.long_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `long_seconds`.\nSeconds of longitude."]
    pub fn set_long_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.long_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `matching_type`.\nMatching Type."]
    pub fn set_matching_type(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.matching_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `order`.\nOrder."]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nThe port of the service."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `precision_horz`.\nHorizontal precision of location."]
    pub fn set_precision_horz(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.precision_horz = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `precision_vert`.\nVertical precision of location."]
    pub fn set_precision_vert(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.precision_vert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preference`.\nPreference."]
    pub fn set_preference(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.preference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\nPriority."]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\nProtocol."]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_key`.\nPublic Key."]
    pub fn set_public_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regex`.\nRegex."]
    pub fn set_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replacement`.\nReplacement."]
    pub fn set_replacement(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.replacement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `selector`.\nSelector."]
    pub fn set_selector(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.selector = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\nService."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size`.\nSize of location in meters."]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\nName of the property controlled by this record (e.g.: issue, issuewild, iodef)."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\nTarget."]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nType."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `usage`.\nUsage."]
    pub fn set_usage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.usage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nValue of the record. This field's semantics depend on the chosen tag."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\nThe record weight."]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DnsRecord_Data {
    type O = terrars::BlockAssignable<DnsRecord_Data>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsRecord_Data {}

impl BuildDnsRecord_Data {
    pub fn build(self) -> DnsRecord_Data {
        DnsRecord_Data {
            algorithm: core::default::Default::default(),
            altitude: core::default::Default::default(),
            certificate: core::default::Default::default(),
            digest: core::default::Default::default(),
            digest_type: core::default::Default::default(),
            fingerprint: core::default::Default::default(),
            flags: core::default::Default::default(),
            key_tag: core::default::Default::default(),
            lat_degrees: core::default::Default::default(),
            lat_direction: core::default::Default::default(),
            lat_minutes: core::default::Default::default(),
            lat_seconds: core::default::Default::default(),
            long_degrees: core::default::Default::default(),
            long_direction: core::default::Default::default(),
            long_minutes: core::default::Default::default(),
            long_seconds: core::default::Default::default(),
            matching_type: core::default::Default::default(),
            order: core::default::Default::default(),
            port: core::default::Default::default(),
            precision_horz: core::default::Default::default(),
            precision_vert: core::default::Default::default(),
            preference: core::default::Default::default(),
            priority: core::default::Default::default(),
            protocol: core::default::Default::default(),
            public_key: core::default::Default::default(),
            regex: core::default::Default::default(),
            replacement: core::default::Default::default(),
            selector: core::default::Default::default(),
            service: core::default::Default::default(),
            size: core::default::Default::default(),
            tag: core::default::Default::default(),
            target: core::default::Default::default(),
            type_: core::default::Default::default(),
            usage: core::default::Default::default(),
            value: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DnsRecord_DataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsRecord_DataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsRecord_DataRef {
        DnsRecord_DataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsRecord_DataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\nAlgorithm."]
    pub fn algorithm(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `altitude` after provisioning.\nAltitude of location in meters."]
    pub fn altitude(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.altitude", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\nCertificate."]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `digest` after provisioning.\nDigest."]
    pub fn digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest", self.base))
    }

    #[doc = "Get a reference to the value of field `digest_type` after provisioning.\nDigest Type."]
    pub fn digest_type(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest_type", self.base))
    }

    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.base))
    }

    #[doc = "Get a reference to the value of field `flags` after provisioning.\nFlags for the CAA record."]
    pub fn flags(&self) -> terrars::PrimExpr<serde_json::Value> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flags", self.base))
    }

    #[doc = "Get a reference to the value of field `key_tag` after provisioning.\nKey Tag."]
    pub fn key_tag(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_tag", self.base))
    }

    #[doc = "Get a reference to the value of field `lat_degrees` after provisioning.\nDegrees of latitude."]
    pub fn lat_degrees(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lat_degrees", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lat_direction` after provisioning.\nLatitude direction.\nAvailable values: \"N\", \"S\"."]
    pub fn lat_direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lat_direction", self.base))
    }

    #[doc = "Get a reference to the value of field `lat_minutes` after provisioning.\nMinutes of latitude."]
    pub fn lat_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lat_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `lat_seconds` after provisioning.\nSeconds of latitude."]
    pub fn lat_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lat_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `long_degrees` after provisioning.\nDegrees of longitude."]
    pub fn long_degrees(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.long_degrees", self.base))
    }

    #[doc =
        "Get a reference to the value of field `long_direction` after provisioning.\nLongitude direction.\nAvailable values: \"E\", \"W\"."]
    pub fn long_direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.long_direction", self.base))
    }

    #[doc = "Get a reference to the value of field `long_minutes` after provisioning.\nMinutes of longitude."]
    pub fn long_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.long_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `long_seconds` after provisioning.\nSeconds of longitude."]
    pub fn long_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.long_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `matching_type` after provisioning.\nMatching Type."]
    pub fn matching_type(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.matching_type", self.base))
    }

    #[doc = "Get a reference to the value of field `order` after provisioning.\nOrder."]
    pub fn order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port of the service."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `precision_horz` after provisioning.\nHorizontal precision of location."]
    pub fn precision_horz(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precision_horz", self.base))
    }

    #[doc =
        "Get a reference to the value of field `precision_vert` after provisioning.\nVertical precision of location."]
    pub fn precision_vert(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precision_vert", self.base))
    }

    #[doc = "Get a reference to the value of field `preference` after provisioning.\nPreference."]
    pub fn preference(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preference", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\nPriority."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nProtocol."]
    pub fn protocol(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `public_key` after provisioning.\nPublic Key."]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\nRegex."]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `replacement` after provisioning.\nReplacement."]
    pub fn replacement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replacement", self.base))
    }

    #[doc = "Get a reference to the value of field `selector` after provisioning.\nSelector."]
    pub fn selector(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.selector", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\nService."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\nSize of location in meters."]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nName of the property controlled by this record (e.g.: issue, issuewild, iodef)."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\nTarget."]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType."]
    pub fn type_(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `usage` after provisioning.\nUsage."]
    pub fn usage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nValue of the record. This field's semantics depend on the chosen tag."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\nThe record weight."]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsRecord_Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flatten_cname: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_only: std::option::Option<terrars::PrimField<bool>>,
}

impl DnsRecord_Settings {
    #[doc =
        "Set the field `flatten_cname`.\nIf enabled, causes the CNAME record to be resolved externally and the resulting address records (e.g., A and AAAA) to be returned instead of the CNAME record itself. This setting is unavailable for proxied records, since they are always flattened."]
    pub fn set_flatten_cname(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.flatten_cname = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipv4_only`.\nWhen enabled, only A records will be generated, and AAAA records will not be created. This setting is intended for exceptional cases. Note that this option only applies to proxied records and it has no effect on whether Cloudflare communicates with the origin using IPv4 or IPv6."]
    pub fn set_ipv4_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ipv4_only = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipv6_only`.\nWhen enabled, only AAAA records will be generated, and A records will not be created. This setting is intended for exceptional cases. Note that this option only applies to proxied records and it has no effect on whether Cloudflare communicates with the origin using IPv4 or IPv6."]
    pub fn set_ipv6_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ipv6_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DnsRecord_Settings {
    type O = terrars::BlockAssignable<DnsRecord_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsRecord_Settings {}

impl BuildDnsRecord_Settings {
    pub fn build(self) -> DnsRecord_Settings {
        DnsRecord_Settings {
            flatten_cname: core::default::Default::default(),
            ipv4_only: core::default::Default::default(),
            ipv6_only: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DnsRecord_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsRecord_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsRecord_SettingsRef {
        DnsRecord_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsRecord_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `flatten_cname` after provisioning.\nIf enabled, causes the CNAME record to be resolved externally and the resulting address records (e.g., A and AAAA) to be returned instead of the CNAME record itself. This setting is unavailable for proxied records, since they are always flattened."]
    pub fn flatten_cname(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flatten_cname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_only` after provisioning.\nWhen enabled, only A records will be generated, and AAAA records will not be created. This setting is intended for exceptional cases. Note that this option only applies to proxied records and it has no effect on whether Cloudflare communicates with the origin using IPv4 or IPv6."]
    pub fn ipv4_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_only` after provisioning.\nWhen enabled, only AAAA records will be generated, and A records will not be created. This setting is intended for exceptional cases. Note that this option only applies to proxied records and it has no effect on whether Cloudflare communicates with the origin using IPv4 or IPv6."]
    pub fn ipv6_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_only", self.base))
    }
}
