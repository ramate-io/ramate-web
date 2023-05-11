# PostWhitelabelIpsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ip** | Option<[**serde_json::Value**](.md)> | The IP address for which you want to set up reverse DNS. | 
**subdomain** | Option<[**serde_json::Value**](.md)> | The subdomain that will be used to send emails from the IP address. This should be the same as the subdomain used to set up an authenticated domain. | [optional]
**domain** | Option<[**serde_json::Value**](.md)> | The root, or sending, domain that will be used to send message from the IP address. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


