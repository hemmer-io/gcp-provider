# Firebasedatabase_api Service



**Resources**: 1

---

## Overview

The firebasedatabase_api service provides access to 1 resource type:

- [Instance](#instance) [CRD]

---

## Resources


### Instance

Requests that a new DatabaseInstance be created. The state of a successfully created DatabaseInstance is ACTIVE. Only available for projects on the Blaze plan. Projects can be upgraded using the Cloud Billing API https://cloud.google.com/billing/reference/rest/v1/projects/updateBillingInfo. Note that it might take a few minutes for billing enablement state to propagate to Firebase systems.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | Immutable. The database instance type. On creation only USER_DATABASE is allowed, which is also the default when omitted. |
| `database_url` | String |  | Output only. Output Only. The globally unique hostname of the database. |
| `project` | String |  | Output only. The resource name of the project this instance belongs to. For example: `projects/{project-number}`. |
| `name` | String |  | The fully qualified resource name of the database instance, in the form: `projects/{project-number}/locations/{location-id}/instances/{database-id}`. |
| `state` | String |  | Output only. The database's lifecycle state. Read-only. |
| `parent` | String | ✅ | Required. The parent project for which to create a database instance, in the form: `projects/{project-number}/locations/{location-id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | Immutable. The database instance type. On creation only USER_DATABASE is allowed, which is also the default when omitted. |
| `database_url` | String | Output only. Output Only. The globally unique hostname of the database. |
| `project` | String | Output only. The resource name of the project this instance belongs to. For example: `projects/{project-number}`. |
| `name` | String | The fully qualified resource name of the database instance, in the form: `projects/{project-number}/locations/{location-id}/instances/{database-id}`. |
| `state` | String | Output only. The database's lifecycle state. Read-only. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.firebasedatabase_api.Instance {
    parent = "value"  # Required. The parent project for which to create a database instance, in the form: `projects/{project-number}/locations/{location-id}`.
}

# Access instance outputs
instance_id = instance.id
instance_type = instance.type
instance_database_url = instance.database_url
instance_project = instance.project
instance_name = instance.name
instance_state = instance.state
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple instance resources
instance_0 = provider.firebasedatabase_api.Instance {
    parent = "value-0"
}
instance_1 = provider.firebasedatabase_api.Instance {
    parent = "value-1"
}
instance_2 = provider.firebasedatabase_api.Instance {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    instance = provider.firebasedatabase_api.Instance {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Firebasedatabase_api Documentation](https://cloud.google.com/firebasedatabase_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
