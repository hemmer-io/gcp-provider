# Rapidmigrationassessment_api Service



**Resources**: 4

---

## Overview

The rapidmigrationassessment_api service provides access to 4 resource types:

- [Collector](#collector) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CRD]
- [Annotation](#annotation) [CR]

---

## Resources


### Collector

Create a Collector to manage the on-prem appliance which collects information about Customer assets.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_account` | String |  | Service Account email used to ingest data to this Collector. |
| `expected_asset_count` | String |  | User specified expected asset count. |
| `eula_uri` | String |  | Uri for EULA (End User License Agreement) from customer. |
| `vsphere_scan` | String |  | Output only. Reference to MC Source vsphere_scan. |
| `display_name` | String |  | User specified name of the Collector. |
| `guest_os_scan` | String |  | Output only. Reference to MC Source Guest Os Scan. |
| `state` | String |  | Output only. State of the Collector. |
| `create_time` | String |  | Output only. Create time stamp. |
| `client_version` | String |  | Output only. Client version. |
| `description` | String |  | User specified description of the Collector. |
| `name` | String |  | name of resource. |
| `update_time` | String |  | Output only. Update time stamp. |
| `bucket` | String |  | Output only. Store cloud storage bucket name (which is a guid) created with this Collector. |
| `collection_days` | i64 |  | How many days to collect data. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs. |
| `parent` | String | ✅ | Required. Name of the parent (project+location). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_account` | String | Service Account email used to ingest data to this Collector. |
| `expected_asset_count` | String | User specified expected asset count. |
| `eula_uri` | String | Uri for EULA (End User License Agreement) from customer. |
| `vsphere_scan` | String | Output only. Reference to MC Source vsphere_scan. |
| `display_name` | String | User specified name of the Collector. |
| `guest_os_scan` | String | Output only. Reference to MC Source Guest Os Scan. |
| `state` | String | Output only. State of the Collector. |
| `create_time` | String | Output only. Create time stamp. |
| `client_version` | String | Output only. Client version. |
| `description` | String | User specified description of the Collector. |
| `name` | String | name of resource. |
| `update_time` | String | Output only. Update time stamp. |
| `bucket` | String | Output only. Store cloud storage bucket name (which is a guid) created with this Collector. |
| `collection_days` | i64 | How many days to collect data. |
| `labels` | HashMap<String, String> | Labels as key value pairs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create collector
collector = provider.rapidmigrationassessment_api.Collector {
    parent = "value"  # Required. Name of the parent (project+location).
}

# Access collector outputs
collector_id = collector.id
collector_service_account = collector.service_account
collector_expected_asset_count = collector.expected_asset_count
collector_eula_uri = collector.eula_uri
collector_vsphere_scan = collector.vsphere_scan
collector_display_name = collector.display_name
collector_guest_os_scan = collector.guest_os_scan
collector_state = collector.state
collector_create_time = collector.create_time
collector_client_version = collector.client_version
collector_description = collector.description
collector_name = collector.name
collector_update_time = collector.update_time
collector_bucket = collector.bucket
collector_collection_days = collector.collection_days
collector_labels = collector.labels
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_location_id = location.location_id
location_display_name = location.display_name
location_name = location.name
location_labels = location.labels
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.rapidmigrationassessment_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
operation_metadata = operation.metadata
```

---


### Annotation

Creates an Annotation

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Labels as key value pairs. |
| `name` | String |  | name of resource. |
| `create_time` | String |  | Output only. Create time stamp. |
| `type` | String |  | Type of an annotation. |
| `update_time` | String |  | Output only. Update time stamp. |
| `parent` | String | ✅ | Required. Name of the parent (project+location). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Labels as key value pairs. |
| `name` | String | name of resource. |
| `create_time` | String | Output only. Create time stamp. |
| `type` | String | Type of an annotation. |
| `update_time` | String | Output only. Update time stamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create annotation
annotation = provider.rapidmigrationassessment_api.Annotation {
    parent = "value"  # Required. Name of the parent (project+location).
}

# Access annotation outputs
annotation_id = annotation.id
annotation_labels = annotation.labels
annotation_name = annotation.name
annotation_create_time = annotation.create_time
annotation_type = annotation.type
annotation_update_time = annotation.update_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple collector resources
collector_0 = provider.rapidmigrationassessment_api.Collector {
    parent = "value-0"
}
collector_1 = provider.rapidmigrationassessment_api.Collector {
    parent = "value-1"
}
collector_2 = provider.rapidmigrationassessment_api.Collector {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    collector = provider.rapidmigrationassessment_api.Collector {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Rapidmigrationassessment_api Documentation](https://cloud.google.com/rapidmigrationassessment_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
