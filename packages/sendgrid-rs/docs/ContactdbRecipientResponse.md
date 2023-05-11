# ContactdbRecipientResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error_count** | Option<[**serde_json::Value**](.md)> | The number of errors found while adding recipients. | [default to 0]
**error_indices** | Option<[**serde_json::Value**](.md)> | The indices of the recipient(s) sent that caused the error.  | [optional][default to []]
**new_count** | Option<[**serde_json::Value**](.md)> | The count of new recipients added to the contactdb. | [default to 0]
**persisted_recipients** | Option<[**serde_json::Value**](.md)> | The recipient IDs of the recipients that already existed from this request. | [default to []]
**updated_count** | Option<[**serde_json::Value**](.md)> | The recipients who were updated from this request. | [default to 0]
**errors** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


