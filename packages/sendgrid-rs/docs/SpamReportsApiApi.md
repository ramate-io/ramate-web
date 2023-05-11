# \SpamReportsApiApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_suppression_spam_reports**](SpamReportsApiApi.md#d_elete_suppression_spam_reports) | **DELETE** /suppression/spam_reports | Delete spam reports
[**d_elete_suppression_spam_reports_email**](SpamReportsApiApi.md#d_elete_suppression_spam_reports_email) | **DELETE** /suppression/spam_reports/{email} | Delete a specific spam report
[**g_et_suppression_spam_reports**](SpamReportsApiApi.md#g_et_suppression_spam_reports) | **GET** /suppression/spam_reports | Retrieve all spam reports
[**g_et_suppression_spam_reports_email**](SpamReportsApiApi.md#g_et_suppression_spam_reports_email) | **GET** /suppression/spam_reports/{email} | Retrieve a specific spam report



## d_elete_suppression_spam_reports

> serde_json::Value d_elete_suppression_spam_reports(on_behalf_of, delete_suppression_spam_reports_request)
Delete spam reports

**This endpoint allows you to delete your spam reports.**  Deleting a spam report will remove the suppression, meaning email will once again be sent to the previously suppressed address. This should be avoided unless a recipient indicates they wish to receive email from you again. You can use our [bypass filters](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) to deliver messages to otherwise suppressed addresses when exceptions are required.  There are two options for deleting spam reports:   1. You can delete all spam reports by setting the `delete_all` field to `true` in the request body. 2. You can delete a list of select spam reports by specifying the email addresses in the `emails` array of the request body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**delete_suppression_spam_reports_request** | Option<[**DeleteSuppressionSpamReportsRequest**](DeleteSuppressionSpamReportsRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_elete_suppression_spam_reports_email

> serde_json::Value d_elete_suppression_spam_reports_email(email, on_behalf_of)
Delete a specific spam report

**This endpoint allows you to delete a specific spam report by email address.**  Deleting a spam report will remove the suppression, meaning email will once again be sent to the previously suppressed address. This should be avoided unless a recipient indicates they wish to receive email from you again. You can use our [bypass filters](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) to deliver messages to otherwise suppressed addresses when exceptions are required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | [**serde_json::Value**](.md) | The email address of a specific spam report that you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_suppression_spam_reports

> serde_json::Value g_et_suppression_spam_reports(start_time, end_time, limit, offset, on_behalf_of)
Retrieve all spam reports

**This endpoint allows you to retrieve all spam reports.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_time** | Option<[**serde_json::Value**](.md)> | The start of the time range when a spam report was created (inclusive). This is a unix timestamp. |  |
**end_time** | Option<[**serde_json::Value**](.md)> | The end of the time range when a spam report was created (inclusive). This is a unix timestamp. |  |
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


## g_et_suppression_spam_reports_email

> serde_json::Value g_et_suppression_spam_reports_email(email, on_behalf_of)
Retrieve a specific spam report

**This endpoint allows you to retrieve a specific spam report by email address.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | [**serde_json::Value**](.md) | The email address of a specific spam report that you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

