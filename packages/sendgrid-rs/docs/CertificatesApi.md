# \CertificatesApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_sso_certificates_cert_id**](CertificatesApi.md#d_elete_sso_certificates_cert_id) | **DELETE** /sso/certificates/{cert_id} | Delete an SSO Certificate
[**g_et_sso_certificates_cert_id**](CertificatesApi.md#g_et_sso_certificates_cert_id) | **GET** /sso/certificates/{cert_id} | Get an SSO Certificate
[**g_et_sso_integrations_integration_id_certificates**](CertificatesApi.md#g_et_sso_integrations_integration_id_certificates) | **GET** /sso/integrations/{integration_id}/certificates | Get All SSO Certificates by Integration
[**p_atch_sso_certificates_cert_id**](CertificatesApi.md#p_atch_sso_certificates_cert_id) | **PATCH** /sso/certificates/{cert_id} | Update SSO Certificate
[**p_ost_sso_certificates**](CertificatesApi.md#p_ost_sso_certificates) | **POST** /sso/certificates | Create an SSO Certificate



## d_elete_sso_certificates_cert_id

> crate::models::SsoCertificateBody d_elete_sso_certificates_cert_id(cert_id)
Delete an SSO Certificate

**This endpoint allows you to delete an SSO certificate.**  You can retrieve a certificate's ID from the response provided by the \"Get All SSO Integrations\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cert_id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**crate::models::SsoCertificateBody**](sso-certificate-body.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_sso_certificates_cert_id

> crate::models::SsoCertificateBody g_et_sso_certificates_cert_id(cert_id)
Get an SSO Certificate

**This endpoint allows you to retrieve an individual SSO certificate.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cert_id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**crate::models::SsoCertificateBody**](sso-certificate-body.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_sso_integrations_integration_id_certificates

> serde_json::Value g_et_sso_integrations_integration_id_certificates(integration_id)
Get All SSO Certificates by Integration

**This endpoint allows you to retrieve all your IdP configurations by configuration ID.**  The `integration_id` expected by this endpoint is the `id` returned in the response by the \"Get All SSO Integrations\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | [**serde_json::Value**](.md) | An ID that matches a certificate to a specific IdP integration. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_sso_certificates_cert_id

> p_atch_sso_certificates_cert_id(cert_id, patch_sso_certificates_cert_id_request)
Update SSO Certificate

**This endpoint allows you to update an existing certificate by ID.**  You can retrieve a certificate's ID from the response provided by the \"Get All SSO Integrations\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cert_id** | [**serde_json::Value**](.md) |  | [required] |
**patch_sso_certificates_cert_id_request** | Option<[**PatchSsoCertificatesCertIdRequest**](PatchSsoCertificatesCertIdRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_sso_certificates

> crate::models::SsoCertificateBody p_ost_sso_certificates(post_sso_certificates_request)
Create an SSO Certificate

**This endpoint allows you to create an SSO certificate.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_sso_certificates_request** | Option<[**PostSsoCertificatesRequest**](PostSsoCertificatesRequest.md)> |  |  |

### Return type

[**crate::models::SsoCertificateBody**](sso-certificate-body.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

