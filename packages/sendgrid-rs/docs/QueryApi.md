# \QueryApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**g_et_messages**](QueryApi.md#g_et_messages) | **GET** /messages | Filter all messages
[**g_etv3_messages_msg_id**](QueryApi.md#g_etv3_messages_msg_id) | **GET** /messages/{msg_id} | Filter messages by message ID



## g_et_messages

> crate::models::GetMessages200Response g_et_messages(query, authorization, limit, x_query_id, x_cursor)
Filter all messages

This is **BETA** functionality. You may not have access, and we reserve the right to change functionality without notice.  Filter all messages to search your Email Activity. All queries need to be [URL encoded](https://meyerweb.com/eric/tools/dencoder/), and have this format:  `query={query_type}=\"{query_content}\"`  encoded, this would look like this:  `query=type%3D%22query_content%22`  for example:  Filter by a specific email - `query=to_email%3D%22example%40example.com%22`  Filter by subject line - `query=subject%3d%22A%20Great%20Subject%22`  **Full list of basic query types and examples:**   | **Filter query**    | **Unencoded Example** (put this one into the try it out query - it'll automatically encode it for you) | **Encoded Example** (use this one in your code)                        | |-----------------|----------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------| | msg_id          | msg_id=“filter0307p1las1-16816-5A023E36-1.0”                                                                               | msg_id%3D%22filter0307p1las1-16816-5A023E36-1.0%22           | | from_email      | from_email=“testing@sendgrid.net”                                                                                          | from_email%3D%22testing%40sendgrid.net%22                    | | subject         | subject=\"This is a subject test\"                                                                                      | subject%22This%20is%20a%20subject%20test%22                  | | to_email        | to_email=\"example@example.com\"                                                                                       | to_email%3D%22example%40example.com%22                       | | status          |                                                                                                                            | status%22processed%22                                        | | template_id     |                                                                                                                            |                                                                    | | asm_group_id    |                                                                                                                            |                                                                    | | api_key_id      |                                                                                                                            |                                                                    | | events          | status=\"processed\"                                                                                                   | status%3D%22processed%22                                     | | originating_ip  |                                                                                                                            |                                                                    | | categories      |                                                                                                                            |                                                                    | | unique_args     |                                                                                                                            |                                                                    | | outbound_ip     |                                                                                                                            |                                                                    | | last_event_time | last_event_time=“2017-11-07T23:13:58Z”                                                                               | last_event_time%3D%E2%80%9C2017-11-07T23%3A13%3A58Z%E2%80%9D | | clicks          | clicks=\"0\"                                                                                                           | clicks%3D%220%22                                             |  For information about building compound queries, and for the full query language functionality, see the [query language reference](https://docs.google.com/a/sendgrid.com/document/d/1fWoKTFNfg5UUsB6t9KuIcSo9CetKF_T0bGfWJ_gdPCs/edit?usp=sharing).  Coming soon, example compound queries: limit + to email + date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | [**serde_json::Value**](.md) | Use the query syntax  to filter your email activity. | [required] |
**authorization** | [**serde_json::Value**](.md) |  | [required] |[default to Bearer <<YOUR_API_KEY_HERE>>]
**limit** | Option<[**serde_json::Value**](.md)> | The number of messages returned. This parameter must be greater than 0 and less than or equal to 1000 |  |[default to 10]
**x_query_id** | Option<[**serde_json::Value**](.md)> |  |  |
**x_cursor** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

[**crate::models::GetMessages200Response**](GET_messages_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_etv3_messages_msg_id

> crate::models::Message g_etv3_messages_msg_id(authorization, msg_id)
Filter messages by message ID

This is BETA functionality. You may not have access, and we reserve the right to change functionality without notice.  Get all of the details about the specified message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | [**serde_json::Value**](.md) |  | [required] |[default to Bearer <<YOUR_API_KEY_HERE>>]
**msg_id** | [**serde_json::Value**](.md) | The ID of the message you are requesting details for. | [required] |

### Return type

[**crate::models::Message**](Message.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

