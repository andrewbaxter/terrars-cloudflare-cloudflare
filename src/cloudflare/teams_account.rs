use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct TeamsAccountData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activity_log_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_decrypt_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_browser_isolation_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    antivirus: Option<Vec<TeamsAccountAntivirusEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_page: Option<Vec<TeamsAccountBlockPageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fips: Option<Vec<TeamsAccountFipsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<Vec<TeamsAccountLoggingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload_log: Option<Vec<TeamsAccountPayloadLogEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy: Option<Vec<TeamsAccountProxyEl>>,
    dynamic: TeamsAccountDynamic,
}

struct TeamsAccount_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TeamsAccountData>,
}

#[derive(Clone)]
pub struct TeamsAccount(Rc<TeamsAccount_>);

impl TeamsAccount {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `activity_log_enabled`.\nWhether to enable the activity log."]
    pub fn set_activity_log_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().activity_log_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `tls_decrypt_enabled`.\nIndicator that decryption of TLS traffic is enabled."]
    pub fn set_tls_decrypt_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().tls_decrypt_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `url_browser_isolation_enabled`.\nSafely browse websites in Browser Isolation through a URL."]
    pub fn set_url_browser_isolation_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().url_browser_isolation_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `antivirus`.\n"]
    pub fn set_antivirus(self, v: impl Into<BlockAssignable<TeamsAccountAntivirusEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().antivirus = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.antivirus = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `block_page`.\n"]
    pub fn set_block_page(self, v: impl Into<BlockAssignable<TeamsAccountBlockPageEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().block_page = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.block_page = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `fips`.\n"]
    pub fn set_fips(self, v: impl Into<BlockAssignable<TeamsAccountFipsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fips = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fips = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `logging`.\n"]
    pub fn set_logging(self, v: impl Into<BlockAssignable<TeamsAccountLoggingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `payload_log`.\n"]
    pub fn set_payload_log(self, v: impl Into<BlockAssignable<TeamsAccountPayloadLogEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().payload_log = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.payload_log = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `proxy`.\n"]
    pub fn set_proxy(self, v: impl Into<BlockAssignable<TeamsAccountProxyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().proxy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.proxy = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `activity_log_enabled` after provisioning.\nWhether to enable the activity log."]
    pub fn activity_log_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.activity_log_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tls_decrypt_enabled` after provisioning.\nIndicator that decryption of TLS traffic is enabled."]
    pub fn tls_decrypt_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_decrypt_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url_browser_isolation_enabled` after provisioning.\nSafely browse websites in Browser Isolation through a URL."]
    pub fn url_browser_isolation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.url_browser_isolation_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `antivirus` after provisioning.\n"]
    pub fn antivirus(&self) -> ListRef<TeamsAccountAntivirusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.antivirus", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `block_page` after provisioning.\n"]
    pub fn block_page(&self) -> ListRef<TeamsAccountBlockPageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.block_page", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fips` after provisioning.\n"]
    pub fn fips(&self) -> ListRef<TeamsAccountFipsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fips", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<TeamsAccountLoggingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `payload_log` after provisioning.\n"]
    pub fn payload_log(&self) -> ListRef<TeamsAccountPayloadLogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.payload_log", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> ListRef<TeamsAccountProxyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.proxy", self.extract_ref()))
    }
}

