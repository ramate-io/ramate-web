# ReverseDns

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**serde_json::Value**](.md)> | The ID of the Reverse DNS. | 
**ip** | Option<[**serde_json::Value**](.md)> | The IP address that this Reverse DNS was created for. | 
**rdns** | Option<[**serde_json::Value**](.md)> | The reverse DNS record for the IP address. This points to the Reverse DNS subdomain. | 
**users** | Option<[**serde_json::Value**](.md)> | The users who are able to send mail from the IP address. | 
**subdomain** | Option<[**serde_json::Value**](.md)> | The subdomain created for this reverse DNS. This is where the rDNS record points. | [optional]
**domain** | Option<[**serde_json::Value**](.md)> | The root, or sending, domain. | 
**valid** | Option<[**serde_json::Value**](.md)> | Indicates if this is a valid Reverse DNS. | 
**legacy** | Option<[**serde_json::Value**](.md)> | Indicates if this Reverse DNS was created using the legacy whitelabel tool. If it is a legacy whitelabel, it will still function, but you'll need to create a new Reverse DNS if you need to update it. | 
**last_validation_attempt_at** | Option<[**serde_json::Value**](.md)> | A Unix epoch timestamp representing the last time of a validation attempt. | [optional]
**a_record** | [**crate::models::ReverseDnsARecord**](reverse_dns_a_record.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


