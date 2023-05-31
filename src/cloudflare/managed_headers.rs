use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ManagedHeadersData {
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
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_request_headers: Option<Vec<ManagedHeadersManagedRequestHeadersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_response_headers: Option<Vec<ManagedHeadersManagedResponseHeadersEl>>,
    dynamic: ManagedHeadersDynamic,
}

struct ManagedHeaders_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ManagedHeadersData>,
}

#[derive(Clone)]
pub struct ManagedHeaders(Rc<ManagedHeaders_>);

impl ManagedHeaders {
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

    #[doc= "Set the field `managed_request_headers`.\n"]
    pub fn set_managed_request_headers(
        self,
        v: impl Into<BlockAssignable<ManagedHeadersManagedRequestHeadersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().managed_request_headers = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.managed_request_headers = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `managed_response_headers`.\n"]
    pub fn set_managed_response_headers(
        self,
        v: impl Into<BlockAssignable<ManagedHeadersManagedResponseHeadersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().managed_response_headers = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.managed_response_headers = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl Referable for ManagedHeaders {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ManagedHeaders { }

impl ToListMappable for ManagedHeaders {
    type O = ListRef<ManagedHeadersRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ManagedHeaders_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_managed_headers".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildManagedHeaders {
    pub tf_id: String,
    #[doc= "The zone identifier to target for the resource."]
    pub zone_id: PrimField<String>,
}

impl BuildManagedHeaders {
    pub fn build(self, stack: &mut Stack) -> ManagedHeaders {
        let out = ManagedHeaders(Rc::new(ManagedHeaders_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ManagedHeadersData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                zone_id: self.zone_id,
                managed_request_headers: core::default::Default::default(),
                managed_response_headers: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ManagedHeadersRef {
    shared: StackShared,
    base: String,
}

impl Ref for ManagedHeadersRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ManagedHeadersRef {
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

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ManagedHeadersManagedRequestHeadersEl {
    enabled: PrimField<bool>,
    id: PrimField<String>,
}

impl ManagedHeadersManagedRequestHeadersEl { }

impl ToListMappable for ManagedHeadersManagedRequestHeadersEl {
    type O = BlockAssignable<ManagedHeadersManagedRequestHeadersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedHeadersManagedRequestHeadersEl {
    #[doc= "Whether the headers rule is active."]
    pub enabled: PrimField<bool>,
    #[doc= "Unique headers rule identifier."]
    pub id: PrimField<String>,
}

impl BuildManagedHeadersManagedRequestHeadersEl {
    pub fn build(self) -> ManagedHeadersManagedRequestHeadersEl {
        ManagedHeadersManagedRequestHeadersEl {
            enabled: self.enabled,
            id: self.id,
        }
    }
}

pub struct ManagedHeadersManagedRequestHeadersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ManagedHeadersManagedRequestHeadersElRef {
    fn new(shared: StackShared, base: String) -> ManagedHeadersManagedRequestHeadersElRef {
        ManagedHeadersManagedRequestHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedHeadersManagedRequestHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether the headers rule is active."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique headers rule identifier."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ManagedHeadersManagedResponseHeadersEl {
    enabled: PrimField<bool>,
    id: PrimField<String>,
}

impl ManagedHeadersManagedResponseHeadersEl { }

impl ToListMappable for ManagedHeadersManagedResponseHeadersEl {
    type O = BlockAssignable<ManagedHeadersManagedResponseHeadersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedHeadersManagedResponseHeadersEl {
    #[doc= "Whether the headers rule is active."]
    pub enabled: PrimField<bool>,
    #[doc= "Unique headers rule identifier."]
    pub id: PrimField<String>,
}

impl BuildManagedHeadersManagedResponseHeadersEl {
    pub fn build(self) -> ManagedHeadersManagedResponseHeadersEl {
        ManagedHeadersManagedResponseHeadersEl {
            enabled: self.enabled,
            id: self.id,
        }
    }
}

pub struct ManagedHeadersManagedResponseHeadersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ManagedHeadersManagedResponseHeadersElRef {
    fn new(shared: StackShared, base: String) -> ManagedHeadersManagedResponseHeadersElRef {
        ManagedHeadersManagedResponseHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedHeadersManagedResponseHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether the headers rule is active."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique headers rule identifier."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize, Default)]
struct ManagedHeadersDynamic {
    managed_request_headers: Option<DynamicBlock<ManagedHeadersManagedRequestHeadersEl>>,
    managed_response_headers: Option<DynamicBlock<ManagedHeadersManagedResponseHeadersEl>>,
}
