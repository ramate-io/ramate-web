# PostValidationsEmail200ResponseResultChecksDomain

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**has_valid_address_syntax** | Option<[**serde_json::Value**](.md)> | Whether the email address syntax is valid. | 
**has_mx_or_a_record** | Option<[**serde_json::Value**](.md)> | Whether the email has appropriate DNS records to deliver a message.  | 
**is_suspected_disposable_address** | Option<[**serde_json::Value**](.md)> | Whether the domain appears to be from a disposable email address service. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


