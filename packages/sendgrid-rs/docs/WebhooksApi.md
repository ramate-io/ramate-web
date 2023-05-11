# \WebhooksApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**g_et_user_webhooks_event_settings**](WebhooksApi.md#g_et_user_webhooks_event_settings) | **GET** /user/webhooks/event/settings | Retrieve Event Webhook settings
[**g_et_user_webhooks_event_settings_signed**](WebhooksApi.md#g_et_user_webhooks_event_settings_signed) | **GET** /user/webhooks/event/settings/signed | Retrieve Signed Webhook Public Key
[**g_et_user_webhooks_parse_settings**](WebhooksApi.md#g_et_user_webhooks_parse_settings) | **GET** /user/webhooks/parse/settings | Retrieve all parse settings
[**g_et_user_webhooks_parse_stats**](WebhooksApi.md#g_et_user_webhooks_parse_stats) | **GET** /user/webhooks/parse/stats | Retrieves Inbound Parse Webhook statistics.
[**p_atch_user_webhooks_event_settings**](WebhooksApi.md#p_atch_user_webhooks_event_settings) | **PATCH** /user/webhooks/event/settings | Update Event Notification Settings
[**p_atch_user_webhooks_event_settings_signed**](WebhooksApi.md#p_atch_user_webhooks_event_settings_signed) | **PATCH** /user/webhooks/event/settings/signed | Enable/Disable Signed Webhook
[**p_ost_user_webhooks_event_test**](WebhooksApi.md#p_ost_user_webhooks_event_test) | **POST** /user/webhooks/event/test | Test Event Notification Settings



## g_et_user_webhooks_event_settings

> crate::models::EventWebhookResponse g_et_user_webhooks_event_settings(on_behalf_of)
Retrieve Event Webhook settings

**This endpoint allows you to retrieve your current event webhook settings.**  If an event type is marked as `true`, then the event webhook will include information about that event.  SendGrid’s Event Webhook will notify a URL of your choice via HTTP POST with information about events that occur as SendGrid processes your email.  Common uses of this data are to remove unsubscribes, react to spam reports, determine unengaged recipients, identify bounced email addresses, or create advanced analytics of your email program.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::EventWebhookResponse**](event-webhook-response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_user_webhooks_event_settings_signed

> crate::models::GetUserWebhooksEventSettingsSigned200Response g_et_user_webhooks_event_settings_signed(on_behalf_of)
Retrieve Signed Webhook Public Key

**This endpoint allows you to retrieve your signed webhook's public key.**  Once you have enabled signing of the Event Webhook, you will need the public key provided to verify the signatures on requests coming from Twilio SendGrid. You can retrieve the public key from this endpoint at any time.  For more information about cryptographically signing the Event Webhook, see [Getting Started with the Event Webhook Security Features](https://sendgrid.com/docs/for-developers/tracking-events/getting-started-event-webhook-security-features).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetUserWebhooksEventSettingsSigned200Response**](GET_user_webhooks_event_settings_signed_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_user_webhooks_parse_settings

> crate::models::GetUserWebhooksParseSettings200Response g_et_user_webhooks_parse_settings(on_behalf_of)
Retrieve all parse settings

**This endpoint allows you to retrieve all of your current inbound parse settings.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetUserWebhooksParseSettings200Response**](GET_user_webhooks_parse_settings_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_user_webhooks_parse_stats

> serde_json::Value g_et_user_webhooks_parse_stats(start_date, limit, offset, aggregated_by, end_date, on_behalf_of)
Retrieves Inbound Parse Webhook statistics.

**This endpoint allows you to retrieve the statistics for your Parse Webhook useage.**  SendGrid's Inbound Parse Webhook allows you to parse the contents and attachments of incomming emails. The Parse API can then POST the parsed emails to a URL that you specify. The Inbound Parse Webhook cannot parse messages greater than 30MB in size, including all attachments.  There are a number of pre-made integrations for the SendGrid Parse Webhook which make processing events easy. You can find these integrations in the [Library Index](https://sendgrid.com/docs/Integrate/libraries.html#-Webhook-Libraries).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | [**serde_json::Value**](.md) | The starting date of the statistics you want to retrieve. Must be in the format YYYY-MM-DD | [required] |
**limit** | Option<[**serde_json::Value**](.md)> | The number of statistics to return on each page. |  |
**offset** | Option<[**serde_json::Value**](.md)> | The number of statistics to skip. |  |
**aggregated_by** | Option<[**serde_json::Value**](.md)> | How you would like the statistics to by grouped.  |  |
**end_date** | Option<[**serde_json::Value**](.md)> | The end date of the statistics you want to retrieve. Must be in the format YYYY-MM-DD |  |[default to The day the request is made.]
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_user_webhooks_event_settings

> crate::models::EventWebhookResponse p_atch_user_webhooks_event_settings(on_behalf_of, event_webhook_update_oauth_request)
Update Event Notification Settings

**This endpoint allows you to update your current event webhook settings.**  If an event type is marked as `true`, then the event webhook will include information about that event.  SendGrid’s Event Webhook will notify a URL of your choice via HTTP POST with information about events that occur as SendGrid processes your email.  Common uses of this data are to remove unsubscribes, react to spam reports, determine unengaged recipients, identify bounced email addresses, or create advanced analytics of your email program.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**event_webhook_update_oauth_request** | Option<[**EventWebhookUpdateOauthRequest**](EventWebhookUpdateOauthRequest.md)> |  |  |

### Return type

[**crate::models::EventWebhookResponse**](event-webhook-response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_user_webhooks_event_settings_signed

> crate::models::GetUserWebhooksEventSettingsSigned200Response p_atch_user_webhooks_event_settings_signed(on_behalf_of, patch_user_webhooks_event_settings_signed_request)
Enable/Disable Signed Webhook

**This endpoint allows you to enable or disable signing of the Event Webhook.**  This endpoint takes a single boolean request parameter, `enabled`. You may either enable or disable signing of the Event Webhook using this endpoint. Once enabled, you can retrieve your public key using the `/webhooks/event/settings/signed` endpoint.  For more information about cryptographically signing the Event Webhook, see [Getting Started with the Event Webhook Security Features](https://sendgrid.com/docs/for-developers/tracking-events/getting-started-event-webhook-security-features).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**patch_user_webhooks_event_settings_signed_request** | Option<[**PatchUserWebhooksEventSettingsSignedRequest**](PatchUserWebhooksEventSettingsSignedRequest.md)> |  |  |

### Return type

[**crate::models::GetUserWebhooksEventSettingsSigned200Response**](GET_user_webhooks_event_settings_signed_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_user_webhooks_event_test

> p_ost_user_webhooks_event_test(on_behalf_of, post_user_webhooks_event_test_request)
Test Event Notification Settings

**This endpoint allows you to test your event webhook by sending a fake event notification post to the provided URL.**  SendGrid’s Event Webhook will notify a URL of your choice via HTTP POST with information about events that occur as SendGrid processes your email.  Common uses of this data are to remove unsubscribes, react to spam reports, determine unengaged recipients, identify bounced email addresses, or create advanced analytics of your email program.  >**Tip**: Retry logic for this endpoint differs from other endpoints, which use a rolling 24-hour retry.  If your web server does not return a 2xx response type, we will retry a POST request until we receive a 2xx response or the maximum time of 10 minutes has expired.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**post_user_webhooks_event_test_request** | Option<[**PostUserWebhooksEventTestRequest**](PostUserWebhooksEventTestRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

