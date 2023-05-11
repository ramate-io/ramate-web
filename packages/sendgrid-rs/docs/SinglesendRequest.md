# SinglesendRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<[**serde_json::Value**](.md)> | The name of the Single Send. | 
**categories** | Option<[**serde_json::Value**](.md)> | The categories to associate with this Single Send. | [optional]
**send_at** | Option<[**serde_json::Value**](.md)> | The ISO 8601 time at which to send the Single Send — this must be set for a future time. | [optional]
**send_to** | Option<[**crate::models::SinglesendRequestSendTo**](singlesend_request_send_to.md)> |  | [optional]
**email_config** | Option<[**crate::models::SinglesendRequestEmailConfig**](singlesend_request_email_config.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


