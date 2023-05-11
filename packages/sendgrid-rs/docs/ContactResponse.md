# ContactResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**serde_json::Value**](.md)> | ID assigned to a contact when added to the system. | 
**email** | Option<[**serde_json::Value**](.md)> | Email of the contact. This is a reserved field. | 
**alternate_emails** | Option<[**serde_json::Value**](.md)> | Alternate emails of the contact. This is a reserved field. | 
**first_name** | Option<[**serde_json::Value**](.md)> | First name of the contact. This is a reserved field. | 
**last_name** | Option<[**serde_json::Value**](.md)> | Last name of the contact. This is a reserved field. | 
**address_line_1** | Option<[**serde_json::Value**](.md)> | First line of address of the contact. This is a reserved field. | 
**address_line_2** | Option<[**serde_json::Value**](.md)> | Second line of address of the contact. This is a reserved field. | 
**city** | Option<[**serde_json::Value**](.md)> | City associated with the contact. This is a reserved field. | 
**state_province_region** | Option<[**serde_json::Value**](.md)> | State associated with the contact. This is a reserved field. | 
**postal_code** | Option<[**serde_json::Value**](.md)> | Zipcode associated with the address of the contact. This is a reserved field. | 
**country** | Option<[**serde_json::Value**](.md)> | Country associated with the address of the contact. This is a reserved field. | 
**list_ids** | Option<[**serde_json::Value**](.md)> | IDs of all lists the contact is part of | [optional]
**custom_fields** | [**crate::models::ContactResponseCustomFields**](contact_response_custom_fields.md) |  | 
**segment_ids** | Option<[**serde_json::Value**](.md)> | IDs of all segments the contact is part of | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


