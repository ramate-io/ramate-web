# \SettingsTrackingApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**g_et_tracking_settings**](SettingsTrackingApi.md#g_et_tracking_settings) | **GET** /tracking_settings | Retrieve Tracking Settings
[**g_et_tracking_settings_click**](SettingsTrackingApi.md#g_et_tracking_settings_click) | **GET** /tracking_settings/click | Retrieve Click Track Settings
[**g_et_tracking_settings_google_analytics**](SettingsTrackingApi.md#g_et_tracking_settings_google_analytics) | **GET** /tracking_settings/google_analytics | Retrieve Google Analytics Settings
[**g_et_tracking_settings_open**](SettingsTrackingApi.md#g_et_tracking_settings_open) | **GET** /tracking_settings/open | Get Open Tracking Settings
[**g_et_tracking_settings_subscription**](SettingsTrackingApi.md#g_et_tracking_settings_subscription) | **GET** /tracking_settings/subscription | Retrieve Subscription Tracking Settings
[**p_atch_tracking_settings_click**](SettingsTrackingApi.md#p_atch_tracking_settings_click) | **PATCH** /tracking_settings/click | Update Click Tracking Settings
[**p_atch_tracking_settings_google_analytics**](SettingsTrackingApi.md#p_atch_tracking_settings_google_analytics) | **PATCH** /tracking_settings/google_analytics | Update Google Analytics Settings
[**p_atch_tracking_settings_open**](SettingsTrackingApi.md#p_atch_tracking_settings_open) | **PATCH** /tracking_settings/open | Update Open Tracking Settings
[**p_atch_tracking_settings_subscription**](SettingsTrackingApi.md#p_atch_tracking_settings_subscription) | **PATCH** /tracking_settings/subscription | Update Subscription Tracking Settings



## g_et_tracking_settings

> crate::models::GetTrackingSettings200Response g_et_tracking_settings(on_behalf_of)
Retrieve Tracking Settings

**This endpoint allows you to retrieve a list of all tracking settings on your account.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetTrackingSettings200Response**](GET_tracking_settings_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_tracking_settings_click

> crate::models::ClickTracking g_et_tracking_settings_click(on_behalf_of)
Retrieve Click Track Settings

**This endpoint allows you to retrieve your current click tracking setting.**  Click Tracking overrides all the links and URLs in your emails and points them to either SendGrid’s servers or the domain with which you branded your link. When a customer clicks a link, SendGrid tracks those [clicks](https://sendgrid.com/docs/glossary/clicks/).  Click tracking helps you understand how users are engaging with your communications. SendGrid can track up to 1000 links per email

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::ClickTracking**](click-tracking.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_tracking_settings_google_analytics

> crate::models::GoogleAnalyticsSettings g_et_tracking_settings_google_analytics(on_behalf_of)
Retrieve Google Analytics Settings

**This endpoint allows you to retrieve your current setting for Google Analytics.**   Google Analytics helps you understand how users got to your site and what they're doing there. For more information about using Google Analytics, please refer to [Google’s URL Builder](https://support.google.com/analytics/answer/1033867?hl=en) and their article on [\"Best Practices for Campaign Building\"](https://support.google.com/analytics/answer/1037445).  We default the settings to Google’s recommendations. For more information, see [Google Analytics Demystified](https://sendgrid.com/docs/ui/analytics-and-reporting/google-analytics/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GoogleAnalyticsSettings**](google_analytics_settings.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_tracking_settings_open

> crate::models::GetTrackingSettingsOpen200Response g_et_tracking_settings_open(on_behalf_of)
Get Open Tracking Settings

**This endpoint allows you to retrieve your current settings for open tracking.**  Open Tracking adds an invisible image at the end of the email which can track email opens.  If the email recipient has images enabled on their email client, a request to SendGrid’s server for the invisible image is executed and an open event is logged.  These events are logged in the Statistics portal, Email Activity interface, and are reported by the Event Webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetTrackingSettingsOpen200Response**](GET_tracking_settings_open_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_tracking_settings_subscription

> crate::models::SubscriptionTrackingSettings g_et_tracking_settings_subscription(on_behalf_of)
Retrieve Subscription Tracking Settings

**This endpoint allows you to retrieve your current settings for subscription tracking.**  Subscription tracking adds links to the bottom of your emails that allows your recipients to subscribe to, or unsubscribe from, your emails.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::SubscriptionTrackingSettings**](subscription_tracking_settings.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_tracking_settings_click

> crate::models::ClickTracking p_atch_tracking_settings_click(on_behalf_of, patch_tracking_settings_click_request)
Update Click Tracking Settings

**This endpoint allows you to enable or disable your current click tracking setting.**  Click Tracking overrides all the links and URLs in your emails and points them to either SendGrid’s servers or the domain with which you branded your link. When a customer clicks a link, SendGrid tracks those [clicks](https://sendgrid.com/docs/glossary/clicks/).  Click tracking helps you understand how users are engaging with your communications. SendGrid can track up to 1000 links per email

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**patch_tracking_settings_click_request** | Option<[**PatchTrackingSettingsClickRequest**](PatchTrackingSettingsClickRequest.md)> |  |  |

### Return type

[**crate::models::ClickTracking**](click-tracking.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_tracking_settings_google_analytics

> crate::models::GoogleAnalyticsSettings p_atch_tracking_settings_google_analytics(on_behalf_of, google_analytics_settings)
Update Google Analytics Settings

**This endpoint allows you to update your current setting for Google Analytics.**  Google Analytics helps you understand how users got to your site and what they're doing there. For more information about using Google Analytics, please refer to [Google’s URL Builder](https://support.google.com/analytics/answer/1033867?hl=en) and their article on [\"Best Practices for Campaign Building\"](https://support.google.com/analytics/answer/1037445).  We default the settings to Google’s recommendations. For more information, see [Google Analytics Demystified](https://sendgrid.com/docs/ui/analytics-and-reporting/google-analytics/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**google_analytics_settings** | Option<[**GoogleAnalyticsSettings**](GoogleAnalyticsSettings.md)> |  |  |

### Return type

[**crate::models::GoogleAnalyticsSettings**](google_analytics_settings.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_tracking_settings_open

> crate::models::GetTrackingSettingsOpen200Response p_atch_tracking_settings_open(on_behalf_of, patch_tracking_settings_open_request)
Update Open Tracking Settings

**This endpoint allows you to update your current settings for open tracking.**  Open Tracking adds an invisible image at the end of the email which can track email opens.  If the email recipient has images enabled on their email client, a request to SendGrid’s server for the invisible image is executed and an open event is logged.  These events are logged in the Statistics portal, Email Activity interface, and are reported by the Event Webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**patch_tracking_settings_open_request** | Option<[**PatchTrackingSettingsOpenRequest**](PatchTrackingSettingsOpenRequest.md)> |  |  |

### Return type

[**crate::models::GetTrackingSettingsOpen200Response**](GET_tracking_settings_open_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_tracking_settings_subscription

> crate::models::SubscriptionTrackingSettings p_atch_tracking_settings_subscription(on_behalf_of, subscription_tracking_settings)
Update Subscription Tracking Settings

**This endpoint allows you to update your current settings for subscription tracking.**  Subscription tracking adds links to the bottom of your emails that allows your recipients to subscribe to, or unsubscribe from, your emails.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**subscription_tracking_settings** | Option<[**SubscriptionTrackingSettings**](SubscriptionTrackingSettings.md)> |  |  |

### Return type

[**crate::models::SubscriptionTrackingSettings**](subscription_tracking_settings.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

