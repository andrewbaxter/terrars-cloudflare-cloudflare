use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZoneData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jump_start: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paused: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plan: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    zone: PrimField<String>,
}

struct Zone_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ZoneData>,
}

#[derive(Clone)]
pub struct Zone(Rc<Zone_>);

impl Zone {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `jump_start`.\nWhether to scan for DNS records on creation. Ignored after zone is created."]
    pub fn set_jump_start(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().jump_start = Some(v.into());
        self
    }

    #[doc= "Set the field `paused`.\nWhether this zone is paused (traffic bypasses Cloudflare). Defaults to `false`."]
    pub fn set_paused(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().paused = Some(v.into());
        self
    }

    #[doc= "Set the field `plan`.\nThe name of the commercial plan to apply to the zone. Available values: `free`, `lite`, `pro`, `pro_plus`, `business`, `enterprise`, `partners_free`, `partners_pro`, `partners_business`, `partners_enterprise`."]
    pub fn set_plan(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().plan = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nA full zone implies that DNS is hosted with Cloudflare. A partial zone is typically a partner-hosted zone or a CNAME setup. Available values: `full`, `partial`. Defaults to `full`."]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nAccount ID to manage the zone resource in."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `jump_start` after provisioning.\nWhether to scan for DNS records on creation. Ignored after zone is created."]
    pub fn jump_start(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.jump_start", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `meta` after provisioning.\n"]
    pub fn meta(&self) -> RecRef<PrimExpr<bool>> {
        RecRef::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name_servers` after provisioning.\nCloudflare-assigned name servers. This is only populated for zones that use Cloudflare DNS."]
    pub fn name_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.name_servers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `paused` after provisioning.\nWhether this zone is paused (traffic bypasses Cloudflare). Defaults to `false`."]
    pub fn paused(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `plan` after provisioning.\nThe name of the commercial plan to apply to the zone. Available values: `free`, `lite`, `pro`, `pro_plus`, `business`, `enterprise`, `partners_free`, `partners_pro`, `partners_business`, `partners_enterprise`."]
    pub fn plan(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plan", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nStatus of the zone. Available values: `active`, `pending`, `initializing`, `moved`, `deleted`, `deactivated`."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nA full zone implies that DNS is hosted with Cloudflare. A partial zone is typically a partner-hosted zone or a CNAME setup. Available values: `full`, `partial`. Defaults to `full`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vanity_name_servers` after provisioning.\nList of Vanity Nameservers (if set)."]
    pub fn vanity_name_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.vanity_name_servers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `verification_key` after provisioning.\nContains the TXT record value to validate domain ownership. This is only populated for zones of type `partial`."]
    pub fn verification_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.verification_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nThe DNS zone name which will be added. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }
}

impl Referable for Zone {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Zone { }

impl ToListMappable for Zone {
    type O = ListRef<ZoneRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Zone_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_zone".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZone {
    pub tf_id: String,
    #[doc= "Account ID to manage the zone resource in."]
    pub account_id: PrimField<String>,
    #[doc= "The DNS zone name which will be added. **Modifying this attribute will force creation of a new resource.**"]
    pub zone: PrimField<String>,
}

impl BuildZone {
    pub fn build(self, stack: &mut Stack) -> Zone {
        let out = Zone(Rc::new(Zone_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ZoneData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                id: core::default::Default::default(),
                jump_start: core::default::Default::default(),
                paused: core::default::Default::default(),
                plan: core::default::Default::default(),
                type_: core::default::Default::default(),
                zone: self.zone,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZoneRef {
    shared: StackShared,
    base: String,
}

impl Ref for ZoneRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZoneRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nAccount ID to manage the zone resource in."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `jump_start` after provisioning.\nWhether to scan for DNS records on creation. Ignored after zone is created."]
    pub fn jump_start(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.jump_start", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `meta` after provisioning.\n"]
    pub fn meta(&self) -> RecRef<PrimExpr<bool>> {
        RecRef::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name_servers` after provisioning.\nCloudflare-assigned name servers. This is only populated for zones that use Cloudflare DNS."]
    pub fn name_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.name_servers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `paused` after provisioning.\nWhether this zone is paused (traffic bypasses Cloudflare). Defaults to `false`."]
    pub fn paused(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `plan` after provisioning.\nThe name of the commercial plan to apply to the zone. Available values: `free`, `lite`, `pro`, `pro_plus`, `business`, `enterprise`, `partners_free`, `partners_pro`, `partners_business`, `partners_enterprise`."]
    pub fn plan(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plan", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nStatus of the zone. Available values: `active`, `pending`, `initializing`, `moved`, `deleted`, `deactivated`."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nA full zone implies that DNS is hosted with Cloudflare. A partial zone is typically a partner-hosted zone or a CNAME setup. Available values: `full`, `partial`. Defaults to `full`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vanity_name_servers` after provisioning.\nList of Vanity Nameservers (if set)."]
    pub fn vanity_name_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.vanity_name_servers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `verification_key` after provisioning.\nContains the TXT record value to validate domain ownership. This is only populated for zones of type `partial`."]
    pub fn verification_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.verification_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nThe DNS zone name which will be added. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }
}
