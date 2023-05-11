# GetIpsIpAddress200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ip** | Option<[**serde_json::Value**](.md)> | The IP address. | 
**subusers** | Option<[**serde_json::Value**](.md)> | The subusers that can send email using this IP address. | 
**rdns** | Option<[**serde_json::Value**](.md)> | The reverse DNS record for this IP address. | 
**pools** | Option<[**serde_json::Value**](.md)> | The list of IP pools that this IP address belongs to. | 
**warmup** | Option<[**serde_json::Value**](.md)> | Indicates if this IP address is currently warming up. | 
**start_date** | Option<[**serde_json::Value**](.md)> | The date that the IP address was entered into warmup. | 
**whitelabeled** | Option<[**serde_json::Value**](.md)> | Indicates if this IP address is associated with a reverse DNS record. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


