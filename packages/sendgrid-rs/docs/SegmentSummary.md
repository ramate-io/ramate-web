# SegmentSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**serde_json::Value**](.md)> |  | 
**contacts_count** | Option<[**serde_json::Value**](.md)> |  | 
**created_at** | Option<[**serde_json::Value**](.md)> | ISO8601 of created timestamp  | 
**name** | Option<[**serde_json::Value**](.md)> |  | [optional]
**parent_list_id** | Option<[**serde_json::Value**](.md)> | The id of the list if this segment is a child of a list.  This implies the query `AND CONTAINS(list_ids, ${parent_list_id})` | [optional]
**sample_updated_at** | Option<[**serde_json::Value**](.md)> | ISO8601 timestamp the sample was last updated | 
**updated_at** | Option<[**serde_json::Value**](.md)> | ISO8601 timestamp the object was last updated | 
**next_sample_update** | Option<[**serde_json::Value**](.md)> | ISO8601 string that is equal to `sample_updated_at` plus an internally calculated offset that depends on how often contacts enter or exit segments as the scheduled pipeline updates the samples. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


