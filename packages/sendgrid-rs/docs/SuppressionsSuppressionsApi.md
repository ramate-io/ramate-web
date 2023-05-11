# \SuppressionsSuppressionsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_asm_groups_group_id_suppressions_email**](SuppressionsSuppressionsApi.md#d_elete_asm_groups_group_id_suppressions_email) | **DELETE** /asm/groups/{group_id}/suppressions/{email} | Delete a suppression from a suppression group
[**g_et_asm_groups_group_id_suppressions**](SuppressionsSuppressionsApi.md#g_et_asm_groups_group_id_suppressions) | **GET** /asm/groups/{group_id}/suppressions | Retrieve all suppressions for a suppression group
[**g_et_asm_suppressions**](SuppressionsSuppressionsApi.md#g_et_asm_suppressions) | **GET** /asm/suppressions | Retrieve all suppressions
[**g_et_asm_suppressions_email**](SuppressionsSuppressionsApi.md#g_et_asm_suppressions_email) | **GET** /asm/suppressions/{email} | Retrieve all suppression groups for an email address
[**p_ost_asm_groups_group_id_suppressions**](SuppressionsSuppressionsApi.md#p_ost_asm_groups_group_id_suppressions) | **POST** /asm/groups/{group_id}/suppressions | Add suppressions to a suppression group
[**p_ost_asm_groups_group_id_suppressions_search**](SuppressionsSuppressionsApi.md#p_ost_asm_groups_group_id_suppressions_search) | **POST** /asm/groups/{group_id}/suppressions/search | Search for suppressions within a group



## d_elete_asm_groups_group_id_suppressions_email

> serde_json::Value d_elete_asm_groups_group_id_suppressions_email(group_id, email, on_behalf_of)
Delete a suppression from a suppression group

**This endpoint allows you to remove a suppressed email address from the given suppression group.**  Removing an address will remove the suppression, meaning email will once again be sent to the previously suppressed addresses. This should be avoided unless a recipient indicates they wish to receive email from you again. You can use our [bypass filters](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) to deliver messages to otherwise suppressed addresses when exceptions are required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**serde_json::Value**](.md) | The id of the suppression group that you are removing an email address from. | [required] |
**email** | [**serde_json::Value**](.md) | The email address that you want to remove from the suppression group. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_asm_groups_group_id_suppressions

> serde_json::Value g_et_asm_groups_group_id_suppressions(group_id, on_behalf_of)
Retrieve all suppressions for a suppression group

**This endpoint allows you to retrieve all suppressed email addresses belonging to the given group.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**serde_json::Value**](.md) | The id of the unsubscribe group that you are adding suppressions to. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_asm_suppressions

> serde_json::Value g_et_asm_suppressions(on_behalf_of)
Retrieve all suppressions

**This endpoint allows you to retrieve a list of all suppressions.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_asm_suppressions_email

> crate::models::GetAsmSuppressionsEmail200Response g_et_asm_suppressions_email(email, on_behalf_of)
Retrieve all suppression groups for an email address

**This endpoint returns a list of all groups from which the given email address has been unsubscribed.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | [**serde_json::Value**](.md) | The email address that you want to search suppression groups for. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetAsmSuppressionsEmail200Response**](GET_asm_suppressions_email_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_asm_groups_group_id_suppressions

> crate::models::PostAsmGroupsGroupIdSuppressions201Response p_ost_asm_groups_group_id_suppressions(group_id, on_behalf_of, suppressions_request)
Add suppressions to a suppression group

**This endpoint allows you to add email addresses to an unsubscribe group.**  If you attempt to add suppressions to a group that has been deleted or does not exist, the suppressions will be added to the global suppressions list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**serde_json::Value**](.md) | The id of the unsubscribe group that you are adding suppressions to. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**suppressions_request** | Option<[**SuppressionsRequest**](SuppressionsRequest.md)> |  |  |

### Return type

[**crate::models::PostAsmGroupsGroupIdSuppressions201Response**](POST_asm_groups_group_id_suppressions_201_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_asm_groups_group_id_suppressions_search

> serde_json::Value p_ost_asm_groups_group_id_suppressions_search(group_id, on_behalf_of, suppressions_request)
Search for suppressions within a group

**This endpoint allows you to search a suppression group for multiple suppressions.**  When given a list of email addresses and a group ID, this endpoint will only return the email addresses that have been unsubscribed from the given group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**serde_json::Value**](.md) | The ID of the suppression group that you would like to search. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**suppressions_request** | Option<[**SuppressionsRequest**](SuppressionsRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

