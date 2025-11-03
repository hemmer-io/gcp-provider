# Securityposture_api Service



**Resources**: 6

---

## Overview

The securityposture_api service provides access to 6 resource types:

- [Posture_deployment](#posture_deployment) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Posture](#posture) [CRUD]
- [Posture_template](#posture_template) [R]
- [Report](#report) [CR]

---

## Resources


### Posture_deployment

Creates a new PostureDeployment in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Optional. An opaque identifier for the current version of the posture deployment. To prevent concurrent updates from overwriting each other, always provide the `etag` when you update a posture deployment. You can also provide the `etag` when you delete a posture deployment, to help ensure that you're deleting the intended posture deployment. |
| `annotations` | HashMap<String, String> |  | Optional. The user-specified annotations for the posture deployment. For details about the values you can use in an annotation, see [AIP-148: Standard fields](https://google.aip.dev/148#annotations). |
| `reconciling` | bool |  | Output only. Whether the posture deployment is in the process of being updated. |
| `target_resource` | String |  | Required. The organization, folder, or project where the posture is deployed. Uses one of the following formats: * `organizations/{organization_number}` * `folders/{folder_number}` * `projects/{project_number}` |
| `categories` | Vec<String> |  | Output only. The categories that the posture deployment belongs to, as determined by the Security Posture API. |
| `name` | String |  | Required. Identifier. The name of the posture deployment, in the format `organizations/{organization}/locations/global/postureDeployments/{deployment_id}`. |
| `description` | String |  | Optional. A description of the posture deployment. |
| `posture_id` | String |  | Required. The posture used in the deployment, in the format `organizations/{organization}/locations/global/postures/{posture_id}`. |
| `state` | String |  | Output only. The state of the posture deployment. |
| `update_time` | String |  | Output only. The time at which the posture deployment was last updated. |
| `desired_posture_revision_id` | String |  | Output only. The revision ID of the posture that was specified for the deployment. Present only if the deployment is in a failed state. |
| `posture_revision_id` | String |  | Required. The revision ID of the posture used in the deployment. |
| `create_time` | String |  | Output only. The time at which the posture deployment was created. |
| `desired_posture_id` | String |  | Output only. The posture ID that was specified for the deployment. Present only if the posture deployment is in a failed state. |
| `failure_message` | String |  | Output only. A description of why the posture deployment failed. Present only if the deployment is in a failed state. |
| `parent` | String | ✅ | Required. The parent resource name, in the format `organizations/{organization}/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Optional. An opaque identifier for the current version of the posture deployment. To prevent concurrent updates from overwriting each other, always provide the `etag` when you update a posture deployment. You can also provide the `etag` when you delete a posture deployment, to help ensure that you're deleting the intended posture deployment. |
| `annotations` | HashMap<String, String> | Optional. The user-specified annotations for the posture deployment. For details about the values you can use in an annotation, see [AIP-148: Standard fields](https://google.aip.dev/148#annotations). |
| `reconciling` | bool | Output only. Whether the posture deployment is in the process of being updated. |
| `target_resource` | String | Required. The organization, folder, or project where the posture is deployed. Uses one of the following formats: * `organizations/{organization_number}` * `folders/{folder_number}` * `projects/{project_number}` |
| `categories` | Vec<String> | Output only. The categories that the posture deployment belongs to, as determined by the Security Posture API. |
| `name` | String | Required. Identifier. The name of the posture deployment, in the format `organizations/{organization}/locations/global/postureDeployments/{deployment_id}`. |
| `description` | String | Optional. A description of the posture deployment. |
| `posture_id` | String | Required. The posture used in the deployment, in the format `organizations/{organization}/locations/global/postures/{posture_id}`. |
| `state` | String | Output only. The state of the posture deployment. |
| `update_time` | String | Output only. The time at which the posture deployment was last updated. |
| `desired_posture_revision_id` | String | Output only. The revision ID of the posture that was specified for the deployment. Present only if the deployment is in a failed state. |
| `posture_revision_id` | String | Required. The revision ID of the posture used in the deployment. |
| `create_time` | String | Output only. The time at which the posture deployment was created. |
| `desired_posture_id` | String | Output only. The posture ID that was specified for the deployment. Present only if the posture deployment is in a failed state. |
| `failure_message` | String | Output only. A description of why the posture deployment failed. Present only if the deployment is in a failed state. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create posture_deployment
posture_deployment = provider.securityposture_api.Posture_deployment {
    parent = "value"  # Required. The parent resource name, in the format `organizations/{organization}/locations/global`.
}

# Access posture_deployment outputs
posture_deployment_id = posture_deployment.id
posture_deployment_etag = posture_deployment.etag
posture_deployment_annotations = posture_deployment.annotations
posture_deployment_reconciling = posture_deployment.reconciling
posture_deployment_target_resource = posture_deployment.target_resource
posture_deployment_categories = posture_deployment.categories
posture_deployment_name = posture_deployment.name
posture_deployment_description = posture_deployment.description
posture_deployment_posture_id = posture_deployment.posture_id
posture_deployment_state = posture_deployment.state
posture_deployment_update_time = posture_deployment.update_time
posture_deployment_desired_posture_revision_id = posture_deployment.desired_posture_revision_id
posture_deployment_posture_revision_id = posture_deployment.posture_revision_id
posture_deployment_create_time = posture_deployment.create_time
posture_deployment_desired_posture_id = posture_deployment.desired_posture_id
posture_deployment_failure_message = posture_deployment.failure_message
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of `1`, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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

# Create operation
operation = provider.securityposture_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
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


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access location outputs
location_id = location.id
location_labels = location.labels
location_name = location.name
location_metadata = location.metadata
location_display_name = location.display_name
location_location_id = location.location_id
```

---


### Posture

Creates a new Posture.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The time at which the posture was last updated. |
| `description` | String |  | Optional. A description of the posture. |
| `name` | String |  | Required. Identifier. The name of the posture, in the format `organizations/{organization}/locations/global/postures/{posture_id}`. |
| `annotations` | HashMap<String, String> |  | Optional. The user-specified annotations for the posture. For details about the values you can use in an annotation, see [AIP-148: Standard fields](https://google.aip.dev/148#annotations). |
| `etag` | String |  | Optional. An opaque identifier for the current version of the posture at the specified `revision_id`. To prevent concurrent updates from overwriting each other, always provide the `etag` when you update a posture. You can also provide the `etag` when you delete a posture, to help ensure that you're deleting the intended version of the posture. |
| `create_time` | String |  | Output only. The time at which the posture was created. |
| `policy_sets` | Vec<String> |  | Required. The PolicySet resources that the posture includes. |
| `categories` | Vec<String> |  | Output only. The categories that the posture belongs to, as determined by the Security Posture API. |
| `reconciling` | bool |  | Output only. Whether the posture is in the process of being updated. |
| `revision_id` | String |  | Output only. Immutable. An opaque eight-character string that identifies the revision of the posture. A posture can have multiple revisions; when you deploy a posture, you deploy a specific revision of the posture. |
| `state` | String |  | Required. The state of the posture at the specified `revision_id`. |
| `parent` | String | ✅ | Required. The parent resource name, in the format `organizations/{organization}/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The time at which the posture was last updated. |
| `description` | String | Optional. A description of the posture. |
| `name` | String | Required. Identifier. The name of the posture, in the format `organizations/{organization}/locations/global/postures/{posture_id}`. |
| `annotations` | HashMap<String, String> | Optional. The user-specified annotations for the posture. For details about the values you can use in an annotation, see [AIP-148: Standard fields](https://google.aip.dev/148#annotations). |
| `etag` | String | Optional. An opaque identifier for the current version of the posture at the specified `revision_id`. To prevent concurrent updates from overwriting each other, always provide the `etag` when you update a posture. You can also provide the `etag` when you delete a posture, to help ensure that you're deleting the intended version of the posture. |
| `create_time` | String | Output only. The time at which the posture was created. |
| `policy_sets` | Vec<String> | Required. The PolicySet resources that the posture includes. |
| `categories` | Vec<String> | Output only. The categories that the posture belongs to, as determined by the Security Posture API. |
| `reconciling` | bool | Output only. Whether the posture is in the process of being updated. |
| `revision_id` | String | Output only. Immutable. An opaque eight-character string that identifies the revision of the posture. A posture can have multiple revisions; when you deploy a posture, you deploy a specific revision of the posture. |
| `state` | String | Required. The state of the posture at the specified `revision_id`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create posture
posture = provider.securityposture_api.Posture {
    parent = "value"  # Required. The parent resource name, in the format `organizations/{organization}/locations/global`.
}

# Access posture outputs
posture_id = posture.id
posture_update_time = posture.update_time
posture_description = posture.description
posture_name = posture.name
posture_annotations = posture.annotations
posture_etag = posture.etag
posture_create_time = posture.create_time
posture_policy_sets = posture.policy_sets
posture_categories = posture.categories
posture_reconciling = posture.reconciling
posture_revision_id = posture.revision_id
posture_state = posture.state
```

---


### Posture_template

Gets a single revision of a PostureTemplate.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `categories` | Vec<String> | Output only. The categories that the posture template belongs to, as determined by the Security Posture API. |
| `name` | String | Output only. Identifier. The name of the posture template, in the format `organizations/{organization}/locations/global/postureTemplates/{posture_template}`. |
| `policy_sets` | Vec<String> | Output only. The PolicySet resources that the posture template includes. |
| `revision_id` | String | Output only. A string that identifies the revision of the posture template. |
| `state` | String | Output only. The state of the posture template at the specified `revision_id`. |
| `description` | String | Output only. A description of the posture template. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access posture_template outputs
posture_template_id = posture_template.id
posture_template_categories = posture_template.categories
posture_template_name = posture_template.name
posture_template_policy_sets = posture_template.policy_sets
posture_template_revision_id = posture_template.revision_id
posture_template_state = posture_template.state
posture_template_description = posture_template.description
```

---


### Report

Validates a specified infrastructure-as-code (IaC) configuration, and creates a Report with the validation results. Only Terraform configurations are supported. Only modified assets are validated.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `iac` | String |  | Required. The infrastructure-as-code (IaC) configuration to validate. |
| `parent` | String | ✅ | Required. The parent resource name, in the format `organizations/{organization}/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The time at which the report was last updated. |
| `iac_validation_report` | String | Output only. An infrastructure-as-code (IaC) validation report. |
| `name` | String | Required. The name of the report, in the format `organizations/{organization}/locations/global/reports/{report_id}`. |
| `create_time` | String | Output only. The time at which the report was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create report
report = provider.securityposture_api.Report {
    parent = "value"  # Required. The parent resource name, in the format `organizations/{organization}/locations/global`.
}

# Access report outputs
report_id = report.id
report_update_time = report.update_time
report_iac_validation_report = report.iac_validation_report
report_name = report.name
report_create_time = report.create_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple posture_deployment resources
posture_deployment_0 = provider.securityposture_api.Posture_deployment {
    parent = "value-0"
}
posture_deployment_1 = provider.securityposture_api.Posture_deployment {
    parent = "value-1"
}
posture_deployment_2 = provider.securityposture_api.Posture_deployment {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    posture_deployment = provider.securityposture_api.Posture_deployment {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Securityposture_api Documentation](https://cloud.google.com/securityposture_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
