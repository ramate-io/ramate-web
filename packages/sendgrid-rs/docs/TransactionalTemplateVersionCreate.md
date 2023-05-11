# TransactionalTemplateVersionCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | Option<[**serde_json::Value**](serde_json::Value.md)> | Set the version as the active version associated with the template (0 is inactive, 1 is active). Only one version of a template can be active. The first version created for a template will automatically be set to Active. | [optional]
**name** | Option<[**serde_json::Value**](.md)> | Name of the transactional template version. | 
**html_content** | Option<[**serde_json::Value**](.md)> | The HTML content of the version. Maximum of 1048576 bytes allowed. | [optional]
**plain_content** | Option<[**serde_json::Value**](.md)> | Text/plain content of the transactional template version. Maximum of 1048576 bytes allowed. | [optional][default to <generated from html_content if left empty>]
**generate_plain_content** | Option<[**serde_json::Value**](.md)> | If true, plain_content is always generated from html_content. If false, plain_content is not altered. | [optional][default to true]
**subject** | Option<[**serde_json::Value**](.md)> | Subject of the new transactional template version. | 
**editor** | Option<[**serde_json::Value**](serde_json::Value.md)> | The editor used in the UI. | [optional]
**test_data** | Option<[**serde_json::Value**](.md)> | For dynamic templates only, the mock json data that will be used for template preview and test sends. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


