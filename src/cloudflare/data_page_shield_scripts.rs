#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataPageShieldScriptsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    script_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataPageShieldScripts_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataPageShieldScriptsData_>,
}

#[derive(Clone)]
pub struct DataPageShieldScripts(std::rc::Rc<DataPageShieldScripts_>);

impl DataPageShieldScripts {
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

    #[doc = "Set the field `zone_id`.\nIdentifier"]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `added_at` after provisioning.\n"]
    pub fn added_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.added_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cryptomining_score` after provisioning.\nThe cryptomining score of the JavaScript content."]
    pub fn cryptomining_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cryptomining_score", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataflow_score` after provisioning.\nThe dataflow score of the JavaScript content. This field has been deprecated in favour of js_integrity_score."]
    pub fn dataflow_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataflow_score", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_reported_malicious` after provisioning.\n"]
    pub fn domain_reported_malicious(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_reported_malicious", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fetched_at` after provisioning.\nThe timestamp of when the script was last fetched."]
    pub fn fetched_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fetched_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `first_page_url` after provisioning.\n"]
    pub fn first_page_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_page_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `first_seen_at` after provisioning.\n"]
    pub fn first_seen_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_seen_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hash` after provisioning.\nThe computed hash of the analyzed script."]
    pub fn hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hash", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `js_integrity_score` after provisioning.\nThe integrity score of the JavaScript content."]
    pub fn js_integrity_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.js_integrity_score", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_seen_at` after provisioning.\n"]
    pub fn last_seen_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_seen_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `magecart_score` after provisioning.\nThe magecart score of the JavaScript content."]
    pub fn magecart_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.magecart_score", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `malicious_domain_categories` after provisioning.\n"]
    pub fn malicious_domain_categories(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.malicious_domain_categories", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `malicious_url_categories` after provisioning.\n"]
    pub fn malicious_url_categories(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.malicious_url_categories", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `malware_score` after provisioning.\nThe malware score of the JavaScript content."]
    pub fn malware_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.malware_score", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `obfuscation_score` after provisioning.\nThe obfuscation score of the JavaScript content. This field has been deprecated in favour of js_integrity_score."]
    pub fn obfuscation_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.obfuscation_score", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `page_urls` after provisioning.\n"]
    pub fn page_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.page_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `script_id` after provisioning.\nIdentifier"]
    pub fn script_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url_contains_cdn_cgi_path` after provisioning.\n"]
    pub fn url_contains_cdn_cgi_path(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_contains_cdn_cgi_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url_reported_malicious` after provisioning.\n"]
    pub fn url_reported_malicious(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_reported_malicious", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `versions` after provisioning.\n"]
    pub fn versions(&self) -> terrars::ListRef<DataPageShieldScripts_VersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataPageShieldScripts {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataPageShieldScripts { }

impl terrars::ToListMappable for DataPageShieldScripts {
    type O = terrars::ListRef<DataPageShieldScriptsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataPageShieldScripts_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_page_shield_scripts".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPageShieldScripts {
    pub tf_id: std::string::String,
    #[doc = "Identifier"]
    pub script_id: terrars::PrimField<std::string::String>,
}

impl BuildDataPageShieldScripts {
    pub fn build(self, stack: &mut terrars::Stack) -> DataPageShieldScripts {
        let out = DataPageShieldScripts(std::rc::Rc::new(DataPageShieldScripts_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataPageShieldScriptsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                script_id: self.script_id,
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataPageShieldScriptsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPageShieldScriptsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPageShieldScriptsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `added_at` after provisioning.\n"]
    pub fn added_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.added_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cryptomining_score` after provisioning.\nThe cryptomining score of the JavaScript content."]
    pub fn cryptomining_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cryptomining_score", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataflow_score` after provisioning.\nThe dataflow score of the JavaScript content. This field has been deprecated in favour of js_integrity_score."]
    pub fn dataflow_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataflow_score", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_reported_malicious` after provisioning.\n"]
    pub fn domain_reported_malicious(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_reported_malicious", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fetched_at` after provisioning.\nThe timestamp of when the script was last fetched."]
    pub fn fetched_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fetched_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `first_page_url` after provisioning.\n"]
    pub fn first_page_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_page_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `first_seen_at` after provisioning.\n"]
    pub fn first_seen_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_seen_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hash` after provisioning.\nThe computed hash of the analyzed script."]
    pub fn hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hash", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `js_integrity_score` after provisioning.\nThe integrity score of the JavaScript content."]
    pub fn js_integrity_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.js_integrity_score", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_seen_at` after provisioning.\n"]
    pub fn last_seen_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_seen_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `magecart_score` after provisioning.\nThe magecart score of the JavaScript content."]
    pub fn magecart_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.magecart_score", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `malicious_domain_categories` after provisioning.\n"]
    pub fn malicious_domain_categories(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.malicious_domain_categories", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `malicious_url_categories` after provisioning.\n"]
    pub fn malicious_url_categories(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.malicious_url_categories", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `malware_score` after provisioning.\nThe malware score of the JavaScript content."]
    pub fn malware_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.malware_score", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `obfuscation_score` after provisioning.\nThe obfuscation score of the JavaScript content. This field has been deprecated in favour of js_integrity_score."]
    pub fn obfuscation_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.obfuscation_score", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `page_urls` after provisioning.\n"]
    pub fn page_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.page_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `script_id` after provisioning.\nIdentifier"]
    pub fn script_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url_contains_cdn_cgi_path` after provisioning.\n"]
    pub fn url_contains_cdn_cgi_path(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_contains_cdn_cgi_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url_reported_malicious` after provisioning.\n"]
    pub fn url_reported_malicious(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_reported_malicious", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `versions` after provisioning.\n"]
    pub fn versions(&self) -> terrars::ListRef<DataPageShieldScripts_VersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataPageShieldScripts_Versions {}

impl DataPageShieldScripts_Versions { }

impl terrars::ToListMappable for DataPageShieldScripts_Versions {
    type O = terrars::BlockAssignable<DataPageShieldScripts_Versions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPageShieldScripts_Versions {}

impl BuildDataPageShieldScripts_Versions {
    pub fn build(self) -> DataPageShieldScripts_Versions {
        DataPageShieldScripts_Versions {}
    }
}

#[allow(dead_code)]
pub struct DataPageShieldScripts_VersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPageShieldScripts_VersionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPageShieldScripts_VersionsRef {
        DataPageShieldScripts_VersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPageShieldScripts_VersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cryptomining_score` after provisioning.\nThe cryptomining score of the JavaScript content."]
    pub fn cryptomining_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cryptomining_score", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dataflow_score` after provisioning.\nThe dataflow score of the JavaScript content. This field has been deprecated in favour of js_integrity_score."]
    pub fn dataflow_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataflow_score", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fetched_at` after provisioning.\nThe timestamp of when the script was last fetched."]
    pub fn fetched_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fetched_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hash` after provisioning.\nThe computed hash of the analyzed script."]
    pub fn hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hash", self.base))
    }

    #[doc =
        "Get a reference to the value of field `js_integrity_score` after provisioning.\nThe integrity score of the JavaScript content."]
    pub fn js_integrity_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.js_integrity_score", self.base))
    }

    #[doc =
        "Get a reference to the value of field `magecart_score` after provisioning.\nThe magecart score of the JavaScript content."]
    pub fn magecart_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.magecart_score", self.base))
    }

    #[doc =
        "Get a reference to the value of field `malware_score` after provisioning.\nThe malware score of the JavaScript content."]
    pub fn malware_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.malware_score", self.base))
    }

    #[doc =
        "Get a reference to the value of field `obfuscation_score` after provisioning.\nThe obfuscation score of the JavaScript content. This field has been deprecated in favour of js_integrity_score."]
    pub fn obfuscation_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.obfuscation_score", self.base))
    }
}
