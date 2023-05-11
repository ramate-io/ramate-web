# CancelOrPauseAScheduledSendRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**batch_id** | Option<[**serde_json::Value**](.md)> | The batch ID is the identifier that your scheduled mail sends share. | 
**status** | Option<[**serde_json::Value**](serde_json::Value.md)> | The status of the send you would like to implement. This can be pause or cancel. To delete a pause or cancel status see DELETE /v3/user/scheduled_sends/{batch_id} | [default to Pause]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


