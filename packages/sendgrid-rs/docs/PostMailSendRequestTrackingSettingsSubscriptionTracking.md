# PostMailSendRequestTrackingSettingsSubscriptionTracking

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enable** | Option<[**serde_json::Value**](.md)> | Indicates if this setting is enabled. | [optional]
**text** | Option<[**serde_json::Value**](.md)> | Text to be appended to the email with the subscription tracking link. You may control where the link is by using the tag <% %> | [optional]
**html** | Option<[**serde_json::Value**](.md)> | HTML to be appended to the email with the subscription tracking link. You may control where the link is by using the tag <% %> | [optional]
**substitution_tag** | Option<[**serde_json::Value**](.md)> | A tag that will be replaced with the unsubscribe URL. for example: `[unsubscribe_url]`. If this parameter is used, it will override both the `text` and `html` parameters. The URL of the link will be placed at the substitution tag’s location with no additional formatting. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


