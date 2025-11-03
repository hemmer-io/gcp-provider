# Config_api Service



**Resources**: 9

---

## Overview

The config_api service provides access to 9 resource types:

- [Preview](#preview) [CRD]
- [Deployment](#deployment) [CRUD]
- [Resource_change](#resource_change) [R]
- [Operation](#operation) [CRD]
- [Resource_drift](#resource_drift) [R]
- [Location](#location) [R]
- [Resource](#resource) [R]
- [Revision](#revision) [CR]
- [Terraform_version](#terraform_version) [R]

---

## Resources


### Preview

Creates a Preview.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `error_logs` | String |  | Output only. Link to tf-error.ndjson file, which contains the full list of the errors encountered during a Terraform preview. Format: `gs://{bucket}/{object}`. |
| `preview_mode` | String |  | Optional. Current mode of preview. |
| `tf_errors` | Vec<String> |  | Output only. Summary of errors encountered during Terraform preview. It has a size limit of 10, i.e. only top 10 errors will be summarized here. |
| `tf_version_constraint` | String |  | Optional. The user-specified Terraform version constraint. Example: "=1.3.10". |
| `error_status` | String |  | Output only. Additional information regarding the current state. |
| `provider_config` | String |  | Optional. This field specifies the provider configurations. |
| `labels` | HashMap<String, String> |  | Optional. User-defined labels for the preview. |
| `create_time` | String |  | Output only. Time the preview was created. |
| `deployment` | String |  | Optional. Optional deployment reference. If specified, the preview will be performed using the provided deployment's current state and use any relevant fields from the deployment unless explicitly specified in the preview create request. |
| `annotations` | HashMap<String, String> |  | Optional. Arbitrary key-value metadata storage e.g. to help client tools identify preview during automation. See https://google.aip.dev/148#annotations for details on format and size limitations. |
| `artifacts_gcs_bucket` | String |  | Optional. User-defined location of Cloud Build logs, artifacts, and in Google Cloud Storage. Format: `gs://{bucket}/{folder}` A default bucket will be bootstrapped if the field is not set or empty Default Bucket Format: `gs://--blueprint-config` Constraints: - The bucket needs to be in the same project as the deployment - The path cannot be within the path of `gcs_source` If omitted and deployment resource ref provided has artifacts_gcs_bucket defined, that artifact bucket is used. |
| `logs` | String |  | Output only. Location of preview logs in `gs://{bucket}/{object}` format. |
| `preview_artifacts` | String |  | Output only. Artifacts from preview. |
| `terraform_blueprint` | String |  | The terraform blueprint to preview. |
| `name` | String |  | Identifier. Resource name of the preview. Resource name can be user provided or server generated ID if unspecified. Format: `projects/{project}/locations/{location}/previews/{preview}` |
| `build` | String |  | Output only. Cloud Build instance UUID associated with this preview. |
| `error_code` | String |  | Output only. Code describing any errors that may have occurred. |
| `service_account` | String |  | Required. User-specified Service Account (SA) credentials to be used when previewing resources. Format: `projects/{projectID}/serviceAccounts/{serviceAccount}` |
| `state` | String |  | Output only. Current state of the preview. |
| `worker_pool` | String |  | Optional. The user-specified Worker Pool resource in which the Cloud Build job will execute. Format projects/{project}/locations/{location}/workerPools/{workerPoolId} If this field is unspecified, the default Cloud Build worker pool will be used. If omitted and deployment resource ref provided has worker_pool defined, that worker pool is used. |
| `tf_version` | String |  | Output only. The current Terraform version set on the preview. It is in the format of "Major.Minor.Patch", for example, "1.3.10". |
| `parent` | String | ✅ | Required. The parent in whose context the Preview is created. The parent value is in the format: 'projects/{project_id}/locations/{location}'. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error_logs` | String | Output only. Link to tf-error.ndjson file, which contains the full list of the errors encountered during a Terraform preview. Format: `gs://{bucket}/{object}`. |
| `preview_mode` | String | Optional. Current mode of preview. |
| `tf_errors` | Vec<String> | Output only. Summary of errors encountered during Terraform preview. It has a size limit of 10, i.e. only top 10 errors will be summarized here. |
| `tf_version_constraint` | String | Optional. The user-specified Terraform version constraint. Example: "=1.3.10". |
| `error_status` | String | Output only. Additional information regarding the current state. |
| `provider_config` | String | Optional. This field specifies the provider configurations. |
| `labels` | HashMap<String, String> | Optional. User-defined labels for the preview. |
| `create_time` | String | Output only. Time the preview was created. |
| `deployment` | String | Optional. Optional deployment reference. If specified, the preview will be performed using the provided deployment's current state and use any relevant fields from the deployment unless explicitly specified in the preview create request. |
| `annotations` | HashMap<String, String> | Optional. Arbitrary key-value metadata storage e.g. to help client tools identify preview during automation. See https://google.aip.dev/148#annotations for details on format and size limitations. |
| `artifacts_gcs_bucket` | String | Optional. User-defined location of Cloud Build logs, artifacts, and in Google Cloud Storage. Format: `gs://{bucket}/{folder}` A default bucket will be bootstrapped if the field is not set or empty Default Bucket Format: `gs://--blueprint-config` Constraints: - The bucket needs to be in the same project as the deployment - The path cannot be within the path of `gcs_source` If omitted and deployment resource ref provided has artifacts_gcs_bucket defined, that artifact bucket is used. |
| `logs` | String | Output only. Location of preview logs in `gs://{bucket}/{object}` format. |
| `preview_artifacts` | String | Output only. Artifacts from preview. |
| `terraform_blueprint` | String | The terraform blueprint to preview. |
| `name` | String | Identifier. Resource name of the preview. Resource name can be user provided or server generated ID if unspecified. Format: `projects/{project}/locations/{location}/previews/{preview}` |
| `build` | String | Output only. Cloud Build instance UUID associated with this preview. |
| `error_code` | String | Output only. Code describing any errors that may have occurred. |
| `service_account` | String | Required. User-specified Service Account (SA) credentials to be used when previewing resources. Format: `projects/{projectID}/serviceAccounts/{serviceAccount}` |
| `state` | String | Output only. Current state of the preview. |
| `worker_pool` | String | Optional. The user-specified Worker Pool resource in which the Cloud Build job will execute. Format projects/{project}/locations/{location}/workerPools/{workerPoolId} If this field is unspecified, the default Cloud Build worker pool will be used. If omitted and deployment resource ref provided has worker_pool defined, that worker pool is used. |
| `tf_version` | String | Output only. The current Terraform version set on the preview. It is in the format of "Major.Minor.Patch", for example, "1.3.10". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create preview
preview = provider.config_api.Preview {
    parent = "value"  # Required. The parent in whose context the Preview is created. The parent value is in the format: 'projects/{project_id}/locations/{location}'.
}

# Access preview outputs
preview_id = preview.id
preview_error_logs = preview.error_logs
preview_preview_mode = preview.preview_mode
preview_tf_errors = preview.tf_errors
preview_tf_version_constraint = preview.tf_version_constraint
preview_error_status = preview.error_status
preview_provider_config = preview.provider_config
preview_labels = preview.labels
preview_create_time = preview.create_time
preview_deployment = preview.deployment
preview_annotations = preview.annotations
preview_artifacts_gcs_bucket = preview.artifacts_gcs_bucket
preview_logs = preview.logs
preview_preview_artifacts = preview.preview_artifacts
preview_terraform_blueprint = preview.terraform_blueprint
preview_name = preview.name
preview_build = preview.build
preview_error_code = preview.error_code
preview_service_account = preview.service_account
preview_state = preview.state
preview_worker_pool = preview.worker_pool
preview_tf_version = preview.tf_version
```

---


### Deployment

Creates a Deployment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time when the deployment was created. |
| `terraform_blueprint` | String |  | A blueprint described using Terraform's HashiCorp Configuration Language as a root module. |
| `annotations` | HashMap<String, String> |  | Optional. Arbitrary key-value metadata storage e.g. to help client tools identify deployments during automation. See https://google.aip.dev/148#annotations for details on format and size limitations. |
| `error_logs` | String |  | Output only. Location of Terraform error logs in Google Cloud Storage. Format: `gs://{bucket}/{object}`. |
| `tf_version_constraint` | String |  | Optional. The user-specified Terraform version constraint. Example: "=1.3.10". |
| `error_code` | String |  | Output only. Error code describing errors that may have occurred. |
| `import_existing_resources` | bool |  | By default, Infra Manager will return a failure when Terraform encounters a 409 code (resource conflict error) during actuation. If this flag is set to true, Infra Manager will instead attempt to automatically import the resource into the Terraform state (for supported resource types) and continue actuation. Not all resource types are supported, refer to documentation. |
| `quota_validation` | String |  | Optional. Input to control quota checks for resources in terraform configuration files. There are limited resources on which quota validation applies. |
| `name` | String |  | Identifier. Resource name of the deployment. Format: `projects/{project}/locations/{location}/deployments/{deployment}` |
| `labels` | HashMap<String, String> |  | Optional. User-defined metadata for the deployment. |
| `artifacts_gcs_bucket` | String |  | Optional. User-defined location of Cloud Build logs and artifacts in Google Cloud Storage. Format: `gs://{bucket}/{folder}` A default bucket will be bootstrapped if the field is not set or empty. Default bucket format: `gs://--blueprint-config` Constraints: - The bucket needs to be in the same project as the deployment - The path cannot be within the path of `gcs_source` - The field cannot be updated, including changing its presence |
| `delete_build` | String |  | Output only. Cloud Build instance UUID associated with deleting this deployment. |
| `lock_state` | String |  | Output only. Current lock state of the deployment. |
| `provider_config` | String |  | Optional. This field specifies the provider configurations. |
| `service_account` | String |  | Required. User-specified Service Account (SA) credentials to be used when actuating resources. Format: `projects/{projectID}/serviceAccounts/{serviceAccount}` |
| `state` | String |  | Output only. Current state of the deployment. |
| `latest_revision` | String |  | Output only. Revision name that was most recently applied. Format: `projects/{project}/locations/{location}/deployments/{deployment}/ revisions/{revision}` |
| `tf_version` | String |  | Output only. The current Terraform version set on the deployment. It is in the format of "Major.Minor.Patch", for example, "1.3.10". |
| `update_time` | String |  | Output only. Time when the deployment was last modified. |
| `worker_pool` | String |  | Optional. The user-specified Cloud Build worker pool resource in which the Cloud Build job will execute. Format: `projects/{project}/locations/{location}/workerPools/{workerPoolId}`. If this field is unspecified, the default Cloud Build worker pool will be used. |
| `delete_results` | String |  | Output only. Location of artifacts from a DeleteDeployment operation. |
| `delete_logs` | String |  | Output only. Location of Cloud Build logs in Google Cloud Storage, populated when deleting this deployment. Format: `gs://{bucket}/{object}`. |
| `tf_errors` | Vec<String> |  | Output only. Errors encountered when deleting this deployment. Errors are truncated to 10 entries, see `delete_results` and `error_logs` for full details. |
| `state_detail` | String |  | Output only. Additional information regarding the current state. |
| `parent` | String | ✅ | Required. The parent in whose context the Deployment is created. The parent value is in the format: 'projects/{project_id}/locations/{location}'. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time when the deployment was created. |
| `terraform_blueprint` | String | A blueprint described using Terraform's HashiCorp Configuration Language as a root module. |
| `annotations` | HashMap<String, String> | Optional. Arbitrary key-value metadata storage e.g. to help client tools identify deployments during automation. See https://google.aip.dev/148#annotations for details on format and size limitations. |
| `error_logs` | String | Output only. Location of Terraform error logs in Google Cloud Storage. Format: `gs://{bucket}/{object}`. |
| `tf_version_constraint` | String | Optional. The user-specified Terraform version constraint. Example: "=1.3.10". |
| `error_code` | String | Output only. Error code describing errors that may have occurred. |
| `import_existing_resources` | bool | By default, Infra Manager will return a failure when Terraform encounters a 409 code (resource conflict error) during actuation. If this flag is set to true, Infra Manager will instead attempt to automatically import the resource into the Terraform state (for supported resource types) and continue actuation. Not all resource types are supported, refer to documentation. |
| `quota_validation` | String | Optional. Input to control quota checks for resources in terraform configuration files. There are limited resources on which quota validation applies. |
| `name` | String | Identifier. Resource name of the deployment. Format: `projects/{project}/locations/{location}/deployments/{deployment}` |
| `labels` | HashMap<String, String> | Optional. User-defined metadata for the deployment. |
| `artifacts_gcs_bucket` | String | Optional. User-defined location of Cloud Build logs and artifacts in Google Cloud Storage. Format: `gs://{bucket}/{folder}` A default bucket will be bootstrapped if the field is not set or empty. Default bucket format: `gs://--blueprint-config` Constraints: - The bucket needs to be in the same project as the deployment - The path cannot be within the path of `gcs_source` - The field cannot be updated, including changing its presence |
| `delete_build` | String | Output only. Cloud Build instance UUID associated with deleting this deployment. |
| `lock_state` | String | Output only. Current lock state of the deployment. |
| `provider_config` | String | Optional. This field specifies the provider configurations. |
| `service_account` | String | Required. User-specified Service Account (SA) credentials to be used when actuating resources. Format: `projects/{projectID}/serviceAccounts/{serviceAccount}` |
| `state` | String | Output only. Current state of the deployment. |
| `latest_revision` | String | Output only. Revision name that was most recently applied. Format: `projects/{project}/locations/{location}/deployments/{deployment}/ revisions/{revision}` |
| `tf_version` | String | Output only. The current Terraform version set on the deployment. It is in the format of "Major.Minor.Patch", for example, "1.3.10". |
| `update_time` | String | Output only. Time when the deployment was last modified. |
| `worker_pool` | String | Optional. The user-specified Cloud Build worker pool resource in which the Cloud Build job will execute. Format: `projects/{project}/locations/{location}/workerPools/{workerPoolId}`. If this field is unspecified, the default Cloud Build worker pool will be used. |
| `delete_results` | String | Output only. Location of artifacts from a DeleteDeployment operation. |
| `delete_logs` | String | Output only. Location of Cloud Build logs in Google Cloud Storage, populated when deleting this deployment. Format: `gs://{bucket}/{object}`. |
| `tf_errors` | Vec<String> | Output only. Errors encountered when deleting this deployment. Errors are truncated to 10 entries, see `delete_results` and `error_logs` for full details. |
| `state_detail` | String | Output only. Additional information regarding the current state. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create deployment
deployment = provider.config_api.Deployment {
    parent = "value"  # Required. The parent in whose context the Deployment is created. The parent value is in the format: 'projects/{project_id}/locations/{location}'.
}

# Access deployment outputs
deployment_id = deployment.id
deployment_create_time = deployment.create_time
deployment_terraform_blueprint = deployment.terraform_blueprint
deployment_annotations = deployment.annotations
deployment_error_logs = deployment.error_logs
deployment_tf_version_constraint = deployment.tf_version_constraint
deployment_error_code = deployment.error_code
deployment_import_existing_resources = deployment.import_existing_resources
deployment_quota_validation = deployment.quota_validation
deployment_name = deployment.name
deployment_labels = deployment.labels
deployment_artifacts_gcs_bucket = deployment.artifacts_gcs_bucket
deployment_delete_build = deployment.delete_build
deployment_lock_state = deployment.lock_state
deployment_provider_config = deployment.provider_config
deployment_service_account = deployment.service_account
deployment_state = deployment.state
deployment_latest_revision = deployment.latest_revision
deployment_tf_version = deployment.tf_version
deployment_update_time = deployment.update_time
deployment_worker_pool = deployment.worker_pool
deployment_delete_results = deployment.delete_results
deployment_delete_logs = deployment.delete_logs
deployment_tf_errors = deployment.tf_errors
deployment_state_detail = deployment.state_detail
```

---


### Resource_change

Get a ResourceChange for a given preview.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `intent` | String | Output only. The intent of the resource change. |
| `name` | String | Identifier. The name of the resource change. Format: 'projects/{project_id}/locations/{location}/previews/{preview}/resourceChanges/{resource_change}'. |
| `property_changes` | Vec<String> | Output only. The property changes of the resource change. |
| `terraform_info` | String | Output only. Terraform info of the resource change. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access resource_change outputs
resource_change_id = resource_change.id
resource_change_intent = resource_change.intent
resource_change_name = resource_change.name
resource_change_property_changes = resource_change.property_changes
resource_change_terraform_info = resource_change.terraform_info
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.config_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
operation_response = operation.response
operation_name = operation.name
```

---


### Resource_drift

Get a ResourceDrift for a given preview.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The name of the resource drift. Format: 'projects/{project_id}/locations/{location}/previews/{preview}/resourceDrifts/{resource_drift}'. |
| `terraform_info` | String | Output only. Terraform info of the resource drift. |
| `property_drifts` | Vec<String> | Output only. The property drifts of the resource drift. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access resource_drift outputs
resource_drift_id = resource_drift.id
resource_drift_name = resource_drift.name
resource_drift_terraform_info = resource_drift.terraform_info
resource_drift_property_drifts = resource_drift.property_drifts
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |


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
location_metadata = location.metadata
location_location_id = location.location_id
location_labels = location.labels
location_display_name = location.display_name
location_name = location.name
```

---


### Resource

Gets details about a Resource deployed by Infra Manager.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. Current state of the resource. |
| `cai_assets` | HashMap<String, String> | Output only. Map of Cloud Asset Inventory (CAI) type to CAI info (e.g. CAI ID). CAI type format follows https://cloud.google.com/asset-inventory/docs/supported-asset-types |
| `name` | String | Output only. Resource name. Format: `projects/{project}/locations/{location}/deployments/{deployment}/revisions/{revision}/resources/{resource}` |
| `terraform_info` | String | Output only. Terraform-specific info if this resource was created using Terraform. |
| `intent` | String | Output only. Intent of the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access resource outputs
resource_id = resource.id
resource_state = resource.state
resource_cai_assets = resource.cai_assets
resource_name = resource.name
resource_terraform_info = resource.terraform_info
resource_intent = resource.intent
```

---


### Revision

Exports Terraform state file from a given revision.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | Required. The parent in whose context the statefile is listed. The parent value is in the format: 'projects/{project_id}/locations/{location}/deployments/{deployment}/revisions/{revision}'. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. Current state of the revision. |
| `build` | String | Output only. Cloud Build instance UUID associated with this revision. |
| `error_code` | String | Output only. Code describing any errors that may have occurred. |
| `logs` | String | Output only. Location of Revision operation logs in `gs://{bucket}/{object}` format. |
| `state_detail` | String | Output only. Additional info regarding the current state. |
| `tf_errors` | Vec<String> | Output only. Errors encountered when creating or updating this deployment. Errors are truncated to 10 entries, see `delete_results` and `error_logs` for full details. |
| `update_time` | String | Output only. Time when the revision was last modified. |
| `action` | String | Output only. The action which created this revision |
| `provider_config` | String | Output only. This field specifies the provider configurations. |
| `tf_version_constraint` | String | Output only. The user-specified Terraform version constraint. Example: "=1.3.10". |
| `import_existing_resources` | bool | Output only. By default, Infra Manager will return a failure when Terraform encounters a 409 code (resource conflict error) during actuation. If this flag is set to true, Infra Manager will instead attempt to automatically import the resource into the Terraform state (for supported resource types) and continue actuation. Not all resource types are supported, refer to documentation. |
| `name` | String | Revision name. Format: `projects/{project}/locations/{location}/deployments/{deployment}/ revisions/{revision}` |
| `apply_results` | String | Output only. Outputs and artifacts from applying a deployment. |
| `tf_version` | String | Output only. The version of Terraform used to create the Revision. It is in the format of "Major.Minor.Patch", for example, "1.3.10". |
| `quota_validation` | String | Optional. Input to control quota checks for resources in terraform configuration files. There are limited resources on which quota validation applies. |
| `worker_pool` | String | Output only. The user-specified Cloud Build worker pool resource in which the Cloud Build job will execute. Format: `projects/{project}/locations/{location}/workerPools/{workerPoolId}`. If this field is unspecified, the default Cloud Build worker pool will be used. |
| `create_time` | String | Output only. Time when the revision was created. |
| `error_logs` | String | Output only. Location of Terraform error logs in Google Cloud Storage. Format: `gs://{bucket}/{object}`. |
| `terraform_blueprint` | String | Output only. A blueprint described using Terraform's HashiCorp Configuration Language as a root module. |
| `quota_validation_results` | String | Output only. Cloud Storage path containing quota validation results. This field is set when a user sets Deployment.quota_validation field to ENABLED or ENFORCED. Format: `gs://{bucket}/{object}`. |
| `service_account` | String | Output only. User-specified Service Account (SA) to be used as credential to manage resources. Format: `projects/{projectID}/serviceAccounts/{serviceAccount}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create revision
revision = provider.config_api.Revision {
    parent = "value"  # Required. The parent in whose context the statefile is listed. The parent value is in the format: 'projects/{project_id}/locations/{location}/deployments/{deployment}/revisions/{revision}'.
}

# Access revision outputs
revision_id = revision.id
revision_state = revision.state
revision_build = revision.build
revision_error_code = revision.error_code
revision_logs = revision.logs
revision_state_detail = revision.state_detail
revision_tf_errors = revision.tf_errors
revision_update_time = revision.update_time
revision_action = revision.action
revision_provider_config = revision.provider_config
revision_tf_version_constraint = revision.tf_version_constraint
revision_import_existing_resources = revision.import_existing_resources
revision_name = revision.name
revision_apply_results = revision.apply_results
revision_tf_version = revision.tf_version
revision_quota_validation = revision.quota_validation
revision_worker_pool = revision.worker_pool
revision_create_time = revision.create_time
revision_error_logs = revision.error_logs
revision_terraform_blueprint = revision.terraform_blueprint
revision_quota_validation_results = revision.quota_validation_results
revision_service_account = revision.service_account
```

---


### Terraform_version

Gets details about a TerraformVersion.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `obsolete_time` | String | Output only. When the version is obsolete. |
| `deprecate_time` | String | Output only. When the version is deprecated. |
| `support_time` | String | Output only. When the version is supported. |
| `state` | String | Output only. The state of the version, ACTIVE, DEPRECATED or OBSOLETE. |
| `name` | String | Identifier. The version name is in the format: 'projects/{project_id}/locations/{location}/terraformVersions/{terraform_version}'. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access terraform_version outputs
terraform_version_id = terraform_version.id
terraform_version_obsolete_time = terraform_version.obsolete_time
terraform_version_deprecate_time = terraform_version.deprecate_time
terraform_version_support_time = terraform_version.support_time
terraform_version_state = terraform_version.state
terraform_version_name = terraform_version.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple preview resources
preview_0 = provider.config_api.Preview {
    parent = "value-0"
}
preview_1 = provider.config_api.Preview {
    parent = "value-1"
}
preview_2 = provider.config_api.Preview {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    preview = provider.config_api.Preview {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Config_api Documentation](https://cloud.google.com/config_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
