# AllSegmentsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**serde_json::Value**](.md)> | ID assigned to the segment when created. | 
**name** | Option<[**serde_json::Value**](.md)> | Name of the segment. | 
**contacts_count** | Option<[**serde_json::Value**](.md)> | Total number of contacts present in the segment | 
**created_at** | Option<[**serde_json::Value**](.md)> | ISO8601 timestamp of when the object was created | 
**updated_at** | Option<[**serde_json::Value**](.md)> | ISO8601 timestamp of when the object was last updated | 
**sample_updated_at** | Option<[**serde_json::Value**](.md)> | ISO8601 timestamp of when the samples were last updated | 
**next_sample_update** | Option<[**serde_json::Value**](.md)> | ISO8601 timestamp of when the samples will be next updated | 
**parent_list_ids** | Option<[**serde_json::Value**](.md)> | The array of list ids to filter contacts on when building this segment. It allows only one such list id for now. We will support more in future | 
**query_version** | Option<[**serde_json::Value**](.md)> | If not set, segment contains a query for use with Segment v1 APIs. If set to '2', segment contains a SQL query for use in v2. | 
**_metadata** | Option<[**crate::models::Metadata**](_metadata.md)> |  | [optional]
**status** | [**crate::models::SegmentStatusResponse**](segment_status_response.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


