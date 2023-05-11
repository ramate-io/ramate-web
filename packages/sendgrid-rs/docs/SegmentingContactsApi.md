# \SegmentingContactsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_marketing_segments_segment_id**](SegmentingContactsApi.md#d_elete_marketing_segments_segment_id) | **DELETE** /marketing/segments/{segment_id} | Delete Segment
[**d_elete_marketing_segments_segment_id_0**](SegmentingContactsApi.md#d_elete_marketing_segments_segment_id_0) | **DELETE** /marketing/segments/{segment_id} | Delete Segment
[**g_et_marketing_segments**](SegmentingContactsApi.md#g_et_marketing_segments) | **GET** /marketing/segments | Get List of Segments
[**g_et_marketing_segments_0**](SegmentingContactsApi.md#g_et_marketing_segments_0) | **GET** /marketing/segments | Get List of Segments
[**g_et_marketing_segments_segment_id**](SegmentingContactsApi.md#g_et_marketing_segments_segment_id) | **GET** /marketing/segments/{segment_id} | Get Segment by ID
[**g_et_marketing_segments_segment_id_0**](SegmentingContactsApi.md#g_et_marketing_segments_segment_id_0) | **GET** /marketing/segments/{segment_id} | Get Segment by ID
[**p_atch_marketing_segments_segment_id**](SegmentingContactsApi.md#p_atch_marketing_segments_segment_id) | **PATCH** /marketing/segments/{segment_id} | Update Segment
[**p_atch_marketing_segments_segment_id_0**](SegmentingContactsApi.md#p_atch_marketing_segments_segment_id_0) | **PATCH** /marketing/segments/{segment_id} | Update Segment
[**p_ost_marketing_segments**](SegmentingContactsApi.md#p_ost_marketing_segments) | **POST** /marketing/segments | Create Segment
[**p_ost_marketing_segments_0**](SegmentingContactsApi.md#p_ost_marketing_segments_0) | **POST** /marketing/segments | Create Segment
[**p_ost_marketing_segments_delete**](SegmentingContactsApi.md#p_ost_marketing_segments_delete) | **POST** /marketing/segments/delete | Bulk Delete Segments



## d_elete_marketing_segments_segment_id

> serde_json::Value d_elete_marketing_segments_segment_id(segment_id)
Delete Segment

**This endpoint allows you to delete a segment by `segment_id`.**  Note that deleting a segment does not delete the contacts associated with the segment by default. Contacts associated with a deleted segment will remain in your list of all contacts and any other segments they belong to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_elete_marketing_segments_segment_id_0

> serde_json::Value d_elete_marketing_segments_segment_id_0(segment_id)
Delete Segment

**This endpoint allows you to delete a segment by `segment_id`.**  Note that deleting a segment does not delete the contacts associated with the segment by default. Contacts associated with a deleted segment will remain in your list of all contacts and any other segments they belong to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_marketing_segments

> crate::models::GetMarketingSegments200Response g_et_marketing_segments(parent_list_ids, no_parent_list_id)
Get List of Segments

**This endpoint allows you to retrieve a list of segments.**  The query param `parent_list_ids` is treated as a filter.  Any match will be returned.  0 matches will return a response code of 200 with an empty `results` array.  `parent_list_ids` | `no_parent_list_id` | `result` -----------------:|:--------------------:|:------------- empty | false | all segments values | false | segments filtered by list_ids values | true | segments filtered by list_ids and segments with no parent list_ids empty | true | segments with no parent list_ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_list_ids** | Option<[**serde_json::Value**](.md)> | A comma separated list of list ids to be used when searching for segments with the specified parent_list_id, no more than 50 is allowed |  |
**no_parent_list_id** | Option<[**serde_json::Value**](.md)> | If set to `true` segments with an empty value of `parent_list_id` will be returned in the filter.  If the value is not present it defaults to 'false'. |  |[default to false]

### Return type

[**crate::models::GetMarketingSegments200Response**](GET_marketing_segments_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_marketing_segments_0

> crate::models::GetMarketingSegments200Response g_et_marketing_segments_0(parent_list_ids, no_parent_list_id)
Get List of Segments

**This endpoint allows you to retrieve a list of segments.**  The query param `parent_list_ids` is treated as a filter.  Any match will be returned.  0 matches will return a response code of 200 with an empty `results` array.  `parent_list_ids` | `no_parent_list_id` | `result` -----------------:|:--------------------:|:------------- empty | false | all segments values | false | segments filtered by list_ids values | true | segments filtered by list_ids and segments with no parent list_ids empty | true | segments with no parent list_ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_list_ids** | Option<[**serde_json::Value**](.md)> | A comma separated list of list ids to be used when searching for segments with the specified parent_list_id, no more than 50 is allowed |  |
**no_parent_list_id** | Option<[**serde_json::Value**](.md)> | If set to `true` segments with an empty value of `parent_list_id` will be returned in the filter.  If the value is not present it defaults to 'false'. |  |[default to false]

### Return type

[**crate::models::GetMarketingSegments200Response**](GET_marketing_segments_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_marketing_segments_segment_id

> serde_json::Value g_et_marketing_segments_segment_id(segment_id, query_json)
Get Segment by ID

**This endpoint allows you to retrieve a single segment by ID.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | [**serde_json::Value**](.md) |  | [required] |
**query_json** | Option<[**serde_json::Value**](.md)> | Defaults to `false`.  Set to `true` to return the parsed SQL AST as a JSON object in the field `query_json` |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_marketing_segments_segment_id_0

> serde_json::Value g_et_marketing_segments_segment_id_0(segment_id, query_json)
Get Segment by ID

**This endpoint allows you to retrieve a single segment by ID.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | [**serde_json::Value**](.md) |  | [required] |
**query_json** | Option<[**serde_json::Value**](.md)> | Defaults to `false`.  Set to `true` to return the parsed SQL AST as a JSON object in the field `query_json` |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_marketing_segments_segment_id

> serde_json::Value p_atch_marketing_segments_segment_id(segment_id, segment_write_v2)
Update Segment

**This endpoint allows you to update a segment.**  Segment `name` needs to be unique. A user can not update a segment name to an existing one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | [**serde_json::Value**](.md) |  | [required] |
**segment_write_v2** | Option<[**SegmentWriteV2**](SegmentWriteV2.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_marketing_segments_segment_id_0

> serde_json::Value p_atch_marketing_segments_segment_id_0(segment_id, segment_write_v2)
Update Segment

**This endpoint allows you to update a segment.**  Segment `name` needs to be unique. A user can not update a segment name to an existing one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | [**serde_json::Value**](.md) |  | [required] |
**segment_write_v2** | Option<[**SegmentWriteV2**](SegmentWriteV2.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_marketing_segments

> serde_json::Value p_ost_marketing_segments(body)
Create Segment

**This endpoint allows you to create a segment.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_marketing_segments_0

> serde_json::Value p_ost_marketing_segments_0(body)
Create Segment

**This endpoint allows you to create a segment.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_marketing_segments_delete

> crate::models::PostMarketingSegmentsDelete200Response p_ost_marketing_segments_delete(post_marketing_segments_delete_request)
Bulk Delete Segments

This endpoint allows you to delete segments in bulk.  If the segments are used by automations or the segments do not exist in the database, the segment IDs that could not be deleted along with automation IDs that are associated to those segments will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_marketing_segments_delete_request** | Option<[**PostMarketingSegmentsDeleteRequest**](PostMarketingSegmentsDeleteRequest.md)> |  |  |

### Return type

[**crate::models::PostMarketingSegmentsDelete200Response**](POST_marketing_segments_delete_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

