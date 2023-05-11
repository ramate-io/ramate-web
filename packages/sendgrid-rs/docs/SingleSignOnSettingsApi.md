# \SingleSignOnSettingsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_sso_integrations_id**](SingleSignOnSettingsApi.md#d_elete_sso_integrations_id) | **DELETE** /sso/integrations/{id} | Delete an SSO Integration
[**g_et_sso_integrations**](SingleSignOnSettingsApi.md#g_et_sso_integrations) | **GET** /sso/integrations | Get All SSO Integrations
[**g_et_sso_integrations_id**](SingleSignOnSettingsApi.md#g_et_sso_integrations_id) | **GET** /sso/integrations/{id} | Get an SSO Integration
[**p_atch_sso_integrations_id**](SingleSignOnSettingsApi.md#p_atch_sso_integrations_id) | **PATCH** /sso/integrations/{id} | Update an SSO Integration
[**p_ost_sso_integrations**](SingleSignOnSettingsApi.md#p_ost_sso_integrations) | **POST** /sso/integrations | Create an SSO Integration



## d_elete_sso_integrations_id

> d_elete_sso_integrations_id(id)
Delete an SSO Integration

**This endpoint allows you to delete an IdP configuration by ID.**  You can retrieve the IDs for your configurations from the response provided by the \"Get All SSO Integrations\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_sso_integrations

> serde_json::Value g_et_sso_integrations(si)
Get All SSO Integrations

**This endpoint allows you to retrieve all SSO integrations tied to your Twilio SendGrid account.**  The IDs returned by this endpoint can be used by the APIs additional endpoints to modify your SSO integrations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**si** | Option<[**serde_json::Value**](.md)> | If this parameter is set to `true`, the response will include the `completed_integration` field. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_sso_integrations_id

> serde_json::Value g_et_sso_integrations_id(id, si)
Get an SSO Integration

**This endpoint allows you to retrieve an SSO integration by ID.**  You can retrieve the IDs for your configurations from the response provided by the \"Get All SSO Integrations\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |
**si** | Option<[**serde_json::Value**](.md)> | If this parameter is set to `true`, the response will include the `completed_integration` field. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_sso_integrations_id

> serde_json::Value p_atch_sso_integrations_id(id, si, create_integration_request)
Update an SSO Integration

**This endpoint allows you to modify an exisiting SSO integration.**  You can retrieve the IDs for your configurations from the response provided by the \"Get All SSO Integrations\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |
**si** | Option<[**serde_json::Value**](.md)> | If this parameter is set to `true`, the response will include the `completed_integration` field. |  |
**create_integration_request** | Option<[**CreateIntegrationRequest**](CreateIntegrationRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_sso_integrations

> serde_json::Value p_ost_sso_integrations(create_integration_request)
Create an SSO Integration

**This endpoint allows you to create an SSO integration.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_integration_request** | Option<[**CreateIntegrationRequest**](CreateIntegrationRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

