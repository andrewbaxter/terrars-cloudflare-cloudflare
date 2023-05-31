use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct TeamsLocationData {
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
    client_default: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    networks: Option<Vec<TeamsLocationNetworksEl>>,
    dynamic: TeamsLocationDynamic,
}

struct TeamsLocation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TeamsLocationData>,
}

#[derive(Clone)]
pub struct TeamsLocation(Rc<TeamsLocation_>);

impl TeamsLocation {
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

    #[doc= "Set the field `client_default`.\nIndicator that this is the default location."]
    pub fn set_client_default(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().client_default = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `networks`.\n"]
    pub fn set_networks(self, v: impl Into<BlockAssignable<TeamsLocationNetworksEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().networks = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.networks = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `anonymized_logs_enabled` after provisioning.\nIndicator that anonymized logs are enabled."]
    pub fn anonymized_logs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.anonymized_logs_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `client_default` after provisioning.\nIndicator that this is the default location."]
    pub fn client_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_default", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `doh_subdomain` after provisioning.\nThe FQDN that DoH clients should be pointed at."]
    pub fn doh_subdomain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.doh_subdomain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\nClient IP address."]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv4_destination` after provisioning.\nIP to direct all IPv4 DNS queries to."]
    pub fn ipv4_destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_destination", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the teams location."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `policy_ids` after provisioning.\n"]
    pub fn policy_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.policy_ids", self.extract_ref()))
    }
}

impl Referable for TeamsLocation {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for TeamsLocation { }

impl ToListMappable for TeamsLocation {
    type O = ListRef<TeamsLocationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TeamsLocation_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_teams_location".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTeamsLocation {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
    #[doc= "Name of the teams location."]
    pub name: PrimField<String>,
}

impl BuildTeamsLocation {
    pub fn build(self, stack: &mut Stack) -> TeamsLocation {
        let out = TeamsLocation(Rc::new(TeamsLocation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TeamsLocationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                client_default: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                networks: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TeamsLocationRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsLocationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TeamsLocationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `anonymized_logs_enabled` after provisioning.\nIndicator that anonymized logs are enabled."]
    pub fn anonymized_logs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.anonymized_logs_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `client_default` after provisioning.\nIndicator that this is the default location."]
    pub fn client_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_default", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `doh_subdomain` after provisioning.\nThe FQDN that DoH clients should be pointed at."]
    pub fn doh_subdomain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.doh_subdomain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\nClient IP address."]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv4_destination` after provisioning.\nIP to direct all IPv4 DNS queries to."]
    pub fn ipv4_destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_destination", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the teams location."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `policy_ids` after provisioning.\n"]
    pub fn policy_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.policy_ids", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TeamsLocationNetworksEl {
    network: PrimField<String>,
}

impl TeamsLocationNetworksEl { }

impl ToListMappable for TeamsLocationNetworksEl {
    type O = BlockAssignable<TeamsLocationNetworksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsLocationNetworksEl {
    #[doc= "CIDR notation representation of the network IP."]
    pub network: PrimField<String>,
}

impl BuildTeamsLocationNetworksEl {
    pub fn build(self) -> TeamsLocationNetworksEl {
        TeamsLocationNetworksEl { network: self.network }
    }
}

pub struct TeamsLocationNetworksElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsLocationNetworksElRef {
    fn new(shared: StackShared, base: String) -> TeamsLocationNetworksElRef {
        TeamsLocationNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsLocationNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nCIDR notation representation of the network IP."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize, Default)]
struct TeamsLocationDynamic {
    networks: Option<DynamicBlock<TeamsLocationNetworksEl>>,
}
