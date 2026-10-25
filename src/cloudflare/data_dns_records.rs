#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataDnsRecordsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<DataDnsRecords_Comment>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<DataDnsRecords_Content>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_shadow_metadata: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<DataDnsRecords_Name>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxied: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shadowed_by_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shadowing_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<DataDnsRecords_Tag>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataDnsRecords_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDnsRecordsData_>,
}

#[derive(Clone)]
pub struct DataDnsRecords(std::rc::Rc<DataDnsRecords_>);

impl DataDnsRecords {
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

    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(self, v: impl std::convert::Into<DataDnsRecords_Comment>) -> Self {
        self.0.data.borrow_mut().comment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content`.\n"]
    pub fn set_content(self, v: impl std::convert::Into<DataDnsRecords_Content>) -> Self {
        self.0.data.borrow_mut().content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\nDirection to order DNS records in.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_shadow_metadata`.\nWhether to include shadow metadata in the `meta` field of each record in the response. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records)."]
    pub fn set_include_shadow_metadata(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_shadow_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `match_`.\nWhether to match all search requirements or at least one (any). If set to `all`, acts like a logical AND between filters. If set to `any`, acts like a logical OR instead. Note that the interaction between tag filters is controlled by the `tag-match` parameter instead.\nAvailable values: \"any\", \"all\"."]
    pub fn set_match(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<DataDnsRecords_Name>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order`.\nField to order DNS records by.\nAvailable values: \"type\", \"name\", \"content\", \"ttl\", \"proxied\"."]
    pub fn set_order(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxied`.\nWhether the record is receiving the performance and security benefits of Cloudflare."]
    pub fn set_proxied(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().proxied = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `search`.\nAllows searching in multiple properties of a DNS record simultaneously. This parameter is intended for human users, not automation. Its exact behavior is intentionally left unspecified and is subject to change in the future. This parameter works independently of the `match` setting. For automated searches, please use the other available parameters."]
    pub fn set_search(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().search = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shadowed_by_name`.\nFilters the response to records at or below the specified NS delegation name. NS, DS, and NSEC records at the delegation name are excluded because they are not shadowed by that delegation. Those record types are included only when they exist below the delegation. The value must be a non-apex subdomain of the zone. Requires `include_shadow_metadata=true`. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records)."]
    pub fn set_shadowed_by_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().shadowed_by_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shadowing_name`.\nReturns NS records that shadow the given name, searching at the name itself and each of its ancestor names within the zone, excluding the zone apex. The value must be a subdomain of the zone; the zone apex is not accepted. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records)."]
    pub fn set_shadowing_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().shadowing_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(self, v: impl std::convert::Into<DataDnsRecords_Tag>) -> Self {
        self.0.data.borrow_mut().tag = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag_match`.\nWhether to match all tag search requirements or at least one (any). If set to `all`, acts like a logical AND between tag filters. If set to `any`, acts like a logical OR instead. Note that the regular `match` parameter is still used to combine the resulting condition with other filters that aren't related to tags.\nAvailable values: \"any\", \"all\"."]
    pub fn set_tag_match(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tag_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nRecord type.\nAvailable values: \"A\", \"AAAA\", \"CAA\", \"CERT\", \"CNAME\", \"DNSKEY\", \"DS\", \"HTTPS\", \"LOC\", \"MX\", \"NAPTR\", \"NS\", \"OPENPGPKEY\", \"PTR\", \"SMIMEA\", \"SRV\", \"SSHFP\", \"SVCB\", \"TLSA\", \"TXT\", \"URI\"."]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> DataDnsRecords_CommentRef {
        DataDnsRecords_CommentRef::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> DataDnsRecords_ContentRef {
        DataDnsRecords_ContentRef::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order DNS records in.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_shadow_metadata` after provisioning.\nWhether to include shadow metadata in the `meta` field of each record in the response. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records)."]
    pub fn include_shadow_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_shadow_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nWhether to match all search requirements or at least one (any). If set to `all`, acts like a logical AND between filters. If set to `any`, acts like a logical OR instead. Note that the interaction between tag filters is controlled by the `tag-match` parameter instead.\nAvailable values: \"any\", \"all\"."]
    pub fn match_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> DataDnsRecords_NameRef {
        DataDnsRecords_NameRef::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to order DNS records by.\nAvailable values: \"type\", \"name\", \"content\", \"ttl\", \"proxied\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proxied` after provisioning.\nWhether the record is receiving the performance and security benefits of Cloudflare."]
    pub fn proxied(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxied", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataDnsRecords_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nAllows searching in multiple properties of a DNS record simultaneously. This parameter is intended for human users, not automation. Its exact behavior is intentionally left unspecified and is subject to change in the future. This parameter works independently of the `match` setting. For automated searches, please use the other available parameters."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shadowed_by_name` after provisioning.\nFilters the response to records at or below the specified NS delegation name. NS, DS, and NSEC records at the delegation name are excluded because they are not shadowed by that delegation. Those record types are included only when they exist below the delegation. The value must be a non-apex subdomain of the zone. Requires `include_shadow_metadata=true`. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records)."]
    pub fn shadowed_by_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shadowed_by_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shadowing_name` after provisioning.\nReturns NS records that shadow the given name, searching at the name itself and each of its ancestor names within the zone, excluding the zone apex. The value must be a subdomain of the zone; the zone apex is not accepted. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records)."]
    pub fn shadowing_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shadowing_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> DataDnsRecords_TagRef {
        DataDnsRecords_TagRef::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tag_match` after provisioning.\nWhether to match all tag search requirements or at least one (any). If set to `all`, acts like a logical AND between tag filters. If set to `any`, acts like a logical OR instead. Note that the regular `match` parameter is still used to combine the resulting condition with other filters that aren't related to tags.\nAvailable values: \"any\", \"all\"."]
    pub fn tag_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_match", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nRecord type.\nAvailable values: \"A\", \"AAAA\", \"CAA\", \"CERT\", \"CNAME\", \"DNSKEY\", \"DS\", \"HTTPS\", \"LOC\", \"MX\", \"NAPTR\", \"NS\", \"OPENPGPKEY\", \"PTR\", \"SMIMEA\", \"SRV\", \"SSHFP\", \"SVCB\", \"TLSA\", \"TXT\", \"URI\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataDnsRecords {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDnsRecords { }

impl terrars::ToListMappable for DataDnsRecords {
    type O = terrars::ListRef<DataDnsRecordsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDnsRecords_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_dns_records".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDnsRecords {
    pub tf_id: std::string::String,
}

impl BuildDataDnsRecords {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDnsRecords {
        let out = DataDnsRecords(std::rc::Rc::new(DataDnsRecords_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDnsRecordsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                comment: core::default::Default::default(),
                content: core::default::Default::default(),
                direction: core::default::Default::default(),
                include_shadow_metadata: core::default::Default::default(),
                match_: core::default::Default::default(),
                max_items: core::default::Default::default(),
                name: core::default::Default::default(),
                order: core::default::Default::default(),
                proxied: core::default::Default::default(),
                search: core::default::Default::default(),
                shadowed_by_name: core::default::Default::default(),
                shadowing_name: core::default::Default::default(),
                tag: core::default::Default::default(),
                tag_match: core::default::Default::default(),
                type_: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDnsRecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDnsRecordsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDnsRecordsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> DataDnsRecords_CommentRef {
        DataDnsRecords_CommentRef::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> DataDnsRecords_ContentRef {
        DataDnsRecords_ContentRef::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order DNS records in.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_shadow_metadata` after provisioning.\nWhether to include shadow metadata in the `meta` field of each record in the response. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records)."]
    pub fn include_shadow_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_shadow_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nWhether to match all search requirements or at least one (any). If set to `all`, acts like a logical AND between filters. If set to `any`, acts like a logical OR instead. Note that the interaction between tag filters is controlled by the `tag-match` parameter instead.\nAvailable values: \"any\", \"all\"."]
    pub fn match_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> DataDnsRecords_NameRef {
        DataDnsRecords_NameRef::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to order DNS records by.\nAvailable values: \"type\", \"name\", \"content\", \"ttl\", \"proxied\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proxied` after provisioning.\nWhether the record is receiving the performance and security benefits of Cloudflare."]
    pub fn proxied(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxied", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataDnsRecords_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nAllows searching in multiple properties of a DNS record simultaneously. This parameter is intended for human users, not automation. Its exact behavior is intentionally left unspecified and is subject to change in the future. This parameter works independently of the `match` setting. For automated searches, please use the other available parameters."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shadowed_by_name` after provisioning.\nFilters the response to records at or below the specified NS delegation name. NS, DS, and NSEC records at the delegation name are excluded because they are not shadowed by that delegation. Those record types are included only when they exist below the delegation. The value must be a non-apex subdomain of the zone. Requires `include_shadow_metadata=true`. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records)."]
    pub fn shadowed_by_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shadowed_by_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shadowing_name` after provisioning.\nReturns NS records that shadow the given name, searching at the name itself and each of its ancestor names within the zone, excluding the zone apex. The value must be a subdomain of the zone; the zone apex is not accepted. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records)."]
    pub fn shadowing_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shadowing_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> DataDnsRecords_TagRef {
        DataDnsRecords_TagRef::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tag_match` after provisioning.\nWhether to match all tag search requirements or at least one (any). If set to `all`, acts like a logical AND between tag filters. If set to `any`, acts like a logical OR instead. Note that the regular `match` parameter is still used to combine the resulting condition with other filters that aren't related to tags.\nAvailable values: \"any\", \"all\"."]
    pub fn tag_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_match", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nRecord type.\nAvailable values: \"A\", \"AAAA\", \"CAA\", \"CERT\", \"CNAME\", \"DNSKEY\", \"DS\", \"HTTPS\", \"LOC\", \"MX\", \"NAPTR\", \"NS\", \"OPENPGPKEY\", \"PTR\", \"SMIMEA\", \"SRV\", \"SSHFP\", \"SVCB\", \"TLSA\", \"TXT\", \"URI\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDnsRecords_Comment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    absent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endswith: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    present: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startswith: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDnsRecords_Comment {
    #[doc = "Set the field `absent`.\nIf this parameter is present, only records *without* a comment are returned."]
    pub fn set_absent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.absent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `contains`.\nSubstring of the DNS record comment. Comment filters are case-insensitive."]
    pub fn set_contains(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endswith`.\nSuffix of the DNS record comment. Comment filters are case-insensitive."]
    pub fn set_endswith(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endswith = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exact`.\nExact value of the DNS record comment. Comment filters are case-insensitive."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `present`.\nIf this parameter is present, only records *with* a comment are returned."]
    pub fn set_present(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.present = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `startswith`.\nPrefix of the DNS record comment. Comment filters are case-insensitive."]
    pub fn set_startswith(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.startswith = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDnsRecords_Comment {
    type O = terrars::BlockAssignable<DataDnsRecords_Comment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDnsRecords_Comment {}

impl BuildDataDnsRecords_Comment {
    pub fn build(self) -> DataDnsRecords_Comment {
        DataDnsRecords_Comment {
            absent: core::default::Default::default(),
            contains: core::default::Default::default(),
            endswith: core::default::Default::default(),
            exact: core::default::Default::default(),
            present: core::default::Default::default(),
            startswith: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDnsRecords_CommentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDnsRecords_CommentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDnsRecords_CommentRef {
        DataDnsRecords_CommentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDnsRecords_CommentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `absent` after provisioning.\nIf this parameter is present, only records *without* a comment are returned."]
    pub fn absent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.absent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `contains` after provisioning.\nSubstring of the DNS record comment. Comment filters are case-insensitive."]
    pub fn contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `endswith` after provisioning.\nSuffix of the DNS record comment. Comment filters are case-insensitive."]
    pub fn endswith(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endswith", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exact` after provisioning.\nExact value of the DNS record comment. Comment filters are case-insensitive."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc =
        "Get a reference to the value of field `present` after provisioning.\nIf this parameter is present, only records *with* a comment are returned."]
    pub fn present(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.present", self.base))
    }

    #[doc =
        "Get a reference to the value of field `startswith` after provisioning.\nPrefix of the DNS record comment. Comment filters are case-insensitive."]
    pub fn startswith(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.startswith", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDnsRecords_Content {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endswith: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startswith: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDnsRecords_Content {
    #[doc = "Set the field `contains`.\nSubstring of the DNS record content. Content filters are case-insensitive."]
    pub fn set_contains(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endswith`.\nSuffix of the DNS record content. Content filters are case-insensitive."]
    pub fn set_endswith(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endswith = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exact`.\nExact value of the DNS record content. Content filters are case-insensitive."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `startswith`.\nPrefix of the DNS record content. Content filters are case-insensitive."]
    pub fn set_startswith(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.startswith = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDnsRecords_Content {
    type O = terrars::BlockAssignable<DataDnsRecords_Content>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDnsRecords_Content {}

impl BuildDataDnsRecords_Content {
    pub fn build(self) -> DataDnsRecords_Content {
        DataDnsRecords_Content {
            contains: core::default::Default::default(),
            endswith: core::default::Default::default(),
            exact: core::default::Default::default(),
            startswith: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDnsRecords_ContentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDnsRecords_ContentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDnsRecords_ContentRef {
        DataDnsRecords_ContentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDnsRecords_ContentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `contains` after provisioning.\nSubstring of the DNS record content. Content filters are case-insensitive."]
    pub fn contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `endswith` after provisioning.\nSuffix of the DNS record content. Content filters are case-insensitive."]
    pub fn endswith(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endswith", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exact` after provisioning.\nExact value of the DNS record content. Content filters are case-insensitive."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc =
        "Get a reference to the value of field `startswith` after provisioning.\nPrefix of the DNS record content. Content filters are case-insensitive."]
    pub fn startswith(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.startswith", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDnsRecords_Name {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endswith: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startswith: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDnsRecords_Name {
    #[doc = "Set the field `contains`.\nSubstring of the DNS record name. Name filters are case-insensitive."]
    pub fn set_contains(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endswith`.\nSuffix of the DNS record name. Name filters are case-insensitive."]
    pub fn set_endswith(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endswith = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exact`.\nExact value of the DNS record name. Name filters are case-insensitive."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `startswith`.\nPrefix of the DNS record name. Name filters are case-insensitive."]
    pub fn set_startswith(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.startswith = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDnsRecords_Name {
    type O = terrars::BlockAssignable<DataDnsRecords_Name>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDnsRecords_Name {}

impl BuildDataDnsRecords_Name {
    pub fn build(self) -> DataDnsRecords_Name {
        DataDnsRecords_Name {
            contains: core::default::Default::default(),
            endswith: core::default::Default::default(),
            exact: core::default::Default::default(),
            startswith: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDnsRecords_NameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDnsRecords_NameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDnsRecords_NameRef {
        DataDnsRecords_NameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDnsRecords_NameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `contains` after provisioning.\nSubstring of the DNS record name. Name filters are case-insensitive."]
    pub fn contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `endswith` after provisioning.\nSuffix of the DNS record name. Name filters are case-insensitive."]
    pub fn endswith(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endswith", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exact` after provisioning.\nExact value of the DNS record name. Name filters are case-insensitive."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc =
        "Get a reference to the value of field `startswith` after provisioning.\nPrefix of the DNS record name. Name filters are case-insensitive."]
    pub fn startswith(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.startswith", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDnsRecords_Result_Data {}

impl DataDnsRecords_Result_Data { }

impl terrars::ToListMappable for DataDnsRecords_Result_Data {
    type O = terrars::BlockAssignable<DataDnsRecords_Result_Data>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDnsRecords_Result_Data {}

impl BuildDataDnsRecords_Result_Data {
    pub fn build(self) -> DataDnsRecords_Result_Data {
        DataDnsRecords_Result_Data {}
    }
}

#[allow(dead_code)]
pub struct DataDnsRecords_Result_DataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDnsRecords_Result_DataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDnsRecords_Result_DataRef {
        DataDnsRecords_Result_DataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDnsRecords_Result_DataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\nAlgorithm."]
    pub fn algorithm(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `altitude` after provisioning.\nAltitude of location in meters."]
    pub fn altitude(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.altitude", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\nCertificate."]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `digest` after provisioning.\nDigest."]
    pub fn digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest", self.base))
    }

    #[doc = "Get a reference to the value of field `digest_type` after provisioning.\nDigest Type."]
    pub fn digest_type(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest_type", self.base))
    }

    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.base))
    }

    #[doc = "Get a reference to the value of field `flags` after provisioning.\nFlags for the CAA record."]
    pub fn flags(&self) -> terrars::PrimExpr<serde_json::Value> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flags", self.base))
    }

    #[doc = "Get a reference to the value of field `key_tag` after provisioning.\nKey Tag."]
    pub fn key_tag(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_tag", self.base))
    }

    #[doc = "Get a reference to the value of field `lat_degrees` after provisioning.\nDegrees of latitude."]
    pub fn lat_degrees(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lat_degrees", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lat_direction` after provisioning.\nLatitude direction.\nAvailable values: \"N\", \"S\"."]
    pub fn lat_direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lat_direction", self.base))
    }

    #[doc = "Get a reference to the value of field `lat_minutes` after provisioning.\nMinutes of latitude."]
    pub fn lat_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lat_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `lat_seconds` after provisioning.\nSeconds of latitude."]
    pub fn lat_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lat_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `long_degrees` after provisioning.\nDegrees of longitude."]
    pub fn long_degrees(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.long_degrees", self.base))
    }

    #[doc =
        "Get a reference to the value of field `long_direction` after provisioning.\nLongitude direction.\nAvailable values: \"E\", \"W\"."]
    pub fn long_direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.long_direction", self.base))
    }

    #[doc = "Get a reference to the value of field `long_minutes` after provisioning.\nMinutes of longitude."]
    pub fn long_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.long_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `long_seconds` after provisioning.\nSeconds of longitude."]
    pub fn long_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.long_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `matching_type` after provisioning.\nMatching Type."]
    pub fn matching_type(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.matching_type", self.base))
    }

    #[doc = "Get a reference to the value of field `order` after provisioning.\nOrder."]
    pub fn order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port of the service."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `precision_horz` after provisioning.\nHorizontal precision of location."]
    pub fn precision_horz(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precision_horz", self.base))
    }

    #[doc =
        "Get a reference to the value of field `precision_vert` after provisioning.\nVertical precision of location."]
    pub fn precision_vert(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precision_vert", self.base))
    }

    #[doc = "Get a reference to the value of field `preference` after provisioning.\nPreference."]
    pub fn preference(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preference", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\nPriority."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nProtocol."]
    pub fn protocol(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `public_key` after provisioning.\nPublic Key."]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\nRegex."]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `replacement` after provisioning.\nReplacement."]
    pub fn replacement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replacement", self.base))
    }

    #[doc = "Get a reference to the value of field `selector` after provisioning.\nSelector."]
    pub fn selector(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.selector", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\nService."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\nSize of location in meters."]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nName of the property controlled by this record (e.g.: issue, issuewild, iodef)."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\nTarget."]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType."]
    pub fn type_(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `usage` after provisioning.\nUsage."]
    pub fn usage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nValue of the record. This field's semantics depend on the chosen tag."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\nThe record weight."]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDnsRecords_Result_Settings {}

impl DataDnsRecords_Result_Settings { }

impl terrars::ToListMappable for DataDnsRecords_Result_Settings {
    type O = terrars::BlockAssignable<DataDnsRecords_Result_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDnsRecords_Result_Settings {}

impl BuildDataDnsRecords_Result_Settings {
    pub fn build(self) -> DataDnsRecords_Result_Settings {
        DataDnsRecords_Result_Settings {}
    }
}

#[allow(dead_code)]
pub struct DataDnsRecords_Result_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDnsRecords_Result_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDnsRecords_Result_SettingsRef {
        DataDnsRecords_Result_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDnsRecords_Result_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `flatten_cname` after provisioning.\nIf enabled, causes the CNAME record to be resolved externally and the resulting address records (e.g., A and AAAA) to be returned instead of the CNAME record itself. This setting is unavailable for proxied records, since they are always flattened."]
    pub fn flatten_cname(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flatten_cname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_only` after provisioning.\nWhen enabled, only A records will be generated, and AAAA records will not be created. This setting is intended for exceptional cases. Note that this option only applies to proxied records and it has no effect on whether Cloudflare communicates with the origin using IPv4 or IPv6."]
    pub fn ipv4_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_only` after provisioning.\nWhen enabled, only AAAA records will be generated, and A records will not be created. This setting is intended for exceptional cases. Note that this option only applies to proxied records and it has no effect on whether Cloudflare communicates with the origin using IPv4 or IPv6."]
    pub fn ipv6_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_only", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDnsRecords_Result {}

impl DataDnsRecords_Result { }

impl terrars::ToListMappable for DataDnsRecords_Result {
    type O = terrars::BlockAssignable<DataDnsRecords_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDnsRecords_Result {}

impl BuildDataDnsRecords_Result {
    pub fn build(self) -> DataDnsRecords_Result {
        DataDnsRecords_Result {}
    }
}

#[allow(dead_code)]
pub struct DataDnsRecords_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDnsRecords_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDnsRecords_ResultRef {
        DataDnsRecords_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDnsRecords_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `comment` after provisioning.\nComments or notes about the DNS record. This field has no effect on DNS responses."]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `comment_modified_on` after provisioning.\nWhen the record comment was last modified. Omitted if there is no comment."]
    pub fn comment_modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment_modified_on", self.base))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\nA valid IPv4 address."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the record was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\nComponents of a CAA record."]
    pub fn data(&self) -> DataDnsRecords_Result_DataRef {
        DataDnsRecords_Result_DataRef::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `meta` after provisioning.\nExtra Cloudflare-specific information about the record."]
    pub fn meta(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.meta", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the record was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nComplete DNS record name, including the zone name, in Punycode."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nRequired for MX and URI records; ignored for other record types (but may still be returned by the API). Records with lower priorities are preferred. This field is to be deprecated in favor of the priority field within the data map."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_routing` after provisioning.\nEnables private network routing to the origin."]
    pub fn private_routing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_routing", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxiable` after provisioning.\nWhether the record can be proxied by Cloudflare or not."]
    pub fn proxiable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxiable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxied` after provisioning.\nWhether the record is receiving the performance and security benefits of Cloudflare."]
    pub fn proxied(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxied", self.base))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\nSettings for the DNS record."]
    pub fn settings(&self) -> DataDnsRecords_Result_SettingsRef {
        DataDnsRecords_Result_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nCustom tags for the DNS record. This field has no effect on DNS responses."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags_modified_on` after provisioning.\nWhen the record tags were last modified. Omitted if there are no tags."]
    pub fn tags_modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tags_modified_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nTime To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'. Value must be between 60 and 86400, with the minimum reduced to 30 for Enterprise zones."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nRecord type.\nAvailable values: \"A\", \"AAAA\", \"CNAME\", \"MX\", \"NS\", \"OPENPGPKEY\", \"PTR\", \"TXT\", \"CAA\", \"CERT\", \"DNSKEY\", \"DS\", \"HTTPS\", \"LOC\", \"NAPTR\", \"SMIMEA\", \"SRV\", \"SSHFP\", \"SVCB\", \"TLSA\", \"URI\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDnsRecords_Tag {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    absent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endswith: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    present: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startswith: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDnsRecords_Tag {
    #[doc =
        "Set the field `absent`.\nName of a tag which must *not* be present on the DNS record. Tag filters are case-insensitive."]
    pub fn set_absent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.absent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `contains`.\nA tag and value, of the form `<tag-name>:<tag-value>`. The API will only return DNS records that have a tag named `<tag-name>` whose value contains `<tag-value>`. Tag filters are case-insensitive."]
    pub fn set_contains(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `endswith`.\nA tag and value, of the form `<tag-name>:<tag-value>`. The API will only return DNS records that have a tag named `<tag-name>` whose value ends with `<tag-value>`. Tag filters are case-insensitive."]
    pub fn set_endswith(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endswith = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exact`.\nA tag and value, of the form `<tag-name>:<tag-value>`. The API will only return DNS records that have a tag named `<tag-name>` whose value is `<tag-value>`. Tag filters are case-insensitive."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `present`.\nName of a tag which must be present on the DNS record. Tag filters are case-insensitive."]
    pub fn set_present(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.present = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `startswith`.\nA tag and value, of the form `<tag-name>:<tag-value>`. The API will only return DNS records that have a tag named `<tag-name>` whose value starts with `<tag-value>`. Tag filters are case-insensitive."]
    pub fn set_startswith(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.startswith = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDnsRecords_Tag {
    type O = terrars::BlockAssignable<DataDnsRecords_Tag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDnsRecords_Tag {}

impl BuildDataDnsRecords_Tag {
    pub fn build(self) -> DataDnsRecords_Tag {
        DataDnsRecords_Tag {
            absent: core::default::Default::default(),
            contains: core::default::Default::default(),
            endswith: core::default::Default::default(),
            exact: core::default::Default::default(),
            present: core::default::Default::default(),
            startswith: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDnsRecords_TagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDnsRecords_TagRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDnsRecords_TagRef {
        DataDnsRecords_TagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDnsRecords_TagRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `absent` after provisioning.\nName of a tag which must *not* be present on the DNS record. Tag filters are case-insensitive."]
    pub fn absent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.absent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `contains` after provisioning.\nA tag and value, of the form `<tag-name>:<tag-value>`. The API will only return DNS records that have a tag named `<tag-name>` whose value contains `<tag-value>`. Tag filters are case-insensitive."]
    pub fn contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `endswith` after provisioning.\nA tag and value, of the form `<tag-name>:<tag-value>`. The API will only return DNS records that have a tag named `<tag-name>` whose value ends with `<tag-value>`. Tag filters are case-insensitive."]
    pub fn endswith(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endswith", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exact` after provisioning.\nA tag and value, of the form `<tag-name>:<tag-value>`. The API will only return DNS records that have a tag named `<tag-name>` whose value is `<tag-value>`. Tag filters are case-insensitive."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc =
        "Get a reference to the value of field `present` after provisioning.\nName of a tag which must be present on the DNS record. Tag filters are case-insensitive."]
    pub fn present(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.present", self.base))
    }

    #[doc =
        "Get a reference to the value of field `startswith` after provisioning.\nA tag and value, of the form `<tag-name>:<tag-value>`. The API will only return DNS records that have a tag named `<tag-name>` whose value starts with `<tag-value>`. Tag filters are case-insensitive."]
    pub fn startswith(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.startswith", self.base))
    }
}
