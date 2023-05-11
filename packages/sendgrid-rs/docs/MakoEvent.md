# MakoEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_name** | Option<[**serde_json::Value**](serde_json::Value.md)> | Name of event | 
**processed** | Option<[**serde_json::Value**](.md)> | Date of when event occurred | 
**reason** | Option<[**serde_json::Value**](.md)> | Explanation of what caused \"bounced\", \"deferred\", or \"blocked\". Usually contains error message from the server - e.g. message from gmail why mail was deferred | [optional]
**attempt_num** | Option<[**serde_json::Value**](.md)> | Used with \"deferred\" events to indicate the attempt number out of 10. One \"deferred\" entry will exists under events array for each time a message was deferred with error message from the server.  | [optional]
**url** | Option<[**serde_json::Value**](.md)> | Used with \"clicked\" event to indicate which url the user clicked. | 
**bounce_type** | Option<[**serde_json::Value**](serde_json::Value.md)> | Use to distinguish between types of bounces | 
**http_user_agent** | Option<[**serde_json::Value**](.md)> | Client recipient used to click or open message | 
**mx_server** | Option<[**serde_json::Value**](.md)> | For example mx.gmail.com | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


