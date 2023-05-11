# \CampaignsApiApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_campaigns_campaign_id**](CampaignsApiApi.md#d_elete_campaigns_campaign_id) | **DELETE** /campaigns/{campaign_id} | Delete a Campaign
[**d_elete_campaigns_campaign_id_schedules**](CampaignsApiApi.md#d_elete_campaigns_campaign_id_schedules) | **DELETE** /campaigns/{campaign_id}/schedules | Unschedule a Scheduled Campaign
[**g_et_campaigns**](CampaignsApiApi.md#g_et_campaigns) | **GET** /campaigns | Retrieve all Campaigns
[**g_et_campaigns_campaign_id**](CampaignsApiApi.md#g_et_campaigns_campaign_id) | **GET** /campaigns/{campaign_id} | Retrieve a single campaign
[**g_et_campaigns_campaign_id_schedules**](CampaignsApiApi.md#g_et_campaigns_campaign_id_schedules) | **GET** /campaigns/{campaign_id}/schedules | View Scheduled Time of a Campaign
[**p_atch_campaigns_campaign_id**](CampaignsApiApi.md#p_atch_campaigns_campaign_id) | **PATCH** /campaigns/{campaign_id} | Update a Campaign
[**p_atch_campaigns_campaign_id_schedules**](CampaignsApiApi.md#p_atch_campaigns_campaign_id_schedules) | **PATCH** /campaigns/{campaign_id}/schedules | Update a Scheduled Campaign
[**p_ost_campaigns**](CampaignsApiApi.md#p_ost_campaigns) | **POST** /campaigns | Create a Campaign
[**p_ost_campaigns_campaign_id_schedules**](CampaignsApiApi.md#p_ost_campaigns_campaign_id_schedules) | **POST** /campaigns/{campaign_id}/schedules | Schedule a Campaign
[**p_ost_campaigns_campaign_id_schedules_now**](CampaignsApiApi.md#p_ost_campaigns_campaign_id_schedules_now) | **POST** /campaigns/{campaign_id}/schedules/now | Send a Campaign
[**p_ost_campaigns_campaign_id_schedules_test**](CampaignsApiApi.md#p_ost_campaigns_campaign_id_schedules_test) | **POST** /campaigns/{campaign_id}/schedules/test | Send a Test Campaign



## d_elete_campaigns_campaign_id

> serde_json::Value d_elete_campaigns_campaign_id(campaign_id, on_behalf_of)
Delete a Campaign

**This endpoint allows you to delete a specific campaign.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | [**serde_json::Value**](.md) | The id of the campaign you would like to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_elete_campaigns_campaign_id_schedules

> serde_json::Value d_elete_campaigns_campaign_id_schedules(campaign_id, on_behalf_of)
Unschedule a Scheduled Campaign

**This endpoint allows you to unschedule a campaign that has already been scheduled to be sent.**  A successful unschedule will return a 204. If the specified campaign is in the process of being sent, the only option is to cancel (a different method).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_campaigns

> crate::models::GetCampaigns200Response g_et_campaigns(limit, offset, on_behalf_of)
Retrieve all Campaigns

**This endpoint allows you to retrieve a list of all of your campaigns.**  Returns campaigns in reverse order they were created (newest first).  Returns an empty array if no campaigns exist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<[**serde_json::Value**](.md)> | The number of results you would like to receive at a time. |  |[default to 10]
**offset** | Option<[**serde_json::Value**](.md)> | The index of the first campaign to return, where 0 is the first campaign. |  |[default to 0]
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetCampaigns200Response**](GET_campaigns_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_campaigns_campaign_id

> crate::models::GetCampaignsCampaignId200Response g_et_campaigns_campaign_id(campaign_id, on_behalf_of)
Retrieve a single campaign

**This endpoint allows you to retrieve a specific campaign.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | [**serde_json::Value**](.md) | The id of the campaign you would like to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetCampaignsCampaignId200Response**](GET_campaigns_campaign_id_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_campaigns_campaign_id_schedules

> crate::models::ViewScheduledTimeOfACampaignResponse g_et_campaigns_campaign_id_schedules(campaign_id, on_behalf_of)
View Scheduled Time of a Campaign

**This endpoint allows you to retrieve the date and time that a campaign has been scheduled to be sent.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::ViewScheduledTimeOfACampaignResponse**](View_Scheduled_Time_of_a_Campaign_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_campaigns_campaign_id

> serde_json::Value p_atch_campaigns_campaign_id(campaign_id, on_behalf_of, update_a_campaign_request)
Update a Campaign

**This endpoint allows you to update a specific campaign.**  This is especially useful if you only set up the campaign using POST /campaigns, but didn't set many of the parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | [**serde_json::Value**](.md) | The id of the campaign you would like to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**update_a_campaign_request** | Option<[**UpdateACampaignRequest**](UpdateACampaignRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_campaigns_campaign_id_schedules

> crate::models::UpdateAScheduledCampaignResponse p_atch_campaigns_campaign_id_schedules(campaign_id, on_behalf_of, update_a_scheduled_campaign_request)
Update a Scheduled Campaign

**This endpoint allows to you change the scheduled time and date for a campaign to be sent.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**update_a_scheduled_campaign_request** | Option<[**UpdateAScheduledCampaignRequest**](UpdateAScheduledCampaignRequest.md)> |  |  |

### Return type

[**crate::models::UpdateAScheduledCampaignResponse**](Update_a_Scheduled_Campaign_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_campaigns

> serde_json::Value p_ost_campaigns(on_behalf_of, campaign_request)
Create a Campaign

**This endpoint allows you to create a campaign.**  In order to send or schedule the campaign, you will be required to provide a subject, sender ID, content (we suggest both html and plain text), and at least one list or segment ID. This information is not required when you create a campaign.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**campaign_request** | Option<[**CampaignRequest**](CampaignRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_campaigns_campaign_id_schedules

> crate::models::ScheduleACampaignResponse p_ost_campaigns_campaign_id_schedules(campaign_id, on_behalf_of, schedule_a_campaign_request)
Schedule a Campaign

**This endpoint allows you to schedule a specific date and time for your campaign to be sent.**  If you have the flexibility, it's better to schedule mail for off-peak times. Most emails are scheduled and sent at the top of the hour or half hour. Scheduling email to avoid those times (for example, scheduling at 10:53) can result in lower deferral rates because it won't be going through our servers at the same times as everyone else's mail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**schedule_a_campaign_request** | Option<[**ScheduleACampaignRequest**](ScheduleACampaignRequest.md)> |  |  |

### Return type

[**crate::models::ScheduleACampaignResponse**](Schedule_a_Campaign_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_campaigns_campaign_id_schedules_now

> crate::models::SendACampaignResponse p_ost_campaigns_campaign_id_schedules_now(campaign_id, on_behalf_of)
Send a Campaign

**This endpoint allows you to immediately send an existing campaign.**  Normally a POST request would have a body, but since this endpoint is telling us to send a resource that is already created, a request body is not needed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::SendACampaignResponse**](Send_a_Campaign_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_campaigns_campaign_id_schedules_test

> crate::models::SendATestCampaignRequest p_ost_campaigns_campaign_id_schedules_test(campaign_id, on_behalf_of, post_campaigns_campaign_id_schedules_test_request)
Send a Test Campaign

**This endpoint allows you to send a test campaign.**  To send to multiple addresses, use an array for the JSON \"to\" value [\"one@address\",\"two@address\"]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**post_campaigns_campaign_id_schedules_test_request** | Option<[**PostCampaignsCampaignIdSchedulesTestRequest**](PostCampaignsCampaignIdSchedulesTestRequest.md)> |  |  |

### Return type

[**crate::models::SendATestCampaignRequest**](Send_a_Test_Campaign_request.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

