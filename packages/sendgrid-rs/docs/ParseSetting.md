# ParseSetting

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | Option<[**serde_json::Value**](.md)> | The public URL where you would like SendGrid to POST the data parsed from your email. Any emails sent with the given hostname provided (whose MX records have been updated to point to SendGrid) will be parsed and POSTed to this URL. | [optional]
**hostname** | Option<[**serde_json::Value**](.md)> | A specific and unique domain or subdomain that you have created to use exclusively to parse your incoming email. For example, `parse.yourdomain.com`. | [optional]
**spam_check** | Option<[**serde_json::Value**](.md)> | Indicates if you would like SendGrid to check the content parsed from your emails for spam before POSTing them to your domain. | [optional]
**send_raw** | Option<[**serde_json::Value**](.md)> | Indicates if you would like SendGrid to post the original MIME-type content of your parsed email. When this parameter is set to `true`, SendGrid will send a JSON payload of the content of your email. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


