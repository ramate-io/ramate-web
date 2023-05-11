# SinglesendRequestEmailConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subject** | Option<[**serde_json::Value**](.md)> | The subject line of the Single Send. Do not include this field when using a `design_id`. | [optional]
**html_content** | Option<[**serde_json::Value**](.md)> | The HTML content of the Single Send. Do not include this field when using a `design_id`. | [optional]
**plain_content** | Option<[**serde_json::Value**](.md)> | The plain text content of the Single Send. Do not include this field when using a `design_id`. | [optional]
**generate_plain_content** | Option<[**serde_json::Value**](.md)> | If set to `true`, `plain_content` is always generated from `html_content`. If set to false, `plain_content` is not altered. | [optional][default to true]
**design_id** | Option<[**serde_json::Value**](.md)> | A `design_id` can be used in place of `html_content`, `plain_content`, and/or `subject`. You can retrieve a design's ID from the [\"List Designs\" endpoint](https://sendgrid.api-docs.io/v3.0/designs-api/list-designs) or by pulling it from the design's detail page URL in the Marketing Campaigns App. | [optional]
**editor** | Option<[**serde_json::Value**](serde_json::Value.md)> | The editor — `\"design\"` or `\"code\"` — used to modify the Single Send's design in the Marketing Campaigns App. | [optional][default to Code]
**suppression_group_id** | Option<[**serde_json::Value**](.md)> | The ID of the Suppression Group to allow recipients to unsubscribe — you must provide this or the `custom_unsubscribe_url`. | [optional]
**custom_unsubscribe_url** | Option<[**serde_json::Value**](.md)> | The URL allowing recipients to unsubscribe — you must provide this or the `suppression_group_id`. | [optional]
**sender_id** | Option<[**serde_json::Value**](.md)> | The ID of the verified Sender. You can retrieve a verified Sender's ID from the [\"Get Verified Senders\" endpoint](https://sendgrid.api-docs.io/v3.0/sender-verification/get-verified-senders) or by pulling it from the Sender's detail page URL in the SendGrid App. | [optional]
**ip_pool** | Option<[**serde_json::Value**](.md)> | The name of the IP Pool from which the Single Send emails are sent. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


