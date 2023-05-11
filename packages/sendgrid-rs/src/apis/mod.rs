use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

pub mod api_key_permissions_api;
pub mod api_keys_api;
pub mod alerts_api;
pub mod blocks_api_api;
pub mod bounces_api_api;
pub mod csvui_only_api;
pub mod campaigns_api_api;
pub mod cancel_scheduled_sends_api;
pub mod categories_api;
pub mod certificates_api;
pub mod contacts_api;
pub mod contacts_api_custom_fields_api;
pub mod contacts_api_lists_api;
pub mod contacts_api_recipients_api;
pub mod contacts_api_segments_api;
pub mod custom_fields_api;
pub mod designs_api_api;
pub mod domain_authentication_api;
pub mod email_address_validation_api;
pub mod email_cname_records_api;
pub mod ip_access_management_api;
pub mod ip_addresses_api;
pub mod ip_pools_api;
pub mod ip_warmup_api;
pub mod invalid_emails_api_api;
pub mod link_branding_api;
pub mod lists_api;
pub mod mail_send_api;
pub mod marketing_campaigns_stats_api;
pub mod messages_api;
pub mod query_api;
pub mod reverse_dns_api;
pub mod segmenting_contacts_api;
pub mod segmenting_contacts_v2_api;
pub mod segmenting_contacts_v2_beta_api;
pub mod send_test_email_api;
pub mod sender_identities_api_api;
pub mod sender_verification_api;
pub mod senders_api;
pub mod settings_enforced_tls_api;
pub mod settings_inbound_parse_api;
pub mod settings_mail_api;
pub mod settings_partner_api;
pub mod settings_tracking_api;
pub mod single_sends_api;
pub mod single_sign_on_settings_api;
pub mod single_sign_on_teammates_api;
pub mod spam_reports_api_api;
pub mod stats_api;
pub mod subuser_monitor_settings_api;
pub mod subuser_statistics_api;
pub mod subusers_api_api;
pub mod suppressions_global_suppressions_api;
pub mod suppressions_suppressions_api;
pub mod suppressions_unsubscribe_groups_api;
pub mod teammates_api;
pub mod transactional_templates_api;
pub mod transactional_templates_versions_api;
pub mod users_api_api;
pub mod v3_api;
pub mod webhooks_api;

pub mod configuration;
