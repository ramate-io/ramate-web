# \SendTestEmailApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**p_ost_marketing_test_send_email**](SendTestEmailApi.md#p_ost_marketing_test_send_email) | **POST** /marketing/test/send_email | Send a Test Marketing Email



## p_ost_marketing_test_send_email

> serde_json::Value p_ost_marketing_test_send_email(post_marketing_test_send_email_request)
Send a Test Marketing Email

**This endpoint allows you to send a test marketing email to a list of email addresses**.  Before sending a marketing message, you can test it using this endpoint. You may specify up to **10 contacts** in the `emails` request body field. You must also specify a `template_id` and include either a `from_address` or `sender_id`. You can manage your templates with the [Twilio SendGrid App](https://mc.sendgrid.com/dynamic-templates) or the [Transactional Templates API](https://sendgrid.api-docs.io/v3.0/transactional-templates).  > Please note that this endpoint works with Dynamic Transactional Templates only. Legacy Transactional Templates will not be delivered.  For more information about managing Dynamic Transactional Templates, see [How to Send Email with Dynamic Transactional Templates](https://sendgrid.com/docs/ui/sending-email/how-to-send-an-email-with-dynamic-transactional-templates/).  You can also test your Single Sends in the [Twilio SendGrid Marketing Campaigns UI](https://mc.sendgrid.com/single-sends).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_marketing_test_send_email_request** | Option<[**PostMarketingTestSendEmailRequest**](PostMarketingTestSendEmailRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

