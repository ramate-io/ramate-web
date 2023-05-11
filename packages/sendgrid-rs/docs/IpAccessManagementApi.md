# \IpAccessManagementApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_access_settings_whitelist**](IpAccessManagementApi.md#d_elete_access_settings_whitelist) | **DELETE** /access_settings/whitelist | Remove one or more IPs from the allow list
[**d_elete_access_settings_whitelist_rule_id**](IpAccessManagementApi.md#d_elete_access_settings_whitelist_rule_id) | **DELETE** /access_settings/whitelist/{rule_id} | Remove a specific IP from the allowed list
[**g_et_access_settings_activity**](IpAccessManagementApi.md#g_et_access_settings_activity) | **GET** /access_settings/activity | Retrieve all recent access attempts
[**g_et_access_settings_whitelist**](IpAccessManagementApi.md#g_et_access_settings_whitelist) | **GET** /access_settings/whitelist | Retrieve a list of currently allowed IPs
[**g_et_access_settings_whitelist_rule_id**](IpAccessManagementApi.md#g_et_access_settings_whitelist_rule_id) | **GET** /access_settings/whitelist/{rule_id} | Retrieve a specific allowed IP
[**p_ost_access_settings_whitelist**](IpAccessManagementApi.md#p_ost_access_settings_whitelist) | **POST** /access_settings/whitelist | Add one or more IPs to the allow list



## d_elete_access_settings_whitelist

> serde_json::Value d_elete_access_settings_whitelist(on_behalf_of, delete_access_settings_whitelist_request)
Remove one or more IPs from the allow list

**This endpoint allows you to remove one or more IP addresses from your list of allowed addresses.**  To remove one or more IP addresses, pass this endpoint an array containing the ID(s) associated with the IP(s) you intend to remove. You can retrieve the IDs associated with your allowed IP addresses using the \"Retrieve a list of currently allowed IPs\" endpoint.  It is possible to remove your own IP address, which will block access to your account. You will need to submit a [support ticket](https://sendgrid.com/docs/ui/account-and-settings/support/) if this happens. For this reason, it is important to double check that you are removing only the IPs you intend to remove when using this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**delete_access_settings_whitelist_request** | Option<[**DeleteAccessSettingsWhitelistRequest**](DeleteAccessSettingsWhitelistRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_elete_access_settings_whitelist_rule_id

> serde_json::Value d_elete_access_settings_whitelist_rule_id(rule_id, on_behalf_of)
Remove a specific IP from the allowed list

**This endpoint allows you to remove a specific IP address from your list of allowed addresses.**  When removing a specific IP address from your list, you must include the ID in your call.  You can retrieve the IDs associated with your allowed IP addresses using the \"Retrieve a list of currently allowed IPs\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | [**serde_json::Value**](.md) | The ID of the allowed IP address that you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_access_settings_activity

> crate::models::GetAccessSettingsActivity200Response g_et_access_settings_activity(limit, on_behalf_of)
Retrieve all recent access attempts

**This endpoint allows you to retrieve a list of all of the IP addresses that recently attempted to access your account either through the User Interface or the API.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<[**serde_json::Value**](.md)> | Limits the number of IPs to return. |  |[default to 20]
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetAccessSettingsActivity200Response**](GET_access_settings_activity_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_access_settings_whitelist

> crate::models::IpAccessResponse g_et_access_settings_whitelist(on_behalf_of)
Retrieve a list of currently allowed IPs

**This endpoint allows you to retrieve a list of IP addresses that are currently allowed to access your account.**  Each IP address returned to you will have `created_at` and `updated_at` dates. Each IP will also be associated with an `id` that can be used to remove the address from your allow list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::IpAccessResponse**](ip-access-response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_access_settings_whitelist_rule_id

> crate::models::IpAccessResponse g_et_access_settings_whitelist_rule_id(rule_id, on_behalf_of)
Retrieve a specific allowed IP

**This endpoint allows you to retreive a specific IP address that has been allowed to access your account.**  You must include the ID for the specific IP address you want to retrieve in your call. You can retrieve the IDs associated with your allowed IP addresses using the \"Retrieve a  list of currently allowed IPs\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | [**serde_json::Value**](.md) | The ID of the allowed IP address that you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::IpAccessResponse**](ip-access-response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_access_settings_whitelist

> crate::models::IpAccessResponse p_ost_access_settings_whitelist(on_behalf_of, post_access_settings_whitelist_request)
Add one or more IPs to the allow list

**This endpoint allows you to add one or more allowed IP addresses.**  To allow one or more IP addresses, pass them to this endpoint in an array. Once an IP address is added to your allow list, it will be assigned an `id` that can be used to remove the address. You can retrieve the ID associated with an IP using the \"Retrieve a list of currently allowed IPs\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**post_access_settings_whitelist_request** | Option<[**PostAccessSettingsWhitelistRequest**](PostAccessSettingsWhitelistRequest.md)> |  |  |

### Return type

[**crate::models::IpAccessResponse**](ip-access-response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

