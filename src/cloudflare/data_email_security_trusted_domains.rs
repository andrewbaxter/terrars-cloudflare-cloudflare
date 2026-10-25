#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataEmailSecurityTrustedDomainsData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataEmailSecurityTrustedDomains_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trusted_domain_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataEmailSecurityTrustedDomains_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEmailSecurityTrustedDomainsData_>,
}

#[derive(Clone)]
pub struct DataEmailSecurityTrustedDomains(std::rc::Rc<DataEmailSecurityTrustedDomains_>);

impl DataEmailSecurityTrustedDomains {
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

    #[doc = "Set the field `account_id`.\nIdentifier."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataEmailSecurityTrustedDomains_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trusted_domain_id`.\nTrusted domain identifier"]
    pub fn set_trusted_domain_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().trusted_domain_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comments` after provisioning.\n"]
    pub fn comments(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataEmailSecurityTrustedDomains_FilterRef {
        DataEmailSecurityTrustedDomains_FilterRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nTrusted domain identifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_recent` after provisioning.\nSelect to prevent recently registered domains from triggering a Suspicious or Malicious disposition."]
    pub fn is_recent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_recent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_regex` after provisioning.\nWhether `pattern` is a regular expression instead of a literal domain."]
    pub fn is_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_regex", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_similarity` after provisioning.\nSelect for partner or other approved domains that have similar spelling to your connected domains. Prevents listed domains from triggering a Spoof disposition."]
    pub fn is_similarity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_similarity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified` after provisioning.\nDeprecated, use `modified_at` instead. End of life: November 1, 2026."]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pattern` after provisioning.\nThe domain pattern to trust, e.g. `example.com`."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trusted_domain_id` after provisioning.\nTrusted domain identifier"]
    pub fn trusted_domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trusted_domain_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataEmailSecurityTrustedDomains {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEmailSecurityTrustedDomains { }

impl terrars::ToListMappable for DataEmailSecurityTrustedDomains {
    type O = terrars::ListRef<DataEmailSecurityTrustedDomainsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEmailSecurityTrustedDomains_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_email_security_trusted_domains".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEmailSecurityTrustedDomains {
    pub tf_id: std::string::String,
}

impl BuildDataEmailSecurityTrustedDomains {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEmailSecurityTrustedDomains {
        let out = DataEmailSecurityTrustedDomains(std::rc::Rc::new(DataEmailSecurityTrustedDomains_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEmailSecurityTrustedDomainsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                trusted_domain_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEmailSecurityTrustedDomainsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailSecurityTrustedDomainsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEmailSecurityTrustedDomainsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comments` after provisioning.\n"]
    pub fn comments(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataEmailSecurityTrustedDomains_FilterRef {
        DataEmailSecurityTrustedDomains_FilterRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nTrusted domain identifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_recent` after provisioning.\nSelect to prevent recently registered domains from triggering a Suspicious or Malicious disposition."]
    pub fn is_recent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_recent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_regex` after provisioning.\nWhether `pattern` is a regular expression instead of a literal domain."]
    pub fn is_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_regex", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_similarity` after provisioning.\nSelect for partner or other approved domains that have similar spelling to your connected domains. Prevents listed domains from triggering a Spoof disposition."]
    pub fn is_similarity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_similarity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified` after provisioning.\nDeprecated, use `modified_at` instead. End of life: November 1, 2026."]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pattern` after provisioning.\nThe domain pattern to trust, e.g. `example.com`."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trusted_domain_id` after provisioning.\nTrusted domain identifier"]
    pub fn trusted_domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trusted_domain_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEmailSecurityTrustedDomains_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_recent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_similarity: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pattern: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEmailSecurityTrustedDomains_Filter {
    #[doc = "Set the field `direction`.\nThe sorting direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_recent`.\nFilter to show only recently registered domains that are trusted to prevent triggering Suspicious or Malicious dispositions."]
    pub fn set_is_recent(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_recent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_similarity`.\nFilter to show only proximity domains (partner or approved domains with similar spelling to connected domains) that prevent Spoof dispositions."]
    pub fn set_is_similarity(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_similarity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `order`.\nField to sort by.\nAvailable values: \"pattern\", \"created_at\"."]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pattern`.\n"]
    pub fn set_pattern(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pattern = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\nSearch term for filtering records. Behavior may change."]
    pub fn set_search(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.search = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEmailSecurityTrustedDomains_Filter {
    type O = terrars::BlockAssignable<DataEmailSecurityTrustedDomains_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmailSecurityTrustedDomains_Filter {}

impl BuildDataEmailSecurityTrustedDomains_Filter {
    pub fn build(self) -> DataEmailSecurityTrustedDomains_Filter {
        DataEmailSecurityTrustedDomains_Filter {
            direction: core::default::Default::default(),
            is_recent: core::default::Default::default(),
            is_similarity: core::default::Default::default(),
            order: core::default::Default::default(),
            pattern: core::default::Default::default(),
            search: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEmailSecurityTrustedDomains_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailSecurityTrustedDomains_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEmailSecurityTrustedDomains_FilterRef {
        DataEmailSecurityTrustedDomains_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmailSecurityTrustedDomains_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe sorting direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_recent` after provisioning.\nFilter to show only recently registered domains that are trusted to prevent triggering Suspicious or Malicious dispositions."]
    pub fn is_recent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_recent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_similarity` after provisioning.\nFilter to show only proximity domains (partner or approved domains with similar spelling to connected domains) that prevent Spoof dispositions."]
    pub fn is_similarity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_similarity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to sort by.\nAvailable values: \"pattern\", \"created_at\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nSearch term for filtering records. Behavior may change."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.base))
    }
}
