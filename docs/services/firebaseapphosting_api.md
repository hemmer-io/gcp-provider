# Firebaseapphosting_api Service



**Resources**: 14

---

## Overview

The firebaseapphosting_api service provides access to 14 resource types:

- [Build](#build) [CRD]
- [Location](#location) [R]
- [Domain](#domain) [CRUD]
- [Operation](#operation) [CRD]
- [Traffic](#traffic) [RU]
- [Rollout](#rollout) [CR]
- [Backend](#backend) [CRUD]
- [Location](#location) [R]
- [Traffic](#traffic) [RU]
- [Domain](#domain) [CRUD]
- [Rollout](#rollout) [CR]
- [Build](#build) [CRD]
- [Operation](#operation) [CRD]
- [Backend](#backend) [CRUD]

---

## Resources


### Build

Creates a new build for a backend.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the build has an ongoing LRO. |
| `environment` | String |  | Output only. The environment name of the backend when this build was created. |
| `source` | String |  | Required. Immutable. The source for the build. |
| `state` | String |  | Output only. The state of the build. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `display_name` | String |  | Optional. Human-readable name. 63 character limit. |
| `errors` | Vec<String> |  | Output only. A list of all errors that occurred during an App Hosting build. |
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `config` | String |  | Optional. Additional configuration of the service. |
| `image` | String |  | Output only. The Artifact Registry [container image](https://cloud.google.com/artifact-registry/docs/reference/rest/v1/projects.locations.repositories.dockerImages) URI, used by the Cloud Run [`revision`](https://cloud.google.com/run/docs/reference/rest/v2/projects.locations.services.revisions) for this build. |
| `build_logs_uri` | String |  | Output only. The location of the [Cloud Build logs](https://cloud.google.com/build/docs/view-build-results) for the build process. |
| `name` | String |  | Identifier. The resource name of the build. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/builds/{buildId}`. |
| `delete_time` | String |  | Output only. Time at which the build was deleted. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `update_time` | String |  | Output only. Time at which the build was last updated. |
| `create_time` | String |  | Output only. Time at which the build was created. |
| `parent` | String | ✅ | Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the build has an ongoing LRO. |
| `environment` | String | Output only. The environment name of the backend when this build was created. |
| `source` | String | Required. Immutable. The source for the build. |
| `state` | String | Output only. The state of the build. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `display_name` | String | Optional. Human-readable name. 63 character limit. |
| `errors` | Vec<String> | Output only. A list of all errors that occurred during an App Hosting build. |
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `config` | String | Optional. Additional configuration of the service. |
| `image` | String | Output only. The Artifact Registry [container image](https://cloud.google.com/artifact-registry/docs/reference/rest/v1/projects.locations.repositories.dockerImages) URI, used by the Cloud Run [`revision`](https://cloud.google.com/run/docs/reference/rest/v2/projects.locations.services.revisions) for this build. |
| `build_logs_uri` | String | Output only. The location of the [Cloud Build logs](https://cloud.google.com/build/docs/view-build-results) for the build process. |
| `name` | String | Identifier. The resource name of the build. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/builds/{buildId}`. |
| `delete_time` | String | Output only. Time at which the build was deleted. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `update_time` | String | Output only. Time at which the build was last updated. |
| `create_time` | String | Output only. Time at which the build was created. |


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
build_annotations = build.annotations
build_reconciling = build.reconciling
build_environment = build.environment
build_source = build.source
build_state = build.state
build_uid = build.uid
build_display_name = build.display_name
build_errors = build.errors
build_labels = build.labels
build_config = build.config
build_image = build.image
build_build_logs_uri = build.build_logs_uri
build_name = build.name
build_delete_time = build.delete_time
build_etag = build.etag
build_update_time = build.update_time
build_create_time = build.create_time
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_display_name = location.display_name
location_name = location.name
location_location_id = location.location_id
location_metadata = location.metadata
```

---


### Domain

Links a new domain to a backend.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of the domain, e.g. `/projects/p/locations/l/backends/b/domains/foo.com` |
| `disabled` | bool |  | Optional. Whether the domain is disabled. Defaults to false. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `delete_time` | String |  | Output only. Time at which the domain was deleted. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `type` | String |  | Output only. The type of the domain. |
| `create_time` | String |  | Output only. Time at which the domain was created. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the build has an ongoing LRO. |
| `serve` | String |  | Optional. The serving behavior of the domain. If specified, the domain will serve content other than its backend's live content. |
| `update_time` | String |  | Output only. Time at which the domain was last updated. |
| `custom_domain_status` | String |  | Output only. Represents the state and configuration of a `CUSTOM` type domain. It is only present on Domains of that type. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `annotations` | HashMap<String, String> |  | Optional. Annotations as key value pairs. |
| `display_name` | String |  | Optional. Mutable human-readable name for the domain. 63 character limit. e.g. `prod domain`. |
| `parent` | String | ✅ | Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the domain, e.g. `/projects/p/locations/l/backends/b/domains/foo.com` |
| `disabled` | bool | Optional. Whether the domain is disabled. Defaults to false. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `delete_time` | String | Output only. Time at which the domain was deleted. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `type` | String | Output only. The type of the domain. |
| `create_time` | String | Output only. Time at which the domain was created. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the build has an ongoing LRO. |
| `serve` | String | Optional. The serving behavior of the domain. If specified, the domain will serve content other than its backend's live content. |
| `update_time` | String | Output only. Time at which the domain was last updated. |
| `custom_domain_status` | String | Output only. Represents the state and configuration of a `CUSTOM` type domain. It is only present on Domains of that type. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `annotations` | HashMap<String, String> | Optional. Annotations as key value pairs. |
| `display_name` | String | Optional. Mutable human-readable name for the domain. 63 character limit. e.g. `prod domain`. |


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
domain_name = domain.name
domain_disabled = domain.disabled
domain_labels = domain.labels
domain_delete_time = domain.delete_time
domain_etag = domain.etag
domain_type = domain.type
domain_create_time = domain.create_time
domain_reconciling = domain.reconciling
domain_serve = domain.serve
domain_update_time = domain.update_time
domain_custom_domain_status = domain.custom_domain_status
domain_uid = domain.uid
domain_annotations = domain.annotations
domain_display_name = domain.display_name
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


### Traffic

Gets information about a backend's traffic.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target` | String |  | Set to manually control the desired traffic for the backend. This will cause `current` to eventually match this value. The percentages must add up to 100%. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `rollout_policy` | String |  | A rollout policy specifies how new builds and automatic deployments are created. |
| `name` | String |  | Identifier. The resource name of the backend's traffic. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/traffic`. |
| `current` | String |  | Output only. Current state of traffic allocation for the backend. When setting `target`, this field may differ for some time until the desired state is reached. |
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the system is working to make the backend's `current` match the requested `target` list. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `create_time` | String |  | Output only. Time at which the backend was created. |
| `update_time` | String |  | Output only. Time at which the backend was last updated. |
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `name` | String | ✅ | Identifier. The resource name of the backend's traffic. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/traffic`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target` | String | Set to manually control the desired traffic for the backend. This will cause `current` to eventually match this value. The percentages must add up to 100%. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `rollout_policy` | String | A rollout policy specifies how new builds and automatic deployments are created. |
| `name` | String | Identifier. The resource name of the backend's traffic. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/traffic`. |
| `current` | String | Output only. Current state of traffic allocation for the backend. When setting `target`, this field may differ for some time until the desired state is reached. |
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the system is working to make the backend's `current` match the requested `target` list. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `create_time` | String | Output only. Time at which the backend was created. |
| `update_time` | String | Output only. Time at which the backend was last updated. |
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
traffic_target = traffic.target
traffic_etag = traffic.etag
traffic_rollout_policy = traffic.rollout_policy
traffic_name = traffic.name
traffic_current = traffic.current
traffic_annotations = traffic.annotations
traffic_reconciling = traffic.reconciling
traffic_uid = traffic.uid
traffic_create_time = traffic.create_time
traffic_update_time = traffic.update_time
traffic_labels = traffic.labels
```

---


### Rollout

Creates a new rollout for a backend.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `build` | String |  | Required. Immutable. The name of a build that already exists. It doesn't have to be built; a rollout will wait for a build to be ready before updating traffic. |
| `create_time` | String |  | Output only. Time at which the rollout was created. |
| `update_time` | String |  | Output only. Time at which the rollout was last updated. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `state` | String |  | Output only. The state of the rollout. |
| `display_name` | String |  | Optional. Human-readable name. 63 character limit. |
| `delete_time` | String |  | Output only. Time at which the rollout was deleted. |
| `error` | String |  | Output only. A status and (human readable) error message for the rollout, if in a `FAILED` state. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the Rollout currently has an LRO. |
| `name` | String |  | Identifier. The resource name of the rollout. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/rollouts/{rolloutId}`. |
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `parent` | String | ✅ | Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `build` | String | Required. Immutable. The name of a build that already exists. It doesn't have to be built; a rollout will wait for a build to be ready before updating traffic. |
| `create_time` | String | Output only. Time at which the rollout was created. |
| `update_time` | String | Output only. Time at which the rollout was last updated. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `state` | String | Output only. The state of the rollout. |
| `display_name` | String | Optional. Human-readable name. 63 character limit. |
| `delete_time` | String | Output only. Time at which the rollout was deleted. |
| `error` | String | Output only. A status and (human readable) error message for the rollout, if in a `FAILED` state. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the Rollout currently has an LRO. |
| `name` | String | Identifier. The resource name of the rollout. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/rollouts/{rolloutId}`. |
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |


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
rollout_build = rollout.build
rollout_create_time = rollout.create_time
rollout_update_time = rollout.update_time
rollout_etag = rollout.etag
rollout_uid = rollout.uid
rollout_labels = rollout.labels
rollout_state = rollout.state
rollout_display_name = rollout.display_name
rollout_delete_time = rollout.delete_time
rollout_error = rollout.error
rollout_reconciling = rollout.reconciling
rollout_name = rollout.name
rollout_annotations = rollout.annotations
```

---


### Backend

Creates a new backend in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delete_time` | String |  | Output only. Time at which the backend was deleted. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `update_time` | String |  | Output only. Time at which the backend was last updated. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `managed_resources` | Vec<String> |  | Output only. A list of the resources managed by this backend. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the system is working to make adjustments to the backend during a LRO. |
| `environment` | String |  | Optional. The environment name of the backend, used to load environment variables from environment specific configuration. |
| `app_id` | String |  | Optional. The [ID of a Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id) associated with the backend. |
| `service_account` | String |  | Required. The name of the service account used for Cloud Build and Cloud Run. Should have the role roles/firebaseapphosting.computeRunner or equivalent permissions. |
| `uri` | String |  | Output only. The primary URI to communicate with the backend. |
| `create_time` | String |  | Output only. Time at which the backend was created. |
| `display_name` | String |  | Optional. Human-readable name. 63 character limit. |
| `codebase` | String |  | Optional. If specified, the connection to an external source repository to watch for event-driven updates to the backend. |
| `mode` | String |  | Optional. Deprecated: Use `environment` instead. |
| `serving_locality` | String |  | Required. Immutable. Specifies how App Hosting will serve the content for this backend. It will either be contained to a single region (REGIONAL_STRICT) or allowed to use App Hosting's global-replicated serving infrastructure (GLOBAL_ACCESS). |
| `request_logs_disabled` | bool |  | Optional. A field that, if true, indicates that incoming request logs are disabled for this backend. Incoming request logs are enabled by default. |
| `name` | String |  | Identifier. The resource name of the backend. Format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `parent` | String | ✅ | Required. A parent name of the form `projects/{project}/locations/{locationId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delete_time` | String | Output only. Time at which the backend was deleted. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `update_time` | String | Output only. Time at which the backend was last updated. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `managed_resources` | Vec<String> | Output only. A list of the resources managed by this backend. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the system is working to make adjustments to the backend during a LRO. |
| `environment` | String | Optional. The environment name of the backend, used to load environment variables from environment specific configuration. |
| `app_id` | String | Optional. The [ID of a Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id) associated with the backend. |
| `service_account` | String | Required. The name of the service account used for Cloud Build and Cloud Run. Should have the role roles/firebaseapphosting.computeRunner or equivalent permissions. |
| `uri` | String | Output only. The primary URI to communicate with the backend. |
| `create_time` | String | Output only. Time at which the backend was created. |
| `display_name` | String | Optional. Human-readable name. 63 character limit. |
| `codebase` | String | Optional. If specified, the connection to an external source repository to watch for event-driven updates to the backend. |
| `mode` | String | Optional. Deprecated: Use `environment` instead. |
| `serving_locality` | String | Required. Immutable. Specifies how App Hosting will serve the content for this backend. It will either be contained to a single region (REGIONAL_STRICT) or allowed to use App Hosting's global-replicated serving infrastructure (GLOBAL_ACCESS). |
| `request_logs_disabled` | bool | Optional. A field that, if true, indicates that incoming request logs are disabled for this backend. Incoming request logs are enabled by default. |
| `name` | String | Identifier. The resource name of the backend. Format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. |


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
backend_delete_time = backend.delete_time
backend_uid = backend.uid
backend_update_time = backend.update_time
backend_etag = backend.etag
backend_annotations = backend.annotations
backend_managed_resources = backend.managed_resources
backend_reconciling = backend.reconciling
backend_environment = backend.environment
backend_app_id = backend.app_id
backend_service_account = backend.service_account
backend_uri = backend.uri
backend_create_time = backend.create_time
backend_display_name = backend.display_name
backend_codebase = backend.codebase
backend_mode = backend.mode
backend_serving_locality = backend.serving_locality
backend_request_logs_disabled = backend.request_logs_disabled
backend_name = backend.name
backend_labels = backend.labels
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_name = location.name
location_metadata = location.metadata
location_display_name = location.display_name
location_labels = location.labels
location_location_id = location.location_id
```

---


### Traffic

Gets information about a backend's traffic.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the system is working to make the backend's `current` match the requested `target` list. |
| `update_time` | String |  | Output only. Time at which the backend was last updated. |
| `rollout_policy` | String |  | A rollout policy specifies how new builds and automatic deployments are created. |
| `current` | String |  | Output only. Current state of traffic allocation for the backend. When setting `target`, this field may differ for some time until the desired state is reached. |
| `name` | String |  | Identifier. The resource name of the backend's traffic. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/traffic`. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `target` | String |  | Set to manually control the desired traffic for the backend. This will cause `current` to eventually match this value. The percentages must add up to 100%. |
| `create_time` | String |  | Output only. Time at which the backend was created. |
| `name` | String | ✅ | Identifier. The resource name of the backend's traffic. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/traffic`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the system is working to make the backend's `current` match the requested `target` list. |
| `update_time` | String | Output only. Time at which the backend was last updated. |
| `rollout_policy` | String | A rollout policy specifies how new builds and automatic deployments are created. |
| `current` | String | Output only. Current state of traffic allocation for the backend. When setting `target`, this field may differ for some time until the desired state is reached. |
| `name` | String | Identifier. The resource name of the backend's traffic. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/traffic`. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `target` | String | Set to manually control the desired traffic for the backend. This will cause `current` to eventually match this value. The percentages must add up to 100%. |
| `create_time` | String | Output only. Time at which the backend was created. |


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
traffic_reconciling = traffic.reconciling
traffic_update_time = traffic.update_time
traffic_rollout_policy = traffic.rollout_policy
traffic_current = traffic.current
traffic_name = traffic.name
traffic_uid = traffic.uid
traffic_etag = traffic.etag
traffic_labels = traffic.labels
traffic_target = traffic.target
traffic_create_time = traffic.create_time
```

---


### Domain

Links a new domain to a backend.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time at which the domain was created. |
| `annotations` | HashMap<String, String> |  | Optional. Annotations as key value pairs. |
| `name` | String |  | Identifier. The resource name of the domain, e.g. `/projects/p/locations/l/backends/b/domains/foo.com` |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `display_name` | String |  | Optional. Mutable human-readable name for the domain. 63 character limit. e.g. `prod domain`. |
| `custom_domain_status` | String |  | Output only. Represents the state and configuration of a `CUSTOM` type domain. It is only present on Domains of that type. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the build has an ongoing LRO. |
| `type` | String |  | Output only. The type of the domain. |
| `serve` | String |  | Optional. The serving behavior of the domain. If specified, the domain will serve content other than its backend's live content. |
| `disabled` | bool |  | Optional. Whether the domain is disabled. Defaults to false. |
| `delete_time` | String |  | Output only. Time at which the domain was deleted. |
| `purge_time` | String |  | Output only. Time at which a soft-deleted domain will be purged, rendering in permanently deleted. |
| `update_time` | String |  | Output only. Time at which the domain was last updated. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `parent` | String | ✅ | Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time at which the domain was created. |
| `annotations` | HashMap<String, String> | Optional. Annotations as key value pairs. |
| `name` | String | Identifier. The resource name of the domain, e.g. `/projects/p/locations/l/backends/b/domains/foo.com` |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `display_name` | String | Optional. Mutable human-readable name for the domain. 63 character limit. e.g. `prod domain`. |
| `custom_domain_status` | String | Output only. Represents the state and configuration of a `CUSTOM` type domain. It is only present on Domains of that type. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the build has an ongoing LRO. |
| `type` | String | Output only. The type of the domain. |
| `serve` | String | Optional. The serving behavior of the domain. If specified, the domain will serve content other than its backend's live content. |
| `disabled` | bool | Optional. Whether the domain is disabled. Defaults to false. |
| `delete_time` | String | Output only. Time at which the domain was deleted. |
| `purge_time` | String | Output only. Time at which a soft-deleted domain will be purged, rendering in permanently deleted. |
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
domain_create_time = domain.create_time
domain_annotations = domain.annotations
domain_name = domain.name
domain_labels = domain.labels
domain_etag = domain.etag
domain_display_name = domain.display_name
domain_custom_domain_status = domain.custom_domain_status
domain_reconciling = domain.reconciling
domain_type = domain.type
domain_serve = domain.serve
domain_disabled = domain.disabled
domain_delete_time = domain.delete_time
domain_purge_time = domain.purge_time
domain_update_time = domain.update_time
domain_uid = domain.uid
```

---


### Rollout

Creates a new rollout for a backend.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `build` | String |  | Required. Immutable. The name of a build that already exists. It doesn't have to be built; a rollout will wait for a build to be ready before updating traffic. |
| `state` | String |  | Output only. The state of the rollout. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `create_time` | String |  | Output only. Time at which the rollout was created. |
| `error` | String |  | Output only. A status and (human readable) error message for the rollout, if in a `FAILED` state. |
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `name` | String |  | Identifier. The resource name of the rollout. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/rollouts/{rolloutId}`. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the Rollout currently has an LRO. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `update_time` | String |  | Output only. Time at which the rollout was last updated. |
| `delete_time` | String |  | Output only. Time at which the rollout was deleted. |
| `display_name` | String |  | Optional. Human-readable name. 63 character limit. |
| `parent` | String | ✅ | Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `build` | String | Required. Immutable. The name of a build that already exists. It doesn't have to be built; a rollout will wait for a build to be ready before updating traffic. |
| `state` | String | Output only. The state of the rollout. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `create_time` | String | Output only. Time at which the rollout was created. |
| `error` | String | Output only. A status and (human readable) error message for the rollout, if in a `FAILED` state. |
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `name` | String | Identifier. The resource name of the rollout. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/rollouts/{rolloutId}`. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the Rollout currently has an LRO. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `update_time` | String | Output only. Time at which the rollout was last updated. |
| `delete_time` | String | Output only. Time at which the rollout was deleted. |
| `display_name` | String | Optional. Human-readable name. 63 character limit. |


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
rollout_labels = rollout.labels
rollout_build = rollout.build
rollout_state = rollout.state
rollout_etag = rollout.etag
rollout_create_time = rollout.create_time
rollout_error = rollout.error
rollout_annotations = rollout.annotations
rollout_name = rollout.name
rollout_reconciling = rollout.reconciling
rollout_uid = rollout.uid
rollout_update_time = rollout.update_time
rollout_delete_time = rollout.delete_time
rollout_display_name = rollout.display_name
```

---


### Build

Creates a new build for a backend.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The state of the build. |
| `delete_time` | String |  | Output only. Time at which the build was deleted. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `error_source` | String |  | Output only. The source of the error for the build, if in a `FAILED` state. Deprecated. Use `errors` instead. |
| `build_logs_uri` | String |  | Output only. The location of the [Cloud Build logs](https://cloud.google.com/build/docs/view-build-results) for the build process. |
| `errors` | Vec<String> |  | Output only. A list of all errors that occurred during an App Hosting build. |
| `display_name` | String |  | Optional. Human-readable name. 63 character limit. |
| `environment` | String |  | Output only. The environment name of the backend when this build was created. |
| `image` | String |  | Output only. The Artifact Registry [container image](https://cloud.google.com/artifact-registry/docs/reference/rest/v1/projects.locations.repositories.dockerImages) URI, used by the Cloud Run [`revision`](https://cloud.google.com/run/docs/reference/rest/v2/projects.locations.services.revisions) for this build. |
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `source` | String |  | Required. Immutable. The source for the build. |
| `update_time` | String |  | Output only. Time at which the build was last updated. |
| `name` | String |  | Identifier. The resource name of the build. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/builds/{buildId}`. |
| `error` | String |  | Output only. A status and (human readable) error message for the build, if in a `FAILED` state. Deprecated. Use `errors` instead. |
| `create_time` | String |  | Output only. Time at which the build was created. |
| `config` | String |  | Optional. Additional configuration of the service. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the build has an ongoing LRO. |
| `parent` | String | ✅ | Required. The parent backend in the format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The state of the build. |
| `delete_time` | String | Output only. Time at which the build was deleted. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `error_source` | String | Output only. The source of the error for the build, if in a `FAILED` state. Deprecated. Use `errors` instead. |
| `build_logs_uri` | String | Output only. The location of the [Cloud Build logs](https://cloud.google.com/build/docs/view-build-results) for the build process. |
| `errors` | Vec<String> | Output only. A list of all errors that occurred during an App Hosting build. |
| `display_name` | String | Optional. Human-readable name. 63 character limit. |
| `environment` | String | Output only. The environment name of the backend when this build was created. |
| `image` | String | Output only. The Artifact Registry [container image](https://cloud.google.com/artifact-registry/docs/reference/rest/v1/projects.locations.repositories.dockerImages) URI, used by the Cloud Run [`revision`](https://cloud.google.com/run/docs/reference/rest/v2/projects.locations.services.revisions) for this build. |
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `source` | String | Required. Immutable. The source for the build. |
| `update_time` | String | Output only. Time at which the build was last updated. |
| `name` | String | Identifier. The resource name of the build. Format: `projects/{project}/locations/{locationId}/backends/{backendId}/builds/{buildId}`. |
| `error` | String | Output only. A status and (human readable) error message for the build, if in a `FAILED` state. Deprecated. Use `errors` instead. |
| `create_time` | String | Output only. Time at which the build was created. |
| `config` | String | Optional. Additional configuration of the service. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the build has an ongoing LRO. |


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
build_state = build.state
build_delete_time = build.delete_time
build_etag = build.etag
build_error_source = build.error_source
build_build_logs_uri = build.build_logs_uri
build_errors = build.errors
build_display_name = build.display_name
build_environment = build.environment
build_image = build.image
build_labels = build.labels
build_annotations = build.annotations
build_uid = build.uid
build_source = build.source
build_update_time = build.update_time
build_name = build.name
build_error = build.error
build_create_time = build.create_time
build_config = build.config
build_reconciling = build.reconciling
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


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
operation_response = operation.response
operation_name = operation.name
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
```

---


### Backend

Creates a new backend in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `codebase` | String |  | Optional. If specified, the connection to an external source repository to watch for event-driven updates to the backend. |
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `update_time` | String |  | Output only. Time at which the backend was last updated. |
| `name` | String |  | Identifier. The resource name of the backend. Format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |
| `mode` | String |  | Optional. Deprecated: Use `environment` instead. |
| `override_env` | Vec<String> |  | Optional. Override environment variables for this Backend. |
| `environment` | String |  | Optional. The environment name of the backend, used to load environment variables from environment specific configuration. |
| `service_account` | String |  | Required. The name of the service account used for Cloud Build and Cloud Run. Should have the role roles/firebaseapphosting.computeRunner or equivalent permissions. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `serving_locality` | String |  | Required. Immutable. Specifies how App Hosting will serve the content for this backend. It will either be contained to a single region (REGIONAL_STRICT) or allowed to use App Hosting's global-replicated serving infrastructure (GLOBAL_ACCESS). |
| `managed_resources` | Vec<String> |  | Output only. A list of the resources managed by this backend. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that the system is working to make adjustments to the backend during a LRO. |
| `app_id` | String |  | Optional. The [ID of a Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id) associated with the backend. |
| `display_name` | String |  | Optional. Human-readable name. 63 character limit. |
| `request_logs_disabled` | bool |  | Optional. A field that, if true, indicates that incoming request logs are disabled for this backend. Incoming request logs are enabled by default. |
| `uri` | String |  | Output only. The primary URI to communicate with the backend. |
| `create_time` | String |  | Output only. Time at which the backend was created. |
| `delete_time` | String |  | Output only. Time at which the backend was deleted. |
| `etag` | String |  | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |
| `parent` | String | ✅ | Required. A parent name of the form `projects/{project}/locations/{locationId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `codebase` | String | Optional. If specified, the connection to an external source repository to watch for event-driven updates to the backend. |
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `update_time` | String | Output only. Time at which the backend was last updated. |
| `name` | String | Identifier. The resource name of the backend. Format: `projects/{project}/locations/{locationId}/backends/{backendId}`. |
| `mode` | String | Optional. Deprecated: Use `environment` instead. |
| `override_env` | Vec<String> | Optional. Override environment variables for this Backend. |
| `environment` | String | Optional. The environment name of the backend, used to load environment variables from environment specific configuration. |
| `service_account` | String | Required. The name of the service account used for Cloud Build and Cloud Run. Should have the role roles/firebaseapphosting.computeRunner or equivalent permissions. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. |
| `serving_locality` | String | Required. Immutable. Specifies how App Hosting will serve the content for this backend. It will either be contained to a single region (REGIONAL_STRICT) or allowed to use App Hosting's global-replicated serving infrastructure (GLOBAL_ACCESS). |
| `managed_resources` | Vec<String> | Output only. A list of the resources managed by this backend. |
| `reconciling` | bool | Output only. A field that, if true, indicates that the system is working to make adjustments to the backend during a LRO. |
| `app_id` | String | Optional. The [ID of a Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id) associated with the backend. |
| `display_name` | String | Optional. Human-readable name. 63 character limit. |
| `request_logs_disabled` | bool | Optional. A field that, if true, indicates that incoming request logs are disabled for this backend. Incoming request logs are enabled by default. |
| `uri` | String | Output only. The primary URI to communicate with the backend. |
| `create_time` | String | Output only. Time at which the backend was created. |
| `delete_time` | String | Output only. Time at which the backend was deleted. |
| `etag` | String | Output only. Server-computed checksum based on other values; may be sent on update or delete to ensure operation is done on expected resource. |


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
backend_codebase = backend.codebase
backend_annotations = backend.annotations
backend_update_time = backend.update_time
backend_name = backend.name
backend_mode = backend.mode
backend_override_env = backend.override_env
backend_environment = backend.environment
backend_service_account = backend.service_account
backend_uid = backend.uid
backend_labels = backend.labels
backend_serving_locality = backend.serving_locality
backend_managed_resources = backend.managed_resources
backend_reconciling = backend.reconciling
backend_app_id = backend.app_id
backend_display_name = backend.display_name
backend_request_logs_disabled = backend.request_logs_disabled
backend_uri = backend.uri
backend_create_time = backend.create_time
backend_delete_time = backend.delete_time
backend_etag = backend.etag
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple build resources
build_0 = provider.firebaseapphosting_api.Build {
    parent = "value-0"
}
build_1 = provider.firebaseapphosting_api.Build {
    parent = "value-1"
}
build_2 = provider.firebaseapphosting_api.Build {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    build = provider.firebaseapphosting_api.Build {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Firebaseapphosting_api Documentation](https://cloud.google.com/firebaseapphosting_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
