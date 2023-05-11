# \SegmentingContactsV2Api

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**g_et_segments**](SegmentingContactsV2Api.md#g_et_segments) | **GET** /marketing/segments/2.0 | Get List of Segments
[**g_et_segments_segment_id**](SegmentingContactsV2Api.md#g_et_segments_segment_id) | **GET** /marketing/segments/2.0/{segment_id} | Get Segment by ID
[**p_atch_segments_segment_id**](SegmentingContactsV2Api.md#p_atch_segments_segment_id) | **PATCH** /marketing/segments/2.0/{segment_id} | Update Segment
[**p_ost_segments**](SegmentingContactsV2Api.md#p_ost_segments) | **POST** /marketing/segments/2.0 | Create Segment



## g_et_segments

> crate::models::AllSegmentsResponse g_et_segments(parent_list_ids, no_parent_list_id)
Get List of Segments

The query param `parent_list_ids` is treated as a filter.  Any match will be returned.  0 matches will return a response code of 200 with an empty `results` array.  `parent_list_ids` | `no_parent_list_id` | `result` -----------------:|:--------------------:|:------------- empty | false | all segments values | false | segments filtered by list_ids values | true | segments filtered by list_ids and segments with no parent list_ids empty | true | segments with no parent list_ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_list_ids** | Option<[**serde_json::Value**](.md)> | A comma separated list up to 50 in size, to filter segments on.  Only segments that have any of these list ids as the parent list will be retrieved. This is different from the parameter of the same name used when creating a segment. |  |
**no_parent_list_id** | Option<[**serde_json::Value**](.md)> | If set to `true` segments with an empty value of `parent_list_id` will be returned in the filter.  If the value is not present it defaults to 'false'. |  |[default to false]

### Return type

[**crate::models::AllSegmentsResponse**](all_segments_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_segments_segment_id

> crate::models::SegmentResponse g_et_segments_segment_id(segment_id, contacts_sample)
Get Segment by ID



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | [**serde_json::Value**](.md) |  | [required] |
**contacts_sample** | Option<[**serde_json::Value**](.md)> | Defaults to `true`. Set to `false` to exclude the contacts_sample in the response. |  |

### Return type

[**crate::models::SegmentResponse**](segment_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_segments_segment_id

> crate::models::SegmentResponse p_atch_segments_segment_id(segment_id, segment_update)
Update Segment

Segment `name` has to be unique. A user can not create a new segment with an existing segment name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | [**serde_json::Value**](.md) |  | [required] |
**segment_update** | Option<[**SegmentUpdate**](SegmentUpdate.md)> |  |  |

### Return type

[**crate::models::SegmentResponse**](segment_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_segments

> crate::models::SegmentResponse p_ost_segments(segment_write_v2)
Create Segment

Segment `name` has to be unique. A user can not create a new segment with an existing segment name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_write_v2** | Option<[**SegmentWriteV2**](SegmentWriteV2.md)> |  |  |

### Return type

[**crate::models::SegmentResponse**](segment_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

