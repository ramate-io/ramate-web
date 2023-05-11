# \BouncesApiApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_suppression_bounces**](BouncesApiApi.md#d_elete_suppression_bounces) | **DELETE** /suppression/bounces | Delete bounces
[**d_elete_suppression_bounces_email**](BouncesApiApi.md#d_elete_suppression_bounces_email) | **DELETE** /suppression/bounces/{email} | Delete a bounce
[**g_et_suppression_bounces**](BouncesApiApi.md#g_et_suppression_bounces) | **GET** /suppression/bounces | Retrieve all bounces
[**g_et_suppression_bounces_email**](BouncesApiApi.md#g_et_suppression_bounces_email) | **GET** /suppression/bounces/{email} | Retrieve a Bounce



## d_elete_suppression_bounces

> serde_json::Value d_elete_suppression_bounces(on_behalf_of, delete_suppression_bounces_request)
Delete bounces

**This endpoint allows you to delete all emails on your bounces list.**  There are two options for deleting bounced emails:   1. You can delete all bounced emails by setting `delete_all` to `true` in the request body.  2. You can delete a selection of bounced emails by specifying the email addresses in the `emails` array of the request body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**delete_suppression_bounces_request** | Option<[**DeleteSuppressionBouncesRequest**](DeleteSuppressionBouncesRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_elete_suppression_bounces_email

> serde_json::Value d_elete_suppression_bounces_email(email_address, email, on_behalf_of, body)
Delete a bounce

**This endpoint allows you to remove an email address from your bounce list.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_address** | [**serde_json::Value**](.md) | The email address you would like to remove from the bounce list. | [required] |
**email** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_suppression_bounces

> serde_json::Value g_et_suppression_bounces(accept, start_time, end_time, on_behalf_of)
Retrieve all bounces

**This endpoint allows you to retrieve all of your bounces.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept** | [**serde_json::Value**](.md) |  | [required] |[default to application/json]
**start_time** | Option<[**serde_json::Value**](.md)> | Refers start of the time range in unix timestamp when a bounce was created (inclusive). |  |
**end_time** | Option<[**serde_json::Value**](.md)> | Refers end of the time range in unix timestamp when a bounce was created (inclusive). |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_suppression_bounces_email

> serde_json::Value g_et_suppression_bounces_email(email, on_behalf_of)
Retrieve a Bounce

**This endpoint allows you to retrieve a specific bounce by email address.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

