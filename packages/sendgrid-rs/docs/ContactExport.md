# ContactExport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**serde_json::Value**](.md)> |  | 
**status** | Option<[**serde_json::Value**](serde_json::Value.md)> | The export job's status. Allowed values: `pending`, `ready`, or `failure`. | 
**created_at** | Option<[**serde_json::Value**](.md)> | The ISO8601 timestamp when the export was begun. | 
**updated_at** | Option<[**serde_json::Value**](.md)> | The ISO8601 timestamp when the export was updated. | 
**completed_at** | Option<[**serde_json::Value**](.md)> | The ISO8601 timestamp when the export was completed. | [optional]
**expires_at** | Option<[**serde_json::Value**](.md)> | The ISO8601 timestamp when the exported file on S3 will expire. | 
**urls** | Option<[**serde_json::Value**](.md)> | One or more download URLs for the contact file if the status is `ready`. | [optional]
**message** | Option<[**serde_json::Value**](.md)> | A human readable message if the status is `failure`. | [optional]
**_metadata** | Option<[**crate::models::Metadata**](metadata.md)> |  | [optional]
**contact_count** | Option<[**serde_json::Value**](.md)> | The total number of exported contacts. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


