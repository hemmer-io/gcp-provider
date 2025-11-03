# Marketingplatformadmin_api Service



**Resources**: 2

---

## Overview

The marketingplatformadmin_api service provides access to 2 resource types:

- [Organization](#organization) [CR]
- [Analytics_account_link](#analytics_account_link) [CRD]

---

## Resources


### Organization

Get the usage and billing data for properties within the organization for the specified month. Per direct client org, user needs to be OrgAdmin/BillingAdmin on the organization in order to view the billing and usage data. Per sales partner client org, user needs to be OrgAdmin/BillingAdmin on the sales partner org in order to view the billing and usage data, or OrgAdmin/BillingAdmin on the sales partner client org in order to view the usage data only.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `month` | String |  | Required. The target month to list property usages. Format: YYYY-MM. For example, "2025-05" |
| `organization` | String | ✅ | Required. Specifies the organization whose property usage will be listed. Format: organizations/{org_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The human-readable name for the organization. |
| `name` | String | Identifier. The resource name of the GMP organization. Format: organizations/{org_id} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create organization
organization = provider.marketingplatformadmin_api.Organization {
    organization = "value"  # Required. Specifies the organization whose property usage will be listed. Format: organizations/{org_id}
}

# Access organization outputs
organization_id = organization.id
organization_display_name = organization.display_name
organization_name = organization.name
```

---


### Analytics_account_link

Creates the link between the Analytics account and the Google Marketing Platform organization. User needs to be an org user, and admin on the Analytics account to create the link. If the account is already linked to an organization, user needs to unlink the account from the current organization, then try link again.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `analytics_account` | String |  | Required. Immutable. The resource name of the AnalyticsAdmin API account. The account ID will be used as the ID of this AnalyticsAccountLink resource, which will become the final component of the resource name. Format: analyticsadmin.googleapis.com/accounts/{account_id} |
| `link_verification_state` | String |  | Output only. The verification state of the link between the Analytics account and the parent organization. |
| `name` | String |  | Identifier. Resource name of this AnalyticsAccountLink. Note the resource ID is the same as the ID of the Analtyics account. Format: organizations/{org_id}/analyticsAccountLinks/{analytics_account_link_id} Example: "organizations/xyz/analyticsAccountLinks/1234" |
| `display_name` | String |  | Output only. The human-readable name for the Analytics account. |
| `parent` | String | ✅ | Required. The parent resource where this Analytics account link will be created. Format: organizations/{org_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `analytics_account_links` | Vec<String> | Analytics account links in this organization. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create analytics_account_link
analytics_account_link = provider.marketingplatformadmin_api.Analytics_account_link {
    parent = "value"  # Required. The parent resource where this Analytics account link will be created. Format: organizations/{org_id}
}

# Access analytics_account_link outputs
analytics_account_link_id = analytics_account_link.id
analytics_account_link_analytics_account_links = analytics_account_link.analytics_account_links
analytics_account_link_next_page_token = analytics_account_link.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple organization resources
organization_0 = provider.marketingplatformadmin_api.Organization {
    organization = "value-0"
}
organization_1 = provider.marketingplatformadmin_api.Organization {
    organization = "value-1"
}
organization_2 = provider.marketingplatformadmin_api.Organization {
    organization = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    organization = provider.marketingplatformadmin_api.Organization {
        organization = "production-value"
    }
```

---

## Related Documentation

- [GCP Marketingplatformadmin_api Documentation](https://cloud.google.com/marketingplatformadmin_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
