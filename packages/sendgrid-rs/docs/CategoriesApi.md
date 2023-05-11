# \CategoriesApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**g_et_categories**](CategoriesApi.md#g_et_categories) | **GET** /categories | Retrieve all categories
[**g_et_categories_stats**](CategoriesApi.md#g_et_categories_stats) | **GET** /categories/stats | Retrieve Email Statistics for Categories
[**g_et_categories_stats_sums**](CategoriesApi.md#g_et_categories_stats_sums) | **GET** /categories/stats/sums | Retrieve sums of email stats for each category [Needs: Stats object defined, has category ID?]



## g_et_categories

> serde_json::Value g_et_categories(limit, category, offset, on_behalf_of)
Retrieve all categories

**This endpoint allows you to retrieve a list of all of your categories.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<[**serde_json::Value**](.md)> | The number of categories to display per page. |  |[default to 50]
**category** | Option<[**serde_json::Value**](.md)> | Allows you to perform a prefix search on this particular category. |  |
**offset** | Option<[**serde_json::Value**](.md)> | The point in the list that you would like to begin displaying results. |  |[default to 0]
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_categories_stats

> serde_json::Value g_et_categories_stats(start_date, categories, end_date, limit, offset, aggregated_by, on_behalf_of)
Retrieve Email Statistics for Categories

**This endpoint allows you to retrieve all of your email statistics for each of your categories.**  If you do not define any query parameters, this endpoint will return a sum for each category in groups of 10.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | [**serde_json::Value**](.md) | The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD | [required] |
**categories** | [**serde_json::Value**](.md) | The individual categories that you want to retrieve statistics for. You may include up to 10 different categories. | [required] |
**end_date** | Option<[**serde_json::Value**](.md)> | The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD. |  |
**limit** | Option<[**serde_json::Value**](.md)> | The number of results to include. |  |[default to 500]
**offset** | Option<[**serde_json::Value**](.md)> | The number of results to skip. |  |
**aggregated_by** | Option<[**serde_json::Value**](.md)> | How to group the statistics. Must be either \"day\", \"week\", or \"month\". |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_categories_stats_sums

> crate::models::CategoryStats g_et_categories_stats_sums(start_date, sort_by_metric, sort_by_direction, end_date, limit, offset, aggregated_by, on_behalf_of)
Retrieve sums of email stats for each category [Needs: Stats object defined, has category ID?]

**This endpoint allows you to retrieve the total sum of each email statistic for every category over the given date range.**  If you do not define any query parameters, this endpoint will return a sum for each category in groups of 10.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | [**serde_json::Value**](.md) | The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD. | [required] |
**sort_by_metric** | Option<[**serde_json::Value**](.md)> | The metric that you want to sort by.  Must be a single metric. |  |[default to delivered]
**sort_by_direction** | Option<[**serde_json::Value**](.md)> | The direction you want to sort. |  |[default to desc]
**end_date** | Option<[**serde_json::Value**](.md)> | The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Limits the number of results returned. |  |[default to 5]
**offset** | Option<[**serde_json::Value**](.md)> | The point in the list to begin retrieving results. |  |[default to 0]
**aggregated_by** | Option<[**serde_json::Value**](.md)> | How to group the statistics. Must be either \"day\", \"week\", or \"month\". |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::CategoryStats**](category_stats.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

