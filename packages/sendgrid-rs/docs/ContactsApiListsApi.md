# \ContactsApiListsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_contactdb_lists**](ContactsApiListsApi.md#d_elete_contactdb_lists) | **DELETE** /contactdb/lists | Delete Multiple lists
[**d_elete_contactdb_lists_list_id**](ContactsApiListsApi.md#d_elete_contactdb_lists_list_id) | **DELETE** /contactdb/lists/{list_id} | Delete a List
[**d_elete_contactdb_lists_list_id_recipients_recipient_id**](ContactsApiListsApi.md#d_elete_contactdb_lists_list_id_recipients_recipient_id) | **DELETE** /contactdb/lists/{list_id}/recipients/{recipient_id} | Delete a Single Recipient from a Single List
[**g_et_contactdb_lists**](ContactsApiListsApi.md#g_et_contactdb_lists) | **GET** /contactdb/lists | Retrieve all lists
[**g_et_contactdb_lists_list_id**](ContactsApiListsApi.md#g_et_contactdb_lists_list_id) | **GET** /contactdb/lists/{list_id} | Retrieve a single list
[**g_et_contactdb_lists_list_id_recipients**](ContactsApiListsApi.md#g_et_contactdb_lists_list_id_recipients) | **GET** /contactdb/lists/{list_id}/recipients | Retrieve all recipients on a List
[**p_atch_contactdb_lists_list_id**](ContactsApiListsApi.md#p_atch_contactdb_lists_list_id) | **PATCH** /contactdb/lists/{list_id} | Update a List
[**p_ost_contactdb_lists**](ContactsApiListsApi.md#p_ost_contactdb_lists) | **POST** /contactdb/lists | Create a List
[**p_ost_contactdb_lists_list_id_recipients**](ContactsApiListsApi.md#p_ost_contactdb_lists_list_id_recipients) | **POST** /contactdb/lists/{list_id}/recipients | Add Multiple Recipients to a List
[**p_ost_contactdb_lists_list_id_recipients_recipient_id**](ContactsApiListsApi.md#p_ost_contactdb_lists_list_id_recipients_recipient_id) | **POST** /contactdb/lists/{list_id}/recipients/{recipient_id} | Add a Single Recipient to a List



## d_elete_contactdb_lists

> serde_json::Value d_elete_contactdb_lists(on_behalf_of, body)
Delete Multiple lists

**This endpoint allows you to delete multiple recipient lists.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_elete_contactdb_lists_list_id

> serde_json::Value d_elete_contactdb_lists_list_id(list_id, delete_contacts, on_behalf_of, body)
Delete a List

**This endpoint allows you to delete a specific recipient list with the given ID.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | [**serde_json::Value**](.md) |  | [required] |
**delete_contacts** | Option<[**serde_json::Value**](.md)> | Adds the ability to delete all contacts on the list in addition to deleting the list. |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_elete_contactdb_lists_list_id_recipients_recipient_id

> serde_json::Value d_elete_contactdb_lists_list_id_recipients_recipient_id(list_id, recipient_id, list_id2, recipient_id2, on_behalf_of, body)
Delete a Single Recipient from a Single List

**This endpoint allows you to delete a single recipient from a list.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | [**serde_json::Value**](.md) | The ID of the list you are taking this recipient away from. | [required] |
**recipient_id** | [**serde_json::Value**](.md) | The ID of the recipient to take off the list. | [required] |
**list_id2** | [**serde_json::Value**](.md) | The ID of the list that you want to add the recipient to. | [required] |
**recipient_id2** | [**serde_json::Value**](.md) | The ID of the recipient you are adding to the list. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_contactdb_lists

> crate::models::ListAllListsResponse g_et_contactdb_lists(on_behalf_of)
Retrieve all lists

**This endpoint allows you to retrieve all of your recipient lists. If you don't have any lists, an empty array will be returned.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::ListAllListsResponse**](List_All_Lists_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_contactdb_lists_list_id

> crate::models::ContactdbList g_et_contactdb_lists_list_id(list_id2, list_id, on_behalf_of)
Retrieve a single list

**This endpoint allows you to retrieve a single recipient list.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id2** | [**serde_json::Value**](.md) |  | [required] |
**list_id** | Option<[**serde_json::Value**](.md)> | The ID of the list to retrieve. |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::ContactdbList**](contactdb_list.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_contactdb_lists_list_id_recipients

> crate::models::GetContactdbListsListIdRecipients200Response g_et_contactdb_lists_list_id_recipients(list_id, list_id2, page, page_size, on_behalf_of)
Retrieve all recipients on a List

**This endpoint allows you to retrieve all recipients on the list with the given ID.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | [**serde_json::Value**](.md) | The ID of the list whose recipients you are requesting. | [required] |
**list_id2** | [**serde_json::Value**](.md) | The id of the list of recipients you want to retrieve. | [required] |
**page** | Option<[**serde_json::Value**](.md)> | Page index of first recipient to return (must be a positive integer) |  |
**page_size** | Option<[**serde_json::Value**](.md)> | Number of recipients to return at a time (must be a positive integer between 1 and 1000) |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetContactdbListsListIdRecipients200Response**](GET_contactdb_lists_list_id_recipients_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_contactdb_lists_list_id

> crate::models::PatchContactdbListsListId200Response p_atch_contactdb_lists_list_id(list_id, list_id2, on_behalf_of, update_a_list_request)
Update a List

**This endpoint allows you to update the name of one of your recipient lists.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | [**serde_json::Value**](.md) | The ID of the list you are updating. | [required] |
**list_id2** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**update_a_list_request** | Option<[**UpdateAListRequest**](UpdateAListRequest.md)> |  |  |

### Return type

[**crate::models::PatchContactdbListsListId200Response**](PATCH_contactdb_lists_list_id_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_contactdb_lists

> crate::models::ContactdbList p_ost_contactdb_lists(on_behalf_of, create_a_list_request)
Create a List

**This endpoint allows you to create a list for your recipients.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**create_a_list_request** | Option<[**CreateAListRequest**](CreateAListRequest.md)> |  |  |

### Return type

[**crate::models::ContactdbList**](contactdb_list.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_contactdb_lists_list_id_recipients

> serde_json::Value p_ost_contactdb_lists_list_id_recipients(list_id, on_behalf_of, body)
Add Multiple Recipients to a List

**This endpoint allows you to add multiple recipients to a list.**  Adds existing recipients to a list, passing in the recipient IDs to add. Recipient IDs should be passed exactly as they are returned from recipient endpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | [**serde_json::Value**](.md) | The id of the list of recipients you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_contactdb_lists_list_id_recipients_recipient_id

> serde_json::Value p_ost_contactdb_lists_list_id_recipients_recipient_id(list_id, recipient_id, on_behalf_of)
Add a Single Recipient to a List

**This endpoint allows you to add a single recipient to a list.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | [**serde_json::Value**](.md) | The ID of the list that you want to add the recipient to. | [required] |
**recipient_id** | [**serde_json::Value**](.md) | The ID of the recipient you are adding to the list. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

