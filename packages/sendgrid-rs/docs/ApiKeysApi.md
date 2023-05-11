# \ApiKeysApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_keys**](ApiKeysApi.md#create_api_keys) | **POST** /api_keys | Create API keys
[**d_elete_api_keys_api_key_id**](ApiKeysApi.md#d_elete_api_keys_api_key_id) | **DELETE** /api_keys/{api_key_id} | Delete API keys
[**g_et_api_keys**](ApiKeysApi.md#g_et_api_keys) | **GET** /api_keys | Retrieve all API Keys belonging to the authenticated user
[**g_et_api_keys_api_key_id**](ApiKeysApi.md#g_et_api_keys_api_key_id) | **GET** /api_keys/{api_key_id} | Retrieve an existing API Key
[**p_atch_api_keys_api_key_id**](ApiKeysApi.md#p_atch_api_keys_api_key_id) | **PATCH** /api_keys/{api_key_id} | Update API key name
[**p_ut_api_keys_api_key_id**](ApiKeysApi.md#p_ut_api_keys_api_key_id) | **PUT** /api_keys/{api_key_id} | Update API key name and scopes



## create_api_keys

> crate::models::CreateApiKeys201Response create_api_keys(on_behalf_of, create_api_keys_request)
Create API keys

**This endpoint allows you to create a new API Key for the user.**  To create your initial SendGrid API Key, you should [use the SendGrid App](https://app.sendgrid.com/settings/api_keys). Once you have created a first key with scopes to manage additional API keys, you can use this API for all other key management.  > There is a limit of 100 API Keys on your account.  A JSON request body containing a `name` property is required when making requests to this endpoint. If the number of maximum keys, 100, is reached, a `403` status will be returned.  Though the `name` field is required, it does not need to be unique. A unique API key ID will be generated for each key you create and returned in the response body.  It is not necessary to pass a `scopes` field to the API when creating a key, but you should be aware that omitting the `scopes` field from your request will create a key with \"Full Access\" permissions by default.  See the [API Key Permissions List](https://sendgrid.api-docs.io/v3.0/how-to-use-the-sendgrid-v3-api/api-authorization) for all available scopes. An API key's scopes can be updated after creation using the \"Update API keys\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**create_api_keys_request** | Option<[**CreateApiKeysRequest**](CreateApiKeysRequest.md)> |  |  |

### Return type

[**crate::models::CreateApiKeys201Response**](create_api_keys_201_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_elete_api_keys_api_key_id

> d_elete_api_keys_api_key_id(api_key_id, on_behalf_of)
Delete API keys

**This endpoint allows you to revoke an existing API Key using an `api_key_id`**  Authentications using a revoked API Key will fail after after some small propogation delay. If the API Key ID does not exist, a `404` status will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_api_keys

> crate::models::GetApiKeys200Response g_et_api_keys(limit, on_behalf_of)
Retrieve all API Keys belonging to the authenticated user

**This endpoint allows you to retrieve all API Keys that belong to the authenticated user.**  A successful response from this API will include all available API keys' names and IDs.  For security reasons, there is not a way to retrieve the key itself after it's created. If you lose your API key, you must create a new one. Only the \"Create API keys\" endpoint will return a key to you and only at the time of creation.  An `api_key_id` can be used to update or delete the key, as well as retrieve the key's details, such as its scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<[**serde_json::Value**](.md)> |  |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetApiKeys200Response**](GET_api_keys_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_api_keys_api_key_id

> crate::models::GetApiKeysApiKeyId200Response g_et_api_keys_api_key_id(api_key_id, on_behalf_of)
Retrieve an existing API Key

**This endpoint allows you to retrieve a single API key using an `api_key_id`.**  The endpoint will return a key's name, ID, and scopes. If the API Key ID does not, exist a `404` status will be returned.  See the [API Key Permissions List](https://sendgrid.api-docs.io/v3.0/how-to-use-the-sendgrid-v3-api/api-authorization) for all available scopes. An API key's scopes can be updated after creation using the \"Update API keys\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetApiKeysApiKeyId200Response**](GET_api_keys_api_key_id_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_api_keys_api_key_id

> crate::models::ApiKeyNameId p_atch_api_keys_api_key_id(api_key_id, on_behalf_of, patch_api_keys_api_key_id_request)
Update API key name

**This endpoint allows you to update the name of an existing API Key.**  You must pass this endpoint a JSON request body with a `name` property, which will be used to rename the key associated with the `api_key_id` passed in the URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**patch_api_keys_api_key_id_request** | Option<[**PatchApiKeysApiKeyIdRequest**](PatchApiKeysApiKeyIdRequest.md)> |  |  |

### Return type

[**crate::models::ApiKeyNameId**](api_key_name_id.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ut_api_keys_api_key_id

> serde_json::Value p_ut_api_keys_api_key_id(api_key_id, on_behalf_of, put_api_keys_api_key_id_request)
Update API key name and scopes

**This endpoint allows you to update the name and scopes of a given API key.**  You must pass this endpoint a JSON request body with a `name` field and a `scopes` array containing at least one scope. The `name` and `scopes` fields will be used to update the key associated with the `api_key_id` in the request URL.  If you need to update a key's scopes only, pass the `name` field with the key's existing name; the `name` will not be modified. If you need to update a key's name only, use the \"Update API key name\" endpoint.  See the [API Key Permissions List](https://sendgrid.api-docs.io/v3.0/how-to-use-the-sendgrid-v3-api/api-authorization) for all available scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**put_api_keys_api_key_id_request** | Option<[**PutApiKeysApiKeyIdRequest**](PutApiKeysApiKeyIdRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

