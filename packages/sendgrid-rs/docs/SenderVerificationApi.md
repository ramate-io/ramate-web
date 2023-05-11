# \SenderVerificationApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**d_elete_verified_senders_id**](SenderVerificationApi.md#d_elete_verified_senders_id) | **DELETE** /verified_senders/{id} | Delete Verified Sender
[**g_et_verified_senders**](SenderVerificationApi.md#g_et_verified_senders) | **GET** /verified_senders | Get All Verified Senders
[**g_et_verified_senders_domains**](SenderVerificationApi.md#g_et_verified_senders_domains) | **GET** /verified_senders/domains | Domain Warn List
[**g_et_verified_senders_steps_completed**](SenderVerificationApi.md#g_et_verified_senders_steps_completed) | **GET** /verified_senders/steps_completed | Completed Steps
[**g_et_verified_senders_verify_token**](SenderVerificationApi.md#g_et_verified_senders_verify_token) | **GET** /verified_senders/verify/{token} | Verify Sender Request
[**p_atch_verified_senders_id**](SenderVerificationApi.md#p_atch_verified_senders_id) | **PATCH** /verified_senders/{id} | Edit Verified Sender
[**p_ost_verified_senders**](SenderVerificationApi.md#p_ost_verified_senders) | **POST** /verified_senders | Create Verified Sender Request
[**p_ost_verified_senders_resend_id**](SenderVerificationApi.md#p_ost_verified_senders_resend_id) | **POST** /verified_senders/resend/{id} | Resend Verified Sender Request



## d_elete_verified_senders_id

> serde_json::Value d_elete_verified_senders_id(id)
Delete Verified Sender

**This endpoint allows you to delete a Sender Identity**.  Pass the `id` assigned to a Sender Identity to this endpoint to delete the Sender Identity from your account.  You can retrieve the IDs associated with Sender Identities using the \"Get All Verified Senders\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_verified_senders

> crate::models::GetVerifiedSenders200Response g_et_verified_senders(limit, last_seen_id, id)
Get All Verified Senders

**This endpoint allows you to retrieve all the Sender Identities associated with an account.**  This endpoint will return both verified and unverified senders.  You can limit the number of results returned using the `limit`, `lastSeenID`, and `id` query string parameters.  * `limit` allows you to specify an exact number of Sender Identities to return. * `lastSeenID` will return senders with an ID number occuring after the passed in ID. In other words, the `lastSeenID` provides a starting point from which SendGrid will iterate to find Sender Identities associated with your account. * `id` will return information about only the Sender Identity passed in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<[**serde_json::Value**](.md)> |  |  |
**last_seen_id** | Option<[**serde_json::Value**](.md)> |  |  |
**id** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

[**crate::models::GetVerifiedSenders200Response**](GET_verified_senders_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_verified_senders_domains

> crate::models::GetVerifiedSendersDomains200Response g_et_verified_senders_domains()
Domain Warn List

**This endpoint returns a list of domains known to implement DMARC and categorizes them by failure type — hard failure or soft failure**.  Domains listed as hard failures will not deliver mail when used as a [Sender Identity](https://sendgrid.com/docs/for-developers/sending-email/sender-identity/) due to the domain's DMARC policy settings.  For example, using a `yahoo.com` email address as a Sender Identity will likely result in the rejection of your mail. For more information about DMARC, see [Everything about DMARC](https://sendgrid.com/docs/ui/sending-email/dmarc/).

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetVerifiedSendersDomains200Response**](GET_verified_senders_domains_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_verified_senders_steps_completed

> crate::models::GetVerifiedSendersStepsCompleted200Response g_et_verified_senders_steps_completed()
Completed Steps

**This endpoint allows you to determine which of SendGrid’s verification processes have been completed for an account**.  This endpoint returns boolean values, `true` and `false`, for [Domain Authentication](https://sendgrid.com/docs/for-developers/sending-email/sender-identity/#domain-authentication), `domain_verified`, and [Single Sender Verification](https://sendgrid.com/docs/for-developers/sending-email/sender-identity/#single-sender-verification), `sender_verified`, for the account.  An account may have one, both, or neither verification steps completed. If you need to authenticate a domain rather than a Single Sender, see the \"Authenticate a domain\" endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetVerifiedSendersStepsCompleted200Response**](GET_verified_senders_steps_completed_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_verified_senders_verify_token

> g_et_verified_senders_verify_token(token)
Verify Sender Request

**This endpoint allows you to verify a sender requests.**  The token is generated by SendGrid and included in a verification email delivered to the address that's pending verification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | [**serde_json::Value**](.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_verified_senders_id

> crate::models::VerifiedSenderResponseSchema p_atch_verified_senders_id(id, verified_sender_request_schema)
Edit Verified Sender

**This endpoint allows you to update an existing Sender Identity**.  Pass the `id` assigned to a Sender Identity to this endpoint as a path parameter. Include any fields you wish to update in the request body in JSON format.  You can retrieve the IDs associated with Sender Identities by passing a `GET` request to the Get All Verified Senders endpoint, `/verified_senders`.  **Note:** Unlike a `PUT` request, `PATCH` allows you to update only the fields you wish to edit. Fields that are not passed as part of a request will remain unaltered.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |
**verified_sender_request_schema** | Option<[**VerifiedSenderRequestSchema**](VerifiedSenderRequestSchema.md)> |  |  |

### Return type

[**crate::models::VerifiedSenderResponseSchema**](verified-sender-response-schema.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_verified_senders

> crate::models::VerifiedSenderResponseSchema p_ost_verified_senders(verified_sender_request_schema)
Create Verified Sender Request

**This endpoint allows you to create a new Sender Identify**.  Upon successful submission of a `POST` request to this endpoint, an identity will be created, and a verification email will be sent to the address assigned to the `from_email` field. You must complete the verification process using the sent email to fully verify the sender.  If you need to resend the verification email, you can do so with the Resend Verified Sender Request, `/resend/{id}`, endpoint.  If you need to authenticate a domain rather than a Single Sender, see the [Domain Authentication API](https://sendgrid.api-docs.io/v3.0/domain-authentication/authenticate-a-domain).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verified_sender_request_schema** | Option<[**VerifiedSenderRequestSchema**](VerifiedSenderRequestSchema.md)> |  |  |

### Return type

[**crate::models::VerifiedSenderResponseSchema**](verified-sender-response-schema.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ost_verified_senders_resend_id

> serde_json::Value p_ost_verified_senders_resend_id(id)
Resend Verified Sender Request

**This endpoint allows you to resend a verification email to a specified Sender Identity**.  Passing the `id` assigned to a Sender Identity to this endpoint will resend a verification email to the `from_address` associated with the Sender Identity. This can be useful if someone loses their verification email or needs to have it resent for any other reason.  You can retrieve the IDs associated with Sender Identities by passing a \"Get All Verified Senders\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

