# \IpAddressesApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**g_et_ips**](IpAddressesApi.md#g_et_ips) | **GET** /ips | Retrieve all IP addresses
[**g_et_ips_assigned**](IpAddressesApi.md#g_et_ips_assigned) | **GET** /ips/assigned | Retrieve all assigned IPs
[**g_et_ips_ip_address**](IpAddressesApi.md#g_et_ips_ip_address) | **GET** /ips/{ip_address} | Retrieve all IP pools an IP address belongs to
[**g_et_ips_remaining**](IpAddressesApi.md#g_et_ips_remaining) | **GET** /ips/remaining | Get remaining IPs count
[**p_ost_ips**](IpAddressesApi.md#p_ost_ips) | **POST** /ips | Add IPs



## g_et_ips

> serde_json::Value g_et_ips(ip, exclude_whitelabels, limit, offset, subuser, sort_by_direction)
Retrieve all IP addresses

**This endpoint allows you to retrieve a list of all assigned and unassigned IPs.**  Response includes warm up status, pools, assigned subusers, and reverse DNS info. The start_date field corresponds to when warmup started for that IP.  A single IP address or a range of IP addresses may be dedicated to an account in order to send email for multiple domains. The reputation of this IP is based on the aggregate performance of all the senders who use it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip** | Option<[**serde_json::Value**](.md)> | The IP address to get |  |
**exclude_whitelabels** | Option<[**serde_json::Value**](.md)> | Should we exclude reverse DNS records (whitelabels)? |  |
**limit** | Option<[**serde_json::Value**](.md)> | The number of IPs you want returned at the same time. |  |[default to 10]
**offset** | Option<[**serde_json::Value**](.md)> | The offset for the number of IPs that you are requesting. |  |[default to 0]
**subuser** | Option<[**serde_json::Value**](.md)> | The subuser you are requesting for. |  |
**sort_by_direction** | Option<[**serde_json::Value**](.md)> | The direction to sort the results. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_ips_assigned

> serde_json::Value g_et_ips_assigned()
Retrieve all assigned IPs

**This endpoint allows you to retrieve only assigned IP addresses.**  A single IP address or a range of IP addresses may be dedicated to an account in order to send email for multiple domains. The reputation of this IP is based on the aggregate performance of all the senders who use it.

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


## g_et_ips_ip_address

> crate::models::GetIpsIpAddress200Response g_et_ips_ip_address(ip_address)
Retrieve all IP pools an IP address belongs to

**This endpoint allows you to see which IP pools a particular IP address has been added to.**  The same IP address can be added to multiple IP pools.  A single IP address or a range of IP addresses may be dedicated to an account in order to send email for multiple domains. The reputation of this IP is based on the aggregate performance of all the senders who use it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip_address** | [**serde_json::Value**](.md) | The IP address you are retrieving the IP pools for. | [required] |

### Return type

[**crate::models::GetIpsIpAddress200Response**](GET_ips_ip_address_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_ips_remaining

> crate::models::GetIpsRemaining200Response g_et_ips_remaining()
Get remaining IPs count

**This endpoint gets amount of IP Addresses that can still be created during a given period and the price of those IPs.**

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetIpsRemaining200Response**](GET_ips_remaining_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_ips

> crate::models::PostIps201Response p_ost_ips(post_ips_request)
Add IPs

**This endpoint is for adding a(n) IP Address(es) to your account.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_ips_request** | Option<[**PostIpsRequest**](PostIpsRequest.md)> |  |  |

### Return type

[**crate::models::PostIps201Response**](POST_ips_201_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

