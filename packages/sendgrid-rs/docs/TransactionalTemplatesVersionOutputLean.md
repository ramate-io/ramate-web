# TransactionalTemplatesVersionOutputLean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**serde_json::Value**](.md)> | ID of the transactional template version. | [optional]
**template_id** | Option<[**serde_json::Value**](.md)> | ID of the transactional template. | [optional]
**active** | Option<[**serde_json::Value**](serde_json::Value.md)> | Set the version as the active version associated with the template. Only one version of a template can be active. The first version created for a template will automatically be set to Active. | [optional]
**name** | Option<[**serde_json::Value**](.md)> | Name of the transactional template version. | [optional]
**subject** | Option<[**serde_json::Value**](.md)> | Subject of the new transactional template version. | [optional]
**updated_at** | Option<[**serde_json::Value**](.md)> | The date and time that this transactional template version was updated. | [optional]
**generate_plain_content** | Option<[**serde_json::Value**](.md)> | If true, plain_content is always generated from html_content. If false, plain_content is not altered. | [optional][default to true]
**editor** | Option<[**serde_json::Value**](serde_json::Value.md)> | The editor used in the UI. | [optional]
**thumbnail_url** | Option<[**serde_json::Value**](.md)> | A Thumbnail preview of the template's html content. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


