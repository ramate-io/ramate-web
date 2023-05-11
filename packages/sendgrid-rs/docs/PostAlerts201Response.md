# PostAlerts201Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<[**serde_json::Value**](.md)> | A Unix timestamp indicating when the alert was created. | 
**email_to** | Option<[**serde_json::Value**](.md)> | The email address that the alert will be sent to. | 
**frequency** | Option<[**serde_json::Value**](.md)> | If the alert is of type stats_notification, this indicates how frequently the stats notifications will be sent. For example, \"daily\", \"weekly\", or \"monthly\". | [optional]
**id** | Option<[**serde_json::Value**](.md)> | The ID of the alert. | 
**r#type** | Option<[**serde_json::Value**](.md)> | The type of alert. | 
**updated_at** | Option<[**serde_json::Value**](.md)> | A Unix timestamp indicating when the alert was last modified. | 
**percentage** | Option<[**serde_json::Value**](.md)> | \"If the alert is of type usage_limit, this indicates the percentage of email usage that must be reached before the alert will be sent. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


