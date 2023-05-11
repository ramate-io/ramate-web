# EmailActivityResponseCommonFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_email** | Option<[**serde_json::Value**](.md)> | The 'From' email address used to deliver the message. This address should be a verified sender in your Twilio SendGrid account. | [optional][default to test0@example.com]
**msg_id** | Option<[**serde_json::Value**](.md)> | A unique ID assigned to the message. This ID can be used to retrieve activity data for the specific message. | [optional]
**subject** | Option<[**serde_json::Value**](.md)> | The email's subject line. | [optional]
**to_email** | Option<[**serde_json::Value**](.md)> | The intended recipient's email address. | [optional]
**status** | Option<[**serde_json::Value**](serde_json::Value.md)> | The message's status. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


