# PostWhitelabelLinksRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | Option<[**serde_json::Value**](.md)> | The root domain for the subdomain that you are creating the link branding for. This should match your FROM email address. | 
**subdomain** | Option<[**serde_json::Value**](.md)> | The subdomain to create the link branding for. Must be different from the subdomain you used for authenticating your domain. | [optional]
**default** | Option<[**serde_json::Value**](serde_json::Value.md)> | Indicates if you want to use this link branding as the default or fallback. When setting a new default, the existing default link branding will have its default status removed automatically. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


