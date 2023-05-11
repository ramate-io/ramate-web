# DomainAuthenticationDomainSpf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**serde_json::Value**](.md)> | The ID of the authenticated domain. | 
**domain** | Option<[**serde_json::Value**](.md)> | The domain authenticated. | 
**subdomain** | Option<[**serde_json::Value**](.md)> | The subdomain that was used to create this authenticated domain. | [optional]
**username** | Option<[**serde_json::Value**](.md)> | The username of the account that this authenticated domain is associated with. | 
**user_id** | Option<[**serde_json::Value**](.md)> | The user_id of the account that this authenticated domain is associated with. | 
**ips** | Option<[**serde_json::Value**](.md)> | The IP addresses that are included in the SPF record for this authenticated domain. | 
**custom_spf** | Option<[**serde_json::Value**](.md)> | Indicates if this authenticated domain uses custom SPF. | 
**default** | Option<[**serde_json::Value**](.md)> | Indicates if this is the default domain. | 
**legacy** | Option<[**serde_json::Value**](.md)> | Indicates if this authenticated domain was created using the legacy whitelabel tool. If it is a legacy whitelabel, it will still function, but you'll need to create a new authenticated domain if you need to update it. | 
**automatic_security** | Option<[**serde_json::Value**](.md)> | Indicates if this authenticated domain uses automated security. | 
**valid** | Option<[**serde_json::Value**](.md)> | Indicates if this is a valid authenticated domain . | 
**dns** | [**crate::models::DomainAuthenticationDomainSpfDns**](domain_authentication_domain_spf_dns.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


