# Recaptchaenterprise_api Service



**Resources**: 6

---

## Overview

The recaptchaenterprise_api service provides access to 6 resource types:

- [Key](#key) [CRUD]
- [Relatedaccountgroupmembership](#relatedaccountgroupmembership) [C]
- [Assessment](#assessment) [C]
- [Relatedaccountgroup](#relatedaccountgroup) [R]
- [Membership](#membership) [R]
- [Firewallpolicie](#firewallpolicie) [CRUD]

---

## Resources


### Key

Creates a new reCAPTCHA Enterprise key.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `web_settings` | String |  | Settings for keys that can be used by websites. |
| `android_settings` | String |  | Settings for keys that can be used by Android apps. |
| `express_settings` | String |  | Settings for keys that can be used by reCAPTCHA Express. |
| `waf_settings` | String |  | Optional. Settings for Web Application Firewall (WAF). |
| `create_time` | String |  | Output only. The timestamp corresponding to the creation of this key. |
| `ios_settings` | String |  | Settings for keys that can be used by iOS apps. |
| `labels` | HashMap<String, String> |  | Optional. See [Creating and managing labels] (https://cloud.google.com/recaptcha/docs/labels). |
| `display_name` | String |  | Required. Human-readable display name of this key. Modifiable by user. |
| `name` | String |  | Identifier. The resource name for the Key in the format `projects/{project}/keys/{key}`. |
| `testing_options` | String |  | Optional. Options for user acceptance testing. |
| `parent` | String | ✅ | Required. The name of the project in which the key is created, in the format `projects/{project}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `web_settings` | String | Settings for keys that can be used by websites. |
| `android_settings` | String | Settings for keys that can be used by Android apps. |
| `express_settings` | String | Settings for keys that can be used by reCAPTCHA Express. |
| `waf_settings` | String | Optional. Settings for Web Application Firewall (WAF). |
| `create_time` | String | Output only. The timestamp corresponding to the creation of this key. |
| `ios_settings` | String | Settings for keys that can be used by iOS apps. |
| `labels` | HashMap<String, String> | Optional. See [Creating and managing labels] (https://cloud.google.com/recaptcha/docs/labels). |
| `display_name` | String | Required. Human-readable display name of this key. Modifiable by user. |
| `name` | String | Identifier. The resource name for the Key in the format `projects/{project}/keys/{key}`. |
| `testing_options` | String | Optional. Options for user acceptance testing. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create key
key = provider.recaptchaenterprise_api.Key {
    parent = "value"  # Required. The name of the project in which the key is created, in the format `projects/{project}`.
}

# Access key outputs
key_id = key.id
key_web_settings = key.web_settings
key_android_settings = key.android_settings
key_express_settings = key.express_settings
key_waf_settings = key.waf_settings
key_create_time = key.create_time
key_ios_settings = key.ios_settings
key_labels = key.labels
key_display_name = key.display_name
key_name = key.name
key_testing_options = key.testing_options
```

---


### Relatedaccountgroupmembership

Search group memberships related to a given account.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_size` | i64 |  | Optional. The maximum number of groups to return. The service might return fewer than this value. If unspecified, at most 50 groups are returned. The maximum value is 1000; values above 1000 are coerced to 1000. |
| `account_id` | String |  | Optional. The unique stable account identifier used to search connections. The identifier should correspond to an `account_id` provided in a previous `CreateAssessment` or `AnnotateAssessment` call. Either hashed_account_id or account_id must be set, but not both. |
| `page_token` | String |  | Optional. A page token, received from a previous `SearchRelatedAccountGroupMemberships` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `SearchRelatedAccountGroupMemberships` must match the call that provided the page token. |
| `hashed_account_id` | String |  | Optional. Deprecated: use `account_id` instead. The unique stable hashed account identifier used to search connections. The identifier should correspond to a `hashed_account_id` provided in a previous `CreateAssessment` or `AnnotateAssessment` call. Either hashed_account_id or account_id must be set, but not both. |
| `project` | String | ✅ | Required. The name of the project to search related account group memberships from. Specify the project name in the following format: `projects/{project}`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create relatedaccountgroupmembership
relatedaccountgroupmembership = provider.recaptchaenterprise_api.Relatedaccountgroupmembership {
    project = "value"  # Required. The name of the project to search related account group memberships from. Specify the project name in the following format: `projects/{project}`.
}

```

---


### Assessment

Creates an Assessment of the likelihood an event is legitimate.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `token_properties` | String |  | Output only. Properties of the provided event token. |
| `account_defender_assessment` | String |  | Output only. Assessment returned by account defender when an account identifier is provided. |
| `assessment_environment` | String |  | Optional. The environment creating the assessment. This describes your environment (the system invoking CreateAssessment), NOT the environment of your user. |
| `event` | String |  | Optional. The event being assessed. |
| `fraud_signals` | String |  | Output only. Fraud Signals specific to the users involved in a payment transaction. |
| `name` | String |  | Output only. Identifier. The resource name for the Assessment in the format `projects/{project}/assessments/{assessment}`. |
| `account_verification` | String |  | Optional. Account verification information for identity verification. The assessment event must include a token and site key to use this feature. |
| `private_password_leak_verification` | String |  | Optional. The private password leak verification field contains the parameters that are used to to check for leaks privately without sharing user credentials. |
| `fraud_prevention_assessment` | String |  | Output only. Assessment returned by Fraud Prevention when TransactionData is provided. |
| `firewall_policy_assessment` | String |  | Output only. Assessment returned when firewall policies belonging to the project are evaluated using the field firewall_policy_evaluation. |
| `phone_fraud_assessment` | String |  | Output only. Assessment returned when a site key, a token, and a phone number as `user_id` are provided. Account defender and SMS toll fraud protection need to be enabled. |
| `risk_analysis` | String |  | Output only. The risk analysis result for the event being assessed. |
| `parent` | String | ✅ | Required. The name of the project in which the assessment is created, in the format `projects/{project}`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create assessment
assessment = provider.recaptchaenterprise_api.Assessment {
    parent = "value"  # Required. The name of the project in which the assessment is created, in the format `projects/{project}`.
}

```

---


### Relatedaccountgroup

List groups of related accounts.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `related_account_groups` | Vec<String> | The groups of related accounts listed by the query. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access relatedaccountgroup outputs
relatedaccountgroup_id = relatedaccountgroup.id
relatedaccountgroup_next_page_token = relatedaccountgroup.next_page_token
relatedaccountgroup_related_account_groups = relatedaccountgroup.related_account_groups
```

---


### Membership

Get memberships in a group of related accounts.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `related_account_group_memberships` | Vec<String> | The memberships listed by the query. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access membership outputs
membership_id = membership.id
membership_next_page_token = membership.next_page_token
membership_related_account_group_memberships = membership.related_account_group_memberships
```

---


### Firewallpolicie

Creates a new FirewallPolicy, specifying conditions at which reCAPTCHA Enterprise actions can be executed. A project may have a maximum of 1000 policies.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `path` | String |  | Optional. The path for which this policy applies, specified as a glob pattern. For more information on glob, see the [manual page](https://man7.org/linux/man-pages/man7/glob.7.html). A path has a max length of 200 characters. |
| `actions` | Vec<String> |  | Optional. The actions that the caller should take regarding user access. There should be at most one terminal action. A terminal action is any action that forces a response, such as `AllowAction`, `BlockAction` or `SubstituteAction`. Zero or more non-terminal actions such as `SetHeader` might be specified. A single policy can contain up to 16 actions. |
| `name` | String |  | Identifier. The resource name for the FirewallPolicy in the format `projects/{project}/firewallpolicies/{firewallpolicy}`. |
| `condition` | String |  | Optional. A CEL (Common Expression Language) conditional expression that specifies if this policy applies to an incoming user request. If this condition evaluates to true and the requested path matched the path pattern, the associated actions should be executed by the caller. The condition string is checked for CEL syntax correctness on creation. For more information, see the [CEL spec](https://github.com/google/cel-spec) and its [language definition](https://github.com/google/cel-spec/blob/master/doc/langdef.md). A condition has a max length of 500 characters. |
| `description` | String |  | Optional. A description of what this policy aims to achieve, for convenience purposes. The description can at most include 256 UTF-8 characters. |
| `parent` | String | ✅ | Required. The name of the project this policy applies to, in the format `projects/{project}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `path` | String | Optional. The path for which this policy applies, specified as a glob pattern. For more information on glob, see the [manual page](https://man7.org/linux/man-pages/man7/glob.7.html). A path has a max length of 200 characters. |
| `actions` | Vec<String> | Optional. The actions that the caller should take regarding user access. There should be at most one terminal action. A terminal action is any action that forces a response, such as `AllowAction`, `BlockAction` or `SubstituteAction`. Zero or more non-terminal actions such as `SetHeader` might be specified. A single policy can contain up to 16 actions. |
| `name` | String | Identifier. The resource name for the FirewallPolicy in the format `projects/{project}/firewallpolicies/{firewallpolicy}`. |
| `condition` | String | Optional. A CEL (Common Expression Language) conditional expression that specifies if this policy applies to an incoming user request. If this condition evaluates to true and the requested path matched the path pattern, the associated actions should be executed by the caller. The condition string is checked for CEL syntax correctness on creation. For more information, see the [CEL spec](https://github.com/google/cel-spec) and its [language definition](https://github.com/google/cel-spec/blob/master/doc/langdef.md). A condition has a max length of 500 characters. |
| `description` | String | Optional. A description of what this policy aims to achieve, for convenience purposes. The description can at most include 256 UTF-8 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create firewallpolicie
firewallpolicie = provider.recaptchaenterprise_api.Firewallpolicie {
    parent = "value"  # Required. The name of the project this policy applies to, in the format `projects/{project}`.
}

# Access firewallpolicie outputs
firewallpolicie_id = firewallpolicie.id
firewallpolicie_path = firewallpolicie.path
firewallpolicie_actions = firewallpolicie.actions
firewallpolicie_name = firewallpolicie.name
firewallpolicie_condition = firewallpolicie.condition
firewallpolicie_description = firewallpolicie.description
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple key resources
key_0 = provider.recaptchaenterprise_api.Key {
    parent = "value-0"
}
key_1 = provider.recaptchaenterprise_api.Key {
    parent = "value-1"
}
key_2 = provider.recaptchaenterprise_api.Key {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    key = provider.recaptchaenterprise_api.Key {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Recaptchaenterprise_api Documentation](https://cloud.google.com/recaptchaenterprise_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
