# CampaignRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<[**serde_json::Value**](.md)> | The display title of your campaign. This will be viewable by you in the Marketing Campaigns UI. | 
**subject** | Option<[**serde_json::Value**](.md)> | The subject of your campaign that your recipients will see. | [optional]
**sender_id** | Option<[**serde_json::Value**](.md)> | The ID of the \"sender\" identity that you have created. Your recipients will see this as the \"from\" on your marketing emails. | [optional]
**list_ids** | Option<[**serde_json::Value**](.md)> | The IDs of the lists you are sending this campaign to. You can have both segment IDs and list IDs | [optional]
**segment_ids** | Option<[**serde_json::Value**](.md)> | The segment IDs that you are sending this list to. You can have both segment IDs and list IDs. Segments are limited to 10 segment IDs. | [optional]
**categories** | Option<[**serde_json::Value**](.md)> | The categories you would like associated to this campaign. | [optional]
**suppression_group_id** | Option<[**serde_json::Value**](.md)> | The suppression group that this marketing email belongs to, allowing recipients to opt-out of emails of this type. | [optional]
**custom_unsubscribe_url** | Option<[**serde_json::Value**](.md)> | This is the url of the custom unsubscribe page that you provide for customers to unsubscribe from your suppression groups. | [optional]
**ip_pool** | Option<[**serde_json::Value**](.md)> | The pool of IPs that you would like to send this email from. | [optional]
**html_content** | Option<[**serde_json::Value**](.md)> | The HTML of your marketing email. | [optional]
**plain_content** | Option<[**serde_json::Value**](.md)> | The plain text content of your emails. | [optional]
**editor** | Option<[**serde_json::Value**](serde_json::Value.md)> | The editor used in the UI. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


