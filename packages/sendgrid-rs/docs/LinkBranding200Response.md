# LinkBranding200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**serde_json::Value**](.md)> | The ID of the branded link. | 
**domain** | Option<[**serde_json::Value**](.md)> | The root domain of the branded link. | 
**subdomain** | Option<[**serde_json::Value**](.md)> | The subdomain used to generate the DNS records for this link branding. This subdomain must be different from the subdomain used for your authenticated domain. | [optional]
**username** | Option<[**serde_json::Value**](.md)> | The username of the account that this link branding is associated with. | 
**user_id** | Option<[**serde_json::Value**](.md)> | The ID of the user that this link branding is associated with. | 
**default** | Option<[**serde_json::Value**](serde_json::Value.md)> | Indicates if this is the default link branding. | 
**valid** | Option<[**serde_json::Value**](serde_json::Value.md)> | Indicates if this link branding is valid. | 
**legacy** | Option<[**serde_json::Value**](serde_json::Value.md)> | Indicates if this link branding was created using the legacy whitelabel tool. If it is a legacy whitelabel, it will still function, but you'll need to create new link branding if you need to update it. | 
**dns** | [**crate::models::LinkBranding200ResponseDns**](link_branding_200_response_dns.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


