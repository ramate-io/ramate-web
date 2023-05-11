# \ReverseDnsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_whitelabel_ips_id**](ReverseDnsApi.md#d_elete_whitelabel_ips_id) | **DELETE** /whitelabel/ips/{id} | Delete a reverse DNS record
[**g_et_whitelabel_ips**](ReverseDnsApi.md#g_et_whitelabel_ips) | **GET** /whitelabel/ips | Retrieve all reverse DNS records
[**g_et_whitelabel_ips_id**](ReverseDnsApi.md#g_et_whitelabel_ips_id) | **GET** /whitelabel/ips/{id} | Retrieve a reverse DNS record
[**p_ost_whitelabel_ips**](ReverseDnsApi.md#p_ost_whitelabel_ips) | **POST** /whitelabel/ips | Set up reverse DNS
[**p_ost_whitelabel_ips_id_validate**](ReverseDnsApi.md#p_ost_whitelabel_ips_id_validate) | **POST** /whitelabel/ips/{id}/validate | Validate a reverse DNS record



## d_elete_whitelabel_ips_id

> serde_json::Value d_elete_whitelabel_ips_id(id, on_behalf_of)
Delete a reverse DNS record

**This endpoint allows you to delete a reverse DNS record.**  A call to this endpoint will respond with a 204 status code if the deletion was successful.  You can retrieve the IDs associated with all your reverse DNS records using the \"Retrieve all reverse DNS records\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The ID of the reverse DNS record that you would like to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_whitelabel_ips

> serde_json::Value g_et_whitelabel_ips(limit, offset, ip, on_behalf_of)
Retrieve all reverse DNS records

**This endpoint allows you to retrieve all of the Reverse DNS records created by this account.**  You may include a search key by using the `ip` query string parameter. This enables you to perform a prefix search for a given IP segment (e.g., `?ip=\"192.\"`).  Use the `limit` query string parameter to reduce the number of records returned. All records will be returned if you have fewer records than the specified limit.  The `offset` query string parameter allows you to specify a non-zero index from which records will be returned. For example, if you have ten records, `?offset=5` will return the last five records (at indexes 5 through 9). The list starts at index zero.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<[**serde_json::Value**](.md)> | The maximum number of results to retrieve. |  |
**offset** | Option<[**serde_json::Value**](.md)> | The point in the list of results to begin retrieving IP addresses from. |  |
**ip** | Option<[**serde_json::Value**](.md)> | The IP address segment that you'd like to use in a prefix search. |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_whitelabel_ips_id

> crate::models::ReverseDns g_et_whitelabel_ips_id(id, on_behalf_of)
Retrieve a reverse DNS record

**This endpoint allows you to retrieve a reverse DNS record.**  You can retrieve the IDs associated with all your reverse DNS records using the \"Retrieve all reverse DNS records\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The ID of the reverse DNS record that you would like to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::ReverseDns**](reverse_dns.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_whitelabel_ips

> crate::models::ReverseDns p_ost_whitelabel_ips(on_behalf_of, post_whitelabel_ips_request)
Set up reverse DNS

**This endpoint allows you to set up reverse DNS.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**post_whitelabel_ips_request** | Option<[**PostWhitelabelIpsRequest**](PostWhitelabelIpsRequest.md)> |  |  |

### Return type

[**crate::models::ReverseDns**](reverse_dns.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_whitelabel_ips_id_validate

> crate::models::PostWhitelabelIpsIdValidate200Response p_ost_whitelabel_ips_id_validate(id, on_behalf_of)
Validate a reverse DNS record

**This endpoint allows you to validate a reverse DNS record.**  Always check the `valid` property of the response’s `validation_results.a_record` object. This field will indicate whether it was possible to validate the reverse DNS record. If the `validation_results.a_record.valid` is `false`, this indicates only that Twilio SendGrid could not determine the validity your reverse DNS record — it may still be valid.  If validity couldn’t be determined, you can check the value of `validation_results.a_record.reason` to find out why.  You can retrieve the IDs associated with all your reverse DNS records using the \"Retrieve all reverse DNS records\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The ID of the reverse DNS record that you would like to validate. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::PostWhitelabelIpsIdValidate200Response**](POST_whitelabel_ips_id_validate_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

