# \SenderIdentitiesApiApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_eletev3_senders_sender_id**](SenderIdentitiesApiApi.md#d_eletev3_senders_sender_id) | **DELETE** /senders/{sender_id} | Delete a Sender Identity
[**g_etv3_senders**](SenderIdentitiesApiApi.md#g_etv3_senders) | **GET** /senders | Get all Sender Identities
[**g_etv3_senders_sender_id**](SenderIdentitiesApiApi.md#g_etv3_senders_sender_id) | **GET** /senders/{sender_id} | View a Sender Identity
[**p_atchv3_senders_sender_id**](SenderIdentitiesApiApi.md#p_atchv3_senders_sender_id) | **PATCH** /senders/{sender_id} | Update a Sender Identity
[**p_ost_senders**](SenderIdentitiesApiApi.md#p_ost_senders) | **POST** /senders | Create a Sender Identity
[**p_ostv3_senders_sender_id_resend_verification**](SenderIdentitiesApiApi.md#p_ostv3_senders_sender_id_resend_verification) | **POST** /senders/{sender_id}/resend_verification | Resend Sender Identity Verification



## d_eletev3_senders_sender_id

> serde_json::Value d_eletev3_senders_sender_id(sender_id, on_behalf_of)
Delete a Sender Identity

**This endoint allows you to delete one of your sender identities.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sender_id** | [**serde_json::Value**](.md) | The ID of the sender identity that you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_etv3_senders

> crate::models::GetV3Senders200Response g_etv3_senders(on_behalf_of)
Get all Sender Identities

**This endpoint allows you to retrieve a list of all sender identities that have been created for your account.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetV3Senders200Response**](GET_v3_senders_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_etv3_senders_sender_id

> serde_json::Value g_etv3_senders_sender_id(sender_id, on_behalf_of)
View a Sender Identity

**This endpoint allows you to retrieve a specific sender identity.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sender_id** | [**serde_json::Value**](.md) | The ID of the sender identity that you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atchv3_senders_sender_id

> serde_json::Value p_atchv3_senders_sender_id(sender_id, on_behalf_of, sender_id_request)
Update a Sender Identity

**This endpoint allows you to update a sender identity.**  Updates to `from.email` require re-verification.  Partial updates are allowed, but fields that are marked as \"required\" in the POST (create) endpoint must not be nil if that field is included in the PATCH request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sender_id** | [**serde_json::Value**](.md) | The ID of the sender identity that you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**sender_id_request** | Option<[**SenderIdRequest**](SenderIdRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_senders

> serde_json::Value p_ost_senders(on_behalf_of, body)
Create a Sender Identity

**This endpoint allows you to create a new sender identity.**  You may create up to 100 unique sender identities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## p_ostv3_senders_sender_id_resend_verification

> serde_json::Value p_ostv3_senders_sender_id_resend_verification(sender_id, on_behalf_of)
Resend Sender Identity Verification

**This enpdoint allows you to resend a sender identity verification email.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sender_id** | [**serde_json::Value**](.md) | The ID of the sender identity for which you would like to resend a verification email. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

