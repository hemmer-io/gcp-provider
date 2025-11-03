# Iap_api Service



**Resources**: 5

---

## Overview

The iap_api service provides access to 5 resource types:

- [Identity_aware_proxy_client](#identity_aware_proxy_client) [CRD]
- [Brand](#brand) [CR]
- [Dest_group](#dest_group) [CRUD]
- [Iap](#iap) [CRU]
- [Iap](#iap) [C]

---

## Resources


### Identity_aware_proxy_client

Creates an Identity Aware Proxy (IAP) OAuth client. The client is owned by IAP. Requires that the brand for the project exists and that it is set for internal-only use.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Unique identifier of the OAuth client. |
| `display_name` | String |  | Human-friendly name given to the OAuth client. |
| `secret` | String |  | Output only. Client secret of the OAuth client. |
| `parent` | String | ✅ | Required. Path to create the client in. In the following format: projects/{project_number/id}/brands/{brand}. The project must belong to a G Suite account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Unique identifier of the OAuth client. |
| `display_name` | String | Human-friendly name given to the OAuth client. |
| `secret` | String | Output only. Client secret of the OAuth client. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create identity_aware_proxy_client
identity_aware_proxy_client = provider.iap_api.Identity_aware_proxy_client {
    parent = "value"  # Required. Path to create the client in. In the following format: projects/{project_number/id}/brands/{brand}. The project must belong to a G Suite account.
}

# Access identity_aware_proxy_client outputs
identity_aware_proxy_client_id = identity_aware_proxy_client.id
identity_aware_proxy_client_name = identity_aware_proxy_client.name
identity_aware_proxy_client_display_name = identity_aware_proxy_client.display_name
identity_aware_proxy_client_secret = identity_aware_proxy_client.secret
```

---


### Brand

Constructs a new OAuth brand for the project if one does not exist. The created brand is "internal only", meaning that OAuth clients created under it only accept requests from users who belong to the same Google Workspace organization as the project. The brand is created in an un-reviewed status. NOTE: The "internal only" status can be manually changed in the Google Cloud Console. Requires that a brand does not already exist for the project, and that the specified support email is owned by the caller.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `support_email` | String |  | Support email displayed on the OAuth consent screen. |
| `name` | String |  | Output only. Identifier of the brand. NOTE: GCP project number achieves the same brand identification purpose as only one brand per project can be created. |
| `application_title` | String |  | Application name displayed on OAuth consent screen. |
| `org_internal_only` | bool |  | Output only. Whether the brand is only intended for usage inside the G Suite organization only. |
| `parent` | String | ✅ | Required. GCP Project number/id under which the brand is to be created. In the following format: projects/{project_number/id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `support_email` | String | Support email displayed on the OAuth consent screen. |
| `name` | String | Output only. Identifier of the brand. NOTE: GCP project number achieves the same brand identification purpose as only one brand per project can be created. |
| `application_title` | String | Application name displayed on OAuth consent screen. |
| `org_internal_only` | bool | Output only. Whether the brand is only intended for usage inside the G Suite organization only. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create brand
brand = provider.iap_api.Brand {
    parent = "value"  # Required. GCP Project number/id under which the brand is to be created. In the following format: projects/{project_number/id}.
}

# Access brand outputs
brand_id = brand.id
brand_support_email = brand.support_email
brand_name = brand.name
brand_application_title = brand.application_title
brand_org_internal_only = brand.org_internal_only
```

---


### Dest_group

Creates a new TunnelDestGroup.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cidrs` | Vec<String> |  | Optional. Unordered list. List of CIDRs that this group applies to. |
| `fqdns` | Vec<String> |  | Optional. Unordered list. List of FQDNs that this group applies to. |
| `name` | String |  | Identifier. Identifier for the TunnelDestGroup. Must be unique within the project and contain only lower case letters (a-z) and dashes (-). |
| `parent` | String | ✅ | Required. Google Cloud Project ID and location. In the following format: `projects/{project_number/id}/iap_tunnel/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cidrs` | Vec<String> | Optional. Unordered list. List of CIDRs that this group applies to. |
| `fqdns` | Vec<String> | Optional. Unordered list. List of FQDNs that this group applies to. |
| `name` | String | Identifier. Identifier for the TunnelDestGroup. Must be unique within the project and contain only lower case letters (a-z) and dashes (-). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dest_group
dest_group = provider.iap_api.Dest_group {
    parent = "value"  # Required. Google Cloud Project ID and location. In the following format: `projects/{project_number/id}/iap_tunnel/locations/{location}`.
}

# Access dest_group outputs
dest_group_id = dest_group.id
dest_group_cidrs = dest_group.cidrs
dest_group_fqdns = dest_group.fqdns
dest_group_name = dest_group.name
```

---


### Iap

Gets the access control policy for an Identity-Aware Proxy protected resource. More information about managing access via IAP can be found at: https://cloud.google.com/iap/docs/managing-access#managing_access_via_the_api

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `options` | String |  | OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application_settings` | String | Optional. Top level wrapper for all application related settings in IAP |
| `access_settings` | String | Optional. Top level wrapper for all access related setting in IAP |
| `name` | String | Required. The resource name of the IAP protected resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create iap
iap = provider.iap_api.Iap {
    resource = "value"  # REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access iap outputs
iap_id = iap.id
iap_application_settings = iap.application_settings
iap_access_settings = iap.access_settings
iap_name = iap.name
```

---


### Iap

Gets the access control policy for an Identity-Aware Proxy protected resource. More information about managing access via IAP can be found at: https://cloud.google.com/iap/docs/managing-access#managing_access_via_the_api

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `options` | String |  | OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create iap
iap = provider.iap_api.Iap {
    resource = "value"  # REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
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

# Create multiple identity_aware_proxy_client resources
identity_aware_proxy_client_0 = provider.iap_api.Identity_aware_proxy_client {
    parent = "value-0"
}
identity_aware_proxy_client_1 = provider.iap_api.Identity_aware_proxy_client {
    parent = "value-1"
}
identity_aware_proxy_client_2 = provider.iap_api.Identity_aware_proxy_client {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    identity_aware_proxy_client = provider.iap_api.Identity_aware_proxy_client {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Iap_api Documentation](https://cloud.google.com/iap_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
