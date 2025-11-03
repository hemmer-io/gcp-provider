# Migrationcenter_api Service



**Resources**: 27

---

## Overview

The migrationcenter_api service provides access to 27 resource types:

- [Report](#report) [CRD]
- [Operation](#operation) [CRD]
- [Preference_set](#preference_set) [CRUD]
- [Import_data_file](#import_data_file) [CRD]
- [Report_config](#report_config) [CRD]
- [Discovery_client](#discovery_client) [CRUD]
- [Source](#source) [CRUD]
- [Error_frame](#error_frame) [R]
- [Asset](#asset) [CRUD]
- [Group](#group) [CRUD]
- [Location](#location) [RU]
- [Import_job](#import_job) [CRUD]
- [Relation](#relation) [R]
- [Report_config](#report_config) [CRD]
- [Source](#source) [CRUD]
- [Asset](#asset) [CRUD]
- [Error_frame](#error_frame) [R]
- [Preference_set](#preference_set) [CRUD]
- [Operation](#operation) [CRD]
- [Import_job](#import_job) [CRUD]
- [Import_data_file](#import_data_file) [CRD]
- [Relation](#relation) [R]
- [Assets_export_job](#assets_export_job) [CRD]
- [Discovery_client](#discovery_client) [CRUD]
- [Group](#group) [CRUD]
- [Location](#location) [RU]
- [Report](#report) [CRD]

---

## Resources


### Report

Creates a report.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Creation timestamp. |
| `type` | String |  | Report type. |
| `summary` | String |  | Output only. Summary view of the Report. |
| `update_time` | String |  | Output only. Last update timestamp. |
| `state` | String |  | Report creation state. |
| `description` | String |  | Free-text description. |
| `display_name` | String |  | User-friendly display name. Maximum length is 63 characters. |
| `name` | String |  | Output only. Name of resource. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Creation timestamp. |
| `type` | String | Report type. |
| `summary` | String | Output only. Summary view of the Report. |
| `update_time` | String | Output only. Last update timestamp. |
| `state` | String | Report creation state. |
| `description` | String | Free-text description. |
| `display_name` | String | User-friendly display name. Maximum length is 63 characters. |
| `name` | String | Output only. Name of resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create report
report = provider.migrationcenter_api.Report {
    parent = "value"  # Required. Value for parent.
}

# Access report outputs
report_id = report.id
report_create_time = report.create_time
report_type = report.type
report_summary = report.summary
report_update_time = report.update_time
report_state = report.state
report_description = report.description
report_display_name = report.display_name
report_name = report.name
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.migrationcenter_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_response = operation.response
operation_done = operation.done
operation_error = operation.error
operation_name = operation.name
```

---


### Preference_set

Creates a new preference set in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The timestamp when the preference set was last updated. |
| `virtual_machine_preferences` | String |  | Optional. A set of preferences that applies to all virtual machines in the context. |
| `create_time` | String |  | Output only. The timestamp when the preference set was created. |
| `name` | String |  | Output only. Name of the preference set. |
| `description` | String |  | A description of the preference set. |
| `display_name` | String |  | User-friendly display name. Maximum length is 63 characters. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The timestamp when the preference set was last updated. |
| `virtual_machine_preferences` | String | Optional. A set of preferences that applies to all virtual machines in the context. |
| `create_time` | String | Output only. The timestamp when the preference set was created. |
| `name` | String | Output only. Name of the preference set. |
| `description` | String | A description of the preference set. |
| `display_name` | String | User-friendly display name. Maximum length is 63 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create preference_set
preference_set = provider.migrationcenter_api.Preference_set {
    parent = "value"  # Required. Value for parent.
}

# Access preference_set outputs
preference_set_id = preference_set.id
preference_set_update_time = preference_set.update_time
preference_set_virtual_machine_preferences = preference_set.virtual_machine_preferences
preference_set_create_time = preference_set.create_time
preference_set_name = preference_set.name
preference_set_description = preference_set.description
preference_set_display_name = preference_set.display_name
```

---


### Import_data_file

Creates an import data file.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The state of the import data file. |
| `upload_file_info` | String |  | Information about a file that is uploaded to a storage service. |
| `create_time` | String |  | Output only. The timestamp when the file was created. |
| `format` | String |  | Required. The payload format. |
| `display_name` | String |  | User-friendly display name. Maximum length is 63 characters. |
| `name` | String |  | Output only. The name of the file. |
| `parent` | String | ✅ | Required. Name of the parent of the ImportDataFile. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The state of the import data file. |
| `upload_file_info` | String | Information about a file that is uploaded to a storage service. |
| `create_time` | String | Output only. The timestamp when the file was created. |
| `format` | String | Required. The payload format. |
| `display_name` | String | User-friendly display name. Maximum length is 63 characters. |
| `name` | String | Output only. The name of the file. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create import_data_file
import_data_file = provider.migrationcenter_api.Import_data_file {
    parent = "value"  # Required. Name of the parent of the ImportDataFile.
}

# Access import_data_file outputs
import_data_file_id = import_data_file.id
import_data_file_state = import_data_file.state
import_data_file_upload_file_info = import_data_file.upload_file_info
import_data_file_create_time = import_data_file.create_time
import_data_file_format = import_data_file.format
import_data_file_display_name = import_data_file.display_name
import_data_file_name = import_data_file.name
```

---


### Report_config

Creates a report configuration.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Free-text description. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `update_time` | String |  | Output only. The timestamp when the resource was last updated. |
| `name` | String |  | Output only. Name of resource. |
| `group_preferenceset_assignments` | Vec<String> |  | Required. Collection of combinations of groups and preference sets. |
| `display_name` | String |  | User-friendly display name. Maximum length is 63 characters. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Free-text description. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `update_time` | String | Output only. The timestamp when the resource was last updated. |
| `name` | String | Output only. Name of resource. |
| `group_preferenceset_assignments` | Vec<String> | Required. Collection of combinations of groups and preference sets. |
| `display_name` | String | User-friendly display name. Maximum length is 63 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create report_config
report_config = provider.migrationcenter_api.Report_config {
    parent = "value"  # Required. Value for parent.
}

# Access report_config outputs
report_config_id = report_config.id
report_config_description = report_config.description
report_config_create_time = report_config.create_time
report_config_update_time = report_config.update_time
report_config_name = report_config.name
report_config_group_preferenceset_assignments = report_config.group_preferenceset_assignments
report_config_display_name = report_config.display_name
```

---


### Discovery_client

Creates a new discovery client.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source` | String |  | Required. Immutable. Full name of the source object associated with this discovery client. |
| `display_name` | String |  | Optional. Free text display name. Maximum length is 63 characters. |
| `errors` | Vec<String> |  | Output only. Errors affecting client functionality. |
| `heartbeat_time` | String |  | Output only. Last heartbeat time. Healthy clients are expected to send heartbeats regularly (normally every few minutes). |
| `expire_time` | String |  | Optional. Client expiration time in UTC. If specified, the backend will not accept new frames after this time. |
| `create_time` | String |  | Output only. Time when the discovery client was first created. |
| `signals_endpoint` | String |  | Output only. This field is intended for internal use. |
| `state` | String |  | Output only. Current state of the discovery client. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `service_account` | String |  | Required. Service account used by the discovery client for various operation. |
| `version` | String |  | Output only. Client version, as reported in recent heartbeat. |
| `description` | String |  | Optional. Free text description. Maximum length is 1000 characters. |
| `ttl` | String |  | Optional. Input only. Client time-to-live. If specified, the backend will not accept new frames after this time. This field is input only. The derived expiration time is provided as output through the `expire_time` field. |
| `name` | String |  | Output only. Identifier. Full name of this discovery client. |
| `update_time` | String |  | Output only. Time when the discovery client was last updated. This value is not updated by heartbeats, to view the last heartbeat time please refer to the `heartbeat_time` field. |
| `parent` | String | ✅ | Required. Parent resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `source` | String | Required. Immutable. Full name of the source object associated with this discovery client. |
| `display_name` | String | Optional. Free text display name. Maximum length is 63 characters. |
| `errors` | Vec<String> | Output only. Errors affecting client functionality. |
| `heartbeat_time` | String | Output only. Last heartbeat time. Healthy clients are expected to send heartbeats regularly (normally every few minutes). |
| `expire_time` | String | Optional. Client expiration time in UTC. If specified, the backend will not accept new frames after this time. |
| `create_time` | String | Output only. Time when the discovery client was first created. |
| `signals_endpoint` | String | Output only. This field is intended for internal use. |
| `state` | String | Output only. Current state of the discovery client. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `service_account` | String | Required. Service account used by the discovery client for various operation. |
| `version` | String | Output only. Client version, as reported in recent heartbeat. |
| `description` | String | Optional. Free text description. Maximum length is 1000 characters. |
| `ttl` | String | Optional. Input only. Client time-to-live. If specified, the backend will not accept new frames after this time. This field is input only. The derived expiration time is provided as output through the `expire_time` field. |
| `name` | String | Output only. Identifier. Full name of this discovery client. |
| `update_time` | String | Output only. Time when the discovery client was last updated. This value is not updated by heartbeats, to view the last heartbeat time please refer to the `heartbeat_time` field. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create discovery_client
discovery_client = provider.migrationcenter_api.Discovery_client {
    parent = "value"  # Required. Parent resource.
}

# Access discovery_client outputs
discovery_client_id = discovery_client.id
discovery_client_source = discovery_client.source
discovery_client_display_name = discovery_client.display_name
discovery_client_errors = discovery_client.errors
discovery_client_heartbeat_time = discovery_client.heartbeat_time
discovery_client_expire_time = discovery_client.expire_time
discovery_client_create_time = discovery_client.create_time
discovery_client_signals_endpoint = discovery_client.signals_endpoint
discovery_client_state = discovery_client.state
discovery_client_labels = discovery_client.labels
discovery_client_service_account = discovery_client.service_account
discovery_client_version = discovery_client.version
discovery_client_description = discovery_client.description
discovery_client_ttl = discovery_client.ttl
discovery_client_name = discovery_client.name
discovery_client_update_time = discovery_client.update_time
```

---


### Source

Creates a new source in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `error_frame_count` | i64 |  | Output only. The number of frames that were reported by the source and contained errors. |
| `priority` | i64 |  | The information confidence of the source. The higher the value, the higher the confidence. |
| `state` | String |  | Output only. The state of the source. |
| `update_time` | String |  | Output only. The timestamp when the source was last updated. |
| `pending_frame_count` | i64 |  | Output only. Number of frames that are still being processed. |
| `name` | String |  | Output only. The full name of the source. |
| `description` | String |  | Free-text description. |
| `create_time` | String |  | Output only. The timestamp when the source was created. |
| `type` | String |  | Data source type. |
| `managed` | bool |  | If `true`, the source is managed by other service(s). |
| `display_name` | String |  | User-friendly display name. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error_frame_count` | i64 | Output only. The number of frames that were reported by the source and contained errors. |
| `priority` | i64 | The information confidence of the source. The higher the value, the higher the confidence. |
| `state` | String | Output only. The state of the source. |
| `update_time` | String | Output only. The timestamp when the source was last updated. |
| `pending_frame_count` | i64 | Output only. Number of frames that are still being processed. |
| `name` | String | Output only. The full name of the source. |
| `description` | String | Free-text description. |
| `create_time` | String | Output only. The timestamp when the source was created. |
| `type` | String | Data source type. |
| `managed` | bool | If `true`, the source is managed by other service(s). |
| `display_name` | String | User-friendly display name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create source
source = provider.migrationcenter_api.Source {
    parent = "value"  # Required. Value for parent.
}

# Access source outputs
source_id = source.id
source_error_frame_count = source.error_frame_count
source_priority = source.priority
source_state = source.state
source_update_time = source.update_time
source_pending_frame_count = source.pending_frame_count
source_name = source.name
source_description = source.description
source_create_time = source.create_time
source_type = source.type
source_managed = source.managed
source_display_name = source.display_name
```

---


### Error_frame

Gets the details of an error frame.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `original_frame` | String | Output only. The frame that was originally reported. |
| `violations` | Vec<String> | Output only. All the violations that were detected for the frame. |
| `name` | String | Output only. The identifier of the ErrorFrame. |
| `ingestion_time` | String | Output only. Frame ingestion time. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access error_frame outputs
error_frame_id = error_frame.id
error_frame_original_frame = error_frame.original_frame
error_frame_violations = error_frame.violations
error_frame_name = error_frame.name
error_frame_ingestion_time = error_frame.ingestion_time
```

---


### Asset

Reports a set of frames.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `frames_data` | Vec<String> |  | A repeated field of asset data. |
| `parent` | String | ✅ | Required. Parent of the resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the asset was created. |
| `labels` | HashMap<String, String> | Labels as key value pairs. |
| `sources` | Vec<String> | Output only. The list of sources contributing to the asset. |
| `hide_reason` | String | Optional. An optional reason for marking this asset as hidden. |
| `machine_details` | String | Output only. Asset information specific for virtual and physical machines. |
| `attributes` | HashMap<String, String> | Generic asset attributes. |
| `assigned_groups` | Vec<String> | Output only. The list of groups that the asset is assigned to. |
| `performance_data` | String | Output only. Performance data for the asset. |
| `title` | String | Output only. Server generated human readable name of the asset. |
| `name` | String | Output only. The full name of the asset. |
| `database_deployment_details` | String | Output only. Asset information specific for database deployments. |
| `hide_time` | String | Output only. The timestamp when the asset was marked as hidden. |
| `update_time` | String | Output only. The timestamp when the asset was last updated. |
| `database_details` | String | Output only. Asset information specific for logical databases. |
| `insight_list` | String | Output only. The list of insights associated with the asset. |
| `hidden` | bool | Optional. Indicates if the asset is hidden. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create asset
asset = provider.migrationcenter_api.Asset {
    parent = "value"  # Required. Parent of the resource.
}

# Access asset outputs
asset_id = asset.id
asset_create_time = asset.create_time
asset_labels = asset.labels
asset_sources = asset.sources
asset_hide_reason = asset.hide_reason
asset_machine_details = asset.machine_details
asset_attributes = asset.attributes
asset_assigned_groups = asset.assigned_groups
asset_performance_data = asset.performance_data
asset_title = asset.title
asset_name = asset.name
asset_database_deployment_details = asset.database_deployment_details
asset_hide_time = asset.hide_time
asset_update_time = asset.update_time
asset_database_details = asset.database_details
asset_insight_list = asset.insight_list
asset_hidden = asset.hidden
```

---


### Group

Creates a new group in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the group was created. |
| `description` | String |  | Optional. The description of the group. |
| `update_time` | String |  | Output only. The timestamp when the group was last updated. |
| `display_name` | String |  | Optional. User-friendly display name. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs. |
| `name` | String |  | Output only. The name of the group. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the group was created. |
| `description` | String | Optional. The description of the group. |
| `update_time` | String | Output only. The timestamp when the group was last updated. |
| `display_name` | String | Optional. User-friendly display name. |
| `labels` | HashMap<String, String> | Labels as key value pairs. |
| `name` | String | Output only. The name of the group. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create group
group = provider.migrationcenter_api.Group {
    parent = "value"  # Required. Value for parent.
}

# Access group outputs
group_id = group.id
group_create_time = group.create_time
group_description = group.description
group_update_time = group.update_time
group_display_name = group.display_name
group_labels = group.labels
group_name = group.name
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `preference_set` | String |  | The preference set used by default for a project. |
| `disable_cloud_logging` | bool |  | Disable Cloud Logging for the Migration Center API. Users are billed for the logs. |
| `name` | String |  | Output only. The name of the resource. |
| `name` | String | ✅ | Output only. The name of the resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |


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
location_metadata = location.metadata
location_name = location.name
location_location_id = location.location_id
location_display_name = location.display_name
```

---


### Import_job

Creates an import job.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The timestamp when the import job was last updated. |
| `validation_report` | String |  | Output only. The report with the validation results of the import job. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs. |
| `name` | String |  | Output only. The full name of the import job. |
| `create_time` | String |  | Output only. The timestamp when the import job was created. |
| `state` | String |  | Output only. The state of the import job. |
| `display_name` | String |  | Optional. User-friendly display name. Maximum length is 256 characters. |
| `execution_report` | String |  | Output only. The report with the results of running the import job. |
| `asset_source` | String |  | Required. Reference to a source. |
| `complete_time` | String |  | Output only. The timestamp when the import job was completed. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The timestamp when the import job was last updated. |
| `validation_report` | String | Output only. The report with the validation results of the import job. |
| `labels` | HashMap<String, String> | Labels as key value pairs. |
| `name` | String | Output only. The full name of the import job. |
| `create_time` | String | Output only. The timestamp when the import job was created. |
| `state` | String | Output only. The state of the import job. |
| `display_name` | String | Optional. User-friendly display name. Maximum length is 256 characters. |
| `execution_report` | String | Output only. The report with the results of running the import job. |
| `asset_source` | String | Required. Reference to a source. |
| `complete_time` | String | Output only. The timestamp when the import job was completed. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create import_job
import_job = provider.migrationcenter_api.Import_job {
    parent = "value"  # Required. Value for parent.
}

# Access import_job outputs
import_job_id = import_job.id
import_job_update_time = import_job.update_time
import_job_validation_report = import_job.validation_report
import_job_labels = import_job.labels
import_job_name = import_job.name
import_job_create_time = import_job.create_time
import_job_state = import_job.state
import_job_display_name = import_job.display_name
import_job_execution_report = import_job.execution_report
import_job_asset_source = import_job.asset_source
import_job_complete_time = import_job.complete_time
```

---


### Relation

Gets the details of an relation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the relation was created. |
| `dst_asset` | String | Output only. The destination asset name in the relation. |
| `name` | String | Output only. Identifier. The identifier of the relation. |
| `type` | String | Optional. The type of the relation. |
| `src_asset` | String | Output only. The source asset name in the relation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access relation outputs
relation_id = relation.id
relation_create_time = relation.create_time
relation_dst_asset = relation.dst_asset
relation_name = relation.name
relation_type = relation.type
relation_src_asset = relation.src_asset
```

---


### Report_config

Creates a report configuration.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Free-text description. |
| `group_preferenceset_assignments` | Vec<String> |  | Required. Collection of combinations of groups and preference sets. |
| `update_time` | String |  | Output only. The timestamp when the resource was last updated. |
| `display_name` | String |  | User-friendly display name. Maximum length is 63 characters. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `name` | String |  | Output only. Name of resource. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Free-text description. |
| `group_preferenceset_assignments` | Vec<String> | Required. Collection of combinations of groups and preference sets. |
| `update_time` | String | Output only. The timestamp when the resource was last updated. |
| `display_name` | String | User-friendly display name. Maximum length is 63 characters. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `name` | String | Output only. Name of resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create report_config
report_config = provider.migrationcenter_api.Report_config {
    parent = "value"  # Required. Value for parent.
}

# Access report_config outputs
report_config_id = report_config.id
report_config_description = report_config.description
report_config_group_preferenceset_assignments = report_config.group_preferenceset_assignments
report_config_update_time = report_config.update_time
report_config_display_name = report_config.display_name
report_config_create_time = report_config.create_time
report_config_name = report_config.name
```

---


### Source

Creates a new source in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `error_frame_count` | i64 |  | Output only. The number of frames that were reported by the source and contained errors. |
| `pending_frame_count` | i64 |  | Output only. Number of frames that are still being processed. |
| `type` | String |  | Data source type. |
| `priority` | i64 |  | The information confidence of the source. The higher the value, the higher the confidence. |
| `is_managed` | bool |  | If `true`, the source is managed by other service(s). |
| `create_time` | String |  | Output only. The timestamp when the source was created. |
| `display_name` | String |  | User-friendly display name. |
| `state` | String |  | Output only. The state of the source. |
| `update_time` | String |  | Output only. The timestamp when the source was last updated. |
| `name` | String |  | Output only. The full name of the source. |
| `description` | String |  | Free-text description. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error_frame_count` | i64 | Output only. The number of frames that were reported by the source and contained errors. |
| `pending_frame_count` | i64 | Output only. Number of frames that are still being processed. |
| `type` | String | Data source type. |
| `priority` | i64 | The information confidence of the source. The higher the value, the higher the confidence. |
| `is_managed` | bool | If `true`, the source is managed by other service(s). |
| `create_time` | String | Output only. The timestamp when the source was created. |
| `display_name` | String | User-friendly display name. |
| `state` | String | Output only. The state of the source. |
| `update_time` | String | Output only. The timestamp when the source was last updated. |
| `name` | String | Output only. The full name of the source. |
| `description` | String | Free-text description. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create source
source = provider.migrationcenter_api.Source {
    parent = "value"  # Required. Value for parent.
}

# Access source outputs
source_id = source.id
source_error_frame_count = source.error_frame_count
source_pending_frame_count = source.pending_frame_count
source_type = source.type
source_priority = source.priority
source_is_managed = source.is_managed
source_create_time = source.create_time
source_display_name = source.display_name
source_state = source.state
source_update_time = source.update_time
source_name = source.name
source_description = source.description
```

---


### Asset

Updates the parameters of a list of assets.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Required. The request message specifying the resources to update. A maximum of 1000 assets can be modified in a batch. |
| `parent` | String | ✅ | Required. Parent value for batch asset update. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Labels as key value pairs. |
| `hide_reason` | String | Optional. An optional reason for marking this asset as hidden. |
| `hidden` | bool | Optional. Indicates if the asset is hidden. |
| `structured_attributes` | HashMap<String, String> | Optional. Generic structured asset attributes. |
| `sources` | Vec<String> | Output only. The list of sources contributing to the asset. |
| `aws_dynamodb_table_details` | String | Output only. Asset information specific for AWS DynamoDB tables. |
| `create_time` | String | Output only. The timestamp when the asset was created. |
| `hide_time` | String | Output only. The timestamp when the asset was marked as hidden. |
| `performance_data` | String | Performance data for the asset. |
| `database_deployment_details` | String | Output only. Asset information specific for database deployments. |
| `aws_redshift_details` | String | Output only. Asset information specific for AWS Redshift |
| `aws_route53_hosted_zone_details` | String | Output only. Asset information specific for AwsRoute53HostedZoneDetails |
| `aws_lambda_function_details` | String | Output only. Asset information specific for AWS Lambda functions. |
| `aws_elb_load_balancer_details` | String | Output only. Asset information specific for AWS Load Balancers. |
| `aws_nat_gateway_details` | String | Output only. Asset information specific for AwsNatGatewayDetails |
| `attributes` | HashMap<String, String> | Generic asset attributes. |
| `database_details` | String | Output only. Asset information specific for logical databases. |
| `name` | String | Output only. The full name of the asset. |
| `virtual_machine_details` | String | Output only. Asset information specific for virtual machines. |
| `assigned_groups` | Vec<String> | Output only. The list of groups that the asset is assigned to. |
| `aws_vpc_details` | String | Output only. Asset information specific for AWS VPCs. |
| `hosting_provider_details` | String | Output only. Details about the hosting provider of the asset. |
| `aws_s3_bucket_details` | String | Output only. Asset information specific for AWS S3 buckets. |
| `machine_details` | String | Output only. Asset information specific for virtual machines. |
| `title` | String | Output only. Server generated human readable name of the asset. |
| `aws_cloud_front_distribution_details` | String | Output only. Asset information specific for AWS CloudFront distributions. |
| `aws_ecs_cluster_details` | String | Output only. Asset information specific for AWS ECS clusters. |
| `insight_list` | String | Output only. The list of insights associated with the asset. |
| `aws_efs_file_system_details` | String | Output only. Asset information specific for AWS EFS file systems. |
| `aws_eks_cluster_details` | String | Output only. Asset information specific for AWS EKS clusters. |
| `update_time` | String | Output only. The timestamp when the asset was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create asset
asset = provider.migrationcenter_api.Asset {
    parent = "value"  # Required. Parent value for batch asset update.
}

# Access asset outputs
asset_id = asset.id
asset_labels = asset.labels
asset_hide_reason = asset.hide_reason
asset_hidden = asset.hidden
asset_structured_attributes = asset.structured_attributes
asset_sources = asset.sources
asset_aws_dynamodb_table_details = asset.aws_dynamodb_table_details
asset_create_time = asset.create_time
asset_hide_time = asset.hide_time
asset_performance_data = asset.performance_data
asset_database_deployment_details = asset.database_deployment_details
asset_aws_redshift_details = asset.aws_redshift_details
asset_aws_route53_hosted_zone_details = asset.aws_route53_hosted_zone_details
asset_aws_lambda_function_details = asset.aws_lambda_function_details
asset_aws_elb_load_balancer_details = asset.aws_elb_load_balancer_details
asset_aws_nat_gateway_details = asset.aws_nat_gateway_details
asset_attributes = asset.attributes
asset_database_details = asset.database_details
asset_name = asset.name
asset_virtual_machine_details = asset.virtual_machine_details
asset_assigned_groups = asset.assigned_groups
asset_aws_vpc_details = asset.aws_vpc_details
asset_hosting_provider_details = asset.hosting_provider_details
asset_aws_s3_bucket_details = asset.aws_s3_bucket_details
asset_machine_details = asset.machine_details
asset_title = asset.title
asset_aws_cloud_front_distribution_details = asset.aws_cloud_front_distribution_details
asset_aws_ecs_cluster_details = asset.aws_ecs_cluster_details
asset_insight_list = asset.insight_list
asset_aws_efs_file_system_details = asset.aws_efs_file_system_details
asset_aws_eks_cluster_details = asset.aws_eks_cluster_details
asset_update_time = asset.update_time
```

---


### Error_frame

Gets the details of an error frame.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `original_frame` | String | Output only. The frame that was originally reported. |
| `violations` | Vec<String> | Output only. All the violations that were detected for the frame. |
| `name` | String | Output only. The identifier of the ErrorFrame. |
| `ingestion_time` | String | Output only. Frame ingestion time. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access error_frame outputs
error_frame_id = error_frame.id
error_frame_original_frame = error_frame.original_frame
error_frame_violations = error_frame.violations
error_frame_name = error_frame.name
error_frame_ingestion_time = error_frame.ingestion_time
```

---


### Preference_set

Creates a new preference set in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The timestamp when the preference set was last updated. |
| `region_preferences` | String |  | Optional. Region preferences for assets using this preference set. If you are unsure which value to set, the migration service API region is often a good value to start with. If unspecified, VirtualMachinePreferences.RegionPreferences is used. |
| `virtual_machine_preferences` | String |  | A set of preferences that applies to all virtual machines in the context. |
| `create_time` | String |  | Output only. The timestamp when the preference set was created. |
| `description` | String |  | A description of the preference set. |
| `database_preferences` | String |  | Optional. A set of preferences that applies to all databases in the context. |
| `display_name` | String |  | User-friendly display name. Maximum length is 63 characters. |
| `name` | String |  | Output only. Name of the PreferenceSet. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The timestamp when the preference set was last updated. |
| `region_preferences` | String | Optional. Region preferences for assets using this preference set. If you are unsure which value to set, the migration service API region is often a good value to start with. If unspecified, VirtualMachinePreferences.RegionPreferences is used. |
| `virtual_machine_preferences` | String | A set of preferences that applies to all virtual machines in the context. |
| `create_time` | String | Output only. The timestamp when the preference set was created. |
| `description` | String | A description of the preference set. |
| `database_preferences` | String | Optional. A set of preferences that applies to all databases in the context. |
| `display_name` | String | User-friendly display name. Maximum length is 63 characters. |
| `name` | String | Output only. Name of the PreferenceSet. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create preference_set
preference_set = provider.migrationcenter_api.Preference_set {
    parent = "value"  # Required. Value for parent.
}

# Access preference_set outputs
preference_set_id = preference_set.id
preference_set_update_time = preference_set.update_time
preference_set_region_preferences = preference_set.region_preferences
preference_set_virtual_machine_preferences = preference_set.virtual_machine_preferences
preference_set_create_time = preference_set.create_time
preference_set_description = preference_set.description
preference_set_database_preferences = preference_set.database_preferences
preference_set_display_name = preference_set.display_name
preference_set_name = preference_set.name
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.migrationcenter_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_name = operation.name
operation_error = operation.error
operation_response = operation.response
operation_metadata = operation.metadata
```

---


### Import_job

Creates an import job.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The timestamp when the import job was last updated. |
| `asset_source` | String |  | Required. Reference to a source. |
| `execution_report` | String |  | Output only. The report with the results of running the import job. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs. |
| `gcs_payload` | String |  | The payload is in Google Cloud Storage. |
| `state` | String |  | Output only. The state of the import job. |
| `create_time` | String |  | Output only. The timestamp when the import job was created. |
| `validation_report` | String |  | Output only. The report with the validation results of the import job. |
| `inline_payload` | String |  | The payload is included in the request, mainly used for small import jobs. |
| `name` | String |  | Output only. The full name of the import job. |
| `complete_time` | String |  | Output only. The timestamp when the import job was completed. |
| `display_name` | String |  | User-friendly display name. Maximum length is 63 characters. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The timestamp when the import job was last updated. |
| `asset_source` | String | Required. Reference to a source. |
| `execution_report` | String | Output only. The report with the results of running the import job. |
| `labels` | HashMap<String, String> | Labels as key value pairs. |
| `gcs_payload` | String | The payload is in Google Cloud Storage. |
| `state` | String | Output only. The state of the import job. |
| `create_time` | String | Output only. The timestamp when the import job was created. |
| `validation_report` | String | Output only. The report with the validation results of the import job. |
| `inline_payload` | String | The payload is included in the request, mainly used for small import jobs. |
| `name` | String | Output only. The full name of the import job. |
| `complete_time` | String | Output only. The timestamp when the import job was completed. |
| `display_name` | String | User-friendly display name. Maximum length is 63 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create import_job
import_job = provider.migrationcenter_api.Import_job {
    parent = "value"  # Required. Value for parent.
}

# Access import_job outputs
import_job_id = import_job.id
import_job_update_time = import_job.update_time
import_job_asset_source = import_job.asset_source
import_job_execution_report = import_job.execution_report
import_job_labels = import_job.labels
import_job_gcs_payload = import_job.gcs_payload
import_job_state = import_job.state
import_job_create_time = import_job.create_time
import_job_validation_report = import_job.validation_report
import_job_inline_payload = import_job.inline_payload
import_job_name = import_job.name
import_job_complete_time = import_job.complete_time
import_job_display_name = import_job.display_name
```

---


### Import_data_file

Creates an import data file.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The state of the import data file. |
| `display_name` | String |  | Optional. User-friendly display name. Maximum length is 256 characters. |
| `upload_file_info` | String |  | Information about a file that is uploaded to a storage service. |
| `create_time` | String |  | Output only. The timestamp when the file was created. |
| `format` | String |  | Required. The payload format. |
| `name` | String |  | Output only. The name of the file. |
| `parent` | String | ✅ | Required. Name of the parent of the ImportDataFile. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The state of the import data file. |
| `display_name` | String | Optional. User-friendly display name. Maximum length is 256 characters. |
| `upload_file_info` | String | Information about a file that is uploaded to a storage service. |
| `create_time` | String | Output only. The timestamp when the file was created. |
| `format` | String | Required. The payload format. |
| `name` | String | Output only. The name of the file. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create import_data_file
import_data_file = provider.migrationcenter_api.Import_data_file {
    parent = "value"  # Required. Name of the parent of the ImportDataFile.
}

# Access import_data_file outputs
import_data_file_id = import_data_file.id
import_data_file_state = import_data_file.state
import_data_file_display_name = import_data_file.display_name
import_data_file_upload_file_info = import_data_file.upload_file_info
import_data_file_create_time = import_data_file.create_time
import_data_file_format = import_data_file.format
import_data_file_name = import_data_file.name
```

---


### Relation

Gets the details of an relation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | Optional. The type of the relation. |
| `create_time` | String | Output only. The timestamp when the relation was created. |
| `dst_asset` | String | Output only. The destination asset name in the relation. |
| `name` | String | Output only. Identifier. The identifier of the relation. |
| `src_asset` | String | Output only. The source asset name in the relation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access relation outputs
relation_id = relation.id
relation_type = relation.type
relation_create_time = relation.create_time
relation_dst_asset = relation.dst_asset
relation_name = relation.name
relation_src_asset = relation.src_asset
```

---


### Assets_export_job

Creates a new assets export job.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `condition` | String |  | Optional. Conditions for selecting assets to export. |
| `show_hidden` | bool |  | Optional. When this value is set to 'true' the response will include all assets, including those that are hidden. |
| `inventory` | String |  | Export asset inventory details. |
| `create_time` | String |  | Output only. Resource creation time. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes. |
| `name` | String |  | Output only. Identifier. Resource name. |
| `network_dependencies` | String |  | Export data regarding asset network dependencies. |
| `recent_executions` | Vec<String> |  | Output only. Recent non expired executions of the job. |
| `performance_data` | String |  | Export asset with performance data. |
| `update_time` | String |  | Output only. Resource update time. |
| `signed_uri_destination` | String |  | Export to Cloud Storage files downloadable using signed URIs. |
| `parent` | String | ✅ | Required. The parent resource where the assts export job will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `condition` | String | Optional. Conditions for selecting assets to export. |
| `show_hidden` | bool | Optional. When this value is set to 'true' the response will include all assets, including those that are hidden. |
| `inventory` | String | Export asset inventory details. |
| `create_time` | String | Output only. Resource creation time. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes. |
| `name` | String | Output only. Identifier. Resource name. |
| `network_dependencies` | String | Export data regarding asset network dependencies. |
| `recent_executions` | Vec<String> | Output only. Recent non expired executions of the job. |
| `performance_data` | String | Export asset with performance data. |
| `update_time` | String | Output only. Resource update time. |
| `signed_uri_destination` | String | Export to Cloud Storage files downloadable using signed URIs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create assets_export_job
assets_export_job = provider.migrationcenter_api.Assets_export_job {
    parent = "value"  # Required. The parent resource where the assts export job will be created.
}

# Access assets_export_job outputs
assets_export_job_id = assets_export_job.id
assets_export_job_condition = assets_export_job.condition
assets_export_job_show_hidden = assets_export_job.show_hidden
assets_export_job_inventory = assets_export_job.inventory
assets_export_job_create_time = assets_export_job.create_time
assets_export_job_labels = assets_export_job.labels
assets_export_job_name = assets_export_job.name
assets_export_job_network_dependencies = assets_export_job.network_dependencies
assets_export_job_recent_executions = assets_export_job.recent_executions
assets_export_job_performance_data = assets_export_job.performance_data
assets_export_job_update_time = assets_export_job.update_time
assets_export_job_signed_uri_destination = assets_export_job.signed_uri_destination
```

---


### Discovery_client

Creates a new discovery client.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_account` | String |  | Required. Service account used by the discovery client for various operation. |
| `recommended_versions` | Vec<String> |  | Output only. The recommended versions of the discovery client. |
| `description` | String |  | Optional. Free text description. Maximum length is 1000 characters. |
| `display_name` | String |  | Optional. Free text display name. Maximum length is 63 characters. |
| `update_time` | String |  | Output only. Time when the discovery client was last updated. This value is not updated by heartbeats, to view the last heartbeat time please refer to the `heartbeat_time` field. |
| `errors` | Vec<String> |  | Output only. Errors affecting client functionality. |
| `signals_endpoint` | String |  | Output only. This field is intended for internal use. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `heartbeat_time` | String |  | Output only. Last heartbeat time. Healthy clients are expected to send heartbeats regularly (normally every few minutes). |
| `state` | String |  | Output only. Current state of the discovery client. |
| `source` | String |  | Required. Full name of the source object associated with this discovery client. |
| `expire_time` | String |  | Optional. Client expiration time in UTC. If specified, the backend will not accept new frames after this time. |
| `version` | String |  | Output only. Client version, as reported in recent heartbeat. |
| `name` | String |  | Output only. Identifier. Full name of this discovery client. |
| `ttl` | String |  | Optional. Input only. Client time-to-live. If specified, the backend will not accept new frames after this time. This field is input only. The derived expiration time is provided as output through the `expire_time` field. |
| `create_time` | String |  | Output only. Time when the discovery client was first created. |
| `parent` | String | ✅ | Required. Parent resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_account` | String | Required. Service account used by the discovery client for various operation. |
| `recommended_versions` | Vec<String> | Output only. The recommended versions of the discovery client. |
| `description` | String | Optional. Free text description. Maximum length is 1000 characters. |
| `display_name` | String | Optional. Free text display name. Maximum length is 63 characters. |
| `update_time` | String | Output only. Time when the discovery client was last updated. This value is not updated by heartbeats, to view the last heartbeat time please refer to the `heartbeat_time` field. |
| `errors` | Vec<String> | Output only. Errors affecting client functionality. |
| `signals_endpoint` | String | Output only. This field is intended for internal use. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `heartbeat_time` | String | Output only. Last heartbeat time. Healthy clients are expected to send heartbeats regularly (normally every few minutes). |
| `state` | String | Output only. Current state of the discovery client. |
| `source` | String | Required. Full name of the source object associated with this discovery client. |
| `expire_time` | String | Optional. Client expiration time in UTC. If specified, the backend will not accept new frames after this time. |
| `version` | String | Output only. Client version, as reported in recent heartbeat. |
| `name` | String | Output only. Identifier. Full name of this discovery client. |
| `ttl` | String | Optional. Input only. Client time-to-live. If specified, the backend will not accept new frames after this time. This field is input only. The derived expiration time is provided as output through the `expire_time` field. |
| `create_time` | String | Output only. Time when the discovery client was first created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create discovery_client
discovery_client = provider.migrationcenter_api.Discovery_client {
    parent = "value"  # Required. Parent resource.
}

# Access discovery_client outputs
discovery_client_id = discovery_client.id
discovery_client_service_account = discovery_client.service_account
discovery_client_recommended_versions = discovery_client.recommended_versions
discovery_client_description = discovery_client.description
discovery_client_display_name = discovery_client.display_name
discovery_client_update_time = discovery_client.update_time
discovery_client_errors = discovery_client.errors
discovery_client_signals_endpoint = discovery_client.signals_endpoint
discovery_client_labels = discovery_client.labels
discovery_client_heartbeat_time = discovery_client.heartbeat_time
discovery_client_state = discovery_client.state
discovery_client_source = discovery_client.source
discovery_client_expire_time = discovery_client.expire_time
discovery_client_version = discovery_client.version
discovery_client_name = discovery_client.name
discovery_client_ttl = discovery_client.ttl
discovery_client_create_time = discovery_client.create_time
```

---


### Group

Creates a new group in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the group was created. |
| `name` | String |  | Output only. The name of the group. |
| `description` | String |  | Optional. The description of the group. |
| `display_name` | String |  | Optional. User-friendly display name. |
| `update_time` | String |  | Output only. The timestamp when the group was last updated. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the group was created. |
| `name` | String | Output only. The name of the group. |
| `description` | String | Optional. The description of the group. |
| `display_name` | String | Optional. User-friendly display name. |
| `update_time` | String | Output only. The timestamp when the group was last updated. |
| `labels` | HashMap<String, String> | Labels as key value pairs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create group
group = provider.migrationcenter_api.Group {
    parent = "value"  # Required. Value for parent.
}

# Access group outputs
group_id = group.id
group_create_time = group.create_time
group_name = group.name
group_description = group.description
group_display_name = group.display_name
group_update_time = group.update_time
group_labels = group.labels
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `customer_consent_for_google_sales_to_access_migration_center` | bool |  | Customer consent for Google sales to access their Cloud Migration Center project. |
| `preference_set` | String |  | The preference set used by default for a project. |
| `disable_cloud_logging` | bool |  | Disable Cloud Logging for the Migration Center API. Users are billed for the logs. |
| `name` | String |  | Output only. The name of the resource. |
| `name` | String | ✅ | Output only. The name of the resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
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
location_labels = location.labels
location_display_name = location.display_name
location_metadata = location.metadata
location_name = location.name
location_location_id = location.location_id
```

---


### Report

Creates a report.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Report creation state. |
| `display_name` | String |  | User-friendly display name. Maximum length is 63 characters. |
| `type` | String |  | Report type. |
| `summary` | String |  | Output only. Summary view of the Report. |
| `name` | String |  | Output only. Name of resource. |
| `create_time` | String |  | Output only. Creation timestamp. |
| `description` | String |  | Free-text description. |
| `update_time` | String |  | Output only. Last update timestamp. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Report creation state. |
| `display_name` | String | User-friendly display name. Maximum length is 63 characters. |
| `type` | String | Report type. |
| `summary` | String | Output only. Summary view of the Report. |
| `name` | String | Output only. Name of resource. |
| `create_time` | String | Output only. Creation timestamp. |
| `description` | String | Free-text description. |
| `update_time` | String | Output only. Last update timestamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create report
report = provider.migrationcenter_api.Report {
    parent = "value"  # Required. Value for parent.
}

# Access report outputs
report_id = report.id
report_state = report.state
report_display_name = report.display_name
report_type = report.type
report_summary = report.summary
report_name = report.name
report_create_time = report.create_time
report_description = report.description
report_update_time = report.update_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple report resources
report_0 = provider.migrationcenter_api.Report {
    parent = "value-0"
}
report_1 = provider.migrationcenter_api.Report {
    parent = "value-1"
}
report_2 = provider.migrationcenter_api.Report {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    report = provider.migrationcenter_api.Report {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Migrationcenter_api Documentation](https://cloud.google.com/migrationcenter_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
