# \LinkBrandingApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_whitelabel_links_id**](LinkBrandingApi.md#d_elete_whitelabel_links_id) | **DELETE** /whitelabel/links/{id} | Delete a branded link
[**d_elete_whitelabel_links_subuser**](LinkBrandingApi.md#d_elete_whitelabel_links_subuser) | **DELETE** /whitelabel/links/subuser | Disassociate a branded link from a subuser
[**g_et_whitelabel_links**](LinkBrandingApi.md#g_et_whitelabel_links) | **GET** /whitelabel/links | Retrieve all branded links
[**g_et_whitelabel_links_default**](LinkBrandingApi.md#g_et_whitelabel_links_default) | **GET** /whitelabel/links/default | Retrieve the default branded link
[**g_et_whitelabel_links_id**](LinkBrandingApi.md#g_et_whitelabel_links_id) | **GET** /whitelabel/links/{id} | Retrieve a branded link
[**g_et_whitelabel_links_subuser**](LinkBrandingApi.md#g_et_whitelabel_links_subuser) | **GET** /whitelabel/links/subuser | Retrieve a subuser's branded link
[**p_atch_whitelabel_links_id**](LinkBrandingApi.md#p_atch_whitelabel_links_id) | **PATCH** /whitelabel/links/{id} | Update a branded link
[**p_ost_whitelabel_links**](LinkBrandingApi.md#p_ost_whitelabel_links) | **POST** /whitelabel/links | Create a branded link
[**p_ost_whitelabel_links_id_validate**](LinkBrandingApi.md#p_ost_whitelabel_links_id_validate) | **POST** /whitelabel/links/{id}/validate | Validate a branded link
[**p_ost_whitelabel_links_link_id_subuser**](LinkBrandingApi.md#p_ost_whitelabel_links_link_id_subuser) | **POST** /whitelabel/links/{link_id}/subuser | Associate a branded link with a subuser



## d_elete_whitelabel_links_id

> serde_json::Value d_elete_whitelabel_links_id(id, on_behalf_of)
Delete a branded link

**This endpoint allows you to delete a branded link.**  Your request will receive a response with a 204 status code if the deletion was successful. The call does not return the link's details, so if you wish to record these make sure you call the  \"Retrieve a branded link\" endpoint *before* you request its deletion.  You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The ID of the branded link you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_elete_whitelabel_links_subuser

> serde_json::Value d_elete_whitelabel_links_subuser(username)
Disassociate a branded link from a subuser

**This endpoint allows you to take a branded link away from a subuser.**  Link branding can be associated with subusers from the parent account. This functionality allows subusers to send mail using their parent's link branding. To associate link branding, the parent account must first create a branded link and validate it. The parent may then associate that branded link with a subuser via the API or the [Subuser Management page of the Twilio SendGrid App](https://app.sendgrid.com/settings/subusers).  Your request will receive a response with a 204 status code if the disassociation was successful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | [**serde_json::Value**](.md) | The username of the subuser account that you want to disassociate a branded link from. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_whitelabel_links

> serde_json::Value g_et_whitelabel_links(limit, on_behalf_of)
Retrieve all branded links

**This endpoint allows you to retrieve all branded links**.  You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<[**serde_json::Value**](.md)> | Limits the number of results returned per page. |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_whitelabel_links_default

> crate::models::LinkBranding200Response g_et_whitelabel_links_default(domain, on_behalf_of)
Retrieve the default branded link

**This endpoint allows you to retrieve the default branded link.**  The default branded link is the actual URL to be used when sending messages. If you have more than one branded link, the default is determined by the following order:  * The validated branded link marked as `default` (set when you call the \"Create a branded link\" endpoint or by calling the \"Update a branded link\" endpoint on an existing link) * Legacy branded links (migrated from the whitelabel wizard) * Default SendGrid-branded links (i.e., `100.ct.sendgrid.net`)  You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | Option<[**serde_json::Value**](.md)> | The domain to match against when finding the default branded link. |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::LinkBranding200Response**](link_branding_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_whitelabel_links_id

> crate::models::LinkBranding200Response g_et_whitelabel_links_id(id, on_behalf_of)
Retrieve a branded link

**This endpoint allows you to retrieve a specific branded link by providing its ID.**  You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The ID of the branded link you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::LinkBranding200Response**](link_branding_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_whitelabel_links_subuser

> crate::models::LinkBranding200Response g_et_whitelabel_links_subuser(username)
Retrieve a subuser's branded link

**This endpoint allows you to retrieve the branded link associated with a subuser.**  Link branding can be associated with subusers from the parent account. This functionality allows subusers to send mail using their parent's link branding. To associate link branding, the parent account must first create a branded link and then validate it. The parent may then associate that branded link with a subuser via the API or the [Subuser Management page of the Twilio SendGrid App](https://app.sendgrid.com/settings/subusers).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | [**serde_json::Value**](.md) | The username of the subuser to retrieve associated branded links for. | [required] |

### Return type

[**crate::models::LinkBranding200Response**](link_branding_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_whitelabel_links_id

> crate::models::LinkBranding200Response p_atch_whitelabel_links_id(id, on_behalf_of, patch_whitelabel_links_id_request)
Update a branded link

**This endpoint allows you to update a specific branded link. You can use this endpoint to change a branded link's default status.**  You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The ID of the branded link you want to retrieve. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**patch_whitelabel_links_id_request** | Option<[**PatchWhitelabelLinksIdRequest**](PatchWhitelabelLinksIdRequest.md)> |  |  |

### Return type

[**crate::models::LinkBranding200Response**](link_branding_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_whitelabel_links

> crate::models::LinkBranding200Response p_ost_whitelabel_links(on_behalf_of, post_whitelabel_links_request)
Create a branded link

**This endpoint allows you to create a new branded link.**  To create the link branding, supply the root domain and, optionally, the subdomain — these go into separate fields in your request body. The root domain should match your FROM email address. If you provide a  subdomain, it must be different from the subdomain you used for authenticating your domain.  You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**post_whitelabel_links_request** | Option<[**PostWhitelabelLinksRequest**](PostWhitelabelLinksRequest.md)> |  |  |

### Return type

[**crate::models::LinkBranding200Response**](link_branding_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_whitelabel_links_id_validate

> crate::models::PostWhitelabelLinksIdValidate200Response p_ost_whitelabel_links_id_validate(id, on_behalf_of)
Validate a branded link

**This endpoint allows you to validate a branded link.**  You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The ID of the branded link that you want to validate. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::PostWhitelabelLinksIdValidate200Response**](POST_whitelabel_links_id_validate_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_whitelabel_links_link_id_subuser

> crate::models::LinkBranding200Response p_ost_whitelabel_links_link_id_subuser(link_id, post_whitelabel_links_link_id_subuser_request)
Associate a branded link with a subuser

**This endpoint allows you to associate a branded link with a subuser account.**  Link branding can be associated with subusers from the parent account. This functionality allows subusers to send mail using their parent's link branding. To associate link branding, the parent account must first create a branded link and validate it. The parent may then associate that branded link with a subuser via the API or the [Subuser Management page of the Twilio SendGrid App](https://app.sendgrid.com/settings/subusers).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_id** | [**serde_json::Value**](.md) | The ID of the branded link you want to associate. | [required] |
**post_whitelabel_links_link_id_subuser_request** | Option<[**PostWhitelabelLinksLinkIdSubuserRequest**](PostWhitelabelLinksLinkIdSubuserRequest.md)> |  |  |

### Return type

[**crate::models::LinkBranding200Response**](link_branding_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

