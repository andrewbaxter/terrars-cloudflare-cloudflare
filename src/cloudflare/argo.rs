use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ArgoData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    smart_routing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiered_caching: Option<PrimField<String>>,
    zone_id: PrimField<String>,
}

struct Argo_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ArgoData>,
}

#[derive(Clone)]
pub struct Argo(Rc<Argo_>);

impl Argo {
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

    #[doc= "Set the field `smart_routing`.\nWhether smart routing is enabled. Available values: `on`, `off`."]
    pub fn set_smart_routing(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().smart_routing = Some(v.into());
        self
    }

    #[doc= "Set the field `tiered_caching`.\nWhether tiered caching is enabled. Available values: `on`, `off`."]
    pub fn set_tiered_caching(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tiered_caching = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `smart_routing` after provisioning.\nWhether smart routing is enabled. Available values: `on`, `off`."]
    pub fn smart_routing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.smart_routing", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tiered_caching` after provisioning.\nWhether tiered caching is enabled. Available values: `on`, `off`."]
    pub fn tiered_caching(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tiered_caching", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl Referable for Argo {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Argo { }

impl ToListMappable for Argo {
    type O = ListRef<ArgoRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Argo_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_argo".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildArgo {
    pub tf_id: String,
    #[doc= "The zone identifier to target for the resource."]
    pub zone_id: PrimField<String>,
}

impl BuildArgo {
    pub fn build(self, stack: &mut Stack) -> Argo {
        let out = Argo(Rc::new(Argo_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ArgoData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                smart_routing: core::default::Default::default(),
                tiered_caching: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ArgoRef {
    shared: StackShared,
    base: String,
}

impl Ref for ArgoRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ArgoRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `smart_routing` after provisioning.\nWhether smart routing is enabled. Available values: `on`, `off`."]
    pub fn smart_routing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.smart_routing", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tiered_caching` after provisioning.\nWhether tiered caching is enabled. Available values: `on`, `off`."]
    pub fn tiered_caching(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tiered_caching", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}
