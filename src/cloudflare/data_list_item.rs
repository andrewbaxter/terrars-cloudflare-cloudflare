#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataListItemData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    item_id: terrars::PrimField<std::string::String>,
    list_id: terrars::PrimField<std::string::String>,
}

struct DataListItem_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataListItemData_>,
}

#[derive(Clone)]
pub struct DataListItem(std::rc::Rc<DataListItem_>);

impl DataListItem {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderCloudflare) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `account_id`.\nThe Account ID for this resource."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe Account ID for this resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `asn` after provisioning.\nDefines a non-negative 32 bit integer."]
    pub fn asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `comment` after provisioning.\nDefines an informative summary of the list item."]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe RFC 3339 timestamp of when the list was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nHostnames support ASCII(7) letters from a to z, the digits from 0 to 9, wildcards (*), and the hyphen (-)."]
    pub fn hostname(&self) -> DataListItem_HostnameRef {
        DataListItem_HostnameRef::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nDefines the unique ID of the item in the List."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nAn IPv4 address, an IPv4 CIDR, an IPv6 address, or an IPv6 CIDR."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `item_id` after provisioning.\nDefines the unique ID of the item in the List."]
    pub fn item_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.item_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `list_id` after provisioning.\nThe unique ID of the list."]
    pub fn list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.list_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe RFC 3339 timestamp of when the list was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\nThe definition of the redirect."]
    pub fn redirect(&self) -> DataListItem_RedirectRef {
        DataListItem_RedirectRef::new(self.shared().clone(), format!("{}.redirect", self.extract_ref()))
    }
}

impl terrars::Referable for DataListItem {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataListItem { }

impl terrars::ToListMappable for DataListItem {
    type O = terrars::ListRef<DataListItemRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataListItem_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_list_item".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataListItem {
    pub tf_id: std::string::String,
    #[doc = "Defines the unique ID of the item in the List."]
    pub item_id: terrars::PrimField<std::string::String>,
    #[doc = "The unique ID of the list."]
    pub list_id: terrars::PrimField<std::string::String>,
}

impl BuildDataListItem {
    pub fn build(self, stack: &mut terrars::Stack) -> DataListItem {
        let out = DataListItem(std::rc::Rc::new(DataListItem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataListItemData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                item_id: self.item_id,
                list_id: self.list_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataListItemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataListItemRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataListItemRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe Account ID for this resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `asn` after provisioning.\nDefines a non-negative 32 bit integer."]
    pub fn asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `comment` after provisioning.\nDefines an informative summary of the list item."]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe RFC 3339 timestamp of when the list was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nHostnames support ASCII(7) letters from a to z, the digits from 0 to 9, wildcards (*), and the hyphen (-)."]
    pub fn hostname(&self) -> DataListItem_HostnameRef {
        DataListItem_HostnameRef::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nDefines the unique ID of the item in the List."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nAn IPv4 address, an IPv4 CIDR, an IPv6 address, or an IPv6 CIDR."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `item_id` after provisioning.\nDefines the unique ID of the item in the List."]
    pub fn item_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.item_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `list_id` after provisioning.\nThe unique ID of the list."]
    pub fn list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.list_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe RFC 3339 timestamp of when the list was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\nThe definition of the redirect."]
    pub fn redirect(&self) -> DataListItem_RedirectRef {
        DataListItem_RedirectRef::new(self.shared().clone(), format!("{}.redirect", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataListItem_Hostname {}

impl DataListItem_Hostname { }

impl terrars::ToListMappable for DataListItem_Hostname {
    type O = terrars::BlockAssignable<DataListItem_Hostname>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataListItem_Hostname {}

impl BuildDataListItem_Hostname {
    pub fn build(self) -> DataListItem_Hostname {
        DataListItem_Hostname {}
    }
}

#[allow(dead_code)]
pub struct DataListItem_HostnameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataListItem_HostnameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataListItem_HostnameRef {
        DataListItem_HostnameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataListItem_HostnameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude_exact_hostname` after provisioning.\nOnly applies to wildcard hostnames (e.g., *.example.com). When true (default), the rule blocks only subdomains. When false, the rule blocks both the root domain and subdomains."]
    pub fn exclude_exact_hostname(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_exact_hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `url_hostname` after provisioning.\n"]
    pub fn url_hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_hostname", self.base))
    }
}

#[derive(Serialize)]
pub struct DataListItem_Redirect {}

impl DataListItem_Redirect { }

impl terrars::ToListMappable for DataListItem_Redirect {
    type O = terrars::BlockAssignable<DataListItem_Redirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataListItem_Redirect {}

impl BuildDataListItem_Redirect {
    pub fn build(self) -> DataListItem_Redirect {
        DataListItem_Redirect {}
    }
}

#[allow(dead_code)]
pub struct DataListItem_RedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataListItem_RedirectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataListItem_RedirectRef {
        DataListItem_RedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataListItem_RedirectRef {
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
