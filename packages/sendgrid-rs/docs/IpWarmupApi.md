# \IpWarmupApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_ips_warmup_ip_address**](IpWarmupApi.md#d_elete_ips_warmup_ip_address) | **DELETE** /ips/warmup/{ip_address} | Stop warming up an IP address
[**g_et_ips_warmup**](IpWarmupApi.md#g_et_ips_warmup) | **GET** /ips/warmup | Retrieve all IPs currently in warmup
[**g_et_ips_warmup_ip_address**](IpWarmupApi.md#g_et_ips_warmup_ip_address) | **GET** /ips/warmup/{ip_address} | Retrieve the warmup status for a specific IP address
[**p_ost_ips_warmup**](IpWarmupApi.md#p_ost_ips_warmup) | **POST** /ips/warmup | Start warming up an IP address



## d_elete_ips_warmup_ip_address

> serde_json::Value d_elete_ips_warmup_ip_address(ip_address)
Stop warming up an IP address

**This endpoint allows you to remove an IP address from warmup mode.**  Your request will return a 204 status code if the specified IP was successfully removed from warmup mode. To retrieve details of the IP’s warmup status *before* removing it from warmup mode, call the  \"Retrieve the warmpup status for a specific IP address\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip_address** | [**serde_json::Value**](.md) | The IP address that you want to retrieve the warmup status for. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_ips_warmup

> serde_json::Value g_et_ips_warmup()
Retrieve all IPs currently in warmup

**This endpoint allows you to retrieve all of your IP addresses that are currently warming up.**

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_ips_warmup_ip_address

> serde_json::Value g_et_ips_warmup_ip_address(ip_address)
Retrieve the warmup status for a specific IP address

**This endpoint allows you to retrieve the warmup status for a specific IP address.**  You can retrieve all of your warming IPs using the \"Retrieve all IPs currently in warmup\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip_address** | [**serde_json::Value**](.md) | The IP address that you want to retrieve the warmup status for. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_ips_warmup

> serde_json::Value p_ost_ips_warmup(post_ips_warmup_request)
Start warming up an IP address

**This endpoint allows you to put an IP address into warmup mode.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_ips_warmup_request** | Option<[**PostIpsWarmupRequest**](PostIpsWarmupRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

