use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataIpRangesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataIpRanges_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIpRangesData>,
}

#[derive(Clone)]
pub struct DataIpRanges(Rc<DataIpRanges_>);

impl DataIpRanges {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderCloudflare) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `china_ipv4_cidr_blocks` after provisioning.\nThe lexically ordered list of only the IPv4 China CIDR blocks."]
    pub fn china_ipv4_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.china_ipv4_cidr_blocks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `china_ipv6_cidr_blocks` after provisioning.\nThe lexically ordered list of only the IPv6 China CIDR blocks."]
    pub fn china_ipv6_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.china_ipv6_cidr_blocks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cidr_blocks` after provisioning.\nThe lexically ordered list of all non-China CIDR blocks."]
    pub fn cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.cidr_blocks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv4_cidr_blocks` after provisioning.\nThe lexically ordered list of only the IPv4 CIDR blocks."]
    pub fn ipv4_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipv4_cidr_blocks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_cidr_blocks` after provisioning.\nThe lexically ordered list of only the IPv6 CIDR blocks."]
    pub fn ipv6_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipv6_cidr_blocks", self.extract_ref()))
    }
}

impl Referable for DataIpRanges {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataIpRanges { }

impl ToListMappable for DataIpRanges {
    type O = ListRef<DataIpRangesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataIpRanges_ {
    fn extract_datasource_type(&self) -> String {
        "cloudflare_ip_ranges".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIpRanges {
    pub tf_id: String,
}

impl BuildDataIpRanges {
    pub fn build(self, stack: &mut Stack) -> DataIpRanges {
        let out = DataIpRanges(Rc::new(DataIpRanges_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIpRangesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIpRangesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIpRangesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIpRangesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `china_ipv4_cidr_blocks` after provisioning.\nThe lexically ordered list of only the IPv4 China CIDR blocks."]
    pub fn china_ipv4_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.china_ipv4_cidr_blocks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `china_ipv6_cidr_blocks` after provisioning.\nThe lexically ordered list of only the IPv6 China CIDR blocks."]
    pub fn china_ipv6_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.china_ipv6_cidr_blocks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cidr_blocks` after provisioning.\nThe lexically ordered list of all non-China CIDR blocks."]
    pub fn cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.cidr_blocks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv4_cidr_blocks` after provisioning.\nThe lexically ordered list of only the IPv4 CIDR blocks."]
    pub fn ipv4_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipv4_cidr_blocks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_cidr_blocks` after provisioning.\nThe lexically ordered list of only the IPv6 CIDR blocks."]
    pub fn ipv6_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipv6_cidr_blocks", self.extract_ref()))
    }
}
