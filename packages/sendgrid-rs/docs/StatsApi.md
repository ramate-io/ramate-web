# \StatsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**g_et_browsers_stats**](StatsApi.md#g_et_browsers_stats) | **GET** /browsers/stats | Retrieve email statistics by browser.
[**g_et_clients_client_type_stats**](StatsApi.md#g_et_clients_client_type_stats) | **GET** /clients/{client_type}/stats | Retrieve stats by a specific client type.
[**g_et_clients_stats**](StatsApi.md#g_et_clients_stats) | **GET** /clients/stats | Retrieve email statistics by client type.
[**g_et_devices_stats**](StatsApi.md#g_et_devices_stats) | **GET** /devices/stats | Retrieve email statistics by device type.
[**g_et_geo_stats**](StatsApi.md#g_et_geo_stats) | **GET** /geo/stats | Retrieve email statistics by country and state/province.
[**g_et_mailbox_providers_stats**](StatsApi.md#g_et_mailbox_providers_stats) | **GET** /mailbox_providers/stats | Retrieve email statistics by mailbox provider.
[**g_et_stats**](StatsApi.md#g_et_stats) | **GET** /stats | Retrieve global email statistics



## g_et_browsers_stats

> serde_json::Value g_et_browsers_stats(start_date, browsers, on_behalf_of, limit, offset, aggregated_by, end_date)
Retrieve email statistics by browser.

**This endpoint allows you to retrieve your email statistics segmented by browser type.**  **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.  Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | [**serde_json::Value**](.md) | The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD. | [required] |
**browsers** | Option<[**serde_json::Value**](.md)> | The browsers to get statistics for. You can include up to 10 different browsers by including this parameter multiple times. |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**limit** | Option<[**serde_json::Value**](.md)> | The number of results to return. |  |
**offset** | Option<[**serde_json::Value**](.md)> | The point in the list to begin retrieving results. |  |
**aggregated_by** | Option<[**serde_json::Value**](.md)> | How to group the statistics. Must be either \"day\", \"week\", or \"month\". |  |
**end_date** | Option<[**serde_json::Value**](.md)> | The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_clients_client_type_stats

> serde_json::Value g_et_clients_client_type_stats(start_date, client_type, on_behalf_of, end_date, aggregated_by)
Retrieve stats by a specific client type.

**This endpoint allows you to retrieve your email statistics segmented by a specific client type.**  **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.  ## Available Client Types - phone - tablet - webmail - desktop  Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | [**serde_json::Value**](.md) | The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD. | [required] |
**client_type** | [**serde_json::Value**](.md) | Specifies the type of client to retrieve stats for. Must be either \"phone\", \"tablet\", \"webmail\", or \"desktop\". | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**end_date** | Option<[**serde_json::Value**](.md)> | The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD. |  |
**aggregated_by** | Option<[**serde_json::Value**](.md)> | How to group the statistics. Must be either \"day\", \"week\", or \"month\". |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_clients_stats

> serde_json::Value g_et_clients_stats(start_date, on_behalf_of, end_date, aggregated_by)
Retrieve email statistics by client type.

**This endpoint allows you to retrieve your email statistics segmented by client type.**  **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.  Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | [**serde_json::Value**](.md) | The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**end_date** | Option<[**serde_json::Value**](.md)> | The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD. |  |
**aggregated_by** | Option<[**serde_json::Value**](.md)> | How to group the statistics. Must be either \"day\", \"week\", or \"month\". |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_devices_stats

> serde_json::Value g_et_devices_stats(start_date, on_behalf_of, limit, offset, aggregated_by, end_date)
Retrieve email statistics by device type.

**This endpoint allows you to retrieve your email statistics segmented by the device type.**  **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.  ## Available Device Types | **Device** | **Description** | **Example** | |---|---|---| | Desktop | Email software on desktop computer. | I.E., Outlook, Sparrow, or Apple Mail. | | Webmail | A web-based email client. | I.E., Yahoo, Google, AOL, or Outlook.com. | | Phone | A smart phone. | iPhone, Android, Blackberry, etc. | Tablet | A tablet computer. | iPad, android based tablet, etc. | | Other | An unrecognized device. |  Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | [**serde_json::Value**](.md) | The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**limit** | Option<[**serde_json::Value**](.md)> | The number of results to return. |  |
**offset** | Option<[**serde_json::Value**](.md)> | The point in the list to begin retrieving results. |  |
**aggregated_by** | Option<[**serde_json::Value**](.md)> | How to group the statistics. Must be either \"day\", \"week\", or \"month\". |  |
**end_date** | Option<[**serde_json::Value**](.md)> | The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_geo_stats

> serde_json::Value g_et_geo_stats(start_date, country, on_behalf_of, limit, offset, aggregated_by, end_date)
Retrieve email statistics by country and state/province.

**This endpoint allows you to retrieve your email statistics segmented by country and state/province.**  **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.  Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [User Guide](https://sendgrid.com/docs/User_Guide/Statistics/index.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | [**serde_json::Value**](.md) | The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD. | [required] |
**country** | Option<[**serde_json::Value**](.md)> | The country you would like to see statistics for. Currently only supported for US and CA. |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**limit** | Option<[**serde_json::Value**](.md)> | The number of results to return. |  |
**offset** | Option<[**serde_json::Value**](.md)> | The point in the list to begin retrieving results. |  |
**aggregated_by** | Option<[**serde_json::Value**](.md)> | How to group the statistics. Must be either \"day\", \"week\", or \"month\". |  |
**end_date** | Option<[**serde_json::Value**](.md)> | The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_mailbox_providers_stats

> serde_json::Value g_et_mailbox_providers_stats(start_date, mailbox_providers, on_behalf_of, limit, offset, aggregated_by, end_date)
Retrieve email statistics by mailbox provider.

**This endpoint allows you to retrieve your email statistics segmented by recipient mailbox provider.**  **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.  Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | [**serde_json::Value**](.md) | The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD. | [required] |
**mailbox_providers** | Option<[**serde_json::Value**](.md)> | The mail box providers to get statistics for. You can include up to 10 by including this parameter multiple times. |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**limit** | Option<[**serde_json::Value**](.md)> | The number of results to return. |  |
**offset** | Option<[**serde_json::Value**](.md)> | The point in the list to begin retrieving results. |  |
**aggregated_by** | Option<[**serde_json::Value**](.md)> | How to group the statistics. Must be either \"day\", \"week\", or \"month\". |  |
**end_date** | Option<[**serde_json::Value**](.md)> | The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_stats

> serde_json::Value g_et_stats(start_date, on_behalf_of, limit, offset, aggregated_by, end_date)
Retrieve global email statistics

**This endpoint allows you to retrieve all of your global email statistics between a given date range.**  Parent accounts will see aggregated stats for their account and all subuser accounts. Subuser accounts will only see their own stats.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | [**serde_json::Value**](.md) | The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**limit** | Option<[**serde_json::Value**](.md)> | The number of results to return. |  |
**offset** | Option<[**serde_json::Value**](.md)> | The point in the list to begin retrieving results. |  |
**aggregated_by** | Option<[**serde_json::Value**](.md)> | How to group the statistics. Must be either \"day\", \"week\", or \"month\". |  |
**end_date** | Option<[**serde_json::Value**](.md)> | The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

