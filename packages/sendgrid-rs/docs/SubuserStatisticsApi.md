# \SubuserStatisticsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**g_et_subusers_stats**](SubuserStatisticsApi.md#g_et_subusers_stats) | **GET** /subusers/stats | Retrieve email statistics for your subusers.
[**g_et_subusers_stats_monthly**](SubuserStatisticsApi.md#g_et_subusers_stats_monthly) | **GET** /subusers/stats/monthly | Retrieve monthly stats for all subusers
[**g_et_subusers_stats_sums**](SubuserStatisticsApi.md#g_et_subusers_stats_sums) | **GET** /subusers/stats/sums | Retrieve the totals for each email statistic metric for all subusers.
[**g_et_subusers_subuser_name_stats_monthly**](SubuserStatisticsApi.md#g_et_subusers_subuser_name_stats_monthly) | **GET** /subusers/{subuser_name}/stats/monthly | Retrieve the monthly email statistics for a single subuser



## g_et_subusers_stats

> serde_json::Value g_et_subusers_stats(subusers, start_date, limit, offset, aggregated_by, end_date)
Retrieve email statistics for your subusers.

**This endpoint allows you to retrieve the email statistics for the given subusers.**  You may retrieve statistics for up to 10 different subusers by including an additional _subusers_ parameter for each additional subuser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subusers** | [**serde_json::Value**](.md) | The subuser you want to retrieve statistics for. You may include this parameter up to 10 times to retrieve statistics for multiple subusers. | [required] |
**start_date** | [**serde_json::Value**](.md) | The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD. | [required] |
**limit** | Option<[**serde_json::Value**](.md)> | Limits the number of results returned per page. |  |
**offset** | Option<[**serde_json::Value**](.md)> | The point in the list to begin retrieving results from. |  |
**aggregated_by** | Option<[**serde_json::Value**](.md)> | How to group the statistics. Must be either \"day\", \"week\", or \"month\". |  |
**end_date** | Option<[**serde_json::Value**](.md)> | The end date of the statistics to retrieve. Defaults to today. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_subusers_stats_monthly

> crate::models::SubuserStats g_et_subusers_stats_monthly(date, subuser, sort_by_metric, sort_by_direction, limit, offset)
Retrieve monthly stats for all subusers

**This endpoint allows you to retrieve the monthly email statistics for all subusers over the given date range.**  When using the `sort_by_metric` to sort your stats by a specific metric, you can not sort by the following metrics: `bounce_drops`, `deferred`, `invalid_emails`, `processed`, `spam_report_drops`, `spam_reports`, or `unsubscribe_drops`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | [**serde_json::Value**](.md) | The date of the month to retrieve statistics for. Must be formatted YYYY-MM-DD | [required] |
**subuser** | Option<[**serde_json::Value**](.md)> | A substring search of your subusers. |  |
**sort_by_metric** | Option<[**serde_json::Value**](.md)> | The metric that you want to sort by. Metrics that you can sort by are: `blocks`, `bounces`, `clicks`, `delivered`, `opens`, `requests`, `unique_clicks`, `unique_opens`, and `unsubscribes`.' |  |[default to delivered]
**sort_by_direction** | Option<[**serde_json::Value**](.md)> | The direction you want to sort. |  |[default to desc]
**limit** | Option<[**serde_json::Value**](.md)> | Optional field to limit the number of results returned. |  |[default to 5]
**offset** | Option<[**serde_json::Value**](.md)> | Optional beginning point in the list to retrieve from. |  |[default to 0]

### Return type

[**crate::models::SubuserStats**](subuser_stats.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_subusers_stats_sums

> crate::models::CategoryStats g_et_subusers_stats_sums(start_date, sort_by_direction, end_date, limit, offset, aggregated_by, sort_by_metric)
Retrieve the totals for each email statistic metric for all subusers.

**This endpoint allows you to retrieve the total sums of each email statistic metric for all subusers over the given date range.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | [**serde_json::Value**](.md) | The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD. | [required] |
**sort_by_direction** | Option<[**serde_json::Value**](.md)> | The direction you want to sort.  |  |[default to desc]
**end_date** | Option<[**serde_json::Value**](.md)> | The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Limits the number of results returned per page. |  |[default to 5]
**offset** | Option<[**serde_json::Value**](.md)> | The point in the list to begin retrieving results from. |  |[default to 0]
**aggregated_by** | Option<[**serde_json::Value**](.md)> | How to group the statistics. Defaults to today. Must follow format YYYY-MM-DD. |  |
**sort_by_metric** | Option<[**serde_json::Value**](.md)> | The metric that you want to sort by.  Must be a single metric. |  |[default to delivered]

### Return type

[**crate::models::CategoryStats**](category_stats.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_subusers_subuser_name_stats_monthly

> crate::models::SubuserStats g_et_subusers_subuser_name_stats_monthly(date, subuser_name, sort_by_metric, sort_by_direction, limit, offset)
Retrieve the monthly email statistics for a single subuser

**This endpoint allows you to retrive the monthly email statistics for a specific subuser.**  When using the `sort_by_metric` to sort your stats by a specific metric, you can not sort by the following metrics: `bounce_drops`, `deferred`, `invalid_emails`, `processed`, `spam_report_drops`, `spam_reports`, or `unsubscribe_drops`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | [**serde_json::Value**](.md) | The date of the month to retrieve statistics for. Must be formatted YYYY-MM-DD | [required] |
**subuser_name** | [**serde_json::Value**](.md) |  | [required] |
**sort_by_metric** | Option<[**serde_json::Value**](.md)> | The metric that you want to sort by. Metrics that you can sort by are: `blocks`, `bounces`, `clicks`, `delivered`, `opens`, `requests`, `unique_clicks`, `unique_opens`, and `unsubscribes`.' |  |[default to delivered]
**sort_by_direction** | Option<[**serde_json::Value**](.md)> | The direction you want to sort. |  |[default to desc]
**limit** | Option<[**serde_json::Value**](.md)> | Optional field to limit the number of results returned. |  |[default to 5]
**offset** | Option<[**serde_json::Value**](.md)> | Optional beginning point in the list to retrieve from. |  |[default to 0]

### Return type

[**crate::models::SubuserStats**](subuser_stats.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

