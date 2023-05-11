# \TeammatesApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_eletev3_scopes_requests_request_id**](TeammatesApi.md#d_eletev3_scopes_requests_request_id) | **DELETE** /scopes/requests/{request_id} | Deny access request
[**d_eletev3_teammates_pending_token**](TeammatesApi.md#d_eletev3_teammates_pending_token) | **DELETE** /teammates/pending/{token} | Delete pending teammate
[**d_eletev3_teammates_username**](TeammatesApi.md#d_eletev3_teammates_username) | **DELETE** /teammates/{username} | Delete teammate
[**g_etv3_scopes_requests**](TeammatesApi.md#g_etv3_scopes_requests) | **GET** /scopes/requests | Retrieve access requests
[**g_etv3_teammates**](TeammatesApi.md#g_etv3_teammates) | **GET** /teammates | Retrieve all teammates
[**g_etv3_teammates_pending**](TeammatesApi.md#g_etv3_teammates_pending) | **GET** /teammates/pending | Retrieve all pending teammates
[**g_etv3_teammates_username**](TeammatesApi.md#g_etv3_teammates_username) | **GET** /teammates/{username} | Retrieve specific teammate
[**p_atchv3_scopes_requests_approve_id**](TeammatesApi.md#p_atchv3_scopes_requests_approve_id) | **PATCH** /scopes/requests/{request_id}/approve | Approve access request
[**p_atchv3_teammates_username**](TeammatesApi.md#p_atchv3_teammates_username) | **PATCH** /teammates/{username} | Update teammate's permissions
[**p_ostv3_teammates**](TeammatesApi.md#p_ostv3_teammates) | **POST** /teammates | Invite teammate
[**p_ostv3_teammates_pending_token_resend**](TeammatesApi.md#p_ostv3_teammates_pending_token_resend) | **POST** /teammates/pending/{token}/resend | Resend teammate invite



## d_eletev3_scopes_requests_request_id

> d_eletev3_scopes_requests_request_id(request_id)
Deny access request

**This endpoint allows you to deny an attempt to access your account.**  **Note:** Only teammate admins may delete a teammate's access request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | [**serde_json::Value**](.md) | The ID of the request that you want to deny. | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_eletev3_teammates_pending_token

> d_eletev3_teammates_pending_token(token, on_behalf_of)
Delete pending teammate

**This endpoint allows you to delete a pending teammate invite.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | [**serde_json::Value**](.md) | The token for the invite you want to delete. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_eletev3_teammates_username

> crate::models::PostV3Teammates400Response d_eletev3_teammates_username(username, on_behalf_of)
Delete teammate

**This endpoint allows you to delete a teammate.**  **Only the parent user or an admin teammate can delete another teammate.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | [**serde_json::Value**](.md) | The username of the teammate that you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::PostV3Teammates400Response**](POST_v3_teammates_400_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_etv3_scopes_requests

> serde_json::Value g_etv3_scopes_requests(limit, offset)
Retrieve access requests

**This endpoint allows you to retrieve a list of all recent access requests.**  The Response Header's `link` parameter will include pagination info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<[**serde_json::Value**](.md)> | Optional field to limit the number of results returned. |  |[default to 50]
**offset** | Option<[**serde_json::Value**](.md)> | Optional beginning point in the list to retrieve from. |  |[default to 0]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_etv3_teammates

> crate::models::GetV3Teammates200Response g_etv3_teammates(limit, offset, on_behalf_of)
Retrieve all teammates

**This endpoint allows you to retrieve a list of all current Teammates.**  You can limit the number of results returned using the `limit` query paramater. To return results from a specific Teammate, use the `offset` paramter. The Response Headers will include pagination info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<[**serde_json::Value**](.md)> | Number of items to return |  |[default to 500]
**offset** | Option<[**serde_json::Value**](.md)> | Paging offset |  |[default to 0]
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetV3Teammates200Response**](GET_v3_teammates_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_etv3_teammates_pending

> crate::models::GetV3TeammatesPending200Response g_etv3_teammates_pending(on_behalf_of)
Retrieve all pending teammates

**This endpoint allows you to retrieve a list of all pending Teammate invitations.**  Each teammate invitation is valid for 7 days. Users may resend the invitation to refresh the expiration date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetV3TeammatesPending200Response**](GET_v3_teammates_pending_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_etv3_teammates_username

> crate::models::GetV3TeammatesUsername200Response g_etv3_teammates_username(username, on_behalf_of)
Retrieve specific teammate

**This endpoint allows you to retrieve a specific Teammate by username.**  You can retrieve the username's for each of your Teammates using the \"Retrieve all Teammates\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | [**serde_json::Value**](.md) | The username of the teammate that you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetV3TeammatesUsername200Response**](GET_v3_teammates_username_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atchv3_scopes_requests_approve_id

> crate::models::PatchV3ScopesRequestsApproveId200Response p_atchv3_scopes_requests_approve_id(request_id)
Approve access request

**This endpoint allows you to approve an access attempt.**  **Note:** Only teammate admins may approve another teammate’s access request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | [**serde_json::Value**](.md) | The ID of the request that you want to approve. | [required] |

### Return type

[**crate::models::PatchV3ScopesRequestsApproveId200Response**](PATCH_v3_scopes_requests_approve_id_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atchv3_teammates_username

> crate::models::PatchV3TeammatesUsername200Response p_atchv3_teammates_username(username, on_behalf_of, patch_v3_teammates_username_request)
Update teammate's permissions

**This endpoint allows you to update a teammate’s permissions.**  To turn a teammate into an admin, the request body should contain an `is_admin` set to `true`. Otherwise, set `is_admin` to `false` and pass in all the scopes that a teammate should have.  **Only the parent user or other admin teammates can update another teammate’s permissions.**  **Admin users can only update permissions.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | [**serde_json::Value**](.md) | The username of the teammate that you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**patch_v3_teammates_username_request** | Option<[**PatchV3TeammatesUsernameRequest**](PatchV3TeammatesUsernameRequest.md)> |  |  |

### Return type

[**crate::models::PatchV3TeammatesUsername200Response**](PATCH_v3_teammates_username_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ostv3_teammates

> crate::models::PostV3Teammates201Response p_ostv3_teammates(on_behalf_of, post_v3_teammates_request)
Invite teammate

**This endpoint allows you to invite a Teammate to your account via email.**  You can set a Teammate's initial permissions using the `scopes` array in the request body. Teammate's will receive a minimum set of scopes from Twilio SendGrid that are necessary for the Teammate to function.  **Note:** A teammate invite will expire after 7 days, but you may resend the invitation at any time to reset the expiration date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**post_v3_teammates_request** | Option<[**PostV3TeammatesRequest**](PostV3TeammatesRequest.md)> |  |  |

### Return type

[**crate::models::PostV3Teammates201Response**](POST_v3_teammates_201_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ostv3_teammates_pending_token_resend

> crate::models::PostV3TeammatesPendingTokenResend200Response p_ostv3_teammates_pending_token_resend(token, on_behalf_of)
Resend teammate invite

**This endpoint allows you to resend a Teammate invitation.**  Teammate invitations will expire after 7 days. Resending an invitation will reset the expiration date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | [**serde_json::Value**](.md) | The token for the invite that you want to resend. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::PostV3TeammatesPendingTokenResend200Response**](POST_v3_teammates_pending_token_resend_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

