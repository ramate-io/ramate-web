# PostMailSendRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**personalizations** | Option<[**serde_json::Value**](.md)> | An array of messages and their metadata. Each object within personalizations can be thought of as an envelope - it defines who should receive an individual message and how that message should be handled. See our [Personalizations documentation](https://sendgrid.com/docs/for-developers/sending-email/personalizations/) for examples. | 
**from** | [**crate::models::FromEmailObject**](from_email_object.md) |  | 
**reply_to** | Option<[**crate::models::ReplyToEmailObject**](reply_to_email_object.md)> |  | [optional]
**reply_to_list** | Option<[**serde_json::Value**](.md)> | An array of recipients who will receive replies and/or bounces. Each object in this array must contain the recipient's email address. Each object in the array may optionally contain the recipient's name. You can either choose to use “reply_to” field or “reply_to_list” but not both. | [optional]
**subject** | Option<[**serde_json::Value**](.md)> | The global or 'message level' subject of your email. This may be overridden by subject lines set in personalizations. | 
**content** | Option<[**serde_json::Value**](.md)> | An array where you can specify the content of your email. You can include multiple [MIME types](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types) of content, but you must specify at least one MIME type. To include more than one MIME type, add another object to the array containing the `type` and `value` parameters. | 
**attachments** | Option<[**serde_json::Value**](.md)> | An array of objects where you can specify any attachments you want to include. | [optional]
**template_id** | Option<[**serde_json::Value**](.md)> | An email template ID. A template that contains a subject and content — either text or html — will override any subject and content values specified at the personalizations or message level. | [optional]
**headers** | Option<[**serde_json::Value**](.md)> | An object containing key/value pairs of header names and the value to substitute for them. The key/value pairs must be strings. You must ensure these are properly encoded if they contain unicode characters. These headers cannot be one of the reserved headers. | [optional]
**categories** | Option<[**serde_json::Value**](.md)> | An array of category names for this message. Each category name may not exceed 255 characters.  | [optional]
**custom_args** | Option<[**serde_json::Value**](.md)> | Values that are specific to the entire send that will be carried along with the email and its activity data.  Key/value pairs must be strings. Substitutions will not be made on custom arguments, so any string that is entered into this parameter will be assumed to be the custom argument that you would like to be used. This parameter is overridden by `custom_args` set at the personalizations level. Total `custom_args` size may not exceed 10,000 bytes. | [optional]
**send_at** | Option<[**serde_json::Value**](.md)> | A unix timestamp allowing you to specify when you want your email to be delivered. This may be overridden by the `send_at` parameter set at the personalizations level. Delivery cannot be scheduled more than 72 hours in advance. If you have the flexibility, it's better to schedule mail for off-peak times. Most emails are scheduled and sent at the top of the hour or half hour. Scheduling email to avoid peak times — for example, scheduling at 10:53 — can result in lower deferral rates due to the reduced traffic during off-peak times. | [optional]
**batch_id** | Option<[**serde_json::Value**](.md)> | An ID representing a batch of emails to be sent at the same time. Including a `batch_id` in your request allows you include this email in that batch. It also enables you to cancel or pause the delivery of that batch. For more information, see the [Cancel Scheduled Sends API](https://sendgrid.com/docs/api-reference/). | [optional]
**asm** | Option<[**crate::models::PostMailSendRequestAsm**](POST_mail_send_request_asm.md)> |  | [optional]
**ip_pool_name** | Option<[**serde_json::Value**](.md)> | The IP Pool that you would like to send this email from. | [optional]
**mail_settings** | Option<[**crate::models::PostMailSendRequestMailSettings**](POST_mail_send_request_mail_settings.md)> |  | [optional]
**tracking_settings** | Option<[**crate::models::PostMailSendRequestTrackingSettings**](POST_mail_send_request_tracking_settings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


