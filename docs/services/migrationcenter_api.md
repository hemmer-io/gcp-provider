# Migrationcenter_api Service



**Resources**: 27

---

## Overview

The migrationcenter_api service provides access to 27 resource types:

- [Group](#group) [CRUD]
- [Relation](#relation) [R]
- [Import_data_file](#import_data_file) [CRD]
- [Operation](#operation) [CRD]
- [Import_job](#import_job) [CRUD]
- [Location](#location) [RU]
- [Preference_set](#preference_set) [CRUD]
- [Asset](#asset) [CRUD]
- [Discovery_client](#discovery_client) [CRUD]
- [Source](#source) [CRUD]
- [Error_frame](#error_frame) [R]
- [Report](#report) [CRD]
- [Report_config](#report_config) [CRD]
- [Report](#report) [CRD]
- [Import_data_file](#import_data_file) [CRD]
- [Error_frame](#error_frame) [R]
- [Import_job](#import_job) [CRUD]
- [Source](#source) [CRUD]
- [Preference_set](#preference_set) [CRUD]
- [Relation](#relation) [R]
- [Report_config](#report_config) [CRD]
- [Discovery_client](#discovery_client) [CRUD]
- [Asset](#asset) [CRUD]
- [Group](#group) [CRUD]
- [Operation](#operation) [CRD]
- [Assets_export_job](#assets_export_job) [CRD]
- [Location](#location) [RU]

---

## Resources


### Group

Creates a new group in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Optional. User-friendly display name. |
| `description` | String |  | Optional. The description of the group. |
| `name` | String |  | Output only. The name of the group. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs. |
| `update_time` | String |  | Output only. The timestamp when the group was last updated. |
| `create_time` | String |  | Output only. The timestamp when the group was created. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Optional. User-friendly display name. |
| `description` | String | Optional. The description of the group. |
| `name` | String | Output only. The name of the group. |
| `labels` | HashMap<String, String> | Labels as key value pairs. |
| `update_time` | String | Output only. The timestamp when the group was last updated. |
| `create_time` | String | Output only. The timestamp when the group was created. |


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
group_display_name = group.display_name
group_description = group.description
group_name = group.name
group_labels = group.labels
group_update_time = group.update_time
group_create_time = group.create_time
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
| `src_asset` | String | Output only. The source asset name in the relation. |
| `type` | String | Optional. The type of the relation. |
| `dst_asset` | String | Output only. The destination asset name in the relation. |
| `name` | String | Output only. Identifier. The identifier of the relation. |


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
relation_src_asset = relation.src_asset
relation_type = relation.type
relation_dst_asset = relation.dst_asset
relation_name = relation.name
```

---


### Import_data_file

Creates an import data file.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The state of the import data file. |
| `create_time` | String |  | Output only. The timestamp when the file was created. |
| `display_name` | String |  | User-friendly display name. Maximum length is 63 characters. |
| `format` | String |  | Required. The payload format. |
| `name` | String |  | Output only. The name of the file. |
| `upload_file_info` | String |  | Information about a file that is uploaded to a storage service. |
| `parent` | String | ✅ | Required. Name of the parent of the ImportDataFile. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The state of the import data file. |
| `create_time` | String | Output only. The timestamp when the file was created. |
| `display_name` | String | User-friendly display name. Maximum length is 63 characters. |
| `format` | String | Required. The payload format. |
| `name` | String | Output only. The name of the file. |
| `upload_file_info` | String | Information about a file that is uploaded to a storage service. |


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
import_data_file_create_time = import_data_file.create_time
import_data_file_display_name = import_data_file.display_name
import_data_file_format = import_data_file.format
import_data_file_name = import_data_file.name
import_data_file_upload_file_info = import_data_file.upload_file_info
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.migrationcenter_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_error = operation.error
operation_response = operation.response
operation_metadata = operation.metadata
operation_done = operation.done
```

---


### Import_job

Creates an import job.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `validation_report` | String |  | Output only. The report with the validation results of the import job. |
| `asset_source` | String |  | Required. Reference to a source. |
| `display_name` | String |  | Optional. User-friendly display name. Maximum length is 256 characters. |
| `name` | String |  | Output only. The full name of the import job. |
| `update_time` | String |  | Output only. The timestamp when the import job was last updated. |
| `complete_time` | String |  | Output only. The timestamp when the import job was completed. |
| `create_time` | String |  | Output only. The timestamp when the import job was created. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs. |
| `execution_report` | String |  | Output only. The report with the results of running the import job. |
| `state` | String |  | Output only. The state of the import job. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `validation_report` | String | Output only. The report with the validation results of the import job. |
| `asset_source` | String | Required. Reference to a source. |
| `display_name` | String | Optional. User-friendly display name. Maximum length is 256 characters. |
| `name` | String | Output only. The full name of the import job. |
| `update_time` | String | Output only. The timestamp when the import job was last updated. |
| `complete_time` | String | Output only. The timestamp when the import job was completed. |
| `create_time` | String | Output only. The timestamp when the import job was created. |
| `labels` | HashMap<String, String> | Labels as key value pairs. |
| `execution_report` | String | Output only. The report with the results of running the import job. |
| `state` | String | Output only. The state of the import job. |


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
import_job_validation_report = import_job.validation_report
import_job_asset_source = import_job.asset_source
import_job_display_name = import_job.display_name
import_job_name = import_job.name
import_job_update_time = import_job.update_time
import_job_complete_time = import_job.complete_time
import_job_create_time = import_job.create_time
import_job_labels = import_job.labels
import_job_execution_report = import_job.execution_report
import_job_state = import_job.state
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The name of the resource. |
| `disable_cloud_logging` | bool |  | Disable Cloud Logging for the Migration Center API. Users are billed for the logs. |
| `preference_set` | String |  | The preference set used by default for a project. |
| `name` | String | ✅ | Output only. The name of the resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
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
location_metadata = location.metadata
location_name = location.name
location_display_name = location.display_name
location_labels = location.labels
location_location_id = location.location_id
```

---


### Preference_set

Creates a new preference set in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the preference set was created. |
| `update_time` | String |  | Output only. The timestamp when the preference set was last updated. |
| `display_name` | String |  | User-friendly display name. Maximum length is 63 characters. |
| `name` | String |  | Output only. Name of the preference set. |
| `virtual_machine_preferences` | String |  | Optional. A set of preferences that applies to all virtual machines in the context. |
| `description` | String |  | A description of the preference set. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the preference set was created. |
| `update_time` | String | Output only. The timestamp when the preference set was last updated. |
| `display_name` | String | User-friendly display name. Maximum length is 63 characters. |
| `name` | String | Output only. Name of the preference set. |
| `virtual_machine_preferences` | String | Optional. A set of preferences that applies to all virtual machines in the context. |
| `description` | String | A description of the preference set. |


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
preference_set_create_time = preference_set.create_time
preference_set_update_time = preference_set.update_time
preference_set_display_name = preference_set.display_name
preference_set_name = preference_set.name
preference_set_virtual_machine_preferences = preference_set.virtual_machine_preferences
preference_set_description = preference_set.description
```

---


### Asset

Deletes list of Assets.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cascading_rules` | Vec<String> |  | Optional. Optional cascading rules for deleting related assets. |
| `allow_missing` | bool |  | Optional. When this value is set to `true` the request is a no-op for non-existing assets. See https://google.aip.dev/135#delete-if-existing for additional details. Default value is `false`. |
| `names` | Vec<String> |  | Required. The IDs of the assets to delete. A maximum of 1000 assets can be deleted in a batch. Format: projects/{project}/locations/{location}/assets/{name}. |
| `parent` | String | ✅ | Required. Parent value for batch asset delete. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `title` | String | Output only. Server generated human readable name of the asset. |
| `create_time` | String | Output only. The timestamp when the asset was created. |
| `sources` | Vec<String> | Output only. The list of sources contributing to the asset. |
| `update_time` | String | Output only. The timestamp when the asset was last updated. |
| `performance_data` | String | Output only. Performance data for the asset. |
| `database_deployment_details` | String | Output only. Asset information specific for database deployments. |
| `assigned_groups` | Vec<String> | Output only. The list of groups that the asset is assigned to. |
| `hide_reason` | String | Optional. An optional reason for marking this asset as hidden. |
| `hide_time` | String | Output only. The timestamp when the asset was marked as hidden. |
| `labels` | HashMap<String, String> | Labels as key value pairs. |
| `machine_details` | String | Output only. Asset information specific for virtual and physical machines. |
| `database_details` | String | Output only. Asset information specific for logical databases. |
| `attributes` | HashMap<String, String> | Generic asset attributes. |
| `insight_list` | String | Output only. The list of insights associated with the asset. |
| `name` | String | Output only. The full name of the asset. |
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
    parent = "value"  # Required. Parent value for batch asset delete.
}

# Access asset outputs
asset_id = asset.id
asset_title = asset.title
asset_create_time = asset.create_time
asset_sources = asset.sources
asset_update_time = asset.update_time
asset_performance_data = asset.performance_data
asset_database_deployment_details = asset.database_deployment_details
asset_assigned_groups = asset.assigned_groups
asset_hide_reason = asset.hide_reason
asset_hide_time = asset.hide_time
asset_labels = asset.labels
asset_machine_details = asset.machine_details
asset_database_details = asset.database_details
asset_attributes = asset.attributes
asset_insight_list = asset.insight_list
asset_name = asset.name
asset_hidden = asset.hidden
```

---


### Discovery_client

Creates a new discovery client.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Identifier. Full name of this discovery client. |
| `expire_time` | String |  | Optional. Client expiration time in UTC. If specified, the backend will not accept new frames after this time. |
| `signals_endpoint` | String |  | Output only. This field is intended for internal use. |
| `create_time` | String |  | Output only. Time when the discovery client was first created. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `display_name` | String |  | Optional. Free text display name. Maximum length is 63 characters. |
| `source` | String |  | Required. Immutable. Full name of the source object associated with this discovery client. |
| `service_account` | String |  | Required. Service account used by the discovery client for various operation. |
| `ttl` | String |  | Optional. Input only. Client time-to-live. If specified, the backend will not accept new frames after this time. This field is input only. The derived expiration time is provided as output through the `expire_time` field. |
| `errors` | Vec<String> |  | Output only. Errors affecting client functionality. |
| `update_time` | String |  | Output only. Time when the discovery client was last updated. This value is not updated by heartbeats, to view the last heartbeat time please refer to the `heartbeat_time` field. |
| `state` | String |  | Output only. Current state of the discovery client. |
| `version` | String |  | Output only. Client version, as reported in recent heartbeat. |
| `description` | String |  | Optional. Free text description. Maximum length is 1000 characters. |
| `heartbeat_time` | String |  | Output only. Last heartbeat time. Healthy clients are expected to send heartbeats regularly (normally every few minutes). |
| `parent` | String | ✅ | Required. Parent resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Identifier. Full name of this discovery client. |
| `expire_time` | String | Optional. Client expiration time in UTC. If specified, the backend will not accept new frames after this time. |
| `signals_endpoint` | String | Output only. This field is intended for internal use. |
| `create_time` | String | Output only. Time when the discovery client was first created. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `display_name` | String | Optional. Free text display name. Maximum length is 63 characters. |
| `source` | String | Required. Immutable. Full name of the source object associated with this discovery client. |
| `service_account` | String | Required. Service account used by the discovery client for various operation. |
| `ttl` | String | Optional. Input only. Client time-to-live. If specified, the backend will not accept new frames after this time. This field is input only. The derived expiration time is provided as output through the `expire_time` field. |
| `errors` | Vec<String> | Output only. Errors affecting client functionality. |
| `update_time` | String | Output only. Time when the discovery client was last updated. This value is not updated by heartbeats, to view the last heartbeat time please refer to the `heartbeat_time` field. |
| `state` | String | Output only. Current state of the discovery client. |
| `version` | String | Output only. Client version, as reported in recent heartbeat. |
| `description` | String | Optional. Free text description. Maximum length is 1000 characters. |
| `heartbeat_time` | String | Output only. Last heartbeat time. Healthy clients are expected to send heartbeats regularly (normally every few minutes). |


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
discovery_client_name = discovery_client.name
discovery_client_expire_time = discovery_client.expire_time
discovery_client_signals_endpoint = discovery_client.signals_endpoint
discovery_client_create_time = discovery_client.create_time
discovery_client_labels = discovery_client.labels
discovery_client_display_name = discovery_client.display_name
discovery_client_source = discovery_client.source
discovery_client_service_account = discovery_client.service_account
discovery_client_ttl = discovery_client.ttl
discovery_client_errors = discovery_client.errors
discovery_client_update_time = discovery_client.update_time
discovery_client_state = discovery_client.state
discovery_client_version = discovery_client.version
discovery_client_description = discovery_client.description
discovery_client_heartbeat_time = discovery_client.heartbeat_time
```

---


### Source

Creates a new source in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `priority` | i64 |  | The information confidence of the source. The higher the value, the higher the confidence. |
| `state` | String |  | Output only. The state of the source. |
| `type` | String |  | Data source type. |
| `pending_frame_count` | i64 |  | Output only. Number of frames that are still being processed. |
| `update_time` | String |  | Output only. The timestamp when the source was last updated. |
| `create_time` | String |  | Output only. The timestamp when the source was created. |
| `name` | String |  | Output only. The full name of the source. |
| `description` | String |  | Free-text description. |
| `managed` | bool |  | If `true`, the source is managed by other service(s). |
| `display_name` | String |  | User-friendly display name. |
| `error_frame_count` | i64 |  | Output only. The number of frames that were reported by the source and contained errors. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `priority` | i64 | The information confidence of the source. The higher the value, the higher the confidence. |
| `state` | String | Output only. The state of the source. |
| `type` | String | Data source type. |
| `pending_frame_count` | i64 | Output only. Number of frames that are still being processed. |
| `update_time` | String | Output only. The timestamp when the source was last updated. |
| `create_time` | String | Output only. The timestamp when the source was created. |
| `name` | String | Output only. The full name of the source. |
| `description` | String | Free-text description. |
| `managed` | bool | If `true`, the source is managed by other service(s). |
| `display_name` | String | User-friendly display name. |
| `error_frame_count` | i64 | Output only. The number of frames that were reported by the source and contained errors. |


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
source_priority = source.priority
source_state = source.state
source_type = source.type
source_pending_frame_count = source.pending_frame_count
source_update_time = source.update_time
source_create_time = source.create_time
source_name = source.name
source_description = source.description
source_managed = source.managed
source_display_name = source.display_name
source_error_frame_count = source.error_frame_count
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
| `name` | String | Output only. The identifier of the ErrorFrame. |
| `original_frame` | String | Output only. The frame that was originally reported. |
| `ingestion_time` | String | Output only. Frame ingestion time. |
| `violations` | Vec<String> | Output only. All the violations that were detected for the frame. |


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
error_frame_name = error_frame.name
error_frame_original_frame = error_frame.original_frame
error_frame_ingestion_time = error_frame.ingestion_time
error_frame_violations = error_frame.violations
```

---


### Report

Creates a report.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | User-friendly display name. Maximum length is 63 characters. |
| `description` | String |  | Free-text description. |
| `type` | String |  | Report type. |
| `state` | String |  | Report creation state. |
| `summary` | String |  | Output only. Summary view of the Report. |
| `create_time` | String |  | Output only. Creation timestamp. |
| `update_time` | String |  | Output only. Last update timestamp. |
| `name` | String |  | Output only. Name of resource. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | User-friendly display name. Maximum length is 63 characters. |
| `description` | String | Free-text description. |
| `type` | String | Report type. |
| `state` | String | Report creation state. |
| `summary` | String | Output only. Summary view of the Report. |
| `create_time` | String | Output only. Creation timestamp. |
| `update_time` | String | Output only. Last update timestamp. |
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
report_display_name = report.display_name
report_description = report.description
report_type = report.type
report_state = report.state
report_summary = report.summary
report_create_time = report.create_time
report_update_time = report.update_time
report_name = report.name
```

---


### Report_config

Creates a report configuration.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | User-friendly display name. Maximum length is 63 characters. |
| `name` | String |  | Output only. Name of resource. |
| `update_time` | String |  | Output only. The timestamp when the resource was last updated. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `group_preferenceset_assignments` | Vec<String> |  | Required. Collection of combinations of groups and preference sets. |
| `description` | String |  | Free-text description. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | User-friendly display name. Maximum length is 63 characters. |
| `name` | String | Output only. Name of resource. |
| `update_time` | String | Output only. The timestamp when the resource was last updated. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `group_preferenceset_assignments` | Vec<String> | Required. Collection of combinations of groups and preference sets. |
| `description` | String | Free-text description. |


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
report_config_display_name = report_config.display_name
report_config_name = report_config.name
report_config_update_time = report_config.update_time
report_config_create_time = report_config.create_time
report_config_group_preferenceset_assignments = report_config.group_preferenceset_assignments
report_config_description = report_config.description
```

---


### Report

Creates a report.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Last update timestamp. |
| `state` | String |  | Report creation state. |
| `name` | String |  | Output only. Name of resource. |
| `display_name` | String |  | User-friendly display name. Maximum length is 63 characters. |
| `summary` | String |  | Output only. Summary view of the Report. |
| `description` | String |  | Free-text description. |
| `type` | String |  | Report type. |
| `create_time` | String |  | Output only. Creation timestamp. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Last update timestamp. |
| `state` | String | Report creation state. |
| `name` | String | Output only. Name of resource. |
| `display_name` | String | User-friendly display name. Maximum length is 63 characters. |
| `summary` | String | Output only. Summary view of the Report. |
| `description` | String | Free-text description. |
| `type` | String | Report type. |
| `create_time` | String | Output only. Creation timestamp. |


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
report_update_time = report.update_time
report_state = report.state
report_name = report.name
report_display_name = report.display_name
report_summary = report.summary
report_description = report.description
report_type = report.type
report_create_time = report.create_time
```

---


### Import_data_file

Creates an import data file.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `format` | String |  | Required. The payload format. |
| `state` | String |  | Output only. The state of the import data file. |
| `create_time` | String |  | Output only. The timestamp when the file was created. |
| `display_name` | String |  | Optional. User-friendly display name. Maximum length is 256 characters. |
| `name` | String |  | Output only. The name of the file. |
| `upload_file_info` | String |  | Information about a file that is uploaded to a storage service. |
| `parent` | String | ✅ | Required. Name of the parent of the ImportDataFile. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `format` | String | Required. The payload format. |
| `state` | String | Output only. The state of the import data file. |
| `create_time` | String | Output only. The timestamp when the file was created. |
| `display_name` | String | Optional. User-friendly display name. Maximum length is 256 characters. |
| `name` | String | Output only. The name of the file. |
| `upload_file_info` | String | Information about a file that is uploaded to a storage service. |


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
import_data_file_format = import_data_file.format
import_data_file_state = import_data_file.state
import_data_file_create_time = import_data_file.create_time
import_data_file_display_name = import_data_file.display_name
import_data_file_name = import_data_file.name
import_data_file_upload_file_info = import_data_file.upload_file_info
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
| `ingestion_time` | String | Output only. Frame ingestion time. |
| `violations` | Vec<String> | Output only. All the violations that were detected for the frame. |
| `original_frame` | String | Output only. The frame that was originally reported. |
| `name` | String | Output only. The identifier of the ErrorFrame. |


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
error_frame_ingestion_time = error_frame.ingestion_time
error_frame_violations = error_frame.violations
error_frame_original_frame = error_frame.original_frame
error_frame_name = error_frame.name
```

---


### Import_job

Creates an import job.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The full name of the import job. |
| `update_time` | String |  | Output only. The timestamp when the import job was last updated. |
| `validation_report` | String |  | Output only. The report with the validation results of the import job. |
| `execution_report` | String |  | Output only. The report with the results of running the import job. |
| `gcs_payload` | String |  | The payload is in Google Cloud Storage. |
| `state` | String |  | Output only. The state of the import job. |
| `inline_payload` | String |  | The payload is included in the request, mainly used for small import jobs. |
| `create_time` | String |  | Output only. The timestamp when the import job was created. |
| `asset_source` | String |  | Required. Reference to a source. |
| `complete_time` | String |  | Output only. The timestamp when the import job was completed. |
| `display_name` | String |  | User-friendly display name. Maximum length is 63 characters. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The full name of the import job. |
| `update_time` | String | Output only. The timestamp when the import job was last updated. |
| `validation_report` | String | Output only. The report with the validation results of the import job. |
| `execution_report` | String | Output only. The report with the results of running the import job. |
| `gcs_payload` | String | The payload is in Google Cloud Storage. |
| `state` | String | Output only. The state of the import job. |
| `inline_payload` | String | The payload is included in the request, mainly used for small import jobs. |
| `create_time` | String | Output only. The timestamp when the import job was created. |
| `asset_source` | String | Required. Reference to a source. |
| `complete_time` | String | Output only. The timestamp when the import job was completed. |
| `display_name` | String | User-friendly display name. Maximum length is 63 characters. |
| `labels` | HashMap<String, String> | Labels as key value pairs. |


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
import_job_name = import_job.name
import_job_update_time = import_job.update_time
import_job_validation_report = import_job.validation_report
import_job_execution_report = import_job.execution_report
import_job_gcs_payload = import_job.gcs_payload
import_job_state = import_job.state
import_job_inline_payload = import_job.inline_payload
import_job_create_time = import_job.create_time
import_job_asset_source = import_job.asset_source
import_job_complete_time = import_job.complete_time
import_job_display_name = import_job.display_name
import_job_labels = import_job.labels
```

---


### Source

Creates a new source in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Free-text description. |
| `create_time` | String |  | Output only. The timestamp when the source was created. |
| `is_managed` | bool |  | If `true`, the source is managed by other service(s). |
| `name` | String |  | Output only. The full name of the source. |
| `type` | String |  | Data source type. |
| `update_time` | String |  | Output only. The timestamp when the source was last updated. |
| `error_frame_count` | i64 |  | Output only. The number of frames that were reported by the source and contained errors. |
| `pending_frame_count` | i64 |  | Output only. Number of frames that are still being processed. |
| `display_name` | String |  | User-friendly display name. |
| `priority` | i64 |  | The information confidence of the source. The higher the value, the higher the confidence. |
| `state` | String |  | Output only. The state of the source. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Free-text description. |
| `create_time` | String | Output only. The timestamp when the source was created. |
| `is_managed` | bool | If `true`, the source is managed by other service(s). |
| `name` | String | Output only. The full name of the source. |
| `type` | String | Data source type. |
| `update_time` | String | Output only. The timestamp when the source was last updated. |
| `error_frame_count` | i64 | Output only. The number of frames that were reported by the source and contained errors. |
| `pending_frame_count` | i64 | Output only. Number of frames that are still being processed. |
| `display_name` | String | User-friendly display name. |
| `priority` | i64 | The information confidence of the source. The higher the value, the higher the confidence. |
| `state` | String | Output only. The state of the source. |


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
source_description = source.description
source_create_time = source.create_time
source_is_managed = source.is_managed
source_name = source.name
source_type = source.type
source_update_time = source.update_time
source_error_frame_count = source.error_frame_count
source_pending_frame_count = source.pending_frame_count
source_display_name = source.display_name
source_priority = source.priority
source_state = source.state
```

---


### Preference_set

Creates a new preference set in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The timestamp when the preference set was last updated. |
| `display_name` | String |  | User-friendly display name. Maximum length is 63 characters. |
| `database_preferences` | String |  | Optional. A set of preferences that applies to all databases in the context. |
| `region_preferences` | String |  | Optional. Region preferences for assets using this preference set. If you are unsure which value to set, the migration service API region is often a good value to start with. If unspecified, VirtualMachinePreferences.RegionPreferences is used. |
| `virtual_machine_preferences` | String |  | A set of preferences that applies to all virtual machines in the context. |
| `create_time` | String |  | Output only. The timestamp when the preference set was created. |
| `name` | String |  | Output only. Name of the PreferenceSet. |
| `description` | String |  | A description of the preference set. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The timestamp when the preference set was last updated. |
| `display_name` | String | User-friendly display name. Maximum length is 63 characters. |
| `database_preferences` | String | Optional. A set of preferences that applies to all databases in the context. |
| `region_preferences` | String | Optional. Region preferences for assets using this preference set. If you are unsure which value to set, the migration service API region is often a good value to start with. If unspecified, VirtualMachinePreferences.RegionPreferences is used. |
| `virtual_machine_preferences` | String | A set of preferences that applies to all virtual machines in the context. |
| `create_time` | String | Output only. The timestamp when the preference set was created. |
| `name` | String | Output only. Name of the PreferenceSet. |
| `description` | String | A description of the preference set. |


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
preference_set_display_name = preference_set.display_name
preference_set_database_preferences = preference_set.database_preferences
preference_set_region_preferences = preference_set.region_preferences
preference_set_virtual_machine_preferences = preference_set.virtual_machine_preferences
preference_set_create_time = preference_set.create_time
preference_set_name = preference_set.name
preference_set_description = preference_set.description
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
| `dst_asset` | String | Output only. The destination asset name in the relation. |
| `name` | String | Output only. Identifier. The identifier of the relation. |
| `type` | String | Optional. The type of the relation. |
| `src_asset` | String | Output only. The source asset name in the relation. |
| `create_time` | String | Output only. The timestamp when the relation was created. |


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
relation_dst_asset = relation.dst_asset
relation_name = relation.name
relation_type = relation.type
relation_src_asset = relation.src_asset
relation_create_time = relation.create_time
```

---


### Report_config

Creates a report configuration.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `group_preferenceset_assignments` | Vec<String> |  | Required. Collection of combinations of groups and preference sets. |
| `update_time` | String |  | Output only. The timestamp when the resource was last updated. |
| `name` | String |  | Output only. Name of resource. |
| `description` | String |  | Free-text description. |
| `display_name` | String |  | User-friendly display name. Maximum length is 63 characters. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `group_preferenceset_assignments` | Vec<String> | Required. Collection of combinations of groups and preference sets. |
| `update_time` | String | Output only. The timestamp when the resource was last updated. |
| `name` | String | Output only. Name of resource. |
| `description` | String | Free-text description. |
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
report_config_create_time = report_config.create_time
report_config_group_preferenceset_assignments = report_config.group_preferenceset_assignments
report_config_update_time = report_config.update_time
report_config_name = report_config.name
report_config_description = report_config.description
report_config_display_name = report_config.display_name
```

---


### Discovery_client

Creates a new discovery client.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. Free text description. Maximum length is 1000 characters. |
| `ttl` | String |  | Optional. Input only. Client time-to-live. If specified, the backend will not accept new frames after this time. This field is input only. The derived expiration time is provided as output through the `expire_time` field. |
| `display_name` | String |  | Optional. Free text display name. Maximum length is 63 characters. |
| `expire_time` | String |  | Optional. Client expiration time in UTC. If specified, the backend will not accept new frames after this time. |
| `source` | String |  | Required. Full name of the source object associated with this discovery client. |
| `state` | String |  | Output only. Current state of the discovery client. |
| `heartbeat_time` | String |  | Output only. Last heartbeat time. Healthy clients are expected to send heartbeats regularly (normally every few minutes). |
| `recommended_versions` | Vec<String> |  | Output only. The recommended versions of the discovery client. |
| `name` | String |  | Output only. Identifier. Full name of this discovery client. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `errors` | Vec<String> |  | Output only. Errors affecting client functionality. |
| `service_account` | String |  | Required. Service account used by the discovery client for various operation. |
| `create_time` | String |  | Output only. Time when the discovery client was first created. |
| `update_time` | String |  | Output only. Time when the discovery client was last updated. This value is not updated by heartbeats, to view the last heartbeat time please refer to the `heartbeat_time` field. |
| `signals_endpoint` | String |  | Output only. This field is intended for internal use. |
| `version` | String |  | Output only. Client version, as reported in recent heartbeat. |
| `parent` | String | ✅ | Required. Parent resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. Free text description. Maximum length is 1000 characters. |
| `ttl` | String | Optional. Input only. Client time-to-live. If specified, the backend will not accept new frames after this time. This field is input only. The derived expiration time is provided as output through the `expire_time` field. |
| `display_name` | String | Optional. Free text display name. Maximum length is 63 characters. |
| `expire_time` | String | Optional. Client expiration time in UTC. If specified, the backend will not accept new frames after this time. |
| `source` | String | Required. Full name of the source object associated with this discovery client. |
| `state` | String | Output only. Current state of the discovery client. |
| `heartbeat_time` | String | Output only. Last heartbeat time. Healthy clients are expected to send heartbeats regularly (normally every few minutes). |
| `recommended_versions` | Vec<String> | Output only. The recommended versions of the discovery client. |
| `name` | String | Output only. Identifier. Full name of this discovery client. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `errors` | Vec<String> | Output only. Errors affecting client functionality. |
| `service_account` | String | Required. Service account used by the discovery client for various operation. |
| `create_time` | String | Output only. Time when the discovery client was first created. |
| `update_time` | String | Output only. Time when the discovery client was last updated. This value is not updated by heartbeats, to view the last heartbeat time please refer to the `heartbeat_time` field. |
| `signals_endpoint` | String | Output only. This field is intended for internal use. |
| `version` | String | Output only. Client version, as reported in recent heartbeat. |


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
discovery_client_description = discovery_client.description
discovery_client_ttl = discovery_client.ttl
discovery_client_display_name = discovery_client.display_name
discovery_client_expire_time = discovery_client.expire_time
discovery_client_source = discovery_client.source
discovery_client_state = discovery_client.state
discovery_client_heartbeat_time = discovery_client.heartbeat_time
discovery_client_recommended_versions = discovery_client.recommended_versions
discovery_client_name = discovery_client.name
discovery_client_labels = discovery_client.labels
discovery_client_errors = discovery_client.errors
discovery_client_service_account = discovery_client.service_account
discovery_client_create_time = discovery_client.create_time
discovery_client_update_time = discovery_client.update_time
discovery_client_signals_endpoint = discovery_client.signals_endpoint
discovery_client_version = discovery_client.version
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
| `virtual_machine_details` | String | Output only. Asset information specific for virtual machines. |
| `create_time` | String | Output only. The timestamp when the asset was created. |
| `database_details` | String | Output only. Asset information specific for logical databases. |
| `database_deployment_details` | String | Output only. Asset information specific for database deployments. |
| `hide_reason` | String | Optional. An optional reason for marking this asset as hidden. |
| `update_time` | String | Output only. The timestamp when the asset was last updated. |
| `machine_details` | String | Output only. Asset information specific for virtual machines. |
| `aws_eks_cluster_details` | String | Output only. Asset information specific for AWS EKS clusters. |
| `structured_attributes` | HashMap<String, String> | Optional. Generic structured asset attributes. |
| `name` | String | Output only. The full name of the asset. |
| `sources` | Vec<String> | Output only. The list of sources contributing to the asset. |
| `performance_data` | String | Performance data for the asset. |
| `aws_vpc_details` | String | Output only. Asset information specific for AWS VPCs. |
| `hosting_provider_details` | String | Output only. Details about the hosting provider of the asset. |
| `hide_time` | String | Output only. The timestamp when the asset was marked as hidden. |
| `insight_list` | String | Output only. The list of insights associated with the asset. |
| `title` | String | Output only. Server generated human readable name of the asset. |
| `aws_cloud_front_distribution_details` | String | Output only. Asset information specific for AWS CloudFront distributions. |
| `aws_s3_bucket_details` | String | Output only. Asset information specific for AWS S3 buckets. |
| `aws_dynamodb_table_details` | String | Output only. Asset information specific for AWS DynamoDB tables. |
| `aws_elb_load_balancer_details` | String | Output only. Asset information specific for AWS Load Balancers. |
| `aws_nat_gateway_details` | String | Output only. Asset information specific for AwsNatGatewayDetails |
| `aws_lambda_function_details` | String | Output only. Asset information specific for AWS Lambda functions. |
| `aws_route53_hosted_zone_details` | String | Output only. Asset information specific for AwsRoute53HostedZoneDetails |
| `attributes` | HashMap<String, String> | Generic asset attributes. |
| `labels` | HashMap<String, String> | Labels as key value pairs. |
| `aws_efs_file_system_details` | String | Output only. Asset information specific for AWS EFS file systems. |
| `aws_ecs_cluster_details` | String | Output only. Asset information specific for AWS ECS clusters. |
| `hidden` | bool | Optional. Indicates if the asset is hidden. |
| `aws_redshift_details` | String | Output only. Asset information specific for AWS Redshift |
| `assigned_groups` | Vec<String> | Output only. The list of groups that the asset is assigned to. |


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
asset_virtual_machine_details = asset.virtual_machine_details
asset_create_time = asset.create_time
asset_database_details = asset.database_details
asset_database_deployment_details = asset.database_deployment_details
asset_hide_reason = asset.hide_reason
asset_update_time = asset.update_time
asset_machine_details = asset.machine_details
asset_aws_eks_cluster_details = asset.aws_eks_cluster_details
asset_structured_attributes = asset.structured_attributes
asset_name = asset.name
asset_sources = asset.sources
asset_performance_data = asset.performance_data
asset_aws_vpc_details = asset.aws_vpc_details
asset_hosting_provider_details = asset.hosting_provider_details
asset_hide_time = asset.hide_time
asset_insight_list = asset.insight_list
asset_title = asset.title
asset_aws_cloud_front_distribution_details = asset.aws_cloud_front_distribution_details
asset_aws_s3_bucket_details = asset.aws_s3_bucket_details
asset_aws_dynamodb_table_details = asset.aws_dynamodb_table_details
asset_aws_elb_load_balancer_details = asset.aws_elb_load_balancer_details
asset_aws_nat_gateway_details = asset.aws_nat_gateway_details
asset_aws_lambda_function_details = asset.aws_lambda_function_details
asset_aws_route53_hosted_zone_details = asset.aws_route53_hosted_zone_details
asset_attributes = asset.attributes
asset_labels = asset.labels
asset_aws_efs_file_system_details = asset.aws_efs_file_system_details
asset_aws_ecs_cluster_details = asset.aws_ecs_cluster_details
asset_hidden = asset.hidden
asset_aws_redshift_details = asset.aws_redshift_details
asset_assigned_groups = asset.assigned_groups
```

---


### Group

Creates a new group in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The name of the group. |
| `create_time` | String |  | Output only. The timestamp when the group was created. |
| `display_name` | String |  | Optional. User-friendly display name. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs. |
| `description` | String |  | Optional. The description of the group. |
| `update_time` | String |  | Output only. The timestamp when the group was last updated. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The name of the group. |
| `create_time` | String | Output only. The timestamp when the group was created. |
| `display_name` | String | Optional. User-friendly display name. |
| `labels` | HashMap<String, String> | Labels as key value pairs. |
| `description` | String | Optional. The description of the group. |
| `update_time` | String | Output only. The timestamp when the group was last updated. |


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
group_name = group.name
group_create_time = group.create_time
group_display_name = group.display_name
group_labels = group.labels
group_description = group.description
group_update_time = group.update_time
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
operation = provider.migrationcenter_api.Operation {
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


### Assets_export_job

Creates a new assets export job.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Resource update time. |
| `network_dependencies` | String |  | Export data regarding asset network dependencies. |
| `create_time` | String |  | Output only. Resource creation time. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes. |
| `name` | String |  | Output only. Identifier. Resource name. |
| `show_hidden` | bool |  | Optional. When this value is set to 'true' the response will include all assets, including those that are hidden. |
| `recent_executions` | Vec<String> |  | Output only. Recent non expired executions of the job. |
| `performance_data` | String |  | Export asset with performance data. |
| `condition` | String |  | Optional. Conditions for selecting assets to export. |
| `inventory` | String |  | Export asset inventory details. |
| `signed_uri_destination` | String |  | Export to Cloud Storage files downloadable using signed URIs. |
| `parent` | String | ✅ | Required. The parent resource where the assts export job will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Resource update time. |
| `network_dependencies` | String | Export data regarding asset network dependencies. |
| `create_time` | String | Output only. Resource creation time. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes. |
| `name` | String | Output only. Identifier. Resource name. |
| `show_hidden` | bool | Optional. When this value is set to 'true' the response will include all assets, including those that are hidden. |
| `recent_executions` | Vec<String> | Output only. Recent non expired executions of the job. |
| `performance_data` | String | Export asset with performance data. |
| `condition` | String | Optional. Conditions for selecting assets to export. |
| `inventory` | String | Export asset inventory details. |
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
assets_export_job_update_time = assets_export_job.update_time
assets_export_job_network_dependencies = assets_export_job.network_dependencies
assets_export_job_create_time = assets_export_job.create_time
assets_export_job_labels = assets_export_job.labels
assets_export_job_name = assets_export_job.name
assets_export_job_show_hidden = assets_export_job.show_hidden
assets_export_job_recent_executions = assets_export_job.recent_executions
assets_export_job_performance_data = assets_export_job.performance_data
assets_export_job_condition = assets_export_job.condition
assets_export_job_inventory = assets_export_job.inventory
assets_export_job_signed_uri_destination = assets_export_job.signed_uri_destination
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The name of the resource. |
| `disable_cloud_logging` | bool |  | Disable Cloud Logging for the Migration Center API. Users are billed for the logs. |
| `preference_set` | String |  | The preference set used by default for a project. |
| `customer_consent_for_google_sales_to_access_migration_center` | bool |  | Customer consent for Google sales to access their Cloud Migration Center project. |
| `name` | String | ✅ | Output only. The name of the resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_location_id = location.location_id
location_labels = location.labels
location_display_name = location.display_name
location_metadata = location.metadata
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple group resources
group_0 = provider.migrationcenter_api.Group {
    parent = "value-0"
}
group_1 = provider.migrationcenter_api.Group {
    parent = "value-1"
}
group_2 = provider.migrationcenter_api.Group {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    group = provider.migrationcenter_api.Group {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Migrationcenter_api Documentation](https://cloud.google.com/migrationcenter_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
