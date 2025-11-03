# Apim_api Service



**Resources**: 6

---

## Overview

The apim_api service provides access to 6 resource types:

- [Observation_source](#observation_source) [CRD]
- [Observation_job](#observation_job) [CRD]
- [Api_observation](#api_observation) [CR]
- [Location](#location) [R]
- [Api_operation](#api_operation) [R]
- [Operation](#operation) [CRD]

---

## Resources


### Observation_source

CreateObservationSource creates a new ObservationSource but does not affect any deployed infrastructure. It is a configuration that can be used in an Observation Job to collect data about APIs running in user's dataplane.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. [Output only] Create time stamp |
| `gclb_observation_source` | String |  | The GCLB observation source |
| `name` | String |  | Identifier. name of resource For MVP, each region can only have 1 source. |
| `state` | String |  | Output only. The observation source state |
| `update_time` | String |  | Output only. [Output only] Update time stamp |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. [Output only] Create time stamp |
| `gclb_observation_source` | String | The GCLB observation source |
| `name` | String | Identifier. name of resource For MVP, each region can only have 1 source. |
| `state` | String | Output only. The observation source state |
| `update_time` | String | Output only. [Output only] Update time stamp |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create observation_source
observation_source = provider.apim_api.Observation_source {
    parent = "value"  # Required. Value for parent.
}

# Access observation_source outputs
observation_source_id = observation_source.id
observation_source_create_time = observation_source.create_time
observation_source_gclb_observation_source = observation_source.gclb_observation_source
observation_source_name = observation_source.name
observation_source_state = observation_source.state
observation_source_update_time = observation_source.update_time
```

---


### Observation_job

CreateObservationJob creates a new ObservationJob but does not have any effecton its own. It is a configuration that can be used in an Observation Job to collect data about existing APIs.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sources` | Vec<String> |  | Optional. These should be of the same kind of source. |
| `create_time` | String |  | Output only. [Output only] Create time stamp |
| `update_time` | String |  | Output only. [Output only] Update time stamp |
| `name` | String |  | Identifier. name of resource Format: projects/{project}/locations/{location}/observationJobs/{observation_job} |
| `state` | String |  | Output only. The observation job state |
| `parent` | String | ✅ | Required. The parent resource where this ObservationJob will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sources` | Vec<String> | Optional. These should be of the same kind of source. |
| `create_time` | String | Output only. [Output only] Create time stamp |
| `update_time` | String | Output only. [Output only] Update time stamp |
| `name` | String | Identifier. name of resource Format: projects/{project}/locations/{location}/observationJobs/{observation_job} |
| `state` | String | Output only. The observation job state |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create observation_job
observation_job = provider.apim_api.Observation_job {
    parent = "value"  # Required. The parent resource where this ObservationJob will be created. Format: projects/{project}/locations/{location}
}

# Access observation_job outputs
observation_job_id = observation_job.id
observation_job_sources = observation_job.sources
observation_job_create_time = observation_job.create_time
observation_job_update_time = observation_job.update_time
observation_job_name = observation_job.name
observation_job_state = observation_job.state
```

---


### Api_observation

BatchEditTagsApiObservations adds or removes Tags for ApiObservations.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Required. The request message specifying the resources to update. A maximum of 1000 apiObservations can be modified in a batch. |
| `parent` | String | ✅ | Required. The parent resource shared by all ApiObservations being edited. Format: projects/{project}/locations/{location}/observationJobs/{observation_job} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Create time stamp |
| `server_ips` | Vec<String> | The IP address (IPv4 or IPv6) of the origin server that the request was sent to. This field can include port information. Examples: `"192.168.1.1"`, `"10.0.0.1:80"`, `"FE80::0202:B3FF:FE1E:8329"`. |
| `last_event_detected_time` | String | Last event detected time stamp |
| `name` | String | Identifier. Name of resource |
| `hostname` | String | The hostname of requests processed for this Observation. |
| `api_operation_count` | String | The number of observed API Operations. |
| `update_time` | String | Update time stamp |
| `tags` | Vec<String> | User-defined tags to organize and sort |
| `source_locations` | Vec<String> | Location of the Observation Source, for example "us-central1" or "europe-west1." |
| `style` | String | Style of ApiObservation |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create api_observation
api_observation = provider.apim_api.Api_observation {
    parent = "value"  # Required. The parent resource shared by all ApiObservations being edited. Format: projects/{project}/locations/{location}/observationJobs/{observation_job}
}

# Access api_observation outputs
api_observation_id = api_observation.id
api_observation_create_time = api_observation.create_time
api_observation_server_ips = api_observation.server_ips
api_observation_last_event_detected_time = api_observation.last_event_detected_time
api_observation_name = api_observation.name
api_observation_hostname = api_observation.hostname
api_observation_api_operation_count = api_observation.api_operation_count
api_observation_update_time = api_observation.update_time
api_observation_tags = api_observation.tags
api_observation_source_locations = api_observation.source_locations
api_observation_style = api_observation.style
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


### Api_operation

GetApiOperation retrieves a single ApiOperation by name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_seen_time` | String | Last seen time stamp |
| `first_seen_time` | String | First seen time stamp |
| `count` | String | The number of occurrences of this API Operation. |
| `name` | String | Identifier. Name of resource |
| `http_operation` | String | An HTTP Operation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access api_operation outputs
api_operation_id = api_operation.id
api_operation_last_seen_time = api_operation.last_seen_time
api_operation_first_seen_time = api_operation.first_seen_time
api_operation_count = api_operation.count
api_operation_name = api_operation.name
api_operation_http_operation = api_operation.http_operation
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
operation = provider.apim_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple observation_source resources
observation_source_0 = provider.apim_api.Observation_source {
    parent = "value-0"
}
observation_source_1 = provider.apim_api.Observation_source {
    parent = "value-1"
}
observation_source_2 = provider.apim_api.Observation_source {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    observation_source = provider.apim_api.Observation_source {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Apim_api Documentation](https://cloud.google.com/apim_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
