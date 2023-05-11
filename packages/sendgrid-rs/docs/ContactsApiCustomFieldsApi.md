# \ContactsApiCustomFieldsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_contactdb_custom_fields_custom_field_id**](ContactsApiCustomFieldsApi.md#d_elete_contactdb_custom_fields_custom_field_id) | **DELETE** /contactdb/custom_fields/{custom_field_id} | Delete a Custom Field
[**g_et_contactdb_custom_fields**](ContactsApiCustomFieldsApi.md#g_et_contactdb_custom_fields) | **GET** /contactdb/custom_fields | Retrieve all custom fields
[**g_et_contactdb_custom_fields_custom_field_id**](ContactsApiCustomFieldsApi.md#g_et_contactdb_custom_fields_custom_field_id) | **GET** /contactdb/custom_fields/{custom_field_id} | Retrieve a Custom Field
[**g_et_contactdb_reserved_fields**](ContactsApiCustomFieldsApi.md#g_et_contactdb_reserved_fields) | **GET** /contactdb/reserved_fields | Retrieve reserved fields
[**p_ost_contactdb_custom_fields**](ContactsApiCustomFieldsApi.md#p_ost_contactdb_custom_fields) | **POST** /contactdb/custom_fields | Create a Custom Field



## d_elete_contactdb_custom_fields_custom_field_id

> crate::models::GlobalErrorResponseSchema d_elete_contactdb_custom_fields_custom_field_id(custom_field_id, on_behalf_of)
Delete a Custom Field

**This endpoint allows you to delete a custom field by ID.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_field_id** | [**serde_json::Value**](.md) | The ID of the custom field that you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GlobalErrorResponseSchema**](global_error_response_schema.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_contactdb_custom_fields

> crate::models::ListAllCustomFieldsResponse g_et_contactdb_custom_fields(on_behalf_of)
Retrieve all custom fields

**This endpoint allows you to retrieve all custom fields.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::ListAllCustomFieldsResponse**](List_All_Custom_Fields_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_contactdb_custom_fields_custom_field_id

> serde_json::Value g_et_contactdb_custom_fields_custom_field_id(custom_field_id, on_behalf_of)
Retrieve a Custom Field

**This endpoint allows you to retrieve a custom field by ID.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_field_id** | [**serde_json::Value**](.md) | The ID of the custom field that you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_contactdb_reserved_fields

> crate::models::GetContactdbReservedFields200Response g_et_contactdb_reserved_fields(on_behalf_of)
Retrieve reserved fields

**This endpoint allows you to list all fields that are reserved and can't be used for custom field names.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetContactdbReservedFields200Response**](GET_contactdb_reserved_fields_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_contactdb_custom_fields

> serde_json::Value p_ost_contactdb_custom_fields(on_behalf_of, post_contactdb_custom_fields_request)
Create a Custom Field

**This endpoint allows you to create a custom field.**  **You can create up to 120 custom fields.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**post_contactdb_custom_fields_request** | Option<[**PostContactdbCustomFieldsRequest**](PostContactdbCustomFieldsRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

