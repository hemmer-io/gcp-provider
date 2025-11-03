# Gkeonprem_api Service



**Resources**: 8

---

## Overview

The gkeonprem_api service provides access to 8 resource types:

- [Vmware_admin_cluster](#vmware_admin_cluster) [CRUD]
- [Bare_metal_admin_cluster](#bare_metal_admin_cluster) [CRUD]
- [Vmware_node_pool](#vmware_node_pool) [CRUD]
- [Bare_metal_node_pool](#bare_metal_node_pool) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Bare_metal_cluster](#bare_metal_cluster) [CRUD]
- [Vmware_cluster](#vmware_cluster) [CRUD]

---

## Resources


### Vmware_admin_cluster

Creates a new VMware admin cluster in a given project and location. The API needs to be combined with creating a bootstrap cluster to work.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status` | String |  | Output only. ResourceStatus representing detailed cluster state. |
| `image_type` | String |  | The OS image type for the VMware admin cluster. |
| `platform_config` | String |  | The VMware platform configuration. |
| `annotations` | HashMap<String, String> |  | Annotations on the VMware admin cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `state` | String |  | Output only. The current state of VMware admin cluster. |
| `private_registry_config` | String |  | Configuration for registry. |
| `uid` | String |  | Output only. The unique identifier of the VMware admin cluster. |
| `prepared_secrets` | String |  | Output only. The VMware admin cluster prepared secrets configuration. It should always be enabled by the Central API, instead of letting users set it. |
| `validation_check` | String |  | Output only. ValidationCheck represents the result of the preflight check job. |
| `addon_node` | String |  | The VMware admin cluster addon node configuration. |
| `network_config` | String |  | The VMware admin cluster network configuration. |
| `endpoint` | String |  | Output only. The DNS name of VMware admin cluster's API server. |
| `load_balancer` | String |  | The VMware admin cluster load balancer configuration. |
| `local_name` | String |  | Output only. The object name of the VMware OnPremAdminCluster custom resource. This field is used to support conflicting names when enrolling existing clusters to the API. When used as a part of cluster enrollment, this field will differ from the ID in the resource name. For new clusters, this field will match the user provided cluster name and be visible in the last component of the resource name. It is not modifiable. All users should use this name to access their cluster using gkectl or kubectl and should expect to see the local name when viewing admin cluster controller logs. |
| `proxy` | String |  | Configuration for proxy. |
| `reconciling` | bool |  | Output only. If set, there are currently changes in flight to the VMware admin cluster. |
| `control_plane_node` | String |  | The VMware admin cluster control plane node configuration. |
| `on_prem_version` | String |  | The Anthos clusters on the VMware version for the admin cluster. |
| `authorization` | String |  | The VMware admin cluster authorization configuration. |
| `create_time` | String |  | Output only. The time at which VMware admin cluster was created. |
| `enable_advanced_cluster` | bool |  | Enable advanced cluster. |
| `bootstrap_cluster_membership` | String |  | The bootstrap cluster this VMware admin cluster belongs to. |
| `name` | String |  | Immutable. The VMware admin cluster resource name. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. Allows clients to perform consistent read-modify-writes through optimistic concurrency control. |
| `anti_affinity_groups` | String |  | The VMware admin cluster anti affinity group configuration. |
| `auto_repair_config` | String |  | The VMware admin cluster auto repair configuration. |
| `update_time` | String |  | Output only. The time at which VMware admin cluster was last updated. |
| `fleet` | String |  | Output only. Fleet configuration for the cluster. |
| `vcenter` | String |  | The VMware admin cluster VCenter configuration. |
| `description` | String |  | A human readable description of this VMware admin cluster. |
| `parent` | String | ✅ | Required. The parent of the project and location where the cluster is created in. Format: "projects/{project}/locations/{location}" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | Output only. ResourceStatus representing detailed cluster state. |
| `image_type` | String | The OS image type for the VMware admin cluster. |
| `platform_config` | String | The VMware platform configuration. |
| `annotations` | HashMap<String, String> | Annotations on the VMware admin cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `state` | String | Output only. The current state of VMware admin cluster. |
| `private_registry_config` | String | Configuration for registry. |
| `uid` | String | Output only. The unique identifier of the VMware admin cluster. |
| `prepared_secrets` | String | Output only. The VMware admin cluster prepared secrets configuration. It should always be enabled by the Central API, instead of letting users set it. |
| `validation_check` | String | Output only. ValidationCheck represents the result of the preflight check job. |
| `addon_node` | String | The VMware admin cluster addon node configuration. |
| `network_config` | String | The VMware admin cluster network configuration. |
| `endpoint` | String | Output only. The DNS name of VMware admin cluster's API server. |
| `load_balancer` | String | The VMware admin cluster load balancer configuration. |
| `local_name` | String | Output only. The object name of the VMware OnPremAdminCluster custom resource. This field is used to support conflicting names when enrolling existing clusters to the API. When used as a part of cluster enrollment, this field will differ from the ID in the resource name. For new clusters, this field will match the user provided cluster name and be visible in the last component of the resource name. It is not modifiable. All users should use this name to access their cluster using gkectl or kubectl and should expect to see the local name when viewing admin cluster controller logs. |
| `proxy` | String | Configuration for proxy. |
| `reconciling` | bool | Output only. If set, there are currently changes in flight to the VMware admin cluster. |
| `control_plane_node` | String | The VMware admin cluster control plane node configuration. |
| `on_prem_version` | String | The Anthos clusters on the VMware version for the admin cluster. |
| `authorization` | String | The VMware admin cluster authorization configuration. |
| `create_time` | String | Output only. The time at which VMware admin cluster was created. |
| `enable_advanced_cluster` | bool | Enable advanced cluster. |
| `bootstrap_cluster_membership` | String | The bootstrap cluster this VMware admin cluster belongs to. |
| `name` | String | Immutable. The VMware admin cluster resource name. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. Allows clients to perform consistent read-modify-writes through optimistic concurrency control. |
| `anti_affinity_groups` | String | The VMware admin cluster anti affinity group configuration. |
| `auto_repair_config` | String | The VMware admin cluster auto repair configuration. |
| `update_time` | String | Output only. The time at which VMware admin cluster was last updated. |
| `fleet` | String | Output only. Fleet configuration for the cluster. |
| `vcenter` | String | The VMware admin cluster VCenter configuration. |
| `description` | String | A human readable description of this VMware admin cluster. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create vmware_admin_cluster
vmware_admin_cluster = provider.gkeonprem_api.Vmware_admin_cluster {
    parent = "value"  # Required. The parent of the project and location where the cluster is created in. Format: "projects/{project}/locations/{location}"
}

# Access vmware_admin_cluster outputs
vmware_admin_cluster_id = vmware_admin_cluster.id
vmware_admin_cluster_status = vmware_admin_cluster.status
vmware_admin_cluster_image_type = vmware_admin_cluster.image_type
vmware_admin_cluster_platform_config = vmware_admin_cluster.platform_config
vmware_admin_cluster_annotations = vmware_admin_cluster.annotations
vmware_admin_cluster_state = vmware_admin_cluster.state
vmware_admin_cluster_private_registry_config = vmware_admin_cluster.private_registry_config
vmware_admin_cluster_uid = vmware_admin_cluster.uid
vmware_admin_cluster_prepared_secrets = vmware_admin_cluster.prepared_secrets
vmware_admin_cluster_validation_check = vmware_admin_cluster.validation_check
vmware_admin_cluster_addon_node = vmware_admin_cluster.addon_node
vmware_admin_cluster_network_config = vmware_admin_cluster.network_config
vmware_admin_cluster_endpoint = vmware_admin_cluster.endpoint
vmware_admin_cluster_load_balancer = vmware_admin_cluster.load_balancer
vmware_admin_cluster_local_name = vmware_admin_cluster.local_name
vmware_admin_cluster_proxy = vmware_admin_cluster.proxy
vmware_admin_cluster_reconciling = vmware_admin_cluster.reconciling
vmware_admin_cluster_control_plane_node = vmware_admin_cluster.control_plane_node
vmware_admin_cluster_on_prem_version = vmware_admin_cluster.on_prem_version
vmware_admin_cluster_authorization = vmware_admin_cluster.authorization
vmware_admin_cluster_create_time = vmware_admin_cluster.create_time
vmware_admin_cluster_enable_advanced_cluster = vmware_admin_cluster.enable_advanced_cluster
vmware_admin_cluster_bootstrap_cluster_membership = vmware_admin_cluster.bootstrap_cluster_membership
vmware_admin_cluster_name = vmware_admin_cluster.name
vmware_admin_cluster_etag = vmware_admin_cluster.etag
vmware_admin_cluster_anti_affinity_groups = vmware_admin_cluster.anti_affinity_groups
vmware_admin_cluster_auto_repair_config = vmware_admin_cluster.auto_repair_config
vmware_admin_cluster_update_time = vmware_admin_cluster.update_time
vmware_admin_cluster_fleet = vmware_admin_cluster.fleet
vmware_admin_cluster_vcenter = vmware_admin_cluster.vcenter
vmware_admin_cluster_description = vmware_admin_cluster.description
```

---


### Bare_metal_admin_cluster

Creates a new bare metal admin cluster in a given project and location. The API needs to be combined with creating a bootstrap cluster to work. See: https://cloud.google.com/anthos/clusters/docs/bare-metal/latest/installing/creating-clusters/create-admin-cluster-api#prepare_bootstrap_environment

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fleet` | String |  | Output only. Fleet configuration for the cluster. |
| `network_config` | String |  | Network configuration. |
| `node_access_config` | String |  | Node access related configurations. |
| `storage` | String |  | Storage configuration. |
| `endpoint` | String |  | Output only. The IP address name of bare metal admin cluster's API server. |
| `reconciling` | bool |  | Output only. If set, there are currently changes in flight to the bare metal Admin Cluster. |
| `validation_check` | String |  | Output only. ValidationCheck representing the result of the preflight check. |
| `local_name` | String |  | Output only. The object name of the bare metal cluster custom resource. This field is used to support conflicting names when enrolling existing clusters to the API. When used as a part of cluster enrollment, this field will differ from the ID in the resource name. For new clusters, this field will match the user provided cluster name and be visible in the last component of the resource name. It is not modifiable. All users should use this name to access their cluster using gkectl or kubectl and should expect to see the local name when viewing admin cluster controller logs. |
| `state` | String |  | Output only. The current state of the bare metal admin cluster. |
| `binary_authorization` | String |  | Binary Authorization related configurations. |
| `status` | String |  | Output only. ResourceStatus representing detailed cluster status. |
| `uid` | String |  | Output only. The unique identifier of the bare metal admin cluster. |
| `maintenance_config` | String |  | Maintenance configuration. |
| `node_config` | String |  | Workload node configuration. |
| `annotations` | HashMap<String, String> |  | Annotations on the bare metal admin cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `bare_metal_version` | String |  | The Anthos clusters on bare metal version for the bare metal admin cluster. |
| `update_time` | String |  | Output only. The time at which this bare metal admin cluster was last updated. |
| `control_plane` | String |  | Control plane configuration. |
| `description` | String |  | A human readable description of this bare metal admin cluster. |
| `cluster_operations` | String |  | Cluster operations configuration. |
| `load_balancer` | String |  | Load balancer configuration. |
| `os_environment_config` | String |  | OS environment related configurations. |
| `security_config` | String |  | Security related configuration. |
| `create_time` | String |  | Output only. The time at which this bare metal admin cluster was created. |
| `name` | String |  | Immutable. The bare metal admin cluster resource name. |
| `delete_time` | String |  | Output only. The time at which this bare metal admin cluster was deleted. If the resource is not deleted, this must be empty |
| `proxy` | String |  | Proxy configuration. |
| `maintenance_status` | String |  | Output only. MaintenanceStatus representing state of maintenance. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. Allows clients to perform consistent read-modify-writes through optimistic concurrency control. |
| `parent` | String | ✅ | Required. The parent of the project and location where the cluster is created in. Format: "projects/{project}/locations/{location}" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fleet` | String | Output only. Fleet configuration for the cluster. |
| `network_config` | String | Network configuration. |
| `node_access_config` | String | Node access related configurations. |
| `storage` | String | Storage configuration. |
| `endpoint` | String | Output only. The IP address name of bare metal admin cluster's API server. |
| `reconciling` | bool | Output only. If set, there are currently changes in flight to the bare metal Admin Cluster. |
| `validation_check` | String | Output only. ValidationCheck representing the result of the preflight check. |
| `local_name` | String | Output only. The object name of the bare metal cluster custom resource. This field is used to support conflicting names when enrolling existing clusters to the API. When used as a part of cluster enrollment, this field will differ from the ID in the resource name. For new clusters, this field will match the user provided cluster name and be visible in the last component of the resource name. It is not modifiable. All users should use this name to access their cluster using gkectl or kubectl and should expect to see the local name when viewing admin cluster controller logs. |
| `state` | String | Output only. The current state of the bare metal admin cluster. |
| `binary_authorization` | String | Binary Authorization related configurations. |
| `status` | String | Output only. ResourceStatus representing detailed cluster status. |
| `uid` | String | Output only. The unique identifier of the bare metal admin cluster. |
| `maintenance_config` | String | Maintenance configuration. |
| `node_config` | String | Workload node configuration. |
| `annotations` | HashMap<String, String> | Annotations on the bare metal admin cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `bare_metal_version` | String | The Anthos clusters on bare metal version for the bare metal admin cluster. |
| `update_time` | String | Output only. The time at which this bare metal admin cluster was last updated. |
| `control_plane` | String | Control plane configuration. |
| `description` | String | A human readable description of this bare metal admin cluster. |
| `cluster_operations` | String | Cluster operations configuration. |
| `load_balancer` | String | Load balancer configuration. |
| `os_environment_config` | String | OS environment related configurations. |
| `security_config` | String | Security related configuration. |
| `create_time` | String | Output only. The time at which this bare metal admin cluster was created. |
| `name` | String | Immutable. The bare metal admin cluster resource name. |
| `delete_time` | String | Output only. The time at which this bare metal admin cluster was deleted. If the resource is not deleted, this must be empty |
| `proxy` | String | Proxy configuration. |
| `maintenance_status` | String | Output only. MaintenanceStatus representing state of maintenance. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. Allows clients to perform consistent read-modify-writes through optimistic concurrency control. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create bare_metal_admin_cluster
bare_metal_admin_cluster = provider.gkeonprem_api.Bare_metal_admin_cluster {
    parent = "value"  # Required. The parent of the project and location where the cluster is created in. Format: "projects/{project}/locations/{location}"
}

# Access bare_metal_admin_cluster outputs
bare_metal_admin_cluster_id = bare_metal_admin_cluster.id
bare_metal_admin_cluster_fleet = bare_metal_admin_cluster.fleet
bare_metal_admin_cluster_network_config = bare_metal_admin_cluster.network_config
bare_metal_admin_cluster_node_access_config = bare_metal_admin_cluster.node_access_config
bare_metal_admin_cluster_storage = bare_metal_admin_cluster.storage
bare_metal_admin_cluster_endpoint = bare_metal_admin_cluster.endpoint
bare_metal_admin_cluster_reconciling = bare_metal_admin_cluster.reconciling
bare_metal_admin_cluster_validation_check = bare_metal_admin_cluster.validation_check
bare_metal_admin_cluster_local_name = bare_metal_admin_cluster.local_name
bare_metal_admin_cluster_state = bare_metal_admin_cluster.state
bare_metal_admin_cluster_binary_authorization = bare_metal_admin_cluster.binary_authorization
bare_metal_admin_cluster_status = bare_metal_admin_cluster.status
bare_metal_admin_cluster_uid = bare_metal_admin_cluster.uid
bare_metal_admin_cluster_maintenance_config = bare_metal_admin_cluster.maintenance_config
bare_metal_admin_cluster_node_config = bare_metal_admin_cluster.node_config
bare_metal_admin_cluster_annotations = bare_metal_admin_cluster.annotations
bare_metal_admin_cluster_bare_metal_version = bare_metal_admin_cluster.bare_metal_version
bare_metal_admin_cluster_update_time = bare_metal_admin_cluster.update_time
bare_metal_admin_cluster_control_plane = bare_metal_admin_cluster.control_plane
bare_metal_admin_cluster_description = bare_metal_admin_cluster.description
bare_metal_admin_cluster_cluster_operations = bare_metal_admin_cluster.cluster_operations
bare_metal_admin_cluster_load_balancer = bare_metal_admin_cluster.load_balancer
bare_metal_admin_cluster_os_environment_config = bare_metal_admin_cluster.os_environment_config
bare_metal_admin_cluster_security_config = bare_metal_admin_cluster.security_config
bare_metal_admin_cluster_create_time = bare_metal_admin_cluster.create_time
bare_metal_admin_cluster_name = bare_metal_admin_cluster.name
bare_metal_admin_cluster_delete_time = bare_metal_admin_cluster.delete_time
bare_metal_admin_cluster_proxy = bare_metal_admin_cluster.proxy
bare_metal_admin_cluster_maintenance_status = bare_metal_admin_cluster.maintenance_status
bare_metal_admin_cluster_etag = bare_metal_admin_cluster.etag
```

---


### Vmware_node_pool

Creates a new VMware node pool in a given project, location and VMWare cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status` | String |  | Output only. ResourceStatus representing the detailed VMware node pool state. |
| `update_time` | String |  | Output only. The time at which this node pool was last updated. |
| `uid` | String |  | Output only. The unique identifier of the node pool. |
| `name` | String |  | Immutable. The resource name of this node pool. |
| `create_time` | String |  | Output only. The time at which this node pool was created. |
| `config` | String |  | Required. The node configuration of the node pool. |
| `annotations` | HashMap<String, String> |  | Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `delete_time` | String |  | Output only. The time at which this node pool was deleted. If the resource is not deleted, this must be empty |
| `display_name` | String |  | The display name for the node pool. |
| `node_pool_autoscaling` | String |  | Node pool autoscaling config for the node pool. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. Allows clients to perform consistent read-modify-writes through optimistic concurrency control. |
| `on_prem_version` | String |  | Anthos version for the node pool. Defaults to the user cluster version. |
| `state` | String |  | Output only. The current state of the node pool. |
| `reconciling` | bool |  | Output only. If set, there are currently changes in flight to the node pool. |
| `parent` | String | ✅ | Required. The parent resource where this node pool will be created. projects/{project}/locations/{location}/vmwareClusters/{cluster} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | Output only. ResourceStatus representing the detailed VMware node pool state. |
| `update_time` | String | Output only. The time at which this node pool was last updated. |
| `uid` | String | Output only. The unique identifier of the node pool. |
| `name` | String | Immutable. The resource name of this node pool. |
| `create_time` | String | Output only. The time at which this node pool was created. |
| `config` | String | Required. The node configuration of the node pool. |
| `annotations` | HashMap<String, String> | Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `delete_time` | String | Output only. The time at which this node pool was deleted. If the resource is not deleted, this must be empty |
| `display_name` | String | The display name for the node pool. |
| `node_pool_autoscaling` | String | Node pool autoscaling config for the node pool. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. Allows clients to perform consistent read-modify-writes through optimistic concurrency control. |
| `on_prem_version` | String | Anthos version for the node pool. Defaults to the user cluster version. |
| `state` | String | Output only. The current state of the node pool. |
| `reconciling` | bool | Output only. If set, there are currently changes in flight to the node pool. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create vmware_node_pool
vmware_node_pool = provider.gkeonprem_api.Vmware_node_pool {
    parent = "value"  # Required. The parent resource where this node pool will be created. projects/{project}/locations/{location}/vmwareClusters/{cluster}
}

# Access vmware_node_pool outputs
vmware_node_pool_id = vmware_node_pool.id
vmware_node_pool_status = vmware_node_pool.status
vmware_node_pool_update_time = vmware_node_pool.update_time
vmware_node_pool_uid = vmware_node_pool.uid
vmware_node_pool_name = vmware_node_pool.name
vmware_node_pool_create_time = vmware_node_pool.create_time
vmware_node_pool_config = vmware_node_pool.config
vmware_node_pool_annotations = vmware_node_pool.annotations
vmware_node_pool_delete_time = vmware_node_pool.delete_time
vmware_node_pool_display_name = vmware_node_pool.display_name
vmware_node_pool_node_pool_autoscaling = vmware_node_pool.node_pool_autoscaling
vmware_node_pool_etag = vmware_node_pool.etag
vmware_node_pool_on_prem_version = vmware_node_pool.on_prem_version
vmware_node_pool_state = vmware_node_pool.state
vmware_node_pool_reconciling = vmware_node_pool.reconciling
```

---


### Bare_metal_node_pool

Creates a new bare metal node pool in a given project, location and Bare Metal cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The time at which this bare metal node pool was last updated. |
| `upgrade_policy` | String |  | The worker node pool upgrade policy. |
| `delete_time` | String |  | Output only. The time at which this bare metal node pool was deleted. If the resource is not deleted, this must be empty |
| `uid` | String |  | Output only. The unique identifier of the bare metal node pool. |
| `create_time` | String |  | Output only. The time at which this bare metal node pool was created. |
| `display_name` | String |  | The display name for the bare metal node pool. |
| `name` | String |  | Immutable. The bare metal node pool resource name. |
| `node_pool_config` | String |  | Required. Node pool configuration. |
| `annotations` | HashMap<String, String> |  | Annotations on the bare metal node pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `reconciling` | bool |  | Output only. If set, there are currently changes in flight to the bare metal node pool. |
| `state` | String |  | Output only. The current state of the bare metal node pool. |
| `status` | String |  | Output only. ResourceStatus representing the detailed node pool status. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. Allows clients to perform consistent read-modify-writes through optimistic concurrency control. |
| `parent` | String | ✅ | Required. The parent resource where this node pool will be created. projects/{project}/locations/{location}/bareMetalClusters/{cluster} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The time at which this bare metal node pool was last updated. |
| `upgrade_policy` | String | The worker node pool upgrade policy. |
| `delete_time` | String | Output only. The time at which this bare metal node pool was deleted. If the resource is not deleted, this must be empty |
| `uid` | String | Output only. The unique identifier of the bare metal node pool. |
| `create_time` | String | Output only. The time at which this bare metal node pool was created. |
| `display_name` | String | The display name for the bare metal node pool. |
| `name` | String | Immutable. The bare metal node pool resource name. |
| `node_pool_config` | String | Required. Node pool configuration. |
| `annotations` | HashMap<String, String> | Annotations on the bare metal node pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `reconciling` | bool | Output only. If set, there are currently changes in flight to the bare metal node pool. |
| `state` | String | Output only. The current state of the bare metal node pool. |
| `status` | String | Output only. ResourceStatus representing the detailed node pool status. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. Allows clients to perform consistent read-modify-writes through optimistic concurrency control. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create bare_metal_node_pool
bare_metal_node_pool = provider.gkeonprem_api.Bare_metal_node_pool {
    parent = "value"  # Required. The parent resource where this node pool will be created. projects/{project}/locations/{location}/bareMetalClusters/{cluster}
}

# Access bare_metal_node_pool outputs
bare_metal_node_pool_id = bare_metal_node_pool.id
bare_metal_node_pool_update_time = bare_metal_node_pool.update_time
bare_metal_node_pool_upgrade_policy = bare_metal_node_pool.upgrade_policy
bare_metal_node_pool_delete_time = bare_metal_node_pool.delete_time
bare_metal_node_pool_uid = bare_metal_node_pool.uid
bare_metal_node_pool_create_time = bare_metal_node_pool.create_time
bare_metal_node_pool_display_name = bare_metal_node_pool.display_name
bare_metal_node_pool_name = bare_metal_node_pool.name
bare_metal_node_pool_node_pool_config = bare_metal_node_pool.node_pool_config
bare_metal_node_pool_annotations = bare_metal_node_pool.annotations
bare_metal_node_pool_reconciling = bare_metal_node_pool.reconciling
bare_metal_node_pool_state = bare_metal_node_pool.state
bare_metal_node_pool_status = bare_metal_node_pool.status
bare_metal_node_pool_etag = bare_metal_node_pool.etag
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.gkeonprem_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_error = operation.error
operation_done = operation.done
operation_response = operation.response
operation_name = operation.name
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
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
location_display_name = location.display_name
location_metadata = location.metadata
location_name = location.name
location_labels = location.labels
location_location_id = location.location_id
```

---


### Bare_metal_cluster

Creates a new bare metal cluster in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `admin_cluster_membership` | String |  | Required. The admin cluster this bare metal user cluster belongs to. This is the full resource name of the admin cluster's fleet membership. |
| `bare_metal_version` | String |  | Required. The Anthos clusters on bare metal version for your user cluster. |
| `node_access_config` | String |  | Node access related configurations. |
| `node_config` | String |  | Workload node configuration. |
| `fleet` | String |  | Output only. Fleet configuration for the cluster. |
| `annotations` | HashMap<String, String> |  | Annotations on the bare metal user cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `maintenance_config` | String |  | Maintenance configuration. |
| `state` | String |  | Output only. The current state of the bare metal user cluster. |
| `cluster_operations` | String |  | Cluster operations configuration. |
| `endpoint` | String |  | Output only. The IP address of the bare metal user cluster's API server. |
| `storage` | String |  | Required. Storage configuration. |
| `load_balancer` | String |  | Required. Load balancer configuration. |
| `description` | String |  | A human readable description of this bare metal user cluster. |
| `local_namespace` | String |  | Output only. The namespace of the cluster. |
| `create_time` | String |  | Output only. The time when the bare metal user cluster was created. |
| `security_config` | String |  | Security related setting configuration. |
| `os_environment_config` | String |  | OS environment related configurations. |
| `status` | String |  | Output only. Detailed cluster status. |
| `binary_authorization` | String |  | Binary Authorization related configurations. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. Allows clients to perform consistent read-modify-writes through optimistic concurrency control. |
| `network_config` | String |  | Required. Network configuration. |
| `uid` | String |  | Output only. The unique identifier of the bare metal user cluster. |
| `control_plane` | String |  | Required. Control plane configuration. |
| `local_name` | String |  | Output only. The object name of the bare metal user cluster custom resource on the associated admin cluster. This field is used to support conflicting names when enrolling existing clusters to the API. When used as a part of cluster enrollment, this field will differ from the name in the resource name. For new clusters, this field will match the user provided cluster name and be visible in the last component of the resource name. It is not modifiable. When the local name and cluster name differ, the local name is used in the admin cluster controller logs. You use the cluster name when accessing the cluster using bmctl and kubectl. |
| `update_time` | String |  | Output only. The time when the bare metal user cluster was last updated. |
| `name` | String |  | Immutable. The bare metal user cluster resource name. |
| `upgrade_policy` | String |  | The cluster upgrade policy. |
| `proxy` | String |  | Proxy configuration. |
| `admin_cluster_name` | String |  | Output only. The resource name of the bare metal admin cluster managing this user cluster. |
| `reconciling` | bool |  | Output only. If set, there are currently changes in flight to the bare metal user cluster. |
| `validation_check` | String |  | Output only. The result of the preflight check. |
| `maintenance_status` | String |  | Output only. Status of on-going maintenance tasks. |
| `delete_time` | String |  | Output only. The time when the bare metal user cluster was deleted. If the resource is not deleted, this must be empty |
| `parent` | String | ✅ | Required. The parent of the project and location where the cluster is created in. Format: "projects/{project}/locations/{location}" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `admin_cluster_membership` | String | Required. The admin cluster this bare metal user cluster belongs to. This is the full resource name of the admin cluster's fleet membership. |
| `bare_metal_version` | String | Required. The Anthos clusters on bare metal version for your user cluster. |
| `node_access_config` | String | Node access related configurations. |
| `node_config` | String | Workload node configuration. |
| `fleet` | String | Output only. Fleet configuration for the cluster. |
| `annotations` | HashMap<String, String> | Annotations on the bare metal user cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `maintenance_config` | String | Maintenance configuration. |
| `state` | String | Output only. The current state of the bare metal user cluster. |
| `cluster_operations` | String | Cluster operations configuration. |
| `endpoint` | String | Output only. The IP address of the bare metal user cluster's API server. |
| `storage` | String | Required. Storage configuration. |
| `load_balancer` | String | Required. Load balancer configuration. |
| `description` | String | A human readable description of this bare metal user cluster. |
| `local_namespace` | String | Output only. The namespace of the cluster. |
| `create_time` | String | Output only. The time when the bare metal user cluster was created. |
| `security_config` | String | Security related setting configuration. |
| `os_environment_config` | String | OS environment related configurations. |
| `status` | String | Output only. Detailed cluster status. |
| `binary_authorization` | String | Binary Authorization related configurations. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. Allows clients to perform consistent read-modify-writes through optimistic concurrency control. |
| `network_config` | String | Required. Network configuration. |
| `uid` | String | Output only. The unique identifier of the bare metal user cluster. |
| `control_plane` | String | Required. Control plane configuration. |
| `local_name` | String | Output only. The object name of the bare metal user cluster custom resource on the associated admin cluster. This field is used to support conflicting names when enrolling existing clusters to the API. When used as a part of cluster enrollment, this field will differ from the name in the resource name. For new clusters, this field will match the user provided cluster name and be visible in the last component of the resource name. It is not modifiable. When the local name and cluster name differ, the local name is used in the admin cluster controller logs. You use the cluster name when accessing the cluster using bmctl and kubectl. |
| `update_time` | String | Output only. The time when the bare metal user cluster was last updated. |
| `name` | String | Immutable. The bare metal user cluster resource name. |
| `upgrade_policy` | String | The cluster upgrade policy. |
| `proxy` | String | Proxy configuration. |
| `admin_cluster_name` | String | Output only. The resource name of the bare metal admin cluster managing this user cluster. |
| `reconciling` | bool | Output only. If set, there are currently changes in flight to the bare metal user cluster. |
| `validation_check` | String | Output only. The result of the preflight check. |
| `maintenance_status` | String | Output only. Status of on-going maintenance tasks. |
| `delete_time` | String | Output only. The time when the bare metal user cluster was deleted. If the resource is not deleted, this must be empty |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create bare_metal_cluster
bare_metal_cluster = provider.gkeonprem_api.Bare_metal_cluster {
    parent = "value"  # Required. The parent of the project and location where the cluster is created in. Format: "projects/{project}/locations/{location}"
}

# Access bare_metal_cluster outputs
bare_metal_cluster_id = bare_metal_cluster.id
bare_metal_cluster_admin_cluster_membership = bare_metal_cluster.admin_cluster_membership
bare_metal_cluster_bare_metal_version = bare_metal_cluster.bare_metal_version
bare_metal_cluster_node_access_config = bare_metal_cluster.node_access_config
bare_metal_cluster_node_config = bare_metal_cluster.node_config
bare_metal_cluster_fleet = bare_metal_cluster.fleet
bare_metal_cluster_annotations = bare_metal_cluster.annotations
bare_metal_cluster_maintenance_config = bare_metal_cluster.maintenance_config
bare_metal_cluster_state = bare_metal_cluster.state
bare_metal_cluster_cluster_operations = bare_metal_cluster.cluster_operations
bare_metal_cluster_endpoint = bare_metal_cluster.endpoint
bare_metal_cluster_storage = bare_metal_cluster.storage
bare_metal_cluster_load_balancer = bare_metal_cluster.load_balancer
bare_metal_cluster_description = bare_metal_cluster.description
bare_metal_cluster_local_namespace = bare_metal_cluster.local_namespace
bare_metal_cluster_create_time = bare_metal_cluster.create_time
bare_metal_cluster_security_config = bare_metal_cluster.security_config
bare_metal_cluster_os_environment_config = bare_metal_cluster.os_environment_config
bare_metal_cluster_status = bare_metal_cluster.status
bare_metal_cluster_binary_authorization = bare_metal_cluster.binary_authorization
bare_metal_cluster_etag = bare_metal_cluster.etag
bare_metal_cluster_network_config = bare_metal_cluster.network_config
bare_metal_cluster_uid = bare_metal_cluster.uid
bare_metal_cluster_control_plane = bare_metal_cluster.control_plane
bare_metal_cluster_local_name = bare_metal_cluster.local_name
bare_metal_cluster_update_time = bare_metal_cluster.update_time
bare_metal_cluster_name = bare_metal_cluster.name
bare_metal_cluster_upgrade_policy = bare_metal_cluster.upgrade_policy
bare_metal_cluster_proxy = bare_metal_cluster.proxy
bare_metal_cluster_admin_cluster_name = bare_metal_cluster.admin_cluster_name
bare_metal_cluster_reconciling = bare_metal_cluster.reconciling
bare_metal_cluster_validation_check = bare_metal_cluster.validation_check
bare_metal_cluster_maintenance_status = bare_metal_cluster.maintenance_status
bare_metal_cluster_delete_time = bare_metal_cluster.delete_time
```

---


### Vmware_cluster

Creates a new VMware user cluster in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vm_tracking_enabled` | bool |  | Enable VM tracking. |
| `upgrade_policy` | String |  | Specifies upgrade policy for the cluster. |
| `validation_check` | String |  | Output only. ValidationCheck represents the result of the preflight check job. |
| `admin_cluster_membership` | String |  | Required. The admin cluster this VMware user cluster belongs to. This is the full resource name of the admin cluster's fleet membership. In the future, references to other resource types might be allowed if admin clusters are modeled as their own resources. |
| `vcenter` | String |  | VmwareVCenterConfig specifies vCenter config for the user cluster. If unspecified, it is inherited from the admin cluster. |
| `uid` | String |  | Output only. The unique identifier of the VMware user cluster. |
| `auto_repair_config` | String |  | Configuration for auto repairing. |
| `reconciling` | bool |  | Output only. If set, there are currently changes in flight to the VMware user cluster. |
| `disable_bundled_ingress` | bool |  | Disable bundled ingress. |
| `dataplane_v2` | String |  | VmwareDataplaneV2Config specifies configuration for Dataplane V2. |
| `anti_affinity_groups` | String |  | AAGConfig specifies whether to spread VMware user cluster nodes across at least three physical hosts in the datacenter. |
| `on_prem_version` | String |  | Required. The Anthos clusters on the VMware version for your user cluster. |
| `admin_cluster_name` | String |  | Output only. The resource name of the VMware admin cluster hosting this user cluster. |
| `load_balancer` | String |  | Load balancer configuration. |
| `authorization` | String |  | RBAC policy that will be applied and managed by the Anthos On-Prem API. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. Allows clients to perform consistent read-modify-writes through optimistic concurrency control. |
| `fleet` | String |  | Output only. Fleet configuration for the cluster. |
| `local_name` | String |  | Output only. The object name of the VMware OnPremUserCluster custom resource on the associated admin cluster. This field is used to support conflicting names when enrolling existing clusters to the API. When used as a part of cluster enrollment, this field will differ from the ID in the resource name. For new clusters, this field will match the user provided cluster name and be visible in the last component of the resource name. It is not modifiable. All users should use this name to access their cluster using gkectl or kubectl and should expect to see the local name when viewing admin cluster controller logs. |
| `storage` | String |  | Storage configuration. |
| `status` | String |  | Output only. ResourceStatus representing detailed cluster state. |
| `enable_advanced_cluster` | bool |  | Enable advanced cluster. |
| `binary_authorization` | String |  | Binary Authorization related configurations. |
| `enable_control_plane_v2` | bool |  | Enable control plane V2. Default to false. |
| `endpoint` | String |  | Output only. The DNS name of VMware user cluster's API server. |
| `name` | String |  | Immutable. The VMware user cluster resource name. |
| `state` | String |  | Output only. The current state of VMware user cluster. |
| `description` | String |  | A human readable description of this VMware user cluster. |
| `create_time` | String |  | Output only. The time at which VMware user cluster was created. |
| `control_plane_node` | String |  | VMware user cluster control plane nodes must have either 1 or 3 replicas. |
| `delete_time` | String |  | Output only. The time at which VMware user cluster was deleted. |
| `update_time` | String |  | Output only. The time at which VMware user cluster was last updated. |
| `network_config` | String |  | The VMware user cluster network configuration. |
| `annotations` | HashMap<String, String> |  | Annotations on the VMware user cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `parent` | String | ✅ | Required. The parent of the project and location where this cluster is created in. Format: "projects/{project}/locations/{location}" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vm_tracking_enabled` | bool | Enable VM tracking. |
| `upgrade_policy` | String | Specifies upgrade policy for the cluster. |
| `validation_check` | String | Output only. ValidationCheck represents the result of the preflight check job. |
| `admin_cluster_membership` | String | Required. The admin cluster this VMware user cluster belongs to. This is the full resource name of the admin cluster's fleet membership. In the future, references to other resource types might be allowed if admin clusters are modeled as their own resources. |
| `vcenter` | String | VmwareVCenterConfig specifies vCenter config for the user cluster. If unspecified, it is inherited from the admin cluster. |
| `uid` | String | Output only. The unique identifier of the VMware user cluster. |
| `auto_repair_config` | String | Configuration for auto repairing. |
| `reconciling` | bool | Output only. If set, there are currently changes in flight to the VMware user cluster. |
| `disable_bundled_ingress` | bool | Disable bundled ingress. |
| `dataplane_v2` | String | VmwareDataplaneV2Config specifies configuration for Dataplane V2. |
| `anti_affinity_groups` | String | AAGConfig specifies whether to spread VMware user cluster nodes across at least three physical hosts in the datacenter. |
| `on_prem_version` | String | Required. The Anthos clusters on the VMware version for your user cluster. |
| `admin_cluster_name` | String | Output only. The resource name of the VMware admin cluster hosting this user cluster. |
| `load_balancer` | String | Load balancer configuration. |
| `authorization` | String | RBAC policy that will be applied and managed by the Anthos On-Prem API. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. Allows clients to perform consistent read-modify-writes through optimistic concurrency control. |
| `fleet` | String | Output only. Fleet configuration for the cluster. |
| `local_name` | String | Output only. The object name of the VMware OnPremUserCluster custom resource on the associated admin cluster. This field is used to support conflicting names when enrolling existing clusters to the API. When used as a part of cluster enrollment, this field will differ from the ID in the resource name. For new clusters, this field will match the user provided cluster name and be visible in the last component of the resource name. It is not modifiable. All users should use this name to access their cluster using gkectl or kubectl and should expect to see the local name when viewing admin cluster controller logs. |
| `storage` | String | Storage configuration. |
| `status` | String | Output only. ResourceStatus representing detailed cluster state. |
| `enable_advanced_cluster` | bool | Enable advanced cluster. |
| `binary_authorization` | String | Binary Authorization related configurations. |
| `enable_control_plane_v2` | bool | Enable control plane V2. Default to false. |
| `endpoint` | String | Output only. The DNS name of VMware user cluster's API server. |
| `name` | String | Immutable. The VMware user cluster resource name. |
| `state` | String | Output only. The current state of VMware user cluster. |
| `description` | String | A human readable description of this VMware user cluster. |
| `create_time` | String | Output only. The time at which VMware user cluster was created. |
| `control_plane_node` | String | VMware user cluster control plane nodes must have either 1 or 3 replicas. |
| `delete_time` | String | Output only. The time at which VMware user cluster was deleted. |
| `update_time` | String | Output only. The time at which VMware user cluster was last updated. |
| `network_config` | String | The VMware user cluster network configuration. |
| `annotations` | HashMap<String, String> | Annotations on the VMware user cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create vmware_cluster
vmware_cluster = provider.gkeonprem_api.Vmware_cluster {
    parent = "value"  # Required. The parent of the project and location where this cluster is created in. Format: "projects/{project}/locations/{location}"
}

# Access vmware_cluster outputs
vmware_cluster_id = vmware_cluster.id
vmware_cluster_vm_tracking_enabled = vmware_cluster.vm_tracking_enabled
vmware_cluster_upgrade_policy = vmware_cluster.upgrade_policy
vmware_cluster_validation_check = vmware_cluster.validation_check
vmware_cluster_admin_cluster_membership = vmware_cluster.admin_cluster_membership
vmware_cluster_vcenter = vmware_cluster.vcenter
vmware_cluster_uid = vmware_cluster.uid
vmware_cluster_auto_repair_config = vmware_cluster.auto_repair_config
vmware_cluster_reconciling = vmware_cluster.reconciling
vmware_cluster_disable_bundled_ingress = vmware_cluster.disable_bundled_ingress
vmware_cluster_dataplane_v2 = vmware_cluster.dataplane_v2
vmware_cluster_anti_affinity_groups = vmware_cluster.anti_affinity_groups
vmware_cluster_on_prem_version = vmware_cluster.on_prem_version
vmware_cluster_admin_cluster_name = vmware_cluster.admin_cluster_name
vmware_cluster_load_balancer = vmware_cluster.load_balancer
vmware_cluster_authorization = vmware_cluster.authorization
vmware_cluster_etag = vmware_cluster.etag
vmware_cluster_fleet = vmware_cluster.fleet
vmware_cluster_local_name = vmware_cluster.local_name
vmware_cluster_storage = vmware_cluster.storage
vmware_cluster_status = vmware_cluster.status
vmware_cluster_enable_advanced_cluster = vmware_cluster.enable_advanced_cluster
vmware_cluster_binary_authorization = vmware_cluster.binary_authorization
vmware_cluster_enable_control_plane_v2 = vmware_cluster.enable_control_plane_v2
vmware_cluster_endpoint = vmware_cluster.endpoint
vmware_cluster_name = vmware_cluster.name
vmware_cluster_state = vmware_cluster.state
vmware_cluster_description = vmware_cluster.description
vmware_cluster_create_time = vmware_cluster.create_time
vmware_cluster_control_plane_node = vmware_cluster.control_plane_node
vmware_cluster_delete_time = vmware_cluster.delete_time
vmware_cluster_update_time = vmware_cluster.update_time
vmware_cluster_network_config = vmware_cluster.network_config
vmware_cluster_annotations = vmware_cluster.annotations
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple vmware_admin_cluster resources
vmware_admin_cluster_0 = provider.gkeonprem_api.Vmware_admin_cluster {
    parent = "value-0"
}
vmware_admin_cluster_1 = provider.gkeonprem_api.Vmware_admin_cluster {
    parent = "value-1"
}
vmware_admin_cluster_2 = provider.gkeonprem_api.Vmware_admin_cluster {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    vmware_admin_cluster = provider.gkeonprem_api.Vmware_admin_cluster {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Gkeonprem_api Documentation](https://cloud.google.com/gkeonprem_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
