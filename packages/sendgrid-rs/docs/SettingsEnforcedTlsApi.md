# \SettingsEnforcedTlsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**g_et_user_settings_enforced_tls**](SettingsEnforcedTlsApi.md#g_et_user_settings_enforced_tls) | **GET** /user/settings/enforced_tls | Retrieve current Enforced TLS settings.
[**p_atch_user_settings_enforced_tls**](SettingsEnforcedTlsApi.md#p_atch_user_settings_enforced_tls) | **PATCH** /user/settings/enforced_tls | Update Enforced TLS settings



## g_et_user_settings_enforced_tls

> crate::models::EnforcedTlsRequestResponse g_et_user_settings_enforced_tls(on_behalf_of)
Retrieve current Enforced TLS settings.

**This endpoint allows you to retrieve your current Enforced TLS settings.**  The Enforced TLS settings specify whether or not the recipient is required to support TLS or have a valid certificate.  If either `require_tls` or `require_valid_cert` is set to `true`, the recipient must support TLS 1.1 or higher or have a valid certificate. If these conditions are not met, Twilio SendGrid will drop the message and send a block event with “TLS required but not supported” as the description.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::EnforcedTlsRequestResponse**](enforced-tls-request-response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_user_settings_enforced_tls

> crate::models::EnforcedTlsRequestResponse p_atch_user_settings_enforced_tls(on_behalf_of, enforced_tls_request_response)
Update Enforced TLS settings

**This endpoint allows you to update your Enforced TLS settings.**  To require TLS from recipients, set `require_tls` to `true`. If either `require_tls` or `require_valid_cert` is set to `true`, the recipient must support TLS 1.1 or higher or have a valid certificate. If these conditions are not met, Twilio SendGrid will drop the message and send a block event with “TLS required but not supported” as the description.  > Twilio SendGrid supports TLS 1.1 and higher and does not support older versions of TLS due to security vulnerabilities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**enforced_tls_request_response** | Option<[**EnforcedTlsRequestResponse**](EnforcedTlsRequestResponse.md)> |  |  |

### Return type

[**crate::models::EnforcedTlsRequestResponse**](enforced-tls-request-response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

