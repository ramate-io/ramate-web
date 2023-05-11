# \CancelScheduledSendsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_user_scheduled_sends_batch_id**](CancelScheduledSendsApi.md#d_elete_user_scheduled_sends_batch_id) | **DELETE** /user/scheduled_sends/{batch_id} | Delete a cancellation or pause from a scheduled send
[**g_et_mail_batch_batch_id**](CancelScheduledSendsApi.md#g_et_mail_batch_batch_id) | **GET** /mail/batch/{batch_id} | Validate batch ID
[**g_et_user_scheduled_sends**](CancelScheduledSendsApi.md#g_et_user_scheduled_sends) | **GET** /user/scheduled_sends | Retrieve all scheduled sends
[**g_et_user_scheduled_sends_batch_id**](CancelScheduledSendsApi.md#g_et_user_scheduled_sends_batch_id) | **GET** /user/scheduled_sends/{batch_id} | Retrieve scheduled send
[**p_atch_user_scheduled_sends_batch_id**](CancelScheduledSendsApi.md#p_atch_user_scheduled_sends_batch_id) | **PATCH** /user/scheduled_sends/{batch_id} | Update a scheduled send
[**p_ost_mail_batch**](CancelScheduledSendsApi.md#p_ost_mail_batch) | **POST** /mail/batch | Create a batch ID
[**p_ost_user_scheduled_sends**](CancelScheduledSendsApi.md#p_ost_user_scheduled_sends) | **POST** /user/scheduled_sends | Cancel or pause a scheduled send



## d_elete_user_scheduled_sends_batch_id

> d_elete_user_scheduled_sends_batch_id(batch_id, on_behalf_of)
Delete a cancellation or pause from a scheduled send

**This endpoint allows you to delete the cancellation/pause of a scheduled send.**  Scheduled sends cancelled less than 10 minutes before the scheduled time are not guaranteed to be cancelled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_mail_batch_batch_id

> crate::models::MailBatchId g_et_mail_batch_batch_id(batch_id, on_behalf_of)
Validate batch ID

**This endpoint allows you to validate a batch ID.**  When you pass a valid `batch_id` to this endpoint, it will return a `200` status code and the batch ID itself.  If you pass an invalid `batch_id` to the endpoint, you will receive a `400` level status code and an error message.  A `batch_id` does not need to be assigned to a scheduled send to be considered valid. A successful response means only that the `batch_id` has been created, but it does not indicate that it has been associated with a send.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::MailBatchId**](mail_batch_id.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_user_scheduled_sends

> serde_json::Value g_et_user_scheduled_sends(on_behalf_of)
Retrieve all scheduled sends

**This endpoint allows you to retrieve all cancelled and paused scheduled send information.**  This endpoint will return only the scheduled sends that are associated with a `batch_id`. If you have scheduled a send using the `/mail/send` endpoint and the `send_at` field but no `batch_id`, the send will be scheduled for delivery; however, it will not be returned by this endpoint. For this reason, you should assign a `batch_id` to any scheduled send you may need to pause or cancel in the future.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_user_scheduled_sends_batch_id

> serde_json::Value g_et_user_scheduled_sends_batch_id(batch_id, on_behalf_of)
Retrieve scheduled send

**This endpoint allows you to retrieve the cancel/paused scheduled send information for a specific `batch_id`.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_user_scheduled_sends_batch_id

> serde_json::Value p_atch_user_scheduled_sends_batch_id(batch_id, on_behalf_of, patch_user_scheduled_sends_batch_id_request)
Update a scheduled send

**This endpoint allows you to update the status of a scheduled send for the given `batch_id`.**  If you have already set a `cancel` or `pause` status on a scheduled send using the \"Cancel or pause a scheduled send\" endpoint, you can update it's status using this endpoint. Attempting to update a status once it has been set with the \"Cancel or pause a scheduled send\" endpoint will result in a `400` error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**patch_user_scheduled_sends_batch_id_request** | Option<[**PatchUserScheduledSendsBatchIdRequest**](PatchUserScheduledSendsBatchIdRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_mail_batch

> crate::models::MailBatchId p_ost_mail_batch(on_behalf_of)
Create a batch ID

**This endpoint allows you to generate a new batch ID.**  Once a `batch_id` is created, you can associate it with a scheduled send using the `/mail/send` endpoint. Passing the `batch_id` as a field in the `/mail/send` request body will assign the ID to the send you are creating.  Once an ID is associated with a scheduled send, the send can be accessed and its send status can be modified using the `batch_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::MailBatchId**](mail_batch_id.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_user_scheduled_sends

> serde_json::Value p_ost_user_scheduled_sends(on_behalf_of, cancel_or_pause_a_scheduled_send_request)
Cancel or pause a scheduled send

**This endpoint allows you to cancel or pause a scheduled send associated with a `batch_id`.**  Passing this endpoint a `batch_id` and status will cancel or pause the scheduled send.  Once a scheduled send is set to `pause` or `cancel` you must use the \"Update a scheduled send\" endpoint to change its status or the \"Delete a cancellation or pause from a scheduled send\" endpoint to remove the status. Passing a status change to a scheduled send that has already been paused or cancelled will result in a `400` level status code.  If the maximum number of cancellations/pauses are added to a send, a `400` level status code will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**cancel_or_pause_a_scheduled_send_request** | Option<[**CancelOrPauseAScheduledSendRequest**](CancelOrPauseAScheduledSendRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

