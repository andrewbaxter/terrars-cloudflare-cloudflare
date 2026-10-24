use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ListItemData {
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
    account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asn: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<terrars::PrimField<std::string::String>>,
    list_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<std::vec::Vec<ListItemHostnameEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<std::vec::Vec<ListItemRedirectEl>>,
    dynamic: ListItemDynamic,
}

struct ListItem_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ListItemData>,
}

#[derive(Clone)]
pub struct ListItem(std::rc::Rc<ListItem_>);

impl ListItem {
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
        "Set the field `asn`.\nAutonomous system number to include in the list. Must provide only one of `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_asn(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().asn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `comment`.\nAn optional comment for the item."]
    pub fn set_comment(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().comment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip`.\nIP address to include in the list. Must provide only one of `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_ip(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(self, v: impl std::convert::Into<terrars::BlockAssignable<ListItemHostnameEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hostname = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hostname = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(self, v: impl std::convert::Into<terrars::BlockAssignable<ListItemRedirectEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redirect = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redirect = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `asn` after provisioning.\nAutonomous system number to include in the list. Must provide only one of `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\nAn optional comment for the item."]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nIP address to include in the list. Must provide only one of `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_id` after provisioning.\nThe list identifier to target for the resource."]
    pub fn list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.list_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::ListRef<ListItemHostnameElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<ListItemRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.extract_ref()))
    }
}

impl terrars::Referable for ListItem {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ListItem { }

impl terrars::ToListMappable for ListItem {
    type O = terrars::ListRef<ListItemRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ListItem_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_list_item".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildListItem {
    pub tf_id: std::string::String,
    #[doc = "The account identifier to target for the resource."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "The list identifier to target for the resource."]
    pub list_id: terrars::PrimField<std::string::String>,
}

impl BuildListItem {
    pub fn build(self, stack: &mut terrars::Stack) -> ListItem {
        let out = ListItem(std::rc::Rc::new(ListItem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ListItemData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                asn: core::default::Default::default(),
                comment: core::default::Default::default(),
                id: core::default::Default::default(),
                ip: core::default::Default::default(),
                list_id: self.list_id,
                hostname: core::default::Default::default(),
                redirect: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ListItemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ListItemRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ListItemRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `asn` after provisioning.\nAutonomous system number to include in the list. Must provide only one of `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\nAn optional comment for the item."]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nIP address to include in the list. Must provide only one of `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_id` after provisioning.\nThe list identifier to target for the resource."]
    pub fn list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.list_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::ListRef<ListItemHostnameElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<ListItemRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ListItemHostnameEl {
    url_hostname: terrars::PrimField<std::string::String>,
}

impl ListItemHostnameEl { }

impl terrars::ToListMappable for ListItemHostnameEl {
    type O = terrars::BlockAssignable<ListItemHostnameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildListItemHostnameEl {
    #[doc = "The FQDN to match on."]
    pub url_hostname: terrars::PrimField<std::string::String>,
}

impl BuildListItemHostnameEl {
    pub fn build(self) -> ListItemHostnameEl {
        ListItemHostnameEl { url_hostname: self.url_hostname }
    }
}

pub struct ListItemHostnameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ListItemHostnameElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ListItemHostnameElRef {
        ListItemHostnameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ListItemHostnameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `url_hostname` after provisioning.\nThe FQDN to match on."]
    pub fn url_hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_hostname", self.base))
    }
}

#[derive(Serialize)]
pub struct ListItemRedirectEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_subdomains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_query_string: std::option::Option<terrars::PrimField<std::string::String>>,
    source_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subpath_matching: std::option::Option<terrars::PrimField<std::string::String>>,
    target_url: terrars::PrimField<std::string::String>,
}

impl ListItemRedirectEl {
    #[doc =
        "Set the field `include_subdomains`.\nWhether the redirect also matches subdomains of the source url. Available values: `disabled`, `enabled`."]
    pub fn set_include_subdomains(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.include_subdomains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preserve_path_suffix`.\nWhether to preserve the path suffix when doing subpath matching. Available values: `disabled`, `enabled`."]
    pub fn set_preserve_path_suffix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.preserve_path_suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preserve_query_string`.\nWhether the redirect target url should keep the query string of the request's url. Available values: `disabled`, `enabled`."]
    pub fn set_preserve_query_string(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.preserve_query_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\nThe status code to be used when redirecting a request."]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subpath_matching`.\nWhether the redirect also matches subpaths of the source url. Available values: `disabled`, `enabled`."]
    pub fn set_subpath_matching(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subpath_matching = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ListItemRedirectEl {
    type O = terrars::BlockAssignable<ListItemRedirectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildListItemRedirectEl {
    #[doc = "The source url of the redirect."]
    pub source_url: terrars::PrimField<std::string::String>,
    #[doc = "The target url of the redirect."]
    pub target_url: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ListItemRedirectElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ListItemRedirectElRef {
        ListItemRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ListItemRedirectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `include_subdomains` after provisioning.\nWhether the redirect also matches subdomains of the source url. Available values: `disabled`, `enabled`."]
    pub fn include_subdomains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_subdomains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preserve_path_suffix` after provisioning.\nWhether to preserve the path suffix when doing subpath matching. Available values: `disabled`, `enabled`."]
    pub fn preserve_path_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_suffix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preserve_query_string` after provisioning.\nWhether the redirect target url should keep the query string of the request's url. Available values: `disabled`, `enabled`."]
    pub fn preserve_query_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `source_url` after provisioning.\nThe source url of the redirect."]
    pub fn source_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status_code` after provisioning.\nThe status code to be used when redirecting a request."]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subpath_matching` after provisioning.\nWhether the redirect also matches subpaths of the source url. Available values: `disabled`, `enabled`."]
    pub fn subpath_matching(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subpath_matching", self.base))
    }

    #[doc = "Get a reference to the value of field `target_url` after provisioning.\nThe target url of the redirect."]
    pub fn target_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct ListItemDynamic {
    hostname: std::option::Option<terrars::DynamicBlock<ListItemHostnameEl>>,
    redirect: std::option::Option<terrars::DynamicBlock<ListItemRedirectEl>>,
}
