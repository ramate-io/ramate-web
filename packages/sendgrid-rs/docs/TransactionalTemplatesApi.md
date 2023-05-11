# \TransactionalTemplatesApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_templates_template_id**](TransactionalTemplatesApi.md#d_elete_templates_template_id) | **DELETE** /templates/{template_id} | Delete a template.
[**g_et_templates**](TransactionalTemplatesApi.md#g_et_templates) | **GET** /templates | Retrieve paged transactional templates.
[**g_et_templates_template_id**](TransactionalTemplatesApi.md#g_et_templates_template_id) | **GET** /templates/{template_id} | Retrieve a single transactional template.
[**p_atch_templates_template_id**](TransactionalTemplatesApi.md#p_atch_templates_template_id) | **PATCH** /templates/{template_id} | Edit a transactional template.
[**p_ost_templates**](TransactionalTemplatesApi.md#p_ost_templates) | **POST** /templates | Create a transactional template.
[**p_ost_templates_template_id**](TransactionalTemplatesApi.md#p_ost_templates_template_id) | **POST** /templates/{template_id} | Duplicate a transactional template.



## d_elete_templates_template_id

> serde_json::Value d_elete_templates_template_id(template_id, on_behalf_of)
Delete a template.

**This endpoint allows you to delete a transactional template.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_templates

> crate::models::GetTemplates200Response g_et_templates(page_size, generations, page_token, on_behalf_of)
Retrieve paged transactional templates.

**This endpoint allows you to retrieve all transactional templates.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | [**serde_json::Value**](.md) | The number of templates to be returned in each page of results | [required] |
**generations** | Option<[**serde_json::Value**](.md)> | Comma-delimited list specifying which generations of templates to return. Options are `legacy`, `dynamic` or `legacy,dynamic`. |  |[default to legacy]
**page_token** | Option<[**serde_json::Value**](.md)> | A token corresponding to a specific page of results, as provided by metadata |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetTemplates200Response**](GET_templates_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_templates_template_id

> serde_json::Value g_et_templates_template_id(template_id, on_behalf_of)
Retrieve a single transactional template.

**This endpoint allows you to retrieve a single transactional template.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_templates_template_id

> serde_json::Value p_atch_templates_template_id(template_id, on_behalf_of, patch_templates_template_id_request)
Edit a transactional template.

**This endpoint allows you to edit the name of a transactional template.**  To edit the template itself, [create a new transactional template version](https://sendgrid.api-docs.io/v3.0/transactional-templates-versions/create-a-new-transactional-template-version).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**patch_templates_template_id_request** | Option<[**PatchTemplatesTemplateIdRequest**](PatchTemplatesTemplateIdRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_templates

> serde_json::Value p_ost_templates(on_behalf_of, post_templates_request)
Create a transactional template.

**This endpoint allows you to create a transactional template.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**post_templates_request** | Option<[**PostTemplatesRequest**](PostTemplatesRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_templates_template_id

> serde_json::Value p_ost_templates_template_id(template_id, on_behalf_of, post_templates_template_id_request)
Duplicate a transactional template.

**This endpoint allows you to duplicate a transactional template.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**post_templates_template_id_request** | Option<[**PostTemplatesTemplateIdRequest**](PostTemplatesTemplateIdRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

