use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct TeamsProxyEndpointData {
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
    ips: SetField<PrimField<String>>,
    name: PrimField<String>,
}

struct TeamsProxyEndpoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TeamsProxyEndpointData>,
}

#[derive(Clone)]
pub struct TeamsProxyEndpoint(Rc<TeamsProxyEndpoint_>);

impl TeamsProxyEndpoint {
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

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ips` after provisioning.\nThe networks CIDRs that will be allowed to initiate proxy connections."]
    pub fn ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ips", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the teams proxy endpoint."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subdomain` after provisioning.\nThe FQDN that proxy clients should be pointed at."]
    pub fn subdomain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }
}

impl Referable for TeamsProxyEndpoint {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for TeamsProxyEndpoint { }

impl ToListMappable for TeamsProxyEndpoint {
    type O = ListRef<TeamsProxyEndpointRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TeamsProxyEndpoint_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_teams_proxy_endpoint".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTeamsProxyEndpoint {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
    #[doc= "The networks CIDRs that will be allowed to initiate proxy connections."]
    pub ips: SetField<PrimField<String>>,
    #[doc= "Name of the teams proxy endpoint."]
    pub name: PrimField<String>,
}

impl BuildTeamsProxyEndpoint {
    pub fn build(self, stack: &mut Stack) -> TeamsProxyEndpoint {
        let out = TeamsProxyEndpoint(Rc::new(TeamsProxyEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TeamsProxyEndpointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                id: core::default::Default::default(),
                ips: self.ips,
                name: self.name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TeamsProxyEndpointRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsProxyEndpointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TeamsProxyEndpointRef {
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

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ips` after provisioning.\nThe networks CIDRs that will be allowed to initiate proxy connections."]
    pub fn ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ips", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the teams proxy endpoint."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subdomain` after provisioning.\nThe FQDN that proxy clients should be pointed at."]
    pub fn subdomain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }
}
