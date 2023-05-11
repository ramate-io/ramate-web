# \DesignsApiApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_design**](DesignsApiApi.md#d_elete_design) | **DELETE** /designs/{id} | Delete Design
[**g_et_design**](DesignsApiApi.md#g_et_design) | **GET** /designs/{id} | Get Design
[**g_et_sendgrid_pre_built_design**](DesignsApiApi.md#g_et_sendgrid_pre_built_design) | **GET** /designs/pre-builts/{id} | Get SendGrid Pre-built Design
[**l_ist_designs**](DesignsApiApi.md#l_ist_designs) | **GET** /designs | List Designs
[**l_ist_sendgrid_pre_built_designs**](DesignsApiApi.md#l_ist_sendgrid_pre_built_designs) | **GET** /designs/pre-builts | List SendGrid Pre-built Designs
[**p_ost_design**](DesignsApiApi.md#p_ost_design) | **POST** /designs | Create Design
[**p_ost_design_dup**](DesignsApiApi.md#p_ost_design_dup) | **POST** /designs/{id} | Duplicate Design
[**p_ost_sendgrid_pre_built_design**](DesignsApiApi.md#p_ost_sendgrid_pre_built_design) | **POST** /designs/pre-builts/{id} | Duplicate SendGrid Pre-built Design
[**p_ut_design**](DesignsApiApi.md#p_ut_design) | **PATCH** /designs/{id} | Update Design



## d_elete_design

> serde_json::Value d_elete_design(id)
Delete Design

**This endpoint allows you to delete a single design**.  Be sure to check the ID of the design you intend to delete before making this request; deleting a design is a permanent action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The ID of the Design you want to duplicate. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_design

> serde_json::Value g_et_design(id)
Get Design

**This endpoint allows you to retrieve a single design**.  A GET request to `/designs/{id}` will retrieve details about a specific design in your Design Library.  This endpoint is valuable when retrieving information stored in a field that you wish to update using a PATCH request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The ID of the Design you want to duplicate. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_sendgrid_pre_built_design

> serde_json::Value g_et_sendgrid_pre_built_design(id)
Get SendGrid Pre-built Design

**This endpoint allows you to retrieve a single pre-built design**.  A GET request to `/designs/pre-builts/{id}` will retrieve details about a specific pre-built design.  This endpoint is valuable when retrieving details about a pre-built design that you wish to duplicate and modify.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The ID of the pre-built Design you want to duplicate. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## l_ist_designs

> crate::models::ListDesigns200Response l_ist_designs(page_size, page_token, summary)
List Designs

**This endpoint allows you to retrieve a list of designs already stored in your Design Library**.  A GET request to `/designs` will return a list of your existing designs. This endpoint will not return the pre-built Twilio SendGrid designs. Pre-built designs can be retrieved using the `/designs/pre-builts` endpoint, which is detailed below.  By default, you will receive 100 results per request; however, you can modify the number of results returned by passing an integer to the `page_size` query parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<[**serde_json::Value**](.md)> | number of results to return |  |[default to 100]
**page_token** | Option<[**serde_json::Value**](.md)> | token corresponding to a specific page of results, as provided by metadata |  |
**summary** | Option<[**serde_json::Value**](.md)> | set to false to return all fields |  |[default to true]

### Return type

[**crate::models::ListDesigns200Response**](LIST_designs_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## l_ist_sendgrid_pre_built_designs

> crate::models::ListDesigns200Response l_ist_sendgrid_pre_built_designs(page_size, page_token, summary)
List SendGrid Pre-built Designs

**This endpoint allows you to retrieve a list of pre-built designs provided by Twilio SendGrid**.  Unlike the `/designs` endpoint where *your* designs are stored, a GET request made to `designs/pre-builts` will retrieve a list of the pre-built Twilio SendGrid designs. This endpoint will not return the designs stored in your Design Library.  By default, you will receive 100 results per request; however, you can modify the number of results returned by passing an integer to the `page_size` query parameter.  This endpoint is useful for retrieving the IDs of Twilio SendGrid designs that you want to duplicate and modify.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<[**serde_json::Value**](.md)> | number of results to return |  |[default to 100]
**page_token** | Option<[**serde_json::Value**](.md)> | token corresponding to a specific page of results, as provided by metadata |  |
**summary** | Option<[**serde_json::Value**](.md)> | set to false to return all fields |  |[default to true]

### Return type

[**crate::models::ListDesigns200Response**](LIST_designs_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_design

> serde_json::Value p_ost_design(body)
Create Design

**This endpoint allows you to create a new design**.  You can add a new design by passing data, including a string of HTML email content, to `/designs`. When creating designs from scratch, be aware of the styling constraints inherent to many email clients. For a list of best practices, see our guide to [Cross-Platform Email Design](https://sendgrid.com/docs/ui/sending-email/cross-platform-html-design/).  The Design Library can also convert your design’s HTML elements into drag and drop modules that are editable in the Designs Library user interface. For more, visit the [Design and Code Editor documentation](https://sendgrid.com/docs/ui/sending-email/editor/#drag--drop-markup).  Because the `/designs` endpoint makes it easy to add designs, you can create a design with your preferred tooling or migrate designs you already own without relying on the Design Library UI.

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


## p_ost_design_dup

> serde_json::Value p_ost_design_dup(id, design_duplicate_input)
Duplicate Design

**This endpoint allows you to duplicate one of your existing designs**.  Modifying an existing design is often the easiest way to create something new.  You are not required to pass any data in the body of a request to this endpoint. If you choose to leave the `name` field blank, your duplicate will be assigned the name of the design it was copied from with the text \"Duplicate: \" prepended to it. This name change is only a convenience, as the duplicate will be assigned a unique ID that differentiates it from your other designs.  You can modify your duplicate’s name at the time of creation by passing an updated value to the `name` field when making the initial request. More on retrieving design IDs can be found below.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The ID of the Design you want to duplicate. | [required] |
**design_duplicate_input** | Option<[**DesignDuplicateInput**](DesignDuplicateInput.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_sendgrid_pre_built_design

> serde_json::Value p_ost_sendgrid_pre_built_design(id, design_duplicate_input)
Duplicate SendGrid Pre-built Design

**This endpoint allows you to duplicate one of the pre-built Twilio SendGrid designs**.  Like duplicating one of your existing designs, you are not required to pass any data in the body of a request to this endpoint. If you choose to leave the `name` field blank, your duplicate will be assigned the name of the design it was copied from with the text \"Duplicate: \" prepended to it. This name change is only a convenience, as the duplicate design will be assigned a unique ID that differentiates it from your other designs. You can retrieve the IDs for Twilio SendGrid pre-built designs using the \"List SendGrid Pre-built Designs\" endpoint.  You can modify your duplicate’s name at the time of creation by passing an updated value to the `name` field when making the initial request. More on retrieving design IDs can be found above.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The ID of the pre-built Design you want to duplicate. | [required] |
**design_duplicate_input** | Option<[**DesignDuplicateInput**](DesignDuplicateInput.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ut_design

> serde_json::Value p_ut_design(id, put_design_request)
Update Design

**This endpoint allows you to edit a design**.  The Design API supports PATCH requests, which allow you to make partial updates to a single design. Passing data to a specific field will update only the data stored in that field; all other fields will be unaltered.  For example, updating a design's name requires that you make a PATCH request to this endpoint with data specified for the `name` field only.  ``` {     \"name\": \"<Updated Name>\" } ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The ID of the Design you want to duplicate. | [required] |
**put_design_request** | Option<[**PutDesignRequest**](PutDesignRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

