# EventWebhookResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<[**serde_json::Value**](.md)> | Indicates if the event webhook is enabled. | 
**url** | Option<[**serde_json::Value**](.md)> | The URL that you want the event webhook to POST to. | 
**group_resubscribe** | Option<[**serde_json::Value**](.md)> | Recipient resubscribes to specific group by updating preferences. You need to enable Subscription Tracking for getting this type of event. | 
**delivered** | Option<[**serde_json::Value**](.md)> | Message has been successfully delivered to the receiving server. | 
**group_unsubscribe** | Option<[**serde_json::Value**](.md)> | Recipient unsubscribe from specific group, by either direct link or updating preferences. You need to enable Subscription Tracking for getting this type of event. | 
**spam_report** | Option<[**serde_json::Value**](.md)> | Recipient marked a message as spam. | 
**bounce** | Option<[**serde_json::Value**](.md)> | Receiving server could not or would not accept message. | 
**deferred** | Option<[**serde_json::Value**](.md)> | Recipient's email server temporarily rejected message. | 
**unsubscribe** | Option<[**serde_json::Value**](.md)> | Recipient clicked on message's subscription management link. You need to enable Subscription Tracking for getting this type of event. | 
**processed** | Option<[**serde_json::Value**](.md)> | Message has been received and is ready to be delivered. | 
**open** | Option<[**serde_json::Value**](.md)> | Recipient has opened the HTML message. You need to enable Open Tracking for getting this type of event. | 
**click** | Option<[**serde_json::Value**](.md)> | Recipient clicked on a link within the message. You need to enable Click Tracking for getting this type of event. | 
**dropped** | Option<[**serde_json::Value**](.md)> | You may see the following drop reasons: Invalid SMTPAPI header, Spam Content (if spam checker app enabled), Unsubscribed Address, Bounced Address, Spam Reporting Address, Invalid, Recipient List over Package Quota | 
**oauth_client_id** | Option<[**serde_json::Value**](.md)> | The client ID Twilio SendGrid sends to your OAuth server or service provider to generate an OAuth access token. | [optional]
**oauth_token_url** | Option<[**serde_json::Value**](.md)> | The URL where Twilio SendGrid sends the Client ID and Client Secret to generate an access token. This should be your OAuth server or service provider. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


