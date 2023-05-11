# \V3Api

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**g_etv3_messages_download_download_uuid**](V3Api.md#g_etv3_messages_download_download_uuid) | **GET** /messages/download/{download_uuid} | Download CSV
[**p_ostv3_messages_download**](V3Api.md#p_ostv3_messages_download) | **POST** /messages/download | Request CSV



## g_etv3_messages_download_download_uuid

> crate::models::GetV3MessagesDownloadDownloadUuid200Response g_etv3_messages_download_download_uuid(authorization, download_uuid)
Download CSV

**This endpoint will return a presigned URL that can be used to download the CSV that was requested from the \"Request a CSV\" endpoint.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | [**serde_json::Value**](.md) |  | [required] |[default to Bearer <<YOUR_API_KEY_HERE>>]
**download_uuid** | [**serde_json::Value**](.md) | UUID used to locate the download csv request entry in the DB.  This is the UUID provided in the email sent to the user when their csv file is ready to download | [required] |

### Return type

[**crate::models::GetV3MessagesDownloadDownloadUuid200Response**](GET_v3_messages_download_download_uuid_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ostv3_messages_download

> crate::models::PostV3MessagesDownload202Response p_ostv3_messages_download(authorization, query)
Request CSV

This is BETA functionality. You may not have access, and we reserve the right to change functionality without notice.  This request will kick off a backend process to generate a CSV file. Once generated, the worker will then send an email for the user download the file. The link will expire in 3 days.  The CSV fill contain the last 1 million messages. This endpoint will be rate limited to 1 request every 12 hours (rate limit may change).  This endpoint is similar to the GET Single Message endpoint - the only difference is that /download is added to indicate that this is a CSV download requests but the same query is used to determine what the CSV should contain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | [**serde_json::Value**](.md) |  | [required] |[default to Bearer <<YOUR_API_KEY_HERE>>]
**query** | Option<[**serde_json::Value**](.md)> | Uses a SQL like syntax to indicate which messages to include in the CSV |  |

### Return type

[**crate::models::PostV3MessagesDownload202Response**](POST_v3_messages_download_202_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

