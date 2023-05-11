# BounceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<[**serde_json::Value**](.md)> | The unix timestamp for when the bounce record was created at SendGrid. | [optional]
**email** | Option<[**serde_json::Value**](.md)> | The email address that was added to the bounce list. | [optional]
**reason** | Option<[**serde_json::Value**](.md)> | The reason for the bounce. This typically will be a bounce code, an enhanced code, and a description. | [optional]
**status** | Option<[**serde_json::Value**](.md)> | Enhanced SMTP bounce response | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


