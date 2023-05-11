# \ApiKeyPermissionsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**g_et_scopes**](ApiKeyPermissionsApi.md#g_et_scopes) | **GET** /scopes | Retrieve a list of scopes for which this user has access.



## g_et_scopes

> crate::models::GetScopes200Response g_et_scopes(on_behalf_of)
Retrieve a list of scopes for which this user has access.

**This endpoint returns a list of all scopes that this user has access to.**  API Keys are used to authenticate with [SendGrid's v3 API](https://sendgrid.api-docs.io/v3.0/how-to-use-the-sendgrid-v3-api/api-authorization).  API Keys may be assigned certain permissions, or scopes, that limit which API endpoints they are able to access.  This endpoint returns all the scopes assigned to the key you use to authenticate with it. To retrieve the scopes assigned to another key, you can pass an API key ID to the \"Retrieve an existing API key\" endpoint.  For a more detailed explanation of how you can use API Key permissions, please visit our [API Keys documentation](https://sendgrid.com/docs/ui/account-and-settings/api-keys/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetScopes200Response**](GET_scopes_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

