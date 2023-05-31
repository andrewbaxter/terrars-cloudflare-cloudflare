use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct RecordData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_overwrite: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<f64>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<RecordDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<RecordTimeoutsEl>,
    dynamic: RecordDynamic,
}

struct Record_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RecordData>,
}

#[derive(Clone)]
pub struct Record(Rc<Record_>);

impl Record {
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

    #[doc= "Set the field `allow_overwrite`.\nAllow creation of this record in Terraform to overwrite an existing record, if any. This does not affect the ability to update the record in Terraform and does not prevent other resources within Terraform or manual changes outside Terraform from overwriting this record. **This configuration is not recommended for most environments**. Defaults to `false`."]
    pub fn set_allow_overwrite(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_overwrite = Some(v.into());
        self
    }

    #[doc= "Set the field `comment`.\nComments or notes about the DNS record. This field has no effect on DNS responses."]
    pub fn set_comment(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().comment = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `priority`.\nThe priority of the record."]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
        self
    }

    #[doc= "Set the field `proxied`.\nWhether the record gets Cloudflare's origin protection."]
    pub fn set_proxied(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().proxied = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\nCustom tags for the DNS record."]
    pub fn set_tags(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `ttl`.\nThe TTL of the record."]
    pub fn set_ttl(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ttl = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\nThe value of the record. Conflicts with `data`."]
    pub fn set_value(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().value = Some(v.into());
        self
    }

    #[doc= "Set the field `data`.\n"]
    pub fn set_data(self, v: impl Into<BlockAssignable<RecordDataEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<RecordTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `allow_overwrite` after provisioning.\nAllow creation of this record in Terraform to overwrite an existing record, if any. This does not affect the ability to update the record in Terraform and does not prevent other resources within Terraform or manual changes outside Terraform from overwriting this record. **This configuration is not recommended for most environments**. Defaults to `false`."]
    pub fn allow_overwrite(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_overwrite", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `comment` after provisioning.\nComments or notes about the DNS record. This field has no effect on DNS responses."]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_on` after provisioning.\nThe RFC3339 timestamp of when the record was created."]
    pub fn created_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\nThe FQDN of the record."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metadata` after provisioning.\nA key-value map of string metadata Cloudflare associates with the record."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nThe RFC3339 timestamp of when the record was last modified."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the record. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nThe priority of the record."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proxiable` after provisioning.\nShows whether this record can be proxied."]
    pub fn proxiable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxiable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proxied` after provisioning.\nWhether the record gets Cloudflare's origin protection."]
    pub fn proxied(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxied", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\nCustom tags for the DNS record."]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\nThe TTL of the record."]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of the record. Available values: `A`, `AAAA`, `CAA`, `CNAME`, `TXT`, `SRV`, `LOC`, `MX`, `NS`, `SPF`, `CERT`, `DNSKEY`, `DS`, `NAPTR`, `SMIMEA`, `SSHFP`, `TLSA`, `URI`, `PTR`, `HTTPS`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe value of the record. Conflicts with `data`."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> ListRef<RecordDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RecordTimeoutsElRef {
        RecordTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for Record {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Record { }

impl ToListMappable for Record {
    type O = ListRef<RecordRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Record_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_record".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRecord {
    pub tf_id: String,
    #[doc= "The name of the record. **Modifying this attribute will force creation of a new resource.**"]
    pub name: PrimField<String>,
    #[doc= "The type of the record. Available values: `A`, `AAAA`, `CAA`, `CNAME`, `TXT`, `SRV`, `LOC`, `MX`, `NS`, `SPF`, `CERT`, `DNSKEY`, `DS`, `NAPTR`, `SMIMEA`, `SSHFP`, `TLSA`, `URI`, `PTR`, `HTTPS`. **Modifying this attribute will force creation of a new resource.**"]
    pub type_: PrimField<String>,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildRecord {
    pub fn build(self, stack: &mut Stack) -> Record {
        let out = Record(Rc::new(Record_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RecordData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_overwrite: core::default::Default::default(),
                comment: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                priority: core::default::Default::default(),
                proxied: core::default::Default::default(),
                tags: core::default::Default::default(),
                ttl: core::default::Default::default(),
                type_: self.type_,
                value: core::default::Default::default(),
                zone_id: self.zone_id,
                data: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RecordRef {
    shared: StackShared,
    base: String,
}

impl Ref for RecordRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RecordRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_overwrite` after provisioning.\nAllow creation of this record in Terraform to overwrite an existing record, if any. This does not affect the ability to update the record in Terraform and does not prevent other resources within Terraform or manual changes outside Terraform from overwriting this record. **This configuration is not recommended for most environments**. Defaults to `false`."]
    pub fn allow_overwrite(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_overwrite", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `comment` after provisioning.\nComments or notes about the DNS record. This field has no effect on DNS responses."]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_on` after provisioning.\nThe RFC3339 timestamp of when the record was created."]
    pub fn created_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\nThe FQDN of the record."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metadata` after provisioning.\nA key-value map of string metadata Cloudflare associates with the record."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nThe RFC3339 timestamp of when the record was last modified."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the record. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nThe priority of the record."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proxiable` after provisioning.\nShows whether this record can be proxied."]
    pub fn proxiable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxiable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proxied` after provisioning.\nWhether the record gets Cloudflare's origin protection."]
    pub fn proxied(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxied", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\nCustom tags for the DNS record."]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\nThe TTL of the record."]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of the record. Available values: `A`, `AAAA`, `CAA`, `CNAME`, `TXT`, `SRV`, `LOC`, `MX`, `NS`, `SPF`, `CERT`, `DNSKEY`, `DS`, `NAPTR`, `SMIMEA`, `SSHFP`, `TLSA`, `URI`, `PTR`, `HTTPS`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe value of the record. Conflicts with `data`."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> ListRef<RecordDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RecordTimeoutsElRef {
        RecordTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RecordDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    altitude: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    digest: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    digest_type: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fingerprint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_tag: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lat_degrees: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lat_direction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lat_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lat_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    long_degrees: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    long_direction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    long_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    long_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matching_type: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    precision_horz: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    precision_vert: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preference: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proto: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replacement: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selector: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl RecordDataEl {
    #[doc= "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.algorithm = Some(v.into());
        self
    }

    #[doc= "Set the field `altitude`.\n"]
    pub fn set_altitude(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.altitude = Some(v.into());
        self
    }

    #[doc= "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `content`.\n"]
    pub fn set_content(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content = Some(v.into());
        self
    }

    #[doc= "Set the field `digest`.\n"]
    pub fn set_digest(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.digest = Some(v.into());
        self
    }

    #[doc= "Set the field `digest_type`.\n"]
    pub fn set_digest_type(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.digest_type = Some(v.into());
        self
    }

    #[doc= "Set the field `fingerprint`.\n"]
    pub fn set_fingerprint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fingerprint = Some(v.into());
        self
    }

    #[doc= "Set the field `flags`.\n"]
    pub fn set_flags(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.flags = Some(v.into());
        self
    }

    #[doc= "Set the field `key_tag`.\n"]
    pub fn set_key_tag(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.key_tag = Some(v.into());
        self
    }

    #[doc= "Set the field `lat_degrees`.\n"]
    pub fn set_lat_degrees(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.lat_degrees = Some(v.into());
        self
    }

    #[doc= "Set the field `lat_direction`.\n"]
    pub fn set_lat_direction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lat_direction = Some(v.into());
        self
    }

    #[doc= "Set the field `lat_minutes`.\n"]
    pub fn set_lat_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.lat_minutes = Some(v.into());
        self
    }

    #[doc= "Set the field `lat_seconds`.\n"]
    pub fn set_lat_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.lat_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `long_degrees`.\n"]
    pub fn set_long_degrees(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.long_degrees = Some(v.into());
        self
    }

    #[doc= "Set the field `long_direction`.\n"]
    pub fn set_long_direction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.long_direction = Some(v.into());
        self
    }

    #[doc= "Set the field `long_minutes`.\n"]
    pub fn set_long_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.long_minutes = Some(v.into());
        self
    }

    #[doc= "Set the field `long_seconds`.\n"]
    pub fn set_long_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.long_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `matching_type`.\n"]
    pub fn set_matching_type(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.matching_type = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `order`.\n"]
    pub fn set_order(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.order = Some(v.into());
        self
    }

    #[doc= "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc= "Set the field `precision_horz`.\n"]
    pub fn set_precision_horz(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.precision_horz = Some(v.into());
        self
    }

    #[doc= "Set the field `precision_vert`.\n"]
    pub fn set_precision_vert(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.precision_vert = Some(v.into());
        self
    }

    #[doc= "Set the field `preference`.\n"]
    pub fn set_preference(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.preference = Some(v.into());
        self
    }

    #[doc= "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }

    #[doc= "Set the field `proto`.\n"]
    pub fn set_proto(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.proto = Some(v.into());
        self
    }

    #[doc= "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc= "Set the field `public_key`.\n"]
    pub fn set_public_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_key = Some(v.into());
        self
    }

    #[doc= "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex = Some(v.into());
        self
    }

    #[doc= "Set the field `replacement`.\n"]
    pub fn set_replacement(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.replacement = Some(v.into());
        self
    }

    #[doc= "Set the field `selector`.\n"]
    pub fn set_selector(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.selector = Some(v.into());
        self
    }

    #[doc= "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }

    #[doc= "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }

    #[doc= "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }

    #[doc= "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `usage`.\n"]
    pub fn set_usage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.usage = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }

    #[doc= "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for RecordDataEl {
    type O = BlockAssignable<RecordDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecordDataEl {}

impl BuildRecordDataEl {
    pub fn build(self) -> RecordDataEl {
        RecordDataEl {
            algorithm: core::default::Default::default(),
            altitude: core::default::Default::default(),
            certificate: core::default::Default::default(),
            content: core::default::Default::default(),
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
            name: core::default::Default::default(),
            order: core::default::Default::default(),
            port: core::default::Default::default(),
            precision_horz: core::default::Default::default(),
            precision_vert: core::default::Default::default(),
            preference: core::default::Default::default(),
            priority: core::default::Default::default(),
            proto: core::default::Default::default(),
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

pub struct RecordDataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RecordDataElRef {
    fn new(shared: StackShared, base: String) -> RecordDataElRef {
        RecordDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecordDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc= "Get a reference to the value of field `altitude` after provisioning.\n"]
    pub fn altitude(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.altitude", self.base))
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc= "Get a reference to the value of field `digest` after provisioning.\n"]
    pub fn digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest", self.base))
    }

    #[doc= "Get a reference to the value of field `digest_type` after provisioning.\n"]
    pub fn digest_type(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest_type", self.base))
    }

    #[doc= "Get a reference to the value of field `fingerprint` after provisioning.\n"]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.base))
    }

    #[doc= "Get a reference to the value of field `flags` after provisioning.\n"]
    pub fn flags(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flags", self.base))
    }

    #[doc= "Get a reference to the value of field `key_tag` after provisioning.\n"]
    pub fn key_tag(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_tag", self.base))
    }

    #[doc= "Get a reference to the value of field `lat_degrees` after provisioning.\n"]
    pub fn lat_degrees(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lat_degrees", self.base))
    }

    #[doc= "Get a reference to the value of field `lat_direction` after provisioning.\n"]
    pub fn lat_direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lat_direction", self.base))
    }

    #[doc= "Get a reference to the value of field `lat_minutes` after provisioning.\n"]
    pub fn lat_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lat_minutes", self.base))
    }

    #[doc= "Get a reference to the value of field `lat_seconds` after provisioning.\n"]
    pub fn lat_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lat_seconds", self.base))
    }

    #[doc= "Get a reference to the value of field `long_degrees` after provisioning.\n"]
    pub fn long_degrees(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.long_degrees", self.base))
    }

    #[doc= "Get a reference to the value of field `long_direction` after provisioning.\n"]
    pub fn long_direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.long_direction", self.base))
    }

    #[doc= "Get a reference to the value of field `long_minutes` after provisioning.\n"]
    pub fn long_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.long_minutes", self.base))
    }

    #[doc= "Get a reference to the value of field `long_seconds` after provisioning.\n"]
    pub fn long_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.long_seconds", self.base))
    }

    #[doc= "Get a reference to the value of field `matching_type` after provisioning.\n"]
    pub fn matching_type(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.matching_type", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `order` after provisioning.\n"]
    pub fn order(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc= "Get a reference to the value of field `precision_horz` after provisioning.\n"]
    pub fn precision_horz(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.precision_horz", self.base))
    }

    #[doc= "Get a reference to the value of field `precision_vert` after provisioning.\n"]
    pub fn precision_vert(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.precision_vert", self.base))
    }

    #[doc= "Get a reference to the value of field `preference` after provisioning.\n"]
    pub fn preference(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.preference", self.base))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc= "Get a reference to the value of field `proto` after provisioning.\n"]
    pub fn proto(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proto", self.base))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc= "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }

    #[doc= "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc= "Get a reference to the value of field `replacement` after provisioning.\n"]
    pub fn replacement(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.replacement", self.base))
    }

    #[doc= "Get a reference to the value of field `selector` after provisioning.\n"]
    pub fn selector(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.selector", self.base))
    }

    #[doc= "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `usage` after provisioning.\n"]
    pub fn usage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.usage", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct RecordTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl RecordTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for RecordTimeoutsEl {
    type O = BlockAssignable<RecordTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecordTimeoutsEl {}

impl BuildRecordTimeoutsEl {
    pub fn build(self) -> RecordTimeoutsEl {
        RecordTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct RecordTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RecordTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> RecordTimeoutsElRef {
        RecordTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecordTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct RecordDynamic {
    data: Option<DynamicBlock<RecordDataEl>>,
}
