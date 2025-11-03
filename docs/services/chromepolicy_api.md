# Chromepolicy_api Service



**Resources**: 6

---

## Overview

The chromepolicy_api service provides access to 6 resource types:

- [Network](#network) [C]
- [Group](#group) [C]
- [Orgunit](#orgunit) [C]
- [Policie](#policie) [C]
- [Policy_schema](#policy_schema) [R]
- [Media](#media) [C]

---

## Resources


### Network

Creates a certificate at a specified OU for a customer.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ceritificate_name` | String |  | Optional. The optional name of the certificate. If not specified, the certificate issuer will be used as the name. |
| `target_resource` | String |  | Required. The target resource on which this certificate is applied. The following resources are supported: * Organizational Unit ("orgunits/{orgunit_id}") |
| `settings` | Vec<String> |  | Optional. Certificate settings within the chrome.networks.certificates namespace. |
| `certificate` | String |  | Required. The raw contents of the .PEM, .CRT, or .CER file. |
| `customer` | String | ✅ | Required. The customer for which the certificate will apply. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create network
network = provider.chromepolicy_api.Network {
    customer = "value"  # Required. The customer for which the certificate will apply.
}

```

---


### Group

Delete multiple policy values that are applied to a specific group. All targets must have the same target format. That is to say that they must point to the same target resource and must have the same keys specified in `additionalTargetKeyNames`, though the values for those keys may be different. On failure the request will return the error details as part of the google.rpc.Status.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | List of policies that will be deleted as defined by the `requests`. All requests in the list must follow these restrictions: 1. All schemas in the list must have the same root namespace. 2. All `policyTargetKey.targetResource` values must point to a group resource. 3. All `policyTargetKey` values must have the same `app_id` key name in the `additionalTargetKeys`. 4. No two modification requests can reference the same `policySchema` + ` policyTargetKey` pair.  |
| `customer` | String | ✅ | ID of the Google Workspace account or literal "my_customer" for the customer associated to the request. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create group
group = provider.chromepolicy_api.Group {
    customer = "value"  # ID of the Google Workspace account or literal "my_customer" for the customer associated to the request.
}

```

---


### Orgunit

Modify multiple policy values that are applied to a specific org unit so that they now inherit the value from a parent (if applicable). All targets must have the same target format. That is to say that they must point to the same target resource and must have the same keys specified in `additionalTargetKeyNames`, though the values for those keys may be different. On failure the request will return the error details as part of the google.rpc.Status.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | List of policies that have to inherit their values as defined by the `requests`. All requests in the list must follow these restrictions: 1. All schemas in the list must have the same root namespace. 2. All `policyTargetKey.targetResource` values must point to an org unit resource. 3. All `policyTargetKey` values must have the same key names in the ` additionalTargetKeys`. This also means if one of the targets has an empty `additionalTargetKeys` map, all of the targets must have an empty `additionalTargetKeys` map. 4. No two modification requests can reference the same `policySchema` + ` policyTargetKey` pair.  |
| `customer` | String | ✅ | ID of the G Suite account or literal "my_customer" for the customer associated to the request. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create orgunit
orgunit = provider.chromepolicy_api.Orgunit {
    customer = "value"  # ID of the G Suite account or literal "my_customer" for the customer associated to the request.
}

```

---


### Policie

Gets the resolved policy values for a list of policies that match a search query.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_token` | String |  | The page token used to retrieve a specific page of the request. |
| `policy_schema_filter` | String |  | Required. The schema filter to apply to the resolve request. Specify a schema name to view a particular schema, for example: chrome.users.ShowLogoutButton Wildcards are supported, but only in the leaf portion of the schema name. Wildcards cannot be used in namespace directly. Please read https://developers.google.com/chrome/policy/guides/policy-schemas for details on schema namespaces. For example: Valid: "chrome.users.*", "chrome.users.apps.*", "chrome.printers.*" Invalid: "*", "*.users", "chrome.*", "chrome.*.apps.*" |
| `page_size` | i64 |  | The maximum number of policies to return, defaults to 100 and has a maximum of 1000. |
| `policy_target_key` | String |  | Required. The key of the target resource on which the policies should be resolved. |
| `customer` | String | ✅ | ID of the G Suite account or literal "my_customer" for the customer associated to the request. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policie
policie = provider.chromepolicy_api.Policie {
    customer = "value"  # ID of the G Suite account or literal "my_customer" for the customer associated to the request.
}

```

---


### Policy_schema

Get a specific policy schema for a customer by its resource name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `schema_name` | String | Output only. The fully qualified name of the policy schema. This value is used to fill the field `policy_schema` in PolicyValue when calling BatchInheritOrgUnitPolicies BatchModifyOrgUnitPolicies BatchModifyGroupPolicies or BatchDeleteGroupPolicies. |
| `support_uri` | String | Output only. URI to related support article for this schema. |
| `notices` | Vec<String> | Output only. Special notice messages related to setting certain values in certain fields in the schema. |
| `category_title` | String | Title of the category in which a setting belongs. |
| `additional_target_key_names` | Vec<String> | Output only. Additional key names that will be used to identify the target of the policy value. When specifying a `policyTargetKey`, each of the additional keys specified here will have to be included in the `additionalTargetKeys` map. |
| `field_descriptions` | Vec<String> | Output only. Detailed description of each field that is part of the schema. Fields are suggested to be displayed by the ordering in this list, not by field number. |
| `name` | String | Format: name=customers/{customer}/policySchemas/{schema_namespace} |
| `policy_api_lifecycle` | String | Output only. Current lifecycle information. |
| `supported_platforms` | Vec<String> | Output only. List indicates that the policy will only apply to devices/users on these platforms. |
| `policy_description` | String | Output only. Description about the policy schema for user consumption. |
| `valid_target_resources` | Vec<String> | Output only. Information about applicable target resources for the policy. |
| `access_restrictions` | Vec<String> | Output only. Specific access restrictions related to this policy. |
| `definition` | String | Schema definition using proto descriptor. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access policy_schema outputs
policy_schema_id = policy_schema.id
policy_schema_schema_name = policy_schema.schema_name
policy_schema_support_uri = policy_schema.support_uri
policy_schema_notices = policy_schema.notices
policy_schema_category_title = policy_schema.category_title
policy_schema_additional_target_key_names = policy_schema.additional_target_key_names
policy_schema_field_descriptions = policy_schema.field_descriptions
policy_schema_name = policy_schema.name
policy_schema_policy_api_lifecycle = policy_schema.policy_api_lifecycle
policy_schema_supported_platforms = policy_schema.supported_platforms
policy_schema_policy_description = policy_schema.policy_description
policy_schema_valid_target_resources = policy_schema.valid_target_resources
policy_schema_access_restrictions = policy_schema.access_restrictions
policy_schema_definition = policy_schema.definition
```

---


### Media

Creates an enterprise file from the content provided by user. Returns a public download url for end user.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_field` | String |  | Required. The fully qualified policy schema and field name this file is uploaded for. This information will be used to validate the content type of the file. |
| `customer` | String | ✅ | Required. The customer for which the file upload will apply. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create media
media = provider.chromepolicy_api.Media {
    customer = "value"  # Required. The customer for which the file upload will apply.
}

```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple network resources
network_0 = provider.chromepolicy_api.Network {
    customer = "value-0"
}
network_1 = provider.chromepolicy_api.Network {
    customer = "value-1"
}
network_2 = provider.chromepolicy_api.Network {
    customer = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    network = provider.chromepolicy_api.Network {
        customer = "production-value"
    }
```

---

## Related Documentation

- [GCP Chromepolicy_api Documentation](https://cloud.google.com/chromepolicy_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
