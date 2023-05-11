# \InvalidEmailsApiApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_suppression_invalid_emails**](InvalidEmailsApiApi.md#d_elete_suppression_invalid_emails) | **DELETE** /suppression/invalid_emails | Delete invalid emails
[**d_elete_suppression_invalid_emails_email**](InvalidEmailsApiApi.md#d_elete_suppression_invalid_emails_email) | **DELETE** /suppression/invalid_emails/{email} | Delete a specific invalid email
[**g_et_suppression_invalid_emails**](InvalidEmailsApiApi.md#g_et_suppression_invalid_emails) | **GET** /suppression/invalid_emails | Retrieve all invalid emails
[**g_et_suppression_invalid_emails_email**](InvalidEmailsApiApi.md#g_et_suppression_invalid_emails_email) | **GET** /suppression/invalid_emails/{email} | Retrieve a specific invalid email



## d_elete_suppression_invalid_emails

> serde_json::Value d_elete_suppression_invalid_emails(on_behalf_of, delete_suppression_invalid_emails_request)
Delete invalid emails

**This endpoint allows you to remove email addresses from your invalid email address list.**  There are two options for deleting invalid email addresses:   1) You can delete all invalid email addresses by setting `delete_all` to true in the request body. 2) You can delete some invalid email addresses by specifying certain addresses in an array in the request body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**delete_suppression_invalid_emails_request** | Option<[**DeleteSuppressionInvalidEmailsRequest**](DeleteSuppressionInvalidEmailsRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_elete_suppression_invalid_emails_email

> serde_json::Value d_elete_suppression_invalid_emails_email(email, on_behalf_of)
Delete a specific invalid email

**This endpoint allows you to remove a specific email address from the invalid email address list.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | [**serde_json::Value**](.md) | The specific email address of the invalid email entry that you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_suppression_invalid_emails

> serde_json::Value g_et_suppression_invalid_emails(start_time, end_time, limit, offset, on_behalf_of)
Retrieve all invalid emails

**This endpoint allows you to retrieve a list of all invalid email addresses.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_time** | Option<[**serde_json::Value**](.md)> | Refers start of the time range in unix timestamp when an invalid email was created (inclusive). |  |
**end_time** | Option<[**serde_json::Value**](.md)> | Refers end of the time range in unix timestamp when an invalid email was created (inclusive). |  |
**limit** | Option<[**serde_json::Value**](.md)> | Limit the number of results to be displayed per page. |  |
**offset** | Option<[**serde_json::Value**](.md)> | Paging offset. The point in the list to begin displaying results. |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_suppression_invalid_emails_email

> serde_json::Value g_et_suppression_invalid_emails_email(email, on_behalf_of)
Retrieve a specific invalid email

**This endpoint allows you to retrieve a specific invalid email addresses.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | [**serde_json::Value**](.md) | The specific email address of the invalid email entry that you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

