# \SubusersApiApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_subusers_subuser_name**](SubusersApiApi.md#d_elete_subusers_subuser_name) | **DELETE** /subusers/{subuser_name} | Delete a subuser
[**g_et_subusers**](SubusersApiApi.md#g_et_subusers) | **GET** /subusers | List all Subusers
[**g_et_subusers_reputations**](SubusersApiApi.md#g_et_subusers_reputations) | **GET** /subusers/reputations | Retrieve Subuser Reputations
[**p_atch_subusers_subuser_name**](SubusersApiApi.md#p_atch_subusers_subuser_name) | **PATCH** /subusers/{subuser_name} | Enable/disable a subuser
[**p_ost_subusers**](SubusersApiApi.md#p_ost_subusers) | **POST** /subusers | Create Subuser
[**p_ut_subusers_subuser_name_ips**](SubusersApiApi.md#p_ut_subusers_subuser_name_ips) | **PUT** /subusers/{subuser_name}/ips | Update IPs assigned to a subuser



## d_elete_subusers_subuser_name

> serde_json::Value d_elete_subusers_subuser_name(subuser_name)
Delete a subuser

**This endpoint allows you to delete a subuser.**  This is a permanent action. Once deleted, a subuser cannot be retrieved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subuser_name** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_subusers

> serde_json::Value g_et_subusers(username, limit, offset)
List all Subusers

**This endpoint allows you to retrieve a list of all of your subusers.**  You can choose to retrieve specific subusers as well as limit the results that come back from the API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<[**serde_json::Value**](.md)> | The username of this subuser. |  |
**limit** | Option<[**serde_json::Value**](.md)> | The number of results you would like to get in each request. |  |
**offset** | Option<[**serde_json::Value**](.md)> | The number of subusers to skip. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_subusers_reputations

> serde_json::Value g_et_subusers_reputations(usernames)
Retrieve Subuser Reputations

**This endpoint allows you to request the reputations for your subusers.**  Subuser sender reputations give a good idea how well a sender is doing with regards to how recipients and recipient servers react to the mail that is being received. When a bounce, spam report, or other negative action happens on a sent email, it will affect your sender rating.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**usernames** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_subusers_subuser_name

> serde_json::Value p_atch_subusers_subuser_name(subuser_name, patch_subusers_subuser_name_request)
Enable/disable a subuser

**This endpoint allows you to enable or disable a subuser.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subuser_name** | [**serde_json::Value**](.md) |  | [required] |
**patch_subusers_subuser_name_request** | Option<[**PatchSubusersSubuserNameRequest**](PatchSubusersSubuserNameRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_subusers

> crate::models::SubuserPost p_ost_subusers(post_subusers_request)
Create Subuser

**This endpoint allows you to create a new subuser.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_subusers_request** | Option<[**PostSubusersRequest**](PostSubusersRequest.md)> |  |  |

### Return type

[**crate::models::SubuserPost**](subuser_post.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ut_subusers_subuser_name_ips

> crate::models::PutSubusersSubuserNameIps200Response p_ut_subusers_subuser_name_ips(subuser_name, body)
Update IPs assigned to a subuser

**This endpoint allows you update your subusers' assigned IP.**  Each subuser should be assigned to an IP address from which all of this subuser's mail will be sent. Often, this is the same IP as the parent account, but each subuser can have one or more of their own IP addresses as well.   More information:  * [How to request more IPs](https://sendgrid.com/docs/ui/account-and-settings/dedicated-ip-addresses/) * [Setup Reverse DNS](https://sendgrid.com/docs/ui/account-and-settings/how-to-set-up-reverse-dns/)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subuser_name** | [**serde_json::Value**](.md) |  | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::PutSubusersSubuserNameIps200Response**](PUT_subusers_subuser_name_ips_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

