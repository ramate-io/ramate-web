# \ContactsApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_eletemc_contacts**](ContactsApi.md#d_eletemc_contacts) | **DELETE** /marketing/contacts | Delete Contacts
[**g_et_marketing_contacts_exports**](ContactsApi.md#g_et_marketing_contacts_exports) | **GET** /marketing/contacts/exports | Get All Existing Exports
[**g_et_marketing_contacts_imports_id**](ContactsApi.md#g_et_marketing_contacts_imports_id) | **GET** /marketing/contacts/imports/{id} | Import Contacts Status
[**g_etmc_contacts_count**](ContactsApi.md#g_etmc_contacts_count) | **GET** /marketing/contacts/count | Get Total Contact Count
[**g_etmc_contacts_exports_id**](ContactsApi.md#g_etmc_contacts_exports_id) | **GET** /marketing/contacts/exports/{id} | Export Contacts Status
[**g_etmc_contacts_id**](ContactsApi.md#g_etmc_contacts_id) | **GET** /marketing/contacts/{id} | Get a Contact by ID
[**g_etmc_contats**](ContactsApi.md#g_etmc_contats) | **GET** /marketing/contacts | Get Sample Contacts
[**p_ost_marketing_contacts_batch**](ContactsApi.md#p_ost_marketing_contacts_batch) | **POST** /marketing/contacts/batch | Get Batched Contacts by IDs
[**p_ost_marketing_contacts_search_emails**](ContactsApi.md#p_ost_marketing_contacts_search_emails) | **POST** /marketing/contacts/search/emails | Get Contacts by Emails
[**p_ostmc_contacts_exports**](ContactsApi.md#p_ostmc_contacts_exports) | **POST** /marketing/contacts/exports | Export Contacts
[**p_ostmc_contacts_search**](ContactsApi.md#p_ostmc_contacts_search) | **POST** /marketing/contacts/search | Search Contacts
[**p_utmc_contacts**](ContactsApi.md#p_utmc_contacts) | **PUT** /marketing/contacts | Add or Update a Contact
[**p_utmc_contacts_imports**](ContactsApi.md#p_utmc_contacts_imports) | **PUT** /marketing/contacts/imports | Import Contacts



## d_eletemc_contacts

> crate::models::DeleteMcContacts202Response d_eletemc_contacts(delete_all_contacts, ids)
Delete Contacts

**This endpoint can be used to delete one or more contacts**.  The query parameter `ids` must set to a comma-separated list of contact IDs for bulk contact deletion.  The query parameter `delete_all_contacts` must be set to `\"true\"` to delete **all** contacts.   You must set either `ids` or `delete_all_contacts`.  Deletion jobs are processed asynchronously.  Twilio SendGrid recommends exporting your contacts regularly as a backup to avoid issues or lost data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_all_contacts** | Option<[**serde_json::Value**](.md)> | Must be set to `\"true\"` to delete all contacts. |  |
**ids** | Option<[**serde_json::Value**](.md)> | A comma-separated list of contact IDs. |  |

### Return type

[**crate::models::DeleteMcContacts202Response**](DELETE_mc_contacts_202_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_marketing_contacts_exports

> crate::models::GetMarketingContactsExports200Response g_et_marketing_contacts_exports()
Get All Existing Exports

**Use this endpoint to retrieve details of all current exported jobs**.  It will return an array of objects, each of which records an export job in flight or recently completed.   Each object's `export_type` field will tell you which kind of export it is and its `status` field will indicate what stage of processing it has reached. Exports which are `ready` will be accompanied by a `urls` field which lists the URLs of the export's downloadable files — there will be more than one if you specified a maximum file size in your initial export request.  Use this endpoint if you have exports in flight but do not know their IDs, which are required for the \"Export Contacts Status\" endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetMarketingContactsExports200Response**](GET_marketing_contacts_exports_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_marketing_contacts_imports_id

> crate::models::ContactImport g_et_marketing_contacts_imports_id(id)
Import Contacts Status

**This endpoint can be used to check the status of a contact import job**.   Use the `job_id` from the \"Import Contacts,\" \"Add or Update a Contact,\" or \"Delete Contacts\" endpoints as the `id` in the path parameter.  If there is an error with your `PUT` request, download the `errors_url` file and open it to view more details.  The job `status` field indicates whether the job is `pending`, `completed`, `errored`, or `failed`.   Pending means not started. Completed means finished without any errors. Errored means finished with some errors. Failed means finshed with all errors, or the job was entirely unprocessable: for example, if you attempt to import file format we do not support.  The `results` object will have fields depending on the job type.  Twilio SendGrid recommends exporting your contacts regularly as a backup to avoid issues or lost data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**crate::models::ContactImport**](contact-import.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_etmc_contacts_count

> crate::models::GetMcContactsCount200Response g_etmc_contacts_count()
Get Total Contact Count

**This endpoint returns the total number of contacts you have stored.**   Twilio SendGrid recommends exporting your contacts regularly as a backup to avoid issues or lost data.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetMcContactsCount200Response**](GET_mc_contacts_count_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_etmc_contacts_exports_id

> crate::models::ContactExport g_etmc_contacts_exports_id(id)
Export Contacts Status

**This endpoint can be used to check the status of a contact export job**.   To use this call, you will need the `id` from the \"Export Contacts\" call.  If you would like to download a list, take the `id` that is returned from the \"Export Contacts\" endpoint and make an API request here to get the `urls`. Once you have the list of URLs, make a `GET` request on each URL to download your CSV file(s).  Twilio SendGrid recommends exporting your contacts regularly as a backup to avoid issues or lost data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**crate::models::ContactExport**](contact-export.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_etmc_contacts_id

> crate::models::ContactDetails3 g_etmc_contacts_id(id)
Get a Contact by ID

**This endpoint returns the full details and all fields for the specified contact**.  The \"Get Contacts by Emails\" endpoint can be used to get the ID of a contact.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**crate::models::ContactDetails3**](contact-details3.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_etmc_contats

> crate::models::GetMcContats200Response g_etmc_contats()
Get Sample Contacts

**This endpoint will return up to 50 of the most recent contacts uploaded or attached to a list**.   This list will then be sorted by email address.  The full contact count is also returned.  Please note that pagination of the contacts has been deprecated.  Twilio SendGrid recommends exporting your contacts regularly as a backup to avoid issues or lost data.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetMcContats200Response**](GET_mc_contats_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_marketing_contacts_batch

> crate::models::PostMarketingContactsBatch200Response p_ost_marketing_contacts_batch(post_marketing_contacts_batch_request)
Get Batched Contacts by IDs

**This endpoint is used to retrieve a set of contacts identified by their IDs.**  This can be more efficient endpoint to get contacts than making a series of individual `GET` requests to the \"Get a Contact by ID\" endpoint.  You can supply up to 100 IDs. Pass them into the `ids` field in your request body as an array or one or more strings.  Twilio SendGrid recommends exporting your contacts regularly as a backup to avoid issues or lost data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_marketing_contacts_batch_request** | Option<[**PostMarketingContactsBatchRequest**](PostMarketingContactsBatchRequest.md)> |  |  |

### Return type

[**crate::models::PostMarketingContactsBatch200Response**](POST_marketing_contacts_batch_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_marketing_contacts_search_emails

> crate::models::PostMarketingContactsSearchEmails200Response p_ost_marketing_contacts_search_emails(post_marketing_contacts_search_emails_request)
Get Contacts by Emails

**This endpoint allows you to retrieve up to 100 contacts matching the searched `email` address(es), including any `alternate_emails`.**   Email addresses are unique to a contact, meaning this endpoint can treat an email address as a primary key to search by. The contact object associated with the address, whether it is their `email` or one of their `alternate_emails` will be returned if matched.  Email addresses in the search request do not need to match the case in which they're stored, but the email addresses in the result will be all lower case. Empty strings are excluded from the search and will not be returned.  This endpoint should be used in place of the \"Search Contacts\" endpoint when you can provide exact email addresses and do not need to include other [Segmentation Query Language (SGQL)](https://sendgrid.com/docs/for-developers/sending-email/segmentation-query-language/) filters when searching.  If you need to access a large percentage of your contacts, we recommend exporting your contacts with the \"Export Contacts\" endpoint and filtering the results client side.  This endpoint returns a `200` status code when any contacts match the address(es) you supplied. When searching multiple addresses in a single request, it is possible that some addresses will match a contact while others will not. When a partially successful search like this is made, the matching contacts are returned in an object and an error message is returned for the email address(es) that are not found.   This endpoint returns a `404` status code when no contacts are found for the provided email address(es).  A `400` status code is returned if any searched addresses are invalid.  Twilio SendGrid recommends exporting your contacts regularly as a backup to avoid issues or lost data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_marketing_contacts_search_emails_request** | Option<[**PostMarketingContactsSearchEmailsRequest**](PostMarketingContactsSearchEmailsRequest.md)> |  |  |

### Return type

[**crate::models::PostMarketingContactsSearchEmails200Response**](POST_marketing_contacts_search_emails_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ostmc_contacts_exports

> crate::models::PostMcContactsExports202Response p_ostmc_contacts_exports(post_mc_contacts_exports_request)
Export Contacts

**Use this endpoint to export lists or segments of contacts**.  If you would just like to have a link to the exported list sent to your email set the `notifications.email` option to `true` in the `POST` payload.  If you would like to download the list, take the `id` that is returned and use the \"Export Contacts Status\" endpoint to get the `urls`. Once you have the list of URLs, make a `GET` request to each URL provided to download your CSV file(s).  You specify the segements and or/contact lists you wish to export by providing the relevant IDs in, respectively, the `segment_ids` and `list_ids` fields in the request body.  The lists will be provided in either JSON or CSV files. To specify which of these you would required, set the request body `file_type` field to `json` or `csv`.  You can also specify a maximum file size (in MB). If the export file is larger than this, it will be split into multiple files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_mc_contacts_exports_request** | Option<[**PostMcContactsExportsRequest**](PostMcContactsExportsRequest.md)> |  |  |

### Return type

[**crate::models::PostMcContactsExports202Response**](POST_mc_contacts_exports_202_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ostmc_contacts_search

> crate::models::PostMcContactsSearch200Response p_ostmc_contacts_search(post_mc_contacts_search_request)
Search Contacts

**Use this endpoint to locate contacts**.  The request body's `query` field accepts valid [SGQL](https://sendgrid.com/docs/for-developers/sending-email/segmentation-query-language/) for searching for a contact.  Because contact emails are stored in lower case, using SGQL to search by email address requires the provided email address to be in lower case. The SGQL `lower()` function can be used for this.  Only the first 50 contacts that meet the search criteria will be returned.  If the query takes longer than 20 seconds, a `408 Request Timeout` status will be returned.  Formatting the `created_at` and `updated_at` values as Unix timestamps is deprecated. Instead they are returned as ISO format as string.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_mc_contacts_search_request** | Option<[**PostMcContactsSearchRequest**](PostMcContactsSearchRequest.md)> |  |  |

### Return type

[**crate::models::PostMcContactsSearch200Response**](POST_mc_contacts_search_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_utmc_contacts

> crate::models::PutMcContacts202Response p_utmc_contacts(put_mc_contacts_request)
Add or Update a Contact

**This endpoint allows the [upsert](https://en.wiktionary.org/wiki/upsert) (insert or update) of up to 30,000 contacts, or 6MB of data, whichever is lower**.   Because the creation and update of contacts is an asynchronous process, the response will not contain immediate feedback on the processing of your upserted contacts. Rather, it will contain an HTTP 202 response indicating the contacts are queued for processing or an HTTP 4XX error containing validation errors. Should you wish to get the resulting contact's ID or confirm your contacts have been updated or added, you can use the \"Get Contacts by Emails\" endpoint.   Please note that custom fields need to have been already created if you wish to set their values for the contacts being upserted. To do this, please use the \"Create Custom Field Definition\" endpoint.  You will see a `job_id` in the response to your request. This can be used to check the status of your upsert job. To do so, please use the \"Import Contacts Status\" endpoint.  If the contact already exists in the system, any entries submitted via this endpoint will update the existing contact. The contact to update will be determined only by the `email` field and any fields omitted from the request will remain as they were. A contact's ID cannot be used to update the contact.  The email field will be changed to all lower-case. If a contact is added with an email that exists but contains capital letters, the existing contact with the all lower-case email will be updated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_mc_contacts_request** | Option<[**PutMcContactsRequest**](PutMcContactsRequest.md)> |  |  |

### Return type

[**crate::models::PutMcContacts202Response**](PUT_mc_contacts_202_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_utmc_contacts_imports

> crate::models::PutMcContactsImports200Response p_utmc_contacts_imports(put_mc_contacts_imports_request)
Import Contacts

**This endpoint allows a CSV upload containing up to one million contacts or 5GB of data, whichever is smaller.**  Imports take place asynchronously: the endpoint returns a URL (`upload_uri`) and HTTP headers (`upload_headers`) which can subsequently be used to `PUT` a file of contacts to be  imported into our system.  Uploaded CSV files may also be [gzip-compressed](https://en.wikipedia.org/wiki/Gzip).  In either case, you must include the field `file_type` with the value `csv` in your request body.  The `field_mappings` paramter is a respective list of field definition IDs to map the uploaded CSV columns to. It allows you to use CSVs where one or more columns are skipped (`null`) or remapped to the contact field.   For example, if `field_mappings` is set to `[null, \"w1\", \"_rf1\"]`, this means skip column 0, map column 1 to the custom field with the ID `w1`, and map column 2 to the reserved field with the ID `_rf1`. See the \"Get All Field Definitions\" endpoint to fetch your custom and reserved field IDs to use with `field_mappings`.  Once you recieve the response body you can then initiate a **second** API call where you use the supplied URL and HTTP header to upload your file. For example:  `curl --upload-file \"file/path.csv\" \"URL_GIVEN\" -H 'HEADER_GIVEN'`  If you'd like to monitor the status of your import job, use the `job_id` and the \"Import Contacts Status\" endpoint.  Twilio SendGrid recommends exporting your contacts regularly as a backup to avoid issues or lost data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_mc_contacts_imports_request** | Option<[**PutMcContactsImportsRequest**](PutMcContactsImportsRequest.md)> |  |  |

### Return type

[**crate::models::PutMcContactsImports200Response**](PUT_mc_contacts_imports_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

