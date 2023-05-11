# \SingleSignOnTeammatesApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**p_atch_sso_teammates_username**](SingleSignOnTeammatesApi.md#p_atch_sso_teammates_username) | **PATCH** /sso/teammates/{username} | Edit an SSO Teammate
[**p_ost_sso_teammates**](SingleSignOnTeammatesApi.md#p_ost_sso_teammates) | **POST** /sso/teammates | Create SSO Teammate



## p_atch_sso_teammates_username

> serde_json::Value p_atch_sso_teammates_username(username, patch_sso_teammates_username_request)
Edit an SSO Teammate

**This endpoint allows you to modify an existing SSO Teammate.**  To turn a teammate into an admin, the request body should contain the `is_admin` field set to `true`. Otherwise, set `is_admin` to false and pass in all the scopes that a teammate should have.  Only the parent user and Teammates with admin permissions can update another Teammate’s permissions. Admin users can only update permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | [**serde_json::Value**](.md) | This email address must be the same address assigned to the teammate in your IdP | [required] |
**patch_sso_teammates_username_request** | Option<[**PatchSsoTeammatesUsernameRequest**](PatchSsoTeammatesUsernameRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_sso_teammates

> serde_json::Value p_ost_sso_teammates(body)
Create SSO Teammate

**This endpoint allows you to create an SSO Teammate.**  The email provided for this user will also function as the Teammate’s username.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

