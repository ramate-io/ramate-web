# \SettingsPartnerApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**g_et_partner_settings**](SettingsPartnerApi.md#g_et_partner_settings) | **GET** /partner_settings | Returns a list of all partner settings.
[**g_et_partner_settings_new_relic**](SettingsPartnerApi.md#g_et_partner_settings_new_relic) | **GET** /partner_settings/new_relic | Returns all New Relic partner settings.
[**p_atch_partner_settings_new_relic**](SettingsPartnerApi.md#p_atch_partner_settings_new_relic) | **PATCH** /partner_settings/new_relic | Updates New Relic partner settings.



## g_et_partner_settings

> crate::models::GetPartnerSettings200Response g_et_partner_settings(limit, offset, on_behalf_of)
Returns a list of all partner settings.

**This endpoint allows you to retrieve a list of all partner settings that you can enable.**  Our partner settings allow you to integrate your SendGrid account with our partners to increase your SendGrid experience and functionality. For more information about our partners, and how you can begin integrating with them, please visit our [Partners documentation](https://sendgrid.com/docs/ui/account-and-settings/partners/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<[**serde_json::Value**](.md)> | The number of settings to return per page. |  |
**offset** | Option<[**serde_json::Value**](.md)> | The paging offset. |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetPartnerSettings200Response**](GET_partner_settings_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_partner_settings_new_relic

> crate::models::PartnerSettingsNewRelic g_et_partner_settings_new_relic(on_behalf_of)
Returns all New Relic partner settings.

**This endpoint allows you to retrieve your current New Relic partner settings.**  Our partner settings allow you to integrate your SendGrid account with our partners to increase your SendGrid experience and functionality. For more information about our partners, and how you can begin integrating with them, please visit our [Partners documentation](https://sendgrid.com/docs/ui/account-and-settings/partners/).  By integrating with New Relic, you can send your SendGrid email statistics to your New Relic Dashboard. If you enable this setting, your stats will be sent to New Relic every 5 minutes. You will need your New Relic License Key to enable this setting. For more information, please see our [SendGrid for New Relic documentation](https://sendgrid.com/docs/ui/analytics-and-reporting/tracking-stats-using-new-relic/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::PartnerSettingsNewRelic**](partner_settings_new_relic.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_partner_settings_new_relic

> crate::models::PartnerSettingsNewRelic p_atch_partner_settings_new_relic(on_behalf_of, patch_partner_settings_new_relic_request)
Updates New Relic partner settings.

**This endpoint allows you to update or change your New Relic partner settings.**  Our partner settings allow you to integrate your SendGrid account with our partners to increase your SendGrid experience and functionality. For more information about our partners, and how you can begin integrating with them, please visit our [Partners documentation](https://sendgrid.com/docs/ui/account-and-settings/partners/).  By integrating with New Relic, you can send your SendGrid email statistics to your New Relic Dashboard. If you enable this setting, your stats will be sent to New Relic every 5 minutes. You will need your New Relic License Key to enable this setting. For more information, please see our [SendGrid for New Relic documentation](https://sendgrid.com/docs/ui/analytics-and-reporting/tracking-stats-using-new-relic/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**patch_partner_settings_new_relic_request** | Option<[**PatchPartnerSettingsNewRelicRequest**](PatchPartnerSettingsNewRelicRequest.md)> |  |  |

### Return type

[**crate::models::PartnerSettingsNewRelic**](partner_settings_new_relic.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

