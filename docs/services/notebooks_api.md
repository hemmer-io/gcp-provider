# Notebooks_api Service



**Resources**: 10

---

## Overview

The notebooks_api service provides access to 10 resource types:

- [Runtime](#runtime) [CRUD]
- [Location](#location) [R]
- [Instance](#instance) [CRUD]
- [Operation](#operation) [CRD]
- [Execution](#execution) [CRD]
- [Environment](#environment) [CRD]
- [Schedule](#schedule) [CRD]
- [Location](#location) [R]
- [Instance](#instance) [CRUD]
- [Operation](#operation) [CRD]

---

## Resources


### Runtime

Creates a new Runtime in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. The labels to associate with this Managed Notebook or Runtime. Label **keys** must contain 1 to 63 characters, and must conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be empty, but, if present, must contain 1 to 63 characters, and must conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a cluster. |
| `access_config` | String |  | The config settings for accessing runtime. |
| `health_state` | String |  | Output only. Runtime health_state. |
| `virtual_machine` | String |  | Use a Compute Engine VM image to start the managed notebook instance. |
| `migrated` | bool |  | Output only. Bool indicating whether this notebook has been migrated to a Workbench Instance |
| `metrics` | String |  | Output only. Contains Runtime daemon metrics such as Service status and JupyterLab stats. |
| `name` | String |  | Output only. The resource name of the runtime. Format: `projects/{project}/locations/{location}/runtimes/{runtimeId}` |
| `runtime_migration_eligibility` | String |  | Output only. Checks how feasible a migration from GmN to WbI is. |
| `software_config` | String |  | The config settings for software inside the runtime. |
| `state` | String |  | Output only. Runtime state. |
| `create_time` | String |  | Output only. Runtime creation time. |
| `update_time` | String |  | Output only. Runtime update time. |
| `parent` | String | ✅ | Required. Format: `parent=projects/{project_id}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. The labels to associate with this Managed Notebook or Runtime. Label **keys** must contain 1 to 63 characters, and must conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be empty, but, if present, must contain 1 to 63 characters, and must conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a cluster. |
| `access_config` | String | The config settings for accessing runtime. |
| `health_state` | String | Output only. Runtime health_state. |
| `virtual_machine` | String | Use a Compute Engine VM image to start the managed notebook instance. |
| `migrated` | bool | Output only. Bool indicating whether this notebook has been migrated to a Workbench Instance |
| `metrics` | String | Output only. Contains Runtime daemon metrics such as Service status and JupyterLab stats. |
| `name` | String | Output only. The resource name of the runtime. Format: `projects/{project}/locations/{location}/runtimes/{runtimeId}` |
| `runtime_migration_eligibility` | String | Output only. Checks how feasible a migration from GmN to WbI is. |
| `software_config` | String | The config settings for software inside the runtime. |
| `state` | String | Output only. Runtime state. |
| `create_time` | String | Output only. Runtime creation time. |
| `update_time` | String | Output only. Runtime update time. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create runtime
runtime = provider.notebooks_api.Runtime {
    parent = "value"  # Required. Format: `parent=projects/{project_id}/locations/{location}`
}

# Access runtime outputs
runtime_id = runtime.id
runtime_labels = runtime.labels
runtime_access_config = runtime.access_config
runtime_health_state = runtime.health_state
runtime_virtual_machine = runtime.virtual_machine
runtime_migrated = runtime.migrated
runtime_metrics = runtime.metrics
runtime_name = runtime.name
runtime_runtime_migration_eligibility = runtime.runtime_migration_eligibility
runtime_software_config = runtime.software_config
runtime_state = runtime.state
runtime_create_time = runtime.create_time
runtime_update_time = runtime.update_time
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_location_id = location.location_id
location_name = location.name
location_metadata = location.metadata
location_labels = location.labels
location_display_name = location.display_name
```

---


### Instance

Creates a new Instance in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_gpu_driver_path` | String |  | Specify a custom Cloud Storage path where the GPU driver is stored. If not specified, we'll automatically choose from official GPU drivers. |
| `nic_type` | String |  | Optional. The type of vNIC to be used on this interface. This may be gVNIC or VirtioNet. |
| `name` | String |  | Output only. The name of this notebook instance. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}` |
| `instance_migration_eligibility` | String |  | Output only. Checks how feasible a migration from UmN to WbI is. |
| `data_disk_size_gb` | String |  | Input only. The size of the data disk in GB attached to this instance, up to a maximum of 64000 GB (64 TB). You can choose the size of the data disk based on how big your notebooks and data are. If not specified, this defaults to 100. |
| `no_proxy_access` | bool |  | If true, the notebook instance will not register with the proxy. |
| `creator` | String |  | Output only. Email address of entity that sent original CreateInstance request. |
| `boot_disk_size_gb` | String |  | Input only. The size of the boot disk in GB attached to this instance, up to a maximum of 64000 GB (64 TB). The minimum recommended value is 100 GB. If not specified, this defaults to 100. |
| `network` | String |  | The name of the VPC that this instance is in. Format: `projects/{project_id}/global/networks/{network_id}` |
| `disks` | Vec<String> |  | Output only. Attached disks to notebook instance. |
| `container_image` | String |  | Use a container image to start the notebook instance. |
| `boot_disk_type` | String |  | Input only. The type of the boot disk attached to this instance, defaults to standard persistent disk (`PD_STANDARD`). |
| `can_ip_forward` | bool |  | Optional. Flag to enable ip forwarding or not, default false/off. https://cloud.google.com/vpc/docs/using-routes#canipforward |
| `state` | String |  | Output only. The state of this instance. |
| `create_time` | String |  | Output only. Instance creation time. |
| `upgrade_history` | Vec<String> |  | The upgrade history of this instance. |
| `data_disk_type` | String |  | Input only. The type of the data disk attached to this instance, defaults to standard persistent disk (`PD_STANDARD`). |
| `no_remove_data_disk` | bool |  | Input only. If true, the data disk will not be auto deleted when deleting the instance. |
| `service_account_scopes` | Vec<String> |  | Optional. The URIs of service account scopes to be included in Compute Engine instances. If not specified, the following [scopes](https://cloud.google.com/compute/docs/access/service-accounts#accesscopesiam) are defined: - https://www.googleapis.com/auth/cloud-platform - https://www.googleapis.com/auth/userinfo.email If not using default scopes, you need at least: https://www.googleapis.com/auth/compute |
| `install_gpu_driver` | bool |  | Whether the end user authorizes Google Cloud to install GPU driver on this instance. If this field is empty or set to false, the GPU driver won't be installed. Only applicable to instances with GPUs. |
| `accelerator_config` | String |  | The hardware accelerator used on this instance. If you use accelerators, make sure that your configuration has [enough vCPUs and memory to support the `machine_type` you have selected](https://cloud.google.com/compute/docs/gpus/#gpus-list). |
| `labels` | HashMap<String, String> |  | Labels to apply to this instance. These can be later modified by the setLabels method. |
| `metadata` | HashMap<String, String> |  | Custom metadata to apply to this instance. For example, to specify a Cloud Storage bucket for automatic backup, you can use the `gcs-data-bucket` metadata tag. Format: `"--metadata=gcs-data-bucket=BUCKET"`. |
| `post_startup_script` | String |  | Path to a Bash script that automatically runs after a notebook instance fully boots up. The path must be a URL or Cloud Storage path (`gs://path-to-file/file-name`). |
| `kms_key` | String |  | Input only. The KMS key used to encrypt the disks, only applicable if disk_encryption is CMEK. Format: `projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}` Learn more about [using your own encryption keys](/kms/docs/quickstart). |
| `shielded_instance_config` | String |  | Optional. Shielded VM configuration. [Images using supported Shielded VM features](https://cloud.google.com/compute/docs/instances/modifying-shielded-vm). |
| `disk_encryption` | String |  | Input only. Disk encryption method used on the boot and data disks, defaults to GMEK. |
| `tags` | Vec<String> |  | Optional. The Compute Engine network tags to add to runtime (see [Add network tags](https://cloud.google.com/vpc/docs/add-remove-network-tags)). |
| `update_time` | String |  | Output only. Instance update time. |
| `vm_image` | String |  | Use a Compute Engine VM image to start the notebook instance. |
| `service_account` | String |  | The service account on this instance, giving access to other Google Cloud services. You can use any service account within the same project, but you must have the service account user permission to use the instance. If not specified, the [Compute Engine default service account](https://cloud.google.com/compute/docs/access/service-accounts#default_service_account) is used. |
| `reservation_affinity` | String |  | Optional. The optional reservation affinity. Setting this field will apply the specified [Zonal Compute Reservation](https://cloud.google.com/compute/docs/instances/reserving-zonal-resources) to this notebook instance. |
| `subnet` | String |  | The name of the subnet that this instance is in. Format: `projects/{project_id}/regions/{region}/subnetworks/{subnetwork_id}` |
| `proxy_uri` | String |  | Output only. The proxy endpoint that is used to access the Jupyter notebook. |
| `migrated` | bool |  | Output only. Bool indicating whether this notebook has been migrated to a Workbench Instance |
| `no_public_ip` | bool |  | If true, no external IP will be assigned to this instance. |
| `instance_owners` | Vec<String> |  | Input only. The owner of this instance after creation. Format: `alias@example.com` Currently supports one owner only. If not specified, all of the service account users of your VM instance's service account can use the instance. |
| `machine_type` | String |  | Required. The [Compute Engine machine type](https://cloud.google.com/compute/docs/machine-resource) of this instance. |
| `parent` | String | ✅ | Required. Format: `parent=projects/{project_id}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `custom_gpu_driver_path` | String | Specify a custom Cloud Storage path where the GPU driver is stored. If not specified, we'll automatically choose from official GPU drivers. |
| `nic_type` | String | Optional. The type of vNIC to be used on this interface. This may be gVNIC or VirtioNet. |
| `name` | String | Output only. The name of this notebook instance. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}` |
| `instance_migration_eligibility` | String | Output only. Checks how feasible a migration from UmN to WbI is. |
| `data_disk_size_gb` | String | Input only. The size of the data disk in GB attached to this instance, up to a maximum of 64000 GB (64 TB). You can choose the size of the data disk based on how big your notebooks and data are. If not specified, this defaults to 100. |
| `no_proxy_access` | bool | If true, the notebook instance will not register with the proxy. |
| `creator` | String | Output only. Email address of entity that sent original CreateInstance request. |
| `boot_disk_size_gb` | String | Input only. The size of the boot disk in GB attached to this instance, up to a maximum of 64000 GB (64 TB). The minimum recommended value is 100 GB. If not specified, this defaults to 100. |
| `network` | String | The name of the VPC that this instance is in. Format: `projects/{project_id}/global/networks/{network_id}` |
| `disks` | Vec<String> | Output only. Attached disks to notebook instance. |
| `container_image` | String | Use a container image to start the notebook instance. |
| `boot_disk_type` | String | Input only. The type of the boot disk attached to this instance, defaults to standard persistent disk (`PD_STANDARD`). |
| `can_ip_forward` | bool | Optional. Flag to enable ip forwarding or not, default false/off. https://cloud.google.com/vpc/docs/using-routes#canipforward |
| `state` | String | Output only. The state of this instance. |
| `create_time` | String | Output only. Instance creation time. |
| `upgrade_history` | Vec<String> | The upgrade history of this instance. |
| `data_disk_type` | String | Input only. The type of the data disk attached to this instance, defaults to standard persistent disk (`PD_STANDARD`). |
| `no_remove_data_disk` | bool | Input only. If true, the data disk will not be auto deleted when deleting the instance. |
| `service_account_scopes` | Vec<String> | Optional. The URIs of service account scopes to be included in Compute Engine instances. If not specified, the following [scopes](https://cloud.google.com/compute/docs/access/service-accounts#accesscopesiam) are defined: - https://www.googleapis.com/auth/cloud-platform - https://www.googleapis.com/auth/userinfo.email If not using default scopes, you need at least: https://www.googleapis.com/auth/compute |
| `install_gpu_driver` | bool | Whether the end user authorizes Google Cloud to install GPU driver on this instance. If this field is empty or set to false, the GPU driver won't be installed. Only applicable to instances with GPUs. |
| `accelerator_config` | String | The hardware accelerator used on this instance. If you use accelerators, make sure that your configuration has [enough vCPUs and memory to support the `machine_type` you have selected](https://cloud.google.com/compute/docs/gpus/#gpus-list). |
| `labels` | HashMap<String, String> | Labels to apply to this instance. These can be later modified by the setLabels method. |
| `metadata` | HashMap<String, String> | Custom metadata to apply to this instance. For example, to specify a Cloud Storage bucket for automatic backup, you can use the `gcs-data-bucket` metadata tag. Format: `"--metadata=gcs-data-bucket=BUCKET"`. |
| `post_startup_script` | String | Path to a Bash script that automatically runs after a notebook instance fully boots up. The path must be a URL or Cloud Storage path (`gs://path-to-file/file-name`). |
| `kms_key` | String | Input only. The KMS key used to encrypt the disks, only applicable if disk_encryption is CMEK. Format: `projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}` Learn more about [using your own encryption keys](/kms/docs/quickstart). |
| `shielded_instance_config` | String | Optional. Shielded VM configuration. [Images using supported Shielded VM features](https://cloud.google.com/compute/docs/instances/modifying-shielded-vm). |
| `disk_encryption` | String | Input only. Disk encryption method used on the boot and data disks, defaults to GMEK. |
| `tags` | Vec<String> | Optional. The Compute Engine network tags to add to runtime (see [Add network tags](https://cloud.google.com/vpc/docs/add-remove-network-tags)). |
| `update_time` | String | Output only. Instance update time. |
| `vm_image` | String | Use a Compute Engine VM image to start the notebook instance. |
| `service_account` | String | The service account on this instance, giving access to other Google Cloud services. You can use any service account within the same project, but you must have the service account user permission to use the instance. If not specified, the [Compute Engine default service account](https://cloud.google.com/compute/docs/access/service-accounts#default_service_account) is used. |
| `reservation_affinity` | String | Optional. The optional reservation affinity. Setting this field will apply the specified [Zonal Compute Reservation](https://cloud.google.com/compute/docs/instances/reserving-zonal-resources) to this notebook instance. |
| `subnet` | String | The name of the subnet that this instance is in. Format: `projects/{project_id}/regions/{region}/subnetworks/{subnetwork_id}` |
| `proxy_uri` | String | Output only. The proxy endpoint that is used to access the Jupyter notebook. |
| `migrated` | bool | Output only. Bool indicating whether this notebook has been migrated to a Workbench Instance |
| `no_public_ip` | bool | If true, no external IP will be assigned to this instance. |
| `instance_owners` | Vec<String> | Input only. The owner of this instance after creation. Format: `alias@example.com` Currently supports one owner only. If not specified, all of the service account users of your VM instance's service account can use the instance. |
| `machine_type` | String | Required. The [Compute Engine machine type](https://cloud.google.com/compute/docs/machine-resource) of this instance. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.notebooks_api.Instance {
    parent = "value"  # Required. Format: `parent=projects/{project_id}/locations/{location}`
}

# Access instance outputs
instance_id = instance.id
instance_custom_gpu_driver_path = instance.custom_gpu_driver_path
instance_nic_type = instance.nic_type
instance_name = instance.name
instance_instance_migration_eligibility = instance.instance_migration_eligibility
instance_data_disk_size_gb = instance.data_disk_size_gb
instance_no_proxy_access = instance.no_proxy_access
instance_creator = instance.creator
instance_boot_disk_size_gb = instance.boot_disk_size_gb
instance_network = instance.network
instance_disks = instance.disks
instance_container_image = instance.container_image
instance_boot_disk_type = instance.boot_disk_type
instance_can_ip_forward = instance.can_ip_forward
instance_state = instance.state
instance_create_time = instance.create_time
instance_upgrade_history = instance.upgrade_history
instance_data_disk_type = instance.data_disk_type
instance_no_remove_data_disk = instance.no_remove_data_disk
instance_service_account_scopes = instance.service_account_scopes
instance_install_gpu_driver = instance.install_gpu_driver
instance_accelerator_config = instance.accelerator_config
instance_labels = instance.labels
instance_metadata = instance.metadata
instance_post_startup_script = instance.post_startup_script
instance_kms_key = instance.kms_key
instance_shielded_instance_config = instance.shielded_instance_config
instance_disk_encryption = instance.disk_encryption
instance_tags = instance.tags
instance_update_time = instance.update_time
instance_vm_image = instance.vm_image
instance_service_account = instance.service_account
instance_reservation_affinity = instance.reservation_affinity
instance_subnet = instance.subnet
instance_proxy_uri = instance.proxy_uri
instance_migrated = instance.migrated
instance_no_public_ip = instance.no_public_ip
instance_instance_owners = instance.instance_owners
instance_machine_type = instance.machine_type
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.notebooks_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
operation_name = operation.name
operation_metadata = operation.metadata
```

---


### Execution

Creates a new Execution in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time the Execution was instantiated. |
| `description` | String |  | A brief description of this execution. |
| `name` | String |  | Output only. The resource name of the execute. Format: `projects/{project_id}/locations/{location}/executions/{execution_id}` |
| `execution_template` | String |  | execute metadata including name, hardware spec, region, labels, etc. |
| `state` | String |  | Output only. State of the underlying AI Platform job. |
| `output_notebook_file` | String |  | Output notebook file generated by this execution |
| `update_time` | String |  | Output only. Time the Execution was last updated. |
| `display_name` | String |  | Output only. Name used for UI purposes. Name can only contain alphanumeric characters and underscores '_'. |
| `job_uri` | String |  | Output only. The URI of the external job used to execute the notebook. |
| `parent` | String | ✅ | Required. Format: `parent=projects/{project_id}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time the Execution was instantiated. |
| `description` | String | A brief description of this execution. |
| `name` | String | Output only. The resource name of the execute. Format: `projects/{project_id}/locations/{location}/executions/{execution_id}` |
| `execution_template` | String | execute metadata including name, hardware spec, region, labels, etc. |
| `state` | String | Output only. State of the underlying AI Platform job. |
| `output_notebook_file` | String | Output notebook file generated by this execution |
| `update_time` | String | Output only. Time the Execution was last updated. |
| `display_name` | String | Output only. Name used for UI purposes. Name can only contain alphanumeric characters and underscores '_'. |
| `job_uri` | String | Output only. The URI of the external job used to execute the notebook. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create execution
execution = provider.notebooks_api.Execution {
    parent = "value"  # Required. Format: `parent=projects/{project_id}/locations/{location}`
}

# Access execution outputs
execution_id = execution.id
execution_create_time = execution.create_time
execution_description = execution.description
execution_name = execution.name
execution_execution_template = execution.execution_template
execution_state = execution.state
execution_output_notebook_file = execution.output_notebook_file
execution_update_time = execution.update_time
execution_display_name = execution.display_name
execution_job_uri = execution.job_uri
```

---


### Environment

Creates a new Environment.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Name of this environment. Format: `projects/{project_id}/locations/{location}/environments/{environment_id}` |
| `post_startup_script` | String |  | Path to a Bash script that automatically runs after a notebook instance fully boots up. The path must be a URL or Cloud Storage path. Example: `"gs://path-to-file/file-name"` |
| `vm_image` | String |  | Use a Compute Engine VM image to start the notebook instance. |
| `create_time` | String |  | Output only. The time at which this environment was created. |
| `display_name` | String |  | Display name of this environment for the UI. |
| `description` | String |  | A brief description of this environment. |
| `container_image` | String |  | Use a container image to start the notebook instance. |
| `parent` | String | ✅ | Required. Format: `projects/{project_id}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Name of this environment. Format: `projects/{project_id}/locations/{location}/environments/{environment_id}` |
| `post_startup_script` | String | Path to a Bash script that automatically runs after a notebook instance fully boots up. The path must be a URL or Cloud Storage path. Example: `"gs://path-to-file/file-name"` |
| `vm_image` | String | Use a Compute Engine VM image to start the notebook instance. |
| `create_time` | String | Output only. The time at which this environment was created. |
| `display_name` | String | Display name of this environment for the UI. |
| `description` | String | A brief description of this environment. |
| `container_image` | String | Use a container image to start the notebook instance. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create environment
environment = provider.notebooks_api.Environment {
    parent = "value"  # Required. Format: `projects/{project_id}/locations/{location}`
}

# Access environment outputs
environment_id = environment.id
environment_name = environment.name
environment_post_startup_script = environment.post_startup_script
environment_vm_image = environment.vm_image
environment_create_time = environment.create_time
environment_display_name = environment.display_name
environment_description = environment.description
environment_container_image = environment.container_image
```

---


### Schedule

Creates a new Scheduled Notebook in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `execution_template` | String |  | Notebook Execution Template corresponding to this schedule. |
| `create_time` | String |  | Output only. Time the schedule was created. |
| `cron_schedule` | String |  | Cron-tab formatted schedule by which the job will execute. Format: minute, hour, day of month, month, day of week, e.g. `0 0 * * WED` = every Wednesday More examples: https://crontab.guru/examples.html |
| `display_name` | String |  | Output only. Display name used for UI purposes. Name can only contain alphanumeric characters, hyphens `-`, and underscores `_`. |
| `recent_executions` | Vec<String> |  | Output only. The most recent execution names triggered from this schedule and their corresponding states. |
| `description` | String |  | A brief description of this environment. |
| `state` | String |  |  |
| `name` | String |  | Output only. The name of this schedule. Format: `projects/{project_id}/locations/{location}/schedules/{schedule_id}` |
| `time_zone` | String |  | Timezone on which the cron_schedule. The value of this field must be a time zone name from the tz database. TZ Database: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones Note that some time zones include a provision for daylight savings time. The rules for daylight saving time are determined by the chosen tz. For UTC use the string "utc". If a time zone is not specified, the default will be in UTC (also known as GMT). |
| `update_time` | String |  | Output only. Time the schedule was last updated. |
| `parent` | String | ✅ | Required. Format: `parent=projects/{project_id}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `execution_template` | String | Notebook Execution Template corresponding to this schedule. |
| `create_time` | String | Output only. Time the schedule was created. |
| `cron_schedule` | String | Cron-tab formatted schedule by which the job will execute. Format: minute, hour, day of month, month, day of week, e.g. `0 0 * * WED` = every Wednesday More examples: https://crontab.guru/examples.html |
| `display_name` | String | Output only. Display name used for UI purposes. Name can only contain alphanumeric characters, hyphens `-`, and underscores `_`. |
| `recent_executions` | Vec<String> | Output only. The most recent execution names triggered from this schedule and their corresponding states. |
| `description` | String | A brief description of this environment. |
| `state` | String |  |
| `name` | String | Output only. The name of this schedule. Format: `projects/{project_id}/locations/{location}/schedules/{schedule_id}` |
| `time_zone` | String | Timezone on which the cron_schedule. The value of this field must be a time zone name from the tz database. TZ Database: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones Note that some time zones include a provision for daylight savings time. The rules for daylight saving time are determined by the chosen tz. For UTC use the string "utc". If a time zone is not specified, the default will be in UTC (also known as GMT). |
| `update_time` | String | Output only. Time the schedule was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create schedule
schedule = provider.notebooks_api.Schedule {
    parent = "value"  # Required. Format: `parent=projects/{project_id}/locations/{location}`
}

# Access schedule outputs
schedule_id = schedule.id
schedule_execution_template = schedule.execution_template
schedule_create_time = schedule.create_time
schedule_cron_schedule = schedule.cron_schedule
schedule_display_name = schedule.display_name
schedule_recent_executions = schedule.recent_executions
schedule_description = schedule.description
schedule_state = schedule.state
schedule_name = schedule.name
schedule_time_zone = schedule.time_zone
schedule_update_time = schedule.update_time
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_labels = location.labels
location_metadata = location.metadata
location_location_id = location.location_id
location_display_name = location.display_name
location_name = location.name
```

---


### Instance

Creates a new Instance in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Instance update time. |
| `instance_owners` | Vec<String> |  | Optional. The owner of this instance after creation. Format: `alias@example.com` Currently supports one owner only. If not specified, all of the service account users of your VM instance's service account can use the instance. |
| `enable_third_party_identity` | bool |  | Optional. Flag that specifies that a notebook can be accessed with third party identity provider. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use for Zone Separation. |
| `id` | String |  | Output only. Unique ID of the resource. |
| `creator` | String |  | Output only. Email address of entity that sent original CreateInstance request. |
| `health_state` | String |  | Output only. Instance health_state. |
| `health_info` | HashMap<String, String> |  | Output only. Additional information about instance health. Example: healthInfo": { "docker_proxy_agent_status": "1", "docker_status": "1", "jupyterlab_api_status": "-1", "jupyterlab_status": "-1", "updated": "2020-10-18 09:40:03.573409" } |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use for Zone Isolation. |
| `proxy_uri` | String |  | Output only. The proxy endpoint that is used to access the Jupyter notebook. |
| `state` | String |  | Output only. The state of this instance. |
| `labels` | HashMap<String, String> |  | Optional. Labels to apply to this instance. These can be later modified by the UpdateInstance method. |
| `disable_proxy_access` | bool |  | Optional. If true, the notebook instance will not register with the proxy. |
| `gce_setup` | String |  | Optional. Compute Engine setup for the notebook. Uses notebook-defined fields. |
| `third_party_proxy_url` | String |  | Output only. The workforce pools proxy endpoint that is used to access the Jupyter notebook. |
| `name` | String |  | Output only. Identifier. The name of this notebook instance. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}` |
| `create_time` | String |  | Output only. Instance creation time. |
| `enable_managed_euc` | bool |  | Optional. Flag to enable managed end user credentials for the instance. |
| `enable_deletion_protection` | bool |  | Optional. If true, deletion protection will be enabled for this Workbench Instance. If false, deletion protection will be disabled for this Workbench Instance. |
| `upgrade_history` | Vec<String> |  | Output only. The upgrade history of this instance. |
| `parent` | String | ✅ | Required. Format: `parent=projects/{project_id}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Instance update time. |
| `instance_owners` | Vec<String> | Optional. The owner of this instance after creation. Format: `alias@example.com` Currently supports one owner only. If not specified, all of the service account users of your VM instance's service account can use the instance. |
| `enable_third_party_identity` | bool | Optional. Flag that specifies that a notebook can be accessed with third party identity provider. |
| `satisfies_pzs` | bool | Output only. Reserved for future use for Zone Separation. |
| `id` | String | Output only. Unique ID of the resource. |
| `creator` | String | Output only. Email address of entity that sent original CreateInstance request. |
| `health_state` | String | Output only. Instance health_state. |
| `health_info` | HashMap<String, String> | Output only. Additional information about instance health. Example: healthInfo": { "docker_proxy_agent_status": "1", "docker_status": "1", "jupyterlab_api_status": "-1", "jupyterlab_status": "-1", "updated": "2020-10-18 09:40:03.573409" } |
| `satisfies_pzi` | bool | Output only. Reserved for future use for Zone Isolation. |
| `proxy_uri` | String | Output only. The proxy endpoint that is used to access the Jupyter notebook. |
| `state` | String | Output only. The state of this instance. |
| `labels` | HashMap<String, String> | Optional. Labels to apply to this instance. These can be later modified by the UpdateInstance method. |
| `disable_proxy_access` | bool | Optional. If true, the notebook instance will not register with the proxy. |
| `gce_setup` | String | Optional. Compute Engine setup for the notebook. Uses notebook-defined fields. |
| `third_party_proxy_url` | String | Output only. The workforce pools proxy endpoint that is used to access the Jupyter notebook. |
| `name` | String | Output only. Identifier. The name of this notebook instance. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}` |
| `create_time` | String | Output only. Instance creation time. |
| `enable_managed_euc` | bool | Optional. Flag to enable managed end user credentials for the instance. |
| `enable_deletion_protection` | bool | Optional. If true, deletion protection will be enabled for this Workbench Instance. If false, deletion protection will be disabled for this Workbench Instance. |
| `upgrade_history` | Vec<String> | Output only. The upgrade history of this instance. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.notebooks_api.Instance {
    parent = "value"  # Required. Format: `parent=projects/{project_id}/locations/{location}`
}

# Access instance outputs
instance_id = instance.id
instance_update_time = instance.update_time
instance_instance_owners = instance.instance_owners
instance_enable_third_party_identity = instance.enable_third_party_identity
instance_satisfies_pzs = instance.satisfies_pzs
instance_id = instance.id
instance_creator = instance.creator
instance_health_state = instance.health_state
instance_health_info = instance.health_info
instance_satisfies_pzi = instance.satisfies_pzi
instance_proxy_uri = instance.proxy_uri
instance_state = instance.state
instance_labels = instance.labels
instance_disable_proxy_access = instance.disable_proxy_access
instance_gce_setup = instance.gce_setup
instance_third_party_proxy_url = instance.third_party_proxy_url
instance_name = instance.name
instance_create_time = instance.create_time
instance_enable_managed_euc = instance.enable_managed_euc
instance_enable_deletion_protection = instance.enable_deletion_protection
instance_upgrade_history = instance.upgrade_history
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.notebooks_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_done = operation.done
operation_error = operation.error
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

# Create multiple runtime resources
runtime_0 = provider.notebooks_api.Runtime {
    parent = "value-0"
}
runtime_1 = provider.notebooks_api.Runtime {
    parent = "value-1"
}
runtime_2 = provider.notebooks_api.Runtime {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    runtime = provider.notebooks_api.Runtime {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Notebooks_api Documentation](https://cloud.google.com/notebooks_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
