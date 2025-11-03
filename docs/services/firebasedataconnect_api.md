# Firebasedataconnect_api Service



**Resources**: 10

---

## Overview

The firebasedataconnect_api service provides access to 10 resource types:

- [Location](#location) [R]
- [Schema](#schema) [CRUD]
- [Service](#service) [CRUD]
- [Connector](#connector) [CRUD]
- [Operation](#operation) [CRD]
- [Connector](#connector) [CRUD]
- [Service](#service) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CRD]
- [Schema](#schema) [CRUD]

---

## Resources


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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
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
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
location_location_id = location.location_id
```

---


### Schema

Creates a new Schema in a given project and location. Only creation of `schemas/main` is supported and calling create with any other schema ID will result in an error.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. [Output only] Create time stamp. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `update_time` | String |  | Output only. [Output only] Update time stamp. |
| `datasources` | Vec<String> |  | Required. The data sources linked in the schema. |
| `reconciling` | bool |  | Output only. A field that if true, indicates that the system is working to compile and deploy the schema. |
| `name` | String |  | Identifier. The relative resource name of the schema, in the format: ``` projects/{project}/locations/{location}/services/{service}/schemas/{schema} ``` Right now, the only supported schema is "main". |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. [AIP-154](https://google.aip.dev/154) |
| `display_name` | String |  | Optional. Mutable human-readable name. 63 character limit. |
| `annotations` | HashMap<String, String> |  | Optional. Stores small amounts of arbitrary data. |
| `source` | String |  | Required. The source files that comprise the application schema. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. [Output only] Create time stamp. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `update_time` | String | Output only. [Output only] Update time stamp. |
| `datasources` | Vec<String> | Required. The data sources linked in the schema. |
| `reconciling` | bool | Output only. A field that if true, indicates that the system is working to compile and deploy the schema. |
| `name` | String | Identifier. The relative resource name of the schema, in the format: ``` projects/{project}/locations/{location}/services/{service}/schemas/{schema} ``` Right now, the only supported schema is "main". |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. [AIP-154](https://google.aip.dev/154) |
| `display_name` | String | Optional. Mutable human-readable name. 63 character limit. |
| `annotations` | HashMap<String, String> | Optional. Stores small amounts of arbitrary data. |
| `source` | String | Required. The source files that comprise the application schema. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create schema
schema = provider.firebasedataconnect_api.Schema {
    parent = "value"  # Required. Value for parent.
}

# Access schema outputs
schema_id = schema.id
schema_create_time = schema.create_time
schema_labels = schema.labels
schema_uid = schema.uid
schema_update_time = schema.update_time
schema_datasources = schema.datasources
schema_reconciling = schema.reconciling
schema_name = schema.name
schema_etag = schema.etag
schema_display_name = schema.display_name
schema_annotations = schema.annotations
schema_source = schema.source
```

---


### Service

Creates a new Service in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The relative resource name of the Firebase Data Connect service, in the format: ``` projects/{project}/locations/{location}/services/{service} ``` Note that the service ID is specific to Firebase Data Connect and does not correspond to any of the instance IDs of the underlying data source connections. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. [AIP-154](https://google.aip.dev/154) |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `create_time` | String |  | Output only. [Output only] Create time stamp. |
| `reconciling` | bool |  | Output only. A field that if true, indicates that the system is working update the service. |
| `annotations` | HashMap<String, String> |  | Optional. Stores small amounts of arbitrary data. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `display_name` | String |  | Optional. Mutable human-readable name. 63 character limit. |
| `update_time` | String |  | Output only. [Output only] Update time stamp. |
| `parent` | String | ✅ | Required. Value of parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The relative resource name of the Firebase Data Connect service, in the format: ``` projects/{project}/locations/{location}/services/{service} ``` Note that the service ID is specific to Firebase Data Connect and does not correspond to any of the instance IDs of the underlying data source connections. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. [AIP-154](https://google.aip.dev/154) |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `create_time` | String | Output only. [Output only] Create time stamp. |
| `reconciling` | bool | Output only. A field that if true, indicates that the system is working update the service. |
| `annotations` | HashMap<String, String> | Optional. Stores small amounts of arbitrary data. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `display_name` | String | Optional. Mutable human-readable name. 63 character limit. |
| `update_time` | String | Output only. [Output only] Update time stamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.firebasedataconnect_api.Service {
    parent = "value"  # Required. Value of parent.
}

# Access service outputs
service_id = service.id
service_name = service.name
service_etag = service.etag
service_labels = service.labels
service_create_time = service.create_time
service_reconciling = service.reconciling
service_annotations = service.annotations
service_uid = service.uid
service_display_name = service.display_name
service_update_time = service.update_time
```

---


### Connector

Creates a new Connector in a given project and location. The operations are validated against and must be compatible with the active schema. If the operations and schema are not compatible or if the schema is not present, this will result in an error.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `annotations` | HashMap<String, String> |  | Optional. Stores small amounts of arbitrary data. |
| `name` | String |  | Identifier. The relative resource name of the connector, in the format: ``` projects/{project}/locations/{location}/services/{service}/connectors/{connector} ``` |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `update_time` | String |  | Output only. [Output only] Update time stamp. |
| `source` | String |  | Required. The source files that comprise the connector. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `create_time` | String |  | Output only. [Output only] Create time stamp. |
| `display_name` | String |  | Optional. Mutable human-readable name. 63 character limit. |
| `reconciling` | bool |  | Output only. A field that if true, indicates that the system is working to compile and deploy the connector. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. [AIP-154](https://google.aip.dev/154) |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotations` | HashMap<String, String> | Optional. Stores small amounts of arbitrary data. |
| `name` | String | Identifier. The relative resource name of the connector, in the format: ``` projects/{project}/locations/{location}/services/{service}/connectors/{connector} ``` |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `update_time` | String | Output only. [Output only] Update time stamp. |
| `source` | String | Required. The source files that comprise the connector. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `create_time` | String | Output only. [Output only] Create time stamp. |
| `display_name` | String | Optional. Mutable human-readable name. 63 character limit. |
| `reconciling` | bool | Output only. A field that if true, indicates that the system is working to compile and deploy the connector. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. [AIP-154](https://google.aip.dev/154) |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connector
connector = provider.firebasedataconnect_api.Connector {
    parent = "value"  # Required. Value for parent.
}

# Access connector outputs
connector_id = connector.id
connector_annotations = connector.annotations
connector_name = connector.name
connector_uid = connector.uid
connector_update_time = connector.update_time
connector_source = connector.source
connector_labels = connector.labels
connector_create_time = connector.create_time
connector_display_name = connector.display_name
connector_reconciling = connector.reconciling
connector_etag = connector.etag
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

# Create operation
operation = provider.firebasedataconnect_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
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


### Connector

Creates a new Connector in a given project and location. The operations are validated against and must be compatible with the active schema. If the operations and schema are not compatible or if the schema is not present, this will result in an error.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. [AIP-154](https://google.aip.dev/154) |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `create_time` | String |  | Output only. [Output only] Create time stamp. |
| `source` | String |  | Required. The source files that comprise the connector. |
| `update_time` | String |  | Output only. [Output only] Update time stamp. |
| `reconciling` | bool |  | Output only. A field that if true, indicates that the system is working to compile and deploy the connector. |
| `annotations` | HashMap<String, String> |  | Optional. Stores small amounts of arbitrary data. |
| `display_name` | String |  | Optional. Mutable human-readable name. 63 character limit. |
| `name` | String |  | Identifier. The relative resource name of the connector, in the format: ``` projects/{project}/locations/{location}/services/{service}/connectors/{connector} ``` |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. System-assigned, unique identifier. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. [AIP-154](https://google.aip.dev/154) |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `create_time` | String | Output only. [Output only] Create time stamp. |
| `source` | String | Required. The source files that comprise the connector. |
| `update_time` | String | Output only. [Output only] Update time stamp. |
| `reconciling` | bool | Output only. A field that if true, indicates that the system is working to compile and deploy the connector. |
| `annotations` | HashMap<String, String> | Optional. Stores small amounts of arbitrary data. |
| `display_name` | String | Optional. Mutable human-readable name. 63 character limit. |
| `name` | String | Identifier. The relative resource name of the connector, in the format: ``` projects/{project}/locations/{location}/services/{service}/connectors/{connector} ``` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connector
connector = provider.firebasedataconnect_api.Connector {
    parent = "value"  # Required. Value for parent.
}

# Access connector outputs
connector_id = connector.id
connector_uid = connector.uid
connector_etag = connector.etag
connector_labels = connector.labels
connector_create_time = connector.create_time
connector_source = connector.source
connector_update_time = connector.update_time
connector_reconciling = connector.reconciling
connector_annotations = connector.annotations
connector_display_name = connector.display_name
connector_name = connector.name
```

---


### Service

Creates a new Service in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Optional. Mutable human-readable name. 63 character limit. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `reconciling` | bool |  | Output only. A field that if true, indicates that the system is working update the service. |
| `name` | String |  | Identifier. The relative resource name of the Firebase Data Connect service, in the format: ``` projects/{project}/locations/{location}/services/{service} ``` Note that the service ID is specific to Firebase Data Connect and does not correspond to any of the instance IDs of the underlying data source connections. |
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `update_time` | String |  | Output only. [Output only] Update time stamp. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. [AIP-154](https://google.aip.dev/154) |
| `annotations` | HashMap<String, String> |  | Optional. Stores small amounts of arbitrary data. |
| `create_time` | String |  | Output only. [Output only] Create time stamp. |
| `parent` | String | ✅ | Required. Value of parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Optional. Mutable human-readable name. 63 character limit. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `reconciling` | bool | Output only. A field that if true, indicates that the system is working update the service. |
| `name` | String | Identifier. The relative resource name of the Firebase Data Connect service, in the format: ``` projects/{project}/locations/{location}/services/{service} ``` Note that the service ID is specific to Firebase Data Connect and does not correspond to any of the instance IDs of the underlying data source connections. |
| `uid` | String | Output only. System-assigned, unique identifier. |
| `update_time` | String | Output only. [Output only] Update time stamp. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. [AIP-154](https://google.aip.dev/154) |
| `annotations` | HashMap<String, String> | Optional. Stores small amounts of arbitrary data. |
| `create_time` | String | Output only. [Output only] Create time stamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.firebasedataconnect_api.Service {
    parent = "value"  # Required. Value of parent.
}

# Access service outputs
service_id = service.id
service_display_name = service.display_name
service_labels = service.labels
service_reconciling = service.reconciling
service_name = service.name
service_uid = service.uid
service_update_time = service.update_time
service_etag = service.etag
service_annotations = service.annotations
service_create_time = service.create_time
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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_name = location.name
location_labels = location.labels
location_location_id = location.location_id
location_display_name = location.display_name
location_metadata = location.metadata
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.firebasedataconnect_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
```

---


### Schema

Creates a new Schema in a given project and location. Only creation of `schemas/main` is supported and calling create with any other schema ID will result in an error.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uid` | String |  | Output only. System-assigned, unique identifier. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. [AIP-154](https://google.aip.dev/154) |
| `annotations` | HashMap<String, String> |  | Optional. Stores small amounts of arbitrary data. |
| `reconciling` | bool |  | Output only. A field that if true, indicates that the system is working to compile and deploy the schema. |
| `create_time` | String |  | Output only. [Output only] Create time stamp. |
| `update_time` | String |  | Output only. [Output only] Update time stamp. |
| `name` | String |  | Identifier. The relative resource name of the schema, in the format: ``` projects/{project}/locations/{location}/services/{service}/schemas/{schema} ``` Right now, the only supported schema is "main". |
| `datasources` | Vec<String> |  | Required. The data sources linked in the schema. |
| `display_name` | String |  | Optional. Mutable human-readable name. 63 character limit. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `source` | String |  | Required. The source files that comprise the application schema. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. System-assigned, unique identifier. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. [AIP-154](https://google.aip.dev/154) |
| `annotations` | HashMap<String, String> | Optional. Stores small amounts of arbitrary data. |
| `reconciling` | bool | Output only. A field that if true, indicates that the system is working to compile and deploy the schema. |
| `create_time` | String | Output only. [Output only] Create time stamp. |
| `update_time` | String | Output only. [Output only] Update time stamp. |
| `name` | String | Identifier. The relative resource name of the schema, in the format: ``` projects/{project}/locations/{location}/services/{service}/schemas/{schema} ``` Right now, the only supported schema is "main". |
| `datasources` | Vec<String> | Required. The data sources linked in the schema. |
| `display_name` | String | Optional. Mutable human-readable name. 63 character limit. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `source` | String | Required. The source files that comprise the application schema. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create schema
schema = provider.firebasedataconnect_api.Schema {
    parent = "value"  # Required. Value for parent.
}

# Access schema outputs
schema_id = schema.id
schema_uid = schema.uid
schema_etag = schema.etag
schema_annotations = schema.annotations
schema_reconciling = schema.reconciling
schema_create_time = schema.create_time
schema_update_time = schema.update_time
schema_name = schema.name
schema_datasources = schema.datasources
schema_display_name = schema.display_name
schema_labels = schema.labels
schema_source = schema.source
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple location resources
location_0 = provider.firebasedataconnect_api.Location {
}
location_1 = provider.firebasedataconnect_api.Location {
}
location_2 = provider.firebasedataconnect_api.Location {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    location = provider.firebasedataconnect_api.Location {
    }
```

---

## Related Documentation

- [GCP Firebasedataconnect_api Documentation](https://cloud.google.com/firebasedataconnect_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
