# \EmailAddressValidationApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**p_ost_validations_email**](EmailAddressValidationApi.md#p_ost_validations_email) | **POST** /validations/email | Validate an email



## p_ost_validations_email

> crate::models::PostValidationsEmail200Response p_ost_validations_email(post_validations_email_request)
Validate an email

**This endpoint allows you to validate an email address.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_validations_email_request** | Option<[**PostValidationsEmailRequest**](PostValidationsEmailRequest.md)> |  |  |

### Return type

[**crate::models::PostValidationsEmail200Response**](POST_validations_email_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

