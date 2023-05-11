# \UsersApiApi

All URIs are relative to *http://api.sendgrid.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**g_et_user_account**](UsersApiApi.md#g_et_user_account) | **GET** /user/account | Get a user's account information.
[**g_et_user_credits**](UsersApiApi.md#g_et_user_credits) | **GET** /user/credits | Retrieve your credit balance
[**g_et_user_email**](UsersApiApi.md#g_et_user_email) | **GET** /user/email | Retrieve your account email address
[**g_et_user_profile**](UsersApiApi.md#g_et_user_profile) | **GET** /user/profile | Get a user's profile
[**g_et_user_username**](UsersApiApi.md#g_et_user_username) | **GET** /user/username | Retrieve your username
[**p_atch_user_profile**](UsersApiApi.md#p_atch_user_profile) | **PATCH** /user/profile | Update a user's profile
[**p_ut_user_email**](UsersApiApi.md#p_ut_user_email) | **PUT** /user/email | Update your account email address
[**p_ut_user_password**](UsersApiApi.md#p_ut_user_password) | **PUT** /user/password | Update your password
[**p_ut_user_username**](UsersApiApi.md#p_ut_user_username) | **PUT** /user/username | Update your username



## g_et_user_account

> crate::models::GetUserAccountResponse g_et_user_account(on_behalf_of)
Get a user's account information.

**This endpoint allows you to retrieve your user account details.**  Your user's account information includes the user's account type and reputation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetUserAccountResponse**](GET_User_Account_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_user_credits

> crate::models::GetUserCredits200Response g_et_user_credits(on_behalf_of)
Retrieve your credit balance

**This endpoint allows you to retrieve the current credit balance for your account.**  Each account has a credit balance, which is a base number of emails it can send before receiving per-email charges. For more information about credits and billing, see [Billing and Plan details information](https://sendgrid.com/docs/ui/account-and-settings/billing/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetUserCredits200Response**](GET_user_credits_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_user_email

> crate::models::GetUserEmail200Response g_et_user_email(on_behalf_of)
Retrieve your account email address

**This endpoint allows you to retrieve the email address currently on file for your account.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetUserEmail200Response**](GET_user_email_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_user_profile

> crate::models::GetUserProfileResponse g_et_user_profile(on_behalf_of)
Get a user's profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetUserProfileResponse**](GET_User_Profile_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## g_et_user_username

> crate::models::GetUserUsername200Response g_et_user_username(on_behalf_of)
Retrieve your username

**This endpoint allows you to retrieve your current account username.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]

### Return type

[**crate::models::GetUserUsername200Response**](GET_user_username_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_atch_user_profile

> crate::models::UserProfile p_atch_user_profile(on_behalf_of, user_profile)
Update a user's profile

**This endpoint allows you to update your current profile details.**  Any one or more of the parameters can be updated via the PATCH `/user/profile` endpoint. You must include at least one when you PATCH.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**user_profile** | Option<[**UserProfile**](UserProfile.md)> |  |  |

### Return type

[**crate::models::UserProfile**](user_profile.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ut_user_email

> crate::models::PutUserEmail200Response p_ut_user_email(on_behalf_of, put_user_email_request)
Update your account email address

**This endpoint allows you to update the email address currently on file for your account.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**put_user_email_request** | Option<[**PutUserEmailRequest**](PutUserEmailRequest.md)> |  |  |

### Return type

[**crate::models::PutUserEmail200Response**](PUT_user_email_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ut_user_password

> serde_json::Value p_ut_user_password(on_behalf_of, put_user_password_request)
Update your password

**This endpoint allows you to update your password.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**put_user_password_request** | Option<[**PutUserPasswordRequest**](PutUserPasswordRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## p_ut_user_username

> crate::models::PutUserUsername200Response p_ut_user_username(on_behalf_of, put_user_username_request)
Update your username

**This endpoint allows you to update the username for your account.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**on_behalf_of** | Option<[**serde_json::Value**](.md)> |  |  |[default to The subuser's username. This header generates the API call as if the subuser account was making the call.]
**put_user_username_request** | Option<[**PutUserUsernameRequest**](PutUserUsernameRequest.md)> |  |  |

### Return type

[**crate::models::PutUserUsername200Response**](PUT_user_username_200_response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

