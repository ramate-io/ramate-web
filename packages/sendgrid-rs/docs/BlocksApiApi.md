# \BlocksApiApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_suppression_blocks**](BlocksApiApi.md#d_elete_suppression_blocks) | **DELETE** /suppression/blocks | Delete blocks
[**d_elete_suppression_blocks_email**](BlocksApiApi.md#d_elete_suppression_blocks_email) | **DELETE** /suppression/blocks/{email} | Delete a specific block
[**g_et_suppression_blocks**](BlocksApiApi.md#g_et_suppression_blocks) | **GET** /suppression/blocks | Retrieve all blocks
[**g_et_suppression_blocks_email**](BlocksApiApi.md#g_et_suppression_blocks_email) | **GET** /suppression/blocks/{email} | Retrieve a specific block



## d_elete_suppression_blocks

> serde_json::Value d_elete_suppression_blocks(on_behalf_of, delete_suppression_blocks_request)
Delete blocks

**This endpoint allows you to delete all email addresses on your blocks list.**  There are two options for deleting blocked emails:   1. You can delete all blocked emails by setting `delete_all` to `true` in the request body.  2. You can delete a selection of blocked emails by specifying the email addresses in the `emails` array of the request body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**delete_suppression_blocks_request** | Option<[**DeleteSuppressionBlocksRequest**](DeleteSuppressionBlocksRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_elete_suppression_blocks_email

> serde_json::Value d_elete_suppression_blocks_email(email, on_behalf_of)
Delete a specific block

**This endpoint allows you to delete a specific email address from your blocks list.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | [**serde_json::Value**](.md) | The email address of the specific block. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_suppression_blocks

> serde_json::Value g_et_suppression_blocks(start_time, end_time, limit, offset, on_behalf_of)
Retrieve all blocks

**This endpoint allows you to retrieve all email addresses that are currently on your blocks list.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_time** | Option<[**serde_json::Value**](.md)> | The start of the time range when a blocked email was created (inclusive). This is a unix timestamp. |  |
**end_time** | Option<[**serde_json::Value**](.md)> | The end of the time range when a blocked email was created (inclusive). This is a unix timestamp. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Limit the number of results to be displayed per page. |  |
**offset** | Option<[**serde_json::Value**](.md)> | The point in the list to begin displaying results. |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_suppression_blocks_email

> serde_json::Value g_et_suppression_blocks_email(email, on_behalf_of)
Retrieve a specific block

**This endpoint allows you to retrieve a specific email address from your blocks list.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | [**serde_json::Value**](.md) | The email address of the specific block. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

