# \AlertsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_alerts_alert_id**](AlertsApi.md#d_elete_alerts_alert_id) | **DELETE** /alerts/{alert_id} | Delete an alert
[**g_et_alerts**](AlertsApi.md#g_et_alerts) | **GET** /alerts | Retrieve all alerts
[**g_et_alerts_alert_id**](AlertsApi.md#g_et_alerts_alert_id) | **GET** /alerts/{alert_id} | Retrieve a specific alert
[**p_atch_alerts_alert_id**](AlertsApi.md#p_atch_alerts_alert_id) | **PATCH** /alerts/{alert_id} | Update an alert
[**p_ost_alerts**](AlertsApi.md#p_ost_alerts) | **POST** /alerts | Create a new Alert



## d_elete_alerts_alert_id

> serde_json::Value d_elete_alerts_alert_id(alert_id, on_behalf_of)
Delete an alert

**This endpoint allows you to delete an alert.**  Alerts allow you to specify an email address to receive notifications regarding your email usage or statistics.  * Usage alerts allow you to set the threshold at which an alert will be sent. * Stats notifications allow you to set how frequently you would like to receive email statistics reports. For example, \"daily\", \"weekly\", or \"monthly\".  For more information about alerts, please see our [Alerts documentation](https://sendgrid.com/docs/ui/account-and-settings/alerts/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | [**serde_json::Value**](.md) | The ID of the alert you would like to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_alerts

> serde_json::Value g_et_alerts(authorization, on_behalf_of)
Retrieve all alerts

**This endpoint allows you to retrieve all of your alerts.**  Alerts allow you to specify an email address to receive notifications regarding your email usage or statistics.  * Usage alerts allow you to set the threshold at which an alert will be sent. * Stats notifications allow you to set how frequently you would like to receive email statistics reports. For example, \"daily\", \"weekly\", or \"monthly\".  For more information about alerts, please see our [Alerts documentation](https://sendgrid.com/docs/ui/account-and-settings/alerts/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_alerts_alert_id

> crate::models::GetAlertsAlertId200Response g_et_alerts_alert_id(alert_id, authorization, on_behalf_of)
Retrieve a specific alert

**This endpoint allows you to retrieve a specific alert.**  Alerts allow you to specify an email address to receive notifications regarding your email usage or statistics.  * Usage alerts allow you to set the threshold at which an alert will be sent. * Stats notifications allow you to set how frequently you would like to receive email statistics reports. For example, \"daily\", \"weekly\", or \"monthly\".  For more information about alerts, please see our [Alerts documentation](https://sendgrid.com/docs/ui/account-and-settings/alerts/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | [**serde_json::Value**](.md) | The ID of the alert you would like to retrieve. | [required] |
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetAlertsAlertId200Response**](GET_alerts_alert_id_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_alerts_alert_id

> crate::models::GetAlertsAlertId200Response p_atch_alerts_alert_id(alert_id, on_behalf_of, patch_alerts_alert_id_request)
Update an alert

**This endpoint allows you to update an alert.**  Alerts allow you to specify an email address to receive notifications regarding your email usage or statistics.  * Usage alerts allow you to set the threshold at which an alert will be sent. * Stats notifications allow you to set how frequently you would like to receive email statistics reports. For example, \"daily\", \"weekly\", or \"monthly\".  For more information about alerts, please see our [Alerts documentation](https://sendgrid.com/docs/ui/account-and-settings/alerts/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | [**serde_json::Value**](.md) | The ID of the alert you would like to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**patch_alerts_alert_id_request** | Option<[**PatchAlertsAlertIdRequest**](PatchAlertsAlertIdRequest.md)> |  |  |

### Return type

[**crate::models::GetAlertsAlertId200Response**](GET_alerts_alert_id_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_alerts

> crate::models::PostAlerts201Response p_ost_alerts(authorization, on_behalf_of, post_alerts_request)
Create a new Alert

**This endpoint allows you to create a new alert.**  Alerts allow you to specify an email address to receive notifications regarding your email usage or statistics. There are two types of alerts that can be created with this endpoint:  * `usage_limit` allows you to set the threshold at which an alert will be sent. * `stats_notification` allows you to set how frequently you would like to receive email statistics reports. For example, \"daily\", \"weekly\", or \"monthly\".  For more information about alerts, please see our [Alerts documentation](https://sendgrid.com/docs/ui/account-and-settings/alerts/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |
**post_alerts_request** | Option<[**PostAlertsRequest**](PostAlertsRequest.md)> |  |  |

### Return type

[**crate::models::PostAlerts201Response**](POST_alerts_201_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

