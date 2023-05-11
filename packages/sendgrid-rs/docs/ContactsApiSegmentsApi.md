# \ContactsApiSegmentsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_contactdb_segments_segment_id**](ContactsApiSegmentsApi.md#d_elete_contactdb_segments_segment_id) | **DELETE** /contactdb/segments/{segment_id} | Delete a segment
[**g_et_contactdb_segments**](ContactsApiSegmentsApi.md#g_et_contactdb_segments) | **GET** /contactdb/segments | Retrieve all segments
[**g_et_contactdb_segments_segment_id**](ContactsApiSegmentsApi.md#g_et_contactdb_segments_segment_id) | **GET** /contactdb/segments/{segment_id} | Retrieve a segment
[**g_et_contactdb_segments_segment_id_recipients**](ContactsApiSegmentsApi.md#g_et_contactdb_segments_segment_id_recipients) | **GET** /contactdb/segments/{segment_id}/recipients | Retrieve recipients on a segment
[**p_atch_contactdb_segments_segment_id**](ContactsApiSegmentsApi.md#p_atch_contactdb_segments_segment_id) | **PATCH** /contactdb/segments/{segment_id} | Update a segment
[**p_ost_contactdb_segments**](ContactsApiSegmentsApi.md#p_ost_contactdb_segments) | **POST** /contactdb/segments | Create a Segment



## d_elete_contactdb_segments_segment_id

> serde_json::Value d_elete_contactdb_segments_segment_id(segment_id, delete_contacts, on_behalf_of, body)
Delete a segment

**This endpoint allows you to delete a segment from your recipients database.**  You also have the option to delete all the contacts from your Marketing Campaigns recipient database who were in this segment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | [**serde_json::Value**](.md) |  | [required] |
**delete_contacts** | Option<[**serde_json::Value**](.md)> | True to delete all contacts matching the segment in addition to deleting the segment |  |
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


## g_et_contactdb_segments

> crate::models::ListAllSegmentsResponse g_et_contactdb_segments(on_behalf_of)
Retrieve all segments

**This endpoint allows you to retrieve all of your segments.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::ListAllSegmentsResponse**](List_All_Segments_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_contactdb_segments_segment_id

> crate::models::ContactdbSegments g_et_contactdb_segments_segment_id(segment_id, segment_id2, on_behalf_of)
Retrieve a segment

**This endpoint allows you to retrieve a single segment with the given ID.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | [**serde_json::Value**](.md) | The ID of the segment you want to request. | [required] |
**segment_id2** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::ContactdbSegments**](contactdb_segments.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_contactdb_segments_segment_id_recipients

> crate::models::ListRecipientsOnASegmentResponse g_et_contactdb_segments_segment_id_recipients(segment_id, page, page_size, on_behalf_of)
Retrieve recipients on a segment

**This endpoint allows you to retrieve all of the recipients in a segment with the given ID.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | [**serde_json::Value**](.md) | The ID of the segment from which you want to retrieve recipients. | [required] |
**page** | Option<[**serde_json::Value**](.md)> |  |  |
**page_size** | Option<[**serde_json::Value**](.md)> |  |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::ListRecipientsOnASegmentResponse**](List_Recipients_On_a_Segment_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_contactdb_segments_segment_id

> crate::models::ContactdbSegments p_atch_contactdb_segments_segment_id(segment_id2, segment_id, on_behalf_of, patch_contactdb_segments_segment_id_request)
Update a segment

**This endpoint allows you to update a segment.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id2** | [**serde_json::Value**](.md) |  | [required] |
**segment_id** | Option<[**serde_json::Value**](.md)> | The ID of the segment you are updating. |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**patch_contactdb_segments_segment_id_request** | Option<[**PatchContactdbSegmentsSegmentIdRequest**](PatchContactdbSegmentsSegmentIdRequest.md)> |  |  |

### Return type

[**crate::models::ContactdbSegments**](contactdb_segments.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_contactdb_segments

> serde_json::Value p_ost_contactdb_segments(on_behalf_of, contactdb_segments)
Create a Segment

**This endpoint allows you to create a new segment.**     Valid operators for create and update depend on the type of the field for which you are searching.  **Dates** - \"eq\", \"ne\", \"lt\" (before), \"gt\" (after)     - You may use MM/DD/YYYY for day granularity or an epoch for second granularity. - \"empty\", \"not_empty\" - \"is within\"     - You may use an [ISO 8601 date format](https://en.wikipedia.org/wiki/ISO_8601) or the # of days.  **Text** - \"contains\" - \"eq\" (is/equals - matches the full field) - \"ne\" (is not/not equals - matches any field where the entire field is not the condition value) - \"empty\" - \"not_empty\"  **Numbers** - \"eq\" (is/equals) - \"lt\" (is less than) - \"gt\" (is greater than) - \"empty\" - \"not_empty\"  **Email Clicks and Opens** - \"eq\" (opened) - \"ne\" (not opened)  All field values must be a string.   Conditions using \"eq\" or \"ne\" for email clicks and opens should provide a \"field\" of either `clicks.campaign_identifier` or `opens.campaign_identifier`. The condition value should be a string containing the id of a completed campaign.   The conditions list may contain multiple conditions, joined by an \"and\" or \"or\" in the \"and_or\" field.  The first condition in the conditions list must have an empty \"and_or\", and subsequent conditions must all specify an \"and_or\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**contactdb_segments** | Option<[**ContactdbSegments**](ContactdbSegments.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