impl Referable for TeamsAccount {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for TeamsAccount { }

impl ToListMappable for TeamsAccount {
    type O = ListRef<TeamsAccountRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TeamsAccount_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_teams_account".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTeamsAccount {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
}

impl BuildTeamsAccount {
    pub fn build(self, stack: &mut Stack) -> TeamsAccount {
        let out = TeamsAccount(Rc::new(TeamsAccount_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TeamsAccountData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                activity_log_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                tls_decrypt_enabled: core::default::Default::default(),
                url_browser_isolation_enabled: core::default::Default::default(),
                antivirus: core::default::Default::default(),
                block_page: core::default::Default::default(),
                fips: core::default::Default::default(),
                logging: core::default::Default::default(),
                payload_log: core::default::Default::default(),
                proxy: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TeamsAccountRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsAccountRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TeamsAccountRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `activity_log_enabled` after provisioning.\nWhether to enable the activity log."]
    pub fn activity_log_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.activity_log_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tls_decrypt_enabled` after provisioning.\nIndicator that decryption of TLS traffic is enabled."]
    pub fn tls_decrypt_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_decrypt_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url_browser_isolation_enabled` after provisioning.\nSafely browse websites in Browser Isolation through a URL."]
    pub fn url_browser_isolation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.url_browser_isolation_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `antivirus` after provisioning.\n"]
    pub fn antivirus(&self) -> ListRef<TeamsAccountAntivirusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.antivirus", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `block_page` after provisioning.\n"]
    pub fn block_page(&self) -> ListRef<TeamsAccountBlockPageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.block_page", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fips` after provisioning.\n"]
    pub fn fips(&self) -> ListRef<TeamsAccountFipsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fips", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<TeamsAccountLoggingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `payload_log` after provisioning.\n"]
    pub fn payload_log(&self) -> ListRef<TeamsAccountPayloadLogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.payload_log", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> ListRef<TeamsAccountProxyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.proxy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TeamsAccountAntivirusEl {
    enabled_download_phase: PrimField<bool>,
    enabled_upload_phase: PrimField<bool>,
    fail_closed: PrimField<bool>,
}

impl TeamsAccountAntivirusEl { }

impl ToListMappable for TeamsAccountAntivirusEl {
    type O = BlockAssignable<TeamsAccountAntivirusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsAccountAntivirusEl {
    #[doc= "Scan on file download."]
    pub enabled_download_phase: PrimField<bool>,
    #[doc= "Scan on file upload."]
    pub enabled_upload_phase: PrimField<bool>,
    #[doc= "Block requests for files that cannot be scanned."]
    pub fail_closed: PrimField<bool>,
}

impl BuildTeamsAccountAntivirusEl {
    pub fn build(self) -> TeamsAccountAntivirusEl {
        TeamsAccountAntivirusEl {
            enabled_download_phase: self.enabled_download_phase,
            enabled_upload_phase: self.enabled_upload_phase,
            fail_closed: self.fail_closed,
        }
    }
}

pub struct TeamsAccountAntivirusElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsAccountAntivirusElRef {
    fn new(shared: StackShared, base: String) -> TeamsAccountAntivirusElRef {
        TeamsAccountAntivirusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsAccountAntivirusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled_download_phase` after provisioning.\nScan on file download."]
    pub fn enabled_download_phase(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled_download_phase", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled_upload_phase` after provisioning.\nScan on file upload."]
    pub fn enabled_upload_phase(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled_upload_phase", self.base))
    }

    #[doc= "Get a reference to the value of field `fail_closed` after provisioning.\nBlock requests for files that cannot be scanned."]
    pub fn fail_closed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_closed", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsAccountBlockPageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    footer_text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logo_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mailto_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mailto_subject: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl TeamsAccountBlockPageEl {
    #[doc= "Set the field `background_color`.\nHex code of block page background color."]
    pub fn set_background_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.background_color = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\nIndicator of enablement."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `footer_text`.\nBlock page footer text."]
    pub fn set_footer_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.footer_text = Some(v.into());
        self
    }

    #[doc= "Set the field `header_text`.\nBlock page header text."]
    pub fn set_header_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_text = Some(v.into());
        self
    }

