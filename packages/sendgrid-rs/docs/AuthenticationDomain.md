# AuthenticationDomain

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**serde_json::Value**](.md)> | The ID of the authenticated domain. | 
**user_id** | Option<[**serde_json::Value**](.md)> | The ID of the user that this domain is associated with. | 
**subdomain** | Option<[**serde_json::Value**](.md)> | The subdomain to use for this authenticated domain. | 
**domain** | Option<[**serde_json::Value**](.md)> | The domain to be authenticated. | 
**username** | Option<[**serde_json::Value**](.md)> | The username that this domain will be associated with. | 
**ips** | Option<[**serde_json::Value**](.md)> | The IPs to be included in the custom SPF record for this authenticated domain. | 
**custom_spf** | Option<[**serde_json::Value**](.md)> | Indicates whether this authenticated domain uses custom SPF. | 
**default** | Option<[**serde_json::Value**](.md)> | Indicates if this is the default authenticated domain. | 
**legacy** | Option<[**serde_json::Value**](.md)> | Indicates if this authenticated domain was created using the legacy whitelabel tool. If it is a legacy whitelabel, it will still function, but you'll need to create a new authenticated domain if you need to update it. | 
**automatic_security** | Option<[**serde_json::Value**](.md)> | Indicates if this authenticated domain uses automated security. | 
**valid** | Option<[**serde_json::Value**](.md)> | Indicates if this is a valid authenticated domain. | 
**dns** | [**crate::models::AuthenticationDomainDns**](authentication_domain_dns.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


