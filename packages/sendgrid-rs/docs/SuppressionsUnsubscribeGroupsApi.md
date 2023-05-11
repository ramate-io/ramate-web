# \SuppressionsUnsubscribeGroupsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_asm_groups_group_id**](SuppressionsUnsubscribeGroupsApi.md#d_elete_asm_groups_group_id) | **DELETE** /asm/groups/{group_id} | Delete a Suppression Group
[**g_et_asm_groups**](SuppressionsUnsubscribeGroupsApi.md#g_et_asm_groups) | **GET** /asm/groups | Retrieve all suppression groups associated with the user.
[**g_et_asm_groups_group_id**](SuppressionsUnsubscribeGroupsApi.md#g_et_asm_groups_group_id) | **GET** /asm/groups/{group_id} | Get information on a single suppression group.
[**p_atch_asm_groups_group_id**](SuppressionsUnsubscribeGroupsApi.md#p_atch_asm_groups_group_id) | **PATCH** /asm/groups/{group_id} | Update a suppression group.
[**p_ost_asm_groups**](SuppressionsUnsubscribeGroupsApi.md#p_ost_asm_groups) | **POST** /asm/groups | Create a new suppression group



## d_elete_asm_groups_group_id

> serde_json::Value d_elete_asm_groups_group_id(group_id, on_behalf_of)
Delete a Suppression Group

**This endpoint allows you to delete a suppression group.**  If a recipient uses the \"one-click unsubscribe\" option on an email associated with a deleted group, that recipient will be added to the global suppression list.  Deleting a suppression group will remove the suppression, meaning email will once again be sent to the previously suppressed addresses. This should be avoided unless a recipient indicates they wish to receive email from you again. You can use our [bypass filters](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) to deliver messages to otherwise suppressed addresses when exceptions are required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**serde_json::Value**](.md) | The ID of the suppression group you would like to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_asm_groups

> serde_json::Value g_et_asm_groups(id, on_behalf_of)
Retrieve all suppression groups associated with the user.

**This endpoint allows you to retrieve a list of all suppression groups created by this user.**  This endpoint can also return information for multiple group IDs that you include in your request. To add a group ID to your request, simply append `?id=123456&id=123456`, with the appropriate group IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<[**serde_json::Value**](.md)> |  |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_asm_groups_group_id

> serde_json::Value g_et_asm_groups_group_id(group_id, on_behalf_of)
Get information on a single suppression group.

**This endpoint allows you to retrieve a single suppression group.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**serde_json::Value**](.md) | The ID of the suppression group you would like to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_asm_groups_group_id

> crate::models::SuppressionGroup p_atch_asm_groups_group_id(group_id, on_behalf_of, body)
Update a suppression group.

**This endpoint allows you to update or change a suppression group.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**serde_json::Value**](.md) | The ID of the suppression group you would like to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**body** | Option<**crate::models::SuppressionGroupRequestBase**> |  |  |

### Return type

[**crate::models::SuppressionGroup**](suppression_group.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_asm_groups

> crate::models::PostAsmGroups201Response p_ost_asm_groups(on_behalf_of, suppression_group_request_base)
Create a new suppression group

**This endpoint allows you to create a new suppression group.**  To add an email address to the suppression group, [create a Suppression](https://sendgrid.api-docs.io/v3.0/suppressions-suppressions/add-suppressions-to-a-suppression-group).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**suppression_group_request_base** | Option<[**SuppressionGroupRequestBase**](SuppressionGroupRequestBase.md)> |  |  |

### Return type

[**crate::models::PostAsmGroups201Response**](POST_asm_groups_201_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

