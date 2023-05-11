# SinglesendResponseShort

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**serde_json::Value**](.md)> |  | 
**name** | Option<[**serde_json::Value**](.md)> | name of the Single Send | 
**abtest** | [**crate::models::AbtestSummary**](abtest_summary.md) |  | 
**status** | Option<[**serde_json::Value**](serde_json::Value.md)> | current status of the Single Send | 
**categories** | Option<[**serde_json::Value**](.md)> | categories to associate with this Single Send | 
**send_at** | Option<[**serde_json::Value**](.md)> | the ISO 8601 time at which to send the Single Send; must be in future | [optional]
**is_abtest** | Option<[**serde_json::Value**](.md)> | true if the Single Send's AB Test functionality has been toggled on | 
**updated_at** | Option<[**serde_json::Value**](.md)> | the ISO 8601 time at which the Single Send was last updated | 
**created_at** | Option<[**serde_json::Value**](.md)> | the ISO 8601 time at which the Single Send was created | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


