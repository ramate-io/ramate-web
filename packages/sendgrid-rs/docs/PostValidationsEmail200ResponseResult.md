# PostValidationsEmail200ResponseResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | Option<[**serde_json::Value**](.md)> | The email being validated | 
**verdict** | Option<[**serde_json::Value**](serde_json::Value.md)> | A generic classification of whether or not the email address is valid. | 
**score** | Option<[**serde_json::Value**](.md)> | A numeric representation of the email validity. | 
**local** | Option<[**serde_json::Value**](.md)> | The local part of the email address. | 
**host** | Option<[**serde_json::Value**](.md)> | The domain of the email address. | 
**suggestion** | Option<[**serde_json::Value**](.md)> | A suggested correction in the event of domain name typos (e.g., gmial.com) | [optional]
**checks** | [**crate::models::PostValidationsEmail200ResponseResultChecks**](POST_validations_email_200_response_result_checks.md) |  | 
**source** | Option<[**serde_json::Value**](.md)> | The source of the validation, as per the API request. | [optional]
**ip_address** | Option<[**serde_json::Value**](.md)> | The IP address associated with this email. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


