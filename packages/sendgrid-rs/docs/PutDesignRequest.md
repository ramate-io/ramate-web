# PutDesignRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<[**serde_json::Value**](.md)> | Name of the Design. | [optional][default to My Design]
**html_content** | Option<[**serde_json::Value**](.md)> | The HTML content of the Design. | [optional]
**plain_content** | Option<[**serde_json::Value**](.md)> | Plain text content of the Design. | [optional][default to <generated from html_content if left empty>]
**generate_plain_content** | Option<[**serde_json::Value**](.md)> | If true, plain_content is always generated from html_content. If false, plain_content is not altered. | [optional][default to true]
**subject** | Option<[**serde_json::Value**](.md)> | Subject of the Design. | [optional]
**categories** | Option<[**serde_json::Value**](.md)> | The list of categories applied to the design | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


