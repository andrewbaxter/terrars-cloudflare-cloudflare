use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct FallbackDomainData {
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
    policy_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domains: Option<Vec<FallbackDomainDomainsEl>>,
    dynamic: FallbackDomainDynamic,
}

struct FallbackDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FallbackDomainData>,
}

#[derive(Clone)]
pub struct FallbackDomain(Rc<FallbackDomain_>);

impl FallbackDomain {
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

    #[doc= "Set the field `policy_id`.\nThe settings policy for which to configure this fallback domain policy."]
    pub fn set_policy_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy_id = Some(v.into());
        self
    }

    #[doc= "Set the field `domains`.\n"]
    pub fn set_domains(self, v: impl Into<BlockAssignable<FallbackDomainDomainsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().domains = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.domains = Some(d);
            },
        }
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

    #[doc= "Get a reference to the value of field `policy_id` after provisioning.\nThe settings policy for which to configure this fallback domain policy."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }
}

impl Referable for FallbackDomain {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for FallbackDomain { }

impl ToListMappable for FallbackDomain {
    type O = ListRef<FallbackDomainRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FallbackDomain_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_fallback_domain".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFallbackDomain {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
}

impl BuildFallbackDomain {
    pub fn build(self, stack: &mut Stack) -> FallbackDomain {
        let out = FallbackDomain(Rc::new(FallbackDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FallbackDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                id: core::default::Default::default(),
                policy_id: core::default::Default::default(),
                domains: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FallbackDomainRef {
    shared: StackShared,
    base: String,
}

impl Ref for FallbackDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FallbackDomainRef {
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

    #[doc= "Get a reference to the value of field `policy_id` after provisioning.\nThe settings policy for which to configure this fallback domain policy."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FallbackDomainDomainsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_server: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}

impl FallbackDomainDomainsEl {
    #[doc= "Set the field `description`.\nA description of the fallback domain, displayed in the client UI."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `dns_server`.\nA list of IP addresses to handle domain resolution."]
    pub fn set_dns_server(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dns_server = Some(v.into());
        self
    }

    #[doc= "Set the field `suffix`.\nThe domain suffix to match when resolving locally."]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}

impl ToListMappable for FallbackDomainDomainsEl {
    type O = BlockAssignable<FallbackDomainDomainsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFallbackDomainDomainsEl {}

impl BuildFallbackDomainDomainsEl {
    pub fn build(self) -> FallbackDomainDomainsEl {
        FallbackDomainDomainsEl {
            description: core::default::Default::default(),
            dns_server: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

pub struct FallbackDomainDomainsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FallbackDomainDomainsElRef {
    fn new(shared: StackShared, base: String) -> FallbackDomainDomainsElRef {
        FallbackDomainDomainsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FallbackDomainDomainsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description of the fallback domain, displayed in the client UI."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `dns_server` after provisioning.\nA list of IP addresses to handle domain resolution."]
    pub fn dns_server(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dns_server", self.base))
    }

    #[doc= "Get a reference to the value of field `suffix` after provisioning.\nThe domain suffix to match when resolving locally."]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize, Default)]
struct FallbackDomainDynamic {
    domains: Option<DynamicBlock<FallbackDomainDomainsEl>>,
}
