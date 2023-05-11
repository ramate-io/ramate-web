# MailSettingsBouncePurge

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<[**serde_json::Value**](.md)> | Indicates if the bounce purge mail setting is enabled. | [optional]
**soft_bounces** | Option<[**serde_json::Value**](.md)> | The number of days after which SendGrid will purge all contacts from your soft bounces suppression lists. | [optional]
**hard_bounces** | Option<[**serde_json::Value**](.md)> | The number of days after which SendGrid will purge all contacts from your hard bounces suppression lists. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


