# \SubuserMonitorSettingsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_subusers_subuser_name_monitor**](SubuserMonitorSettingsApi.md#d_elete_subusers_subuser_name_monitor) | **DELETE** /subusers/{subuser_name}/monitor | Delete monitor settings
[**g_et_subusers_subuser_name_monitor**](SubuserMonitorSettingsApi.md#g_et_subusers_subuser_name_monitor) | **GET** /subusers/{subuser_name}/monitor | Retrieve monitor settings for a subuser
[**p_ost_subusers_subuser_name_monitor**](SubuserMonitorSettingsApi.md#p_ost_subusers_subuser_name_monitor) | **POST** /subusers/{subuser_name}/monitor | Create monitor settings
[**p_ut_subusers_subuser_name_monitor**](SubuserMonitorSettingsApi.md#p_ut_subusers_subuser_name_monitor) | **PUT** /subusers/{subuser_name}/monitor | Update Monitor Settings for a subuser



## d_elete_subusers_subuser_name_monitor

> serde_json::Value d_elete_subusers_subuser_name_monitor(subuser_name)
Delete monitor settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subuser_name** | [**serde_json::Value**](.md) | The name of the subuser for which to retrieve monitor settings. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_subusers_subuser_name_monitor

> crate::models::Monitor g_et_subusers_subuser_name_monitor(subuser_name)
Retrieve monitor settings for a subuser

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subuser_name** | [**serde_json::Value**](.md) | The name of the subuser for which to retrieve monitor settings. | [required] |

### Return type

[**crate::models::Monitor**](monitor.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_subusers_subuser_name_monitor

> crate::models::Monitor p_ost_subusers_subuser_name_monitor(subuser_name, monitor)
Create monitor settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subuser_name** | [**serde_json::Value**](.md) | The name of the subuser for which to retrieve monitor settings. | [required] |
**monitor** | Option<[**Monitor**](Monitor.md)> |  |  |

### Return type

[**crate::models::Monitor**](monitor.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ut_subusers_subuser_name_monitor

> crate::models::Monitor p_ut_subusers_subuser_name_monitor(subuser_name, monitor)
Update Monitor Settings for a subuser

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subuser_name** | [**serde_json::Value**](.md) | The name of the subuser for which to retrieve monitor settings. | [required] |
**monitor** | Option<[**Monitor**](Monitor.md)> |  |  |

### Return type

[**crate::models::Monitor**](monitor.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

