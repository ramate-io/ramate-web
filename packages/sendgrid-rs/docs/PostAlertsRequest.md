# PostAlertsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<[**serde_json::Value**](serde_json::Value.md)> | The type of alert you want to create. Can be either usage_limit or stats_notification. Example: usage_limit | 
**email_to** | Option<[**serde_json::Value**](.md)> | The email address the alert will be sent to. Example: test@example.com | 
**frequency** | Option<[**serde_json::Value**](.md)> | Required for stats_notification. How frequently the alert will be sent. Example: daily | [optional]
**percentage** | Option<[**serde_json::Value**](.md)> | Required for usage_alert. When this usage threshold is reached, the alert will be sent. Example: 90 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


