# \CustomFieldsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_eletemc_field_definitions_custom_field_id**](CustomFieldsApi.md#d_eletemc_field_definitions_custom_field_id) | **DELETE** /marketing/field_definitions/{custom_field_id} | Delete Custom Field Definition
[**g_etmc_field_definitions**](CustomFieldsApi.md#g_etmc_field_definitions) | **GET** /marketing/field_definitions | Get All Field Definitions
[**p_atchmc_field_definitions_custom_field_id**](CustomFieldsApi.md#p_atchmc_field_definitions_custom_field_id) | **PATCH** /marketing/field_definitions/{custom_field_id} | Update Custom Field Definition
[**p_ostmc_field_definitions**](CustomFieldsApi.md#p_ostmc_field_definitions) | **POST** /marketing/field_definitions | Create Custom Field Definition



## d_eletemc_field_definitions_custom_field_id

> d_eletemc_field_definitions_custom_field_id(custom_field_id)
Delete Custom Field Definition

**This endpoint deletes a defined Custom Field.**  You cand delete only Custom Fields; Reserved Fields cannot be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_field_id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_etmc_field_definitions

> crate::models::GetMcFieldDefinitions200Response g_etmc_field_definitions()
Get All Field Definitions

**This endpoint retrieves all defined Custom Fields and Reserved Fields.**

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetMcFieldDefinitions200Response**](GET_mc_field_definitions_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atchmc_field_definitions_custom_field_id

> serde_json::Value p_atchmc_field_definitions_custom_field_id(custom_field_id, patch_mc_field_definitions_custom_field_id_request)
Update Custom Field Definition

**This endopoint allows you to update a defined Custom Field.**  Only your Custom fields can be modified; Reserved Fields cannot be updated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_field_id** | [**serde_json::Value**](.md) |  | [required] |
**patch_mc_field_definitions_custom_field_id_request** | Option<[**PatchMcFieldDefinitionsCustomFieldIdRequest**](PatchMcFieldDefinitionsCustomFieldIdRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ostmc_field_definitions

> serde_json::Value p_ostmc_field_definitions(post_mc_field_definitions_request)
Create Custom Field Definition

**This endpoint creates a new custom field definition.**  Custom field definitions are created with the given `name` and `field_type`. Although field names are stored in a case-sensitive manner, all field names must be case-insensitively unique. This means you may create a field named `CamelCase` or `camelcase`, but not both. Additionally, a Custom Field name cannot collide with any Reserved Field names. You should save the returned `id` value in order to update or delete the field at a later date. You can have up to 120 custom fields.  The custom field name should be created using only alphanumeric characters (A-Z and 0-9) and underscores (\\_). Custom fields can only begin with letters  A-Z or underscores (_). The field type can be date, text, or number fields. The field type is important for creating segments from your contact database.  **Note: Creating a custom field that begins with a number will cause issues with sending in Marketing Campaigns.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_mc_field_definitions_request** | Option<[**PostMcFieldDefinitionsRequest**](PostMcFieldDefinitionsRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

