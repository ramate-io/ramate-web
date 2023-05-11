# PostSsoCertificatesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public_certificate** | Option<[**serde_json::Value**](.md)> | This public certificate allows SendGrid to verify that SAML requests it receives are signed by an IdP that it recognizes. | 
**enabled** | Option<[**serde_json::Value**](.md)> | Indicates if the certificate is enabled. | [optional]
**integration_id** | Option<[**serde_json::Value**](.md)> | An ID that matches a certificate to a specific IdP integration. This is the `id` returned by the \"Get All SSO Integrations\" endpoint. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


