# ContactdbSegments

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<[**serde_json::Value**](.md)> | The name of this segment. | 
**list_id** | Option<[**serde_json::Value**](.md)> | The list id from which to make this segment. Not including this ID will mean your segment is created from the main contactdb rather than a list. | [optional]
**conditions** | Option<[**serde_json::Value**](.md)> | The conditions for a recipient to be included in this segment. | 
**recipient_count** | Option<[**serde_json::Value**](.md)> | The count of recipients in this list. This is not included on creation of segments. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


