# \SuppressionsGlobalSuppressionsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_asm_suppressions_global_email**](SuppressionsGlobalSuppressionsApi.md#d_elete_asm_suppressions_global_email) | **DELETE** /asm/suppressions/global/{email} | Delete a Global Suppression
[**g_et_asm_suppressions_global_email**](SuppressionsGlobalSuppressionsApi.md#g_et_asm_suppressions_global_email) | **GET** /asm/suppressions/global/{email} | Retrieve a Global Suppression
[**g_et_suppression_unsubscribes**](SuppressionsGlobalSuppressionsApi.md#g_et_suppression_unsubscribes) | **GET** /suppression/unsubscribes | Retrieve all global suppressions
[**p_ost_asm_suppressions_global**](SuppressionsGlobalSuppressionsApi.md#p_ost_asm_suppressions_global) | **POST** /asm/suppressions/global | Add recipient addresses to the global suppression group.



## d_elete_asm_suppressions_global_email

> serde_json::Value d_elete_asm_suppressions_global_email(email, on_behalf_of)
Delete a Global Suppression

**This endpoint allows you to remove an email address from the global suppressions group.**  Deleting a suppression group will remove the suppression, meaning email will once again be sent to the previously suppressed addresses. This should be avoided unless a recipient indicates they wish to receive email from you again. You can use our [bypass filters](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) to deliver messages to otherwise suppressed addresses when exceptions are required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | [**serde_json::Value**](.md) | The email address of the global suppression you want to retrieve. Or, if you want to check if an email address is on the global suppressions list, enter that email address here. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_asm_suppressions_global_email

> crate::models::RetrieveAGlobalSuppressionResponse g_et_asm_suppressions_global_email(email, on_behalf_of)
Retrieve a Global Suppression

**This endpoint allows you to retrieve a global suppression. You can also use this endpoint to confirm if an email address is already globally suppresed.**  If the email address you include in the URL path parameter `{email}` is already globally suppressed, the response will include that email address. If the address you enter for `{email}` is not globally suppressed, an empty JSON object `{}` will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | [**serde_json::Value**](.md) | The email address of the global suppression you want to retrieve. Or, if you want to check if an email address is on the global suppressions list, enter that email address here. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::RetrieveAGlobalSuppressionResponse**](Retrieve_a_Global_Suppression_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_suppression_unsubscribes

> serde_json::Value g_et_suppression_unsubscribes(start_time, end_time, limit, offset, on_behalf_of)
Retrieve all global suppressions

**This endpoint allows you to retrieve a list of all email address that are globally suppressed.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_time** | Option<[**serde_json::Value**](.md)> | Refers start of the time range in unix timestamp when an unsubscribe email was created (inclusive). |  |
**end_time** | Option<[**serde_json::Value**](.md)> | Refers end of the time range in unix timestamp when an unsubscribe email was created (inclusive). |  |
**limit** | Option<[**serde_json::Value**](.md)> | The number of results to display on each page. |  |
**offset** | Option<[**serde_json::Value**](.md)> | The point in the list of results to begin displaying global suppressions. |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_asm_suppressions_global

> crate::models::PostAsmSuppressionsGlobal201Response p_ost_asm_suppressions_global(on_behalf_of, suppressions_request)
Add recipient addresses to the global suppression group.

**This endpoint allows you to add one or more email addresses to the global suppressions group.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**suppressions_request** | Option<[**SuppressionsRequest**](SuppressionsRequest.md)> |  |  |

### Return type

[**crate::models::PostAsmSuppressionsGlobal201Response**](POST_asm_suppressions_global_201_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

