# PostMcContactsExportsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**list_ids** | Option<[**serde_json::Value**](.md)> | IDs of the contact lists you want to export. | [optional]
**segment_ids** | Option<[**serde_json::Value**](.md)> | IDs of the contact segments you want to export. | [optional]
**notifications** | Option<[**crate::models::PostMcContactsExportsRequestNotifications**](POST_mc_contacts_exports_request_notifications.md)> |  | [optional]
**file_type** | Option<[**serde_json::Value**](serde_json::Value.md)> | File type for export file. Choose from `json` or `csv`. | [optional][default to Csv]
**max_file_size** | Option<[**serde_json::Value**](.md)> | The maximum size of an export file in MB. Note that when this option is specified, multiple output files may be returned from the export. | [optional][default to 5000]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


