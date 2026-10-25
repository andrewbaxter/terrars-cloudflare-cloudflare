#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AiSearchInstanceData_ {
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
    ai_gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aisearch_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_threshold: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chunk: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chunk_overlap: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chunk_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_metadata: std::option::Option<std::vec::Vec<AiSearchInstance_CustomMetadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embedding_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fusion_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hybrid_search_enabled: std::option::Option<terrars::PrimField<bool>>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_method: std::option::Option<AiSearchInstance_IndexMethod>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indexing_options: std::option::Option<AiSearchInstance_IndexingOptions>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_num_results: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<AiSearchInstance_Metadata>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paused: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_endpoint_params: std::option::Option<AiSearchInstance_PublicEndpointParams>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reranking: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reranking_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retrieval_options: std::option::Option<AiSearchInstance_RetrievalOptions>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewrite_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewrite_query: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    score_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_params: std::option::Option<AiSearchInstance_SourceParams>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summarization: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summarization_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_prompt_aisearch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_prompt_index_summarization: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_prompt_rewrite_query: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct AiSearchInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AiSearchInstanceData_>,
}

#[derive(Clone)]
pub struct AiSearchInstance(std::rc::Rc<AiSearchInstance_>);

impl AiSearchInstance {
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

    #[doc = "Set the field `ai_gateway_id`.\n"]
    pub fn set_ai_gateway_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ai_gateway_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `aisearch_model`.\nA Workers AI model ID or an AI Gateway model ID compatible with the OpenAI Chat Completions API. An empty string uses the configured or default model."]
    pub fn set_aisearch_model(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().aisearch_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache`.\n"]
    pub fn set_cache(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cache = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cache_threshold`.\nAvailable values: \"super_strict_match\", \"close_enough\", \"flexible_friend\", \"anything_goes\"."]
    pub fn set_cache_threshold(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cache_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cache_ttl`.\nCache entry TTL in seconds. Allowed values: 600 (10min), 1800 (30min), 3600 (1h), 7200 (2h), 21600 (6h), 43200 (12h), 86400 (24h), 172800 (48h), 259200 (72h), 518400 (6d).\nAvailable values: 600, 1800, 3600, 7200, 21600, 43200, 86400, 172800, 259200, 518400."]
    pub fn set_cache_ttl(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cache_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chunk`.\n"]
    pub fn set_chunk(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().chunk = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chunk_overlap`.\n"]
    pub fn set_chunk_overlap(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().chunk_overlap = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chunk_size`.\n"]
    pub fn set_chunk_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().chunk_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_metadata`.\n"]
    pub fn set_custom_metadata(
        self,
        v: impl std::convert::Into<std::vec::Vec<AiSearchInstance_CustomMetadata>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `embedding_model`.\n"]
    pub fn set_embedding_model(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().embedding_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fusion_method`.\nAvailable values: \"max\", \"rrf\"."]
    pub fn set_fusion_method(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().fusion_method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `hybrid_search_enabled`.\nDeprecated — use index_method instead. Defaults to true for new instances; set false to create a vector-only instance."]
    pub fn set_hybrid_search_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().hybrid_search_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `index_method`.\nControls which storage backends are used during indexing. Defaults to vector and keyword indexing for new instances."]
    pub fn set_index_method(self, v: impl std::convert::Into<AiSearchInstance_IndexMethod>) -> Self {
        self.0.data.borrow_mut().index_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `indexing_options`.\n"]
    pub fn set_indexing_options(self, v: impl std::convert::Into<AiSearchInstance_IndexingOptions>) -> Self {
        self.0.data.borrow_mut().indexing_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_num_results`.\n"]
    pub fn set_max_num_results(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_num_results = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(self, v: impl std::convert::Into<AiSearchInstance_Metadata>) -> Self {
        self.0.data.borrow_mut().metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `paused`.\n"]
    pub fn set_paused(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().paused = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_endpoint_params`.\n"]
    pub fn set_public_endpoint_params(self, v: impl std::convert::Into<AiSearchInstance_PublicEndpointParams>) -> Self {
        self.0.data.borrow_mut().public_endpoint_params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reranking`.\n"]
    pub fn set_reranking(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().reranking = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reranking_model`.\n"]
    pub fn set_reranking_model(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().reranking_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retrieval_options`.\n"]
    pub fn set_retrieval_options(self, v: impl std::convert::Into<AiSearchInstance_RetrievalOptions>) -> Self {
        self.0.data.borrow_mut().retrieval_options = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rewrite_model`.\nA Workers AI model ID or an AI Gateway model ID compatible with the OpenAI Chat Completions API. An empty string uses the configured or default model."]
    pub fn set_rewrite_model(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().rewrite_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rewrite_query`.\n"]
    pub fn set_rewrite_query(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().rewrite_query = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `score_threshold`.\n"]
    pub fn set_score_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().score_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_params`.\n"]
    pub fn set_source_params(self, v: impl std::convert::Into<AiSearchInstance_SourceParams>) -> Self {
        self.0.data.borrow_mut().source_params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `summarization`.\n"]
    pub fn set_summarization(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().summarization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `summarization_model`.\n"]
    pub fn set_summarization_model(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().summarization_model = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sync_interval`.\nInterval between automatic syncs, in seconds. Allowed values: 900 (15min), 1800 (30min), 3600 (1h), 7200 (2h), 14400 (4h), 21600 (6h), 43200 (12h), 86400 (24h).\nAvailable values: 900, 1800, 3600, 7200, 14400, 21600, 43200, 86400."]
    pub fn set_sync_interval(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().sync_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `system_prompt_aisearch`.\n"]
    pub fn set_system_prompt_aisearch(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().system_prompt_aisearch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `system_prompt_index_summarization`.\n"]
    pub fn set_system_prompt_index_summarization(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().system_prompt_index_summarization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `system_prompt_rewrite_query`.\n"]
    pub fn set_system_prompt_rewrite_query(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().system_prompt_rewrite_query = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_id`.\n"]
    pub fn set_token_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().token_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nSource type. When omitted or null with a non-blank source, HTTP(S) URLs infer web-crawler and existing R2 bucket names infer r2. A missing or blank source without a type uses managed upload-only storage.\nAvailable values: \"r2\", \"web-crawler\"."]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ai_gateway_id` after provisioning.\n"]
    pub fn ai_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_gateway_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `aisearch_model` after provisioning.\nA Workers AI model ID or an AI Gateway model ID compatible with the OpenAI Chat Completions API. An empty string uses the configured or default model."]
    pub fn aisearch_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aisearch_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache` after provisioning.\n"]
    pub fn cache(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cache_threshold` after provisioning.\nAvailable values: \"super_strict_match\", \"close_enough\", \"flexible_friend\", \"anything_goes\"."]
    pub fn cache_threshold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_threshold", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cache_ttl` after provisioning.\nCache entry TTL in seconds. Allowed values: 600 (10min), 1800 (30min), 3600 (1h), 7200 (2h), 21600 (6h), 43200 (12h), 86400 (24h), 172800 (48h), 259200 (72h), 518400 (6d).\nAvailable values: 600, 1800, 3600, 7200, 21600, 43200, 86400, 172800, 259200, 518400."]
    pub fn cache_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chunk` after provisioning.\n"]
    pub fn chunk(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chunk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chunk_overlap` after provisioning.\n"]
    pub fn chunk_overlap(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chunk_overlap", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chunk_size` after provisioning.\n"]
    pub fn chunk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chunk_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_metadata` after provisioning.\n"]
    pub fn custom_metadata(&self) -> terrars::ListRef<AiSearchInstance_CustomMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `embedding_model` after provisioning.\n"]
    pub fn embedding_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embedding_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fusion_method` after provisioning.\nAvailable values: \"max\", \"rrf\"."]
    pub fn fusion_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fusion_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hybrid_search_enabled` after provisioning.\nDeprecated — use index_method instead. Defaults to true for new instances; set false to create a vector-only instance."]
    pub fn hybrid_search_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hybrid_search_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nAI Search instance ID. Lowercase alphanumeric, hyphens, and underscores."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `index_method` after provisioning.\nControls which storage backends are used during indexing. Defaults to vector and keyword indexing for new instances."]
    pub fn index_method(&self) -> AiSearchInstance_IndexMethodRef {
        AiSearchInstance_IndexMethodRef::new(self.shared().clone(), format!("{}.index_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `indexing_options` after provisioning.\n"]
    pub fn indexing_options(&self) -> AiSearchInstance_IndexingOptionsRef {
        AiSearchInstance_IndexingOptionsRef::new(
            self.shared().clone(),
            format!("{}.indexing_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_activity` after provisioning.\n"]
    pub fn last_activity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_activity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_num_results` after provisioning.\n"]
    pub fn max_num_results(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_num_results", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> AiSearchInstance_MetadataRef {
        AiSearchInstance_MetadataRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_by` after provisioning.\n"]
    pub fn modified_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `paused` after provisioning.\n"]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_endpoint_id` after provisioning.\n"]
    pub fn public_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_endpoint_params` after provisioning.\n"]
    pub fn public_endpoint_params(&self) -> AiSearchInstance_PublicEndpointParamsRef {
        AiSearchInstance_PublicEndpointParamsRef::new(
            self.shared().clone(),
            format!("{}.public_endpoint_params", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `reranking` after provisioning.\n"]
    pub fn reranking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reranking", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reranking_model` after provisioning.\n"]
    pub fn reranking_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reranking_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retrieval_options` after provisioning.\n"]
    pub fn retrieval_options(&self) -> AiSearchInstance_RetrievalOptionsRef {
        AiSearchInstance_RetrievalOptionsRef::new(
            self.shared().clone(),
            format!("{}.retrieval_options", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `rewrite_model` after provisioning.\nA Workers AI model ID or an AI Gateway model ID compatible with the OpenAI Chat Completions API. An empty string uses the configured or default model."]
    pub fn rewrite_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rewrite_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rewrite_query` after provisioning.\n"]
    pub fn rewrite_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rewrite_query", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `score_threshold` after provisioning.\n"]
    pub fn score_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_params` after provisioning.\n"]
    pub fn source_params(&self) -> AiSearchInstance_SourceParamsRef {
        AiSearchInstance_SourceParamsRef::new(self.shared().clone(), format!("{}.source_params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `summarization` after provisioning.\n"]
    pub fn summarization(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.summarization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `summarization_model` after provisioning.\n"]
    pub fn summarization_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.summarization_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sync_interval` after provisioning.\nInterval between automatic syncs, in seconds. Allowed values: 900 (15min), 1800 (30min), 3600 (1h), 7200 (2h), 14400 (4h), 21600 (6h), 43200 (12h), 86400 (24h).\nAvailable values: 900, 1800, 3600, 7200, 14400, 21600, 43200, 86400."]
    pub fn sync_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `system_prompt_aisearch` after provisioning.\n"]
    pub fn system_prompt_aisearch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_prompt_aisearch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `system_prompt_index_summarization` after provisioning.\n"]
    pub fn system_prompt_index_summarization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.system_prompt_index_summarization", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `system_prompt_rewrite_query` after provisioning.\n"]
    pub fn system_prompt_rewrite_query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_prompt_rewrite_query", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_id` after provisioning.\n"]
    pub fn token_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nSource type. When omitted or null with a non-blank source, HTTP(S) URLs infer web-crawler and existing R2 bucket names infer r2. A missing or blank source without a type uses managed upload-only storage.\nAvailable values: \"r2\", \"web-crawler\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for AiSearchInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AiSearchInstance { }

impl terrars::ToListMappable for AiSearchInstance {
    type O = terrars::ListRef<AiSearchInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AiSearchInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_ai_search_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAiSearchInstance {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "AI Search instance ID. Lowercase alphanumeric, hyphens, and underscores."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildAiSearchInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> AiSearchInstance {
        let out = AiSearchInstance(std::rc::Rc::new(AiSearchInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AiSearchInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                ai_gateway_id: core::default::Default::default(),
                aisearch_model: core::default::Default::default(),
                cache: core::default::Default::default(),
                cache_threshold: core::default::Default::default(),
                cache_ttl: core::default::Default::default(),
                chunk: core::default::Default::default(),
                chunk_overlap: core::default::Default::default(),
                chunk_size: core::default::Default::default(),
                custom_metadata: core::default::Default::default(),
                embedding_model: core::default::Default::default(),
                fusion_method: core::default::Default::default(),
                hybrid_search_enabled: core::default::Default::default(),
                id: self.id,
                index_method: core::default::Default::default(),
                indexing_options: core::default::Default::default(),
                max_num_results: core::default::Default::default(),
                metadata: core::default::Default::default(),
                paused: core::default::Default::default(),
                public_endpoint_params: core::default::Default::default(),
                reranking: core::default::Default::default(),
                reranking_model: core::default::Default::default(),
                retrieval_options: core::default::Default::default(),
                rewrite_model: core::default::Default::default(),
                rewrite_query: core::default::Default::default(),
                score_threshold: core::default::Default::default(),
                source: core::default::Default::default(),
                source_params: core::default::Default::default(),
                summarization: core::default::Default::default(),
                summarization_model: core::default::Default::default(),
                sync_interval: core::default::Default::default(),
                system_prompt_aisearch: core::default::Default::default(),
                system_prompt_index_summarization: core::default::Default::default(),
                system_prompt_rewrite_query: core::default::Default::default(),
                token_id: core::default::Default::default(),
                type_: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AiSearchInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AiSearchInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ai_gateway_id` after provisioning.\n"]
    pub fn ai_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_gateway_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `aisearch_model` after provisioning.\nA Workers AI model ID or an AI Gateway model ID compatible with the OpenAI Chat Completions API. An empty string uses the configured or default model."]
    pub fn aisearch_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aisearch_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache` after provisioning.\n"]
    pub fn cache(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cache_threshold` after provisioning.\nAvailable values: \"super_strict_match\", \"close_enough\", \"flexible_friend\", \"anything_goes\"."]
    pub fn cache_threshold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_threshold", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cache_ttl` after provisioning.\nCache entry TTL in seconds. Allowed values: 600 (10min), 1800 (30min), 3600 (1h), 7200 (2h), 21600 (6h), 43200 (12h), 86400 (24h), 172800 (48h), 259200 (72h), 518400 (6d).\nAvailable values: 600, 1800, 3600, 7200, 21600, 43200, 86400, 172800, 259200, 518400."]
    pub fn cache_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chunk` after provisioning.\n"]
    pub fn chunk(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chunk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chunk_overlap` after provisioning.\n"]
    pub fn chunk_overlap(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chunk_overlap", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chunk_size` after provisioning.\n"]
    pub fn chunk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chunk_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_metadata` after provisioning.\n"]
    pub fn custom_metadata(&self) -> terrars::ListRef<AiSearchInstance_CustomMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `embedding_model` after provisioning.\n"]
    pub fn embedding_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embedding_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fusion_method` after provisioning.\nAvailable values: \"max\", \"rrf\"."]
    pub fn fusion_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fusion_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hybrid_search_enabled` after provisioning.\nDeprecated — use index_method instead. Defaults to true for new instances; set false to create a vector-only instance."]
    pub fn hybrid_search_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hybrid_search_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nAI Search instance ID. Lowercase alphanumeric, hyphens, and underscores."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `index_method` after provisioning.\nControls which storage backends are used during indexing. Defaults to vector and keyword indexing for new instances."]
    pub fn index_method(&self) -> AiSearchInstance_IndexMethodRef {
        AiSearchInstance_IndexMethodRef::new(self.shared().clone(), format!("{}.index_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `indexing_options` after provisioning.\n"]
    pub fn indexing_options(&self) -> AiSearchInstance_IndexingOptionsRef {
        AiSearchInstance_IndexingOptionsRef::new(
            self.shared().clone(),
            format!("{}.indexing_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_activity` after provisioning.\n"]
    pub fn last_activity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_activity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_num_results` after provisioning.\n"]
    pub fn max_num_results(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_num_results", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> AiSearchInstance_MetadataRef {
        AiSearchInstance_MetadataRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_by` after provisioning.\n"]
    pub fn modified_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `paused` after provisioning.\n"]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_endpoint_id` after provisioning.\n"]
    pub fn public_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_endpoint_params` after provisioning.\n"]
    pub fn public_endpoint_params(&self) -> AiSearchInstance_PublicEndpointParamsRef {
        AiSearchInstance_PublicEndpointParamsRef::new(
            self.shared().clone(),
            format!("{}.public_endpoint_params", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `reranking` after provisioning.\n"]
    pub fn reranking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reranking", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reranking_model` after provisioning.\n"]
    pub fn reranking_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reranking_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retrieval_options` after provisioning.\n"]
    pub fn retrieval_options(&self) -> AiSearchInstance_RetrievalOptionsRef {
        AiSearchInstance_RetrievalOptionsRef::new(
            self.shared().clone(),
            format!("{}.retrieval_options", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `rewrite_model` after provisioning.\nA Workers AI model ID or an AI Gateway model ID compatible with the OpenAI Chat Completions API. An empty string uses the configured or default model."]
    pub fn rewrite_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rewrite_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rewrite_query` after provisioning.\n"]
    pub fn rewrite_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rewrite_query", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `score_threshold` after provisioning.\n"]
    pub fn score_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_params` after provisioning.\n"]
    pub fn source_params(&self) -> AiSearchInstance_SourceParamsRef {
        AiSearchInstance_SourceParamsRef::new(self.shared().clone(), format!("{}.source_params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `summarization` after provisioning.\n"]
    pub fn summarization(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.summarization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `summarization_model` after provisioning.\n"]
    pub fn summarization_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.summarization_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sync_interval` after provisioning.\nInterval between automatic syncs, in seconds. Allowed values: 900 (15min), 1800 (30min), 3600 (1h), 7200 (2h), 14400 (4h), 21600 (6h), 43200 (12h), 86400 (24h).\nAvailable values: 900, 1800, 3600, 7200, 14400, 21600, 43200, 86400."]
    pub fn sync_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `system_prompt_aisearch` after provisioning.\n"]
    pub fn system_prompt_aisearch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_prompt_aisearch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `system_prompt_index_summarization` after provisioning.\n"]
    pub fn system_prompt_index_summarization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.system_prompt_index_summarization", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `system_prompt_rewrite_query` after provisioning.\n"]
    pub fn system_prompt_rewrite_query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_prompt_rewrite_query", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_id` after provisioning.\n"]
    pub fn token_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nSource type. When omitted or null with a non-blank source, HTTP(S) URLs infer web-crawler and existing R2 bucket names infer r2. A missing or blank source without a type uses managed upload-only storage.\nAvailable values: \"r2\", \"web-crawler\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AiSearchInstance_CustomMetadata {
    data_type: terrars::PrimField<std::string::String>,
    field_name: terrars::PrimField<std::string::String>,
}

impl AiSearchInstance_CustomMetadata { }

impl terrars::ToListMappable for AiSearchInstance_CustomMetadata {
    type O = terrars::BlockAssignable<AiSearchInstance_CustomMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_CustomMetadata {
    #[doc = "Available values: \"text\", \"number\", \"boolean\", \"datetime\"."]
    pub data_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub field_name: terrars::PrimField<std::string::String>,
}

impl BuildAiSearchInstance_CustomMetadata {
    pub fn build(self) -> AiSearchInstance_CustomMetadata {
        AiSearchInstance_CustomMetadata {
            data_type: self.data_type,
            field_name: self.field_name,
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_CustomMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_CustomMetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiSearchInstance_CustomMetadataRef {
        AiSearchInstance_CustomMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_CustomMetadataRef {
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
pub struct AiSearchInstance_IndexMethod {
    keyword: terrars::PrimField<bool>,
    vector: terrars::PrimField<bool>,
}

impl AiSearchInstance_IndexMethod { }

impl terrars::ToListMappable for AiSearchInstance_IndexMethod {
    type O = terrars::BlockAssignable<AiSearchInstance_IndexMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_IndexMethod {
    #[doc = "Enable keyword (BM25) storage backend."]
    pub keyword: terrars::PrimField<bool>,
    #[doc = "Enable vector (embedding) storage backend."]
    pub vector: terrars::PrimField<bool>,
}

impl BuildAiSearchInstance_IndexMethod {
    pub fn build(self) -> AiSearchInstance_IndexMethod {
        AiSearchInstance_IndexMethod {
            keyword: self.keyword,
            vector: self.vector,
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_IndexMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_IndexMethodRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiSearchInstance_IndexMethodRef {
        AiSearchInstance_IndexMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_IndexMethodRef {
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
pub struct AiSearchInstance_IndexingOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keyword_tokenizer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_ocr: std::option::Option<terrars::PrimField<bool>>,
}

impl AiSearchInstance_IndexingOptions {
    #[doc =
        "Set the field `keyword_tokenizer`.\nTokenizer used for keyword search indexing. porter provides word-level tokenization with Porter stemming (good for natural language queries). trigram enables character-level substring matching (good for partial matches, code, identifiers). Changing this triggers a full re-index. Defaults to porter.\nAvailable values: \"porter\", \"trigram\"."]
    pub fn set_keyword_tokenizer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.keyword_tokenizer = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_ocr`.\nEnables OCR ingestion for PDFs and images. Changing this triggers a full re-index. Defaults to false."]
    pub fn set_use_ocr(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_ocr = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchInstance_IndexingOptions {
    type O = terrars::BlockAssignable<AiSearchInstance_IndexingOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_IndexingOptions {}

impl BuildAiSearchInstance_IndexingOptions {
    pub fn build(self) -> AiSearchInstance_IndexingOptions {
        AiSearchInstance_IndexingOptions {
            keyword_tokenizer: core::default::Default::default(),
            use_ocr: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_IndexingOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_IndexingOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiSearchInstance_IndexingOptionsRef {
        AiSearchInstance_IndexingOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_IndexingOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `keyword_tokenizer` after provisioning.\nTokenizer used for keyword search indexing. porter provides word-level tokenization with Porter stemming (good for natural language queries). trigram enables character-level substring matching (good for partial matches, code, identifiers). Changing this triggers a full re-index. Defaults to porter.\nAvailable values: \"porter\", \"trigram\"."]
    pub fn keyword_tokenizer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keyword_tokenizer", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_ocr` after provisioning.\nEnables OCR ingestion for PDFs and images. Changing this triggers a full re-index. Defaults to false."]
    pub fn use_ocr(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_ocr", self.base))
    }
}

#[derive(Serialize)]
pub struct AiSearchInstance_Metadata_CreatedFromEmdashPlugin {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    version: terrars::PrimField<std::string::String>,
}

impl AiSearchInstance_Metadata_CreatedFromEmdashPlugin { }

impl terrars::ToListMappable for AiSearchInstance_Metadata_CreatedFromEmdashPlugin {
    type O = terrars::BlockAssignable<AiSearchInstance_Metadata_CreatedFromEmdashPlugin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_Metadata_CreatedFromEmdashPlugin {
    #[doc = "Available values: \"native\", \"rest\"."]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildAiSearchInstance_Metadata_CreatedFromEmdashPlugin {
    pub fn build(self) -> AiSearchInstance_Metadata_CreatedFromEmdashPlugin {
        AiSearchInstance_Metadata_CreatedFromEmdashPlugin {
            type_: self.type_,
            version: self.version,
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_Metadata_CreatedFromEmdashPluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_Metadata_CreatedFromEmdashPluginRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiSearchInstance_Metadata_CreatedFromEmdashPluginRef {
        AiSearchInstance_Metadata_CreatedFromEmdashPluginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_Metadata_CreatedFromEmdashPluginRef {
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
pub struct AiSearchInstance_Metadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_from_aisearch_wizard: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_from_emdash_plugin: std::option::Option<AiSearchInstance_Metadata_CreatedFromEmdashPlugin>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_domain: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AiSearchInstance_Metadata {
    #[doc = "Set the field `created_from_aisearch_wizard`.\n"]
    pub fn set_created_from_aisearch_wizard(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.created_from_aisearch_wizard = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_from_emdash_plugin`.\n"]
    pub fn set_created_from_emdash_plugin(
        mut self,
        v: impl std::convert::Into<AiSearchInstance_Metadata_CreatedFromEmdashPlugin>,
    ) -> Self {
        self.created_from_emdash_plugin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `worker_domain`.\n"]
    pub fn set_worker_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.worker_domain = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchInstance_Metadata {
    type O = terrars::BlockAssignable<AiSearchInstance_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_Metadata {}

impl BuildAiSearchInstance_Metadata {
    pub fn build(self) -> AiSearchInstance_Metadata {
        AiSearchInstance_Metadata {
            created_from_aisearch_wizard: core::default::Default::default(),
            created_from_emdash_plugin: core::default::Default::default(),
            worker_domain: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_MetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiSearchInstance_MetadataRef {
        AiSearchInstance_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_from_aisearch_wizard` after provisioning.\n"]
    pub fn created_from_aisearch_wizard(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_from_aisearch_wizard", self.base))
    }

    #[doc = "Get a reference to the value of field `created_from_emdash_plugin` after provisioning.\n"]
    pub fn created_from_emdash_plugin(&self) -> AiSearchInstance_Metadata_CreatedFromEmdashPluginRef {
        AiSearchInstance_Metadata_CreatedFromEmdashPluginRef::new(
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
pub struct AiSearchInstance_PublicEndpointParams_ChatCompletionsEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl AiSearchInstance_PublicEndpointParams_ChatCompletionsEndpoint {
    #[doc = "Set the field `disabled`.\nDisable chat completions endpoint for this public endpoint"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchInstance_PublicEndpointParams_ChatCompletionsEndpoint {
    type O = terrars::BlockAssignable<AiSearchInstance_PublicEndpointParams_ChatCompletionsEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_PublicEndpointParams_ChatCompletionsEndpoint {}

impl BuildAiSearchInstance_PublicEndpointParams_ChatCompletionsEndpoint {
    pub fn build(self) -> AiSearchInstance_PublicEndpointParams_ChatCompletionsEndpoint {
        AiSearchInstance_PublicEndpointParams_ChatCompletionsEndpoint { disabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_PublicEndpointParams_ChatCompletionsEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_PublicEndpointParams_ChatCompletionsEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiSearchInstance_PublicEndpointParams_ChatCompletionsEndpointRef {
        AiSearchInstance_PublicEndpointParams_ChatCompletionsEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_PublicEndpointParams_ChatCompletionsEndpointRef {
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
pub struct AiSearchInstance_PublicEndpointParams_Mcp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl AiSearchInstance_PublicEndpointParams_Mcp {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\nDisable MCP endpoint for this public endpoint"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchInstance_PublicEndpointParams_Mcp {
    type O = terrars::BlockAssignable<AiSearchInstance_PublicEndpointParams_Mcp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_PublicEndpointParams_Mcp {}

impl BuildAiSearchInstance_PublicEndpointParams_Mcp {
    pub fn build(self) -> AiSearchInstance_PublicEndpointParams_Mcp {
        AiSearchInstance_PublicEndpointParams_Mcp {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_PublicEndpointParams_McpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_PublicEndpointParams_McpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiSearchInstance_PublicEndpointParams_McpRef {
        AiSearchInstance_PublicEndpointParams_McpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_PublicEndpointParams_McpRef {
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
pub struct AiSearchInstance_PublicEndpointParams_RateLimit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technique: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AiSearchInstance_PublicEndpointParams_RateLimit {
    #[doc = "Set the field `period_ms`.\n"]
    pub fn set_period_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requests`.\n"]
    pub fn set_requests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.requests = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technique`.\nAvailable values: \"fixed\", \"sliding\"."]
    pub fn set_technique(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technique = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchInstance_PublicEndpointParams_RateLimit {
    type O = terrars::BlockAssignable<AiSearchInstance_PublicEndpointParams_RateLimit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_PublicEndpointParams_RateLimit {}

impl BuildAiSearchInstance_PublicEndpointParams_RateLimit {
    pub fn build(self) -> AiSearchInstance_PublicEndpointParams_RateLimit {
        AiSearchInstance_PublicEndpointParams_RateLimit {
            period_ms: core::default::Default::default(),
            requests: core::default::Default::default(),
            technique: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_PublicEndpointParams_RateLimitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_PublicEndpointParams_RateLimitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiSearchInstance_PublicEndpointParams_RateLimitRef {
        AiSearchInstance_PublicEndpointParams_RateLimitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_PublicEndpointParams_RateLimitRef {
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
pub struct AiSearchInstance_PublicEndpointParams_SearchEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl AiSearchInstance_PublicEndpointParams_SearchEndpoint {
    #[doc = "Set the field `disabled`.\nDisable search endpoint for this public endpoint"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchInstance_PublicEndpointParams_SearchEndpoint {
    type O = terrars::BlockAssignable<AiSearchInstance_PublicEndpointParams_SearchEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_PublicEndpointParams_SearchEndpoint {}

impl BuildAiSearchInstance_PublicEndpointParams_SearchEndpoint {
    pub fn build(self) -> AiSearchInstance_PublicEndpointParams_SearchEndpoint {
        AiSearchInstance_PublicEndpointParams_SearchEndpoint { disabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_PublicEndpointParams_SearchEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_PublicEndpointParams_SearchEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiSearchInstance_PublicEndpointParams_SearchEndpointRef {
        AiSearchInstance_PublicEndpointParams_SearchEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_PublicEndpointParams_SearchEndpointRef {
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
pub struct AiSearchInstance_PublicEndpointParams {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorized_hosts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chat_completions_endpoint: std::option::Option<AiSearchInstance_PublicEndpointParams_ChatCompletionsEndpoint>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_domain_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcp: std::option::Option<AiSearchInstance_PublicEndpointParams_Mcp>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit: std::option::Option<AiSearchInstance_PublicEndpointParams_RateLimit>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search_endpoint: std::option::Option<AiSearchInstance_PublicEndpointParams_SearchEndpoint>,
}

impl AiSearchInstance_PublicEndpointParams {
    #[doc = "Set the field `authorized_hosts`.\n"]
    pub fn set_authorized_hosts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.authorized_hosts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chat_completions_endpoint`.\n"]
    pub fn set_chat_completions_endpoint(
        mut self,
        v: impl std::convert::Into<AiSearchInstance_PublicEndpointParams_ChatCompletionsEndpoint>,
    ) -> Self {
        self.chat_completions_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_domains`.\nCustom domain hostnames that alias this public endpoint. GET and create responses return the current set; on update (PUT) this field is only echoed back when supplied in the request body, otherwise it is null (omit it to leave domains unchanged)."]
    pub fn set_custom_domains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_domains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_domain_enabled`.\nWhen false, the instance is reachable only via a registered custom domain and the default <public_endpoint_id>.search.ai.cloudflare.com host returns 404. Requires at least one custom domain. Defaults to true. public_endpoint_params is replaced wholesale on update, so resend default_domain_enabled on every update to keep the default host off — omitting it resets to true."]
    pub fn set_default_domain_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.default_domain_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mcp`.\n"]
    pub fn set_mcp(mut self, v: impl std::convert::Into<AiSearchInstance_PublicEndpointParams_Mcp>) -> Self {
        self.mcp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rate_limit`.\n"]
    pub fn set_rate_limit(mut self, v: impl std::convert::Into<AiSearchInstance_PublicEndpointParams_RateLimit>) -> Self {
        self.rate_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search_endpoint`.\n"]
    pub fn set_search_endpoint(
        mut self,
        v: impl std::convert::Into<AiSearchInstance_PublicEndpointParams_SearchEndpoint>,
    ) -> Self {
        self.search_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchInstance_PublicEndpointParams {
    type O = terrars::BlockAssignable<AiSearchInstance_PublicEndpointParams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_PublicEndpointParams {}

impl BuildAiSearchInstance_PublicEndpointParams {
    pub fn build(self) -> AiSearchInstance_PublicEndpointParams {
        AiSearchInstance_PublicEndpointParams {
            authorized_hosts: core::default::Default::default(),
            chat_completions_endpoint: core::default::Default::default(),
            custom_domains: core::default::Default::default(),
            default_domain_enabled: core::default::Default::default(),
            enabled: core::default::Default::default(),
            mcp: core::default::Default::default(),
            rate_limit: core::default::Default::default(),
            search_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_PublicEndpointParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_PublicEndpointParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiSearchInstance_PublicEndpointParamsRef {
        AiSearchInstance_PublicEndpointParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_PublicEndpointParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorized_hosts` after provisioning.\n"]
    pub fn authorized_hosts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorized_hosts", self.base))
    }

    #[doc = "Get a reference to the value of field `chat_completions_endpoint` after provisioning.\n"]
    pub fn chat_completions_endpoint(&self) -> AiSearchInstance_PublicEndpointParams_ChatCompletionsEndpointRef {
        AiSearchInstance_PublicEndpointParams_ChatCompletionsEndpointRef::new(
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
    pub fn mcp(&self) -> AiSearchInstance_PublicEndpointParams_McpRef {
        AiSearchInstance_PublicEndpointParams_McpRef::new(self.shared().clone(), format!("{}.mcp", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limit` after provisioning.\n"]
    pub fn rate_limit(&self) -> AiSearchInstance_PublicEndpointParams_RateLimitRef {
        AiSearchInstance_PublicEndpointParams_RateLimitRef::new(
            self.shared().clone(),
            format!("{}.rate_limit", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `search_endpoint` after provisioning.\n"]
    pub fn search_endpoint(&self) -> AiSearchInstance_PublicEndpointParams_SearchEndpointRef {
        AiSearchInstance_PublicEndpointParams_SearchEndpointRef::new(
            self.shared().clone(),
            format!("{}.search_endpoint", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AiSearchInstance_RetrievalOptions_BoostBy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    field: terrars::PrimField<std::string::String>,
}

impl AiSearchInstance_RetrievalOptions_BoostBy {
    #[doc =
        "Set the field `direction`.\nBoost direction. 'desc' = higher values rank higher (e.g. newer timestamps). 'asc' = lower values rank higher. 'exists' = boost chunks that have the field. 'not_exists' = boost chunks that lack the field. Optional — defaults to 'asc' for numeric/datetime fields, 'exists' for text/boolean fields.\nAvailable values: \"asc\", \"desc\", \"exists\", \"not_exists\"."]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchInstance_RetrievalOptions_BoostBy {
    type O = terrars::BlockAssignable<AiSearchInstance_RetrievalOptions_BoostBy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_RetrievalOptions_BoostBy {
    #[doc =
        "Metadata field name to boost by. Use 'timestamp' for document freshness, or any custom_metadata field. Numeric and datetime fields support all four directions (asc, desc, exists, not_exists); text/boolean fields only support exists/not_exists."]
    pub field: terrars::PrimField<std::string::String>,
}

impl BuildAiSearchInstance_RetrievalOptions_BoostBy {
    pub fn build(self) -> AiSearchInstance_RetrievalOptions_BoostBy {
        AiSearchInstance_RetrievalOptions_BoostBy {
            direction: core::default::Default::default(),
            field: self.field,
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_RetrievalOptions_BoostByRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_RetrievalOptions_BoostByRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiSearchInstance_RetrievalOptions_BoostByRef {
        AiSearchInstance_RetrievalOptions_BoostByRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_RetrievalOptions_BoostByRef {
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
pub struct AiSearchInstance_RetrievalOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boost_by: std::option::Option<std::vec::Vec<AiSearchInstance_RetrievalOptions_BoostBy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keyword_match_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AiSearchInstance_RetrievalOptions {
    #[doc =
        "Set the field `boost_by`.\nMetadata fields to boost search results by. Each entry specifies a metadata field and an optional direction. Direction defaults to 'asc' for numeric/datetime fields and 'exists' for text/boolean fields. Fields must match 'timestamp' or a defined custom_metadata field."]
    pub fn set_boost_by(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<AiSearchInstance_RetrievalOptions_BoostBy>>,
    ) -> Self {
        self.boost_by = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `keyword_match_mode`.\nControls which documents are candidates for BM25 scoring. 'and' restricts candidates to documents containing all query terms; 'or' includes any document containing at least one term, ranked by BM25 relevance. When omitted on an update, the existing stored value is preserved; when never set, search falls back to 'and'.\nAvailable values: \"and\", \"or\"."]
    pub fn set_keyword_match_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.keyword_match_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchInstance_RetrievalOptions {
    type O = terrars::BlockAssignable<AiSearchInstance_RetrievalOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_RetrievalOptions {}

impl BuildAiSearchInstance_RetrievalOptions {
    pub fn build(self) -> AiSearchInstance_RetrievalOptions {
        AiSearchInstance_RetrievalOptions {
            boost_by: core::default::Default::default(),
            keyword_match_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_RetrievalOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_RetrievalOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiSearchInstance_RetrievalOptionsRef {
        AiSearchInstance_RetrievalOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_RetrievalOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boost_by` after provisioning.\nMetadata fields to boost search results by. Each entry specifies a metadata field and an optional direction. Direction defaults to 'asc' for numeric/datetime fields and 'exists' for text/boolean fields. Fields must match 'timestamp' or a defined custom_metadata field."]
    pub fn boost_by(&self) -> terrars::ListRef<AiSearchInstance_RetrievalOptions_BoostByRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boost_by", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keyword_match_mode` after provisioning.\nControls which documents are candidates for BM25 scoring. 'and' restricts candidates to documents containing all query terms; 'or' includes any document containing at least one term, ranked by BM25 relevance. When omitted on an update, the existing stored value is preserved; when never set, search falls back to 'and'.\nAvailable values: \"and\", \"or\"."]
    pub fn keyword_match_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keyword_match_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct AiSearchInstance_SourceParams_WebCrawler_DiscoverOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    depth: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_external_links: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_subdomains: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AiSearchInstance_SourceParams_WebCrawler_DiscoverOptions {
    #[doc = "Set the field `depth`.\nMaximum link-follow depth from the seed URL."]
    pub fn set_depth(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.depth = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_external_links`.\nFollow links that point outside the source domain. Must stay `false` — discover crawls are restricted to the zone you own."]
    pub fn set_include_external_links(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_external_links = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_subdomains`.\nFollow links to subdomains of the source host."]
    pub fn set_include_subdomains(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_subdomains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limit`.\nMaximum number of pages to crawl (1-100000)."]
    pub fn set_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age`.\nMaximum content age in seconds to accept (0–604800)."]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source`.\nWhere the crawler looks for URLs: 'sitemaps' reads sitemap XML only, 'links' follows page links only, 'all' does both.\nAvailable values: \"all\", \"sitemaps\", \"links\"."]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchInstance_SourceParams_WebCrawler_DiscoverOptions {
    type O = terrars::BlockAssignable<AiSearchInstance_SourceParams_WebCrawler_DiscoverOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_SourceParams_WebCrawler_DiscoverOptions {}

impl BuildAiSearchInstance_SourceParams_WebCrawler_DiscoverOptions {
    pub fn build(self) -> AiSearchInstance_SourceParams_WebCrawler_DiscoverOptions {
        AiSearchInstance_SourceParams_WebCrawler_DiscoverOptions {
            depth: core::default::Default::default(),
            include_external_links: core::default::Default::default(),
            include_subdomains: core::default::Default::default(),
            limit: core::default::Default::default(),
            max_age: core::default::Default::default(),
            source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_SourceParams_WebCrawler_DiscoverOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_SourceParams_WebCrawler_DiscoverOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiSearchInstance_SourceParams_WebCrawler_DiscoverOptionsRef {
        AiSearchInstance_SourceParams_WebCrawler_DiscoverOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_SourceParams_WebCrawler_DiscoverOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `depth` after provisioning.\nMaximum link-follow depth from the seed URL."]
    pub fn depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.depth", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_external_links` after provisioning.\nFollow links that point outside the source domain. Must stay `false` — discover crawls are restricted to the zone you own."]
    pub fn include_external_links(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_external_links", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_subdomains` after provisioning.\nFollow links to subdomains of the source host."]
    pub fn include_subdomains(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_subdomains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `limit` after provisioning.\nMaximum number of pages to crawl (1-100000)."]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_age` after provisioning.\nMaximum content age in seconds to accept (0–604800)."]
    pub fn max_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nWhere the crawler looks for URLs: 'sitemaps' reads sitemap XML only, 'links' follows page links only, 'all' does both.\nAvailable values: \"all\", \"sitemaps\", \"links\"."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct AiSearchInstance_SourceParams_WebCrawler_ParseOptions_ContentSelector {
    path: terrars::PrimField<std::string::String>,
    selector: terrars::PrimField<std::string::String>,
}

impl AiSearchInstance_SourceParams_WebCrawler_ParseOptions_ContentSelector { }

impl terrars::ToListMappable for AiSearchInstance_SourceParams_WebCrawler_ParseOptions_ContentSelector {
    type O = terrars::BlockAssignable<AiSearchInstance_SourceParams_WebCrawler_ParseOptions_ContentSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_SourceParams_WebCrawler_ParseOptions_ContentSelector {
    #[doc =
        "Glob pattern to match against the page URL path. Uses standard glob syntax: * matches within a segment, ** crosses directories."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc =
        "CSS selector to extract content from pages matching the path pattern. Must not contain disallowed characters (;, `, $, {, }, \\). Must target a single element; if multiple elements match, the selector is ignored and the full page is used."]
    pub selector: terrars::PrimField<std::string::String>,
}

impl BuildAiSearchInstance_SourceParams_WebCrawler_ParseOptions_ContentSelector {
    pub fn build(self) -> AiSearchInstance_SourceParams_WebCrawler_ParseOptions_ContentSelector {
        AiSearchInstance_SourceParams_WebCrawler_ParseOptions_ContentSelector {
            path: self.path,
            selector: self.selector,
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_SourceParams_WebCrawler_ParseOptions_ContentSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_SourceParams_WebCrawler_ParseOptions_ContentSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiSearchInstance_SourceParams_WebCrawler_ParseOptions_ContentSelectorRef {
        AiSearchInstance_SourceParams_WebCrawler_ParseOptions_ContentSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_SourceParams_WebCrawler_ParseOptions_ContentSelectorRef {
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
pub struct AiSearchInstance_SourceParams_WebCrawler_ParseOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_selector: std
    ::option
    ::Option<std::vec::Vec<AiSearchInstance_SourceParams_WebCrawler_ParseOptions_ContentSelector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_headers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_images: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    specific_sitemaps: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_browser_rendering: std::option::Option<terrars::PrimField<bool>>,
}

impl AiSearchInstance_SourceParams_WebCrawler_ParseOptions {
    #[doc =
        "Set the field `content_selector`.\nList of path-to-selector mappings for extracting specific content from crawled pages. Each entry pairs a URL glob pattern with a CSS selector. The first matching path wins. Only the matched HTML fragment is stored and indexed. Omit the field to disable content selection — empty arrays are rejected."]
    pub fn set_content_selector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<std::vec::Vec<AiSearchInstance_SourceParams_WebCrawler_ParseOptions_ContentSelector>>,
    ) -> Self {
        self.content_selector = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_headers`.\nUp to 5 custom HTTP headers sent with each crawl request. Names must be RFC-7230 token characters (no spaces, colons, or control characters); values must be HTAB + printable ASCII (no CR/LF)."]
    pub fn set_include_headers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_images`.\n"]
    pub fn set_include_images(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_images = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `specific_sitemaps`.\nList of specific sitemap URLs to use for crawling. Only valid when parse_type is 'sitemap'."]
    pub fn set_specific_sitemaps(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.specific_sitemaps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_browser_rendering`.\n"]
    pub fn set_use_browser_rendering(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_browser_rendering = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchInstance_SourceParams_WebCrawler_ParseOptions {
    type O = terrars::BlockAssignable<AiSearchInstance_SourceParams_WebCrawler_ParseOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_SourceParams_WebCrawler_ParseOptions {}

impl BuildAiSearchInstance_SourceParams_WebCrawler_ParseOptions {
    pub fn build(self) -> AiSearchInstance_SourceParams_WebCrawler_ParseOptions {
        AiSearchInstance_SourceParams_WebCrawler_ParseOptions {
            content_selector: core::default::Default::default(),
            include_headers: core::default::Default::default(),
            include_images: core::default::Default::default(),
            specific_sitemaps: core::default::Default::default(),
            use_browser_rendering: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_SourceParams_WebCrawler_ParseOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_SourceParams_WebCrawler_ParseOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiSearchInstance_SourceParams_WebCrawler_ParseOptionsRef {
        AiSearchInstance_SourceParams_WebCrawler_ParseOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_SourceParams_WebCrawler_ParseOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content_selector` after provisioning.\nList of path-to-selector mappings for extracting specific content from crawled pages. Each entry pairs a URL glob pattern with a CSS selector. The first matching path wins. Only the matched HTML fragment is stored and indexed. Omit the field to disable content selection — empty arrays are rejected."]
    pub fn content_selector(
        &self,
    ) -> terrars::ListRef<AiSearchInstance_SourceParams_WebCrawler_ParseOptions_ContentSelectorRef> {
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
pub struct AiSearchInstance_SourceParams_WebCrawler {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discover_options: std::option::Option<AiSearchInstance_SourceParams_WebCrawler_DiscoverOptions>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parse_options: std::option::Option<AiSearchInstance_SourceParams_WebCrawler_ParseOptions>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parse_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AiSearchInstance_SourceParams_WebCrawler {
    #[doc =
        "Set the field `discover_options`.\nOptions for parse_type 'discover', where Browser Run discovers URLs by link following and sitemaps. Ignored for 'sitemap'."]
    pub fn set_discover_options(
        mut self,
        v: impl std::convert::Into<AiSearchInstance_SourceParams_WebCrawler_DiscoverOptions>,
    ) -> Self {
        self.discover_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parse_options`.\n"]
    pub fn set_parse_options(
        mut self,
        v: impl std::convert::Into<AiSearchInstance_SourceParams_WebCrawler_ParseOptions>,
    ) -> Self {
        self.parse_options = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parse_type`.\nHow URLs are discovered. 'sitemap' reads XML sitemaps; 'discover' follows links recursively and requires the source to be a Verified zone on this account.\nAvailable values: \"sitemap\", \"discover\"."]
    pub fn set_parse_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parse_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchInstance_SourceParams_WebCrawler {
    type O = terrars::BlockAssignable<AiSearchInstance_SourceParams_WebCrawler>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_SourceParams_WebCrawler {}

impl BuildAiSearchInstance_SourceParams_WebCrawler {
    pub fn build(self) -> AiSearchInstance_SourceParams_WebCrawler {
        AiSearchInstance_SourceParams_WebCrawler {
            discover_options: core::default::Default::default(),
            parse_options: core::default::Default::default(),
            parse_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_SourceParams_WebCrawlerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_SourceParams_WebCrawlerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiSearchInstance_SourceParams_WebCrawlerRef {
        AiSearchInstance_SourceParams_WebCrawlerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_SourceParams_WebCrawlerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `discover_options` after provisioning.\nOptions for parse_type 'discover', where Browser Run discovers URLs by link following and sitemaps. Ignored for 'sitemap'."]
    pub fn discover_options(&self) -> AiSearchInstance_SourceParams_WebCrawler_DiscoverOptionsRef {
        AiSearchInstance_SourceParams_WebCrawler_DiscoverOptionsRef::new(
            self.shared().clone(),
            format!("{}.discover_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `parse_options` after provisioning.\n"]
    pub fn parse_options(&self) -> AiSearchInstance_SourceParams_WebCrawler_ParseOptionsRef {
        AiSearchInstance_SourceParams_WebCrawler_ParseOptionsRef::new(
            self.shared().clone(),
            format!("{}.parse_options", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `parse_type` after provisioning.\nHow URLs are discovered. 'sitemap' reads XML sitemaps; 'discover' follows links recursively and requires the source to be a Verified zone on this account.\nAvailable values: \"sitemap\", \"discover\"."]
    pub fn parse_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parse_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AiSearchInstance_SourceParams {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_items: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_items: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    r2_jurisdiction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_crawler: std::option::Option<AiSearchInstance_SourceParams_WebCrawler>,
}

impl AiSearchInstance_SourceParams {
    #[doc =
        "Set the field `exclude_items`.\nList of path patterns to exclude. Uses micromatch glob syntax: * matches within a path segment, ** matches across path segments (e.g., /admin/** matches /admin/users and /admin/settings/advanced). Most accounts are limited to 10 rules; contact support to raise it."]
    pub fn set_exclude_items(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_items = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_items`.\nList of path patterns to include. Uses micromatch glob syntax: * matches within a path segment, ** matches across path segments (e.g., /blog/** matches /blog/post and /blog/2024/post). Most accounts are limited to 10 rules; contact support to raise it."]
    pub fn set_include_items(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `r2_jurisdiction`.\n"]
    pub fn set_r2_jurisdiction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.r2_jurisdiction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_crawler`.\n"]
    pub fn set_web_crawler(mut self, v: impl std::convert::Into<AiSearchInstance_SourceParams_WebCrawler>) -> Self {
        self.web_crawler = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchInstance_SourceParams {
    type O = terrars::BlockAssignable<AiSearchInstance_SourceParams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchInstance_SourceParams {}

impl BuildAiSearchInstance_SourceParams {
    pub fn build(self) -> AiSearchInstance_SourceParams {
        AiSearchInstance_SourceParams {
            exclude_items: core::default::Default::default(),
            include_items: core::default::Default::default(),
            prefix: core::default::Default::default(),
            r2_jurisdiction: core::default::Default::default(),
            web_crawler: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchInstance_SourceParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchInstance_SourceParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiSearchInstance_SourceParamsRef {
        AiSearchInstance_SourceParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchInstance_SourceParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude_items` after provisioning.\nList of path patterns to exclude. Uses micromatch glob syntax: * matches within a path segment, ** matches across path segments (e.g., /admin/** matches /admin/users and /admin/settings/advanced). Most accounts are limited to 10 rules; contact support to raise it."]
    pub fn exclude_items(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_items", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_items` after provisioning.\nList of path patterns to include. Uses micromatch glob syntax: * matches within a path segment, ** matches across path segments (e.g., /blog/** matches /blog/post and /blog/2024/post). Most accounts are limited to 10 rules; contact support to raise it."]
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
    pub fn web_crawler(&self) -> AiSearchInstance_SourceParams_WebCrawlerRef {
        AiSearchInstance_SourceParams_WebCrawlerRef::new(self.shared().clone(), format!("{}.web_crawler", self.base))
    }
}
