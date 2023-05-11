# Message

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_email** | Option<[**serde_json::Value**](.md)> |  | 
**msg_id** | Option<[**serde_json::Value**](.md)> |  | 
**subject** | Option<[**serde_json::Value**](.md)> |  | 
**to_email** | Option<[**serde_json::Value**](.md)> |  | 
**status** | Option<[**serde_json::Value**](serde_json::Value.md)> | Quick summary of the status of a message | 
**template_id** | Option<[**serde_json::Value**](.md)> |  | 
**asm_group_id** | Option<[**serde_json::Value**](.md)> |  | 
**teammate** | Option<[**serde_json::Value**](.md)> | Teammate's username | 
**api_key_id** | Option<[**serde_json::Value**](.md)> |  | 
**events** | Option<[**serde_json::Value**](.md)> | List of events related to email message | 
**originating_ip** | Option<[**serde_json::Value**](.md)> | This is the IP of the user who sent the message. | 
**categories** | Option<[**serde_json::Value**](.md)> | Categories users associated to the message | 
**unique_args** | Option<[**serde_json::Value**](.md)> | JSON hash of arbitrary key-value pairs | [default to Null]
**outbound_ip** | Option<[**serde_json::Value**](.md)> | IP used to send to the remote MTA. Used by UI to display IP in detailed view | 
**outbound_ip_type** | Option<[**serde_json::Value**](serde_json::Value.md)> | Whether or not the outbound IP is dedicated vs shared | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


