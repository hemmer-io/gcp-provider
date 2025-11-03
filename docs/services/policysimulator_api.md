# Policysimulator_api Service



**Resources**: 12

---

## Overview

The policysimulator_api service provides access to 12 resource types:

- [Replay](#replay) [CR]
- [Operation](#operation) [R]
- [Org_policy_violations_preview](#org_policy_violations_preview) [CR]
- [Org_policy_violation](#org_policy_violation) [R]
- [Result](#result) [R]
- [Operation](#operation) [R]
- [Org_policy_violations_preview](#org_policy_violations_preview) [CR]
- [Operation](#operation) [R]
- [Result](#result) [R]
- [Replay](#replay) [CR]
- [Org_policy_violation](#org_policy_violation) [R]
- [Operation](#operation) [R]

---

## Resources


### Replay

Creates and starts a Replay using the given ReplayConfig.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The resource name of the `Replay`, which has the following format: `{projects|folders|organizations}/{resource-id}/locations/global/replays/{replay-id}`, where `{resource-id}` is the ID of the project, folder, or organization that owns the Replay. Example: `projects/my-example-project/locations/global/replays/506a5f7f-38ce-4d7d-8e03-479ce1833c36` |
| `state` | String |  | Output only. The current state of the `Replay`. |
| `results_summary` | String |  | Output only. Summary statistics about the replayed log entries. |
| `config` | String |  | Required. The configuration used for the `Replay`. |
| `parent` | String | ✅ | Required. The parent resource where this Replay will be created. This resource must be a project, folder, or organization with a location. Example: `projects/my-example-project/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The resource name of the `Replay`, which has the following format: `{projects|folders|organizations}/{resource-id}/locations/global/replays/{replay-id}`, where `{resource-id}` is the ID of the project, folder, or organization that owns the Replay. Example: `projects/my-example-project/locations/global/replays/506a5f7f-38ce-4d7d-8e03-479ce1833c36` |
| `state` | String | Output only. The current state of the `Replay`. |
| `results_summary` | String | Output only. Summary statistics about the replayed log entries. |
| `config` | String | Required. The configuration used for the `Replay`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create replay
replay = provider.policysimulator_api.Replay {
    parent = "value"  # Required. The parent resource where this Replay will be created. This resource must be a project, folder, or organization with a location. Example: `projects/my-example-project/locations/global`
}

# Access replay outputs
replay_id = replay.id
replay_name = replay.name
replay_state = replay.state
replay_results_summary = replay.results_summary
replay_config = replay.config
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
operation_metadata = operation.metadata
```

---


### Org_policy_violations_preview

CreateOrgPolicyViolationsPreview creates an OrgPolicyViolationsPreview for the proposed changes in the provided OrgPolicyViolationsPreview.OrgPolicyOverlay. The changes to OrgPolicy are specified by this `OrgPolicyOverlay`. The resources to scan are inferred from these specified changes.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time when this `OrgPolicyViolationsPreview` was created. |
| `resource_counts` | String |  | Output only. A summary of the state of all resources scanned for compliance with the changed OrgPolicy. |
| `violations_count` | i64 |  | Output only. The number of OrgPolicyViolations in this `OrgPolicyViolationsPreview`. This count may differ from `resource_summary.noncompliant_count` because each OrgPolicyViolation is specific to a resource **and** constraint. If there are multiple constraints being evaluated (i.e. multiple policies in the overlay), a single resource may violate multiple constraints. |
| `state` | String |  | Output only. The state of the `OrgPolicyViolationsPreview`. |
| `custom_constraints` | Vec<String> |  | Output only. The names of the constraints against which all `OrgPolicyViolations` were evaluated. If `OrgPolicyOverlay` only contains `PolicyOverlay` then it contains the name of the configured custom constraint, applicable to the specified policies. Otherwise it contains the name of the constraint specified in `CustomConstraintOverlay`. Format: `organizations/{organization_id}/customConstraints/{custom_constraint_id}` Example: `organizations/123/customConstraints/custom.createOnlyE2TypeVms` |
| `name` | String |  | Output only. The resource name of the `OrgPolicyViolationsPreview`. It has the following format: `organizations/{organization}/locations/{location}/orgPolicyViolationsPreviews/{orgPolicyViolationsPreview}` Example: `organizations/my-example-org/locations/global/orgPolicyViolationsPreviews/506a5f7f` |
| `overlay` | String |  | Required. The proposed changes we are previewing violations for. |
| `parent` | String | ✅ | Required. The organization under which this OrgPolicyViolationsPreview will be created. Example: `organizations/my-example-org/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time when this `OrgPolicyViolationsPreview` was created. |
| `resource_counts` | String | Output only. A summary of the state of all resources scanned for compliance with the changed OrgPolicy. |
| `violations_count` | i64 | Output only. The number of OrgPolicyViolations in this `OrgPolicyViolationsPreview`. This count may differ from `resource_summary.noncompliant_count` because each OrgPolicyViolation is specific to a resource **and** constraint. If there are multiple constraints being evaluated (i.e. multiple policies in the overlay), a single resource may violate multiple constraints. |
| `state` | String | Output only. The state of the `OrgPolicyViolationsPreview`. |
| `custom_constraints` | Vec<String> | Output only. The names of the constraints against which all `OrgPolicyViolations` were evaluated. If `OrgPolicyOverlay` only contains `PolicyOverlay` then it contains the name of the configured custom constraint, applicable to the specified policies. Otherwise it contains the name of the constraint specified in `CustomConstraintOverlay`. Format: `organizations/{organization_id}/customConstraints/{custom_constraint_id}` Example: `organizations/123/customConstraints/custom.createOnlyE2TypeVms` |
| `name` | String | Output only. The resource name of the `OrgPolicyViolationsPreview`. It has the following format: `organizations/{organization}/locations/{location}/orgPolicyViolationsPreviews/{orgPolicyViolationsPreview}` Example: `organizations/my-example-org/locations/global/orgPolicyViolationsPreviews/506a5f7f` |
| `overlay` | String | Required. The proposed changes we are previewing violations for. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create org_policy_violations_preview
org_policy_violations_preview = provider.policysimulator_api.Org_policy_violations_preview {
    parent = "value"  # Required. The organization under which this OrgPolicyViolationsPreview will be created. Example: `organizations/my-example-org/locations/global`
}

# Access org_policy_violations_preview outputs
org_policy_violations_preview_id = org_policy_violations_preview.id
org_policy_violations_preview_create_time = org_policy_violations_preview.create_time
org_policy_violations_preview_resource_counts = org_policy_violations_preview.resource_counts
org_policy_violations_preview_violations_count = org_policy_violations_preview.violations_count
org_policy_violations_preview_state = org_policy_violations_preview.state
org_policy_violations_preview_custom_constraints = org_policy_violations_preview.custom_constraints
org_policy_violations_preview_name = org_policy_violations_preview.name
org_policy_violations_preview_overlay = org_policy_violations_preview.overlay
```

---


### Org_policy_violation

ListOrgPolicyViolations lists the OrgPolicyViolations that are present in an OrgPolicyViolationsPreview.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `org_policy_violations` | Vec<String> | The list of OrgPolicyViolations |
| `next_page_token` | String | A token that you can use to retrieve the next page of results. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access org_policy_violation outputs
org_policy_violation_id = org_policy_violation.id
org_policy_violation_org_policy_violations = org_policy_violation.org_policy_violations
org_policy_violation_next_page_token = org_policy_violation.next_page_token
```

---


### Result

Lists the results of running a Replay.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token that you can use to retrieve the next page of ReplayResult objects. If this field is omitted, there are no subsequent pages. |
| `replay_results` | Vec<String> | The results of running a Replay. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access result outputs
result_id = result.id
result_next_page_token = result.next_page_token
result_replay_results = result.replay_results
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_done = operation.done
operation_response = operation.response
operation_error = operation.error
operation_metadata = operation.metadata
```

---


### Org_policy_violations_preview

CreateOrgPolicyViolationsPreview creates an OrgPolicyViolationsPreview for the proposed changes in the provided OrgPolicyViolationsPreview.OrgPolicyOverlay. The changes to OrgPolicy are specified by this `OrgPolicyOverlay`. The resources to scan are inferred from these specified changes.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_counts` | String |  | Output only. A summary of the state of all resources scanned for compliance with the changed OrgPolicy. |
| `violations_count` | i64 |  | Output only. The number of OrgPolicyViolations in this `OrgPolicyViolationsPreview`. This count may differ from `resource_summary.noncompliant_count` because each OrgPolicyViolation is specific to a resource **and** constraint. If there are multiple constraints being evaluated (i.e. multiple policies in the overlay), a single resource may violate multiple constraints. |
| `custom_constraints` | Vec<String> |  | Output only. The names of the constraints against which all `OrgPolicyViolations` were evaluated. If `OrgPolicyOverlay` only contains `PolicyOverlay` then it contains the name of the configured custom constraint, applicable to the specified policies. Otherwise it contains the name of the constraint specified in `CustomConstraintOverlay`. Format: `organizations/{organization_id}/customConstraints/{custom_constraint_id}` Example: `organizations/123/customConstraints/custom.createOnlyE2TypeVms` |
| `overlay` | String |  | Required. The proposed changes we are previewing violations for. |
| `create_time` | String |  | Output only. Time when this `OrgPolicyViolationsPreview` was created. |
| `name` | String |  | Output only. The resource name of the `OrgPolicyViolationsPreview`. It has the following format: `organizations/{organization}/locations/{location}/orgPolicyViolationsPreviews/{orgPolicyViolationsPreview}` Example: `organizations/my-example-org/locations/global/orgPolicyViolationsPreviews/506a5f7f` |
| `state` | String |  | Output only. The state of the `OrgPolicyViolationsPreview`. |
| `parent` | String | ✅ | Required. The organization under which this OrgPolicyViolationsPreview will be created. Example: `organizations/my-example-org/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_counts` | String | Output only. A summary of the state of all resources scanned for compliance with the changed OrgPolicy. |
| `violations_count` | i64 | Output only. The number of OrgPolicyViolations in this `OrgPolicyViolationsPreview`. This count may differ from `resource_summary.noncompliant_count` because each OrgPolicyViolation is specific to a resource **and** constraint. If there are multiple constraints being evaluated (i.e. multiple policies in the overlay), a single resource may violate multiple constraints. |
| `custom_constraints` | Vec<String> | Output only. The names of the constraints against which all `OrgPolicyViolations` were evaluated. If `OrgPolicyOverlay` only contains `PolicyOverlay` then it contains the name of the configured custom constraint, applicable to the specified policies. Otherwise it contains the name of the constraint specified in `CustomConstraintOverlay`. Format: `organizations/{organization_id}/customConstraints/{custom_constraint_id}` Example: `organizations/123/customConstraints/custom.createOnlyE2TypeVms` |
| `overlay` | String | Required. The proposed changes we are previewing violations for. |
| `create_time` | String | Output only. Time when this `OrgPolicyViolationsPreview` was created. |
| `name` | String | Output only. The resource name of the `OrgPolicyViolationsPreview`. It has the following format: `organizations/{organization}/locations/{location}/orgPolicyViolationsPreviews/{orgPolicyViolationsPreview}` Example: `organizations/my-example-org/locations/global/orgPolicyViolationsPreviews/506a5f7f` |
| `state` | String | Output only. The state of the `OrgPolicyViolationsPreview`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create org_policy_violations_preview
org_policy_violations_preview = provider.policysimulator_api.Org_policy_violations_preview {
    parent = "value"  # Required. The organization under which this OrgPolicyViolationsPreview will be created. Example: `organizations/my-example-org/locations/global`
}

# Access org_policy_violations_preview outputs
org_policy_violations_preview_id = org_policy_violations_preview.id
org_policy_violations_preview_resource_counts = org_policy_violations_preview.resource_counts
org_policy_violations_preview_violations_count = org_policy_violations_preview.violations_count
org_policy_violations_preview_custom_constraints = org_policy_violations_preview.custom_constraints
org_policy_violations_preview_overlay = org_policy_violations_preview.overlay
org_policy_violations_preview_create_time = org_policy_violations_preview.create_time
org_policy_violations_preview_name = org_policy_violations_preview.name
org_policy_violations_preview_state = org_policy_violations_preview.state
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
operation_response = operation.response
```

---


### Result

Lists the results of running a Replay.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token that you can use to retrieve the next page of ReplayResult objects. If this field is omitted, there are no subsequent pages. |
| `replay_results` | Vec<String> | The results of running a Replay. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access result outputs
result_id = result.id
result_next_page_token = result.next_page_token
result_replay_results = result.replay_results
```

---


### Replay

Creates and starts a Replay using the given ReplayConfig.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `config` | String |  | Required. The configuration used for the `Replay`. |
| `results_summary` | String |  | Output only. Summary statistics about the replayed log entries. |
| `name` | String |  | Output only. The resource name of the `Replay`, which has the following format: `{projects|folders|organizations}/{resource-id}/locations/global/replays/{replay-id}`, where `{resource-id}` is the ID of the project, folder, or organization that owns the Replay. Example: `projects/my-example-project/locations/global/replays/506a5f7f-38ce-4d7d-8e03-479ce1833c36` |
| `state` | String |  | Output only. The current state of the `Replay`. |
| `parent` | String | ✅ | Required. The parent resource where this Replay will be created. This resource must be a project, folder, or organization with a location. Example: `projects/my-example-project/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `config` | String | Required. The configuration used for the `Replay`. |
| `results_summary` | String | Output only. Summary statistics about the replayed log entries. |
| `name` | String | Output only. The resource name of the `Replay`, which has the following format: `{projects|folders|organizations}/{resource-id}/locations/global/replays/{replay-id}`, where `{resource-id}` is the ID of the project, folder, or organization that owns the Replay. Example: `projects/my-example-project/locations/global/replays/506a5f7f-38ce-4d7d-8e03-479ce1833c36` |
| `state` | String | Output only. The current state of the `Replay`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create replay
replay = provider.policysimulator_api.Replay {
    parent = "value"  # Required. The parent resource where this Replay will be created. This resource must be a project, folder, or organization with a location. Example: `projects/my-example-project/locations/global`
}

# Access replay outputs
replay_id = replay.id
replay_config = replay.config
replay_results_summary = replay.results_summary
replay_name = replay.name
replay_state = replay.state
```

---


### Org_policy_violation

ListOrgPolicyViolations lists the OrgPolicyViolations that are present in an OrgPolicyViolationsPreview.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token that you can use to retrieve the next page of results. If this field is omitted, there are no subsequent pages. |
| `org_policy_violations` | Vec<String> | The list of OrgPolicyViolations |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access org_policy_violation outputs
org_policy_violation_id = org_policy_violation.id
org_policy_violation_next_page_token = org_policy_violation.next_page_token
org_policy_violation_org_policy_violations = org_policy_violation.org_policy_violations
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_metadata = operation.metadata
operation_done = operation.done
operation_error = operation.error
operation_name = operation.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple replay resources
replay_0 = provider.policysimulator_api.Replay {
    parent = "value-0"
}
replay_1 = provider.policysimulator_api.Replay {
    parent = "value-1"
}
replay_2 = provider.policysimulator_api.Replay {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    replay = provider.policysimulator_api.Replay {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Policysimulator_api Documentation](https://cloud.google.com/policysimulator_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
