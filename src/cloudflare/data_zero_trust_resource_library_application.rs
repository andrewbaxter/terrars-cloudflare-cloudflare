#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustResourceLibraryApplicationData_ {
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
    filter: std::option::Option<DataZeroTrustResourceLibraryApplication_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<f64>>,
}

struct DataZeroTrustResourceLibraryApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustResourceLibraryApplicationData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustResourceLibraryApplication(std::rc::Rc<DataZeroTrustResourceLibraryApplication_>);

impl DataZeroTrustResourceLibraryApplication {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataZeroTrustResourceLibraryApplication_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nNumeric identifier for the application."]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_confidence_score` after provisioning.\nConfidence score for the application. Returns -1 when no score is available."]
    pub fn application_confidence_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_confidence_score", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_score_composition` after provisioning.\nReturns the score composition breakdown for the application."]
    pub fn application_score_composition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_score_composition", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `application_source` after provisioning.\nReturns the application source."]
    pub fn application_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_type` after provisioning.\nReturns the application type."]
    pub fn application_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_type_description` after provisioning.\nReturns the application type description."]
    pub fn application_type_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_type_description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `category_id` after provisioning.\nNumeric identifier for an application category."]
    pub fn category_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nReturns the application creation time."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustResourceLibraryApplication_FilterRef {
        DataZeroTrustResourceLibraryApplication_FilterRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `gen_ai_score` after provisioning.\nGenAI score for the application. Returns -1 when no score is available."]
    pub fn gen_ai_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gen_ai_score", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostnames` after provisioning.\nHostnames matched by the application."]
    pub fn hostnames(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hostnames", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `human_id` after provisioning.\nHuman-readable identifier for the application."]
    pub fn human_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.human_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nNumeric identifier for the application."]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_subnets` after provisioning.\nIP subnets for this application. Custom application create and update requests accept IPv4 prefix lengths /8 through /32 and IPv6 prefix lengths /32 through /128."]
    pub fn ip_subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_subnets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nApplication name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port_protocols` after provisioning.\nPorts matched by the application, in `protocol/port` or inclusive `protocol/start-end` format (for example, `tcp/443` or `udp/10000-20000`). TCP and UDP ports must be between 0 and 65535."]
    pub fn port_protocols(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.port_protocols", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `support_domains` after provisioning.\nSupport domains matched by the application."]
    pub fn support_domains(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.support_domains", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported` after provisioning.\nCloudflare products that support this application."]
    pub fn supported(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nReturns the application update time."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nReturns the application version."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustResourceLibraryApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustResourceLibraryApplication { }

impl terrars::ToListMappable for DataZeroTrustResourceLibraryApplication {
    type O = terrars::ListRef<DataZeroTrustResourceLibraryApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustResourceLibraryApplication_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_resource_library_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustResourceLibraryApplication {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustResourceLibraryApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustResourceLibraryApplication {
        let out = DataZeroTrustResourceLibraryApplication(std::rc::Rc::new(DataZeroTrustResourceLibraryApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustResourceLibraryApplicationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustResourceLibraryApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustResourceLibraryApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustResourceLibraryApplicationRef {
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
        "Get a reference to the value of field `application_confidence_score` after provisioning.\nConfidence score for the application. Returns -1 when no score is available."]
    pub fn application_confidence_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_confidence_score", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_score_composition` after provisioning.\nReturns the score composition breakdown for the application."]
    pub fn application_score_composition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_score_composition", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `application_source` after provisioning.\nReturns the application source."]
    pub fn application_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_type` after provisioning.\nReturns the application type."]
    pub fn application_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_type_description` after provisioning.\nReturns the application type description."]
    pub fn application_type_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_type_description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `category_id` after provisioning.\nNumeric identifier for an application category."]
    pub fn category_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nReturns the application creation time."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustResourceLibraryApplication_FilterRef {
        DataZeroTrustResourceLibraryApplication_FilterRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `gen_ai_score` after provisioning.\nGenAI score for the application. Returns -1 when no score is available."]
    pub fn gen_ai_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gen_ai_score", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostnames` after provisioning.\nHostnames matched by the application."]
    pub fn hostnames(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hostnames", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `human_id` after provisioning.\nHuman-readable identifier for the application."]
    pub fn human_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.human_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nNumeric identifier for the application."]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_subnets` after provisioning.\nIP subnets for this application. Custom application create and update requests accept IPv4 prefix lengths /8 through /32 and IPv6 prefix lengths /32 through /128."]
    pub fn ip_subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_subnets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nApplication name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port_protocols` after provisioning.\nPorts matched by the application, in `protocol/port` or inclusive `protocol/start-end` format (for example, `tcp/443` or `udp/10000-20000`). TCP and UDP ports must be between 0 and 65535."]
    pub fn port_protocols(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.port_protocols", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `support_domains` after provisioning.\nSupport domains matched by the application."]
    pub fn support_domains(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.support_domains", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported` after provisioning.\nCloudflare products that support this application."]
    pub fn supported(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nReturns the application update time."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nReturns the application version."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustResourceLibraryApplication_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lookup: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offset: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataZeroTrustResourceLibraryApplication_Filter {
    #[doc =
        "Set the field `fields`.\nReturn only the listed properties on each application, as a comma-separated list.\nUse this to keep responses small when you only need part of each application, for\nexample populating a picker with `fields=id,name` instead of downloading every\nhostname and IP subnet.\n\nOmit this parameter to receive the full application object, including\nwhen lookup is used.\n\n`id` is always returned.\n\nSelectable properties: `id`, `name`, `human_id`, `version`, `hostnames`,\n`support_domains`, `ip_subnets`, `port_protocols`, `supported`, `gen_ai_score`,\n`application_confidence_score`, `created_at`, `updated_at`, `review_status`.\n\nUnknown or empty property names return `400`."]
    pub fn set_fields(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fields = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter`.\nFilter applications using `key:value` format. Supported filter keys:\n- `name`: application name (for example, `name:HR`).\n- `id`: application ID (for example, `id:498`).\n- `human_id`: human-readable ID (for example, `human_id:HR`).\n- `hostname`: hostname or support domain (for example, `hostname:portal.example.com`).\n- `source`: application source name (for example, `source:cloudflare`).\n- `ip_subnet`: CIDR containment; returns applications where a stored subnet contains the search value (for example, `ip_subnet:10.0.1.5/32` matches `10.0.0.0/16`).\n- `category_id`: category ID (for example, `category_id:12`).\n- `category_name`: category name (for example, `category_name:HR`).\n- `supported`: supported Cloudflare product. Values: `GATEWAY`, `ACCESS`, `CASB`.\n- `review_status`: account-specific Gateway review status. Values: `approved`, `unapproved`, `in_review`, `unreviewed`."]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limit`.\nNumber of results to return. Defaults to 25; values are clamped to 1–250."]
    pub fn set_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.limit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `lookup`.\nFind complete, case-insensitive application name mentions in a phrase or\nrule sentence. Misspelled names do not match. Returns matching applications\nranked by relevance. Must be between 2 and 1000 characters. Uses the same\nfilters, fields, and pagination as listing. `search` cannot be combined\nwith lookup. Results are ranked by relevance, so `order_by` is ignored."]
    pub fn set_lookup(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lookup = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `offset`.\nNumber of results to skip. Defaults to 0; negative values are clamped to 0."]
    pub fn set_offset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.offset = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order_by`.\nOrder results using field:direction format. Supported fields are name, id, human_id,\ncategory_id, application_type, application_confidence_score, and gen_ai_score.\nSupported directions are asc and desc. Ignored when search is provided; results are\nranked by relevance instead. Also ignored when lookup is provided."]
    pub fn set_order_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order_by = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `search`.\nFuzzy search across application name and hostnames. Results are ranked by relevance. Must be between 2 and 200 characters. Can be combined with filter parameters."]
    pub fn set_search(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.search = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataZeroTrustResourceLibraryApplication_Filter {
    type O = terrars::BlockAssignable<DataZeroTrustResourceLibraryApplication_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustResourceLibraryApplication_Filter {}

impl BuildDataZeroTrustResourceLibraryApplication_Filter {
    pub fn build(self) -> DataZeroTrustResourceLibraryApplication_Filter {
        DataZeroTrustResourceLibraryApplication_Filter {
            fields: core::default::Default::default(),
            filter: core::default::Default::default(),
            limit: core::default::Default::default(),
            lookup: core::default::Default::default(),
            offset: core::default::Default::default(),
            order_by: core::default::Default::default(),
            search: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustResourceLibraryApplication_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustResourceLibraryApplication_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustResourceLibraryApplication_FilterRef {
        DataZeroTrustResourceLibraryApplication_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustResourceLibraryApplication_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fields` after provisioning.\nReturn only the listed properties on each application, as a comma-separated list.\nUse this to keep responses small when you only need part of each application, for\nexample populating a picker with `fields=id,name` instead of downloading every\nhostname and IP subnet.\n\nOmit this parameter to receive the full application object, including\nwhen lookup is used.\n\n`id` is always returned.\n\nSelectable properties: `id`, `name`, `human_id`, `version`, `hostnames`,\n`support_domains`, `ip_subnets`, `port_protocols`, `supported`, `gen_ai_score`,\n`application_confidence_score`, `created_at`, `updated_at`, `review_status`.\n\nUnknown or empty property names return `400`."]
    pub fn fields(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter applications using `key:value` format. Supported filter keys:\n- `name`: application name (for example, `name:HR`).\n- `id`: application ID (for example, `id:498`).\n- `human_id`: human-readable ID (for example, `human_id:HR`).\n- `hostname`: hostname or support domain (for example, `hostname:portal.example.com`).\n- `source`: application source name (for example, `source:cloudflare`).\n- `ip_subnet`: CIDR containment; returns applications where a stored subnet contains the search value (for example, `ip_subnet:10.0.1.5/32` matches `10.0.0.0/16`).\n- `category_id`: category ID (for example, `category_id:12`).\n- `category_name`: category name (for example, `category_name:HR`).\n- `supported`: supported Cloudflare product. Values: `GATEWAY`, `ACCESS`, `CASB`.\n- `review_status`: account-specific Gateway review status. Values: `approved`, `unapproved`, `in_review`, `unreviewed`."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `limit` after provisioning.\nNumber of results to return. Defaults to 25; values are clamped to 1–250."]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lookup` after provisioning.\nFind complete, case-insensitive application name mentions in a phrase or\nrule sentence. Misspelled names do not match. Returns matching applications\nranked by relevance. Must be between 2 and 1000 characters. Uses the same\nfilters, fields, and pagination as listing. `search` cannot be combined\nwith lookup. Results are ranked by relevance, so `order_by` is ignored."]
    pub fn lookup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lookup", self.base))
    }

    #[doc =
        "Get a reference to the value of field `offset` after provisioning.\nNumber of results to skip. Defaults to 0; negative values are clamped to 0."]
    pub fn offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order_by` after provisioning.\nOrder results using field:direction format. Supported fields are name, id, human_id,\ncategory_id, application_type, application_confidence_score, and gen_ai_score.\nSupported directions are asc and desc. Ignored when search is provided; results are\nranked by relevance instead. Also ignored when lookup is provided."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.base))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nFuzzy search across application name and hostnames. Results are ranked by relevance. Must be between 2 and 200 characters. Can be combined with filter parameters."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.base))
    }
}
