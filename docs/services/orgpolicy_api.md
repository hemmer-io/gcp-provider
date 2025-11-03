# Orgpolicy_api Service



**Resources**: 3

---

## Overview

The orgpolicy_api service provides access to 3 resource types:

- [Constraint](#constraint) [R]
- [Policie](#policie) [CRUD]
- [Custom_constraint](#custom_constraint) [CRUD]

---

## Resources


### Constraint

Lists constraints that could be applied on the specified resource.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Page token used to retrieve the next page. This is currently not used. |
| `constraints` | Vec<String> | The collection of constraints that are available on the targeted resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access constraint outputs
constraint_id = constraint.id
constraint_next_page_token = constraint.next_page_token
constraint_constraints = constraint.constraints
```

---


### Policie

Creates a policy. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the constraint does not exist. Returns a `google.rpc.Status` with `google.rpc.Code.ALREADY_EXISTS` if the policy already exists on the given Google Cloud resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. The resource name of the policy. Must be one of the following forms, where `constraint_name` is the name of the constraint which this policy configures: * `projects/{project_number}/policies/{constraint_name}` * `folders/{folder_id}/policies/{constraint_name}` * `organizations/{organization_id}/policies/{constraint_name}` For example, `projects/123/policies/compute.disableSerialPortAccess`. Note: `projects/{project_id}/policies/{constraint_name}` is also an acceptable name for API requests, but responses will return the name using the equivalent project number. |
| `spec` | String |  | Basic information about the organization policy. |
| `dry_run_spec` | String |  | Dry-run policy. Audit-only policy, can be used to monitor how the policy would have impacted the existing and future resources if it's enforced. |
| `etag` | String |  | Optional. An opaque tag indicating the current state of the policy, used for concurrency control. This 'etag' is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `alternate` | String |  | Deprecated. |
| `parent` | String | ✅ | Required. The Google Cloud resource that will parent the new policy. Must be in one of the following forms: * `projects/{project_number}` * `projects/{project_id}` * `folders/{folder_id}` * `organizations/{organization_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. The resource name of the policy. Must be one of the following forms, where `constraint_name` is the name of the constraint which this policy configures: * `projects/{project_number}/policies/{constraint_name}` * `folders/{folder_id}/policies/{constraint_name}` * `organizations/{organization_id}/policies/{constraint_name}` For example, `projects/123/policies/compute.disableSerialPortAccess`. Note: `projects/{project_id}/policies/{constraint_name}` is also an acceptable name for API requests, but responses will return the name using the equivalent project number. |
| `spec` | String | Basic information about the organization policy. |
| `dry_run_spec` | String | Dry-run policy. Audit-only policy, can be used to monitor how the policy would have impacted the existing and future resources if it's enforced. |
| `etag` | String | Optional. An opaque tag indicating the current state of the policy, used for concurrency control. This 'etag' is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `alternate` | String | Deprecated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policie
policie = provider.orgpolicy_api.Policie {
    parent = "value"  # Required. The Google Cloud resource that will parent the new policy. Must be in one of the following forms: * `projects/{project_number}` * `projects/{project_id}` * `folders/{folder_id}` * `organizations/{organization_id}`
}

# Access policie outputs
policie_id = policie.id
policie_name = policie.name
policie_spec = policie.spec
policie_dry_run_spec = policie.dry_run_spec
policie_etag = policie.etag
policie_alternate = policie.alternate
```

---


### Custom_constraint

Creates a custom constraint. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the organization does not exist. Returns a `google.rpc.Status` with `google.rpc.Code.ALREADY_EXISTS` if the constraint already exists on the given organization.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action_type` | String |  | Allow or deny type. |
| `display_name` | String |  | One line display name for the UI. The max length of the display_name is 200 characters. |
| `condition` | String |  | A Common Expression Language (CEL) condition which is used in the evaluation of the constraint. For example: `resource.instanceName.matches("[production|test]_.*_(\d)+")` or, `resource.management.auto_upgrade == true` The max length of the condition is 1000 characters. |
| `name` | String |  | Immutable. Name of the constraint. This is unique within the organization. Format of the name should be * `organizations/{organization_id}/customConstraints/{custom_constraint_id}` Example: `organizations/123/customConstraints/custom.createOnlyE2TypeVms` The max length is 70 characters and the minimum length is 1. Note that the prefix `organizations/{organization_id}/customConstraints/` is not counted. |
| `resource_types` | Vec<String> |  | Immutable. The resource instance type on which this policy applies. Format will be of the form : `/` Example: * `compute.googleapis.com/Instance`. |
| `method_types` | Vec<String> |  | All the operations being applied for this constraint. |
| `description` | String |  | Detailed information about this custom policy constraint. The max length of the description is 2000 characters. |
| `update_time` | String |  | Output only. The last time this custom constraint was updated. This represents the last time that the `CreateCustomConstraint` or `UpdateCustomConstraint` methods were called. |
| `parent` | String | ✅ | Required. Must be in the following form: * `organizations/{organization_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `action_type` | String | Allow or deny type. |
| `display_name` | String | One line display name for the UI. The max length of the display_name is 200 characters. |
| `condition` | String | A Common Expression Language (CEL) condition which is used in the evaluation of the constraint. For example: `resource.instanceName.matches("[production|test]_.*_(\d)+")` or, `resource.management.auto_upgrade == true` The max length of the condition is 1000 characters. |
| `name` | String | Immutable. Name of the constraint. This is unique within the organization. Format of the name should be * `organizations/{organization_id}/customConstraints/{custom_constraint_id}` Example: `organizations/123/customConstraints/custom.createOnlyE2TypeVms` The max length is 70 characters and the minimum length is 1. Note that the prefix `organizations/{organization_id}/customConstraints/` is not counted. |
| `resource_types` | Vec<String> | Immutable. The resource instance type on which this policy applies. Format will be of the form : `/` Example: * `compute.googleapis.com/Instance`. |
| `method_types` | Vec<String> | All the operations being applied for this constraint. |
| `description` | String | Detailed information about this custom policy constraint. The max length of the description is 2000 characters. |
| `update_time` | String | Output only. The last time this custom constraint was updated. This represents the last time that the `CreateCustomConstraint` or `UpdateCustomConstraint` methods were called. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_constraint
custom_constraint = provider.orgpolicy_api.Custom_constraint {
    parent = "value"  # Required. Must be in the following form: * `organizations/{organization_id}`
}

# Access custom_constraint outputs
custom_constraint_id = custom_constraint.id
custom_constraint_action_type = custom_constraint.action_type
custom_constraint_display_name = custom_constraint.display_name
custom_constraint_condition = custom_constraint.condition
custom_constraint_name = custom_constraint.name
custom_constraint_resource_types = custom_constraint.resource_types
custom_constraint_method_types = custom_constraint.method_types
custom_constraint_description = custom_constraint.description
custom_constraint_update_time = custom_constraint.update_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple constraint resources
constraint_0 = provider.orgpolicy_api.Constraint {
}
constraint_1 = provider.orgpolicy_api.Constraint {
}
constraint_2 = provider.orgpolicy_api.Constraint {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    constraint = provider.orgpolicy_api.Constraint {
    }
```

---

## Related Documentation

- [GCP Orgpolicy_api Documentation](https://cloud.google.com/orgpolicy_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
