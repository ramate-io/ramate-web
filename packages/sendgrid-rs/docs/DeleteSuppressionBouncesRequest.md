# DeleteSuppressionBouncesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delete_all** | Option<[**serde_json::Value**](.md)> | This parameter allows you to delete **every** email in your bounce list. This should not be used with the emails parameter. | [optional]
**emails** | Option<[**serde_json::Value**](.md)> | Delete multiple emails from your bounce list at the same time. This should not be used with the delete_all parameter. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


