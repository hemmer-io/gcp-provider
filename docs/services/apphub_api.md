# Apphub_api Service



**Resources**: 16

---

## Overview

The apphub_api service provides access to 16 resource types:

- [Operation](#operation) [CRD]
- [Location](#location) [CR]
- [Application](#application) [CRUD]
- [Discovered_workload](#discovered_workload) [R]
- [Discovered_service](#discovered_service) [R]
- [Workload](#workload) [CRUD]
- [Service](#service) [CRUD]
- [Service_project_attachment](#service_project_attachment) [CRD]
- [Location](#location) [CR]
- [Service](#service) [CRUD]
- [Service_project_attachment](#service_project_attachment) [CRD]
- [Discovered_service](#discovered_service) [R]
- [Workload](#workload) [CRUD]
- [Application](#application) [CRUD]
- [Discovered_workload](#discovered_workload) [R]
- [Operation](#operation) [CRD]

---

## Resources


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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.apphub_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_error = operation.error
operation_metadata = operation.metadata
operation_response = operation.response
operation_done = operation.done
```

---


### Location

Detaches a service project from a host project. You can call this API from any service project without needing access to the host project that it is attached to.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. Service project id and location to detach from a host project. Only global location is supported. Expected format: `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.apphub_api.Location {
    name = "value"  # Required. Service project id and location to detach from a host project. Only global location is supported. Expected format: `projects/{project}/locations/{location}`.
}

# Access location outputs
location_id = location.id
location_labels = location.labels
location_display_name = location.display_name
location_location_id = location.location_id
location_name = location.name
location_metadata = location.metadata
```

---


### Application

Creates an Application in a host project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Optional. User-defined name for the Application. Can have a maximum length of 63 characters. |
| `state` | String |  | Output only. Application state. |
| `create_time` | String |  | Output only. Create time. |
| `name` | String |  | Identifier. The resource name of an Application. Format: `"projects/{host-project-id}/locations/{location}/applications/{application-id}"` |
| `update_time` | String |  | Output only. Update time. |
| `scope` | String |  | Required. Immutable. Defines what data can be included into this Application. Limits which Services and Workloads can be registered. |
| `description` | String |  | Optional. User-defined description of an Application. Can have a maximum length of 2048 characters. |
| `attributes` | String |  | Optional. Consumer provided attributes. |
| `uid` | String |  | Output only. A universally unique identifier (in UUID4 format) for the `Application`. |
| `parent` | String | ✅ | Required. Project and location to create Application in. Expected format: `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Optional. User-defined name for the Application. Can have a maximum length of 63 characters. |
| `state` | String | Output only. Application state. |
| `create_time` | String | Output only. Create time. |
| `name` | String | Identifier. The resource name of an Application. Format: `"projects/{host-project-id}/locations/{location}/applications/{application-id}"` |
| `update_time` | String | Output only. Update time. |
| `scope` | String | Required. Immutable. Defines what data can be included into this Application. Limits which Services and Workloads can be registered. |
| `description` | String | Optional. User-defined description of an Application. Can have a maximum length of 2048 characters. |
| `attributes` | String | Optional. Consumer provided attributes. |
| `uid` | String | Output only. A universally unique identifier (in UUID4 format) for the `Application`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create application
application = provider.apphub_api.Application {
    parent = "value"  # Required. Project and location to create Application in. Expected format: `projects/{project}/locations/{location}`.
}

# Access application outputs
application_id = application.id
application_display_name = application.display_name
application_state = application.state
application_create_time = application.create_time
application_name = application.name
application_update_time = application.update_time
application_scope = application.scope
application_description = application.description
application_attributes = application.attributes
application_uid = application.uid
```

---


### Discovered_workload

Gets a Discovered Workload in a host project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workload_reference` | String | Output only. Reference of an underlying compute resource represented by the Workload. These are immutable. |
| `workload_properties` | String | Output only. Properties of an underlying compute resource represented by the Workload. These are immutable. |
| `name` | String | Identifier. The resource name of the discovered workload. Format: `"projects/{host-project-id}/locations/{location}/discoveredWorkloads/{uuid}"` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access discovered_workload outputs
discovered_workload_id = discovered_workload.id
discovered_workload_workload_reference = discovered_workload.workload_reference
discovered_workload_workload_properties = discovered_workload.workload_properties
discovered_workload_name = discovered_workload.name
```

---


### Discovered_service

Gets a Discovered Service in a host project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_reference` | String | Output only. Reference to an underlying networking resource that can comprise a Service. These are immutable. |
| `service_properties` | String | Output only. Properties of an underlying compute resource that can comprise a Service. These are immutable. |
| `name` | String | Identifier. The resource name of the discovered service. Format: `"projects/{host-project-id}/locations/{location}/discoveredServices/{uuid}"` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access discovered_service outputs
discovered_service_id = discovered_service.id
discovered_service_service_reference = discovered_service.service_reference
discovered_service_service_properties = discovered_service.service_properties
discovered_service_name = discovered_service.name
```

---


### Workload

Creates a Workload in an Application.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. User-defined description of a Workload. Can have a maximum length of 2048 characters. |
| `workload_properties` | String |  | Output only. Properties of an underlying compute resource represented by the Workload. These are immutable. |
| `uid` | String |  | Output only. A universally unique identifier (UUID) for the `Workload` in the UUID4 format. |
| `state` | String |  | Output only. Workload state. |
| `workload_reference` | String |  | Output only. Reference of an underlying compute resource represented by the Workload. These are immutable. |
| `update_time` | String |  | Output only. Update time. |
| `attributes` | String |  | Optional. Consumer provided attributes. |
| `discovered_workload` | String |  | Required. Immutable. The resource name of the original discovered workload. |
| `create_time` | String |  | Output only. Create time. |
| `display_name` | String |  | Optional. User-defined name for the Workload. Can have a maximum length of 63 characters. |
| `name` | String |  | Identifier. The resource name of the Workload. Format: `"projects/{host-project-id}/locations/{location}/applications/{application-id}/workloads/{workload-id}"` |
| `parent` | String | ✅ | Required. Fully qualified name of the Application to create Workload in. Expected format: `projects/{project}/locations/{location}/applications/{application}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. User-defined description of a Workload. Can have a maximum length of 2048 characters. |
| `workload_properties` | String | Output only. Properties of an underlying compute resource represented by the Workload. These are immutable. |
| `uid` | String | Output only. A universally unique identifier (UUID) for the `Workload` in the UUID4 format. |
| `state` | String | Output only. Workload state. |
| `workload_reference` | String | Output only. Reference of an underlying compute resource represented by the Workload. These are immutable. |
| `update_time` | String | Output only. Update time. |
| `attributes` | String | Optional. Consumer provided attributes. |
| `discovered_workload` | String | Required. Immutable. The resource name of the original discovered workload. |
| `create_time` | String | Output only. Create time. |
| `display_name` | String | Optional. User-defined name for the Workload. Can have a maximum length of 63 characters. |
| `name` | String | Identifier. The resource name of the Workload. Format: `"projects/{host-project-id}/locations/{location}/applications/{application-id}/workloads/{workload-id}"` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workload
workload = provider.apphub_api.Workload {
    parent = "value"  # Required. Fully qualified name of the Application to create Workload in. Expected format: `projects/{project}/locations/{location}/applications/{application}`.
}

# Access workload outputs
workload_id = workload.id
workload_description = workload.description
workload_workload_properties = workload.workload_properties
workload_uid = workload.uid
workload_state = workload.state
workload_workload_reference = workload.workload_reference
workload_update_time = workload.update_time
workload_attributes = workload.attributes
workload_discovered_workload = workload.discovered_workload
workload_create_time = workload.create_time
workload_display_name = workload.display_name
workload_name = workload.name
```

---


### Service

Creates a Service in an Application.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Update time. |
| `attributes` | String |  | Optional. Consumer provided attributes. |
| `description` | String |  | Optional. User-defined description of a Service. Can have a maximum length of 2048 characters. |
| `service_reference` | String |  | Output only. Reference to an underlying networking resource that can comprise a Service. These are immutable. |
| `uid` | String |  | Output only. A universally unique identifier (UUID) for the `Service` in the UUID4 format. |
| `display_name` | String |  | Optional. User-defined name for the Service. Can have a maximum length of 63 characters. |
| `service_properties` | String |  | Output only. Properties of an underlying compute resource that can comprise a Service. These are immutable. |
| `state` | String |  | Output only. Service state. |
| `create_time` | String |  | Output only. Create time. |
| `name` | String |  | Identifier. The resource name of a Service. Format: `"projects/{host-project-id}/locations/{location}/applications/{application-id}/services/{service-id}"` |
| `discovered_service` | String |  | Required. Immutable. The resource name of the original discovered service. |
| `parent` | String | ✅ | Required. Fully qualified name of the parent Application to create the Service in. Expected format: `projects/{project}/locations/{location}/applications/{application}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Update time. |
| `attributes` | String | Optional. Consumer provided attributes. |
| `description` | String | Optional. User-defined description of a Service. Can have a maximum length of 2048 characters. |
| `service_reference` | String | Output only. Reference to an underlying networking resource that can comprise a Service. These are immutable. |
| `uid` | String | Output only. A universally unique identifier (UUID) for the `Service` in the UUID4 format. |
| `display_name` | String | Optional. User-defined name for the Service. Can have a maximum length of 63 characters. |
| `service_properties` | String | Output only. Properties of an underlying compute resource that can comprise a Service. These are immutable. |
| `state` | String | Output only. Service state. |
| `create_time` | String | Output only. Create time. |
| `name` | String | Identifier. The resource name of a Service. Format: `"projects/{host-project-id}/locations/{location}/applications/{application-id}/services/{service-id}"` |
| `discovered_service` | String | Required. Immutable. The resource name of the original discovered service. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.apphub_api.Service {
    parent = "value"  # Required. Fully qualified name of the parent Application to create the Service in. Expected format: `projects/{project}/locations/{location}/applications/{application}`.
}

# Access service outputs
service_id = service.id
service_update_time = service.update_time
service_attributes = service.attributes
service_description = service.description
service_service_reference = service.service_reference
service_uid = service.uid
service_display_name = service.display_name
service_service_properties = service.service_properties
service_state = service.state
service_create_time = service.create_time
service_name = service.name
service_discovered_service = service.discovered_service
```

---


### Service_project_attachment

Attaches a service project to the host project.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of a ServiceProjectAttachment. Format: `"projects/{host-project-id}/locations/global/serviceProjectAttachments/{service-project-id}."` |
| `service_project` | String |  | Required. Immutable. Service project name in the format: `"projects/abc"` or `"projects/123"`. As input, project name with either project id or number are accepted. As output, this field will contain project number. |
| `create_time` | String |  | Output only. Create time. |
| `state` | String |  | Output only. ServiceProjectAttachment state. |
| `uid` | String |  | Output only. A globally unique identifier (in UUID4 format) for the `ServiceProjectAttachment`. |
| `parent` | String | ✅ | Required. Host project ID and location to which service project is being attached. Only global location is supported. Expected format: `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of a ServiceProjectAttachment. Format: `"projects/{host-project-id}/locations/global/serviceProjectAttachments/{service-project-id}."` |
| `service_project` | String | Required. Immutable. Service project name in the format: `"projects/abc"` or `"projects/123"`. As input, project name with either project id or number are accepted. As output, this field will contain project number. |
| `create_time` | String | Output only. Create time. |
| `state` | String | Output only. ServiceProjectAttachment state. |
| `uid` | String | Output only. A globally unique identifier (in UUID4 format) for the `ServiceProjectAttachment`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_project_attachment
service_project_attachment = provider.apphub_api.Service_project_attachment {
    parent = "value"  # Required. Host project ID and location to which service project is being attached. Only global location is supported. Expected format: `projects/{project}/locations/{location}`.
}

# Access service_project_attachment outputs
service_project_attachment_id = service_project_attachment.id
service_project_attachment_name = service_project_attachment.name
service_project_attachment_service_project = service_project_attachment.service_project
service_project_attachment_create_time = service_project_attachment.create_time
service_project_attachment_state = service_project_attachment.state
service_project_attachment_uid = service_project_attachment.uid
```

---


### Location

Detaches a service project from a host project. You can call this API from any service project without needing access to the host project that it is attached to.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. Service project id and location to detach from a host project. Only global location is supported. Expected format: `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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

# Create location
location = provider.apphub_api.Location {
    name = "value"  # Required. Service project id and location to detach from a host project. Only global location is supported. Expected format: `projects/{project}/locations/{location}`.
}

# Access location outputs
location_id = location.id
location_display_name = location.display_name
location_labels = location.labels
location_name = location.name
location_location_id = location.location_id
location_metadata = location.metadata
```

---


### Service

Creates a Service in an Application.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attributes` | String |  | Optional. Consumer provided attributes. |
| `description` | String |  | Optional. User-defined description of a Service. Can have a maximum length of 2048 characters. |
| `service_reference` | String |  | Output only. Reference to an underlying networking resource that can comprise a Service. These are immutable. |
| `state` | String |  | Output only. Service state. |
| `service_properties` | String |  | Output only. Properties of an underlying compute resource that can comprise a Service. These are immutable. |
| `create_time` | String |  | Output only. Create time. |
| `discovered_service` | String |  | Required. Immutable. The resource name of the original discovered service. |
| `display_name` | String |  | Optional. User-defined name for the Service. Can have a maximum length of 63 characters. |
| `uid` | String |  | Output only. A universally unique identifier (UUID) for the `Service` in the UUID4 format. |
| `name` | String |  | Identifier. The resource name of a Service. Format: `"projects/{host-project-id}/locations/{location}/applications/{application-id}/services/{service-id}"` |
| `update_time` | String |  | Output only. Update time. |
| `parent` | String | ✅ | Required. Fully qualified name of the parent Application to create the Service in. Expected format: `projects/{project}/locations/{location}/applications/{application}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attributes` | String | Optional. Consumer provided attributes. |
| `description` | String | Optional. User-defined description of a Service. Can have a maximum length of 2048 characters. |
| `service_reference` | String | Output only. Reference to an underlying networking resource that can comprise a Service. These are immutable. |
| `state` | String | Output only. Service state. |
| `service_properties` | String | Output only. Properties of an underlying compute resource that can comprise a Service. These are immutable. |
| `create_time` | String | Output only. Create time. |
| `discovered_service` | String | Required. Immutable. The resource name of the original discovered service. |
| `display_name` | String | Optional. User-defined name for the Service. Can have a maximum length of 63 characters. |
| `uid` | String | Output only. A universally unique identifier (UUID) for the `Service` in the UUID4 format. |
| `name` | String | Identifier. The resource name of a Service. Format: `"projects/{host-project-id}/locations/{location}/applications/{application-id}/services/{service-id}"` |
| `update_time` | String | Output only. Update time. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.apphub_api.Service {
    parent = "value"  # Required. Fully qualified name of the parent Application to create the Service in. Expected format: `projects/{project}/locations/{location}/applications/{application}`.
}

# Access service outputs
service_id = service.id
service_attributes = service.attributes
service_description = service.description
service_service_reference = service.service_reference
service_state = service.state
service_service_properties = service.service_properties
service_create_time = service.create_time
service_discovered_service = service.discovered_service
service_display_name = service.display_name
service_uid = service.uid
service_name = service.name
service_update_time = service.update_time
```

---


### Service_project_attachment

Attaches a service project to the host project.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. ServiceProjectAttachment state. |
| `service_project` | String |  | Required. Immutable. Service project name in the format: `"projects/abc"` or `"projects/123"`. As input, project name with either project id or number are accepted. As output, this field will contain project number. |
| `create_time` | String |  | Output only. Create time. |
| `uid` | String |  | Output only. A globally unique identifier (in UUID4 format) for the `ServiceProjectAttachment`. |
| `name` | String |  | Identifier. The resource name of a ServiceProjectAttachment. Format: `"projects/{host-project-id}/locations/global/serviceProjectAttachments/{service-project-id}."` |
| `parent` | String | ✅ | Required. Host project ID and location to which service project is being attached. Only global location is supported. Expected format: `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. ServiceProjectAttachment state. |
| `service_project` | String | Required. Immutable. Service project name in the format: `"projects/abc"` or `"projects/123"`. As input, project name with either project id or number are accepted. As output, this field will contain project number. |
| `create_time` | String | Output only. Create time. |
| `uid` | String | Output only. A globally unique identifier (in UUID4 format) for the `ServiceProjectAttachment`. |
| `name` | String | Identifier. The resource name of a ServiceProjectAttachment. Format: `"projects/{host-project-id}/locations/global/serviceProjectAttachments/{service-project-id}."` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_project_attachment
service_project_attachment = provider.apphub_api.Service_project_attachment {
    parent = "value"  # Required. Host project ID and location to which service project is being attached. Only global location is supported. Expected format: `projects/{project}/locations/{location}`.
}

# Access service_project_attachment outputs
service_project_attachment_id = service_project_attachment.id
service_project_attachment_state = service_project_attachment.state
service_project_attachment_service_project = service_project_attachment.service_project
service_project_attachment_create_time = service_project_attachment.create_time
service_project_attachment_uid = service_project_attachment.uid
service_project_attachment_name = service_project_attachment.name
```

---


### Discovered_service

Gets a Discovered Service in a host project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_reference` | String | Output only. Reference to an underlying networking resource that can comprise a Service. These are immutable. |
| `name` | String | Identifier. The resource name of the discovered service. Format: `"projects/{host-project-id}/locations/{location}/discoveredServices/{uuid}"` |
| `service_properties` | String | Output only. Properties of an underlying compute resource that can comprise a Service. These are immutable. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access discovered_service outputs
discovered_service_id = discovered_service.id
discovered_service_service_reference = discovered_service.service_reference
discovered_service_name = discovered_service.name
discovered_service_service_properties = discovered_service.service_properties
```

---


### Workload

Creates a Workload in an Application.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `discovered_workload` | String |  | Required. Immutable. The resource name of the original discovered workload. |
| `name` | String |  | Identifier. The resource name of the Workload. Format: `"projects/{host-project-id}/locations/{location}/applications/{application-id}/workloads/{workload-id}"` |
| `uid` | String |  | Output only. A universally unique identifier (UUID) for the `Workload` in the UUID4 format. |
| `workload_properties` | String |  | Output only. Properties of an underlying compute resource represented by the Workload. These are immutable. |
| `description` | String |  | Optional. User-defined description of a Workload. Can have a maximum length of 2048 characters. |
| `update_time` | String |  | Output only. Update time. |
| `create_time` | String |  | Output only. Create time. |
| `workload_reference` | String |  | Output only. Reference of an underlying compute resource represented by the Workload. These are immutable. |
| `display_name` | String |  | Optional. User-defined name for the Workload. Can have a maximum length of 63 characters. |
| `state` | String |  | Output only. Workload state. |
| `attributes` | String |  | Optional. Consumer provided attributes. |
| `parent` | String | ✅ | Required. Fully qualified name of the Application to create Workload in. Expected format: `projects/{project}/locations/{location}/applications/{application}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `discovered_workload` | String | Required. Immutable. The resource name of the original discovered workload. |
| `name` | String | Identifier. The resource name of the Workload. Format: `"projects/{host-project-id}/locations/{location}/applications/{application-id}/workloads/{workload-id}"` |
| `uid` | String | Output only. A universally unique identifier (UUID) for the `Workload` in the UUID4 format. |
| `workload_properties` | String | Output only. Properties of an underlying compute resource represented by the Workload. These are immutable. |
| `description` | String | Optional. User-defined description of a Workload. Can have a maximum length of 2048 characters. |
| `update_time` | String | Output only. Update time. |
| `create_time` | String | Output only. Create time. |
| `workload_reference` | String | Output only. Reference of an underlying compute resource represented by the Workload. These are immutable. |
| `display_name` | String | Optional. User-defined name for the Workload. Can have a maximum length of 63 characters. |
| `state` | String | Output only. Workload state. |
| `attributes` | String | Optional. Consumer provided attributes. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workload
workload = provider.apphub_api.Workload {
    parent = "value"  # Required. Fully qualified name of the Application to create Workload in. Expected format: `projects/{project}/locations/{location}/applications/{application}`.
}

# Access workload outputs
workload_id = workload.id
workload_discovered_workload = workload.discovered_workload
workload_name = workload.name
workload_uid = workload.uid
workload_workload_properties = workload.workload_properties
workload_description = workload.description
workload_update_time = workload.update_time
workload_create_time = workload.create_time
workload_workload_reference = workload.workload_reference
workload_display_name = workload.display_name
workload_state = workload.state
workload_attributes = workload.attributes
```

---


### Application

Creates an Application in a host project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attributes` | String |  | Optional. Consumer provided attributes. |
| `description` | String |  | Optional. User-defined description of an Application. Can have a maximum length of 2048 characters. |
| `state` | String |  | Output only. Application state. |
| `name` | String |  | Identifier. The resource name of an Application. Format: `"projects/{host-project-id}/locations/{location}/applications/{application-id}"` |
| `display_name` | String |  | Optional. User-defined name for the Application. Can have a maximum length of 63 characters. |
| `uid` | String |  | Output only. A universally unique identifier (in UUID4 format) for the `Application`. |
| `update_time` | String |  | Output only. Update time. |
| `create_time` | String |  | Output only. Create time. |
| `scope` | String |  | Required. Immutable. Defines what data can be included into this Application. Limits which Services and Workloads can be registered. |
| `parent` | String | ✅ | Required. Project and location to create Application in. Expected format: `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attributes` | String | Optional. Consumer provided attributes. |
| `description` | String | Optional. User-defined description of an Application. Can have a maximum length of 2048 characters. |
| `state` | String | Output only. Application state. |
| `name` | String | Identifier. The resource name of an Application. Format: `"projects/{host-project-id}/locations/{location}/applications/{application-id}"` |
| `display_name` | String | Optional. User-defined name for the Application. Can have a maximum length of 63 characters. |
| `uid` | String | Output only. A universally unique identifier (in UUID4 format) for the `Application`. |
| `update_time` | String | Output only. Update time. |
| `create_time` | String | Output only. Create time. |
| `scope` | String | Required. Immutable. Defines what data can be included into this Application. Limits which Services and Workloads can be registered. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create application
application = provider.apphub_api.Application {
    parent = "value"  # Required. Project and location to create Application in. Expected format: `projects/{project}/locations/{location}`.
}

# Access application outputs
application_id = application.id
application_attributes = application.attributes
application_description = application.description
application_state = application.state
application_name = application.name
application_display_name = application.display_name
application_uid = application.uid
application_update_time = application.update_time
application_create_time = application.create_time
application_scope = application.scope
```

---


### Discovered_workload

Gets a Discovered Workload in a host project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workload_properties` | String | Output only. Properties of an underlying compute resource represented by the Workload. These are immutable. |
| `workload_reference` | String | Output only. Reference of an underlying compute resource represented by the Workload. These are immutable. |
| `name` | String | Identifier. The resource name of the discovered workload. Format: `"projects/{host-project-id}/locations/{location}/discoveredWorkloads/{uuid}"` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access discovered_workload outputs
discovered_workload_id = discovered_workload.id
discovered_workload_workload_properties = discovered_workload.workload_properties
discovered_workload_workload_reference = discovered_workload.workload_reference
discovered_workload_name = discovered_workload.name
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation = provider.apphub_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
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

# Create multiple operation resources
operation_0 = provider.apphub_api.Operation {
    name = "value-0"
}
operation_1 = provider.apphub_api.Operation {
    name = "value-1"
}
operation_2 = provider.apphub_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.apphub_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Apphub_api Documentation](https://cloud.google.com/apphub_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
