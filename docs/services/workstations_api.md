# Workstations_api Service



**Resources**: 9

---

## Overview

The workstations_api service provides access to 9 resource types:

- [Workstation_cluster](#workstation_cluster) [CRUD]
- [Workstation_config](#workstation_config) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CRD]
- [Workstation](#workstation) [CRUD]
- [Operation](#operation) [CRD]
- [Workstation_config](#workstation_config) [CRUD]
- [Workstation_cluster](#workstation_cluster) [CRUD]
- [Workstation](#workstation) [CRUD]

---

## Resources


### Workstation_cluster

Creates a new workstation cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subnetwork` | String |  | Immutable. Name of the Compute Engine subnetwork in which instances associated with this workstation cluster will be created. Must be part of the subnetwork specified for this workstation cluster. |
| `etag` | String |  | Optional. Checksum computed by the server. May be sent on update and delete requests to make sure that the client has an up-to-date value before proceeding. |
| `labels` | HashMap<String, String> |  | Optional. [Labels](https://cloud.google.com/workstations/docs/label-resources) that are applied to the workstation cluster and that are also propagated to the underlying Compute Engine resources. |
| `gateway_config` | String |  | Optional. Configuration options for Cluster HTTP Gateway. |
| `uid` | String |  | Output only. A system-assigned unique identifier for this workstation cluster. |
| `reconciling` | bool |  | Output only. Indicates whether this workstation cluster is currently being updated to match its intended state. |
| `conditions` | Vec<String> |  | Output only. Status conditions describing the workstation cluster's current state. |
| `delete_time` | String |  | Output only. Time when this workstation cluster was soft-deleted. |
| `control_plane_ip` | String |  | Output only. The private IP address of the control plane for this workstation cluster. Workstation VMs need access to this IP address to work with the service, so make sure that your firewall rules allow egress from the workstation VMs to this address. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `degraded` | bool |  | Output only. Whether this workstation cluster is in degraded mode, in which case it may require user action to restore full functionality. The conditions field contains detailed information about the status of the cluster. |
| `update_time` | String |  | Output only. Time when this workstation cluster was most recently updated. |
| `private_cluster_config` | String |  | Optional. Configuration for private workstation cluster. |
| `create_time` | String |  | Output only. Time when this workstation cluster was created. |
| `display_name` | String |  | Optional. Human-readable name for this workstation cluster. |
| `network` | String |  | Immutable. Name of the Compute Engine network in which instances associated with this workstation cluster will be created. |
| `domain_config` | String |  | Optional. Configuration options for a custom domain. |
| `annotations` | HashMap<String, String> |  | Optional. Client-specified annotations. |
| `name` | String |  | Identifier. Full name of this workstation cluster. |
| `parent` | String | ✅ | Required. Parent resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subnetwork` | String | Immutable. Name of the Compute Engine subnetwork in which instances associated with this workstation cluster will be created. Must be part of the subnetwork specified for this workstation cluster. |
| `etag` | String | Optional. Checksum computed by the server. May be sent on update and delete requests to make sure that the client has an up-to-date value before proceeding. |
| `labels` | HashMap<String, String> | Optional. [Labels](https://cloud.google.com/workstations/docs/label-resources) that are applied to the workstation cluster and that are also propagated to the underlying Compute Engine resources. |
| `gateway_config` | String | Optional. Configuration options for Cluster HTTP Gateway. |
| `uid` | String | Output only. A system-assigned unique identifier for this workstation cluster. |
| `reconciling` | bool | Output only. Indicates whether this workstation cluster is currently being updated to match its intended state. |
| `conditions` | Vec<String> | Output only. Status conditions describing the workstation cluster's current state. |
| `delete_time` | String | Output only. Time when this workstation cluster was soft-deleted. |
| `control_plane_ip` | String | Output only. The private IP address of the control plane for this workstation cluster. Workstation VMs need access to this IP address to work with the service, so make sure that your firewall rules allow egress from the workstation VMs to this address. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `degraded` | bool | Output only. Whether this workstation cluster is in degraded mode, in which case it may require user action to restore full functionality. The conditions field contains detailed information about the status of the cluster. |
| `update_time` | String | Output only. Time when this workstation cluster was most recently updated. |
| `private_cluster_config` | String | Optional. Configuration for private workstation cluster. |
| `create_time` | String | Output only. Time when this workstation cluster was created. |
| `display_name` | String | Optional. Human-readable name for this workstation cluster. |
| `network` | String | Immutable. Name of the Compute Engine network in which instances associated with this workstation cluster will be created. |
| `domain_config` | String | Optional. Configuration options for a custom domain. |
| `annotations` | HashMap<String, String> | Optional. Client-specified annotations. |
| `name` | String | Identifier. Full name of this workstation cluster. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workstation_cluster
workstation_cluster = provider.workstations_api.Workstation_cluster {
    parent = "value"  # Required. Parent resource name.
}

# Access workstation_cluster outputs
workstation_cluster_id = workstation_cluster.id
workstation_cluster_subnetwork = workstation_cluster.subnetwork
workstation_cluster_etag = workstation_cluster.etag
workstation_cluster_labels = workstation_cluster.labels
workstation_cluster_gateway_config = workstation_cluster.gateway_config
workstation_cluster_uid = workstation_cluster.uid
workstation_cluster_reconciling = workstation_cluster.reconciling
workstation_cluster_conditions = workstation_cluster.conditions
workstation_cluster_delete_time = workstation_cluster.delete_time
workstation_cluster_control_plane_ip = workstation_cluster.control_plane_ip
workstation_cluster_tags = workstation_cluster.tags
workstation_cluster_degraded = workstation_cluster.degraded
workstation_cluster_update_time = workstation_cluster.update_time
workstation_cluster_private_cluster_config = workstation_cluster.private_cluster_config
workstation_cluster_create_time = workstation_cluster.create_time
workstation_cluster_display_name = workstation_cluster.display_name
workstation_cluster_network = workstation_cluster.network
workstation_cluster_domain_config = workstation_cluster.domain_config
workstation_cluster_annotations = workstation_cluster.annotations
workstation_cluster_name = workstation_cluster.name
```

---


### Workstation_config

Creates a new workstation configuration.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `degraded` | bool |  | Output only. Whether this workstation configuration is in degraded mode, in which case it may require user action to restore full functionality. The conditions field contains detailed information about the status of the configuration. |
| `display_name` | String |  | Optional. Human-readable name for this workstation configuration. |
| `max_usable_workstations` | i64 |  | Optional. Maximum number of workstations under this configuration a user can have `workstations.workstation.use` permission on. Only enforced on CreateWorkstation API calls on the user issuing the API request. Can be overridden by: - granting a user workstations.workstationConfigs.exemptMaxUsableWorkstationLimit permission, or - having a user with that permission create a workstation and granting another user `workstations.workstation.use` permission on that workstation. If not specified, defaults to `0`, which indicates unlimited. |
| `name` | String |  | Identifier. Full name of this workstation configuration. |
| `enable_audit_agent` | bool |  | Optional. Whether to enable Linux `auditd` logging on the workstation. When enabled, a service_account must also be specified that has `roles/logging.logWriter` and `roles/monitoring.metricWriter` on the project. Operating system audit logging is distinct from [Cloud Audit Logs](https://cloud.google.com/workstations/docs/audit-logging) and [Container output logging](https://cloud.google.com/workstations/docs/container-output-logging#overview). Operating system audit logs are available in the [Cloud Logging](https://cloud.google.com/logging/docs) console by querying: resource.type="gce_instance" log_name:"/logs/linux-auditd" |
| `annotations` | HashMap<String, String> |  | Optional. Client-specified annotations. |
| `grant_workstation_admin_role_on_create` | bool |  | Optional. Grant creator of a workstation `roles/workstations.policyAdmin` role along with `roles/workstations.user` role on the workstation created by them. This allows workstation users to share access to either their entire workstation, or individual ports. Defaults to false. |
| `reconciling` | bool |  | Output only. Indicates whether this workstation configuration is currently being updated to match its intended state. |
| `create_time` | String |  | Output only. Time when this workstation configuration was created. |
| `disable_tcp_connections` | bool |  | Optional. Disables support for plain TCP connections in the workstation. By default the service supports TCP connections through a websocket relay. Setting this option to true disables that relay, which prevents the usage of services that require plain TCP connections, such as SSH. When enabled, all communication must occur over HTTPS or WSS. |
| `idle_timeout` | String |  | Optional. Number of seconds to wait before automatically stopping a workstation after it last received user traffic. A value of `"0s"` indicates that Cloud Workstations VMs created with this configuration should never time out due to idleness. Provide [duration](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#duration) terminated by `s` for seconds—for example, `"7200s"` (2 hours). The default is `"1200s"` (20 minutes). |
| `delete_time` | String |  | Output only. Time when this workstation configuration was soft-deleted. |
| `running_timeout` | String |  | Optional. Number of seconds that a workstation can run until it is automatically shut down. We recommend that workstations be shut down daily to reduce costs and so that security updates can be applied upon restart. The idle_timeout and running_timeout fields are independent of each other. Note that the running_timeout field shuts down VMs after the specified time, regardless of whether or not the VMs are idle. Provide duration terminated by `s` for seconds—for example, `"54000s"` (15 hours). Defaults to `"43200s"` (12 hours). A value of `"0s"` indicates that workstations using this configuration should never time out. If encryption_key is set, it must be greater than `"0s"` and less than `"86400s"` (24 hours). Warning: A value of `"0s"` indicates that Cloud Workstations VMs created with this configuration have no maximum running time. This is strongly discouraged because you incur costs and will not pick up security updates. |
| `container` | String |  | Optional. Container that runs upon startup for each workstation using this workstation configuration. |
| `uid` | String |  | Output only. A system-assigned unique identifier for this workstation configuration. |
| `ephemeral_directories` | Vec<String> |  | Optional. Ephemeral directories which won't persist across workstation sessions. |
| `update_time` | String |  | Output only. Time when this workstation configuration was most recently updated. |
| `etag` | String |  | Optional. Checksum computed by the server. May be sent on update and delete requests to make sure that the client has an up-to-date value before proceeding. |
| `readiness_checks` | Vec<String> |  | Optional. Readiness checks to perform when starting a workstation using this workstation configuration. Mark a workstation as running only after all specified readiness checks return 200 status codes. |
| `persistent_directories` | Vec<String> |  | Optional. Directories to persist across workstation sessions. |
| `allowed_ports` | Vec<String> |  | Optional. A list of PortRanges specifying single ports or ranges of ports that are externally accessible in the workstation. Allowed ports must be one of 22, 80, or within range 1024-65535. If not specified defaults to ports 22, 80, and ports 1024-65535. |
| `labels` | HashMap<String, String> |  | Optional. [Labels](https://cloud.google.com/workstations/docs/label-resources) that are applied to the workstation configuration and that are also propagated to the underlying Compute Engine resources. |
| `replica_zones` | Vec<String> |  | Optional. Immutable. Specifies the zones used to replicate the VM and disk resources within the region. If set, exactly two zones within the workstation cluster's region must be specified—for example, `['us-central1-a', 'us-central1-f']`. If this field is empty, two default zones within the region are used. Immutable after the workstation configuration is created. |
| `conditions` | Vec<String> |  | Output only. Status conditions describing the workstation configuration's current state. |
| `encryption_key` | String |  | Immutable. Encrypts resources of this workstation configuration using a customer-managed encryption key (CMEK). If specified, the boot disk of the Compute Engine instance and the persistent disk are encrypted using this encryption key. If this field is not set, the disks are encrypted using a generated key. Customer-managed encryption keys do not protect disk metadata. If the customer-managed encryption key is rotated, when the workstation instance is stopped, the system attempts to recreate the persistent disk with the new version of the key. Be sure to keep older versions of the key until the persistent disk is recreated. Otherwise, data on the persistent disk might be lost. If the encryption key is revoked, the workstation session automatically stops within 7 hours. Immutable after the workstation configuration is created. |
| `host` | String |  | Optional. Runtime host for the workstation. |
| `parent` | String | ✅ | Required. Parent resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `degraded` | bool | Output only. Whether this workstation configuration is in degraded mode, in which case it may require user action to restore full functionality. The conditions field contains detailed information about the status of the configuration. |
| `display_name` | String | Optional. Human-readable name for this workstation configuration. |
| `max_usable_workstations` | i64 | Optional. Maximum number of workstations under this configuration a user can have `workstations.workstation.use` permission on. Only enforced on CreateWorkstation API calls on the user issuing the API request. Can be overridden by: - granting a user workstations.workstationConfigs.exemptMaxUsableWorkstationLimit permission, or - having a user with that permission create a workstation and granting another user `workstations.workstation.use` permission on that workstation. If not specified, defaults to `0`, which indicates unlimited. |
| `name` | String | Identifier. Full name of this workstation configuration. |
| `enable_audit_agent` | bool | Optional. Whether to enable Linux `auditd` logging on the workstation. When enabled, a service_account must also be specified that has `roles/logging.logWriter` and `roles/monitoring.metricWriter` on the project. Operating system audit logging is distinct from [Cloud Audit Logs](https://cloud.google.com/workstations/docs/audit-logging) and [Container output logging](https://cloud.google.com/workstations/docs/container-output-logging#overview). Operating system audit logs are available in the [Cloud Logging](https://cloud.google.com/logging/docs) console by querying: resource.type="gce_instance" log_name:"/logs/linux-auditd" |
| `annotations` | HashMap<String, String> | Optional. Client-specified annotations. |
| `grant_workstation_admin_role_on_create` | bool | Optional. Grant creator of a workstation `roles/workstations.policyAdmin` role along with `roles/workstations.user` role on the workstation created by them. This allows workstation users to share access to either their entire workstation, or individual ports. Defaults to false. |
| `reconciling` | bool | Output only. Indicates whether this workstation configuration is currently being updated to match its intended state. |
| `create_time` | String | Output only. Time when this workstation configuration was created. |
| `disable_tcp_connections` | bool | Optional. Disables support for plain TCP connections in the workstation. By default the service supports TCP connections through a websocket relay. Setting this option to true disables that relay, which prevents the usage of services that require plain TCP connections, such as SSH. When enabled, all communication must occur over HTTPS or WSS. |
| `idle_timeout` | String | Optional. Number of seconds to wait before automatically stopping a workstation after it last received user traffic. A value of `"0s"` indicates that Cloud Workstations VMs created with this configuration should never time out due to idleness. Provide [duration](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#duration) terminated by `s` for seconds—for example, `"7200s"` (2 hours). The default is `"1200s"` (20 minutes). |
| `delete_time` | String | Output only. Time when this workstation configuration was soft-deleted. |
| `running_timeout` | String | Optional. Number of seconds that a workstation can run until it is automatically shut down. We recommend that workstations be shut down daily to reduce costs and so that security updates can be applied upon restart. The idle_timeout and running_timeout fields are independent of each other. Note that the running_timeout field shuts down VMs after the specified time, regardless of whether or not the VMs are idle. Provide duration terminated by `s` for seconds—for example, `"54000s"` (15 hours). Defaults to `"43200s"` (12 hours). A value of `"0s"` indicates that workstations using this configuration should never time out. If encryption_key is set, it must be greater than `"0s"` and less than `"86400s"` (24 hours). Warning: A value of `"0s"` indicates that Cloud Workstations VMs created with this configuration have no maximum running time. This is strongly discouraged because you incur costs and will not pick up security updates. |
| `container` | String | Optional. Container that runs upon startup for each workstation using this workstation configuration. |
| `uid` | String | Output only. A system-assigned unique identifier for this workstation configuration. |
| `ephemeral_directories` | Vec<String> | Optional. Ephemeral directories which won't persist across workstation sessions. |
| `update_time` | String | Output only. Time when this workstation configuration was most recently updated. |
| `etag` | String | Optional. Checksum computed by the server. May be sent on update and delete requests to make sure that the client has an up-to-date value before proceeding. |
| `readiness_checks` | Vec<String> | Optional. Readiness checks to perform when starting a workstation using this workstation configuration. Mark a workstation as running only after all specified readiness checks return 200 status codes. |
| `persistent_directories` | Vec<String> | Optional. Directories to persist across workstation sessions. |
| `allowed_ports` | Vec<String> | Optional. A list of PortRanges specifying single ports or ranges of ports that are externally accessible in the workstation. Allowed ports must be one of 22, 80, or within range 1024-65535. If not specified defaults to ports 22, 80, and ports 1024-65535. |
| `labels` | HashMap<String, String> | Optional. [Labels](https://cloud.google.com/workstations/docs/label-resources) that are applied to the workstation configuration and that are also propagated to the underlying Compute Engine resources. |
| `replica_zones` | Vec<String> | Optional. Immutable. Specifies the zones used to replicate the VM and disk resources within the region. If set, exactly two zones within the workstation cluster's region must be specified—for example, `['us-central1-a', 'us-central1-f']`. If this field is empty, two default zones within the region are used. Immutable after the workstation configuration is created. |
| `conditions` | Vec<String> | Output only. Status conditions describing the workstation configuration's current state. |
| `encryption_key` | String | Immutable. Encrypts resources of this workstation configuration using a customer-managed encryption key (CMEK). If specified, the boot disk of the Compute Engine instance and the persistent disk are encrypted using this encryption key. If this field is not set, the disks are encrypted using a generated key. Customer-managed encryption keys do not protect disk metadata. If the customer-managed encryption key is rotated, when the workstation instance is stopped, the system attempts to recreate the persistent disk with the new version of the key. Be sure to keep older versions of the key until the persistent disk is recreated. Otherwise, data on the persistent disk might be lost. If the encryption key is revoked, the workstation session automatically stops within 7 hours. Immutable after the workstation configuration is created. |
| `host` | String | Optional. Runtime host for the workstation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workstation_config
workstation_config = provider.workstations_api.Workstation_config {
    parent = "value"  # Required. Parent resource name.
}

# Access workstation_config outputs
workstation_config_id = workstation_config.id
workstation_config_degraded = workstation_config.degraded
workstation_config_display_name = workstation_config.display_name
workstation_config_max_usable_workstations = workstation_config.max_usable_workstations
workstation_config_name = workstation_config.name
workstation_config_enable_audit_agent = workstation_config.enable_audit_agent
workstation_config_annotations = workstation_config.annotations
workstation_config_grant_workstation_admin_role_on_create = workstation_config.grant_workstation_admin_role_on_create
workstation_config_reconciling = workstation_config.reconciling
workstation_config_create_time = workstation_config.create_time
workstation_config_disable_tcp_connections = workstation_config.disable_tcp_connections
workstation_config_idle_timeout = workstation_config.idle_timeout
workstation_config_delete_time = workstation_config.delete_time
workstation_config_running_timeout = workstation_config.running_timeout
workstation_config_container = workstation_config.container
workstation_config_uid = workstation_config.uid
workstation_config_ephemeral_directories = workstation_config.ephemeral_directories
workstation_config_update_time = workstation_config.update_time
workstation_config_etag = workstation_config.etag
workstation_config_readiness_checks = workstation_config.readiness_checks
workstation_config_persistent_directories = workstation_config.persistent_directories
workstation_config_allowed_ports = workstation_config.allowed_ports
workstation_config_labels = workstation_config.labels
workstation_config_replica_zones = workstation_config.replica_zones
workstation_config_conditions = workstation_config.conditions
workstation_config_encryption_key = workstation_config.encryption_key
workstation_config_host = workstation_config.host
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
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
location_name = location.name
location_labels = location.labels
location_display_name = location.display_name
location_location_id = location.location_id
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.workstations_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
```

---


### Workstation

Creates a new workstation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `host` | String |  | Output only. Host to which clients can send HTTPS traffic that will be received by the workstation. Authorized traffic will be received to the workstation as HTTP on port 80. To send traffic to a different port, clients may prefix the host with the destination port in the format `{port}-{host}`. |
| `kms_key` | String |  | Output only. The name of the Google Cloud KMS encryption key used to encrypt this workstation. The KMS key can only be configured in the WorkstationConfig. The expected format is `projects/*/locations/*/keyRings/*/cryptoKeys/*`. |
| `create_time` | String |  | Output only. Time when this workstation was created. |
| `display_name` | String |  | Optional. Human-readable name for this workstation. |
| `start_time` | String |  | Output only. Time when this workstation was most recently successfully started, regardless of the workstation's initial state. |
| `source_workstation` | String |  | Optional. The source workstation from which this workstation's persistent directories were cloned on creation. |
| `uid` | String |  | Output only. A system-assigned unique identifier for this workstation. |
| `etag` | String |  | Optional. Checksum computed by the server. May be sent on update and delete requests to make sure that the client has an up-to-date value before proceeding. |
| `state` | String |  | Output only. Current state of the workstation. |
| `env` | HashMap<String, String> |  | Optional. Environment variables passed to the workstation container's entrypoint. |
| `labels` | HashMap<String, String> |  | Optional. [Labels](https://cloud.google.com/workstations/docs/label-resources) that are applied to the workstation and that are also propagated to the underlying Compute Engine resources. |
| `update_time` | String |  | Output only. Time when this workstation was most recently updated. |
| `runtime_host` | String |  | Optional. Output only. Runtime host for the workstation when in STATE_RUNNING. |
| `delete_time` | String |  | Output only. Time when this workstation was soft-deleted. |
| `reconciling` | bool |  | Output only. Indicates whether this workstation is currently being updated to match its intended state. |
| `annotations` | HashMap<String, String> |  | Optional. Client-specified annotations. |
| `name` | String |  | Identifier. Full name of this workstation. |
| `parent` | String | ✅ | Required. Parent resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `host` | String | Output only. Host to which clients can send HTTPS traffic that will be received by the workstation. Authorized traffic will be received to the workstation as HTTP on port 80. To send traffic to a different port, clients may prefix the host with the destination port in the format `{port}-{host}`. |
| `kms_key` | String | Output only. The name of the Google Cloud KMS encryption key used to encrypt this workstation. The KMS key can only be configured in the WorkstationConfig. The expected format is `projects/*/locations/*/keyRings/*/cryptoKeys/*`. |
| `create_time` | String | Output only. Time when this workstation was created. |
| `display_name` | String | Optional. Human-readable name for this workstation. |
| `start_time` | String | Output only. Time when this workstation was most recently successfully started, regardless of the workstation's initial state. |
| `source_workstation` | String | Optional. The source workstation from which this workstation's persistent directories were cloned on creation. |
| `uid` | String | Output only. A system-assigned unique identifier for this workstation. |
| `etag` | String | Optional. Checksum computed by the server. May be sent on update and delete requests to make sure that the client has an up-to-date value before proceeding. |
| `state` | String | Output only. Current state of the workstation. |
| `env` | HashMap<String, String> | Optional. Environment variables passed to the workstation container's entrypoint. |
| `labels` | HashMap<String, String> | Optional. [Labels](https://cloud.google.com/workstations/docs/label-resources) that are applied to the workstation and that are also propagated to the underlying Compute Engine resources. |
| `update_time` | String | Output only. Time when this workstation was most recently updated. |
| `runtime_host` | String | Optional. Output only. Runtime host for the workstation when in STATE_RUNNING. |
| `delete_time` | String | Output only. Time when this workstation was soft-deleted. |
| `reconciling` | bool | Output only. Indicates whether this workstation is currently being updated to match its intended state. |
| `annotations` | HashMap<String, String> | Optional. Client-specified annotations. |
| `name` | String | Identifier. Full name of this workstation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workstation
workstation = provider.workstations_api.Workstation {
    parent = "value"  # Required. Parent resource name.
}

# Access workstation outputs
workstation_id = workstation.id
workstation_host = workstation.host
workstation_kms_key = workstation.kms_key
workstation_create_time = workstation.create_time
workstation_display_name = workstation.display_name
workstation_start_time = workstation.start_time
workstation_source_workstation = workstation.source_workstation
workstation_uid = workstation.uid
workstation_etag = workstation.etag
workstation_state = workstation.state
workstation_env = workstation.env
workstation_labels = workstation.labels
workstation_update_time = workstation.update_time
workstation_runtime_host = workstation.runtime_host
workstation_delete_time = workstation.delete_time
workstation_reconciling = workstation.reconciling
workstation_annotations = workstation.annotations
workstation_name = workstation.name
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
operation = provider.workstations_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
operation_metadata = operation.metadata
operation_name = operation.name
```

---


### Workstation_config

Creates a new workstation configuration.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `container` | String |  | Optional. Container that runs upon startup for each workstation using this workstation configuration. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `conditions` | Vec<String> |  | Output only. Status conditions describing the workstation configuration's current state. |
| `update_time` | String |  | Output only. Time when this workstation configuration was most recently updated. |
| `replica_zones` | Vec<String> |  | Optional. Immutable. Specifies the zones used to replicate the VM and disk resources within the region. If set, exactly two zones within the workstation cluster's region must be specified—for example, `['us-central1-a', 'us-central1-f']`. If this field is empty, two default zones within the region are used. Immutable after the workstation configuration is created. |
| `name` | String |  | Identifier. Full name of this workstation configuration. |
| `persistent_directories` | Vec<String> |  | Optional. Directories to persist across workstation sessions. |
| `encryption_key` | String |  | Immutable. Encrypts resources of this workstation configuration using a customer-managed encryption key (CMEK). If specified, the boot disk of the Compute Engine instance and the persistent disk are encrypted using this encryption key. If this field is not set, the disks are encrypted using a generated key. Customer-managed encryption keys do not protect disk metadata. If the customer-managed encryption key is rotated, when the workstation instance is stopped, the system attempts to recreate the persistent disk with the new version of the key. Be sure to keep older versions of the key until the persistent disk is recreated. Otherwise, data on the persistent disk might be lost. If the encryption key is revoked, the workstation session automatically stops within 7 hours. Immutable after the workstation configuration is created. |
| `readiness_checks` | Vec<String> |  | Optional. Readiness checks to perform when starting a workstation using this workstation configuration. Mark a workstation as running only after all specified readiness checks return 200 status codes. |
| `allowed_ports` | Vec<String> |  | Optional. A list of PortRanges specifying single ports or ranges of ports that are externally accessible in the workstation. Allowed ports must be one of 22, 80, or within range 1024-65535. If not specified defaults to ports 22, 80, and ports 1024-65535. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `annotations` | HashMap<String, String> |  | Optional. Client-specified annotations. |
| `create_time` | String |  | Output only. Time when this workstation configuration was created. |
| `idle_timeout` | String |  | Optional. Number of seconds to wait before automatically stopping a workstation after it last received user traffic. A value of `"0s"` indicates that Cloud Workstations VMs created with this configuration should never time out due to idleness. Provide [duration](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#duration) terminated by `s` for seconds—for example, `"7200s"` (2 hours). The default is `"1200s"` (20 minutes). |
| `reconciling` | bool |  | Output only. Indicates whether this workstation configuration is currently being updated to match its intended state. |
| `labels` | HashMap<String, String> |  | Optional. [Labels](https://cloud.google.com/workstations/docs/label-resources) that are applied to the workstation configuration and that are also propagated to the underlying Compute Engine resources. |
| `delete_time` | String |  | Output only. Time when this workstation configuration was soft-deleted. |
| `http_options` | String |  | Optional. HTTP options that customize the behavior of the workstation service's HTTP proxy. |
| `max_usable_workstations` | i64 |  | Optional. Maximum number of workstations under this configuration a user can have `workstations.workstation.use` permission on. Only enforced on CreateWorkstation API calls on the user issuing the API request. Can be overridden by: - granting a user workstations.workstationConfigs.exemptMaxUsableWorkstationLimit permission, or - having a user with that permission create a workstation and granting another user `workstations.workstation.use` permission on that workstation. If not specified, defaults to `0`, which indicates unlimited. |
| `host` | String |  | Optional. Runtime host for the workstation. |
| `etag` | String |  | Optional. Checksum computed by the server. May be sent on update and delete requests to make sure that the client has an up-to-date value before proceeding. |
| `degraded` | bool |  | Output only. Whether this workstation configuration is in degraded mode, in which case it may require user action to restore full functionality. The conditions field contains detailed information about the status of the configuration. |
| `ephemeral_directories` | Vec<String> |  | Optional. Ephemeral directories which won't persist across workstation sessions. |
| `grant_workstation_admin_role_on_create` | bool |  | Optional. Grant creator of a workstation `roles/workstations.policyAdmin` role along with `roles/workstations.user` role on the workstation created by them. This allows workstation users to share access to either their entire workstation, or individual ports. Defaults to false. |
| `uid` | String |  | Output only. A system-assigned unique identifier for this workstation configuration. |
| `display_name` | String |  | Optional. Human-readable name for this workstation configuration. |
| `disable_tcp_connections` | bool |  | Optional. Disables support for plain TCP connections in the workstation. By default the service supports TCP connections through a websocket relay. Setting this option to true disables that relay, which prevents the usage of services that require plain TCP connections, such as SSH. When enabled, all communication must occur over HTTPS or WSS. |
| `enable_audit_agent` | bool |  | Optional. Whether to enable Linux `auditd` logging on the workstation. When enabled, a service_account must also be specified that has `roles/logging.logWriter` and `roles/monitoring.metricWriter` on the project. Operating system audit logging is distinct from [Cloud Audit Logs](https://cloud.google.com/workstations/docs/audit-logging) and [Container output logging](https://cloud.google.com/workstations/docs/container-output-logging#overview). Operating system audit logs are available in the [Cloud Logging](https://cloud.google.com/logging/docs) console by querying: resource.type="gce_instance" log_name:"/logs/linux-auditd" |
| `running_timeout` | String |  | Optional. Number of seconds that a workstation can run until it is automatically shut down. We recommend that workstations be shut down daily to reduce costs and so that security updates can be applied upon restart. The idle_timeout and running_timeout fields are independent of each other. Note that the running_timeout field shuts down VMs after the specified time, regardless of whether or not the VMs are idle. Provide duration terminated by `s` for seconds—for example, `"54000s"` (15 hours). Defaults to `"43200s"` (12 hours). A value of `"0s"` indicates that workstations using this configuration should never time out. If encryption_key is set, it must be greater than `"0s"` and less than `"86400s"` (24 hours). Warning: A value of `"0s"` indicates that Cloud Workstations VMs created with this configuration have no maximum running time. This is strongly discouraged because you incur costs and will not pick up security updates. |
| `parent` | String | ✅ | Required. Parent resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `container` | String | Optional. Container that runs upon startup for each workstation using this workstation configuration. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `conditions` | Vec<String> | Output only. Status conditions describing the workstation configuration's current state. |
| `update_time` | String | Output only. Time when this workstation configuration was most recently updated. |
| `replica_zones` | Vec<String> | Optional. Immutable. Specifies the zones used to replicate the VM and disk resources within the region. If set, exactly two zones within the workstation cluster's region must be specified—for example, `['us-central1-a', 'us-central1-f']`. If this field is empty, two default zones within the region are used. Immutable after the workstation configuration is created. |
| `name` | String | Identifier. Full name of this workstation configuration. |
| `persistent_directories` | Vec<String> | Optional. Directories to persist across workstation sessions. |
| `encryption_key` | String | Immutable. Encrypts resources of this workstation configuration using a customer-managed encryption key (CMEK). If specified, the boot disk of the Compute Engine instance and the persistent disk are encrypted using this encryption key. If this field is not set, the disks are encrypted using a generated key. Customer-managed encryption keys do not protect disk metadata. If the customer-managed encryption key is rotated, when the workstation instance is stopped, the system attempts to recreate the persistent disk with the new version of the key. Be sure to keep older versions of the key until the persistent disk is recreated. Otherwise, data on the persistent disk might be lost. If the encryption key is revoked, the workstation session automatically stops within 7 hours. Immutable after the workstation configuration is created. |
| `readiness_checks` | Vec<String> | Optional. Readiness checks to perform when starting a workstation using this workstation configuration. Mark a workstation as running only after all specified readiness checks return 200 status codes. |
| `allowed_ports` | Vec<String> | Optional. A list of PortRanges specifying single ports or ranges of ports that are externally accessible in the workstation. Allowed ports must be one of 22, 80, or within range 1024-65535. If not specified defaults to ports 22, 80, and ports 1024-65535. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `annotations` | HashMap<String, String> | Optional. Client-specified annotations. |
| `create_time` | String | Output only. Time when this workstation configuration was created. |
| `idle_timeout` | String | Optional. Number of seconds to wait before automatically stopping a workstation after it last received user traffic. A value of `"0s"` indicates that Cloud Workstations VMs created with this configuration should never time out due to idleness. Provide [duration](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#duration) terminated by `s` for seconds—for example, `"7200s"` (2 hours). The default is `"1200s"` (20 minutes). |
| `reconciling` | bool | Output only. Indicates whether this workstation configuration is currently being updated to match its intended state. |
| `labels` | HashMap<String, String> | Optional. [Labels](https://cloud.google.com/workstations/docs/label-resources) that are applied to the workstation configuration and that are also propagated to the underlying Compute Engine resources. |
| `delete_time` | String | Output only. Time when this workstation configuration was soft-deleted. |
| `http_options` | String | Optional. HTTP options that customize the behavior of the workstation service's HTTP proxy. |
| `max_usable_workstations` | i64 | Optional. Maximum number of workstations under this configuration a user can have `workstations.workstation.use` permission on. Only enforced on CreateWorkstation API calls on the user issuing the API request. Can be overridden by: - granting a user workstations.workstationConfigs.exemptMaxUsableWorkstationLimit permission, or - having a user with that permission create a workstation and granting another user `workstations.workstation.use` permission on that workstation. If not specified, defaults to `0`, which indicates unlimited. |
| `host` | String | Optional. Runtime host for the workstation. |
| `etag` | String | Optional. Checksum computed by the server. May be sent on update and delete requests to make sure that the client has an up-to-date value before proceeding. |
| `degraded` | bool | Output only. Whether this workstation configuration is in degraded mode, in which case it may require user action to restore full functionality. The conditions field contains detailed information about the status of the configuration. |
| `ephemeral_directories` | Vec<String> | Optional. Ephemeral directories which won't persist across workstation sessions. |
| `grant_workstation_admin_role_on_create` | bool | Optional. Grant creator of a workstation `roles/workstations.policyAdmin` role along with `roles/workstations.user` role on the workstation created by them. This allows workstation users to share access to either their entire workstation, or individual ports. Defaults to false. |
| `uid` | String | Output only. A system-assigned unique identifier for this workstation configuration. |
| `display_name` | String | Optional. Human-readable name for this workstation configuration. |
| `disable_tcp_connections` | bool | Optional. Disables support for plain TCP connections in the workstation. By default the service supports TCP connections through a websocket relay. Setting this option to true disables that relay, which prevents the usage of services that require plain TCP connections, such as SSH. When enabled, all communication must occur over HTTPS or WSS. |
| `enable_audit_agent` | bool | Optional. Whether to enable Linux `auditd` logging on the workstation. When enabled, a service_account must also be specified that has `roles/logging.logWriter` and `roles/monitoring.metricWriter` on the project. Operating system audit logging is distinct from [Cloud Audit Logs](https://cloud.google.com/workstations/docs/audit-logging) and [Container output logging](https://cloud.google.com/workstations/docs/container-output-logging#overview). Operating system audit logs are available in the [Cloud Logging](https://cloud.google.com/logging/docs) console by querying: resource.type="gce_instance" log_name:"/logs/linux-auditd" |
| `running_timeout` | String | Optional. Number of seconds that a workstation can run until it is automatically shut down. We recommend that workstations be shut down daily to reduce costs and so that security updates can be applied upon restart. The idle_timeout and running_timeout fields are independent of each other. Note that the running_timeout field shuts down VMs after the specified time, regardless of whether or not the VMs are idle. Provide duration terminated by `s` for seconds—for example, `"54000s"` (15 hours). Defaults to `"43200s"` (12 hours). A value of `"0s"` indicates that workstations using this configuration should never time out. If encryption_key is set, it must be greater than `"0s"` and less than `"86400s"` (24 hours). Warning: A value of `"0s"` indicates that Cloud Workstations VMs created with this configuration have no maximum running time. This is strongly discouraged because you incur costs and will not pick up security updates. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workstation_config
workstation_config = provider.workstations_api.Workstation_config {
    parent = "value"  # Required. Parent resource name.
}

# Access workstation_config outputs
workstation_config_id = workstation_config.id
workstation_config_container = workstation_config.container
workstation_config_satisfies_pzi = workstation_config.satisfies_pzi
workstation_config_conditions = workstation_config.conditions
workstation_config_update_time = workstation_config.update_time
workstation_config_replica_zones = workstation_config.replica_zones
workstation_config_name = workstation_config.name
workstation_config_persistent_directories = workstation_config.persistent_directories
workstation_config_encryption_key = workstation_config.encryption_key
workstation_config_readiness_checks = workstation_config.readiness_checks
workstation_config_allowed_ports = workstation_config.allowed_ports
workstation_config_satisfies_pzs = workstation_config.satisfies_pzs
workstation_config_annotations = workstation_config.annotations
workstation_config_create_time = workstation_config.create_time
workstation_config_idle_timeout = workstation_config.idle_timeout
workstation_config_reconciling = workstation_config.reconciling
workstation_config_labels = workstation_config.labels
workstation_config_delete_time = workstation_config.delete_time
workstation_config_http_options = workstation_config.http_options
workstation_config_max_usable_workstations = workstation_config.max_usable_workstations
workstation_config_host = workstation_config.host
workstation_config_etag = workstation_config.etag
workstation_config_degraded = workstation_config.degraded
workstation_config_ephemeral_directories = workstation_config.ephemeral_directories
workstation_config_grant_workstation_admin_role_on_create = workstation_config.grant_workstation_admin_role_on_create
workstation_config_uid = workstation_config.uid
workstation_config_display_name = workstation_config.display_name
workstation_config_disable_tcp_connections = workstation_config.disable_tcp_connections
workstation_config_enable_audit_agent = workstation_config.enable_audit_agent
workstation_config_running_timeout = workstation_config.running_timeout
```

---


### Workstation_cluster

Creates a new workstation cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `control_plane_ip` | String |  | Output only. The private IP address of the control plane for this workstation cluster. Workstation VMs need access to this IP address to work with the service, so make sure that your firewall rules allow egress from the workstation VMs to this address. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `reconciling` | bool |  | Output only. Indicates whether this workstation cluster is currently being updated to match its intended state. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `annotations` | HashMap<String, String> |  | Optional. Client-specified annotations. |
| `subnetwork` | String |  | Immutable. Name of the Compute Engine subnetwork in which instances associated with this workstation cluster will be created. Must be part of the subnetwork specified for this workstation cluster. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `update_time` | String |  | Output only. Time when this workstation cluster was most recently updated. |
| `labels` | HashMap<String, String> |  | Optional. [Labels](https://cloud.google.com/workstations/docs/label-resources) that are applied to the workstation cluster and that are also propagated to the underlying Compute Engine resources. |
| `uid` | String |  | Output only. A system-assigned unique identifier for this workstation cluster. |
| `degraded` | bool |  | Output only. Whether this workstation cluster is in degraded mode, in which case it may require user action to restore full functionality. The conditions field contains detailed information about the status of the cluster. |
| `domain_config` | String |  | Optional. Configuration options for a custom domain. |
| `gateway_config` | String |  | Optional. Configuration options for Cluster HTTP Gateway. |
| `delete_time` | String |  | Output only. Time when this workstation cluster was soft-deleted. |
| `display_name` | String |  | Optional. Human-readable name for this workstation cluster. |
| `create_time` | String |  | Output only. Time when this workstation cluster was created. |
| `conditions` | Vec<String> |  | Output only. Status conditions describing the workstation cluster's current state. |
| `name` | String |  | Identifier. Full name of this workstation cluster. |
| `private_cluster_config` | String |  | Optional. Configuration for private workstation cluster. |
| `etag` | String |  | Optional. Checksum computed by the server. May be sent on update and delete requests to make sure that the client has an up-to-date value before proceeding. |
| `network` | String |  | Immutable. Name of the Compute Engine network in which instances associated with this workstation cluster will be created. |
| `parent` | String | ✅ | Required. Parent resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `control_plane_ip` | String | Output only. The private IP address of the control plane for this workstation cluster. Workstation VMs need access to this IP address to work with the service, so make sure that your firewall rules allow egress from the workstation VMs to this address. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `reconciling` | bool | Output only. Indicates whether this workstation cluster is currently being updated to match its intended state. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `annotations` | HashMap<String, String> | Optional. Client-specified annotations. |
| `subnetwork` | String | Immutable. Name of the Compute Engine subnetwork in which instances associated with this workstation cluster will be created. Must be part of the subnetwork specified for this workstation cluster. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `update_time` | String | Output only. Time when this workstation cluster was most recently updated. |
| `labels` | HashMap<String, String> | Optional. [Labels](https://cloud.google.com/workstations/docs/label-resources) that are applied to the workstation cluster and that are also propagated to the underlying Compute Engine resources. |
| `uid` | String | Output only. A system-assigned unique identifier for this workstation cluster. |
| `degraded` | bool | Output only. Whether this workstation cluster is in degraded mode, in which case it may require user action to restore full functionality. The conditions field contains detailed information about the status of the cluster. |
| `domain_config` | String | Optional. Configuration options for a custom domain. |
| `gateway_config` | String | Optional. Configuration options for Cluster HTTP Gateway. |
| `delete_time` | String | Output only. Time when this workstation cluster was soft-deleted. |
| `display_name` | String | Optional. Human-readable name for this workstation cluster. |
| `create_time` | String | Output only. Time when this workstation cluster was created. |
| `conditions` | Vec<String> | Output only. Status conditions describing the workstation cluster's current state. |
| `name` | String | Identifier. Full name of this workstation cluster. |
| `private_cluster_config` | String | Optional. Configuration for private workstation cluster. |
| `etag` | String | Optional. Checksum computed by the server. May be sent on update and delete requests to make sure that the client has an up-to-date value before proceeding. |
| `network` | String | Immutable. Name of the Compute Engine network in which instances associated with this workstation cluster will be created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workstation_cluster
workstation_cluster = provider.workstations_api.Workstation_cluster {
    parent = "value"  # Required. Parent resource name.
}

# Access workstation_cluster outputs
workstation_cluster_id = workstation_cluster.id
workstation_cluster_control_plane_ip = workstation_cluster.control_plane_ip
workstation_cluster_satisfies_pzi = workstation_cluster.satisfies_pzi
workstation_cluster_reconciling = workstation_cluster.reconciling
workstation_cluster_satisfies_pzs = workstation_cluster.satisfies_pzs
workstation_cluster_annotations = workstation_cluster.annotations
workstation_cluster_subnetwork = workstation_cluster.subnetwork
workstation_cluster_tags = workstation_cluster.tags
workstation_cluster_update_time = workstation_cluster.update_time
workstation_cluster_labels = workstation_cluster.labels
workstation_cluster_uid = workstation_cluster.uid
workstation_cluster_degraded = workstation_cluster.degraded
workstation_cluster_domain_config = workstation_cluster.domain_config
workstation_cluster_gateway_config = workstation_cluster.gateway_config
workstation_cluster_delete_time = workstation_cluster.delete_time
workstation_cluster_display_name = workstation_cluster.display_name
workstation_cluster_create_time = workstation_cluster.create_time
workstation_cluster_conditions = workstation_cluster.conditions
workstation_cluster_name = workstation_cluster.name
workstation_cluster_private_cluster_config = workstation_cluster.private_cluster_config
workstation_cluster_etag = workstation_cluster.etag
workstation_cluster_network = workstation_cluster.network
```

---


### Workstation

Creates a new workstation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `annotations` | HashMap<String, String> |  | Optional. Client-specified annotations. |
| `start_time` | String |  | Output only. Time when this workstation was most recently successfully started, regardless of the workstation's initial state. |
| `update_time` | String |  | Output only. Time when this workstation was most recently updated. |
| `display_name` | String |  | Optional. Human-readable name for this workstation. |
| `name` | String |  | Identifier. Full name of this workstation. |
| `state` | String |  | Output only. Current state of the workstation. |
| `etag` | String |  | Optional. Checksum computed by the server. May be sent on update and delete requests to make sure that the client has an up-to-date value before proceeding. |
| `runtime_host` | String |  | Optional. Output only. Runtime host for the workstation when in STATE_RUNNING. |
| `labels` | HashMap<String, String> |  | Optional. [Labels](https://cloud.google.com/workstations/docs/label-resources) that are applied to the workstation and that are also propagated to the underlying Compute Engine resources. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `kms_key` | String |  | Output only. The name of the Google Cloud KMS encryption key used to encrypt this workstation. The KMS key can only be configured in the WorkstationConfig. The expected format is `projects/*/locations/*/keyRings/*/cryptoKeys/*`. |
| `uid` | String |  | Output only. A system-assigned unique identifier for this workstation. |
| `degraded` | bool |  | Output only. Whether this workstation is in degraded mode, in which case it may require user action to restore full functionality. The conditions field contains detailed information about the status of the workstation. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `boost_configs` | Vec<String> |  | Output only. List of available boost configuration IDs that this workstation can be boosted up to. |
| `delete_time` | String |  | Output only. Time when this workstation was soft-deleted. |
| `reconciling` | bool |  | Output only. Indicates whether this workstation is currently being updated to match its intended state. |
| `create_time` | String |  | Output only. Time when this workstation was created. |
| `env` | HashMap<String, String> |  | Optional. Environment variables passed to the workstation container's entrypoint. |
| `conditions` | Vec<String> |  | Output only. Status conditions describing the workstation's current state. |
| `host` | String |  | Output only. Host to which clients can send HTTPS traffic that will be received by the workstation. Authorized traffic will be received to the workstation as HTTP on port 80. To send traffic to a different port, clients may prefix the host with the destination port in the format `{port}-{host}`. |
| `source_workstation` | String |  | Optional. The source workstation from which this workstation's persistent directories were cloned on creation. |
| `parent` | String | ✅ | Required. Parent resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotations` | HashMap<String, String> | Optional. Client-specified annotations. |
| `start_time` | String | Output only. Time when this workstation was most recently successfully started, regardless of the workstation's initial state. |
| `update_time` | String | Output only. Time when this workstation was most recently updated. |
| `display_name` | String | Optional. Human-readable name for this workstation. |
| `name` | String | Identifier. Full name of this workstation. |
| `state` | String | Output only. Current state of the workstation. |
| `etag` | String | Optional. Checksum computed by the server. May be sent on update and delete requests to make sure that the client has an up-to-date value before proceeding. |
| `runtime_host` | String | Optional. Output only. Runtime host for the workstation when in STATE_RUNNING. |
| `labels` | HashMap<String, String> | Optional. [Labels](https://cloud.google.com/workstations/docs/label-resources) that are applied to the workstation and that are also propagated to the underlying Compute Engine resources. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `kms_key` | String | Output only. The name of the Google Cloud KMS encryption key used to encrypt this workstation. The KMS key can only be configured in the WorkstationConfig. The expected format is `projects/*/locations/*/keyRings/*/cryptoKeys/*`. |
| `uid` | String | Output only. A system-assigned unique identifier for this workstation. |
| `degraded` | bool | Output only. Whether this workstation is in degraded mode, in which case it may require user action to restore full functionality. The conditions field contains detailed information about the status of the workstation. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `boost_configs` | Vec<String> | Output only. List of available boost configuration IDs that this workstation can be boosted up to. |
| `delete_time` | String | Output only. Time when this workstation was soft-deleted. |
| `reconciling` | bool | Output only. Indicates whether this workstation is currently being updated to match its intended state. |
| `create_time` | String | Output only. Time when this workstation was created. |
| `env` | HashMap<String, String> | Optional. Environment variables passed to the workstation container's entrypoint. |
| `conditions` | Vec<String> | Output only. Status conditions describing the workstation's current state. |
| `host` | String | Output only. Host to which clients can send HTTPS traffic that will be received by the workstation. Authorized traffic will be received to the workstation as HTTP on port 80. To send traffic to a different port, clients may prefix the host with the destination port in the format `{port}-{host}`. |
| `source_workstation` | String | Optional. The source workstation from which this workstation's persistent directories were cloned on creation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workstation
workstation = provider.workstations_api.Workstation {
    parent = "value"  # Required. Parent resource name.
}

# Access workstation outputs
workstation_id = workstation.id
workstation_annotations = workstation.annotations
workstation_start_time = workstation.start_time
workstation_update_time = workstation.update_time
workstation_display_name = workstation.display_name
workstation_name = workstation.name
workstation_state = workstation.state
workstation_etag = workstation.etag
workstation_runtime_host = workstation.runtime_host
workstation_labels = workstation.labels
workstation_satisfies_pzi = workstation.satisfies_pzi
workstation_kms_key = workstation.kms_key
workstation_uid = workstation.uid
workstation_degraded = workstation.degraded
workstation_satisfies_pzs = workstation.satisfies_pzs
workstation_boost_configs = workstation.boost_configs
workstation_delete_time = workstation.delete_time
workstation_reconciling = workstation.reconciling
workstation_create_time = workstation.create_time
workstation_env = workstation.env
workstation_conditions = workstation.conditions
workstation_host = workstation.host
workstation_source_workstation = workstation.source_workstation
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple workstation_cluster resources
workstation_cluster_0 = provider.workstations_api.Workstation_cluster {
    parent = "value-0"
}
workstation_cluster_1 = provider.workstations_api.Workstation_cluster {
    parent = "value-1"
}
workstation_cluster_2 = provider.workstations_api.Workstation_cluster {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    workstation_cluster = provider.workstations_api.Workstation_cluster {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Workstations_api Documentation](https://cloud.google.com/workstations_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
