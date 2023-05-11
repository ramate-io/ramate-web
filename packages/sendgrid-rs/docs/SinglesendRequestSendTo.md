# SinglesendRequestSendTo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**list_ids** | Option<[**serde_json::Value**](.md)> | The recipient List IDs that will receive the Single Send. | [optional]
**segment_ids** | Option<[**serde_json::Value**](.md)> | The recipient Segment IDs that will receive the Single Send. | [optional]
**all** | Option<[**serde_json::Value**](.md)> | Set to `true` to send to All Contacts. If set to `false`, at least one `list_ids` or `segment_ids` value must be provided before the Single Send is scheduled to be sent to recipients. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


