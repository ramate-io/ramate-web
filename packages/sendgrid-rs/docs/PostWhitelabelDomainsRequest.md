# PostWhitelabelDomainsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | Option<[**serde_json::Value**](.md)> | Domain being authenticated. | 
**subdomain** | Option<[**serde_json::Value**](.md)> | The subdomain to use for this authenticated domain. | [optional]
**username** | Option<[**serde_json::Value**](.md)> | The username associated with this domain. | [optional]
**ips** | Option<[**serde_json::Value**](.md)> | The IP addresses that will be included in the custom SPF record for this authenticated domain. | [optional]
**custom_spf** | Option<[**serde_json::Value**](.md)> | Specify whether to use a custom SPF or allow SendGrid to manage your SPF. This option is only available to authenticated domains set up for manual security. | [optional]
**default** | Option<[**serde_json::Value**](.md)> | Whether to use this authenticated domain as the fallback if no authenticated domains match the sender's domain. | [optional]
**automatic_security** | Option<[**serde_json::Value**](.md)> | Whether to allow SendGrid to manage your SPF records, DKIM keys, and DKIM key rotation. | [optional]
**custom_dkim_selector** | Option<[**serde_json::Value**](.md)> | Add a custom DKIM selector. Accepts three letters or numbers. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


