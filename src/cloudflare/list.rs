use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ListData {
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
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    kind: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    item: Option<Vec<ListItemEl>>,
    dynamic: ListDynamic,
}

struct List_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ListData>,
}

#[derive(Clone)]
pub struct List(Rc<List_>);

impl List {
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

    #[doc= "Set the field `description`.\nAn optional description of the list."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `item`.\n"]
    pub fn set_item(self, v: impl Into<BlockAssignable<ListItemEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().item = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.item = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of the list."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kind` after provisioning.\nThe type of items the list will contain. Available values: `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the list. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

impl Referable for List {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for List { }

impl ToListMappable for List {
    type O = ListRef<ListRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for List_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_list".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildList {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
    #[doc= "The type of items the list will contain. Available values: `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**"]
    pub kind: PrimField<String>,
    #[doc= "The name of the list. **Modifying this attribute will force creation of a new resource.**"]
    pub name: PrimField<String>,
}

impl BuildList {
    pub fn build(self, stack: &mut Stack) -> List {
        let out = List(Rc::new(List_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ListData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                kind: self.kind,
                name: self.name,
                item: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ListRef {
    shared: StackShared,
    base: String,
}

impl Ref for ListRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ListRef {
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

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of the list."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kind` after provisioning.\nThe type of items the list will contain. Available values: `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the list. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ListItemElValueElHostnameEl {
    url_hostname: PrimField<String>,
}

impl ListItemElValueElHostnameEl { }

impl ToListMappable for ListItemElValueElHostnameEl {
    type O = BlockAssignable<ListItemElValueElHostnameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildListItemElValueElHostnameEl {
    #[doc= "The FQDN to match on. Wildcard sub-domain matching is allowed. Eg. *.abc.com."]
    pub url_hostname: PrimField<String>,
}

impl BuildListItemElValueElHostnameEl {
    pub fn build(self) -> ListItemElValueElHostnameEl {
        ListItemElValueElHostnameEl { url_hostname: self.url_hostname }
    }
}

pub struct ListItemElValueElHostnameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ListItemElValueElHostnameElRef {
    fn new(shared: StackShared, base: String) -> ListItemElValueElHostnameElRef {
        ListItemElValueElHostnameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ListItemElValueElHostnameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `url_hostname` after provisioning.\nThe FQDN to match on. Wildcard sub-domain matching is allowed. Eg. *.abc.com."]
    pub fn url_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url_hostname", self.base))
    }
}

#[derive(Serialize)]
pub struct ListItemElValueElRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_subdomains: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_path_suffix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_query_string: Option<PrimField<String>>,
    source_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subpath_matching: Option<PrimField<String>>,
    target_url: PrimField<String>,
}

impl ListItemElValueElRedirectEl {
    #[doc= "Set the field `include_subdomains`.\nWhether the redirect also matches subdomains of the source url. Available values: `disabled`, `enabled`."]
    pub fn set_include_subdomains(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.include_subdomains = Some(v.into());
        self
    }

    #[doc= "Set the field `preserve_path_suffix`.\nWhether to preserve the path suffix when doing subpath matching. Available values: `disabled`, `enabled`."]
    pub fn set_preserve_path_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preserve_path_suffix = Some(v.into());
        self
    }

    #[doc= "Set the field `preserve_query_string`.\nWhether the redirect target url should keep the query string of the request's url. Available values: `disabled`, `enabled`."]
    pub fn set_preserve_query_string(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preserve_query_string = Some(v.into());
        self
    }

    #[doc= "Set the field `status_code`.\nThe status code to be used when redirecting a request."]
    pub fn set_status_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.status_code = Some(v.into());
        self
    }

    #[doc= "Set the field `subpath_matching`.\nWhether the redirect also matches subpaths of the source url. Available values: `disabled`, `enabled`."]
    pub fn set_subpath_matching(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subpath_matching = Some(v.into());
        self
    }
}

impl ToListMappable for ListItemElValueElRedirectEl {
    type O = BlockAssignable<ListItemElValueElRedirectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildListItemElValueElRedirectEl {
    #[doc= "The source url of the redirect."]
    pub source_url: PrimField<String>,
    #[doc= "The target url of the redirect."]
    pub target_url: PrimField<String>,
}

impl BuildListItemElValueElRedirectEl {
    pub fn build(self) -> ListItemElValueElRedirectEl {
        ListItemElValueElRedirectEl {
            include_subdomains: core::default::Default::default(),
            preserve_path_suffix: core::default::Default::default(),
            preserve_query_string: core::default::Default::default(),
            source_url: self.source_url,
            status_code: core::default::Default::default(),
            subpath_matching: core::default::Default::default(),
            target_url: self.target_url,
        }
    }
}

pub struct ListItemElValueElRedirectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ListItemElValueElRedirectElRef {
    fn new(shared: StackShared, base: String) -> ListItemElValueElRedirectElRef {
        ListItemElValueElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ListItemElValueElRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `include_subdomains` after provisioning.\nWhether the redirect also matches subdomains of the source url. Available values: `disabled`, `enabled`."]
    pub fn include_subdomains(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_subdomains", self.base))
    }

    #[doc= "Get a reference to the value of field `preserve_path_suffix` after provisioning.\nWhether to preserve the path suffix when doing subpath matching. Available values: `disabled`, `enabled`."]
    pub fn preserve_path_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_suffix", self.base))
    }

    #[doc= "Get a reference to the value of field `preserve_query_string` after provisioning.\nWhether the redirect target url should keep the query string of the request's url. Available values: `disabled`, `enabled`."]
    pub fn preserve_query_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_query_string", self.base))
    }

    #[doc= "Get a reference to the value of field `source_url` after provisioning.\nThe source url of the redirect."]
    pub fn source_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_url", self.base))
    }

    #[doc= "Get a reference to the value of field `status_code` after provisioning.\nThe status code to be used when redirecting a request."]
    pub fn status_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc= "Get a reference to the value of field `subpath_matching` after provisioning.\nWhether the redirect also matches subpaths of the source url. Available values: `disabled`, `enabled`."]
    pub fn subpath_matching(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subpath_matching", self.base))
    }

    #[doc= "Get a reference to the value of field `target_url` after provisioning.\nThe target url of the redirect."]
    pub fn target_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct ListItemElValueElDynamic {
    hostname: Option<DynamicBlock<ListItemElValueElHostnameEl>>,
    redirect: Option<DynamicBlock<ListItemElValueElRedirectEl>>,
}

#[derive(Serialize)]
pub struct ListItemElValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    asn: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<Vec<ListItemElValueElHostnameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect: Option<Vec<ListItemElValueElRedirectEl>>,
    dynamic: ListItemElValueElDynamic,
}

impl ListItemElValueEl {
    #[doc= "Set the field `asn`.\n"]
    pub fn set_asn(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.asn = Some(v.into());
        self
    }

    #[doc= "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip = Some(v.into());
        self
    }

    #[doc= "Set the field `hostname`.\n"]
    pub fn set_hostname(mut self, v: impl Into<BlockAssignable<ListItemElValueElHostnameEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hostname = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hostname = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `redirect`.\n"]
    pub fn set_redirect(mut self, v: impl Into<BlockAssignable<ListItemElValueElRedirectEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redirect = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redirect = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ListItemElValueEl {
    type O = BlockAssignable<ListItemElValueEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildListItemElValueEl {}

impl BuildListItemElValueEl {
    pub fn build(self) -> ListItemElValueEl {
        ListItemElValueEl {
            asn: core::default::Default::default(),
            ip: core::default::Default::default(),
            hostname: core::default::Default::default(),
            redirect: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ListItemElValueElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ListItemElValueElRef {
    fn new(shared: StackShared, base: String) -> ListItemElValueElRef {
        ListItemElValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ListItemElValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `asn` after provisioning.\n"]
    pub fn asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> ListRef<ListItemElValueElHostnameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc= "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> ListRef<ListItemElValueElRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}

#[derive(Serialize, Default)]
struct ListItemElDynamic {
    value: Option<DynamicBlock<ListItemElValueEl>>,
}

#[derive(Serialize)]
pub struct ListItemEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Vec<ListItemElValueEl>>,
    dynamic: ListItemElDynamic,
}

impl ListItemEl {
    #[doc= "Set the field `comment`.\nAn optional comment for the item."]
    pub fn set_comment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<BlockAssignable<ListItemElValueEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.value = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.value = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ListItemEl {
    type O = BlockAssignable<ListItemEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildListItemEl {}

impl BuildListItemEl {
    pub fn build(self) -> ListItemEl {
        ListItemEl {
            comment: core::default::Default::default(),
            value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ListItemElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ListItemElRef {
    fn new(shared: StackShared, base: String) -> ListItemElRef {
        ListItemElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ListItemElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `comment` after provisioning.\nAn optional comment for the item."]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> ListRef<ListItemElValueElRef> {
        ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ListDynamic {
    item: Option<DynamicBlock<ListItemEl>>,
}
