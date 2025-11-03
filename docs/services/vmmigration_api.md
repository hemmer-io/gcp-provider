# Vmmigration_api Service



**Resources**: 28

---

## Overview

The vmmigration_api service provides access to 28 resource types:

- [Image_import](#image_import) [CRD]
- [Target_project](#target_project) [CRUD]
- [Group](#group) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CRD]
- [Image_import_job](#image_import_job) [CR]
- [Source](#source) [CRUD]
- [Datacenter_connector](#datacenter_connector) [CRD]
- [Utilization_report](#utilization_report) [CRD]
- [Migrating_vm](#migrating_vm) [CRUD]
- [Disk_migration_job](#disk_migration_job) [CRUD]
- [Replication_cycle](#replication_cycle) [R]
- [Clone_job](#clone_job) [CR]
- [Cutover_job](#cutover_job) [CR]
- [Datacenter_connector](#datacenter_connector) [CRD]
- [Migrating_vm](#migrating_vm) [CRUD]
- [Replication_cycle](#replication_cycle) [R]
- [Image_import_job](#image_import_job) [CR]
- [Source](#source) [CRUD]
- [Utilization_report](#utilization_report) [CRD]
- [Disk_migration_job](#disk_migration_job) [CRUD]
- [Target_project](#target_project) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Cutover_job](#cutover_job) [CR]
- [Image_import](#image_import) [CRD]
- [Group](#group) [CRUD]
- [Clone_job](#clone_job) [CR]

---

## Resources


### Image_import

Creates a new ImageImport in a given project.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `recent_image_import_jobs` | Vec<String> |  | Output only. The result of the most recent runs for this ImageImport. All jobs for this ImageImport can be listed via ListImageImportJobs. |
| `disk_image_target_defaults` | String |  | Immutable. Target details for importing a disk image, will be used by ImageImportJob. |
| `cloud_storage_uri` | String |  | Immutable. The path to the Cloud Storage file from which the image should be imported. |
| `create_time` | String |  | Output only. The time the image import was created. |
| `encryption` | String |  | Immutable. The encryption details used by the image import process during the image adaptation for Compute Engine. |
| `machine_image_target_defaults` | String |  | Immutable. Target details for importing a machine image, will be used by ImageImportJob. |
| `name` | String |  | Output only. The resource path of the ImageImport. |
| `parent` | String | ✅ | Required. The ImageImport's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recent_image_import_jobs` | Vec<String> | Output only. The result of the most recent runs for this ImageImport. All jobs for this ImageImport can be listed via ListImageImportJobs. |
| `disk_image_target_defaults` | String | Immutable. Target details for importing a disk image, will be used by ImageImportJob. |
| `cloud_storage_uri` | String | Immutable. The path to the Cloud Storage file from which the image should be imported. |
| `create_time` | String | Output only. The time the image import was created. |
| `encryption` | String | Immutable. The encryption details used by the image import process during the image adaptation for Compute Engine. |
| `machine_image_target_defaults` | String | Immutable. Target details for importing a machine image, will be used by ImageImportJob. |
| `name` | String | Output only. The resource path of the ImageImport. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create image_import
image_import = provider.vmmigration_api.Image_import {
    parent = "value"  # Required. The ImageImport's parent.
}

# Access image_import outputs
image_import_id = image_import.id
image_import_recent_image_import_jobs = image_import.recent_image_import_jobs
image_import_disk_image_target_defaults = image_import.disk_image_target_defaults
image_import_cloud_storage_uri = image_import.cloud_storage_uri
image_import_create_time = image_import.create_time
image_import_encryption = image_import.encryption
image_import_machine_image_target_defaults = image_import.machine_image_target_defaults
image_import_name = image_import.name
```

---


### Target_project

Creates a new TargetProject in a given project. NOTE: TargetProject is a global resource; hence the only supported value for location is `global`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | The target project's description. |
| `project` | String |  | Required. The target project ID (number) or project name. |
| `update_time` | String |  | Output only. The last time the target project resource was updated. |
| `create_time` | String |  | Output only. The time this target project resource was created (not related to when the Compute Engine project it points to was created). |
| `name` | String |  | Output only. The name of the target project. |
| `parent` | String | ✅ | Required. The TargetProject's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | The target project's description. |
| `project` | String | Required. The target project ID (number) or project name. |
| `update_time` | String | Output only. The last time the target project resource was updated. |
| `create_time` | String | Output only. The time this target project resource was created (not related to when the Compute Engine project it points to was created). |
| `name` | String | Output only. The name of the target project. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create target_project
target_project = provider.vmmigration_api.Target_project {
    parent = "value"  # Required. The TargetProject's parent.
}

# Access target_project outputs
target_project_id = target_project.id
target_project_description = target_project.description
target_project_project = target_project.project
target_project_update_time = target_project.update_time
target_project_create_time = target_project.create_time
target_project_name = target_project.name
```

---


### Group

Creates a new Group in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | User-provided description of the group. |
| `display_name` | String |  | Display name is a user defined name for this group which can be updated. |
| `migration_target_type` | String |  | Immutable. The target type of this group. |
| `name` | String |  | Output only. The Group name. |
| `update_time` | String |  | Output only. The update time timestamp. |
| `create_time` | String |  | Output only. The create time timestamp. |
| `parent` | String | ✅ | Required. The Group's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | User-provided description of the group. |
| `display_name` | String | Display name is a user defined name for this group which can be updated. |
| `migration_target_type` | String | Immutable. The target type of this group. |
| `name` | String | Output only. The Group name. |
| `update_time` | String | Output only. The update time timestamp. |
| `create_time` | String | Output only. The create time timestamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create group
group = provider.vmmigration_api.Group {
    parent = "value"  # Required. The Group's parent.
}

# Access group outputs
group_id = group.id
group_description = group.description
group_display_name = group.display_name
group_migration_target_type = group.migration_target_type
group_name = group.name
group_update_time = group.update_time
group_create_time = group.create_time
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
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
location_labels = location.labels
location_display_name = location.display_name
location_location_id = location.location_id
location_metadata = location.metadata
location_name = location.name
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
operation = provider.vmmigration_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_name = operation.name
operation_response = operation.response
operation_metadata = operation.metadata
```

---


### Image_import_job

Initiates the cancellation of a running ImageImportJob.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The image import job id. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_resources` | Vec<String> | Output only. The resource paths of the resources created by the image import job. |
| `machine_image_target_details` | String | Output only. Target details used to import a machine image. |
| `steps` | Vec<String> | Output only. The image import steps list representing its progress. |
| `end_time` | String | Output only. The time the image import was ended. |
| `cloud_storage_uri` | String | Output only. The path to the Cloud Storage file from which the image should be imported. |
| `name` | String | Output only. The resource path of the ImageImportJob. |
| `state` | String | Output only. The state of the image import. |
| `warnings` | Vec<String> | Output only. Warnings that occurred during the image import. |
| `create_time` | String | Output only. The time the image import was created (as an API call, not when it was actually created in the target). |
| `errors` | Vec<String> | Output only. Provides details on the error that led to the image import state in case of an error. |
| `disk_image_target_details` | String | Output only. Target details used to import a disk image. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create image_import_job
image_import_job = provider.vmmigration_api.Image_import_job {
    name = "value"  # Required. The image import job id.
}

# Access image_import_job outputs
image_import_job_id = image_import_job.id
image_import_job_created_resources = image_import_job.created_resources
image_import_job_machine_image_target_details = image_import_job.machine_image_target_details
image_import_job_steps = image_import_job.steps
image_import_job_end_time = image_import_job.end_time
image_import_job_cloud_storage_uri = image_import_job.cloud_storage_uri
image_import_job_name = image_import_job.name
image_import_job_state = image_import_job.state
image_import_job_warnings = image_import_job.warnings
image_import_job_create_time = image_import_job.create_time
image_import_job_errors = image_import_job.errors
image_import_job_disk_image_target_details = image_import_job.disk_image_target_details
```

---


### Source

Creates a new Source in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption` | String |  | Optional. Immutable. The encryption details of the source data stored by the service. |
| `aws` | String |  | AWS type source details. |
| `name` | String |  | Output only. The Source name. |
| `description` | String |  | User-provided description of the source. |
| `labels` | HashMap<String, String> |  | The labels of the source. |
| `azure` | String |  | Azure type source details. |
| `vmware` | String |  | Vmware type source details. |
| `create_time` | String |  | Output only. The create time timestamp. |
| `update_time` | String |  | Output only. The update time timestamp. |
| `parent` | String | ✅ | Required. The Source's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `encryption` | String | Optional. Immutable. The encryption details of the source data stored by the service. |
| `aws` | String | AWS type source details. |
| `name` | String | Output only. The Source name. |
| `description` | String | User-provided description of the source. |
| `labels` | HashMap<String, String> | The labels of the source. |
| `azure` | String | Azure type source details. |
| `vmware` | String | Vmware type source details. |
| `create_time` | String | Output only. The create time timestamp. |
| `update_time` | String | Output only. The update time timestamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create source
source = provider.vmmigration_api.Source {
    parent = "value"  # Required. The Source's parent.
}

# Access source outputs
source_id = source.id
source_encryption = source.encryption
source_aws = source.aws
source_name = source.name
source_description = source.description
source_labels = source.labels
source_azure = source.azure
source_vmware = source.vmware
source_create_time = source.create_time
source_update_time = source.update_time
```

---


### Datacenter_connector

Creates a new DatacenterConnector in a given Source.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The last time the connector was updated with an API call. |
| `upgrade_status` | String |  | Output only. The status of the current / last upgradeAppliance operation. |
| `available_versions` | String |  | Output only. The available versions for updating this appliance. |
| `state` | String |  | Output only. State of the DatacenterConnector, as determined by the health checks. |
| `appliance_infrastructure_version` | String |  | Output only. Appliance OVA version. This is the OVA which is manually installed by the user and contains the infrastructure for the automatically updatable components on the appliance. |
| `bucket` | String |  | Output only. The communication channel between the datacenter connector and Google Cloud. |
| `create_time` | String |  | Output only. The time the connector was created (as an API call, not when it was actually installed). |
| `service_account` | String |  | The service account to use in the connector when communicating with the cloud. |
| `error` | String |  | Output only. Provides details on the state of the Datacenter Connector in case of an error. |
| `version` | String |  | The version running in the DatacenterConnector. This is supplied by the OVA connector during the registration process and can not be modified. |
| `state_time` | String |  | Output only. The time the state was last set. |
| `registration_id` | String |  | Immutable. A unique key for this connector. This key is internal to the OVA connector and is supplied with its creation during the registration process and can not be modified. |
| `name` | String |  | Output only. The connector's name. |
| `appliance_software_version` | String |  | Output only. Appliance last installed update bundle version. This is the version of the automatically updatable components on the appliance. |
| `parent` | String | ✅ | Required. The DatacenterConnector's parent. Required. The Source in where the new DatacenterConnector will be created. For example: `projects/my-project/locations/us-central1/sources/my-source` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The last time the connector was updated with an API call. |
| `upgrade_status` | String | Output only. The status of the current / last upgradeAppliance operation. |
| `available_versions` | String | Output only. The available versions for updating this appliance. |
| `state` | String | Output only. State of the DatacenterConnector, as determined by the health checks. |
| `appliance_infrastructure_version` | String | Output only. Appliance OVA version. This is the OVA which is manually installed by the user and contains the infrastructure for the automatically updatable components on the appliance. |
| `bucket` | String | Output only. The communication channel between the datacenter connector and Google Cloud. |
| `create_time` | String | Output only. The time the connector was created (as an API call, not when it was actually installed). |
| `service_account` | String | The service account to use in the connector when communicating with the cloud. |
| `error` | String | Output only. Provides details on the state of the Datacenter Connector in case of an error. |
| `version` | String | The version running in the DatacenterConnector. This is supplied by the OVA connector during the registration process and can not be modified. |
| `state_time` | String | Output only. The time the state was last set. |
| `registration_id` | String | Immutable. A unique key for this connector. This key is internal to the OVA connector and is supplied with its creation during the registration process and can not be modified. |
| `name` | String | Output only. The connector's name. |
| `appliance_software_version` | String | Output only. Appliance last installed update bundle version. This is the version of the automatically updatable components on the appliance. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create datacenter_connector
datacenter_connector = provider.vmmigration_api.Datacenter_connector {
    parent = "value"  # Required. The DatacenterConnector's parent. Required. The Source in where the new DatacenterConnector will be created. For example: `projects/my-project/locations/us-central1/sources/my-source`
}

# Access datacenter_connector outputs
datacenter_connector_id = datacenter_connector.id
datacenter_connector_update_time = datacenter_connector.update_time
datacenter_connector_upgrade_status = datacenter_connector.upgrade_status
datacenter_connector_available_versions = datacenter_connector.available_versions
datacenter_connector_state = datacenter_connector.state
datacenter_connector_appliance_infrastructure_version = datacenter_connector.appliance_infrastructure_version
datacenter_connector_bucket = datacenter_connector.bucket
datacenter_connector_create_time = datacenter_connector.create_time
datacenter_connector_service_account = datacenter_connector.service_account
datacenter_connector_error = datacenter_connector.error
datacenter_connector_version = datacenter_connector.version
datacenter_connector_state_time = datacenter_connector.state_time
datacenter_connector_registration_id = datacenter_connector.registration_id
datacenter_connector_name = datacenter_connector.name
datacenter_connector_appliance_software_version = datacenter_connector.appliance_software_version
```

---


### Utilization_report

Creates a new UtilizationReport.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | The report display name, as assigned by the user. |
| `vm_count` | i64 |  | Output only. Total number of VMs included in the report. |
| `name` | String |  | Output only. The report unique name. |
| `vms` | Vec<String> |  | List of utilization information per VM. When sent as part of the request, the "vm_id" field is used in order to specify which VMs to include in the report. In that case all other fields are ignored. |
| `frame_end_time` | String |  | Output only. The point in time when the time frame ends. Notice that the time frame is counted backwards. For instance if the "frame_end_time" value is 2021/01/20 and the time frame is WEEK then the report covers the week between 2021/01/20 and 2021/01/14. |
| `state_time` | String |  | Output only. The time the state was last set. |
| `error` | String |  | Output only. Provides details on the state of the report in case of an error. |
| `time_frame` | String |  | Time frame of the report. |
| `create_time` | String |  | Output only. The time the report was created (this refers to the time of the request, not the time the report creation completed). |
| `state` | String |  | Output only. Current state of the report. |
| `parent` | String | ✅ | Required. The Utilization Report's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The report display name, as assigned by the user. |
| `vm_count` | i64 | Output only. Total number of VMs included in the report. |
| `name` | String | Output only. The report unique name. |
| `vms` | Vec<String> | List of utilization information per VM. When sent as part of the request, the "vm_id" field is used in order to specify which VMs to include in the report. In that case all other fields are ignored. |
| `frame_end_time` | String | Output only. The point in time when the time frame ends. Notice that the time frame is counted backwards. For instance if the "frame_end_time" value is 2021/01/20 and the time frame is WEEK then the report covers the week between 2021/01/20 and 2021/01/14. |
| `state_time` | String | Output only. The time the state was last set. |
| `error` | String | Output only. Provides details on the state of the report in case of an error. |
| `time_frame` | String | Time frame of the report. |
| `create_time` | String | Output only. The time the report was created (this refers to the time of the request, not the time the report creation completed). |
| `state` | String | Output only. Current state of the report. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create utilization_report
utilization_report = provider.vmmigration_api.Utilization_report {
    parent = "value"  # Required. The Utilization Report's parent.
}

# Access utilization_report outputs
utilization_report_id = utilization_report.id
utilization_report_display_name = utilization_report.display_name
utilization_report_vm_count = utilization_report.vm_count
utilization_report_name = utilization_report.name
utilization_report_vms = utilization_report.vms
utilization_report_frame_end_time = utilization_report.frame_end_time
utilization_report_state_time = utilization_report.state_time
utilization_report_error = utilization_report.error
utilization_report_time_frame = utilization_report.time_frame
utilization_report_create_time = utilization_report.create_time
utilization_report_state = utilization_report.state
```

---


### Migrating_vm

Creates a new MigratingVm in a given Source.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `error` | String |  | Output only. Provides details on the state of the Migrating VM in case of an error in replication. |
| `vmware_source_vm_details` | String |  | Output only. Details of the VM from a Vmware source. |
| `recent_cutover_jobs` | Vec<String> |  | Output only. The recent cutover jobs performed on the migrating VM. This field holds the vm's last completed cutover job and the vm's running cutover job, if one exists. Note: To have this field populated you need to explicitly request it via the "view" parameter of the Get/List request. |
| `state` | String |  | Output only. State of the MigratingVm. |
| `create_time` | String |  | Output only. The time the migrating VM was created (this refers to this resource and not to the time it was installed in the source). |
| `current_sync_info` | String |  | Output only. Details of the current running replication cycle. |
| `name` | String |  | Output only. The identifier of the MigratingVm. |
| `display_name` | String |  | The display name attached to the MigratingVm by the user. |
| `expiration` | String |  | Output only. Provides details about the expiration state of the migrating VM. |
| `cutover_forecast` | String |  | Output only. Provides details of future CutoverJobs of a MigratingVm. Set to empty when cutover forecast is unavailable. |
| `description` | String |  | The description attached to the migrating VM by the user. |
| `azure_source_vm_details` | String |  | Output only. Details of the VM from an Azure source. |
| `aws_source_vm_details` | String |  | Output only. Details of the VM from an AWS source. |
| `compute_engine_disks_target_defaults` | String |  | Details of the target Persistent Disks in Compute Engine. |
| `last_sync` | String |  | Output only. The most updated snapshot created time in the source that finished replication. |
| `recent_clone_jobs` | Vec<String> |  | Output only. The recent clone jobs performed on the migrating VM. This field holds the vm's last completed clone job and the vm's running clone job, if one exists. Note: To have this field populated you need to explicitly request it via the "view" parameter of the Get/List request. |
| `source_vm_id` | String |  | The unique ID of the VM in the source. The VM's name in vSphere can be changed, so this is not the VM's name but rather its moRef id. This id is of the form vm-. |
| `state_time` | String |  | Output only. The last time the migrating VM state was updated. |
| `labels` | HashMap<String, String> |  | The labels of the migrating VM. |
| `compute_engine_target_defaults` | String |  | Details of the target VM in Compute Engine. |
| `update_time` | String |  | Output only. The last time the migrating VM resource was updated. |
| `last_replication_cycle` | String |  | Output only. Details of the last replication cycle. This will be updated whenever a replication cycle is finished and is not to be confused with last_sync which is only updated on successful replication cycles. |
| `policy` | String |  | The replication schedule policy. |
| `group` | String |  | Output only. The group this migrating vm is included in, if any. The group is represented by the full path of the appropriate Group resource. |
| `parent` | String | ✅ | Required. The MigratingVm's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | Output only. Provides details on the state of the Migrating VM in case of an error in replication. |
| `vmware_source_vm_details` | String | Output only. Details of the VM from a Vmware source. |
| `recent_cutover_jobs` | Vec<String> | Output only. The recent cutover jobs performed on the migrating VM. This field holds the vm's last completed cutover job and the vm's running cutover job, if one exists. Note: To have this field populated you need to explicitly request it via the "view" parameter of the Get/List request. |
| `state` | String | Output only. State of the MigratingVm. |
| `create_time` | String | Output only. The time the migrating VM was created (this refers to this resource and not to the time it was installed in the source). |
| `current_sync_info` | String | Output only. Details of the current running replication cycle. |
| `name` | String | Output only. The identifier of the MigratingVm. |
| `display_name` | String | The display name attached to the MigratingVm by the user. |
| `expiration` | String | Output only. Provides details about the expiration state of the migrating VM. |
| `cutover_forecast` | String | Output only. Provides details of future CutoverJobs of a MigratingVm. Set to empty when cutover forecast is unavailable. |
| `description` | String | The description attached to the migrating VM by the user. |
| `azure_source_vm_details` | String | Output only. Details of the VM from an Azure source. |
| `aws_source_vm_details` | String | Output only. Details of the VM from an AWS source. |
| `compute_engine_disks_target_defaults` | String | Details of the target Persistent Disks in Compute Engine. |
| `last_sync` | String | Output only. The most updated snapshot created time in the source that finished replication. |
| `recent_clone_jobs` | Vec<String> | Output only. The recent clone jobs performed on the migrating VM. This field holds the vm's last completed clone job and the vm's running clone job, if one exists. Note: To have this field populated you need to explicitly request it via the "view" parameter of the Get/List request. |
| `source_vm_id` | String | The unique ID of the VM in the source. The VM's name in vSphere can be changed, so this is not the VM's name but rather its moRef id. This id is of the form vm-. |
| `state_time` | String | Output only. The last time the migrating VM state was updated. |
| `labels` | HashMap<String, String> | The labels of the migrating VM. |
| `compute_engine_target_defaults` | String | Details of the target VM in Compute Engine. |
| `update_time` | String | Output only. The last time the migrating VM resource was updated. |
| `last_replication_cycle` | String | Output only. Details of the last replication cycle. This will be updated whenever a replication cycle is finished and is not to be confused with last_sync which is only updated on successful replication cycles. |
| `policy` | String | The replication schedule policy. |
| `group` | String | Output only. The group this migrating vm is included in, if any. The group is represented by the full path of the appropriate Group resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create migrating_vm
migrating_vm = provider.vmmigration_api.Migrating_vm {
    parent = "value"  # Required. The MigratingVm's parent.
}

# Access migrating_vm outputs
migrating_vm_id = migrating_vm.id
migrating_vm_error = migrating_vm.error
migrating_vm_vmware_source_vm_details = migrating_vm.vmware_source_vm_details
migrating_vm_recent_cutover_jobs = migrating_vm.recent_cutover_jobs
migrating_vm_state = migrating_vm.state
migrating_vm_create_time = migrating_vm.create_time
migrating_vm_current_sync_info = migrating_vm.current_sync_info
migrating_vm_name = migrating_vm.name
migrating_vm_display_name = migrating_vm.display_name
migrating_vm_expiration = migrating_vm.expiration
migrating_vm_cutover_forecast = migrating_vm.cutover_forecast
migrating_vm_description = migrating_vm.description
migrating_vm_azure_source_vm_details = migrating_vm.azure_source_vm_details
migrating_vm_aws_source_vm_details = migrating_vm.aws_source_vm_details
migrating_vm_compute_engine_disks_target_defaults = migrating_vm.compute_engine_disks_target_defaults
migrating_vm_last_sync = migrating_vm.last_sync
migrating_vm_recent_clone_jobs = migrating_vm.recent_clone_jobs
migrating_vm_source_vm_id = migrating_vm.source_vm_id
migrating_vm_state_time = migrating_vm.state_time
migrating_vm_labels = migrating_vm.labels
migrating_vm_compute_engine_target_defaults = migrating_vm.compute_engine_target_defaults
migrating_vm_update_time = migrating_vm.update_time
migrating_vm_last_replication_cycle = migrating_vm.last_replication_cycle
migrating_vm_policy = migrating_vm.policy
migrating_vm_group = migrating_vm.group
```

---


### Disk_migration_job

Creates a new disk migration job in a given Source.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The last time the DiskMigrationJob resource was updated. |
| `errors` | Vec<String> |  | Output only. Provides details on the errors that led to the disk migration job's state in case of an error. |
| `steps` | Vec<String> |  | Output only. The disk migration steps list representing its progress. |
| `create_time` | String |  | Output only. The time the DiskMigrationJob resource was created. |
| `name` | String |  | Output only. Identifier. The identifier of the DiskMigrationJob. |
| `state` | String |  | Output only. State of the DiskMigrationJob. |
| `aws_source_disk_details` | String |  | Details of the unattached AWS source disk. |
| `target_details` | String |  | Required. Details of the target Disk in Compute Engine. |
| `parent` | String | ✅ | Required. The DiskMigrationJob's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The last time the DiskMigrationJob resource was updated. |
| `errors` | Vec<String> | Output only. Provides details on the errors that led to the disk migration job's state in case of an error. |
| `steps` | Vec<String> | Output only. The disk migration steps list representing its progress. |
| `create_time` | String | Output only. The time the DiskMigrationJob resource was created. |
| `name` | String | Output only. Identifier. The identifier of the DiskMigrationJob. |
| `state` | String | Output only. State of the DiskMigrationJob. |
| `aws_source_disk_details` | String | Details of the unattached AWS source disk. |
| `target_details` | String | Required. Details of the target Disk in Compute Engine. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create disk_migration_job
disk_migration_job = provider.vmmigration_api.Disk_migration_job {
    parent = "value"  # Required. The DiskMigrationJob's parent.
}

# Access disk_migration_job outputs
disk_migration_job_id = disk_migration_job.id
disk_migration_job_update_time = disk_migration_job.update_time
disk_migration_job_errors = disk_migration_job.errors
disk_migration_job_steps = disk_migration_job.steps
disk_migration_job_create_time = disk_migration_job.create_time
disk_migration_job_name = disk_migration_job.name
disk_migration_job_state = disk_migration_job.state
disk_migration_job_aws_source_disk_details = disk_migration_job.aws_source_disk_details
disk_migration_job_target_details = disk_migration_job.target_details
```

---


### Replication_cycle

Gets details of a single ReplicationCycle.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `progress_percent` | i64 | The current progress in percentage of this cycle. Was replaced by 'steps' field, which breaks down the cycle progression more accurately. |
| `warnings` | Vec<String> | Output only. Warnings that occurred during the cycle. |
| `state` | String | State of the ReplicationCycle. |
| `total_pause_duration` | String | The accumulated duration the replication cycle was paused. |
| `error` | String | Output only. Provides details on the state of the cycle in case of an error. |
| `start_time` | String | The time the replication cycle has started. |
| `steps` | Vec<String> | The cycle's steps list representing its progress. |
| `end_time` | String | The time the replication cycle has ended. |
| `name` | String | The identifier of the ReplicationCycle. |
| `cycle_number` | i64 | The cycle's ordinal number. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access replication_cycle outputs
replication_cycle_id = replication_cycle.id
replication_cycle_progress_percent = replication_cycle.progress_percent
replication_cycle_warnings = replication_cycle.warnings
replication_cycle_state = replication_cycle.state
replication_cycle_total_pause_duration = replication_cycle.total_pause_duration
replication_cycle_error = replication_cycle.error
replication_cycle_start_time = replication_cycle.start_time
replication_cycle_steps = replication_cycle.steps
replication_cycle_end_time = replication_cycle.end_time
replication_cycle_name = replication_cycle.name
replication_cycle_cycle_number = replication_cycle.cycle_number
```

---


### Clone_job

Initiates a Clone of a specific migrating VM.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `compute_engine_target_details` | String |  | Output only. Details of the target VM in Compute Engine. |
| `name` | String |  | Output only. The name of the clone. |
| `steps` | Vec<String> |  | Output only. The clone steps list representing its progress. |
| `state_time` | String |  | Output only. The time the state was last updated. |
| `create_time` | String |  | Output only. The time the clone job was created (as an API call, not when it was actually created in the target). |
| `end_time` | String |  | Output only. The time the clone job was ended. |
| `error` | String |  | Output only. Provides details for the errors that led to the Clone Job's state. |
| `compute_engine_disks_target_details` | String |  | Output only. Details of the target Persistent Disks in Compute Engine. |
| `state` | String |  | Output only. State of the clone job. |
| `parent` | String | ✅ | Required. The Clone's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `compute_engine_target_details` | String | Output only. Details of the target VM in Compute Engine. |
| `name` | String | Output only. The name of the clone. |
| `steps` | Vec<String> | Output only. The clone steps list representing its progress. |
| `state_time` | String | Output only. The time the state was last updated. |
| `create_time` | String | Output only. The time the clone job was created (as an API call, not when it was actually created in the target). |
| `end_time` | String | Output only. The time the clone job was ended. |
| `error` | String | Output only. Provides details for the errors that led to the Clone Job's state. |
| `compute_engine_disks_target_details` | String | Output only. Details of the target Persistent Disks in Compute Engine. |
| `state` | String | Output only. State of the clone job. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create clone_job
clone_job = provider.vmmigration_api.Clone_job {
    parent = "value"  # Required. The Clone's parent.
}

# Access clone_job outputs
clone_job_id = clone_job.id
clone_job_compute_engine_target_details = clone_job.compute_engine_target_details
clone_job_name = clone_job.name
clone_job_steps = clone_job.steps
clone_job_state_time = clone_job.state_time
clone_job_create_time = clone_job.create_time
clone_job_end_time = clone_job.end_time
clone_job_error = clone_job.error
clone_job_compute_engine_disks_target_details = clone_job.compute_engine_disks_target_details
clone_job_state = clone_job.state
```

---


### Cutover_job

Initiates a Cutover of a specific migrating VM. The returned LRO is completed when the cutover job resource is created and the job is initiated.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. State of the cutover job. |
| `compute_engine_target_details` | String |  | Output only. Details of the target VM in Compute Engine. |
| `create_time` | String |  | Output only. The time the cutover job was created (as an API call, not when it was actually created in the target). |
| `end_time` | String |  | Output only. The time the cutover job had finished. |
| `error` | String |  | Output only. Provides details for the errors that led to the Cutover Job's state. |
| `name` | String |  | Output only. The name of the cutover job. |
| `state_message` | String |  | Output only. A message providing possible extra details about the current state. |
| `compute_engine_disks_target_details` | String |  | Output only. Details of the target Persistent Disks in Compute Engine. |
| `state_time` | String |  | Output only. The time the state was last updated. |
| `steps` | Vec<String> |  | Output only. The cutover steps list representing its progress. |
| `progress_percent` | i64 |  | Output only. The current progress in percentage of the cutover job. |
| `parent` | String | ✅ | Required. The Cutover's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the cutover job. |
| `compute_engine_target_details` | String | Output only. Details of the target VM in Compute Engine. |
| `create_time` | String | Output only. The time the cutover job was created (as an API call, not when it was actually created in the target). |
| `end_time` | String | Output only. The time the cutover job had finished. |
| `error` | String | Output only. Provides details for the errors that led to the Cutover Job's state. |
| `name` | String | Output only. The name of the cutover job. |
| `state_message` | String | Output only. A message providing possible extra details about the current state. |
| `compute_engine_disks_target_details` | String | Output only. Details of the target Persistent Disks in Compute Engine. |
| `state_time` | String | Output only. The time the state was last updated. |
| `steps` | Vec<String> | Output only. The cutover steps list representing its progress. |
| `progress_percent` | i64 | Output only. The current progress in percentage of the cutover job. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cutover_job
cutover_job = provider.vmmigration_api.Cutover_job {
    parent = "value"  # Required. The Cutover's parent.
}

# Access cutover_job outputs
cutover_job_id = cutover_job.id
cutover_job_state = cutover_job.state
cutover_job_compute_engine_target_details = cutover_job.compute_engine_target_details
cutover_job_create_time = cutover_job.create_time
cutover_job_end_time = cutover_job.end_time
cutover_job_error = cutover_job.error
cutover_job_name = cutover_job.name
cutover_job_state_message = cutover_job.state_message
cutover_job_compute_engine_disks_target_details = cutover_job.compute_engine_disks_target_details
cutover_job_state_time = cutover_job.state_time
cutover_job_steps = cutover_job.steps
cutover_job_progress_percent = cutover_job.progress_percent
```

---


### Datacenter_connector

Creates a new DatacenterConnector in a given Source.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_account` | String |  | The service account to use in the connector when communicating with the cloud. |
| `state` | String |  | Output only. State of the DatacenterConnector, as determined by the health checks. |
| `error` | String |  | Output only. Provides details on the state of the Datacenter Connector in case of an error. |
| `upgrade_status` | String |  | Output only. The status of the current / last upgradeAppliance operation. |
| `version` | String |  | The version running in the DatacenterConnector. This is supplied by the OVA connector during the registration process and can not be modified. |
| `available_versions` | String |  | Output only. The available versions for updating this appliance. |
| `update_time` | String |  | Output only. The last time the connector was updated with an API call. |
| `state_time` | String |  | Output only. The time the state was last set. |
| `registration_id` | String |  | Immutable. A unique key for this connector. This key is internal to the OVA connector and is supplied with its creation during the registration process and can not be modified. |
| `name` | String |  | Output only. The connector's name. |
| `appliance_infrastructure_version` | String |  | Output only. Appliance OVA version. This is the OVA which is manually installed by the user and contains the infrastructure for the automatically updatable components on the appliance. |
| `bucket` | String |  | Output only. The communication channel between the datacenter connector and Google Cloud. |
| `appliance_software_version` | String |  | Output only. Appliance last installed update bundle version. This is the version of the automatically updatable components on the appliance. |
| `create_time` | String |  | Output only. The time the connector was created (as an API call, not when it was actually installed). |
| `parent` | String | ✅ | Required. The DatacenterConnector's parent. Required. The Source in where the new DatacenterConnector will be created. For example: `projects/my-project/locations/us-central1/sources/my-source` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_account` | String | The service account to use in the connector when communicating with the cloud. |
| `state` | String | Output only. State of the DatacenterConnector, as determined by the health checks. |
| `error` | String | Output only. Provides details on the state of the Datacenter Connector in case of an error. |
| `upgrade_status` | String | Output only. The status of the current / last upgradeAppliance operation. |
| `version` | String | The version running in the DatacenterConnector. This is supplied by the OVA connector during the registration process and can not be modified. |
| `available_versions` | String | Output only. The available versions for updating this appliance. |
| `update_time` | String | Output only. The last time the connector was updated with an API call. |
| `state_time` | String | Output only. The time the state was last set. |
| `registration_id` | String | Immutable. A unique key for this connector. This key is internal to the OVA connector and is supplied with its creation during the registration process and can not be modified. |
| `name` | String | Output only. The connector's name. |
| `appliance_infrastructure_version` | String | Output only. Appliance OVA version. This is the OVA which is manually installed by the user and contains the infrastructure for the automatically updatable components on the appliance. |
| `bucket` | String | Output only. The communication channel between the datacenter connector and Google Cloud. |
| `appliance_software_version` | String | Output only. Appliance last installed update bundle version. This is the version of the automatically updatable components on the appliance. |
| `create_time` | String | Output only. The time the connector was created (as an API call, not when it was actually installed). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create datacenter_connector
datacenter_connector = provider.vmmigration_api.Datacenter_connector {
    parent = "value"  # Required. The DatacenterConnector's parent. Required. The Source in where the new DatacenterConnector will be created. For example: `projects/my-project/locations/us-central1/sources/my-source`
}

# Access datacenter_connector outputs
datacenter_connector_id = datacenter_connector.id
datacenter_connector_service_account = datacenter_connector.service_account
datacenter_connector_state = datacenter_connector.state
datacenter_connector_error = datacenter_connector.error
datacenter_connector_upgrade_status = datacenter_connector.upgrade_status
datacenter_connector_version = datacenter_connector.version
datacenter_connector_available_versions = datacenter_connector.available_versions
datacenter_connector_update_time = datacenter_connector.update_time
datacenter_connector_state_time = datacenter_connector.state_time
datacenter_connector_registration_id = datacenter_connector.registration_id
datacenter_connector_name = datacenter_connector.name
datacenter_connector_appliance_infrastructure_version = datacenter_connector.appliance_infrastructure_version
datacenter_connector_bucket = datacenter_connector.bucket
datacenter_connector_appliance_software_version = datacenter_connector.appliance_software_version
datacenter_connector_create_time = datacenter_connector.create_time
```

---


### Migrating_vm

Creates a new MigratingVm in a given Source.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `current_sync_info` | String |  | Output only. Details of the current running replication cycle. |
| `aws_source_vm_details` | String |  | Output only. Details of the VM from an AWS source. |
| `policy` | String |  | The replication schedule policy. |
| `error` | String |  | Output only. Provides details on the state of the Migrating VM in case of an error in replication. |
| `last_replication_cycle` | String |  | Output only. Details of the last replication cycle. This will be updated whenever a replication cycle is finished and is not to be confused with last_sync which is only updated on successful replication cycles. |
| `recent_clone_jobs` | Vec<String> |  | Output only. The recent clone jobs performed on the migrating VM. This field holds the vm's last completed clone job and the vm's running clone job, if one exists. Note: To have this field populated you need to explicitly request it via the "view" parameter of the Get/List request. |
| `create_time` | String |  | Output only. The time the migrating VM was created (this refers to this resource and not to the time it was installed in the source). |
| `compute_engine_target_defaults` | String |  | Details of the target VM in Compute Engine. |
| `target_defaults` | String |  | The default configuration of the target VM that will be created in Google Cloud as a result of the migration. Deprecated: Use compute_engine_target_defaults instead. |
| `cutover_forecast` | String |  | Output only. Provides details of future CutoverJobs of a MigratingVm. Set to empty when cutover forecast is unavailable. |
| `display_name` | String |  | The display name attached to the MigratingVm by the user. |
| `last_sync` | String |  | Output only. The most updated snapshot created time in the source that finished replication. |
| `state_time` | String |  | Output only. The last time the migrating VM state was updated. |
| `expiration` | String |  | Output only. Provides details about the expiration state of the migrating VM. |
| `source_vm_id` | String |  | The unique ID of the VM in the source. The VM's name in vSphere can be changed, so this is not the VM's name but rather its moRef id. This id is of the form vm-. |
| `update_time` | String |  | Output only. The last time the migrating VM resource was updated. |
| `state` | String |  | Output only. State of the MigratingVm. |
| `compute_engine_vm_defaults` | String |  | Details of the VM in Compute Engine. Deprecated: Use compute_engine_target_defaults instead. |
| `group` | String |  | Output only. The group this migrating vm is included in, if any. The group is represented by the full path of the appropriate Group resource. |
| `labels` | HashMap<String, String> |  | The labels of the migrating VM. |
| `name` | String |  | Output only. The identifier of the MigratingVm. |
| `vmware_source_vm_details` | String |  | Output only. Details of the VM from a Vmware source. |
| `recent_cutover_jobs` | Vec<String> |  | Output only. The recent cutover jobs performed on the migrating VM. This field holds the vm's last completed cutover job and the vm's running cutover job, if one exists. Note: To have this field populated you need to explicitly request it via the "view" parameter of the Get/List request. |
| `azure_source_vm_details` | String |  | Output only. Details of the VM from an Azure source. |
| `compute_engine_disks_target_defaults` | String |  | Details of the target Persistent Disks in Compute Engine. |
| `description` | String |  | The description attached to the migrating VM by the user. |
| `parent` | String | ✅ | Required. The MigratingVm's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `current_sync_info` | String | Output only. Details of the current running replication cycle. |
| `aws_source_vm_details` | String | Output only. Details of the VM from an AWS source. |
| `policy` | String | The replication schedule policy. |
| `error` | String | Output only. Provides details on the state of the Migrating VM in case of an error in replication. |
| `last_replication_cycle` | String | Output only. Details of the last replication cycle. This will be updated whenever a replication cycle is finished and is not to be confused with last_sync which is only updated on successful replication cycles. |
| `recent_clone_jobs` | Vec<String> | Output only. The recent clone jobs performed on the migrating VM. This field holds the vm's last completed clone job and the vm's running clone job, if one exists. Note: To have this field populated you need to explicitly request it via the "view" parameter of the Get/List request. |
| `create_time` | String | Output only. The time the migrating VM was created (this refers to this resource and not to the time it was installed in the source). |
| `compute_engine_target_defaults` | String | Details of the target VM in Compute Engine. |
| `target_defaults` | String | The default configuration of the target VM that will be created in Google Cloud as a result of the migration. Deprecated: Use compute_engine_target_defaults instead. |
| `cutover_forecast` | String | Output only. Provides details of future CutoverJobs of a MigratingVm. Set to empty when cutover forecast is unavailable. |
| `display_name` | String | The display name attached to the MigratingVm by the user. |
| `last_sync` | String | Output only. The most updated snapshot created time in the source that finished replication. |
| `state_time` | String | Output only. The last time the migrating VM state was updated. |
| `expiration` | String | Output only. Provides details about the expiration state of the migrating VM. |
| `source_vm_id` | String | The unique ID of the VM in the source. The VM's name in vSphere can be changed, so this is not the VM's name but rather its moRef id. This id is of the form vm-. |
| `update_time` | String | Output only. The last time the migrating VM resource was updated. |
| `state` | String | Output only. State of the MigratingVm. |
| `compute_engine_vm_defaults` | String | Details of the VM in Compute Engine. Deprecated: Use compute_engine_target_defaults instead. |
| `group` | String | Output only. The group this migrating vm is included in, if any. The group is represented by the full path of the appropriate Group resource. |
| `labels` | HashMap<String, String> | The labels of the migrating VM. |
| `name` | String | Output only. The identifier of the MigratingVm. |
| `vmware_source_vm_details` | String | Output only. Details of the VM from a Vmware source. |
| `recent_cutover_jobs` | Vec<String> | Output only. The recent cutover jobs performed on the migrating VM. This field holds the vm's last completed cutover job and the vm's running cutover job, if one exists. Note: To have this field populated you need to explicitly request it via the "view" parameter of the Get/List request. |
| `azure_source_vm_details` | String | Output only. Details of the VM from an Azure source. |
| `compute_engine_disks_target_defaults` | String | Details of the target Persistent Disks in Compute Engine. |
| `description` | String | The description attached to the migrating VM by the user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create migrating_vm
migrating_vm = provider.vmmigration_api.Migrating_vm {
    parent = "value"  # Required. The MigratingVm's parent.
}

# Access migrating_vm outputs
migrating_vm_id = migrating_vm.id
migrating_vm_current_sync_info = migrating_vm.current_sync_info
migrating_vm_aws_source_vm_details = migrating_vm.aws_source_vm_details
migrating_vm_policy = migrating_vm.policy
migrating_vm_error = migrating_vm.error
migrating_vm_last_replication_cycle = migrating_vm.last_replication_cycle
migrating_vm_recent_clone_jobs = migrating_vm.recent_clone_jobs
migrating_vm_create_time = migrating_vm.create_time
migrating_vm_compute_engine_target_defaults = migrating_vm.compute_engine_target_defaults
migrating_vm_target_defaults = migrating_vm.target_defaults
migrating_vm_cutover_forecast = migrating_vm.cutover_forecast
migrating_vm_display_name = migrating_vm.display_name
migrating_vm_last_sync = migrating_vm.last_sync
migrating_vm_state_time = migrating_vm.state_time
migrating_vm_expiration = migrating_vm.expiration
migrating_vm_source_vm_id = migrating_vm.source_vm_id
migrating_vm_update_time = migrating_vm.update_time
migrating_vm_state = migrating_vm.state
migrating_vm_compute_engine_vm_defaults = migrating_vm.compute_engine_vm_defaults
migrating_vm_group = migrating_vm.group
migrating_vm_labels = migrating_vm.labels
migrating_vm_name = migrating_vm.name
migrating_vm_vmware_source_vm_details = migrating_vm.vmware_source_vm_details
migrating_vm_recent_cutover_jobs = migrating_vm.recent_cutover_jobs
migrating_vm_azure_source_vm_details = migrating_vm.azure_source_vm_details
migrating_vm_compute_engine_disks_target_defaults = migrating_vm.compute_engine_disks_target_defaults
migrating_vm_description = migrating_vm.description
```

---


### Replication_cycle

Gets details of a single ReplicationCycle.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_pause_duration` | String | The accumulated duration the replication cycle was paused. |
| `progress` | i64 | The current progress in percentage of this cycle. |
| `start_time` | String | The time the replication cycle has started. |
| `state` | String | State of the ReplicationCycle. |
| `cycle_number` | i64 | The cycle's ordinal number. |
| `end_time` | String | The time the replication cycle has ended. |
| `steps` | Vec<String> | The cycle's steps list representing its progress. |
| `warnings` | Vec<String> | Output only. Warnings that occurred during the cycle. |
| `progress_percent` | i64 | The current progress in percentage of this cycle. Was replaced by 'steps' field, which breaks down the cycle progression more accurately. |
| `name` | String | The identifier of the ReplicationCycle. |
| `error` | String | Output only. Provides details on the state of the cycle in case of an error. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access replication_cycle outputs
replication_cycle_id = replication_cycle.id
replication_cycle_total_pause_duration = replication_cycle.total_pause_duration
replication_cycle_progress = replication_cycle.progress
replication_cycle_start_time = replication_cycle.start_time
replication_cycle_state = replication_cycle.state
replication_cycle_cycle_number = replication_cycle.cycle_number
replication_cycle_end_time = replication_cycle.end_time
replication_cycle_steps = replication_cycle.steps
replication_cycle_warnings = replication_cycle.warnings
replication_cycle_progress_percent = replication_cycle.progress_percent
replication_cycle_name = replication_cycle.name
replication_cycle_error = replication_cycle.error
```

---


### Image_import_job

Initiates the cancellation of a running ImageImportJob.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The image import job id. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The state of the image import. |
| `disk_image_target_details` | String | Output only. Target details used to import a disk image. |
| `end_time` | String | Output only. The time the image import was ended. |
| `name` | String | Output only. The resource path of the ImageImportJob. |
| `created_resources` | Vec<String> | Output only. The resource paths of the resources created by the image import job. |
| `steps` | Vec<String> | Output only. The image import steps list representing its progress. |
| `warnings` | Vec<String> | Output only. Warnings that occurred during the image import. |
| `cloud_storage_uri` | String | Output only. The path to the Cloud Storage file from which the image should be imported. |
| `create_time` | String | Output only. The time the image import was created (as an API call, not when it was actually created in the target). |
| `errors` | Vec<String> | Output only. Provides details on the error that led to the image import state in case of an error. |
| `machine_image_target_details` | String | Output only. Target details used to import a machine image. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create image_import_job
image_import_job = provider.vmmigration_api.Image_import_job {
    name = "value"  # Required. The image import job id.
}

# Access image_import_job outputs
image_import_job_id = image_import_job.id
image_import_job_state = image_import_job.state
image_import_job_disk_image_target_details = image_import_job.disk_image_target_details
image_import_job_end_time = image_import_job.end_time
image_import_job_name = image_import_job.name
image_import_job_created_resources = image_import_job.created_resources
image_import_job_steps = image_import_job.steps
image_import_job_warnings = image_import_job.warnings
image_import_job_cloud_storage_uri = image_import_job.cloud_storage_uri
image_import_job_create_time = image_import_job.create_time
image_import_job_errors = image_import_job.errors
image_import_job_machine_image_target_details = image_import_job.machine_image_target_details
```

---


### Source

Creates a new Source in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The create time timestamp. |
| `name` | String |  | Output only. The Source name. |
| `encryption` | String |  | Optional. Immutable. The encryption details of the source data stored by the service. |
| `description` | String |  | User-provided description of the source. |
| `azure` | String |  | Azure type source details. |
| `vmware` | String |  | Vmware type source details. |
| `update_time` | String |  | Output only. The update time timestamp. |
| `error` | String |  | Output only. Provides details on the state of the Source in case of an error. |
| `aws` | String |  | AWS type source details. |
| `labels` | HashMap<String, String> |  | The labels of the source. |
| `parent` | String | ✅ | Required. The Source's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The create time timestamp. |
| `name` | String | Output only. The Source name. |
| `encryption` | String | Optional. Immutable. The encryption details of the source data stored by the service. |
| `description` | String | User-provided description of the source. |
| `azure` | String | Azure type source details. |
| `vmware` | String | Vmware type source details. |
| `update_time` | String | Output only. The update time timestamp. |
| `error` | String | Output only. Provides details on the state of the Source in case of an error. |
| `aws` | String | AWS type source details. |
| `labels` | HashMap<String, String> | The labels of the source. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create source
source = provider.vmmigration_api.Source {
    parent = "value"  # Required. The Source's parent.
}

# Access source outputs
source_id = source.id
source_create_time = source.create_time
source_name = source.name
source_encryption = source.encryption
source_description = source.description
source_azure = source.azure
source_vmware = source.vmware
source_update_time = source.update_time
source_error = source.error
source_aws = source.aws
source_labels = source.labels
```

---


### Utilization_report

Creates a new UtilizationReport.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state_time` | String |  | Output only. The time the state was last set. |
| `name` | String |  | Output only. The report unique name. |
| `frame_end_time` | String |  | Output only. The point in time when the time frame ends. Notice that the time frame is counted backwards. For instance if the "frame_end_time" value is 2021/01/20 and the time frame is WEEK then the report covers the week between 2021/01/20 and 2021/01/14. |
| `create_time` | String |  | Output only. The time the report was created (this refers to the time of the request, not the time the report creation completed). |
| `error` | String |  | Output only. Provides details on the state of the report in case of an error. |
| `display_name` | String |  | The report display name, as assigned by the user. |
| `time_frame` | String |  | Time frame of the report. |
| `vm_count` | i64 |  | Output only. Total number of VMs included in the report. |
| `vms` | Vec<String> |  | List of utilization information per VM. When sent as part of the request, the "vm_id" field is used in order to specify which VMs to include in the report. In that case all other fields are ignored. |
| `vms_count` | i64 |  | Output only. Total number of VMs included in the report. |
| `state` | String |  | Output only. Current state of the report. |
| `parent` | String | ✅ | Required. The Utilization Report's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state_time` | String | Output only. The time the state was last set. |
| `name` | String | Output only. The report unique name. |
| `frame_end_time` | String | Output only. The point in time when the time frame ends. Notice that the time frame is counted backwards. For instance if the "frame_end_time" value is 2021/01/20 and the time frame is WEEK then the report covers the week between 2021/01/20 and 2021/01/14. |
| `create_time` | String | Output only. The time the report was created (this refers to the time of the request, not the time the report creation completed). |
| `error` | String | Output only. Provides details on the state of the report in case of an error. |
| `display_name` | String | The report display name, as assigned by the user. |
| `time_frame` | String | Time frame of the report. |
| `vm_count` | i64 | Output only. Total number of VMs included in the report. |
| `vms` | Vec<String> | List of utilization information per VM. When sent as part of the request, the "vm_id" field is used in order to specify which VMs to include in the report. In that case all other fields are ignored. |
| `vms_count` | i64 | Output only. Total number of VMs included in the report. |
| `state` | String | Output only. Current state of the report. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create utilization_report
utilization_report = provider.vmmigration_api.Utilization_report {
    parent = "value"  # Required. The Utilization Report's parent.
}

# Access utilization_report outputs
utilization_report_id = utilization_report.id
utilization_report_state_time = utilization_report.state_time
utilization_report_name = utilization_report.name
utilization_report_frame_end_time = utilization_report.frame_end_time
utilization_report_create_time = utilization_report.create_time
utilization_report_error = utilization_report.error
utilization_report_display_name = utilization_report.display_name
utilization_report_time_frame = utilization_report.time_frame
utilization_report_vm_count = utilization_report.vm_count
utilization_report_vms = utilization_report.vms
utilization_report_vms_count = utilization_report.vms_count
utilization_report_state = utilization_report.state
```

---


### Disk_migration_job

Creates a new disk migration job in a given Source.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The last time the DiskMigrationJob resource was updated. |
| `aws_source_disk_details` | String |  | Details of the unattached AWS source disk. |
| `target_details` | String |  | Required. Details of the target Disk in Compute Engine. |
| `errors` | Vec<String> |  | Output only. Provides details on the errors that led to the disk migration job's state in case of an error. |
| `create_time` | String |  | Output only. The time the DiskMigrationJob resource was created. |
| `state` | String |  | Output only. State of the DiskMigrationJob. |
| `steps` | Vec<String> |  | Output only. The disk migration steps list representing its progress. |
| `name` | String |  | Output only. Identifier. The identifier of the DiskMigrationJob. |
| `parent` | String | ✅ | Required. The DiskMigrationJob's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The last time the DiskMigrationJob resource was updated. |
| `aws_source_disk_details` | String | Details of the unattached AWS source disk. |
| `target_details` | String | Required. Details of the target Disk in Compute Engine. |
| `errors` | Vec<String> | Output only. Provides details on the errors that led to the disk migration job's state in case of an error. |
| `create_time` | String | Output only. The time the DiskMigrationJob resource was created. |
| `state` | String | Output only. State of the DiskMigrationJob. |
| `steps` | Vec<String> | Output only. The disk migration steps list representing its progress. |
| `name` | String | Output only. Identifier. The identifier of the DiskMigrationJob. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create disk_migration_job
disk_migration_job = provider.vmmigration_api.Disk_migration_job {
    parent = "value"  # Required. The DiskMigrationJob's parent.
}

# Access disk_migration_job outputs
disk_migration_job_id = disk_migration_job.id
disk_migration_job_update_time = disk_migration_job.update_time
disk_migration_job_aws_source_disk_details = disk_migration_job.aws_source_disk_details
disk_migration_job_target_details = disk_migration_job.target_details
disk_migration_job_errors = disk_migration_job.errors
disk_migration_job_create_time = disk_migration_job.create_time
disk_migration_job_state = disk_migration_job.state
disk_migration_job_steps = disk_migration_job.steps
disk_migration_job_name = disk_migration_job.name
```

---


### Target_project

Creates a new TargetProject in a given project. NOTE: TargetProject is a global resource; hence the only supported value for location is `global`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The last time the target project resource was updated. |
| `description` | String |  | The target project's description. |
| `name` | String |  | Output only. The name of the target project. |
| `create_time` | String |  | Output only. The time this target project resource was created (not related to when the Compute Engine project it points to was created). |
| `project` | String |  | Required. The target project ID (number) or project name. |
| `parent` | String | ✅ | Required. The TargetProject's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The last time the target project resource was updated. |
| `description` | String | The target project's description. |
| `name` | String | Output only. The name of the target project. |
| `create_time` | String | Output only. The time this target project resource was created (not related to when the Compute Engine project it points to was created). |
| `project` | String | Required. The target project ID (number) or project name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create target_project
target_project = provider.vmmigration_api.Target_project {
    parent = "value"  # Required. The TargetProject's parent.
}

# Access target_project outputs
target_project_id = target_project.id
target_project_update_time = target_project.update_time
target_project_description = target_project.description
target_project_name = target_project.name
target_project_create_time = target_project.create_time
target_project_project = target_project.project
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.vmmigration_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
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
location_location_id = location.location_id
location_metadata = location.metadata
location_name = location.name
location_display_name = location.display_name
```

---


### Cutover_job

Initiates a Cutover of a specific migrating VM. The returned LRO is completed when the cutover job resource is created and the job is initiated.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `compute_engine_target_details` | String |  | Output only. Details of the target VM in Compute Engine. |
| `create_time` | String |  | Output only. The time the cutover job was created (as an API call, not when it was actually created in the target). |
| `progress` | i64 |  | Output only. The current progress in percentage of the cutover job. |
| `state_time` | String |  | Output only. The time the state was last updated. |
| `state_message` | String |  | Output only. A message providing possible extra details about the current state. |
| `end_time` | String |  | Output only. The time the cutover job had finished. |
| `name` | String |  | Output only. The name of the cutover job. |
| `progress_percent` | i64 |  | Output only. The current progress in percentage of the cutover job. |
| `steps` | Vec<String> |  | Output only. The cutover steps list representing its progress. |
| `target_details` | String |  | Output only. Details of the VM to create as the target of this cutover job. Deprecated: Use compute_engine_target_details instead. |
| `error` | String |  | Output only. Provides details for the errors that led to the Cutover Job's state. |
| `state` | String |  | Output only. State of the cutover job. |
| `compute_engine_disks_target_details` | String |  | Output only. Details of the target Persistent Disks in Compute Engine. |
| `compute_engine_vm_details` | String |  | Output only. Details of the VM in Compute Engine. Deprecated: Use compute_engine_target_details instead. |
| `parent` | String | ✅ | Required. The Cutover's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `compute_engine_target_details` | String | Output only. Details of the target VM in Compute Engine. |
| `create_time` | String | Output only. The time the cutover job was created (as an API call, not when it was actually created in the target). |
| `progress` | i64 | Output only. The current progress in percentage of the cutover job. |
| `state_time` | String | Output only. The time the state was last updated. |
| `state_message` | String | Output only. A message providing possible extra details about the current state. |
| `end_time` | String | Output only. The time the cutover job had finished. |
| `name` | String | Output only. The name of the cutover job. |
| `progress_percent` | i64 | Output only. The current progress in percentage of the cutover job. |
| `steps` | Vec<String> | Output only. The cutover steps list representing its progress. |
| `target_details` | String | Output only. Details of the VM to create as the target of this cutover job. Deprecated: Use compute_engine_target_details instead. |
| `error` | String | Output only. Provides details for the errors that led to the Cutover Job's state. |
| `state` | String | Output only. State of the cutover job. |
| `compute_engine_disks_target_details` | String | Output only. Details of the target Persistent Disks in Compute Engine. |
| `compute_engine_vm_details` | String | Output only. Details of the VM in Compute Engine. Deprecated: Use compute_engine_target_details instead. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cutover_job
cutover_job = provider.vmmigration_api.Cutover_job {
    parent = "value"  # Required. The Cutover's parent.
}

# Access cutover_job outputs
cutover_job_id = cutover_job.id
cutover_job_compute_engine_target_details = cutover_job.compute_engine_target_details
cutover_job_create_time = cutover_job.create_time
cutover_job_progress = cutover_job.progress
cutover_job_state_time = cutover_job.state_time
cutover_job_state_message = cutover_job.state_message
cutover_job_end_time = cutover_job.end_time
cutover_job_name = cutover_job.name
cutover_job_progress_percent = cutover_job.progress_percent
cutover_job_steps = cutover_job.steps
cutover_job_target_details = cutover_job.target_details
cutover_job_error = cutover_job.error
cutover_job_state = cutover_job.state
cutover_job_compute_engine_disks_target_details = cutover_job.compute_engine_disks_target_details
cutover_job_compute_engine_vm_details = cutover_job.compute_engine_vm_details
```

---


### Image_import

Creates a new ImageImport in a given project.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `machine_image_target_defaults` | String |  | Immutable. Target details for importing a machine image, will be used by ImageImportJob. |
| `create_time` | String |  | Output only. The time the image import was created. |
| `cloud_storage_uri` | String |  | Immutable. The path to the Cloud Storage file from which the image should be imported. |
| `disk_image_target_defaults` | String |  | Immutable. Target details for importing a disk image, will be used by ImageImportJob. |
| `encryption` | String |  | Immutable. The encryption details used by the image import process during the image adaptation for Compute Engine. |
| `name` | String |  | Output only. The resource path of the ImageImport. |
| `recent_image_import_jobs` | Vec<String> |  | Output only. The result of the most recent runs for this ImageImport. All jobs for this ImageImport can be listed via ListImageImportJobs. |
| `parent` | String | ✅ | Required. The ImageImport's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `machine_image_target_defaults` | String | Immutable. Target details for importing a machine image, will be used by ImageImportJob. |
| `create_time` | String | Output only. The time the image import was created. |
| `cloud_storage_uri` | String | Immutable. The path to the Cloud Storage file from which the image should be imported. |
| `disk_image_target_defaults` | String | Immutable. Target details for importing a disk image, will be used by ImageImportJob. |
| `encryption` | String | Immutable. The encryption details used by the image import process during the image adaptation for Compute Engine. |
| `name` | String | Output only. The resource path of the ImageImport. |
| `recent_image_import_jobs` | Vec<String> | Output only. The result of the most recent runs for this ImageImport. All jobs for this ImageImport can be listed via ListImageImportJobs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create image_import
image_import = provider.vmmigration_api.Image_import {
    parent = "value"  # Required. The ImageImport's parent.
}

# Access image_import outputs
image_import_id = image_import.id
image_import_machine_image_target_defaults = image_import.machine_image_target_defaults
image_import_create_time = image_import.create_time
image_import_cloud_storage_uri = image_import.cloud_storage_uri
image_import_disk_image_target_defaults = image_import.disk_image_target_defaults
image_import_encryption = image_import.encryption
image_import_name = image_import.name
image_import_recent_image_import_jobs = image_import.recent_image_import_jobs
```

---


### Group

Creates a new Group in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | User-provided description of the group. |
| `display_name` | String |  | Display name is a user defined name for this group which can be updated. |
| `name` | String |  | Output only. The Group name. |
| `migration_target_type` | String |  | Immutable. The target type of this group. |
| `update_time` | String |  | Output only. The update time timestamp. |
| `create_time` | String |  | Output only. The create time timestamp. |
| `parent` | String | ✅ | Required. The Group's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | User-provided description of the group. |
| `display_name` | String | Display name is a user defined name for this group which can be updated. |
| `name` | String | Output only. The Group name. |
| `migration_target_type` | String | Immutable. The target type of this group. |
| `update_time` | String | Output only. The update time timestamp. |
| `create_time` | String | Output only. The create time timestamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create group
group = provider.vmmigration_api.Group {
    parent = "value"  # Required. The Group's parent.
}

# Access group outputs
group_id = group.id
group_description = group.description
group_display_name = group.display_name
group_name = group.name
group_migration_target_type = group.migration_target_type
group_update_time = group.update_time
group_create_time = group.create_time
```

---


### Clone_job

Initiates a Clone of a specific migrating VM.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time the clone job was created (as an API call, not when it was actually created in the target). |
| `compute_engine_disks_target_details` | String |  | Output only. Details of the target Persistent Disks in Compute Engine. |
| `end_time` | String |  | Output only. The time the clone job was ended. |
| `state_time` | String |  | Output only. The time the state was last updated. |
| `target_details` | String |  | Output only. Details of the VM to create as the target of this clone job. Deprecated: Use compute_engine_target_details instead. |
| `steps` | Vec<String> |  | Output only. The clone steps list representing its progress. |
| `compute_engine_vm_details` | String |  | Output only. Details of the VM in Compute Engine. Deprecated: Use compute_engine_target_details instead. |
| `name` | String |  | Output only. The name of the clone. |
| `state` | String |  | Output only. State of the clone job. |
| `compute_engine_target_details` | String |  | Output only. Details of the target VM in Compute Engine. |
| `error` | String |  | Output only. Provides details for the errors that led to the Clone Job's state. |
| `parent` | String | ✅ | Required. The Clone's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time the clone job was created (as an API call, not when it was actually created in the target). |
| `compute_engine_disks_target_details` | String | Output only. Details of the target Persistent Disks in Compute Engine. |
| `end_time` | String | Output only. The time the clone job was ended. |
| `state_time` | String | Output only. The time the state was last updated. |
| `target_details` | String | Output only. Details of the VM to create as the target of this clone job. Deprecated: Use compute_engine_target_details instead. |
| `steps` | Vec<String> | Output only. The clone steps list representing its progress. |
| `compute_engine_vm_details` | String | Output only. Details of the VM in Compute Engine. Deprecated: Use compute_engine_target_details instead. |
| `name` | String | Output only. The name of the clone. |
| `state` | String | Output only. State of the clone job. |
| `compute_engine_target_details` | String | Output only. Details of the target VM in Compute Engine. |
| `error` | String | Output only. Provides details for the errors that led to the Clone Job's state. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create clone_job
clone_job = provider.vmmigration_api.Clone_job {
    parent = "value"  # Required. The Clone's parent.
}

# Access clone_job outputs
clone_job_id = clone_job.id
clone_job_create_time = clone_job.create_time
clone_job_compute_engine_disks_target_details = clone_job.compute_engine_disks_target_details
clone_job_end_time = clone_job.end_time
clone_job_state_time = clone_job.state_time
clone_job_target_details = clone_job.target_details
clone_job_steps = clone_job.steps
clone_job_compute_engine_vm_details = clone_job.compute_engine_vm_details
clone_job_name = clone_job.name
clone_job_state = clone_job.state
clone_job_compute_engine_target_details = clone_job.compute_engine_target_details
clone_job_error = clone_job.error
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple image_import resources
image_import_0 = provider.vmmigration_api.Image_import {
    parent = "value-0"
}
image_import_1 = provider.vmmigration_api.Image_import {
    parent = "value-1"
}
image_import_2 = provider.vmmigration_api.Image_import {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    image_import = provider.vmmigration_api.Image_import {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Vmmigration_api Documentation](https://cloud.google.com/vmmigration_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
