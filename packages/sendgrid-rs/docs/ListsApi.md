# \ListsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_lists_id**](ListsApi.md#d_elete_lists_id) | **DELETE** /marketing/lists/{id} | Delete a list
[**d_eletemc_lists_id_contacts**](ListsApi.md#d_eletemc_lists_id_contacts) | **DELETE** /marketing/lists/{id}/contacts | Remove Contacts from a List
[**g_etmc_lists**](ListsApi.md#g_etmc_lists) | **GET** /marketing/lists | Get All Lists
[**g_etmc_lists_id**](ListsApi.md#g_etmc_lists_id) | **GET** /marketing/lists/{id} | Get a List by ID
[**g_etmc_lists_id_contacts_count**](ListsApi.md#g_etmc_lists_id_contacts_count) | **GET** /marketing/lists/{id}/contacts/count | Get List Contact Count
[**p_atchmc_lists_id**](ListsApi.md#p_atchmc_lists_id) | **PATCH** /marketing/lists/{id} | Update List
[**p_ostmc_lists**](ListsApi.md#p_ostmc_lists) | **POST** /marketing/lists | Create List



## d_elete_lists_id

> crate::models::DeleteListsId200Response d_elete_lists_id(id, delete_contacts)
Delete a list

**This endpoint allows you to deletes a specific list.**  Optionally, you can also delete contacts associated to the list. The query parameter, `delete_contacts=true`, will delete the list and start an asynchronous job to delete associated contacts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |
**delete_contacts** | Option<[**serde_json::Value**](.md)> | Flag indicates that all contacts on the list are also to be deleted. |  |[default to false]

### Return type

[**crate::models::DeleteListsId200Response**](DELETE_lists_id_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_eletemc_lists_id_contacts

> crate::models::DeleteMcListsIdContacts202Response d_eletemc_lists_id_contacts(contact_ids, id)
Remove Contacts from a List

**This endpoint allows you to remove contacts from a given list.**  The contacts will not be deleted. Only their list membership will be changed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_ids** | [**serde_json::Value**](.md) | comma separated list of contact ids | [required] |
**id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**crate::models::DeleteMcListsIdContacts202Response**](DELETE_mc_lists_id_contacts_202_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_etmc_lists

> crate::models::GetMcLists200Response g_etmc_lists(page_size, page_token)
Get All Lists

**This endpoint returns an array of all of your contact lists.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<[**serde_json::Value**](.md)> | Maximum number of elements to return. Defaults to 100, returns 1000 max |  |[default to 100]
**page_token** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

[**crate::models::GetMcLists200Response**](GET_mc_lists_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_etmc_lists_id

> serde_json::Value g_etmc_lists_id(id, contact_sample)
Get a List by ID

**This endpoint returns data about a specific list.**  Setting the optional parameter `contact_sample=true` returns the `contact_sample` in the response body. Up to fifty of the most recent contacts uploaded or attached to a list will be returned, sorted alphabetically, by email address.  The full contact count is also returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |
**contact_sample** | Option<[**serde_json::Value**](.md)> | Setting this parameter to the true  will cause the contact_sample to be returned |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_etmc_lists_id_contacts_count

> crate::models::GetMcListsIdContactsCount200Response g_etmc_lists_id_contacts_count(id)
Get List Contact Count

**This endpoint returns the number of contacts on a specific list.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**crate::models::GetMcListsIdContactsCount200Response**](GET_mc_lists_id_contacts_count_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atchmc_lists_id

> crate::models::List p_atchmc_lists_id(id, patch_mc_lists_id_request)
Update List

**This endpoint updates the name of a list.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |
**patch_mc_lists_id_request** | Option<[**PatchMcListsIdRequest**](PatchMcListsIdRequest.md)> |  |  |

### Return type

[**crate::models::List**](list.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ostmc_lists

> crate::models::List p_ostmc_lists(post_mc_lists_request)
Create List

**This endpoint creates a new contacts list.**  Once you create a list, you can use the UI to [trigger an automation](https://sendgrid.com/docs/ui/sending-email/getting-started-with-automation/#create-an-automation) every time you add a new contact to the list.  A link to the newly created object is in `_metadata`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_mc_lists_request** | Option<[**PostMcListsRequest**](PostMcListsRequest.md)> |  |  |

### Return type

[**crate::models::List**](list.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