    #[doc= "Set the field `logo_path`.\nURL of block page logo."]
    pub fn set_logo_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logo_path = Some(v.into());
        self
    }

    #[doc= "Set the field `mailto_address`.\nAdmin email for users to contact."]
    pub fn set_mailto_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mailto_address = Some(v.into());
        self
    }

    #[doc= "Set the field `mailto_subject`.\nSubject line for emails created from block page."]
    pub fn set_mailto_subject(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mailto_subject = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nName of block page configuration."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for TeamsAccountBlockPageEl {
    type O = BlockAssignable<TeamsAccountBlockPageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsAccountBlockPageEl {}

impl BuildTeamsAccountBlockPageEl {
    pub fn build(self) -> TeamsAccountBlockPageEl {
        TeamsAccountBlockPageEl {
            background_color: core::default::Default::default(),
            enabled: core::default::Default::default(),
            footer_text: core::default::Default::default(),
            header_text: core::default::Default::default(),
            logo_path: core::default::Default::default(),
            mailto_address: core::default::Default::default(),
            mailto_subject: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct TeamsAccountBlockPageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsAccountBlockPageElRef {
    fn new(shared: StackShared, base: String) -> TeamsAccountBlockPageElRef {
        TeamsAccountBlockPageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsAccountBlockPageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `background_color` after provisioning.\nHex code of block page background color."]
    pub fn background_color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.background_color", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nIndicator of enablement."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `footer_text` after provisioning.\nBlock page footer text."]
    pub fn footer_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.footer_text", self.base))
    }

    #[doc= "Get a reference to the value of field `header_text` after provisioning.\nBlock page header text."]
    pub fn header_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_text", self.base))
    }

    #[doc= "Get a reference to the value of field `logo_path` after provisioning.\nURL of block page logo."]
    pub fn logo_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logo_path", self.base))
    }

    #[doc= "Get a reference to the value of field `mailto_address` after provisioning.\nAdmin email for users to contact."]
    pub fn mailto_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mailto_address", self.base))
    }

    #[doc= "Get a reference to the value of field `mailto_subject` after provisioning.\nSubject line for emails created from block page."]
    pub fn mailto_subject(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mailto_subject", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of block page configuration."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsAccountFipsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<PrimField<bool>>,
}

impl TeamsAccountFipsEl {
    #[doc= "Set the field `tls`.\nOnly allow FIPS-compliant TLS configuration."]
    pub fn set_tls(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.tls = Some(v.into());
        self
    }
}

impl ToListMappable for TeamsAccountFipsEl {
    type O = BlockAssignable<TeamsAccountFipsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsAccountFipsEl {}

impl BuildTeamsAccountFipsEl {
    pub fn build(self) -> TeamsAccountFipsEl {
        TeamsAccountFipsEl { tls: core::default::Default::default() }
    }
}

pub struct TeamsAccountFipsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsAccountFipsElRef {
    fn new(shared: StackShared, base: String) -> TeamsAccountFipsElRef {
        TeamsAccountFipsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsAccountFipsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `tls` after provisioning.\nOnly allow FIPS-compliant TLS configuration."]
    pub fn tls(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsAccountLoggingElSettingsByRuleTypeElDnsEl {
    log_all: PrimField<bool>,
    log_blocks: PrimField<bool>,
}

impl TeamsAccountLoggingElSettingsByRuleTypeElDnsEl { }

impl ToListMappable for TeamsAccountLoggingElSettingsByRuleTypeElDnsEl {
    type O = BlockAssignable<TeamsAccountLoggingElSettingsByRuleTypeElDnsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsAccountLoggingElSettingsByRuleTypeElDnsEl {
    #[doc= "Whether to log all activity."]
    pub log_all: PrimField<bool>,
    #[doc= ""]
    pub log_blocks: PrimField<bool>,
}

impl BuildTeamsAccountLoggingElSettingsByRuleTypeElDnsEl {
    pub fn build(self) -> TeamsAccountLoggingElSettingsByRuleTypeElDnsEl {
        TeamsAccountLoggingElSettingsByRuleTypeElDnsEl {
            log_all: self.log_all,
            log_blocks: self.log_blocks,
        }
    }
}

pub struct TeamsAccountLoggingElSettingsByRuleTypeElDnsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsAccountLoggingElSettingsByRuleTypeElDnsElRef {
    fn new(shared: StackShared, base: String) -> TeamsAccountLoggingElSettingsByRuleTypeElDnsElRef {
        TeamsAccountLoggingElSettingsByRuleTypeElDnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsAccountLoggingElSettingsByRuleTypeElDnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `log_all` after provisioning.\nWhether to log all activity."]
    pub fn log_all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_all", self.base))
    }

    #[doc= "Get a reference to the value of field `log_blocks` after provisioning.\n"]
    pub fn log_blocks(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_blocks", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsAccountLoggingElSettingsByRuleTypeElHttpEl {
    log_all: PrimField<bool>,
    log_blocks: PrimField<bool>,
}

impl TeamsAccountLoggingElSettingsByRuleTypeElHttpEl { }

impl ToListMappable for TeamsAccountLoggingElSettingsByRuleTypeElHttpEl {
    type O = BlockAssignable<TeamsAccountLoggingElSettingsByRuleTypeElHttpEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsAccountLoggingElSettingsByRuleTypeElHttpEl {
    #[doc= "Whether to log all activity."]
    pub log_all: PrimField<bool>,
    #[doc= ""]
    pub log_blocks: PrimField<bool>,
}

impl BuildTeamsAccountLoggingElSettingsByRuleTypeElHttpEl {
    pub fn build(self) -> TeamsAccountLoggingElSettingsByRuleTypeElHttpEl {
        TeamsAccountLoggingElSettingsByRuleTypeElHttpEl {
            log_all: self.log_all,
            log_blocks: self.log_blocks,
        }
    }
}

pub struct TeamsAccountLoggingElSettingsByRuleTypeElHttpElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsAccountLoggingElSettingsByRuleTypeElHttpElRef {
    fn new(shared: StackShared, base: String) -> TeamsAccountLoggingElSettingsByRuleTypeElHttpElRef {
        TeamsAccountLoggingElSettingsByRuleTypeElHttpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsAccountLoggingElSettingsByRuleTypeElHttpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `log_all` after provisioning.\nWhether to log all activity."]
    pub fn log_all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_all", self.base))
    }

    #[doc= "Get a reference to the value of field `log_blocks` after provisioning.\n"]
    pub fn log_blocks(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_blocks", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsAccountLoggingElSettingsByRuleTypeElL4El {
    log_all: PrimField<bool>,
    log_blocks: PrimField<bool>,
}

impl TeamsAccountLoggingElSettingsByRuleTypeElL4El { }

impl ToListMappable for TeamsAccountLoggingElSettingsByRuleTypeElL4El {
    type O = BlockAssignable<TeamsAccountLoggingElSettingsByRuleTypeElL4El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsAccountLoggingElSettingsByRuleTypeElL4El {
    #[doc= "Whether to log all activity."]
    pub log_all: PrimField<bool>,
    #[doc= ""]
    pub log_blocks: PrimField<bool>,
}

impl BuildTeamsAccountLoggingElSettingsByRuleTypeElL4El {
    pub fn build(self) -> TeamsAccountLoggingElSettingsByRuleTypeElL4El {
        TeamsAccountLoggingElSettingsByRuleTypeElL4El {
            log_all: self.log_all,
            log_blocks: self.log_blocks,
        }
    }
}

pub struct TeamsAccountLoggingElSettingsByRuleTypeElL4ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsAccountLoggingElSettingsByRuleTypeElL4ElRef {
    fn new(shared: StackShared, base: String) -> TeamsAccountLoggingElSettingsByRuleTypeElL4ElRef {
        TeamsAccountLoggingElSettingsByRuleTypeElL4ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsAccountLoggingElSettingsByRuleTypeElL4ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `log_all` after provisioning.\nWhether to log all activity."]
    pub fn log_all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_all", self.base))
    }

    #[doc= "Get a reference to the value of field `log_blocks` after provisioning.\n"]
    pub fn log_blocks(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_blocks", self.base))
    }
}

#[derive(Serialize, Default)]
struct TeamsAccountLoggingElSettingsByRuleTypeElDynamic {
    dns: Option<DynamicBlock<TeamsAccountLoggingElSettingsByRuleTypeElDnsEl>>,
    http: Option<DynamicBlock<TeamsAccountLoggingElSettingsByRuleTypeElHttpEl>>,
    l4: Option<DynamicBlock<TeamsAccountLoggingElSettingsByRuleTypeElL4El>>,
}

#[derive(Serialize)]
pub struct TeamsAccountLoggingElSettingsByRuleTypeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns: Option<Vec<TeamsAccountLoggingElSettingsByRuleTypeElDnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http: Option<Vec<TeamsAccountLoggingElSettingsByRuleTypeElHttpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    l4: Option<Vec<TeamsAccountLoggingElSettingsByRuleTypeElL4El>>,
    dynamic: TeamsAccountLoggingElSettingsByRuleTypeElDynamic,
}

impl TeamsAccountLoggingElSettingsByRuleTypeEl {
    #[doc= "Set the field `dns`.\n"]
    pub fn set_dns(mut self, v: impl Into<BlockAssignable<TeamsAccountLoggingElSettingsByRuleTypeElDnsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dns = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dns = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `http`.\n"]
    pub fn set_http(mut self, v: impl Into<BlockAssignable<TeamsAccountLoggingElSettingsByRuleTypeElHttpEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `l4`.\n"]
    pub fn set_l4(mut self, v: impl Into<BlockAssignable<TeamsAccountLoggingElSettingsByRuleTypeElL4El>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.l4 = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.l4 = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for TeamsAccountLoggingElSettingsByRuleTypeEl {
    type O = BlockAssignable<TeamsAccountLoggingElSettingsByRuleTypeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsAccountLoggingElSettingsByRuleTypeEl {}

impl BuildTeamsAccountLoggingElSettingsByRuleTypeEl {
    pub fn build(self) -> TeamsAccountLoggingElSettingsByRuleTypeEl {
        TeamsAccountLoggingElSettingsByRuleTypeEl {
            dns: core::default::Default::default(),
            http: core::default::Default::default(),
            l4: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TeamsAccountLoggingElSettingsByRuleTypeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsAccountLoggingElSettingsByRuleTypeElRef {
    fn new(shared: StackShared, base: String) -> TeamsAccountLoggingElSettingsByRuleTypeElRef {
        TeamsAccountLoggingElSettingsByRuleTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsAccountLoggingElSettingsByRuleTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dns` after provisioning.\n"]
    pub fn dns(&self) -> ListRef<TeamsAccountLoggingElSettingsByRuleTypeElDnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dns", self.base))
    }

    #[doc= "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> ListRef<TeamsAccountLoggingElSettingsByRuleTypeElHttpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc= "Get a reference to the value of field `l4` after provisioning.\n"]
    pub fn l4(&self) -> ListRef<TeamsAccountLoggingElSettingsByRuleTypeElL4ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.l4", self.base))
    }
}

#[derive(Serialize, Default)]
struct TeamsAccountLoggingElDynamic {
    settings_by_rule_type: Option<DynamicBlock<TeamsAccountLoggingElSettingsByRuleTypeEl>>,
}

#[derive(Serialize)]
pub struct TeamsAccountLoggingEl {
    redact_pii: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings_by_rule_type: Option<Vec<TeamsAccountLoggingElSettingsByRuleTypeEl>>,
    dynamic: TeamsAccountLoggingElDynamic,
}

impl TeamsAccountLoggingEl {
    #[doc= "Set the field `settings_by_rule_type`.\n"]
    pub fn set_settings_by_rule_type(
        mut self,
        v: impl Into<BlockAssignable<TeamsAccountLoggingElSettingsByRuleTypeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.settings_by_rule_type = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.settings_by_rule_type = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for TeamsAccountLoggingEl {
    type O = BlockAssignable<TeamsAccountLoggingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsAccountLoggingEl {
    #[doc= "Redact personally identifiable information from activity logging (PII fields are: source IP, user email, user ID, device ID, URL, referrer, user agent)."]
    pub redact_pii: PrimField<bool>,
}

impl BuildTeamsAccountLoggingEl {
    pub fn build(self) -> TeamsAccountLoggingEl {
        TeamsAccountLoggingEl {
            redact_pii: self.redact_pii,
            settings_by_rule_type: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TeamsAccountLoggingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsAccountLoggingElRef {
    fn new(shared: StackShared, base: String) -> TeamsAccountLoggingElRef {
        TeamsAccountLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsAccountLoggingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `redact_pii` after provisioning.\nRedact personally identifiable information from activity logging (PII fields are: source IP, user email, user ID, device ID, URL, referrer, user agent)."]
    pub fn redact_pii(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.redact_pii", self.base))
    }

    #[doc= "Get a reference to the value of field `settings_by_rule_type` after provisioning.\n"]
    pub fn settings_by_rule_type(&self) -> ListRef<TeamsAccountLoggingElSettingsByRuleTypeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings_by_rule_type", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsAccountPayloadLogEl {
    public_key: PrimField<String>,
}

impl TeamsAccountPayloadLogEl { }

impl ToListMappable for TeamsAccountPayloadLogEl {
    type O = BlockAssignable<TeamsAccountPayloadLogEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsAccountPayloadLogEl {
    #[doc= "Public key used to encrypt matched payloads."]
    pub public_key: PrimField<String>,
}

impl BuildTeamsAccountPayloadLogEl {
    pub fn build(self) -> TeamsAccountPayloadLogEl {
        TeamsAccountPayloadLogEl { public_key: self.public_key }
    }
}

pub struct TeamsAccountPayloadLogElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsAccountPayloadLogElRef {
    fn new(shared: StackShared, base: String) -> TeamsAccountPayloadLogElRef {
        TeamsAccountPayloadLogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsAccountPayloadLogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `public_key` after provisioning.\nPublic key used to encrypt matched payloads."]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }
}

#[derive(Serialize)]
pub struct TeamsAccountProxyEl {
    tcp: PrimField<bool>,
    udp: PrimField<bool>,
}

impl TeamsAccountProxyEl { }

impl ToListMappable for TeamsAccountProxyEl {
    type O = BlockAssignable<TeamsAccountProxyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamsAccountProxyEl {
    #[doc= "Whether gateway proxy is enabled on gateway devices for TCP traffic."]
    pub tcp: PrimField<bool>,
    #[doc= "Whether gateway proxy is enabled on gateway devices for UDP traffic."]
    pub udp: PrimField<bool>,
}

impl BuildTeamsAccountProxyEl {
    pub fn build(self) -> TeamsAccountProxyEl {
        TeamsAccountProxyEl {
            tcp: self.tcp,
            udp: self.udp,
        }
    }
}

pub struct TeamsAccountProxyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamsAccountProxyElRef {
    fn new(shared: StackShared, base: String) -> TeamsAccountProxyElRef {
        TeamsAccountProxyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamsAccountProxyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `tcp` after provisioning.\nWhether gateway proxy is enabled on gateway devices for TCP traffic."]
    pub fn tcp(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tcp", self.base))
    }

    #[doc= "Get a reference to the value of field `udp` after provisioning.\nWhether gateway proxy is enabled on gateway devices for UDP traffic."]
    pub fn udp(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.udp", self.base))
    }
}

#[derive(Serialize, Default)]
struct TeamsAccountDynamic {
    antivirus: Option<DynamicBlock<TeamsAccountAntivirusEl>>,
    block_page: Option<DynamicBlock<TeamsAccountBlockPageEl>>,
    fips: Option<DynamicBlock<TeamsAccountFipsEl>>,
    logging: Option<DynamicBlock<TeamsAccountLoggingEl>>,
    payload_log: Option<DynamicBlock<TeamsAccountPayloadLogEl>>,
    proxy: Option<DynamicBlock<TeamsAccountProxyEl>>,
}
