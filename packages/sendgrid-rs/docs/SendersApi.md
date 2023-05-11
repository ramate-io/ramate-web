# \SendersApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**p_ost_marketing_senders**](SendersApi.md#p_ost_marketing_senders) | **POST** /marketing/senders | Create a Sender Identity



## p_ost_marketing_senders

> serde_json::Value p_ost_marketing_senders(on_behalf_of, post_marketing_senders_request)
Create a Sender Identity

**This endpoint allows you to create a new sender identity.**  *You may create up to 100 unique sender identities.*  Sender identities are required to be verified before use. If your domain has been authenticated, a new sender identity will auto verify on creation. Otherwise an email will be sent to the `from.email`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**post_marketing_senders_request** | Option<[**PostMarketingSendersRequest**](PostMarketingSendersRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

