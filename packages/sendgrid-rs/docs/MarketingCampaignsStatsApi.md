# \MarketingCampaignsStatsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_automation_link_stat**](MarketingCampaignsStatsApi.md#get_automation_link_stat) | **GET** /marketing/stats/automations/{id}/links | Get Automation Click Tracking Stats by ID
[**get_automation_stat**](MarketingCampaignsStatsApi.md#get_automation_stat) | **GET** /marketing/stats/automations/{id} | Get Automation Stats by ID
[**get_automations_stats_export**](MarketingCampaignsStatsApi.md#get_automations_stats_export) | **GET** /marketing/stats/automations/export | Export Automation Stats
[**get_singlesend_link_stat**](MarketingCampaignsStatsApi.md#get_singlesend_link_stat) | **GET** /marketing/stats/singlesends/{id}/links | Get Single Send Click Tracking Stats by ID
[**get_singlesend_stat**](MarketingCampaignsStatsApi.md#get_singlesend_stat) | **GET** /marketing/stats/singlesends/{id} | Get Single Send Stats by ID
[**get_singlesend_stats_export**](MarketingCampaignsStatsApi.md#get_singlesend_stats_export) | **GET** /marketing/stats/singlesends/export | Export Single Send Stats
[**getall_automation_stats**](MarketingCampaignsStatsApi.md#getall_automation_stats) | **GET** /marketing/stats/automations | Get All Automation Stats
[**getall_singlesend_stats**](MarketingCampaignsStatsApi.md#getall_singlesend_stats) | **GET** /marketing/stats/singlesends | Get All Single Sends Stats



## get_automation_link_stat

> crate::models::AutomationsLinkStatsResponse get_automation_link_stat(id, group_by, step_ids, page_size, page_token)
Get Automation Click Tracking Stats by ID

**This endpoint lets you retrieve click-tracking stats for a single Automation**.  The stats returned list the URLs embedded in your Automation and the number of clicks each one received.  Responses are paginated. You can limit the number of responses returned per batch using the `page_size` query string parameter. The default is 50, but you specify a value between 1 and 100.  You can retrieve a specific page of responses with the `page_token` query string parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The ID of the Automation you want to get click tracking stats for.  | [required] |
**group_by** | Option<[**serde_json::Value**](.md)> | Automations can have multiple steps. Including `step_id` as a `group_by` metric allows further granularity of stats. |  |
**step_ids** | Option<[**serde_json::Value**](.md)> | Comma-separated list of `step_ids` that you want the link stats for. |  |
**page_size** | Option<[**serde_json::Value**](.md)> | The number of elements you want returned on each page. |  |[default to 50]
**page_token** | Option<[**serde_json::Value**](.md)> | The stats endpoints are paginated. To get the next page, call the passed `_metadata.next` URL. If `_metadata.prev` doesn't exist, you're at the first page. Similarly, if `_metadata.next` is not present, you're at the last page. |  |

### Return type

[**crate::models::AutomationsLinkStatsResponse**](automations-link-stats-response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_automation_stat

> crate::models::AutomationsResponse get_automation_stat(id, group_by, step_ids, aggregated_by, start_date, end_date, timezone, page_size, page_token)
Get Automation Stats by ID

**This endpoint allows you to retrieve stats for a single Automation using its ID.**  Multiple Automation IDs can be retrieved using the \"Get All Automation Stats\" endpoint. Once you have an ID, this endpoint will return detailed stats for the single automation specified.  You may constrain the stats returned using the `start_date` and `end_date` query string parameters. You can also use the `group_by` and `aggregated_by` query string parameters to further refine the stats returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |
**group_by** | Option<[**serde_json::Value**](.md)> | Automations can have multiple steps. Including `step_id` as a `group_by` metric allows further granularity of stats. |  |
**step_ids** | Option<[**serde_json::Value**](.md)> | Comma-separated list of `step_ids` that you want the link stats for. |  |
**aggregated_by** | Option<[**serde_json::Value**](.md)> | Dictates how the stats are time-sliced. Currently, `\"total\"` and `\"day\"` are supported. |  |[default to total]
**start_date** | Option<[**serde_json::Value**](.md)> | Format: `YYYY-MM-DD`. If this parameter is included, the stats' start date is included in the search. |  |[default to 2021-01-01]
**end_date** | Option<[**serde_json::Value**](.md)> | Format: `YYYY-MM-DD`.If this parameter is included, the stats' end date is included in the search. |  |[default to 2021-12-31]
**timezone** | Option<[**serde_json::Value**](.md)> | [IANA Area/Region](https://en.wikipedia.org/wiki/Tz_database#Names_of_time_zones) string representing the timezone in which the stats are to be presented, e.g., \"America/Chicago\". |  |[default to UTC]
**page_size** | Option<[**serde_json::Value**](.md)> | The number of elements you want returned on each page. |  |[default to 50]
**page_token** | Option<[**serde_json::Value**](.md)> | The stats endpoints are paginated. To get the next page, call the passed `_metadata.next` URL. If `_metadata.prev` doesn't exist, you're at the first page. Similarly, if `_metadata.next` is not present, you're at the last page. |  |

### Return type

[**crate::models::AutomationsResponse**](automations-response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_automations_stats_export

> serde_json::Value get_automations_stats_export(ids, timezone)
Export Automation Stats

**This endpoint allows you to export Automation stats as CSV data**.  You can specify one Automation or many: include as many Automation IDs as you need, separating them with commas, as the value of the `ids` query string paramter.  The data is returned as plain text response but in CSV format, so your application making the call can present the information in whatever way is most appropriate, or just save the data as a `.csv` file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**serde_json::Value**](.md)> | The IDs of Automations for which to export stats. |  |
**timezone** | Option<[**serde_json::Value**](.md)> | The [IANA Area/Region](https://en.wikipedia.org/wiki/Tz_database#Names_of_time_zones) string representing the timezone in which the stats are to be presented; i.e. `\"America/Chicago\"`. This parameter changes the timezone format only; it does not alter which stats are returned. |  |[default to UTC]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_singlesend_link_stat

> crate::models::SinglesendsLinkStatsResponse get_singlesend_link_stat(id, page_size, page_token, group_by, ab_variation_id, ab_phase_id)
Get Single Send Click Tracking Stats by ID

**This endpoint lets you retrieve click-tracking stats for one Single Send**.  The stats returned list the URLs embedded in the specified Single Send and the number of clicks each one received.  Responses are paginated. You can limit the number of responses returned per batch using the `page_size` query string parameter. The default is 50, but you specify a value between 1 and 100.  You can retrieve a specific page of responses with the `page_token` query string parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |
**page_size** | Option<[**serde_json::Value**](.md)> | The number of elements you want returned on each page. |  |[default to 50]
**page_token** | Option<[**serde_json::Value**](.md)> | The stats endpoints are paginated. To get the next page, call the passed `_metadata.next` URL. If `_metadata.prev` doesn't exist, you're at the first page. Similarly, if `_metadata.next` is not present, you're at the last page. |  |
**group_by** | Option<[**serde_json::Value**](.md)> | A/B Single Sends have multiple variation IDs and phase IDs. Including these additional fields allows further granularity of stats by these fields. |  |
**ab_variation_id** | Option<[**serde_json::Value**](.md)> |  |  |
**ab_phase_id** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

[**crate::models::SinglesendsLinkStatsResponse**](singlesends-link-stats-response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_singlesend_stat

> crate::models::SinglesendsResponse get_singlesend_stat(id, aggregated_by, start_date, end_date, timezone, page_size, page_token, group_by)
Get Single Send Stats by ID

**This endpoint allows you to retrieve stats for an individual Single Send using a Single Send ID.**  Multiple Single Send IDs can be retrieved using the \"Get All Single Sends Stats\" endpoint. Once you have an ID, this endpoint will return detailed stats for the Single Send specified.  You may constrain the stats returned using the `start_date` and `end_date` query string parameters. You can also use the `group_by` and `aggregated_by` query string parameters to further refine the stats returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |
**aggregated_by** | Option<[**serde_json::Value**](.md)> | Dictates how the stats are time-sliced. Currently, `\"total\"` and `\"day\"` are supported. |  |[default to total]
**start_date** | Option<[**serde_json::Value**](.md)> | Format: `YYYY-MM-DD`. If this parameter is included, the stats' start date is included in the search. |  |[default to 2021-01-01]
**end_date** | Option<[**serde_json::Value**](.md)> | Format: `YYYY-MM-DD`.If this parameter is included, the stats' end date is included in the search. |  |[default to 2021-12-31]
**timezone** | Option<[**serde_json::Value**](.md)> | [IANA Area/Region](https://en.wikipedia.org/wiki/Tz_database#Names_of_time_zones) string representing the timezone in which the stats are to be presented, e.g., \"America/Chicago\". |  |[default to UTC]
**page_size** | Option<[**serde_json::Value**](.md)> | The number of elements you want returned on each page. |  |[default to 50]
**page_token** | Option<[**serde_json::Value**](.md)> | The stats endpoints are paginated. To get the next page, call the passed `_metadata.next` URL. If `_metadata.prev` doesn't exist, you're at the first page. Similarly, if `_metadata.next` is not present, you're at the last page. |  |
**group_by** | Option<[**serde_json::Value**](.md)> | A/B Single Sends have multiple variation IDs and phase IDs. Including these additional fields allows further granularity of stats by these fields. |  |

### Return type

[**crate::models::SinglesendsResponse**](singlesends-response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_singlesend_stats_export

> serde_json::Value get_singlesend_stats_export(ids, timezone)
Export Single Send Stats

**This endpoint allows you to export Single Send stats as .CSV data**.  You can specify one Single Send or many: include as many Single Send IDs as you need, separating them with commas, as the value of the `ids` query string paramter.  The data is returned as plain text response but in .CSV format, so your application making the call can present the information in whatever way is most appropriate, or just save the data as a .csv file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**serde_json::Value**](.md)> | The IDs of Single Sends for which to export stats. |  |
**timezone** | Option<[**serde_json::Value**](.md)> | The [IANA Area/Region](https://en.wikipedia.org/wiki/Tz_database#Names_of_time_zones) string representing the timezone in which the stats are to be presented; i.e. `\"America/Chicago\"`. This parameter changes the timezone format only; it does not alter which stats are returned. |  |[default to UTC]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## getall_automation_stats

> crate::models::AutomationsResponse getall_automation_stats(automation_ids, page_size, page_token)
Get All Automation Stats

**This endpoint allows you to retrieve stats for all your Automations.**  By default, all of your Automations will be returned, but you can specify a selection by passing in a comma-separated list of Automation IDs as the value of the query string parameter `automation_ids`.  Responses are paginated. You can limit the number of responses returned per batch using the `page_size` query string parameter. The default is 50, but you specify a value between 1 and 100.  You can retrieve a specific page of responses with the `page_token` query string parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**automation_ids** | Option<[**serde_json::Value**](.md)> | This endpoint returns all automation IDs if no `automation_ids` are specified. |  |
**page_size** | Option<[**serde_json::Value**](.md)> | The number of elements you want returned on each page. |  |[default to 50]
**page_token** | Option<[**serde_json::Value**](.md)> | The stats endpoints are paginated. To get the next page, call the passed `_metadata.next` URL. If `_metadata.prev` doesn't exist, you're at the first page. Similarly, if `_metadata.next` is not present, you're at the last page. |  |

### Return type

[**crate::models::AutomationsResponse**](automations-response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## getall_singlesend_stats

> crate::models::SinglesendsResponse getall_singlesend_stats(singlesend_ids, page_size, page_token)
Get All Single Sends Stats

**This endpoint allows you to retrieve stats for all your Single Sends.**  By default, all of your Single Sends will be returned, but you can specify a selection by passing in a comma-separated list of Single Send IDs as the value of the query string parameter `singlesend_ids`.  Responses are paginated. You can limit the number of responses returned per batch using the `page_size` query string parameter. The default is 50, but you specify a value between 1 and 100.  You can retrieve a specific page of responses with the `page_token` query string parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**singlesend_ids** | Option<[**serde_json::Value**](.md)> | This endpoint returns all Single Send IDs if no IDs are included in `singlesend_ids`. |  |
**page_size** | Option<[**serde_json::Value**](.md)> | The number of elements you want returned on each page. |  |[default to 50]
**page_token** | Option<[**serde_json::Value**](.md)> | The stats endpoints are paginated. To get the next page, call the passed `_metadata.next` URL. If `_metadata.prev` doesn't exist, you're at the first page. Similarly, if `_metadata.next` is not present, you're at the last page. |  |

### Return type

[**crate::models::SinglesendsResponse**](singlesends-response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

