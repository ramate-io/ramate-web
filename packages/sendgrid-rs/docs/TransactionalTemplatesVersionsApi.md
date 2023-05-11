# \TransactionalTemplatesVersionsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_templates_template_id_versions_version_id**](TransactionalTemplatesVersionsApi.md#d_elete_templates_template_id_versions_version_id) | **DELETE** /templates/{template_id}/versions/{version_id} | Delete a transactional template version.
[**g_et_templates_template_id_versions_version_id**](TransactionalTemplatesVersionsApi.md#g_et_templates_template_id_versions_version_id) | **GET** /templates/{template_id}/versions/{version_id} | Retrieve a specific transactional template version.
[**p_atch_templates_template_id_versions_version_id**](TransactionalTemplatesVersionsApi.md#p_atch_templates_template_id_versions_version_id) | **PATCH** /templates/{template_id}/versions/{version_id} | Edit a transactional template version.
[**p_ost_templates_template_id_versions**](TransactionalTemplatesVersionsApi.md#p_ost_templates_template_id_versions) | **POST** /templates/{template_id}/versions | Create a new transactional template version.
[**p_ost_templates_template_id_versions_version_id_activate**](TransactionalTemplatesVersionsApi.md#p_ost_templates_template_id_versions_version_id_activate) | **POST** /templates/{template_id}/versions/{version_id}/activate | Activate a transactional template version.



## d_elete_templates_template_id_versions_version_id

> d_elete_templates_template_id_versions_version_id(template_id, version_id, on_behalf_of)
Delete a transactional template version.

**This endpoint allows you to delete a transactional template version.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | [**serde_json::Value**](.md) |  The ID of the original template | [required] |
**version_id** | [**serde_json::Value**](.md) | The ID of the template version | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_templates_template_id_versions_version_id

> serde_json::Value g_et_templates_template_id_versions_version_id(template_id, version_id, on_behalf_of)
Retrieve a specific transactional template version.

**This endpoint allows you to retrieve a specific version of a template.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | [**serde_json::Value**](.md) |  The ID of the original template | [required] |
**version_id** | [**serde_json::Value**](.md) | The ID of the template version | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_templates_template_id_versions_version_id

> serde_json::Value p_atch_templates_template_id_versions_version_id(template_id, version_id, on_behalf_of, transactional_template_version_create)
Edit a transactional template version.

**This endpoint allows you to edit the content of your template version.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | [**serde_json::Value**](.md) |  The ID of the original template | [required] |
**version_id** | [**serde_json::Value**](.md) | The ID of the template version | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**transactional_template_version_create** | Option<[**TransactionalTemplateVersionCreate**](TransactionalTemplateVersionCreate.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_templates_template_id_versions

> serde_json::Value p_ost_templates_template_id_versions(template_id, on_behalf_of, transactional_template_version_create)
Create a new transactional template version.

**This endpoint allows you to create a new version of a template.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**transactional_template_version_create** | Option<[**TransactionalTemplateVersionCreate**](TransactionalTemplateVersionCreate.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_templates_template_id_versions_version_id_activate

> serde_json::Value p_ost_templates_template_id_versions_version_id_activate(template_id, version_id, on_behalf_of)
Activate a transactional template version.

**This endpoint allows you to activate a version of one of your templates.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | [**serde_json::Value**](.md) | The ID of the original template | [required] |
**version_id** | [**serde_json::Value**](.md) | The ID of the template version | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

