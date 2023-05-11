# ContactImport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**serde_json::Value**](.md)> | The job ID. | [optional]
**status** | Option<[**serde_json::Value**](.md)> | The job state. Allowed values: `pending`, `completed`, `errored`, or `failed`. | [optional]
**job_type** | Option<[**serde_json::Value**](.md)> | The job type. Allowed values: `upsert`, or `delete`. | [optional]
**results** | Option<[**crate::models::ContactImportResults**](contact_import_results.md)> |  | [optional]
**started_at** | Option<[**serde_json::Value**](.md)> | The ISO8601 timestamp when the job was created. | [optional]
**finished_at** | Option<[**serde_json::Value**](.md)> | The ISO8601 timestamp when the job was finished. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


