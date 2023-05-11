# SubscriptionTrackingSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<[**serde_json::Value**](.md)> | Indicates if subscription tracking is enabled. | [optional]
**html_content** | Option<[**serde_json::Value**](.md)> | The information and HTML for your unsubscribe link.  | [optional]
**landing** | Option<[**serde_json::Value**](.md)> | The HTML that will be displayed on the page that your customers will see after clicking unsubscribe, hosted on SendGrid’s server. | [optional]
**plain_content** | Option<[**serde_json::Value**](.md)> | The information in plain text for your unsubscribe link. You should have the “<% %>” tag in your content, otherwise the user will have no URL for unsubscribing. | [optional]
**replace** | Option<[**serde_json::Value**](.md)> | Your custom defined replacement tag for your templates. Use this tag to place your unsubscribe content anywhere in your emailtemplate. | [optional]
**url** | Option<[**serde_json::Value**](.md)> | The URL where you would like your users sent to unsubscribe. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


