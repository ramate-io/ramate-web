# \SettingsMailApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**g_et_mail_settings**](SettingsMailApi.md#g_et_mail_settings) | **GET** /mail_settings | Retrieve all mail settings
[**g_et_mail_settings_address_whitelist**](SettingsMailApi.md#g_et_mail_settings_address_whitelist) | **GET** /mail_settings/address_whitelist | Retrieve address whitelist mail settings
[**g_et_mail_settings_bounce_purge**](SettingsMailApi.md#g_et_mail_settings_bounce_purge) | **GET** /mail_settings/bounce_purge | Retrieve bounce purge mail settings
[**g_et_mail_settings_footer**](SettingsMailApi.md#g_et_mail_settings_footer) | **GET** /mail_settings/footer | Retrieve footer mail settings
[**g_et_mail_settings_forward_bounce**](SettingsMailApi.md#g_et_mail_settings_forward_bounce) | **GET** /mail_settings/forward_bounce | Retrieve forward bounce mail settings
[**g_et_mail_settings_forward_spam**](SettingsMailApi.md#g_et_mail_settings_forward_spam) | **GET** /mail_settings/forward_spam | Retrieve forward spam mail settings
[**g_et_mail_settings_template**](SettingsMailApi.md#g_et_mail_settings_template) | **GET** /mail_settings/template | Retrieve legacy template mail settings
[**p_atch_mail_settings_address_whitelist**](SettingsMailApi.md#p_atch_mail_settings_address_whitelist) | **PATCH** /mail_settings/address_whitelist | Update address whitelist mail settings
[**p_atch_mail_settings_bounce_purge**](SettingsMailApi.md#p_atch_mail_settings_bounce_purge) | **PATCH** /mail_settings/bounce_purge | Update bounce purge mail settings
[**p_atch_mail_settings_footer**](SettingsMailApi.md#p_atch_mail_settings_footer) | **PATCH** /mail_settings/footer | Update footer mail settings
[**p_atch_mail_settings_forward_bounce**](SettingsMailApi.md#p_atch_mail_settings_forward_bounce) | **PATCH** /mail_settings/forward_bounce | Update forward bounce mail settings
[**p_atch_mail_settings_forward_spam**](SettingsMailApi.md#p_atch_mail_settings_forward_spam) | **PATCH** /mail_settings/forward_spam | Update forward spam mail settings
[**p_atch_mail_settings_template**](SettingsMailApi.md#p_atch_mail_settings_template) | **PATCH** /mail_settings/template | Update template mail settings



## g_et_mail_settings

> crate::models::GetMailSettings200Response g_et_mail_settings(limit, offset, on_behalf_of)
Retrieve all mail settings

**This endpoint allows you to retrieve a list of all mail settings.**  Each setting will be returned with an `enabled` status set to `true` or `false` and a short description that explains what the setting does.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<[**serde_json::Value**](.md)> | The number of settings to return. |  |
**offset** | Option<[**serde_json::Value**](.md)> | Where in the list of results to begin displaying settings. |  |
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetMailSettings200Response**](GET_mail_settings_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_mail_settings_address_whitelist

> crate::models::MailSettingsAddressWhitelabel g_et_mail_settings_address_whitelist(on_behalf_of)
Retrieve address whitelist mail settings

**This endpoint allows you to retrieve your current email address whitelist settings.**  The Address Whitelist setting allows you to specify email addresses or domains for which mail should never be suppressed.  For example, if you own the domain `example.com`, and one or more of your recipients use `email@example.com` addresses, placing `example.com` in the address whitelist setting instructs Twilio SendGrid to ignore all bounces, blocks, and unsubscribes logged for that domain. In other words, all bounces, blocks, and unsubscribes will still be sent to `example.com` as if they were sent under normal sending conditions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::MailSettingsAddressWhitelabel**](mail_settings_address_whitelabel.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_mail_settings_bounce_purge

> crate::models::MailSettingsBouncePurge g_et_mail_settings_bounce_purge(on_behalf_of)
Retrieve bounce purge mail settings

**This endpoint allows you to retrieve your current bounce and purge settings.**  The Bounce Perge setting allows you to set a schedule that Twilio SendGrid will use to automatically delete contacts from your soft and hard bounce suppression lists.  A hard bounce occurs when an email message has been returned to the sender because the recipient's address is invalid. A hard bounce might occur because the domain name doesn't exist or because the recipient is unknown.  A soft bounce occurs when an email message reaches the recipient's mail server but is bounced back undelivered before it actually reaches the recipient. A soft bounce might occur because the recipient's inbox is full.  You can also manage this setting in the [Mail Settings section of the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings). You can manage your bounces manually using the [Bounces API](https://sendgrid.api-docs.io/v3.0/bounces-api) or the [Bounces menu in the Twilio SendGrid App](https://app.sendgrid.com/suppressions/bounces).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::MailSettingsBouncePurge**](mail_settings_bounce_purge.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_mail_settings_footer

> crate::models::MailSettingsFooter g_et_mail_settings_footer(on_behalf_of)
Retrieve footer mail settings

**This endpoint allows you to retrieve your current Footer mail settings.**  The Footer setting will insert a custom footer at the bottom of your text and HTML email message bodies.  You can insert your HTML or plain text directly using the \"Update footer mail settings\" endpoint, or you can create the footer using the [Mail Settings menu in the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::MailSettingsFooter**](mail_settings_footer.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_mail_settings_forward_bounce

> crate::models::MailSettingsForwardBounce g_et_mail_settings_forward_bounce(on_behalf_of)
Retrieve forward bounce mail settings

**This endpoint allows you to retrieve your current bounce forwarding mail settings.**  Enabling the Forward Bounce setting allows you to specify `email` addresses to which bounce reports will be forwarded. This endpoint returns the email address you have set to receive forwarded bounces and an `enabled` status indicating if the setting is active.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::MailSettingsForwardBounce**](mail_settings_forward_bounce.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_mail_settings_forward_spam

> crate::models::MailSettingsForwardSpam g_et_mail_settings_forward_spam(on_behalf_of)
Retrieve forward spam mail settings

**This endpoint allows you to retrieve your current Forward Spam mail settings.**  Enabling the Forward Spam setting allows you to specify `email` addresses to which spam reports will be forwarded. This endpoint returns any email address(es) you have set to receive forwarded spam and an `enabled` status indicating if the setting is active.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::MailSettingsForwardSpam**](mail_settings_forward_spam.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_mail_settings_template

> crate::models::MailSettingsTemplate g_et_mail_settings_template(on_behalf_of)
Retrieve legacy template mail settings

**This endpoint allows you to retrieve your current legacy email template settings.**  This setting refers to our original email templates. We currently support more fully featured [Dynamic Transactional Templates](https://sendgrid.com/docs/ui/sending-email/how-to-send-an-email-with-dynamic-transactional-templates/).  The legacy email template setting wraps an HTML template around your email content. This can be useful for sending out marketing email and/or other HTML formatted messages. For instructions on using legacy templates, see how to [\"Create and Edit Legacy Transactional Templates](https://sendgrid.com/docs/ui/sending-email/create-and-edit-legacy-transactional-templates/). For help migrating to our current template system, see [\"Migrating from Legacy Templates\"](https://sendgrid.com/docs/ui/sending-email/migrating-from-legacy-templates/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::MailSettingsTemplate**](mail_settings_template.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_mail_settings_address_whitelist

> crate::models::MailSettingsAddressWhitelabel p_atch_mail_settings_address_whitelist(on_behalf_of, patch_mail_settings_address_whitelist_request)
Update address whitelist mail settings

**This endpoint allows you to update your current email address whitelist settings.**  You can select whether or not this setting should be enabled by assigning the `enabled` field a `true` or `false` value.  Passing only the `enabled` field to this endpoint will not alter your current `list` of whitelist entries. However, any modifications to your `list` of entries will overwrite the entire list. For this reason, you must included all existing entries you wish to retain in your `list` in addition to any new entries you intend to add. To remove one or more `list` entries, pass a `list` with only the entries you wish to retain.  You should not add generic domains such as `gmail.com` or `yahoo.com`  in your `list` because your emails will not honor recipients' unsubscribes. This may cause a legal violation of [CAN-SPAM](https://sendgrid.com/docs/glossary/can-spam/) and could damage your sending reputation.  The Address Whitelist setting allows you to specify email addresses or domains for which mail should never be suppressed.  For example, if you own the domain `example.com`, and one or more of your recipients use `email@example.com` addresses, placing `example.com` in the address whitelist setting instructs Twilio SendGrid to ignore all bounces, blocks, and unsubscribes logged for that domain. In other words, all bounces, blocks, and unsubscribes will still be sent to `example.com` as if they were sent under normal sending conditions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**patch_mail_settings_address_whitelist_request** | Option<[**PatchMailSettingsAddressWhitelistRequest**](PatchMailSettingsAddressWhitelistRequest.md)> |  |  |

### Return type

[**crate::models::MailSettingsAddressWhitelabel**](mail_settings_address_whitelabel.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_mail_settings_bounce_purge

> crate::models::MailSettingsBouncePurge p_atch_mail_settings_bounce_purge(on_behalf_of, mail_settings_bounce_purge)
Update bounce purge mail settings

**This endpoint allows you to update your current bounce and purge settings.**  The Bounce Perge setting allows you to set a schedule that Twilio SendGrid will use to automatically delete contacts from your soft and hard bounce suppression lists. The schedule is set in full days by assigning the number of days, an integer, to the `soft_bounces` and/or `hard_bounces` fields.  A hard bounce occurs when an email message has been returned to the sender because the recipient's address is invalid. A hard bounce might occur because the domain name doesn't exist or because the recipient is unknown.  A soft bounce occurs when an email message reaches the recipient's mail server but is bounced back undelivered before it actually reaches the recipient. A soft bounce might occur because the recipient's inbox is full.  You can also manage this setting in the [Mail Settings section of the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings). You can manage your bounces manually using the [Bounces API](https://sendgrid.api-docs.io/v3.0/bounces-api) or the [Bounces menu in the Twilio SendGrid App](https://app.sendgrid.com/suppressions/bounces).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**mail_settings_bounce_purge** | Option<[**MailSettingsBouncePurge**](MailSettingsBouncePurge.md)> |  |  |

### Return type

[**crate::models::MailSettingsBouncePurge**](mail_settings_bounce_purge.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_mail_settings_footer

> crate::models::MailSettingsFooter p_atch_mail_settings_footer(on_behalf_of, mail_settings_footer)
Update footer mail settings

**This endpoint allows you to update your current Footer mail settings.**  The Footer setting will insert a custom footer at the bottom of your text and HTML email message bodies.  You can insert your HTML or plain text directly using this endpoint, or you can create the footer using the [Mail Settings menu in the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**mail_settings_footer** | Option<[**MailSettingsFooter**](MailSettingsFooter.md)> |  |  |

### Return type

[**crate::models::MailSettingsFooter**](mail_settings_footer.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_mail_settings_forward_bounce

> crate::models::MailSettingsForwardBounce p_atch_mail_settings_forward_bounce(on_behalf_of, mail_settings_forward_bounce)
Update forward bounce mail settings

**This endpoint allows you to update your current bounce forwarding mail settings.**  Enabling the Forward Bounce setting allows you to specify an `email` address to which bounce reports will be forwarded.  You can also configure the Forward Spam mail settings in the [Mail Settings section of the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**mail_settings_forward_bounce** | Option<[**MailSettingsForwardBounce**](MailSettingsForwardBounce.md)> |  |  |

### Return type

[**crate::models::MailSettingsForwardBounce**](mail_settings_forward_bounce.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_mail_settings_forward_spam

> crate::models::MailSettingsForwardSpam p_atch_mail_settings_forward_spam(on_behalf_of, mail_settings_forward_spam)
Update forward spam mail settings

**This endpoint allows you to update your current Forward Spam mail settings.**  Enabling the Forward Spam setting allows you to specify `email` addresses to which spam reports will be forwarded. You can set multiple addresses by passing this endpoint a comma separated list of emails in a single string.  ``` {   \"email\": \"address1@example.com, address2@exapmle.com\",   \"enabled\": true } ```  The Forward Spam setting may also be used to receive emails sent to `abuse@` and `postmaster@` role addresses if you have authenticated your domain.  For example, if you authenticated `example.com` as your root domain and set a custom return path of `sub` for that domain, you could turn on Forward Spam, and any emails sent to `abuse@sub.example.com` or `postmaster@sub.example.com` would be forwarded to the email address you entered in the `email` field.  You can authenticate your domain using the \"Authenticate a domain\" endpoint or in the [Sender Authentication section of the Twilio SendGrid App](https://app.sendgrid.com/settings/sender_auth). You can also configure the Forward Spam mail settings in the [Mail Settings section of the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**mail_settings_forward_spam** | Option<[**MailSettingsForwardSpam**](MailSettingsForwardSpam.md)> |  |  |

### Return type

[**crate::models::MailSettingsForwardSpam**](mail_settings_forward_spam.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_mail_settings_template

> crate::models::PatchMailSettingsTemplate200Response p_atch_mail_settings_template(on_behalf_of, patch_mail_settings_template_request)
Update template mail settings

**This endpoint allows you to update your current legacy email template settings.**  This setting refers to our original email templates. We currently support more fully featured [Dynamic Transactional Templates](https://sendgrid.com/docs/ui/sending-email/how-to-send-an-email-with-dynamic-transactional-templates/).  The legacy email template setting wraps an HTML template around your email content. This can be useful for sending out marketing email and/or other HTML formatted messages. For instructions on using legacy templates, see how to [\"Create and Edit Legacy Transactional Templates](https://sendgrid.com/docs/ui/sending-email/create-and-edit-legacy-transactional-templates/). For help migrating to our current template system, see [\"Migrating from Legacy Templates\"](https://sendgrid.com/docs/ui/sending-email/migrating-from-legacy-templates/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**patch_mail_settings_template_request** | Option<[**PatchMailSettingsTemplateRequest**](PatchMailSettingsTemplateRequest.md)> |  |  |

### Return type

[**crate::models::PatchMailSettingsTemplate200Response**](PATCH_mail_settings_template_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

