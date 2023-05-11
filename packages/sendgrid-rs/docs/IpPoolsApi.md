# \IpPoolsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_ips_pools_pool_name**](IpPoolsApi.md#d_elete_ips_pools_pool_name) | **DELETE** /ips/pools/{pool_name} | Delete an IP pool
[**d_elete_ips_pools_pool_name_ips_ip**](IpPoolsApi.md#d_elete_ips_pools_pool_name_ips_ip) | **DELETE** /ips/pools/{pool_name}/ips/{ip} | Remove an IP address from a pool
[**g_et_ips_pools**](IpPoolsApi.md#g_et_ips_pools) | **GET** /ips/pools | Retrieve all IP pools
[**g_et_ips_pools_pool_name**](IpPoolsApi.md#g_et_ips_pools_pool_name) | **GET** /ips/pools/{pool_name} | Retrieve all the IPs in a specified pool
[**p_ost_ips_pools**](IpPoolsApi.md#p_ost_ips_pools) | **POST** /ips/pools | Create an IP pool
[**p_ost_ips_pools_pool_name_ips**](IpPoolsApi.md#p_ost_ips_pools_pool_name_ips) | **POST** /ips/pools/{pool_name}/ips | Add an IP address to a pool
[**p_ut_ips_pools_pool_name**](IpPoolsApi.md#p_ut_ips_pools_pool_name) | **PUT** /ips/pools/{pool_name} | Rename an IP pool



## d_elete_ips_pools_pool_name

> serde_json::Value d_elete_ips_pools_pool_name(pool_name)
Delete an IP pool

**This endpoint allows you to delete an IP pool.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_name** | [**serde_json::Value**](.md) | The name of the IP pool that you want to retrieve the IP addresses for. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_elete_ips_pools_pool_name_ips_ip

> serde_json::Value d_elete_ips_pools_pool_name_ips_ip(pool_name, ip)
Remove an IP address from a pool

**This endpoint allows you to remove an IP address from an IP pool.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_name** | [**serde_json::Value**](.md) | The name of the IP pool that you are removing the IP address from. | [required] |
**ip** | [**serde_json::Value**](.md) | The IP address that you wish to remove. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_ips_pools

> serde_json::Value g_et_ips_pools()
Retrieve all IP pools

**This endpoint allows you to get all of your IP pools.**

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


## g_et_ips_pools_pool_name

> crate::models::GetIpsPoolsPoolName200Response g_et_ips_pools_pool_name(pool_name)
Retrieve all the IPs in a specified pool

**This endpoint allows you to get all of the IP addresses that are in a specific IP pool.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_name** | [**serde_json::Value**](.md) | The name of the IP pool that you want to retrieve the IP addresses for. | [required] |

### Return type

[**crate::models::GetIpsPoolsPoolName200Response**](GET_ips_pools_pool_name_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_ips_pools

> crate::models::IpPoolResponse p_ost_ips_pools(post_ips_pools_request)
Create an IP pool

**This endpoint allows you to create an IP pool.**  Before you can create an IP pool, you need to activate the IP in your SendGrid account:   1. Log into your SendGrid account.   1. Navigate to **Settings** and then select **IP Addresses**.   1. Find the IP address you want to activate and then click **Edit**.   1. Check **Allow my account to send mail using this IP address**. 1. Click **Save**.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_ips_pools_request** | Option<[**PostIpsPoolsRequest**](PostIpsPoolsRequest.md)> |  |  |

### Return type

[**crate::models::IpPoolResponse**](ip_pool_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_ips_pools_pool_name_ips

> crate::models::PostIpsPoolsPoolNameIps201Response p_ost_ips_pools_pool_name_ips(pool_name, post_ips_pools_pool_name_ips_request)
Add an IP address to a pool

**This endpoint allows you to add an IP address to an IP pool.**  You can add the same IP address to multiple pools. It may take up to 60 seconds for your IP address to be added to a pool after your request is made.  Before you can add an IP to a pool, you need to activate it in your SendGrid account:  1. Log into your SendGrid account.   1. Navigate to **Settings** and then select **IP Addresses**.   1. Find the IP address you want to activate and then click **Edit**.   1. Check **Allow my account to send mail using this IP address**. 1. Click **Save**.  You can retrieve all of your available IP addresses from the \"Retrieve all IP addresses\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_name** | [**serde_json::Value**](.md) | The name of the IP pool you want to add the address to. If the name contains spaces, they must be URL encoded (e.g., \"Test Pool\" becomes \"Test%20Pool\"). | [required] |
**post_ips_pools_pool_name_ips_request** | Option<[**PostIpsPoolsPoolNameIpsRequest**](PostIpsPoolsPoolNameIpsRequest.md)> |  |  |

### Return type

[**crate::models::PostIpsPoolsPoolNameIps201Response**](POST_ips_pools_pool_name_ips_201_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ut_ips_pools_pool_name

> crate::models::IpPoolResponse p_ut_ips_pools_pool_name(pool_name, put_ips_pools_pool_name_request)
Rename an IP pool

**This endpoint allows you to update the name of an IP pool.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_name** | [**serde_json::Value**](.md) | The name of the IP pool that you want to retrieve the IP addresses for. | [required] |
**put_ips_pools_pool_name_request** | Option<[**PutIpsPoolsPoolNameRequest**](PutIpsPoolsPoolNameRequest.md)> |  |  |

### Return type

[**crate::models::IpPoolResponse**](ip_pool_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

