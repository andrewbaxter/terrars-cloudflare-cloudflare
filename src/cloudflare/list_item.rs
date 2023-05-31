use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ListItemData {
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
    asn: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<PrimField<String>>,
    list_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<Vec<ListItemHostnameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect: Option<Vec<ListItemRedirectEl>>,
    dynamic: ListItemDynamic,
}

struct ListItem_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ListItemData>,
}

#[derive(Clone)]
pub struct ListItem(Rc<ListItem_>);

impl ListItem {
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

    #[doc= "Set the field `asn`.\nAutonomous system number to include in the list. Must provide only one of `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_asn(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().asn = Some(v.into());
        self
    }

    #[doc= "Set the field `comment`.\nAn optional comment for the item."]
    pub fn set_comment(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().comment = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ip`.\nIP address to include in the list. Must provide only one of `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_ip(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip = Some(v.into());
        self
    }

    #[doc= "Set the field `hostname`.\n"]
    pub fn set_hostname(self, v: impl Into<BlockAssignable<ListItemHostnameEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hostname = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hostname = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `redirect`.\n"]
    pub fn set_redirect(self, v: impl Into<BlockAssignable<ListItemRedirectEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redirect = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redirect = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `asn` after provisioning.\nAutonomous system number to include in the list. Must provide only one of `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.asn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `comment` after provisioning.\nAn optional comment for the item."]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\nIP address to include in the list. Must provide only one of `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `list_id` after provisioning.\nThe list identifier to target for the resource."]
    pub fn list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.list_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> ListRef<ListItemHostnameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> ListRef<ListItemRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redirect", self.extract_ref()))
    }
}

impl Referable for ListItem {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ListItem { }

impl ToListMappable for ListItem {
    type O = ListRef<ListItemRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ListItem_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_list_item".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildListItem {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
    #[doc= "The list identifier to target for the resource."]
    pub list_id: PrimField<String>,
}

impl BuildListItem {
    pub fn build(self, stack: &mut Stack) -> ListItem {
        let out = ListItem(Rc::new(ListItem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ListItemData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                asn: core::default::Default::default(),
                comment: core::default::Default::default(),
                id: core::default::Default::default(),
                ip: core::default::Default::default(),
                list_id: self.list_id,
                hostname: core::default::Default::default(),
                redirect: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ListItemRef {
    shared: StackShared,
    base: String,
}

impl Ref for ListItemRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ListItemRef {
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

    #[doc= "Get a reference to the value of field `asn` after provisioning.\nAutonomous system number to include in the list. Must provide only one of `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.asn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `comment` after provisioning.\nAn optional comment for the item."]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\nIP address to include in the list. Must provide only one of `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `list_id` after provisioning.\nThe list identifier to target for the resource."]
    pub fn list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.list_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> ListRef<ListItemHostnameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> ListRef<ListItemRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redirect", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ListItemHostnameEl {
    url_hostname: PrimField<String>,
}

impl ListItemHostnameEl { }

impl ToListMappable for ListItemHostnameEl {
    type O = BlockAssignable<ListItemHostnameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildListItemHostnameEl {
    #[doc= "The FQDN to match on."]
    pub url_hostname: PrimField<String>,
}

impl BuildListItemHostnameEl {
    pub fn build(self) -> ListItemHostnameEl {
        ListItemHostnameEl { url_hostname: self.url_hostname }
    }
}

pub struct ListItemHostnameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ListItemHostnameElRef {
    fn new(shared: StackShared, base: String) -> ListItemHostnameElRef {
        ListItemHostnameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ListItemHostnameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `url_hostname` after provisioning.\nThe FQDN to match on."]
    pub fn url_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url_hostname", self.base))
    }
}

#[derive(Serialize)]
pub struct ListItemRedirectEl {
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

impl ListItemRedirectEl {
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

impl ToListMappable for ListItemRedirectEl {
    type O = BlockAssignable<ListItemRedirectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildListItemRedirectEl {
    #[doc= "The source url of the redirect."]
    pub source_url: PrimField<String>,
    #[doc= "The target url of the redirect."]
    pub target_url: PrimField<String>,
}

impl BuildListItemRedirectEl {
    pub fn build(self) -> ListItemRedirectEl {
        ListItemRedirectEl {
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

pub struct ListItemRedirectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ListItemRedirectElRef {
    fn new(shared: StackShared, base: String) -> ListItemRedirectElRef {
        ListItemRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ListItemRedirectElRef {
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
struct ListItemDynamic {
    hostname: Option<DynamicBlock<ListItemHostnameEl>>,
    redirect: Option<DynamicBlock<ListItemRedirectEl>>,
}
