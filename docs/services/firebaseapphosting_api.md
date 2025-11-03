# Firebaseapphosting_api Service



**Resources**: 14

---

## Overview

The firebaseapphosting_api service provides access to 14 resource types:

- [Backend](#backend) [CRUD]
- [Domain](#domain) [CRUD]
- [Location](#location) [R]
- [Build](#build) [CRD]
- [Rollout](#rollout) [CR]
- [Traffic](#traffic) [RU]
- [Operation](#operation) [CRD]
- [Backend](#backend) [CRUD]
- [Build](#build) [CRD]
- [Domain](#domain) [CRUD]
- [Location](#location) [R]
- [Rollout](#rollout) [CR]
- [Operation](#operation) [CRD]
- [Traffic](#traffic) [RU]

---

## Resources


### Backend

Creates a new backend in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `managed_resources` | Vec<String> |  | Output only. A list of the resources managed by this backend. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `app_id` | String |  | Optional. The [ID of a Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id) associated with the backend. |
| `uri` | String |  | Output only. The primary URI to communicate with the backend. |
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `codebase` | String |  | Optional. If specified, the connection to an external source repository to watch for event-driven updates to the backend. |
| `name` | String |  | Identifier. The resource name of the backend. Format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |
| `service_account` | String |  | Required. The name of the service account used for Cloud Build and Cloud Run. Should have the role roles/firebaseapphosting.computeRunner or equivalent permissions. |
| `serving_locality` | String |  | Required. Immutable. Specifies how App Hosting will serve the content for this backend. It will either be contained to a single region (REGIONAL_STRICT) or allowed to use App Hosting's global-replicated serving infrastructure (GLOBAL_ACCESS). |
| `update_time` | String |  | Output only. Time at which the backend was last updated. |
| `environment` | String |  | Optional. The environment name of the backend, used to load environment variables from environment specific configuration. |
| `create_time` | String |  | Output only. Time at which the backend was created. |
| `mode` | String |  | Optional. Deprecated: Use `environment` instead. |
| `display_name` | String |  | Optional. Human-readable name. 63 character limit. |
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the system is working to make adjustments to the backend during a LRO. |
| `request_logs_disabled` | bool |  | Optional. A field that, if true, indicates that incoming request logs are disabled for this backend. Incoming request logs are enabled by default. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `delete_time` | String |  | Output only. Time at which the backend was deleted. |
| `parent` | String | ✅ | Required. A parent name of the form `projects/{project}/locations/{locationId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `managed_resources` | Vec<String> | Output only. A list of the resources managed by this backend. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `app_id` | String | Optional. The [ID of a Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id) associated with the backend. |
| `uri` | String | Output only. The primary URI to communicate with the backend. |
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `codebase` | String | Optional. If specified, the connection to an external source repository to watch for event-driven updates to the backend. |
| `name` | String | Identifier. The resource name of the backend. Format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |
| `service_account` | String | Required. The name of the service account used for Cloud Build and Cloud Run. Should have the role roles/firebaseapphosting.computeRunner or equivalent permissions. |
| `serving_locality` | String | Required. Immutable. Specifies how App Hosting will serve the content for this backend. It will either be contained to a single region (REGIONAL_STRICT) or allowed to use App Hosting's global-replicated serving infrastructure (GLOBAL_ACCESS). |
| `update_time` | String | Output only. Time at which the backend was last updated. |
| `environment` | String | Optional. The environment name of the backend, used to load environment variables from environment specific configuration. |
| `create_time` | String | Output only. Time at which the backend was created. |
| `mode` | String | Optional. Deprecated: Use `environment` instead. |
| `display_name` | String | Optional. Human-readable name. 63 character limit. |
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the system is working to make adjustments to the backend during a LRO. |
| `request_logs_disabled` | bool | Optional. A field that, if true, indicates that incoming request logs are disabled for this backend. Incoming request logs are enabled by default. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `delete_time` | String | Output only. Time at which the backend was deleted. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backend
backend = provider.firebaseapphosting_api.Backend {
    parent = "value"  # Required. A parent name of the form `projects/{project}/locations/{locationId}`.
}

# Access backend outputs
backend_id = backend.id
backend_managed_resources = backend.managed_resources
backend_etag = backend.etag
backend_app_id = backend.app_id
backend_uri = backend.uri
backend_annotations = backend.annotations
backend_codebase = backend.codebase
backend_name = backend.name
backend_service_account = backend.service_account
backend_serving_locality = backend.serving_locality
backend_update_time = backend.update_time
backend_environment = backend.environment
backend_create_time = backend.create_time
backend_mode = backend.mode
backend_display_name = backend.display_name
backend_labels = backend.labels
backend_reconciling = backend.reconciling
backend_request_logs_disabled = backend.request_logs_disabled
backend_uid = backend.uid
backend_delete_time = backend.delete_time
```

---


### Domain

Links a new domain to a backend.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | Output only. The type of the domain. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `annotations` | HashMap<String, String> |  | Optional. Annotations as key value pairs. |
| `delete_time` | String |  | Output only. Time at which the domain was deleted. |
| `create_time` | String |  | Output only. Time at which the domain was created. |
| `disabled` | bool |  | Optional. Whether the domain is disabled. Defaults to false. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the build has an ongoing LRO. |
| `custom_domain_status` | String |  | Output only. Represents the state and configuration of a `CUSTOM` type domain. It is only present on Domains of that type. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `serve` | String |  | Optional. The serving behavior of the domain. If specified, the domain will serve content other than its backend's live content. |
| `name` | String |  | Identifier. The resource name of the domain, e.g. `/projects/p/locations/l/backends/b/domains/foo.com` |
| `display_name` | String |  | Optional. Mutable human-readable name for the domain. 63 character limit. e.g. `prod domain`. |
| `update_time` | String |  | Output only. Time at which the domain was last updated. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `parent` | String | ✅ | Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | Output only. The type of the domain. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `annotations` | HashMap<String, String> | Optional. Annotations as key value pairs. |
| `delete_time` | String | Output only. Time at which the domain was deleted. |
| `create_time` | String | Output only. Time at which the domain was created. |
| `disabled` | bool | Optional. Whether the domain is disabled. Defaults to false. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the build has an ongoing LRO. |
| `custom_domain_status` | String | Output only. Represents the state and configuration of a `CUSTOM` type domain. It is only present on Domains of that type. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `serve` | String | Optional. The serving behavior of the domain. If specified, the domain will serve content other than its backend's live content. |
| `name` | String | Identifier. The resource name of the domain, e.g. `/projects/p/locations/l/backends/b/domains/foo.com` |
| `display_name` | String | Optional. Mutable human-readable name for the domain. 63 character limit. e.g. `prod domain`. |
| `update_time` | String | Output only. Time at which the domain was last updated. |
| `uid` | String | Output only. System-assigned, unique identifier. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create domain
domain = provider.firebaseapphosting_api.Domain {
    parent = "value"  # Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`.
}

# Access domain outputs
domain_id = domain.id
domain_type = domain.type
domain_labels = domain.labels
domain_annotations = domain.annotations
domain_delete_time = domain.delete_time
domain_create_time = domain.create_time
domain_disabled = domain.disabled
domain_reconciling = domain.reconciling
domain_custom_domain_status = domain.custom_domain_status
domain_etag = domain.etag
domain_serve = domain.serve
domain_name = domain.name
domain_display_name = domain.display_name
domain_update_time = domain.update_time
domain_uid = domain.uid
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |


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
location_location_id = location.location_id
location_name = location.name
location_display_name = location.display_name
location_metadata = location.metadata
```

---


### Build

Creates a new build for a backend.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `image` | String |  | Output only. The Artifact Registry [container image](https://cloud.google.com/artifact-registry/docs/reference/rest/v1/projects.locations.repositories.dockerImages) URI, used by the Cloud Run [`revision`](https://cloud.google.com/run/docs/reference/rest/v2/projects.locations.services.revisions) for this build. |
| `environment` | String |  | Output only. The environment name of the backend when this build was created. |
| `display_name` | String |  | Optional. Human-readable name. 63 character limit. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the build has an ongoing LRO. |
| `errors` | Vec<String> |  | Output only. A list of all errors that occurred during an App Hosting build. |
| `name` | String |  | Identifier. The resource name of the build. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/builds/{buildId}`. |
| `config` | String |  | Optional. Additional configuration of the service. |
| `delete_time` | String |  | Output only. Time at which the build was deleted. |
| `create_time` | String |  | Output only. Time at which the build was created. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `source` | String |  | Required. Immutable. The source for the build. |
| `state` | String |  | Output only. The state of the build. |
| `update_time` | String |  | Output only. Time at which the build was last updated. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `build_logs_uri` | String |  | Output only. The location of the [Cloud Build logs](https://cloud.google.com/build/docs/view-build-results) for the build process. |
| `parent` | String | ✅ | Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `image` | String | Output only. The Artifact Registry [container image](https://cloud.google.com/artifact-registry/docs/reference/rest/v1/projects.locations.repositories.dockerImages) URI, used by the Cloud Run [`revision`](https://cloud.google.com/run/docs/reference/rest/v2/projects.locations.services.revisions) for this build. |
| `environment` | String | Output only. The environment name of the backend when this build was created. |
| `display_name` | String | Optional. Human-readable name. 63 character limit. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the build has an ongoing LRO. |
| `errors` | Vec<String> | Output only. A list of all errors that occurred during an App Hosting build. |
| `name` | String | Identifier. The resource name of the build. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/builds/{buildId}`. |
| `config` | String | Optional. Additional configuration of the service. |
| `delete_time` | String | Output only. Time at which the build was deleted. |
| `create_time` | String | Output only. Time at which the build was created. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `source` | String | Required. Immutable. The source for the build. |
| `state` | String | Output only. The state of the build. |
| `update_time` | String | Output only. Time at which the build was last updated. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `build_logs_uri` | String | Output only. The location of the [Cloud Build logs](https://cloud.google.com/build/docs/view-build-results) for the build process. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create build
build = provider.firebaseapphosting_api.Build {
    parent = "value"  # Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`.
}

# Access build outputs
build_id = build.id
build_labels = build.labels
build_annotations = build.annotations
build_image = build.image
build_environment = build.environment
build_display_name = build.display_name
build_reconciling = build.reconciling
build_errors = build.errors
build_name = build.name
build_config = build.config
build_delete_time = build.delete_time
build_create_time = build.create_time
build_etag = build.etag
build_source = build.source
build_state = build.state
build_update_time = build.update_time
build_uid = build.uid
build_build_logs_uri = build.build_logs_uri
```

---


### Rollout

Creates a new rollout for a backend.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the Rollout currently has an LRO. |
| `error` | String |  | Output only. A status and (human readable) error message for the rollout, if in a `FAILED` state. |
| `state` | String |  | Output only. The state of the rollout. |
| `create_time` | String |  | Output only. Time at which the rollout was created. |
| `display_name` | String |  | Optional. Human-readable name. 63 character limit. |
| `name` | String |  | Identifier. The resource name of the rollout. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/rollouts/{rolloutId}`. |
| `delete_time` | String |  | Output only. Time at which the rollout was deleted. |
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `update_time` | String |  | Output only. Time at which the rollout was last updated. |
| `build` | String |  | Required. Immutable. The name of a build that already exists. It doesn't have to be built; a rollout will wait for a build to be ready before updating traffic. |
| `parent` | String | ✅ | Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the Rollout currently has an LRO. |
| `error` | String | Output only. A status and (human readable) error message for the rollout, if in a `FAILED` state. |
| `state` | String | Output only. The state of the rollout. |
| `create_time` | String | Output only. Time at which the rollout was created. |
| `display_name` | String | Optional. Human-readable name. 63 character limit. |
| `name` | String | Identifier. The resource name of the rollout. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/rollouts/{rolloutId}`. |
| `delete_time` | String | Output only. Time at which the rollout was deleted. |
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `update_time` | String | Output only. Time at which the rollout was last updated. |
| `build` | String | Required. Immutable. The name of a build that already exists. It doesn't have to be built; a rollout will wait for a build to be ready before updating traffic. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rollout
rollout = provider.firebaseapphosting_api.Rollout {
    parent = "value"  # Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`.
}

# Access rollout outputs
rollout_id = rollout.id
rollout_etag = rollout.etag
rollout_labels = rollout.labels
rollout_reconciling = rollout.reconciling
rollout_error = rollout.error
rollout_state = rollout.state
rollout_create_time = rollout.create_time
rollout_display_name = rollout.display_name
rollout_name = rollout.name
rollout_delete_time = rollout.delete_time
rollout_annotations = rollout.annotations
rollout_uid = rollout.uid
rollout_update_time = rollout.update_time
rollout_build = rollout.build
```

---


### Traffic

Gets information about a backend's traffic.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `update_time` | String |  | Output only. Time at which the backend was last updated. |
| `create_time` | String |  | Output only. Time at which the backend was created. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `name` | String |  | Identifier. The resource name of the backend's traffic. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/traffic`. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the system is working to make the backend's `current` match the requested `target` list. |
| `rollout_policy` | String |  | A rollout policy specifies how new builds and automatic deployments are created. |
| `current` | String |  | Output only. Current state of traffic allocation for the backend. When setting `target`, this field may differ for some time until the desired state is reached. |
| `target` | String |  | Set to manually control the desired traffic for the backend. This will cause `current` to eventually match this value. The percentages must add up to 100%. |
| `name` | String | ✅ | Identifier. The resource name of the backend's traffic. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/traffic`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `update_time` | String | Output only. Time at which the backend was last updated. |
| `create_time` | String | Output only. Time at which the backend was created. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `name` | String | Identifier. The resource name of the backend's traffic. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/traffic`. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the system is working to make the backend's `current` match the requested `target` list. |
| `rollout_policy` | String | A rollout policy specifies how new builds and automatic deployments are created. |
| `current` | String | Output only. Current state of traffic allocation for the backend. When setting `target`, this field may differ for some time until the desired state is reached. |
| `target` | String | Set to manually control the desired traffic for the backend. This will cause `current` to eventually match this value. The percentages must add up to 100%. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access traffic outputs
traffic_id = traffic.id
traffic_annotations = traffic.annotations
traffic_update_time = traffic.update_time
traffic_create_time = traffic.create_time
traffic_uid = traffic.uid
traffic_etag = traffic.etag
traffic_labels = traffic.labels
traffic_name = traffic.name
traffic_reconciling = traffic.reconciling
traffic_rollout_policy = traffic.rollout_policy
traffic_current = traffic.current
traffic_target = traffic.target
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.firebaseapphosting_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
operation_response = operation.response
operation_name = operation.name
```

---


### Backend

Creates a new backend in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_id` | String |  | Optional. The [ID of a Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id) associated with the backend. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `create_time` | String |  | Output only. Time at which the backend was created. |
| `service_account` | String |  | Required. The name of the service account used for Cloud Build and Cloud Run. Should have the role roles/firebaseapphosting.computeRunner or equivalent permissions. |
| `serving_locality` | String |  | Required. Immutable. Specifies how App Hosting will serve the content for this backend. It will either be contained to a single region (REGIONAL_STRICT) or allowed to use App Hosting's global-replicated serving infrastructure (GLOBAL_ACCESS). |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `update_time` | String |  | Output only. Time at which the backend was last updated. |
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `environment` | String |  | Optional. The environment name of the backend, used to load environment variables from environment specific configuration. |
| `uri` | String |  | Output only. The primary URI to communicate with the backend. |
| `codebase` | String |  | Optional. If specified, the connection to an external source repository to watch for event-driven updates to the backend. |
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the system is working to make adjustments to the backend during a LRO. |
| `mode` | String |  | Optional. Deprecated: Use `environment` instead. |
| `display_name` | String |  | Optional. Human-readable name. 63 character limit. |
| `override_env` | Vec<String> |  | Optional. Override environment variables for this Backend. |
| `name` | String |  | Identifier. The resource name of the backend. Format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |
| `delete_time` | String |  | Output only. Time at which the backend was deleted. |
| `managed_resources` | Vec<String> |  | Output only. A list of the resources managed by this backend. |
| `request_logs_disabled` | bool |  | Optional. A field that, if true, indicates that incoming request logs are disabled for this backend. Incoming request logs are enabled by default. |
| `parent` | String | ✅ | Required. A parent name of the form `projects/{project}/locations/{locationId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_id` | String | Optional. The [ID of a Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id) associated with the backend. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `create_time` | String | Output only. Time at which the backend was created. |
| `service_account` | String | Required. The name of the service account used for Cloud Build and Cloud Run. Should have the role roles/firebaseapphosting.computeRunner or equivalent permissions. |
| `serving_locality` | String | Required. Immutable. Specifies how App Hosting will serve the content for this backend. It will either be contained to a single region (REGIONAL_STRICT) or allowed to use App Hosting's global-replicated serving infrastructure (GLOBAL_ACCESS). |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `update_time` | String | Output only. Time at which the backend was last updated. |
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `environment` | String | Optional. The environment name of the backend, used to load environment variables from environment specific configuration. |
| `uri` | String | Output only. The primary URI to communicate with the backend. |
| `codebase` | String | Optional. If specified, the connection to an external source repository to watch for event-driven updates to the backend. |
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the system is working to make adjustments to the backend during a LRO. |
| `mode` | String | Optional. Deprecated: Use `environment` instead. |
| `display_name` | String | Optional. Human-readable name. 63 character limit. |
| `override_env` | Vec<String> | Optional. Override environment variables for this Backend. |
| `name` | String | Identifier. The resource name of the backend. Format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |
| `delete_time` | String | Output only. Time at which the backend was deleted. |
| `managed_resources` | Vec<String> | Output only. A list of the resources managed by this backend. |
| `request_logs_disabled` | bool | Optional. A field that, if true, indicates that incoming request logs are disabled for this backend. Incoming request logs are enabled by default. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backend
backend = provider.firebaseapphosting_api.Backend {
    parent = "value"  # Required. A parent name of the form `projects/{project}/locations/{locationId}`.
}

# Access backend outputs
backend_id = backend.id
backend_app_id = backend.app_id
backend_etag = backend.etag
backend_create_time = backend.create_time
backend_service_account = backend.service_account
backend_serving_locality = backend.serving_locality
backend_uid = backend.uid
backend_update_time = backend.update_time
backend_annotations = backend.annotations
backend_environment = backend.environment
backend_uri = backend.uri
backend_codebase = backend.codebase
backend_labels = backend.labels
backend_reconciling = backend.reconciling
backend_mode = backend.mode
backend_display_name = backend.display_name
backend_override_env = backend.override_env
backend_name = backend.name
backend_delete_time = backend.delete_time
backend_managed_resources = backend.managed_resources
backend_request_logs_disabled = backend.request_logs_disabled
```

---


### Build

Creates a new build for a backend.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the build has an ongoing LRO. |
| `build_logs_uri` | String |  | Output only. The location of the [Cloud Build logs](https://cloud.google.com/build/docs/view-build-results) for the build process. |
| `display_name` | String |  | Optional. Human-readable name. 63 character limit. |
| `delete_time` | String |  | Output only. Time at which the build was deleted. |
| `errors` | Vec<String> |  | Output only. A list of all errors that occurred during an App Hosting build. |
| `create_time` | String |  | Output only. Time at which the build was created. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `error` | String |  | Output only. A status and (human readable) error message for the build, if in a `FAILED` state. Deprecated. Use `errors` instead. |
| `image` | String |  | Output only. The Artifact Registry [container image](https://cloud.google.com/artifact-registry/docs/reference/rest/v1/projects.locations.repositories.dockerImages) URI, used by the Cloud Run [`revision`](https://cloud.google.com/run/docs/reference/rest/v2/projects.locations.services.revisions) for this build. |
| `state` | String |  | Output only. The state of the build. |
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `update_time` | String |  | Output only. Time at which the build was last updated. |
| `name` | String |  | Identifier. The resource name of the build. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/builds/{buildId}`. |
| `source` | String |  | Required. Immutable. The source for the build. |
| `error_source` | String |  | Output only. The source of the error for the build, if in a `FAILED` state. Deprecated. Use `errors` instead. |
| `environment` | String |  | Output only. The environment name of the backend when this build was created. |
| `config` | String |  | Optional. Additional configuration of the service. |
| `parent` | String | ✅ | Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reconciling` | bool | Output only. A field that, if true, indicates that the build has an ongoing LRO. |
| `build_logs_uri` | String | Output only. The location of the [Cloud Build logs](https://cloud.google.com/build/docs/view-build-results) for the build process. |
| `display_name` | String | Optional. Human-readable name. 63 character limit. |
| `delete_time` | String | Output only. Time at which the build was deleted. |
| `errors` | Vec<String> | Output only. A list of all errors that occurred during an App Hosting build. |
| `create_time` | String | Output only. Time at which the build was created. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `error` | String | Output only. A status and (human readable) error message for the build, if in a `FAILED` state. Deprecated. Use `errors` instead. |
| `image` | String | Output only. The Artifact Registry [container image](https://cloud.google.com/artifact-registry/docs/reference/rest/v1/projects.locations.repositories.dockerImages) URI, used by the Cloud Run [`revision`](https://cloud.google.com/run/docs/reference/rest/v2/projects.locations.services.revisions) for this build. |
| `state` | String | Output only. The state of the build. |
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `update_time` | String | Output only. Time at which the build was last updated. |
| `name` | String | Identifier. The resource name of the build. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/builds/{buildId}`. |
| `source` | String | Required. Immutable. The source for the build. |
| `error_source` | String | Output only. The source of the error for the build, if in a `FAILED` state. Deprecated. Use `errors` instead. |
| `environment` | String | Output only. The environment name of the backend when this build was created. |
| `config` | String | Optional. Additional configuration of the service. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create build
build = provider.firebaseapphosting_api.Build {
    parent = "value"  # Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`.
}

# Access build outputs
build_id = build.id
build_reconciling = build.reconciling
build_build_logs_uri = build.build_logs_uri
build_display_name = build.display_name
build_delete_time = build.delete_time
build_errors = build.errors
build_create_time = build.create_time
build_etag = build.etag
build_labels = build.labels
build_error = build.error
build_image = build.image
build_state = build.state
build_annotations = build.annotations
build_uid = build.uid
build_update_time = build.update_time
build_name = build.name
build_source = build.source
build_error_source = build.error_source
build_environment = build.environment
build_config = build.config
```

---


### Domain

Links a new domain to a backend.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `purge_time` | String |  | Output only. Time at which a soft-deleted domain will be purged, rendering in permanently deleted. |
| `type` | String |  | Output only. The type of the domain. |
| `disabled` | bool |  | Optional. Whether the domain is disabled. Defaults to false. |
| `custom_domain_status` | String |  | Output only. Represents the state and configuration of a `CUSTOM` type domain. It is only present on Domains of that type. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `create_time` | String |  | Output only. Time at which the domain was created. |
| `serve` | String |  | Optional. The serving behavior of the domain. If specified, the domain will serve content other than its backend's live content. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `name` | String |  | Identifier. The resource name of the domain, e.g. `/projects/p/locations/l/backends/b/domains/foo.com` |
| `update_time` | String |  | Output only. Time at which the domain was last updated. |
| `display_name` | String |  | Optional. Mutable human-readable name for the domain. 63 character limit. e.g. `prod domain`. |
| `annotations` | HashMap<String, String> |  | Optional. Annotations as key value pairs. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `delete_time` | String |  | Output only. Time at which the domain was deleted. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the build has an ongoing LRO. |
| `parent` | String | ✅ | Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `purge_time` | String | Output only. Time at which a soft-deleted domain will be purged, rendering in permanently deleted. |
| `type` | String | Output only. The type of the domain. |
| `disabled` | bool | Optional. Whether the domain is disabled. Defaults to false. |
| `custom_domain_status` | String | Output only. Represents the state and configuration of a `CUSTOM` type domain. It is only present on Domains of that type. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `create_time` | String | Output only. Time at which the domain was created. |
| `serve` | String | Optional. The serving behavior of the domain. If specified, the domain will serve content other than its backend's live content. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `name` | String | Identifier. The resource name of the domain, e.g. `/projects/p/locations/l/backends/b/domains/foo.com` |
| `update_time` | String | Output only. Time at which the domain was last updated. |
| `display_name` | String | Optional. Mutable human-readable name for the domain. 63 character limit. e.g. `prod domain`. |
| `annotations` | HashMap<String, String> | Optional. Annotations as key value pairs. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `delete_time` | String | Output only. Time at which the domain was deleted. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the build has an ongoing LRO. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create domain
domain = provider.firebaseapphosting_api.Domain {
    parent = "value"  # Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`.
}

# Access domain outputs
domain_id = domain.id
domain_purge_time = domain.purge_time
domain_type = domain.type
domain_disabled = domain.disabled
domain_custom_domain_status = domain.custom_domain_status
domain_uid = domain.uid
domain_create_time = domain.create_time
domain_serve = domain.serve
domain_etag = domain.etag
domain_name = domain.name
domain_update_time = domain.update_time
domain_display_name = domain.display_name
domain_annotations = domain.annotations
domain_labels = domain.labels
domain_delete_time = domain.delete_time
domain_reconciling = domain.reconciling
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
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
location_display_name = location.display_name
location_name = location.name
location_labels = location.labels
location_metadata = location.metadata
location_location_id = location.location_id
```

---


### Rollout

Creates a new rollout for a backend.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `display_name` | String |  | Optional. Human-readable name. 63 character limit. |
| `delete_time` | String |  | Output only. Time at which the rollout was deleted. |
| `build` | String |  | Required. Immutable. The name of a build that already exists. It doesn't have to be built; a rollout will wait for a build to be ready before updating traffic. |
| `error` | String |  | Output only. A status and (human readable) error message for the rollout, if in a `FAILED` state. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the Rollout currently has an LRO. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `create_time` | String |  | Output only. Time at which the rollout was created. |
| `state` | String |  | Output only. The state of the rollout. |
| `name` | String |  | Identifier. The resource name of the rollout. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/rollouts/{rolloutId}`. |
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `update_time` | String |  | Output only. Time at which the rollout was last updated. |
| `parent` | String | ✅ | Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `display_name` | String | Optional. Human-readable name. 63 character limit. |
| `delete_time` | String | Output only. Time at which the rollout was deleted. |
| `build` | String | Required. Immutable. The name of a build that already exists. It doesn't have to be built; a rollout will wait for a build to be ready before updating traffic. |
| `error` | String | Output only. A status and (human readable) error message for the rollout, if in a `FAILED` state. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the Rollout currently has an LRO. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `create_time` | String | Output only. Time at which the rollout was created. |
| `state` | String | Output only. The state of the rollout. |
| `name` | String | Identifier. The resource name of the rollout. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/rollouts/{rolloutId}`. |
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `update_time` | String | Output only. Time at which the rollout was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rollout
rollout = provider.firebaseapphosting_api.Rollout {
    parent = "value"  # Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`.
}

# Access rollout outputs
rollout_id = rollout.id
rollout_etag = rollout.etag
rollout_display_name = rollout.display_name
rollout_delete_time = rollout.delete_time
rollout_build = rollout.build
rollout_error = rollout.error
rollout_reconciling = rollout.reconciling
rollout_uid = rollout.uid
rollout_labels = rollout.labels
rollout_create_time = rollout.create_time
rollout_state = rollout.state
rollout_name = rollout.name
rollout_annotations = rollout.annotations
rollout_update_time = rollout.update_time
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.firebaseapphosting_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
operation_done = operation.done
operation_response = operation.response
```

---


### Traffic

Gets information about a backend's traffic.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the system is working to make the backend's `current` match the requested `target` list. |
| `current` | String |  | Output only. Current state of traffic allocation for the backend. When setting `target`, this field may differ for some time until the desired state is reached. |
| `create_time` | String |  | Output only. Time at which the backend was created. |
| `target` | String |  | Set to manually control the desired traffic for the backend. This will cause `current` to eventually match this value. The percentages must add up to 100%. |
| `update_time` | String |  | Output only. Time at which the backend was last updated. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `name` | String |  | Identifier. The resource name of the backend's traffic. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/traffic`. |
| `rollout_policy` | String |  | A rollout policy specifies how new builds and automatic deployments are created. |
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `name` | String | ✅ | Identifier. The resource name of the backend's traffic. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/traffic`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reconciling` | bool | Output only. A field that, if true, indicates that the system is working to make the backend's `current` match the requested `target` list. |
| `current` | String | Output only. Current state of traffic allocation for the backend. When setting `target`, this field may differ for some time until the desired state is reached. |
| `create_time` | String | Output only. Time at which the backend was created. |
| `target` | String | Set to manually control the desired traffic for the backend. This will cause `current` to eventually match this value. The percentages must add up to 100%. |
| `update_time` | String | Output only. Time at which the backend was last updated. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `name` | String | Identifier. The resource name of the backend's traffic. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/traffic`. |
| `rollout_policy` | String | A rollout policy specifies how new builds and automatic deployments are created. |
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access traffic outputs
traffic_id = traffic.id
traffic_reconciling = traffic.reconciling
traffic_current = traffic.current
traffic_create_time = traffic.create_time
traffic_target = traffic.target
traffic_update_time = traffic.update_time
traffic_etag = traffic.etag
traffic_annotations = traffic.annotations
traffic_uid = traffic.uid
traffic_name = traffic.name
traffic_rollout_policy = traffic.rollout_policy
traffic_labels = traffic.labels
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple backend resources
backend_0 = provider.firebaseapphosting_api.Backend {
    parent = "value-0"
}
backend_1 = provider.firebaseapphosting_api.Backend {
    parent = "value-1"
}
backend_2 = provider.firebaseapphosting_api.Backend {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    backend = provider.firebaseapphosting_api.Backend {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Firebaseapphosting_api Documentation](https://cloud.google.com/firebaseapphosting_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
