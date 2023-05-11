# \SingleSendsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_marketing_singlesends**](SingleSendsApi.md#d_elete_marketing_singlesends) | **DELETE** /marketing/singlesends | Bulk Delete Single Sends
[**d_elete_marketing_singlesends_id**](SingleSendsApi.md#d_elete_marketing_singlesends_id) | **DELETE** /marketing/singlesends/{id} | Delete Single Send by ID
[**d_elete_marketing_singlesends_id_schedule**](SingleSendsApi.md#d_elete_marketing_singlesends_id_schedule) | **DELETE** /marketing/singlesends/{id}/schedule | Delete Single Send Schedule
[**g_et_marketing_singlesends**](SingleSendsApi.md#g_et_marketing_singlesends) | **GET** /marketing/singlesends | Get All Single Sends
[**g_et_marketing_singlesends_categories**](SingleSendsApi.md#g_et_marketing_singlesends_categories) | **GET** /marketing/singlesends/categories | Get All Categories
[**g_et_marketing_singlesends_id**](SingleSendsApi.md#g_et_marketing_singlesends_id) | **GET** /marketing/singlesends/{id} | Get Single Send by ID
[**p_atch_marketing_singlesends_id**](SingleSendsApi.md#p_atch_marketing_singlesends_id) | **PATCH** /marketing/singlesends/{id} | Update Single Send
[**p_ost_marketing_singlesends**](SingleSendsApi.md#p_ost_marketing_singlesends) | **POST** /marketing/singlesends | Create Single Send
[**p_ost_marketing_singlesends_id**](SingleSendsApi.md#p_ost_marketing_singlesends_id) | **POST** /marketing/singlesends/{id} | Duplicate Single Send
[**p_ost_marketing_singlesends_search**](SingleSendsApi.md#p_ost_marketing_singlesends_search) | **POST** /marketing/singlesends/search | Get Single Sends Search
[**p_ut_marketing_singlesends_id_schedule**](SingleSendsApi.md#p_ut_marketing_singlesends_id_schedule) | **PUT** /marketing/singlesends/{id}/schedule | Schedule Single Send



## d_elete_marketing_singlesends

> d_elete_marketing_singlesends(ids)
Bulk Delete Single Sends

**This endpoint allows you to delete multiple Single Sends using an array of Single Sends IDs.**  To first retrieve all your Single Sends' IDs, you can make a GET request to the `/marketing/singlensends` endpoint.  Please note that a DELETE request is permanent, and your Single Sends will not be recoverable after deletion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**serde_json::Value**](.md)> | Single Send IDs to delete |  |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_elete_marketing_singlesends_id

> d_elete_marketing_singlesends_id(id)
Delete Single Send by ID

**This endpoint allows you to delete one Single Send using a Single Send ID.**  To first retrieve all your Single Sends' IDs, you can make a GET request to the `/marketing/singlensends` endpoint.  Please note that a `DELETE` request is permanent, and your Single Send will not be recoverable after deletion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_elete_marketing_singlesends_id_schedule

> crate::models::SinglesendSchedule d_elete_marketing_singlesends_id_schedule(id)
Delete Single Send Schedule

**This endpoint allows you to cancel a scheduled Single Send using a Single Send ID.**  Making a DELETE request to this endpoint will cancel the scheduled sending of a Single Send. The request will not delete the Single Send itself. Deleting a Single Send can be done by passing a DELETE request to `/marketing/singlesends/{id}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**crate::models::SinglesendSchedule**](singlesend_schedule.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_marketing_singlesends

> crate::models::GetMarketingSinglesends200Response g_et_marketing_singlesends(page_size, page_token)
Get All Single Sends

**This endpoint allows you to retrieve all your Single Sends.**  Returns all of your Single Sends with condensed details about each, including the Single Sends' IDs. For more details about an individual Single Send, pass the Single Send's ID to the `/marketing/singlesends/{id}` endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<[**serde_json::Value**](.md)> |  |  |
**page_token** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

[**crate::models::GetMarketingSinglesends200Response**](GET_marketing_singlesends_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_marketing_singlesends_categories

> crate::models::GetMarketingSinglesendsCategories200Response g_et_marketing_singlesends_categories()
Get All Categories

**This endpoint allows you to retrieve all the categories associated with your Single Sends.**  This endpoint will return your latest 1,000 categories.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetMarketingSinglesendsCategories200Response**](GET_marketing_singlesends_categories_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_marketing_singlesends_id

> serde_json::Value g_et_marketing_singlesends_id(id)
Get Single Send by ID

**This endpoint allows you to retrieve details about one Single Send using a Single Send ID.**  You can retrieve all of your Single Sends by making a GET request to the `/marketing/singlesends` endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_marketing_singlesends_id

> serde_json::Value p_atch_marketing_singlesends_id(id, singlesend_request)
Update Single Send

**This endpoint allows you to update a Single Send using a Single Send ID.**  You only need to pass the fields you want to update. Any blank/missing fields will remain unaltered.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |
**singlesend_request** | Option<[**SinglesendRequest**](SinglesendRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_marketing_singlesends

> serde_json::Value p_ost_marketing_singlesends(singlesend_request)
Create Single Send

**This endpoint allows you to create a new Single Send.**  Please note that if you are migrating from the previous version of Single Sends, you no longer need to pass a template ID with your request to this endpoint. Instead, you will pass all template data in the `email_config` object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**singlesend_request** | Option<[**SinglesendRequest**](SinglesendRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_marketing_singlesends_id

> serde_json::Value p_ost_marketing_singlesends_id(id, post_marketing_singlesends_id_request)
Duplicate Single Send

**This endpoint allows you to duplicate an existing Single Send using its Single Send ID.**  Duplicating a Single Send is useful when you want to create a Single Send but don't want to start from scratch. Once duplicated, you can update or edit the Single Send by making a PATCH request to the `/marketing/singlesends/{id}` endpoint.   If you leave the `name` field blank, your duplicate will be assigned the name of the Single Send it was copied from with the text “Copy of ” prepended to it. The `name` field length is limited to 100 characters, so the end of the new Single Send name, including “Copy of ”, will be trimmed if the name exceeds this limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |
**post_marketing_singlesends_id_request** | Option<[**PostMarketingSinglesendsIdRequest**](PostMarketingSinglesendsIdRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_marketing_singlesends_search

> crate::models::GetMarketingSinglesends200Response p_ost_marketing_singlesends_search(page_size, page_token, singlesend_search)
Get Single Sends Search

**This endpoint allows you to search for Single Sends based on specified criteria.**  You can search for Single Sends by passing a combination of values using the `name`, `status`, and `categories` request body fields.  For example, if you want to search for all Single Sends that are \"drafts\" or \"scheduled\" and also associated with the category \"shoes,\" your request body may look like the example below.  ```javascript {   \"status\": [     \"draft\",     \"scheduled\"   ],   \"categories\": [     \"shoes\"   ], } ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<[**serde_json::Value**](.md)> |  |  |
**page_token** | Option<[**serde_json::Value**](.md)> |  |  |
**singlesend_search** | Option<[**SinglesendSearch**](SinglesendSearch.md)> |  |  |

### Return type

[**crate::models::GetMarketingSinglesends200Response**](GET_marketing_singlesends_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ut_marketing_singlesends_id_schedule

> crate::models::PutMarketingSinglesendsIdSchedule201Response p_ut_marketing_singlesends_id_schedule(id, put_marketing_singlesends_id_schedule_request)
Schedule Single Send

**This endpoint allows you to schedule a Single Send for future delivery using a Single Send ID.**  To schedule a Single Send, you must pass a date string in ISO 8601 time format (yyyy-MM-ddTHH:mm:ssZ)  using the required `send_at` field. For example, the ISO 8601 format for 9:00 AM UTC on May 6, 2020 would be `2020-05-06T09:00:00Z`. You may also pass the string `\"now\"` to send the Single Send immediately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |
**put_marketing_singlesends_id_schedule_request** | Option<[**PutMarketingSinglesendsIdScheduleRequest**](PutMarketingSinglesendsIdScheduleRequest.md)> |  |  |

### Return type

[**crate::models::PutMarketingSinglesendsIdSchedule201Response**](PUT_marketing_singlesends_id_schedule_201_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

