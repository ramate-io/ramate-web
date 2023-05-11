# \ContactsApiRecipientsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_contactdb_recipients**](ContactsApiRecipientsApi.md#d_elete_contactdb_recipients) | **DELETE** /contactdb/recipients | Delete Recipients
[**d_elete_contactdb_recipients_recipient_id**](ContactsApiRecipientsApi.md#d_elete_contactdb_recipients_recipient_id) | **DELETE** /contactdb/recipients/{recipient_id} | Delete a Recipient
[**g_et_contactdb_recipients**](ContactsApiRecipientsApi.md#g_et_contactdb_recipients) | **GET** /contactdb/recipients | Retrieve recipients
[**g_et_contactdb_recipients_billable_count**](ContactsApiRecipientsApi.md#g_et_contactdb_recipients_billable_count) | **GET** /contactdb/recipients/billable_count | Retrieve the count of billable recipients
[**g_et_contactdb_recipients_count**](ContactsApiRecipientsApi.md#g_et_contactdb_recipients_count) | **GET** /contactdb/recipients/count | Retrieve a Count of Recipients
[**g_et_contactdb_recipients_recipient_id**](ContactsApiRecipientsApi.md#g_et_contactdb_recipients_recipient_id) | **GET** /contactdb/recipients/{recipient_id} | Retrieve a single recipient
[**g_et_contactdb_recipients_recipient_id_lists**](ContactsApiRecipientsApi.md#g_et_contactdb_recipients_recipient_id_lists) | **GET** /contactdb/recipients/{recipient_id}/lists | Retrieve the lists that a recipient is on
[**g_et_contactdb_recipients_search**](ContactsApiRecipientsApi.md#g_et_contactdb_recipients_search) | **GET** /contactdb/recipients/search | Search recipients
[**g_et_contactdb_status**](ContactsApiRecipientsApi.md#g_et_contactdb_status) | **GET** /contactdb/status | Get Recipient Upload Status
[**p_atch_contactdb_recipients**](ContactsApiRecipientsApi.md#p_atch_contactdb_recipients) | **PATCH** /contactdb/recipients | Update Recipient
[**p_ost_contactdb_recipients**](ContactsApiRecipientsApi.md#p_ost_contactdb_recipients) | **POST** /contactdb/recipients | Add recipients
[**p_ost_contactdb_recipients_search**](ContactsApiRecipientsApi.md#p_ost_contactdb_recipients_search) | **POST** /contactdb/recipients/search | Search recipients



## d_elete_contactdb_recipients

> serde_json::Value d_elete_contactdb_recipients(on_behalf_of, body)
Delete Recipients

**This endpoint allows you to deletes one or more recipients.**  The body of an API call to this endpoint must include an array of recipient IDs of the recipients you want to delete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## d_elete_contactdb_recipients_recipient_id

> serde_json::Value d_elete_contactdb_recipients_recipient_id(recipient_id, on_behalf_of)
Delete a Recipient

**This endpoint allows you to delete a single recipient with the given ID from your contact database.**  > Use this to permanently delete your recipients from all of your contact lists and all segments if required by applicable law.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recipient_id** | [**serde_json::Value**](.md) | The ID of the recipient that you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_contactdb_recipients

> crate::models::ListRecipientsResponse g_et_contactdb_recipients(page, page_size, on_behalf_of)
Retrieve recipients

**This endpoint allows you to retrieve all of your Marketing Campaigns recipients.**  Batch deletion of a page makes it possible to receive an empty page of recipients before reaching the end of the list of recipients. To avoid this issue; iterate over pages until a 404 is retrieved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<[**serde_json::Value**](.md)> | Page index of first recipients to return (must be a positive integer) |  |
**page_size** | Option<[**serde_json::Value**](.md)> | Number of recipients to return at a time (must be a positive integer between 1 and 1000) |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::ListRecipientsResponse**](List_Recipients_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_contactdb_recipients_billable_count

> crate::models::ContactdbRecipientCount g_et_contactdb_recipients_billable_count(on_behalf_of)
Retrieve the count of billable recipients

**This endpoint allows you to retrieve the number of Marketing Campaigns recipients that you will be billed for.**  You are billed for marketing campaigns based on the highest number of recipients you have had in your account at one time. This endpoint will allow you to know the current billable count value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::ContactdbRecipientCount**](contactdb_recipient_count.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_contactdb_recipients_count

> crate::models::ContactdbRecipientCount g_et_contactdb_recipients_count(on_behalf_of)
Retrieve a Count of Recipients

**This endpoint allows you to retrieve the total number of Marketing Campaigns recipients.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::ContactdbRecipientCount**](contactdb_recipient_count.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_contactdb_recipients_recipient_id

> crate::models::ContactdbRecipient g_et_contactdb_recipients_recipient_id(recipient_id, on_behalf_of)
Retrieve a single recipient

**This endpoint allows you to retrieve a single recipient by ID from your contact database.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recipient_id** | [**serde_json::Value**](.md) | The ID of the recipient that you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::ContactdbRecipient**](contactdb_recipient.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_contactdb_recipients_recipient_id_lists

> crate::models::GetContactdbRecipientsRecipientIdLists200Response g_et_contactdb_recipients_recipient_id_lists(recipient_id, on_behalf_of)
Retrieve the lists that a recipient is on

**This endpoint allows you to retrieve the lists that a given recipient belongs to.**  Each recipient can be on many lists. This endpoint gives you all of the lists that any one recipient has been added to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recipient_id** | [**serde_json::Value**](.md) | The ID of the recipient for whom you are retrieving lists. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetContactdbRecipientsRecipientIdLists200Response**](GET_contactdb_recipients_recipient_id_lists_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_contactdb_recipients_search

> crate::models::GetContactdbListsListIdRecipients200Response g_et_contactdb_recipients_search(left_curly_bracket_field_name_right_curly_bracket, on_behalf_of)
Search recipients

**This endpoint allows you to perform a search on all of your Marketing Campaigns recipients.**  field_name:  * is a variable that is substituted for your actual custom field name from your recipient. * Text fields must be url-encoded. Date fields are searchable only by unix timestamp (e.g. 2/2/2015 becomes 1422835200) * If field_name is a 'reserved' date field, such as created_at or updated_at, the system will internally convert your epoch time to a date range encompassing the entire day. For example, an epoch time of 1422835600 converts to Mon, 02 Feb 2015 00:06:40 GMT, but internally the system will search from Mon, 02 Feb 2015 00:00:00 GMT through Mon, 02 Feb 2015 23:59:59 GMT.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**left_curly_bracket_field_name_right_curly_bracket** | Option<[**serde_json::Value**](.md)> |  |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetContactdbListsListIdRecipients200Response**](GET_contactdb_lists_list_id_recipients_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_contactdb_status

> crate::models::GetContactdbStatus200Response g_et_contactdb_status(on_behalf_of)
Get Recipient Upload Status

**This endpoint allows you to check the upload status of a Marketing Campaigns recipient.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetContactdbStatus200Response**](GET_contactdb_status_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_contactdb_recipients

> crate::models::ContactdbRecipientResponse p_atch_contactdb_recipients(on_behalf_of, body)
Update Recipient

**This endpoint allows you to update one or more recipients.**  The body of an API call to this endpoint must include an array of one or more recipient objects.  It is of note that you can add custom field data as parameters on recipient objects. We have provided an example using some of the default custom fields SendGrid provides.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::ContactdbRecipientResponse**](contactdb_recipient_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_contactdb_recipients

> crate::models::ContactdbRecipientResponse p_ost_contactdb_recipients(on_behalf_of, body)
Add recipients

**This endpoint allows you to add a Marketing Campaigns recipient.**  You can add custom field data as a parameter on this endpoint. We have provided an example using some of the default custom fields SendGrid provides.  The rate limit is three requests every 2 seconds. You can upload 1000  contacts per request. So the maximum upload rate is 1500 recipients per second.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::ContactdbRecipientResponse**](contactdb_recipient_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_contactdb_recipients_search

> crate::models::PostContactdbRecipientsSearch200Response p_ost_contactdb_recipients_search(post_contactdb_recipients_search_request)
Search recipients

<p>   Search using segment conditions without actually creating a segment.   Body contains a JSON object with <code>conditions</code>, a list of conditions as described below, and an optional <code>list_id</code>, which is a valid list ID for a list to limit the search on. </p>  <p>   Valid operators for create and update depend on the type of the field for which you are searching. </p>  <ul>   <li>Dates:     <ul>       <li>\"eq\", \"ne\", \"lt\" (before), \"gt\" (after)         <ul>           <li>You may use MM/DD/YYYY for day granularity or an epoch for second granularity.</li>         </ul>       </li>       <li>\"empty\", \"not_empty\"</li>       <li>\"is within\"         <ul>           <li>You may use an <a href=\"https://en.wikipedia.org/wiki/ISO_8601\">ISO 8601 date format</a> or the # of days.</li>         </ul>       </li>     </ul>   </li>   <li>Text: \"contains\", \"eq\" (is - matches the full field), \"ne\" (is not - matches any field where the entire field is not the condition value), \"empty\", \"not_empty\"</li>   <li>Numbers: \"eq\", \"lt\", \"gt\", \"empty\", \"not_empty\"</li>   <li>Email Clicks and Opens: \"eq\" (opened), \"ne\" (not opened)</li> </ul>  <p>   Field values must all be a string. </p>  <p>   Search conditions using \"eq\" or \"ne\" for email clicks and opens should provide a \"field\" of either <code>clicks.campaign_identifier</code> or <code>opens.campaign_identifier</code>.   The condition value should be a string containing the id of a completed campaign. </p>  <p>   Search conditions list may contain multiple conditions, joined by an \"and\" or \"or\" in the \"and_or\" field.   The first condition in the conditions list must have an empty \"and_or\", and subsequent conditions must all specify an \"and_or\". </p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_contactdb_recipients_search_request** | Option<[**PostContactdbRecipientsSearchRequest**](PostContactdbRecipientsSearchRequest.md)> |  |  |

### Return type

[**crate::models::PostContactdbRecipientsSearch200Response**](POST_contactdb_recipients_search_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

