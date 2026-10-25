#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataAiSearchInstancesData_ {
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
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order_by_direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataAiSearchInstances_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAiSearchInstancesData_>,
}

#[derive(Clone)]
pub struct DataAiSearchInstances(std::rc::Rc<DataAiSearchInstances_>);

impl DataAiSearchInstances {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nFilter by exact Search for Agents hostname (case-insensitive)."]
    pub fn set_hostname(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace`.\nFilter by namespace."]
    pub fn set_namespace(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `order_by`.\nField to order results by.\nAvailable values: \"created_at\"."]
    pub fn set_order_by(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `order_by_direction`.\nOrder direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_order_by_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order_by_direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\nFilter instances whose id contains this string (case-insensitive)."]
    pub fn set_search(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().search = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nFilter by exact Search for Agents hostname (case-insensitive)."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nFilter by namespace."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order_by` after provisioning.\nField to order results by.\nAvailable values: \"created_at\"."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order_by_direction` after provisioning.\nOrder direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn order_by_direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by_direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataAiSearchInstances_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nFilter instances whose id contains this string (case-insensitive)."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }
}

impl terrars::Referable for DataAiSearchInstances {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAiSearchInstances { }

impl terrars::ToListMappable for DataAiSearchInstances {
    type O = terrars::ListRef<DataAiSearchInstancesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAiSearchInstances_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_ai_search_instances".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAiSearchInstances {
    pub tf_id: std::string::String,
}

impl BuildDataAiSearchInstances {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAiSearchInstances {
        let out = DataAiSearchInstances(std::rc::Rc::new(DataAiSearchInstances_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAiSearchInstancesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                hostname: core::default::Default::default(),
                max_items: core::default::Default::default(),
                namespace: core::default::Default::default(),
                order_by: core::default::Default::default(),
                order_by_direction: core::default::Default::default(),
                search: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAiSearchInstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstancesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAiSearchInstancesRef {
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
        "Get a reference to the value of field `hostname` after provisioning.\nFilter by exact Search for Agents hostname (case-insensitive)."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nFilter by namespace."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order_by` after provisioning.\nField to order results by.\nAvailable values: \"created_at\"."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order_by_direction` after provisioning.\nOrder direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn order_by_direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by_direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataAiSearchInstances_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nFilter instances whose id contains this string (case-insensitive)."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_CustomMetadata {}

impl DataAiSearchInstances_Result_CustomMetadata { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_CustomMetadata {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_CustomMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_CustomMetadata {}

impl BuildDataAiSearchInstances_Result_CustomMetadata {
    pub fn build(self) -> DataAiSearchInstances_Result_CustomMetadata {
        DataAiSearchInstances_Result_CustomMetadata {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_CustomMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_CustomMetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchInstances_Result_CustomMetadataRef {
        DataAiSearchInstances_Result_CustomMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_CustomMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_type` after provisioning.\nAvailable values: \"text\", \"number\", \"boolean\", \"datetime\"."]
    pub fn data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `field_name` after provisioning.\n"]
    pub fn field_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_IndexMethod {}

impl DataAiSearchInstances_Result_IndexMethod { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_IndexMethod {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_IndexMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_IndexMethod {}

impl BuildDataAiSearchInstances_Result_IndexMethod {
    pub fn build(self) -> DataAiSearchInstances_Result_IndexMethod {
        DataAiSearchInstances_Result_IndexMethod {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_IndexMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_IndexMethodRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiSearchInstances_Result_IndexMethodRef {
        DataAiSearchInstances_Result_IndexMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_IndexMethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `keyword` after provisioning.\nEnable keyword (BM25) storage backend."]
    pub fn keyword(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keyword", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vector` after provisioning.\nEnable vector (embedding) storage backend."]
    pub fn vector(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vector", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_IndexingOptions {}

impl DataAiSearchInstances_Result_IndexingOptions { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_IndexingOptions {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_IndexingOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_IndexingOptions {}

impl BuildDataAiSearchInstances_Result_IndexingOptions {
    pub fn build(self) -> DataAiSearchInstances_Result_IndexingOptions {
        DataAiSearchInstances_Result_IndexingOptions {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_IndexingOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_IndexingOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchInstances_Result_IndexingOptionsRef {
        DataAiSearchInstances_Result_IndexingOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_IndexingOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `keyword_tokenizer` after provisioning.\nTokenizer used for keyword search indexing. porter provides word-level tokenization with Porter stemming (good for natural language queries). trigram enables character-level substring matching (good for partial matches, code, identifiers). Changing this triggers a full re-index. Defaults to porter.\nAvailable values: \"porter\", \"trigram\"."]
    pub fn keyword_tokenizer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keyword_tokenizer", self.base))
    }

    #[doc = "Get a reference to the value of field `use_ocr` after provisioning.\n"]
    pub fn use_ocr(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_ocr", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_Metadata_CreatedFromEmdashPlugin {}

impl DataAiSearchInstances_Result_Metadata_CreatedFromEmdashPlugin { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_Metadata_CreatedFromEmdashPlugin {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_Metadata_CreatedFromEmdashPlugin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_Metadata_CreatedFromEmdashPlugin {}

impl BuildDataAiSearchInstances_Result_Metadata_CreatedFromEmdashPlugin {
    pub fn build(self) -> DataAiSearchInstances_Result_Metadata_CreatedFromEmdashPlugin {
        DataAiSearchInstances_Result_Metadata_CreatedFromEmdashPlugin {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_Metadata_CreatedFromEmdashPluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_Metadata_CreatedFromEmdashPluginRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchInstances_Result_Metadata_CreatedFromEmdashPluginRef {
        DataAiSearchInstances_Result_Metadata_CreatedFromEmdashPluginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_Metadata_CreatedFromEmdashPluginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"native\", \"rest\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_Metadata {}

impl DataAiSearchInstances_Result_Metadata { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_Metadata {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_Metadata {}

impl BuildDataAiSearchInstances_Result_Metadata {
    pub fn build(self) -> DataAiSearchInstances_Result_Metadata {
        DataAiSearchInstances_Result_Metadata {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_MetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiSearchInstances_Result_MetadataRef {
        DataAiSearchInstances_Result_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_from_aisearch_wizard` after provisioning.\n"]
    pub fn created_from_aisearch_wizard(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_from_aisearch_wizard", self.base))
    }

    #[doc = "Get a reference to the value of field `created_from_emdash_plugin` after provisioning.\n"]
    pub fn created_from_emdash_plugin(&self) -> DataAiSearchInstances_Result_Metadata_CreatedFromEmdashPluginRef {
        DataAiSearchInstances_Result_Metadata_CreatedFromEmdashPluginRef::new(
            self.shared().clone(),
            format!("{}.created_from_emdash_plugin", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `worker_domain` after provisioning.\n"]
    pub fn worker_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_domain", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_PublicEndpointParams_ChatCompletionsEndpoint {}

impl DataAiSearchInstances_Result_PublicEndpointParams_ChatCompletionsEndpoint { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_PublicEndpointParams_ChatCompletionsEndpoint {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_PublicEndpointParams_ChatCompletionsEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_PublicEndpointParams_ChatCompletionsEndpoint {}

impl BuildDataAiSearchInstances_Result_PublicEndpointParams_ChatCompletionsEndpoint {
    pub fn build(self) -> DataAiSearchInstances_Result_PublicEndpointParams_ChatCompletionsEndpoint {
        DataAiSearchInstances_Result_PublicEndpointParams_ChatCompletionsEndpoint {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_PublicEndpointParams_ChatCompletionsEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_PublicEndpointParams_ChatCompletionsEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchInstances_Result_PublicEndpointParams_ChatCompletionsEndpointRef {
        DataAiSearchInstances_Result_PublicEndpointParams_ChatCompletionsEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_PublicEndpointParams_ChatCompletionsEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nDisable chat completions endpoint for this public endpoint"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_PublicEndpointParams_Mcp {}

impl DataAiSearchInstances_Result_PublicEndpointParams_Mcp { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_PublicEndpointParams_Mcp {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_PublicEndpointParams_Mcp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_PublicEndpointParams_Mcp {}

impl BuildDataAiSearchInstances_Result_PublicEndpointParams_Mcp {
    pub fn build(self) -> DataAiSearchInstances_Result_PublicEndpointParams_Mcp {
        DataAiSearchInstances_Result_PublicEndpointParams_Mcp {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_PublicEndpointParams_McpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_PublicEndpointParams_McpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchInstances_Result_PublicEndpointParams_McpRef {
        DataAiSearchInstances_Result_PublicEndpointParams_McpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_PublicEndpointParams_McpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nDisable MCP endpoint for this public endpoint"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_PublicEndpointParams_RateLimit {}

impl DataAiSearchInstances_Result_PublicEndpointParams_RateLimit { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_PublicEndpointParams_RateLimit {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_PublicEndpointParams_RateLimit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_PublicEndpointParams_RateLimit {}

impl BuildDataAiSearchInstances_Result_PublicEndpointParams_RateLimit {
    pub fn build(self) -> DataAiSearchInstances_Result_PublicEndpointParams_RateLimit {
        DataAiSearchInstances_Result_PublicEndpointParams_RateLimit {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_PublicEndpointParams_RateLimitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_PublicEndpointParams_RateLimitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchInstances_Result_PublicEndpointParams_RateLimitRef {
        DataAiSearchInstances_Result_PublicEndpointParams_RateLimitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_PublicEndpointParams_RateLimitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `period_ms` after provisioning.\n"]
    pub fn period_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `requests` after provisioning.\n"]
    pub fn requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requests", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technique` after provisioning.\nAvailable values: \"fixed\", \"sliding\"."]
    pub fn technique(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technique", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_PublicEndpointParams_SearchEndpoint {}

impl DataAiSearchInstances_Result_PublicEndpointParams_SearchEndpoint { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_PublicEndpointParams_SearchEndpoint {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_PublicEndpointParams_SearchEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_PublicEndpointParams_SearchEndpoint {}

impl BuildDataAiSearchInstances_Result_PublicEndpointParams_SearchEndpoint {
    pub fn build(self) -> DataAiSearchInstances_Result_PublicEndpointParams_SearchEndpoint {
        DataAiSearchInstances_Result_PublicEndpointParams_SearchEndpoint {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_PublicEndpointParams_SearchEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_PublicEndpointParams_SearchEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchInstances_Result_PublicEndpointParams_SearchEndpointRef {
        DataAiSearchInstances_Result_PublicEndpointParams_SearchEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_PublicEndpointParams_SearchEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nDisable search endpoint for this public endpoint"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_PublicEndpointParams {}

impl DataAiSearchInstances_Result_PublicEndpointParams { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_PublicEndpointParams {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_PublicEndpointParams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_PublicEndpointParams {}

impl BuildDataAiSearchInstances_Result_PublicEndpointParams {
    pub fn build(self) -> DataAiSearchInstances_Result_PublicEndpointParams {
        DataAiSearchInstances_Result_PublicEndpointParams {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_PublicEndpointParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_PublicEndpointParamsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchInstances_Result_PublicEndpointParamsRef {
        DataAiSearchInstances_Result_PublicEndpointParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_PublicEndpointParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorized_hosts` after provisioning.\n"]
    pub fn authorized_hosts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorized_hosts", self.base))
    }

    #[doc = "Get a reference to the value of field `chat_completions_endpoint` after provisioning.\n"]
    pub fn chat_completions_endpoint(
        &self,
    ) -> DataAiSearchInstances_Result_PublicEndpointParams_ChatCompletionsEndpointRef {
        DataAiSearchInstances_Result_PublicEndpointParams_ChatCompletionsEndpointRef::new(
            self.shared().clone(),
            format!("{}.chat_completions_endpoint", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `custom_domains` after provisioning.\nCustom domain hostnames that alias this public endpoint. GET and create responses return the current set; on update (PUT) this field is only echoed back when supplied in the request body, otherwise it is null (omit it to leave domains unchanged)."]
    pub fn custom_domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_domains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_domain_enabled` after provisioning.\nWhen false, the instance is reachable only via a registered custom domain and the default <public_endpoint_id>.search.ai.cloudflare.com host returns 404. Requires at least one custom domain. Defaults to true. public_endpoint_params is replaced wholesale on update, so resend default_domain_enabled on every update to keep the default host off — omitting it resets to true."]
    pub fn default_domain_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_domain_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `mcp` after provisioning.\n"]
    pub fn mcp(&self) -> DataAiSearchInstances_Result_PublicEndpointParams_McpRef {
        DataAiSearchInstances_Result_PublicEndpointParams_McpRef::new(
            self.shared().clone(),
            format!("{}.mcp", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `rate_limit` after provisioning.\n"]
    pub fn rate_limit(&self) -> DataAiSearchInstances_Result_PublicEndpointParams_RateLimitRef {
        DataAiSearchInstances_Result_PublicEndpointParams_RateLimitRef::new(
            self.shared().clone(),
            format!("{}.rate_limit", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `search_endpoint` after provisioning.\n"]
    pub fn search_endpoint(&self) -> DataAiSearchInstances_Result_PublicEndpointParams_SearchEndpointRef {
        DataAiSearchInstances_Result_PublicEndpointParams_SearchEndpointRef::new(
            self.shared().clone(),
            format!("{}.search_endpoint", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_RetrievalOptions_BoostBy {}

impl DataAiSearchInstances_Result_RetrievalOptions_BoostBy { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_RetrievalOptions_BoostBy {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_RetrievalOptions_BoostBy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_RetrievalOptions_BoostBy {}

impl BuildDataAiSearchInstances_Result_RetrievalOptions_BoostBy {
    pub fn build(self) -> DataAiSearchInstances_Result_RetrievalOptions_BoostBy {
        DataAiSearchInstances_Result_RetrievalOptions_BoostBy {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_RetrievalOptions_BoostByRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_RetrievalOptions_BoostByRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchInstances_Result_RetrievalOptions_BoostByRef {
        DataAiSearchInstances_Result_RetrievalOptions_BoostByRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_RetrievalOptions_BoostByRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nBoost direction. 'desc' = higher values rank higher (e.g. newer timestamps). 'asc' = lower values rank higher. 'exists' = boost chunks that have the field. 'not_exists' = boost chunks that lack the field. Optional — defaults to 'asc' for numeric/datetime fields, 'exists' for text/boolean fields.\nAvailable values: \"asc\", \"desc\", \"exists\", \"not_exists\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `field` after provisioning.\nMetadata field name to boost by. Use 'timestamp' for document freshness, or any custom_metadata field. Numeric and datetime fields support all four directions (asc, desc, exists, not_exists); text/boolean fields only support exists/not_exists."]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_RetrievalOptions {}

impl DataAiSearchInstances_Result_RetrievalOptions { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_RetrievalOptions {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_RetrievalOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_RetrievalOptions {}

impl BuildDataAiSearchInstances_Result_RetrievalOptions {
    pub fn build(self) -> DataAiSearchInstances_Result_RetrievalOptions {
        DataAiSearchInstances_Result_RetrievalOptions {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_RetrievalOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_RetrievalOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchInstances_Result_RetrievalOptionsRef {
        DataAiSearchInstances_Result_RetrievalOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_RetrievalOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boost_by` after provisioning.\nMetadata fields to boost search results by. Each entry specifies a metadata field and an optional direction. Direction defaults to 'asc' for numeric/datetime fields and 'exists' for text/boolean fields. Fields must match 'timestamp' or a defined custom_metadata field."]
    pub fn boost_by(&self) -> terrars::ListRef<DataAiSearchInstances_Result_RetrievalOptions_BoostByRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boost_by", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keyword_match_mode` after provisioning.\nControls which documents are candidates for BM25 scoring. 'and' restricts candidates to documents containing all query terms; 'or' includes any document containing at least one term, ranked by BM25 relevance. Defaults to 'and'.\nAvailable values: \"and\", \"or\"."]
    pub fn keyword_match_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keyword_match_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_SourceParams_WebCrawler_DiscoverOptions {}

impl DataAiSearchInstances_Result_SourceParams_WebCrawler_DiscoverOptions { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_SourceParams_WebCrawler_DiscoverOptions {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_SourceParams_WebCrawler_DiscoverOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_SourceParams_WebCrawler_DiscoverOptions {}

impl BuildDataAiSearchInstances_Result_SourceParams_WebCrawler_DiscoverOptions {
    pub fn build(self) -> DataAiSearchInstances_Result_SourceParams_WebCrawler_DiscoverOptions {
        DataAiSearchInstances_Result_SourceParams_WebCrawler_DiscoverOptions {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_SourceParams_WebCrawler_DiscoverOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_SourceParams_WebCrawler_DiscoverOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchInstances_Result_SourceParams_WebCrawler_DiscoverOptionsRef {
        DataAiSearchInstances_Result_SourceParams_WebCrawler_DiscoverOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_SourceParams_WebCrawler_DiscoverOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `depth` after provisioning.\n"]
    pub fn depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.depth", self.base))
    }

    #[doc = "Get a reference to the value of field `include_external_links` after provisioning.\n"]
    pub fn include_external_links(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_external_links", self.base))
    }

    #[doc = "Get a reference to the value of field `include_subdomains` after provisioning.\n"]
    pub fn include_subdomains(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_subdomains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `limit` after provisioning.\nMaximum number of pages to crawl. New values are capped at 100000; instances configured before that cap may report a higher stored value, which the crawler clamps at run time."]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nAvailable values: \"all\", \"sitemaps\", \"links\"."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions_ContentSelector {}

impl DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions_ContentSelector { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions_ContentSelector {
    type O =
        terrars::BlockAssignable<DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions_ContentSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions_ContentSelector {}

impl BuildDataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions_ContentSelector {
    pub fn build(self) -> DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions_ContentSelector {
        DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions_ContentSelector {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions_ContentSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions_ContentSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions_ContentSelectorRef {
        DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions_ContentSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions_ContentSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nGlob pattern to match against the page URL path. Uses standard glob syntax: * matches within a segment, ** crosses directories."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `selector` after provisioning.\nCSS selector to extract content from pages matching the path pattern. Must not contain disallowed characters (;, `, $, {, }, \\). Must target a single element; if multiple elements match, the selector is ignored and the full page is used."]
    pub fn selector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.selector", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions {}

impl DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions {}

impl BuildDataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions {
    pub fn build(self) -> DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions {
        DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptionsRef {
        DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content_selector` after provisioning.\nList of path-to-selector mappings for extracting specific content from crawled pages. Each entry pairs a URL glob pattern with a CSS selector. The first matching path wins. Only the matched HTML fragment is stored and indexed. Omit the field to disable content selection — empty arrays are rejected."]
    pub fn content_selector(
        &self,
    ) -> terrars::ListRef<DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptions_ContentSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_selector", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_headers` after provisioning.\nUp to 5 custom HTTP headers sent with each crawl request. Names must be RFC-7230 token characters (no spaces, colons, or control characters); values must be HTAB + printable ASCII (no CR/LF)."]
    pub fn include_headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.include_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `include_images` after provisioning.\n"]
    pub fn include_images(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_images", self.base))
    }

    #[doc =
        "Get a reference to the value of field `specific_sitemaps` after provisioning.\nList of specific sitemap URLs to use for crawling. Only valid when parse_type is 'sitemap'."]
    pub fn specific_sitemaps(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specific_sitemaps", self.base))
    }

    #[doc = "Get a reference to the value of field `use_browser_rendering` after provisioning.\n"]
    pub fn use_browser_rendering(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_browser_rendering", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_SourceParams_WebCrawler_StoreOptions {}

impl DataAiSearchInstances_Result_SourceParams_WebCrawler_StoreOptions { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_SourceParams_WebCrawler_StoreOptions {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_SourceParams_WebCrawler_StoreOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_SourceParams_WebCrawler_StoreOptions {}

impl BuildDataAiSearchInstances_Result_SourceParams_WebCrawler_StoreOptions {
    pub fn build(self) -> DataAiSearchInstances_Result_SourceParams_WebCrawler_StoreOptions {
        DataAiSearchInstances_Result_SourceParams_WebCrawler_StoreOptions {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_SourceParams_WebCrawler_StoreOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_SourceParams_WebCrawler_StoreOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchInstances_Result_SourceParams_WebCrawler_StoreOptionsRef {
        DataAiSearchInstances_Result_SourceParams_WebCrawler_StoreOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_SourceParams_WebCrawler_StoreOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `r2_jurisdiction` after provisioning.\n"]
    pub fn r2_jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.r2_jurisdiction", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_id` after provisioning.\n"]
    pub fn storage_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_id", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\nAvailable values: \"r2\"."]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_SourceParams_WebCrawler {}

impl DataAiSearchInstances_Result_SourceParams_WebCrawler { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_SourceParams_WebCrawler {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_SourceParams_WebCrawler>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_SourceParams_WebCrawler {}

impl BuildDataAiSearchInstances_Result_SourceParams_WebCrawler {
    pub fn build(self) -> DataAiSearchInstances_Result_SourceParams_WebCrawler {
        DataAiSearchInstances_Result_SourceParams_WebCrawler {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_SourceParams_WebCrawlerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_SourceParams_WebCrawlerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchInstances_Result_SourceParams_WebCrawlerRef {
        DataAiSearchInstances_Result_SourceParams_WebCrawlerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_SourceParams_WebCrawlerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `discover_options` after provisioning.\n"]
    pub fn discover_options(&self) -> DataAiSearchInstances_Result_SourceParams_WebCrawler_DiscoverOptionsRef {
        DataAiSearchInstances_Result_SourceParams_WebCrawler_DiscoverOptionsRef::new(
            self.shared().clone(),
            format!("{}.discover_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `parse_options` after provisioning.\n"]
    pub fn parse_options(&self) -> DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptionsRef {
        DataAiSearchInstances_Result_SourceParams_WebCrawler_ParseOptionsRef::new(
            self.shared().clone(),
            format!("{}.parse_options", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `parse_type` after provisioning.\nAvailable values: \"sitemap\", \"feed-rss\", \"crawl\"."]
    pub fn parse_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parse_type", self.base))
    }

    #[doc = "Get a reference to the value of field `store_options` after provisioning.\n"]
    pub fn store_options(&self) -> DataAiSearchInstances_Result_SourceParams_WebCrawler_StoreOptionsRef {
        DataAiSearchInstances_Result_SourceParams_WebCrawler_StoreOptionsRef::new(
            self.shared().clone(),
            format!("{}.store_options", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result_SourceParams {}

impl DataAiSearchInstances_Result_SourceParams { }

impl terrars::ToListMappable for DataAiSearchInstances_Result_SourceParams {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result_SourceParams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result_SourceParams {}

impl BuildDataAiSearchInstances_Result_SourceParams {
    pub fn build(self) -> DataAiSearchInstances_Result_SourceParams {
        DataAiSearchInstances_Result_SourceParams {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_Result_SourceParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_Result_SourceParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiSearchInstances_Result_SourceParamsRef {
        DataAiSearchInstances_Result_SourceParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_Result_SourceParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude_items` after provisioning.\nList of path patterns to exclude. Uses micromatch glob syntax: * matches within a path segment, ** matches across path segments (e.g., /admin/** matches /admin/users and /admin/settings/advanced)"]
    pub fn exclude_items(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_items", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_items` after provisioning.\nList of path patterns to include. Uses micromatch glob syntax: * matches within a path segment, ** matches across path segments (e.g., /blog/** matches /blog/post and /blog/2024/post)"]
    pub fn include_items(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_items", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `r2_jurisdiction` after provisioning.\n"]
    pub fn r2_jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.r2_jurisdiction", self.base))
    }

    #[doc = "Get a reference to the value of field `web_crawler` after provisioning.\n"]
    pub fn web_crawler(&self) -> DataAiSearchInstances_Result_SourceParams_WebCrawlerRef {
        DataAiSearchInstances_Result_SourceParams_WebCrawlerRef::new(
            self.shared().clone(),
            format!("{}.web_crawler", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataAiSearchInstances_Result {}

impl DataAiSearchInstances_Result { }

impl terrars::ToListMappable for DataAiSearchInstances_Result {
    type O = terrars::BlockAssignable<DataAiSearchInstances_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchInstances_Result {}

impl BuildDataAiSearchInstances_Result {
    pub fn build(self) -> DataAiSearchInstances_Result {
        DataAiSearchInstances_Result {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchInstances_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchInstances_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiSearchInstances_ResultRef {
        DataAiSearchInstances_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchInstances_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ai_gateway_id` after provisioning.\n"]
    pub fn ai_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_gateway_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `aisearch_model` after provisioning.\nAvailable values: \"@cf/meta/llama-3.3-70b-instruct-fp8-fast\", \"@cf/zai-org/glm-4.7-flash\", \"@cf/meta/llama-3.1-8b-instruct-fast\", \"@cf/meta/llama-3.1-8b-instruct-fp8\", \"@cf/meta/llama-4-scout-17b-16e-instruct\", \"@cf/qwen/qwen3-30b-a3b-fp8\", \"@cf/deepseek-ai/deepseek-r1-distill-qwen-32b\", \"@cf/moonshotai/kimi-k2-instruct\", \"@cf/google/gemma-3-12b-it\", \"@cf/google/gemma-4-26b-a4b-it\", \"@cf/moonshotai/kimi-k2.5\", \"anthropic/claude-3-7-sonnet\", \"anthropic/claude-sonnet-4\", \"anthropic/claude-opus-4\", \"anthropic/claude-3-5-haiku\", \"cerebras/qwen-3-235b-a22b-instruct\", \"cerebras/qwen-3-235b-a22b-thinking\", \"cerebras/llama-3.3-70b\", \"cerebras/llama-4-maverick-17b-128e-instruct\", \"cerebras/llama-4-scout-17b-16e-instruct\", \"cerebras/gpt-oss-120b\", \"google-ai-studio/gemini-2.5-flash\", \"google-ai-studio/gemini-2.5-pro\", \"grok/grok-4\", \"groq/llama-3.3-70b-versatile\", \"groq/llama-3.1-8b-instant\", \"openai/gpt-5\", \"openai/gpt-5-mini\", \"openai/gpt-5-nano\", \"\"."]
    pub fn aisearch_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aisearch_model", self.base))
    }

    #[doc = "Get a reference to the value of field `cache` after provisioning.\n"]
    pub fn cache(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cache_threshold` after provisioning.\nAvailable values: \"super_strict_match\", \"close_enough\", \"flexible_friend\", \"anything_goes\"."]
    pub fn cache_threshold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cache_ttl` after provisioning.\nCache entry TTL in seconds. Allowed values: 600 (10min), 1800 (30min), 3600 (1h), 7200 (2h), 21600 (6h), 43200 (12h), 86400 (24h), 172800 (48h), 259200 (72h), 518400 (6d).\nAvailable values: 600, 1800, 3600, 7200, 21600, 43200, 86400, 172800, 259200, 518400."]
    pub fn cache_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `chunk_overlap` after provisioning.\n"]
    pub fn chunk_overlap(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chunk_overlap", self.base))
    }

    #[doc = "Get a reference to the value of field `chunk_size` after provisioning.\n"]
    pub fn chunk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chunk_size", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_metadata` after provisioning.\n"]
    pub fn custom_metadata(&self) -> terrars::ListRef<DataAiSearchInstances_Result_CustomMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_metadata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `embedding_model` after provisioning.\nAvailable values: \"@cf/qwen/qwen3-embedding-0.6b\", \"@cf/baai/bge-m3\", \"@cf/baai/bge-large-en-v1.5\", \"@cf/google/embeddinggemma-300m\", \"google-ai-studio/gemini-embedding-001\", \"google-ai-studio/gemini-embedding-2-preview\", \"google-ai-studio/gemini-embedding-2\", \"openai/text-embedding-3-small\", \"openai/text-embedding-3-large\", \"\"."]
    pub fn embedding_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embedding_model", self.base))
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fusion_method` after provisioning.\nAvailable values: \"max\", \"rrf\"."]
    pub fn fusion_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fusion_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hybrid_search_enabled` after provisioning.\nDeprecated — use index_method instead."]
    pub fn hybrid_search_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hybrid_search_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nAI Search instance ID. Lowercase alphanumeric, hyphens, and underscores."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `index_method` after provisioning.\nControls which storage backends are used during indexing. Defaults to vector-only."]
    pub fn index_method(&self) -> DataAiSearchInstances_Result_IndexMethodRef {
        DataAiSearchInstances_Result_IndexMethodRef::new(
            self.shared().clone(),
            format!("{}.index_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `indexing_options` after provisioning.\n"]
    pub fn indexing_options(&self) -> DataAiSearchInstances_Result_IndexingOptionsRef {
        DataAiSearchInstances_Result_IndexingOptionsRef::new(
            self.shared().clone(),
            format!("{}.indexing_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `last_activity` after provisioning.\n"]
    pub fn last_activity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_activity", self.base))
    }

    #[doc = "Get a reference to the value of field `max_num_results` after provisioning.\n"]
    pub fn max_num_results(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_num_results", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> DataAiSearchInstances_Result_MetadataRef {
        DataAiSearchInstances_Result_MetadataRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.base))
    }

    #[doc = "Get a reference to the value of field `modified_by` after provisioning.\n"]
    pub fn modified_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_by", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `paused` after provisioning.\n"]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.base))
    }

    #[doc = "Get a reference to the value of field `public_endpoint_id` after provisioning.\n"]
    pub fn public_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `public_endpoint_params` after provisioning.\n"]
    pub fn public_endpoint_params(&self) -> DataAiSearchInstances_Result_PublicEndpointParamsRef {
        DataAiSearchInstances_Result_PublicEndpointParamsRef::new(
            self.shared().clone(),
            format!("{}.public_endpoint_params", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `reranking` after provisioning.\n"]
    pub fn reranking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reranking", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reranking_model` after provisioning.\nAvailable values: \"@cf/baai/bge-reranker-base\", \"\"."]
    pub fn reranking_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reranking_model", self.base))
    }

    #[doc = "Get a reference to the value of field `retrieval_options` after provisioning.\n"]
    pub fn retrieval_options(&self) -> DataAiSearchInstances_Result_RetrievalOptionsRef {
        DataAiSearchInstances_Result_RetrievalOptionsRef::new(
            self.shared().clone(),
            format!("{}.retrieval_options", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `rewrite_model` after provisioning.\nAvailable values: \"@cf/meta/llama-3.3-70b-instruct-fp8-fast\", \"@cf/zai-org/glm-4.7-flash\", \"@cf/meta/llama-3.1-8b-instruct-fast\", \"@cf/meta/llama-3.1-8b-instruct-fp8\", \"@cf/meta/llama-4-scout-17b-16e-instruct\", \"@cf/qwen/qwen3-30b-a3b-fp8\", \"@cf/deepseek-ai/deepseek-r1-distill-qwen-32b\", \"@cf/moonshotai/kimi-k2-instruct\", \"@cf/google/gemma-3-12b-it\", \"@cf/google/gemma-4-26b-a4b-it\", \"@cf/moonshotai/kimi-k2.5\", \"anthropic/claude-3-7-sonnet\", \"anthropic/claude-sonnet-4\", \"anthropic/claude-opus-4\", \"anthropic/claude-3-5-haiku\", \"cerebras/qwen-3-235b-a22b-instruct\", \"cerebras/qwen-3-235b-a22b-thinking\", \"cerebras/llama-3.3-70b\", \"cerebras/llama-4-maverick-17b-128e-instruct\", \"cerebras/llama-4-scout-17b-16e-instruct\", \"cerebras/gpt-oss-120b\", \"google-ai-studio/gemini-2.5-flash\", \"google-ai-studio/gemini-2.5-pro\", \"grok/grok-4\", \"groq/llama-3.3-70b-versatile\", \"groq/llama-3.1-8b-instant\", \"openai/gpt-5\", \"openai/gpt-5-mini\", \"openai/gpt-5-nano\", \"\"."]
    pub fn rewrite_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rewrite_model", self.base))
    }

    #[doc = "Get a reference to the value of field `rewrite_query` after provisioning.\n"]
    pub fn rewrite_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rewrite_query", self.base))
    }

    #[doc = "Get a reference to the value of field `score_threshold` after provisioning.\n"]
    pub fn score_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `source_params` after provisioning.\n"]
    pub fn source_params(&self) -> DataAiSearchInstances_Result_SourceParamsRef {
        DataAiSearchInstances_Result_SourceParamsRef::new(
            self.shared().clone(),
            format!("{}.source_params", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sync_interval` after provisioning.\nInterval between automatic syncs, in seconds. Allowed values: 900 (15min), 1800 (30min), 3600 (1h), 7200 (2h), 14400 (4h), 21600 (6h), 43200 (12h), 86400 (24h).\nAvailable values: 900, 1800, 3600, 7200, 14400, 21600, 43200, 86400."]
    pub fn sync_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `token_id` after provisioning.\n"]
    pub fn token_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"r2\", \"web-crawler\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
