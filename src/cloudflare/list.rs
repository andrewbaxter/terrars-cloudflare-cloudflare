#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ListData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<std::vec::Vec<List_Items>>,
    kind: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

struct List_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ListData_>,
}

#[derive(Clone)]
pub struct List(std::rc::Rc<List_>);

impl List {
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

    #[doc = "Set the field `description`.\nAn informative summary of the list."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `items`.\nThe items in the list. If set, this overwrites all items in the list. Do not use with `cloudflare_list_item`."]
    pub fn set_items(self, v: impl std::convert::Into<std::vec::Vec<List_Items>>) -> Self {
        self.0.data.borrow_mut().items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe Account ID for this resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe RFC 3339 timestamp of when the list was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative summary of the list."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique ID of the list."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `items` after provisioning.\nThe items in the list. If set, this overwrites all items in the list. Do not use with `cloudflare_list_item`."]
    pub fn items(&self) -> terrars::SetRef<List_ItemsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nThe type of the list. Each type supports specific list items (IP addresses, ASNs, hostnames or redirects).\nAvailable values: \"ip\", \"redirect\", \"hostname\", \"asn\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe RFC 3339 timestamp of when the list was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nAn informative name for the list. Use this name in filter and rule expressions."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_items` after provisioning.\nThe number of items in the list."]
    pub fn num_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `num_referencing_filters` after provisioning.\nThe number of [filters](/api/resources/filters/) referencing the list."]
    pub fn num_referencing_filters(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_referencing_filters", self.extract_ref()))
    }
}

impl terrars::Referable for List {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for List { }

impl terrars::ToListMappable for List {
    type O = terrars::ListRef<ListRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for List_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_list".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildList {
    pub tf_id: std::string::String,
    #[doc = "The Account ID for this resource."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of the list. Each type supports specific list items (IP addresses, ASNs, hostnames or redirects).\nAvailable values: \"ip\", \"redirect\", \"hostname\", \"asn\"."]
    pub kind: terrars::PrimField<std::string::String>,
    #[doc = "An informative name for the list. Use this name in filter and rule expressions."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildList {
    pub fn build(self, stack: &mut terrars::Stack) -> List {
        let out = List(std::rc::Rc::new(List_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ListData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                description: core::default::Default::default(),
                items: core::default::Default::default(),
                kind: self.kind,
                name: self.name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ListRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe Account ID for this resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe RFC 3339 timestamp of when the list was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative summary of the list."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique ID of the list."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `items` after provisioning.\nThe items in the list. If set, this overwrites all items in the list. Do not use with `cloudflare_list_item`."]
    pub fn items(&self) -> terrars::SetRef<List_ItemsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nThe type of the list. Each type supports specific list items (IP addresses, ASNs, hostnames or redirects).\nAvailable values: \"ip\", \"redirect\", \"hostname\", \"asn\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe RFC 3339 timestamp of when the list was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nAn informative name for the list. Use this name in filter and rule expressions."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_items` after provisioning.\nThe number of items in the list."]
    pub fn num_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `num_referencing_filters` after provisioning.\nThe number of [filters](/api/resources/filters/) referencing the list."]
    pub fn num_referencing_filters(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_referencing_filters", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct List_Items_Hostname {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_exact_hostname: std::option::Option<terrars::PrimField<bool>>,
    url_hostname: terrars::PrimField<std::string::String>,
}

impl List_Items_Hostname {
    #[doc =
        "Set the field `exclude_exact_hostname`.\nOnly applies to wildcard hostnames (e.g., *.example.com). When true (default), only subdomains are blocked. When false, both the root domain and subdomains are blocked."]
    pub fn set_exclude_exact_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_exact_hostname = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for List_Items_Hostname {
    type O = terrars::BlockAssignable<List_Items_Hostname>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildList_Items_Hostname {
    #[doc = ""]
    pub url_hostname: terrars::PrimField<std::string::String>,
}

impl BuildList_Items_Hostname {
    pub fn build(self) -> List_Items_Hostname {
        List_Items_Hostname {
            exclude_exact_hostname: core::default::Default::default(),
            url_hostname: self.url_hostname,
        }
    }
}

#[allow(dead_code)]
pub struct List_Items_HostnameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for List_Items_HostnameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> List_Items_HostnameRef {
        List_Items_HostnameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl List_Items_HostnameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude_exact_hostname` after provisioning.\nOnly applies to wildcard hostnames (e.g., *.example.com). When true (default), only subdomains are blocked. When false, both the root domain and subdomains are blocked."]
    pub fn exclude_exact_hostname(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_exact_hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `url_hostname` after provisioning.\n"]
    pub fn url_hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_hostname", self.base))
    }
}

#[derive(Serialize)]
pub struct List_Items_Redirect {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_subdomains: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_suffix: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_query_string: std::option::Option<terrars::PrimField<bool>>,
    source_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subpath_matching: std::option::Option<terrars::PrimField<bool>>,
    target_url: terrars::PrimField<std::string::String>,
}

impl List_Items_Redirect {
    #[doc = "Set the field `include_subdomains`.\n"]
    pub fn set_include_subdomains(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_subdomains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preserve_path_suffix`.\n"]
    pub fn set_preserve_path_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_path_suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preserve_query_string`.\n"]
    pub fn set_preserve_query_string(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_query_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\nAvailable values: 301, 302, 307, 308."]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subpath_matching`.\n"]
    pub fn set_subpath_matching(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.subpath_matching = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for List_Items_Redirect {
    type O = terrars::BlockAssignable<List_Items_Redirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildList_Items_Redirect {
    #[doc = ""]
    pub source_url: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_url: terrars::PrimField<std::string::String>,
}

impl BuildList_Items_Redirect {
    pub fn build(self) -> List_Items_Redirect {
        List_Items_Redirect {
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

#[allow(dead_code)]
pub struct List_Items_RedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for List_Items_RedirectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> List_Items_RedirectRef {
        List_Items_RedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl List_Items_RedirectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_subdomains` after provisioning.\n"]
    pub fn include_subdomains(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_subdomains", self.base))
    }

    #[doc = "Get a reference to the value of field `preserve_path_suffix` after provisioning.\n"]
    pub fn preserve_path_suffix(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_suffix", self.base))
    }

    #[doc = "Get a reference to the value of field `preserve_query_string` after provisioning.\n"]
    pub fn preserve_query_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `source_url` after provisioning.\n"]
    pub fn source_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status_code` after provisioning.\nAvailable values: 301, 302, 307, 308."]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc = "Get a reference to the value of field `subpath_matching` after provisioning.\n"]
    pub fn subpath_matching(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subpath_matching", self.base))
    }

    #[doc = "Get a reference to the value of field `target_url` after provisioning.\n"]
    pub fn target_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_url", self.base))
    }
}

#[derive(Serialize)]
pub struct List_Items {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asn: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<List_Items_Hostname>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<List_Items_Redirect>,
}

impl List_Items {
    #[doc = "Set the field `asn`.\nA non-negative 32 bit integer"]
    pub fn set_asn(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.asn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `comment`.\nAn informative summary of the list item."]
    pub fn set_comment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `hostname`.\nValid characters for hostnames are ASCII(7) letters from a to z, the digits from 0 to 9, wildcards (*), and the hyphen (-)."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<List_Items_Hostname>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip`.\nAn IPv4 address, an IPv4 CIDR, an IPv6 address, or an IPv6 CIDR."]
    pub fn set_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redirect`.\nThe definition of the redirect."]
    pub fn set_redirect(mut self, v: impl std::convert::Into<List_Items_Redirect>) -> Self {
        self.redirect = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for List_Items {
    type O = terrars::BlockAssignable<List_Items>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildList_Items {}

impl BuildList_Items {
    pub fn build(self) -> List_Items {
        List_Items {
            asn: core::default::Default::default(),
            comment: core::default::Default::default(),
            hostname: core::default::Default::default(),
            ip: core::default::Default::default(),
            redirect: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct List_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for List_ItemsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> List_ItemsRef {
        List_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl List_ItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asn` after provisioning.\nA non-negative 32 bit integer"]
    pub fn asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `comment` after provisioning.\nAn informative summary of the list item."]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nValid characters for hostnames are ASCII(7) letters from a to z, the digits from 0 to 9, wildcards (*), and the hyphen (-)."]
    pub fn hostname(&self) -> List_Items_HostnameRef {
        List_Items_HostnameRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nAn IPv4 address, an IPv4 CIDR, an IPv6 address, or an IPv6 CIDR."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\nThe definition of the redirect."]
    pub fn redirect(&self) -> List_Items_RedirectRef {
        List_Items_RedirectRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}
