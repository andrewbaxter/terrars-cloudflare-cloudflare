#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustResourceLibraryApplicationsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lookup: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offset: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustResourceLibraryApplications_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustResourceLibraryApplicationsData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustResourceLibraryApplications(std::rc::Rc<DataZeroTrustResourceLibraryApplications_>);

impl DataZeroTrustResourceLibraryApplications {
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

    #[doc =
        "Set the field `fields`.\nReturn only the listed properties on each application, as a comma-separated list.\nUse this to keep responses small when you only need part of each application, for\nexample populating a picker with `fields=id,name` instead of downloading every\nhostname and IP subnet.\n\nOmit this parameter to receive the full application object, including\nwhen lookup is used.\n\n`id` is always returned.\n\nSelectable properties: `id`, `name`, `human_id`, `version`, `hostnames`,\n`support_domains`, `ip_subnets`, `port_protocols`, `supported`, `gen_ai_score`,\n`application_confidence_score`, `created_at`, `updated_at`, `review_status`.\n\nUnknown or empty property names return `400`."]
    pub fn set_fields(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().fields = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter`.\nFilter applications using `key:value` format. Supported filter keys:\n- `name`: application name (for example, `name:HR`).\n- `id`: application ID (for example, `id:498`).\n- `human_id`: human-readable ID (for example, `human_id:HR`).\n- `hostname`: hostname or support domain (for example, `hostname:portal.example.com`).\n- `source`: application source name (for example, `source:cloudflare`).\n- `ip_subnet`: CIDR containment; returns applications where a stored subnet contains the search value (for example, `ip_subnet:10.0.1.5/32` matches `10.0.0.0/16`).\n- `category_id`: category ID (for example, `category_id:12`).\n- `category_name`: category name (for example, `category_name:HR`).\n- `supported`: supported Cloudflare product. Values: `GATEWAY`, `ACCESS`, `CASB`.\n- `review_status`: account-specific Gateway review status. Values: `approved`, `unapproved`, `in_review`, `unreviewed`."]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limit`.\nNumber of results to return. Defaults to 25; values are clamped to 1–250."]
    pub fn set_limit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().limit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `lookup`.\nFind complete, case-insensitive application name mentions in a phrase or\nrule sentence. Misspelled names do not match. Returns matching applications\nranked by relevance. Must be between 2 and 1000 characters. Uses the same\nfilters, fields, and pagination as listing. `search` cannot be combined\nwith lookup. Results are ranked by relevance, so `order_by` is ignored."]
    pub fn set_lookup(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().lookup = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `offset`.\nNumber of results to skip. Defaults to 0; negative values are clamped to 0."]
    pub fn set_offset(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().offset = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order_by`.\nOrder results using field:direction format. Supported fields are name, id, human_id,\ncategory_id, application_type, application_confidence_score, and gen_ai_score.\nSupported directions are asc and desc. Ignored when search is provided; results are\nranked by relevance instead. Also ignored when lookup is provided."]
    pub fn set_order_by(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order_by = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `search`.\nFuzzy search across application name and hostnames. Results are ranked by relevance. Must be between 2 and 200 characters. Can be combined with filter parameters."]
    pub fn set_search(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().search = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fields` after provisioning.\nReturn only the listed properties on each application, as a comma-separated list.\nUse this to keep responses small when you only need part of each application, for\nexample populating a picker with `fields=id,name` instead of downloading every\nhostname and IP subnet.\n\nOmit this parameter to receive the full application object, including\nwhen lookup is used.\n\n`id` is always returned.\n\nSelectable properties: `id`, `name`, `human_id`, `version`, `hostnames`,\n`support_domains`, `ip_subnets`, `port_protocols`, `supported`, `gen_ai_score`,\n`application_confidence_score`, `created_at`, `updated_at`, `review_status`.\n\nUnknown or empty property names return `400`."]
    pub fn fields(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fields", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter applications using `key:value` format. Supported filter keys:\n- `name`: application name (for example, `name:HR`).\n- `id`: application ID (for example, `id:498`).\n- `human_id`: human-readable ID (for example, `human_id:HR`).\n- `hostname`: hostname or support domain (for example, `hostname:portal.example.com`).\n- `source`: application source name (for example, `source:cloudflare`).\n- `ip_subnet`: CIDR containment; returns applications where a stored subnet contains the search value (for example, `ip_subnet:10.0.1.5/32` matches `10.0.0.0/16`).\n- `category_id`: category ID (for example, `category_id:12`).\n- `category_name`: category name (for example, `category_name:HR`).\n- `supported`: supported Cloudflare product. Values: `GATEWAY`, `ACCESS`, `CASB`.\n- `review_status`: account-specific Gateway review status. Values: `approved`, `unapproved`, `in_review`, `unreviewed`."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `limit` after provisioning.\nNumber of results to return. Defaults to 25; values are clamped to 1–250."]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lookup` after provisioning.\nFind complete, case-insensitive application name mentions in a phrase or\nrule sentence. Misspelled names do not match. Returns matching applications\nranked by relevance. Must be between 2 and 1000 characters. Uses the same\nfilters, fields, and pagination as listing. `search` cannot be combined\nwith lookup. Results are ranked by relevance, so `order_by` is ignored."]
    pub fn lookup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lookup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `offset` after provisioning.\nNumber of results to skip. Defaults to 0; negative values are clamped to 0."]
    pub fn offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offset", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order_by` after provisioning.\nOrder results using field:direction format. Supported fields are name, id, human_id,\ncategory_id, application_type, application_confidence_score, and gen_ai_score.\nSupported directions are asc and desc. Ignored when search is provided; results are\nranked by relevance instead. Also ignored when lookup is provided."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustResourceLibraryApplications_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nFuzzy search across application name and hostnames. Results are ranked by relevance. Must be between 2 and 200 characters. Can be combined with filter parameters."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustResourceLibraryApplications {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustResourceLibraryApplications { }

impl terrars::ToListMappable for DataZeroTrustResourceLibraryApplications {
    type O = terrars::ListRef<DataZeroTrustResourceLibraryApplicationsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustResourceLibraryApplications_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_resource_library_applications".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustResourceLibraryApplications {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustResourceLibraryApplications {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustResourceLibraryApplications {
        let out =
            DataZeroTrustResourceLibraryApplications(std::rc::Rc::new(DataZeroTrustResourceLibraryApplications_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(DataZeroTrustResourceLibraryApplicationsData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    account_id: self.account_id,
                    fields: core::default::Default::default(),
                    filter: core::default::Default::default(),
                    limit: core::default::Default::default(),
                    lookup: core::default::Default::default(),
                    max_items: core::default::Default::default(),
                    offset: core::default::Default::default(),
                    order_by: core::default::Default::default(),
                    search: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustResourceLibraryApplicationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustResourceLibraryApplicationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustResourceLibraryApplicationsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fields` after provisioning.\nReturn only the listed properties on each application, as a comma-separated list.\nUse this to keep responses small when you only need part of each application, for\nexample populating a picker with `fields=id,name` instead of downloading every\nhostname and IP subnet.\n\nOmit this parameter to receive the full application object, including\nwhen lookup is used.\n\n`id` is always returned.\n\nSelectable properties: `id`, `name`, `human_id`, `version`, `hostnames`,\n`support_domains`, `ip_subnets`, `port_protocols`, `supported`, `gen_ai_score`,\n`application_confidence_score`, `created_at`, `updated_at`, `review_status`.\n\nUnknown or empty property names return `400`."]
    pub fn fields(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fields", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter applications using `key:value` format. Supported filter keys:\n- `name`: application name (for example, `name:HR`).\n- `id`: application ID (for example, `id:498`).\n- `human_id`: human-readable ID (for example, `human_id:HR`).\n- `hostname`: hostname or support domain (for example, `hostname:portal.example.com`).\n- `source`: application source name (for example, `source:cloudflare`).\n- `ip_subnet`: CIDR containment; returns applications where a stored subnet contains the search value (for example, `ip_subnet:10.0.1.5/32` matches `10.0.0.0/16`).\n- `category_id`: category ID (for example, `category_id:12`).\n- `category_name`: category name (for example, `category_name:HR`).\n- `supported`: supported Cloudflare product. Values: `GATEWAY`, `ACCESS`, `CASB`.\n- `review_status`: account-specific Gateway review status. Values: `approved`, `unapproved`, `in_review`, `unreviewed`."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `limit` after provisioning.\nNumber of results to return. Defaults to 25; values are clamped to 1–250."]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lookup` after provisioning.\nFind complete, case-insensitive application name mentions in a phrase or\nrule sentence. Misspelled names do not match. Returns matching applications\nranked by relevance. Must be between 2 and 1000 characters. Uses the same\nfilters, fields, and pagination as listing. `search` cannot be combined\nwith lookup. Results are ranked by relevance, so `order_by` is ignored."]
    pub fn lookup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lookup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `offset` after provisioning.\nNumber of results to skip. Defaults to 0; negative values are clamped to 0."]
    pub fn offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offset", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order_by` after provisioning.\nOrder results using field:direction format. Supported fields are name, id, human_id,\ncategory_id, application_type, application_confidence_score, and gen_ai_score.\nSupported directions are asc and desc. Ignored when search is provided; results are\nranked by relevance instead. Also ignored when lookup is provided."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustResourceLibraryApplications_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nFuzzy search across application name and hostnames. Results are ranked by relevance. Must be between 2 and 200 characters. Can be combined with filter parameters."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustResourceLibraryApplications_Result {}

impl DataZeroTrustResourceLibraryApplications_Result { }

impl terrars::ToListMappable for DataZeroTrustResourceLibraryApplications_Result {
    type O = terrars::BlockAssignable<DataZeroTrustResourceLibraryApplications_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustResourceLibraryApplications_Result {}

impl BuildDataZeroTrustResourceLibraryApplications_Result {
    pub fn build(self) -> DataZeroTrustResourceLibraryApplications_Result {
        DataZeroTrustResourceLibraryApplications_Result {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustResourceLibraryApplications_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustResourceLibraryApplications_ResultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustResourceLibraryApplications_ResultRef {
        DataZeroTrustResourceLibraryApplications_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustResourceLibraryApplications_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `application_confidence_score` after provisioning.\nConfidence score for the application. Returns -1 when no score is available."]
    pub fn application_confidence_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_confidence_score", self.base))
    }

    #[doc =
        "Get a reference to the value of field `application_score_composition` after provisioning.\nReturns the score composition breakdown for the application."]
    pub fn application_score_composition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_score_composition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `application_source` after provisioning.\nReturns the application source."]
    pub fn application_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_source", self.base))
    }

    #[doc =
        "Get a reference to the value of field `application_type` after provisioning.\nReturns the application type."]
    pub fn application_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `application_type_description` after provisioning.\nReturns the application type description."]
    pub fn application_type_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_type_description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `category_id` after provisioning.\nNumeric identifier for an application category."]
    pub fn category_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nReturns the application creation time."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gen_ai_score` after provisioning.\nGenAI score for the application. Returns -1 when no score is available."]
    pub fn gen_ai_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gen_ai_score", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hostnames` after provisioning.\nHostnames matched by the application."]
    pub fn hostnames(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hostnames", self.base))
    }

    #[doc =
        "Get a reference to the value of field `human_id` after provisioning.\nHuman-readable identifier for the application."]
    pub fn human_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.human_id", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nNumeric identifier for the application."]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_subnets` after provisioning.\nIP subnets for this application. Custom application create and update requests accept IPv4 prefix lengths /8 through /32 and IPv6 prefix lengths /32 through /128."]
    pub fn ip_subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_subnets", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nApplication name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_protocols` after provisioning.\nPorts matched by the application, in `protocol/port` or inclusive `protocol/start-end` format (for example, `tcp/443` or `udp/10000-20000`). TCP and UDP ports must be between 0 and 65535."]
    pub fn port_protocols(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.port_protocols", self.base))
    }

    #[doc =
        "Get a reference to the value of field `review_status` after provisioning.\nThe account-specific Gateway review status. Applications with no assigned review status are returned as `unreviewed`.\nAvailable values: \"approved\", \"unapproved\", \"in_review\", \"unreviewed\"."]
    pub fn review_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.review_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `support_domains` after provisioning.\nSupport domains matched by the application."]
    pub fn support_domains(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.support_domains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `supported` after provisioning.\nCloudflare products that support this application."]
    pub fn supported(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nReturns the application update time."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nReturns the application version."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
