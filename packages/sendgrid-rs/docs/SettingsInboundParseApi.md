# \SettingsInboundParseApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_user_webhooks_parse_settings_hostname**](SettingsInboundParseApi.md#d_elete_user_webhooks_parse_settings_hostname) | **DELETE** /user/webhooks/parse/settings/{hostname} | Delete a parse setting
[**g_et_user_webhooks_parse_settings_hostname**](SettingsInboundParseApi.md#g_et_user_webhooks_parse_settings_hostname) | **GET** /user/webhooks/parse/settings/{hostname} | Retrieve a specific parse setting
[**p_atch_user_webhooks_parse_settings_hostname**](SettingsInboundParseApi.md#p_atch_user_webhooks_parse_settings_hostname) | **PATCH** /user/webhooks/parse/settings/{hostname} | Update a parse setting
[**p_ost_user_webhooks_parse_settings**](SettingsInboundParseApi.md#p_ost_user_webhooks_parse_settings) | **POST** /user/webhooks/parse/settings | Create a parse setting



## d_elete_user_webhooks_parse_settings_hostname

> serde_json::Value d_elete_user_webhooks_parse_settings_hostname(hostname, on_behalf_of)
Delete a parse setting

**This endpoint allows you to delete a specific inbound parse setting by hostname.**  You can retrieve all your Inbound Parse settings and their associated host names with the \"Retrieve all parse settings\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hostname** | [**serde_json::Value**](.md) | The hostname associated with the inbound parse setting that you would like to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_user_webhooks_parse_settings_hostname

> crate::models::ParseSetting g_et_user_webhooks_parse_settings_hostname(hostname, on_behalf_of)
Retrieve a specific parse setting

**This endpoint allows you to retrieve a specific inbound parse setting by hostname.**  You can retrieve all your Inbound Parse settings and their associated host names with the \"Retrieve all parse settings\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hostname** | [**serde_json::Value**](.md) | The hostname associated with the inbound parse setting that you would like to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::ParseSetting**](parse-setting.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_user_webhooks_parse_settings_hostname

> crate::models::ParseSetting p_atch_user_webhooks_parse_settings_hostname(hostname, on_behalf_of, parse_setting)
Update a parse setting

**This endpoint allows you to update a specific inbound parse setting by hostname.**  You can retrieve all your Inbound Parse settings and their associated host names with the \"Retrieve all parse settings\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hostname** | [**serde_json::Value**](.md) | The hostname associated with the inbound parse setting that you would like to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**parse_setting** | Option<[**ParseSetting**](ParseSetting.md)> |  |  |

### Return type

[**crate::models::ParseSetting**](parse-setting.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_user_webhooks_parse_settings

> crate::models::ParseSetting p_ost_user_webhooks_parse_settings(on_behalf_of, parse_setting)
Create a parse setting

**This endpoint allows you to create a new inbound parse setting.**  Creating an Inbound Parse setting requires two pieces of information: a `url` and a `hostname`.  The `hostname` must correspond to a domain authenticated by Twilio SendGrid on your account. If you need to complete domain authentication, you can use the [Twilio SendGrid App](https://app.sendgrid.com/settings/sender_auth) or the \"Authenticate a domain\" endpoint. See \"[How to Set Up Domain Authentication](https://sendgrid.com/docs/ui/account-and-settings/how-to-set-up-domain-authentication/)\" for instructions.  Any email received by the `hostname` will be parsed when you complete this setup. You must also add a Twilio SendGrid MX record to this domain's DNS records. See \"[Setting up the Inbound Parse Webhook](https://sendgrid.com/docs/for-developers/parsing-email/setting-up-the-inbound-parse-webhook/)\" for full instructions.  The `url` represents a location where the parsed message data will be delivered. Twilio SendGrid will make an HTTP POST request to this `url` with the message data. The `url` must be publicly reachable, and your application must return a `200` status code to signal that the message data has been received.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**parse_setting** | Option<[**ParseSetting**](ParseSetting.md)> |  |  |

### Return type

[**crate::models::ParseSetting**](parse-setting.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

