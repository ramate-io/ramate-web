# \DomainAuthenticationApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_whitelabel_domains_domain_id**](DomainAuthenticationApi.md#d_elete_whitelabel_domains_domain_id) | **DELETE** /whitelabel/domains/{domain_id} | Delete an authenticated domain.
[**d_elete_whitelabel_domains_id_ips_ip**](DomainAuthenticationApi.md#d_elete_whitelabel_domains_id_ips_ip) | **DELETE** /whitelabel/domains/{id}/ips/{ip} | Remove an IP from an authenticated domain.
[**d_elete_whitelabel_domains_subuser**](DomainAuthenticationApi.md#d_elete_whitelabel_domains_subuser) | **DELETE** /whitelabel/domains/subuser | Disassociate an authenticated domain from a given user.
[**g_et_whitelabel_domains**](DomainAuthenticationApi.md#g_et_whitelabel_domains) | **GET** /whitelabel/domains | List all authenticated domains
[**g_et_whitelabel_domains_default**](DomainAuthenticationApi.md#g_et_whitelabel_domains_default) | **GET** /whitelabel/domains/default | Get the default authentication
[**g_et_whitelabel_domains_domain_id**](DomainAuthenticationApi.md#g_et_whitelabel_domains_domain_id) | **GET** /whitelabel/domains/{domain_id} | Retrieve an authenticated domain
[**g_et_whitelabel_domains_subuser**](DomainAuthenticationApi.md#g_et_whitelabel_domains_subuser) | **GET** /whitelabel/domains/subuser | List the authenticated domain associated with the given user.
[**p_atch_whitelabel_domains_domain_id**](DomainAuthenticationApi.md#p_atch_whitelabel_domains_domain_id) | **PATCH** /whitelabel/domains/{domain_id} | Update an authenticated domain
[**p_ost_whitelabel_domains**](DomainAuthenticationApi.md#p_ost_whitelabel_domains) | **POST** /whitelabel/domains | Authenticate a domain
[**p_ost_whitelabel_domains_domain_id_subuser**](DomainAuthenticationApi.md#p_ost_whitelabel_domains_domain_id_subuser) | **POST** /whitelabel/domains/{domain_id}/subuser | Associate an authenticated domain with a given user.
[**p_ost_whitelabel_domains_id_ips**](DomainAuthenticationApi.md#p_ost_whitelabel_domains_id_ips) | **POST** /whitelabel/domains/{id}/ips | Add an IP to an authenticated domain
[**p_ost_whitelabel_domains_id_validate**](DomainAuthenticationApi.md#p_ost_whitelabel_domains_id_validate) | **POST** /whitelabel/domains/{id}/validate | Validate a domain authentication.



## d_elete_whitelabel_domains_domain_id

> serde_json::Value d_elete_whitelabel_domains_domain_id(domain_id, on_behalf_of)
Delete an authenticated domain.

**This endpoint allows you to delete an authenticated domain.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_elete_whitelabel_domains_id_ips_ip

> crate::models::DomainAuthenticationDomainSpf d_elete_whitelabel_domains_id_ips_ip(id, ip, on_behalf_of)
Remove an IP from an authenticated domain.

**This endpoint allows you to remove an IP address from that domain's authentication.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | ID of the domain to delete the IP from. | [required] |
**ip** | [**serde_json::Value**](.md) | IP to remove from the domain. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::DomainAuthenticationDomainSpf**](domain_authentication_domain_spf.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_elete_whitelabel_domains_subuser

> serde_json::Value d_elete_whitelabel_domains_subuser(username)
Disassociate an authenticated domain from a given user.

**This endpoint allows you to disassociate a specific authenticated domain from a subuser.**  Authenticated domains can be associated with (i.e. assigned to) subusers from a parent account. This functionality allows subusers to send mail using their parent's domain authentication. To associate an authenticated domain with a subuser, the parent account must first authenticate and validate the domain. The parent may then associate the authenticated domain via the subuser management tools.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<[**serde_json::Value**](.md)> | Username for the subuser to find associated authenticated domain. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_whitelabel_domains

> serde_json::Value g_et_whitelabel_domains(limit, offset, exclude_subusers, username, domain, on_behalf_of)
List all authenticated domains

**This endpoint allows you to retrieve a list of all domains you have authenticated.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<[**serde_json::Value**](.md)> | Number of domains to return. |  |
**offset** | Option<[**serde_json::Value**](.md)> | Paging offset. |  |
**exclude_subusers** | Option<[**serde_json::Value**](.md)> | Exclude subuser domains from the result. |  |
**username** | Option<[**serde_json::Value**](.md)> | The username associated with an authenticated domain. |  |
**domain** | Option<[**serde_json::Value**](.md)> | Search for authenticated domains. |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_whitelabel_domains_default

> serde_json::Value g_et_whitelabel_domains_default(domain, on_behalf_of)
Get the default authentication

**This endpoint allows you to retrieve the default authentication for a domain.**  When creating or updating a domain authentication, you can set the domain as a default. The default domain will be used to send all mail. If you have multiple authenticated domains, the authenticated domain matching the domain of the From address will be used, and the default will be overridden.  This endpoint will return a default domain and its details only if a default is set. You are not required to set a default. If you do not set a default domain, this endpoint will return general information about your domain authentication status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | Option<[**serde_json::Value**](.md)> | The domain to find a default authentication. |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_whitelabel_domains_domain_id

> crate::models::AuthenticationDomain g_et_whitelabel_domains_domain_id(domain_id, on_behalf_of)
Retrieve an authenticated domain

**This endpoint allows you to retrieve a specific authenticated domain.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::AuthenticationDomain**](authentication_domain.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_whitelabel_domains_subuser

> crate::models::DomainAuthenticationDomainSpf g_et_whitelabel_domains_subuser(username)
List the authenticated domain associated with the given user.

**This endpoint allows you to retrieve all of the authenticated domains that have been assigned to a specific subuser.**  Authenticated domains can be associated with (i.e. assigned to) subusers from a parent account. This functionality allows subusers to send mail using their parent's domain authentication. To associate an authenticated domain with a subuser, the parent account must first authenticate and validate the domain. The parent may then associate the authenticated domain via the subuser management tools.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | [**serde_json::Value**](.md) | Username for the subuser to find associated authenticated domain. | [required] |

### Return type

[**crate::models::DomainAuthenticationDomainSpf**](domain_authentication_domain_spf.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_whitelabel_domains_domain_id

> serde_json::Value p_atch_whitelabel_domains_domain_id(domain_id, on_behalf_of, patch_whitelabel_domains_domain_id_request)
Update an authenticated domain

**This endpoint allows you to update the settings for an authenticated domain.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | [**serde_json::Value**](.md) |  | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**patch_whitelabel_domains_domain_id_request** | Option<[**PatchWhitelabelDomainsDomainIdRequest**](PatchWhitelabelDomainsDomainIdRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_whitelabel_domains

> crate::models::AuthenticationDomain p_ost_whitelabel_domains(on_behalf_of, post_whitelabel_domains_request)
Authenticate a domain

**This endpoint allows you to authenticate a domain.**  If you are authenticating a domain for a subuser, you have two options: 1. Use the \"username\" parameter. This allows you to authenticate a domain on behalf of your subuser. This means the subuser is able to see and modify the authenticated domain. 2. Use the Association workflow (see Associate Domain section). This allows you to authenticate a domain created by the parent to a subuser. This means the subuser will default to the assigned domain, but will not be able to see or modify that authenticated domain. However, if the subuser authenticates their own domain it will overwrite the assigned domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**post_whitelabel_domains_request** | Option<[**PostWhitelabelDomainsRequest**](PostWhitelabelDomainsRequest.md)> |  |  |

### Return type

[**crate::models::AuthenticationDomain**](authentication_domain.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_whitelabel_domains_domain_id_subuser

> crate::models::DomainAuthenticationDomainSpf p_ost_whitelabel_domains_domain_id_subuser(domain_id, post_whitelabel_domains_domain_id_subuser_request)
Associate an authenticated domain with a given user.

**This endpoint allows you to associate a specific authenticated domain with a subuser.**  Authenticated domains can be associated with (i.e. assigned to) subusers from a parent account. This functionality allows subusers to send mail using their parent's domain authentication. To associate an authenticated domain with a subuser, the parent account must first authenticate and validate the domain. The parent may then associate the authenticated domain via the subuser management tools.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | [**serde_json::Value**](.md) | ID of the authenticated domain to associate with the subuser | [required] |
**post_whitelabel_domains_domain_id_subuser_request** | Option<[**PostWhitelabelDomainsDomainIdSubuserRequest**](PostWhitelabelDomainsDomainIdSubuserRequest.md)> |  |  |

### Return type

[**crate::models::DomainAuthenticationDomainSpf**](domain_authentication_domain_spf.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_whitelabel_domains_id_ips

> crate::models::DomainAuthenticationDomainSpf p_ost_whitelabel_domains_id_ips(id, on_behalf_of, post_whitelabel_domains_id_ips_request)
Add an IP to an authenticated domain

**This endpoint allows you to add an IP address to an authenticated domain.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | ID of the domain to which you are adding an IP | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**post_whitelabel_domains_id_ips_request** | Option<[**PostWhitelabelDomainsIdIpsRequest**](PostWhitelabelDomainsIdIpsRequest.md)> |  |  |

### Return type

[**crate::models::DomainAuthenticationDomainSpf**](domain_authentication_domain_spf.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_whitelabel_domains_id_validate

> crate::models::PostWhitelabelDomainsIdValidate200Response p_ost_whitelabel_domains_id_validate(id, on_behalf_of)
Validate a domain authentication.

**This endpoint allows you to validate an authenticated domain. If it fails, it will return an error message describing why the domain could not be validated.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | ID of the domain to validate. | [required] |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::PostWhitelabelDomainsIdValidate200Response**](POST_whitelabel_domains_id_validate_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

