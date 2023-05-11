# SegmentWriteV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<[**serde_json::Value**](.md)> | Name of the segment. | 
**parent_list_ids** | Option<[**serde_json::Value**](.md)> | The array of list ids to filter contacts on when building this segment. It allows only one such list id for now. We will support more in future | [optional]
**query_dsl** | Option<[**serde_json::Value**](.md)> | SQL query which will filter contacts based on the conditions provided | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


