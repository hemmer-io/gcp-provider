# Container_api Service



**Resources**: 14

---

## Overview

The container_api service provides access to 14 resource types:

- [Operation](#operation) [CR]
- [Zone](#zone) [R]
- [Usable_subnetwork](#usable_subnetwork) [R]
- [Location](#location) [R]
- [Node_pool](#node_pool) [CRUD]
- [Cluster](#cluster) [CRUD]
- [Well_known](#well_known) [R]
- [Usable_subnetwork](#usable_subnetwork) [R]
- [Zone](#zone) [R]
- [Location](#location) [R]
- [Well_known](#well_known) [R]
- [Operation](#operation) [CR]
- [Cluster](#cluster) [CRUD]
- [Node_pool](#node_pool) [CRUD]

---

## Resources


### Operation

Cancels the specified operation.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `zone` | String |  | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation resides. This field has been deprecated and replaced by the name field. |
| `operation_id` | String |  | Deprecated. The server-assigned `name` of the operation. This field has been deprecated and replaced by the name field. |
| `project_id` | String |  | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field. |
| `name` | String |  | The name (project, location, operation id) of the operation to cancel. Specified in the format `projects/*/locations/*/operations/*`. |
| `name` | String | ✅ | The name (project, location, operation id) of the operation to cancel. Specified in the format `projects/*/locations/*/operations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The server-assigned ID for the operation. |
| `operation_type` | String | Output only. The operation type. |
| `cluster_conditions` | Vec<String> | Which conditions caused the current cluster state. Deprecated. Use field error instead. |
| `progress` | String | Output only. Progress information for an operation. |
| `start_time` | String | Output only. The time the operation started, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `status` | String | Output only. The current status of the operation. |
| `zone` | String | Output only. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation is taking place. This field is deprecated, use location instead. |
| `status_message` | String | Output only. If an error has occurred, a textual description of the error. Deprecated. Use the field error instead. |
| `target_link` | String | Output only. Server-defined URI for the target of the operation. The format of this is a URI to the resource being modified (such as a cluster, node pool, or node). For node pool repairs, there may be multiple nodes being repaired, but only one will be the target. Examples: - ## `https://container.googleapis.com/v1/projects/123/locations/us-central1/clusters/my-cluster` ## `https://container.googleapis.com/v1/projects/123/zones/us-central1-c/clusters/my-cluster/nodePools/my-np` `https://container.googleapis.com/v1/projects/123/zones/us-central1-c/clusters/my-cluster/nodePools/my-np/node/my-node` |
| `detail` | String | Output only. Detailed operation progress, if available. |
| `location` | String | Output only. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in which the cluster resides. |
| `nodepool_conditions` | Vec<String> | Which conditions caused the current node pool state. Deprecated. Use field error instead. |
| `error` | String | The error result of the operation in case of failure. |
| `self_link` | String | Output only. Server-defined URI for the operation. Example: `https://container.googleapis.com/v1alpha1/projects/123/locations/us-central1/operations/operation-123`. |
| `end_time` | String | Output only. The time the operation completed, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.container_api.Operation {
    name = "value"  # The name (project, location, operation id) of the operation to cancel. Specified in the format `projects/*/locations/*/operations/*`.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_operation_type = operation.operation_type
operation_cluster_conditions = operation.cluster_conditions
operation_progress = operation.progress
operation_start_time = operation.start_time
operation_status = operation.status
operation_zone = operation.zone
operation_status_message = operation.status_message
operation_target_link = operation.target_link
operation_detail = operation.detail
operation_location = operation.location
operation_nodepool_conditions = operation.nodepool_conditions
operation_error = operation.error
operation_self_link = operation.self_link
operation_end_time = operation.end_time
```

---


### Zone

Returns configuration info about the Google Kubernetes Engine service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channels` | Vec<String> | List of release channel configurations. |
| `valid_master_versions` | Vec<String> | List of valid master versions, in descending order. |
| `default_cluster_version` | String | Version of Kubernetes the service deploys by default. |
| `default_image_type` | String | Default image type. |
| `valid_image_types` | Vec<String> | List of valid image types. |
| `valid_node_versions` | Vec<String> | List of valid node upgrade target versions, in descending order. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access zone outputs
zone_id = zone.id
zone_channels = zone.channels
zone_valid_master_versions = zone.valid_master_versions
zone_default_cluster_version = zone.default_cluster_version
zone_default_image_type = zone.default_image_type
zone_valid_image_types = zone.valid_image_types
zone_valid_node_versions = zone.valid_node_versions
```

---


### Usable_subnetwork

Lists subnetworks that are usable for creating clusters in a project.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subnetworks` | Vec<String> | A list of usable subnetworks in the specified network project. |
| `next_page_token` | String | This token allows you to get the next page of results for list requests. If the number of results is larger than `page_size`, use the `next_page_token` as a value for the query parameter `page_token` in the next request. The value will become empty when there are no more pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access usable_subnetwork outputs
usable_subnetwork_id = usable_subnetwork.id
usable_subnetwork_subnetworks = usable_subnetwork.subnetworks
usable_subnetwork_next_page_token = usable_subnetwork.next_page_token
```

---


### Location

Returns configuration info about the Google Kubernetes Engine service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channels` | Vec<String> | List of release channel configurations. |
| `valid_master_versions` | Vec<String> | List of valid master versions, in descending order. |
| `default_cluster_version` | String | Version of Kubernetes the service deploys by default. |
| `default_image_type` | String | Default image type. |
| `valid_image_types` | Vec<String> | List of valid image types. |
| `valid_node_versions` | Vec<String> | List of valid node upgrade target versions, in descending order. |


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
location_channels = location.channels
location_valid_master_versions = location.valid_master_versions
location_default_cluster_version = location.default_cluster_version
location_default_image_type = location.default_image_type
location_valid_image_types = location.valid_image_types
location_valid_node_versions = location.valid_node_versions
```

---


### Node_pool

Creates a node pool for a cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `zone` | String |  | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field. |
| `project_id` | String |  | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field. |
| `parent` | String |  | The parent (project, location, cluster name) where the node pool will be created. Specified in the format `projects/*/locations/*/clusters/*`. |
| `node_pool` | String |  | Required. The node pool to create. |
| `cluster_id` | String |  | Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field. |
| `cluster_id` | String | ✅ | Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field. |
| `project_id` | String | ✅ | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field. |
| `zone` | String | ✅ | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `self_link` | String | Output only. Server-defined URL for the resource. |
| `update_info` | String | Output only. Update info contains relevant information during a node pool update. |
| `best_effort_provisioning` | String | Enable best effort provisioning for nodes |
| `pod_ipv4_cidr_size` | i64 | Output only. The pod CIDR block size per node in this node pool. |
| `version` | String | The version of Kubernetes running on this NodePool's nodes. If unspecified, it defaults as described [here](https://cloud.google.com/kubernetes-engine/versioning#specifying_node_version). |
| `status` | String | Output only. The status of the nodes in this pool instance. |
| `management` | String | NodeManagement configuration for this NodePool. |
| `placement_policy` | String | Specifies the node placement policy. |
| `locations` | Vec<String> | The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the NodePool's nodes should be located. If this value is unspecified during node pool creation, the [Cluster.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters#Cluster.FIELDS.locations) value will be used, instead. Warning: changing node pool locations will result in nodes being added and/or removed. |
| `status_message` | String | Output only. Deprecated. Use conditions instead. Additional information about the current status of this node pool instance, if available. |
| `queued_provisioning` | String | Specifies the configuration of queued provisioning. |
| `upgrade_settings` | String | Upgrade settings control disruption and speed of the upgrade. |
| `autopilot_config` | String | Specifies the autopilot configuration for this node pool. This field is exclusively reserved for Cluster Autoscaler. |
| `config` | String | The node configuration of the pool. |
| `instance_group_urls` | Vec<String> | Output only. The resource URLs of the [managed instance groups](https://cloud.google.com/compute/docs/instance-groups/creating-groups-of-managed-instances) associated with this node pool. During the node pool blue-green upgrade operation, the URLs contain both blue and green resources. |
| `network_config` | String | Networking configuration for this NodePool. If specified, it overrides the cluster-level defaults. |
| `etag` | String | This checksum is computed by the server based on the value of node pool fields, and may be sent on update requests to ensure the client has an up-to-date value before proceeding. |
| `max_pods_constraint` | String | The constraint on the maximum number of pods that can be run simultaneously on a node in the node pool. |
| `initial_node_count` | i64 | The initial node count for the pool. You must ensure that your Compute Engine [resource quota](https://cloud.google.com/compute/quotas) is sufficient for this number of instances. You must also have available firewall and routes quota. |
| `autoscaling` | String | Autoscaler configuration for this NodePool. Autoscaler is enabled only if a valid configuration is present. |
| `conditions` | Vec<String> | Which conditions caused the current node pool state. |
| `name` | String | The name of the node pool. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create node_pool
node_pool = provider.container_api.Node_pool {
    cluster_id = "value"  # Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field.
    project_id = "value"  # Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field.
    zone = "value"  # Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field.
}

# Access node_pool outputs
node_pool_id = node_pool.id
node_pool_self_link = node_pool.self_link
node_pool_update_info = node_pool.update_info
node_pool_best_effort_provisioning = node_pool.best_effort_provisioning
node_pool_pod_ipv4_cidr_size = node_pool.pod_ipv4_cidr_size
node_pool_version = node_pool.version
node_pool_status = node_pool.status
node_pool_management = node_pool.management
node_pool_placement_policy = node_pool.placement_policy
node_pool_locations = node_pool.locations
node_pool_status_message = node_pool.status_message
node_pool_queued_provisioning = node_pool.queued_provisioning
node_pool_upgrade_settings = node_pool.upgrade_settings
node_pool_autopilot_config = node_pool.autopilot_config
node_pool_config = node_pool.config
node_pool_instance_group_urls = node_pool.instance_group_urls
node_pool_network_config = node_pool.network_config
node_pool_etag = node_pool.etag
node_pool_max_pods_constraint = node_pool.max_pods_constraint
node_pool_initial_node_count = node_pool.initial_node_count
node_pool_autoscaling = node_pool.autoscaling
node_pool_conditions = node_pool.conditions
node_pool_name = node_pool.name
```

---


### Cluster

Creates a cluster, consisting of the specified number and type of Google Compute Engine instances. By default, the cluster is created in the project's [default network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks). One firewall is added for the cluster. After cluster creation, the kubelet creates routes for each node to allow the containers on that node to communicate with all other instances in the cluster. Finally, an entry is added to the project's global metadata indicating which CIDR range the cluster is using.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project_id` | String |  | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field. |
| `cluster` | String |  | Required. A [cluster resource](https://cloud.google.com/container-engine/reference/rest/v1/projects.locations.clusters) |
| `parent` | String |  | The parent (project and location) where the cluster will be created. Specified in the format `projects/*/locations/*`. |
| `zone` | String |  | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field. |
| `zone` | String | ✅ | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field. |
| `project_id` | String | ✅ | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fleet` | String | Fleet information for the cluster. |
| `node_pool_defaults` | String | Default NodePool settings for the entire cluster. These settings are overridden if specified on the specific NodePool object. |
| `subnetwork` | String | The name of the Google Compute Engine [subnetwork](https://cloud.google.com/compute/docs/subnetworks) to which the cluster is connected. |
| `control_plane_endpoints_config` | String | Configuration for all cluster's control plane endpoints. |
| `instance_group_urls` | Vec<String> | Output only. Deprecated. Use node_pools.instance_group_urls. |
| `network_config` | String | Configuration for cluster networking. |
| `master_auth` | String | The authentication information for accessing the master endpoint. If unspecified, the defaults are used: For clusters before v1.12, if master_auth is unspecified, `username` will be set to "admin", a random password will be generated, and a client certificate will be issued. |
| `resource_labels` | HashMap<String, String> | The resource labels for the cluster to use to annotate any related Google Compute Engine resources. |
| `confidential_nodes` | String | Configuration of Confidential Nodes. All the nodes in the cluster will be Confidential VM once enabled. |
| `ip_allocation_policy` | String | Configuration for cluster IP allocation. |
| `logging_config` | String | Logging configuration for the cluster. |
| `node_pool_auto_config` | String | Node pool configs that apply to all auto-provisioned node pools in autopilot clusters and node auto-provisioning enabled clusters. |
| `private_cluster_config` | String | Configuration for private cluster. |
| `node_pools` | Vec<String> | The node pools associated with this cluster. This field should not be set if "node_config" or "initial_node_count" are specified. |
| `services_ipv4_cidr` | String | Output only. The IP address range of the Kubernetes services in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `1.2.3.4/29`). Service addresses are typically put in the last `/16` from the container CIDR. |
| `tpu_ipv4_cidr_block` | String | Output only. The IP address range of the Cloud TPUs in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `1.2.3.4/29`). This field is deprecated due to the deprecation of 2VM TPU. The end of life date for 2VM TPU is 2025-04-25. |
| `gke_auto_upgrade_config` | String | Configuration for GKE auto upgrades. |
| `label_fingerprint` | String | The fingerprint of the set of labels for this cluster. |
| `monitoring_service` | String | The monitoring service the cluster should use to write metrics. Currently available options: * `monitoring.googleapis.com/kubernetes` - The Cloud Monitoring service with a Kubernetes-native resource model * `monitoring.googleapis.com` - The legacy Cloud Monitoring service (no longer available as of GKE 1.15). * `none` - No metrics will be exported from the cluster. If left as an empty string,`monitoring.googleapis.com/kubernetes` will be used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions. |
| `node_ipv4_cidr_size` | i64 | Output only. The size of the address space on each node for hosting containers. This is provisioned from within the `container_ipv4_cidr` range. This field will only be set when cluster is in route-based network mode. |
| `secret_manager_config` | String | Secret CSI driver configuration. |
| `current_node_count` | i64 | Output only. The number of nodes currently in the cluster. Deprecated. Call Kubernetes API directly to retrieve node information. |
| `user_managed_keys_config` | String | The Custom keys configuration for the cluster. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `network` | String | The name of the Google Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the cluster is connected. If left unspecified, the `default` network will be used. |
| `default_max_pods_constraint` | String | The default constraint on the maximum number of pods that can be run simultaneously on a node in the node pool of this cluster. Only honored if cluster created with IP Alias support. |
| `autoscaling` | String | Cluster-level autoscaling configuration. |
| `alpha_cluster_feature_gates` | Vec<String> | The list of user specified Kubernetes feature gates. Each string represents the activation status of a feature gate (e.g. "featureX=true" or "featureX=false") |
| `enable_tpu` | bool | Enable the ability to use Cloud TPUs in this cluster. This field is deprecated due to the deprecation of 2VM TPU. The end of life date for 2VM TPU is 2025-04-25. |
| `node_config` | String | Parameters used in creating the cluster's nodes. For requests, this field should only be used in lieu of a "node_pool" object, since this configuration (along with the "initial_node_count") will be used to create a "NodePool" object with an auto-generated name. Do not use this and a node_pool at the same time. For responses, this field will be populated with the node configuration of the first node pool. (For configuration of each node pool, see `node_pool.config`) If unspecified, the defaults are used. This field is deprecated, use node_pool.config instead. |
| `self_link` | String | Output only. Server-defined URL for the resource. |
| `enable_k8s_beta_apis` | String | Beta APIs Config |
| `maintenance_policy` | String | Configure the maintenance policy for this cluster. |
| `authenticator_groups_config` | String | Configuration controlling RBAC group membership information. |
| `logging_service` | String | The logging service the cluster should use to write logs. Currently available options: * `logging.googleapis.com/kubernetes` - The Cloud Logging service with a Kubernetes-native resource model * `logging.googleapis.com` - The legacy Cloud Logging service (no longer available as of GKE 1.15). * `none` - no logs will be exported from the cluster. If left as an empty string,`logging.googleapis.com/kubernetes` will be used for GKE 1.14+ or `logging.googleapis.com` for earlier versions. |
| `mesh_certificates` | String | Configuration for issuance of mTLS keys and certificates to Kubernetes pods. |
| `current_master_version` | String | Output only. The current software version of the master endpoint. |
| `initial_node_count` | i64 | The number of nodes to create in this cluster. You must ensure that your Compute Engine [resource quota](https://cloud.google.com/compute/quotas) is sufficient for this number of instances. You must also have available firewall and routes quota. For requests, this field should only be used in lieu of a "node_pool" object, since this configuration (along with the "node_config") will be used to create a "NodePool" object with an auto-generated name. Do not use this and a node_pool at the same time. This field is deprecated, use node_pool.initial_node_count instead. |
| `id` | String | Output only. Unique id for the cluster. |
| `release_channel` | String | Release channel configuration. If left unspecified on cluster creation and a version is specified, the cluster is enrolled in the most mature release channel where the version is available (first checking STABLE, then REGULAR, and finally RAPID). Otherwise, if no release channel configuration and no version is specified, the cluster is enrolled in the REGULAR channel with its default version. |
| `enterprise_config` | String | GKE Enterprise Configuration. Deprecated: GKE Enterprise features are now available without an Enterprise tier. |
| `security_posture_config` | String | Enable/Disable Security Posture API features for the cluster. |
| `master_authorized_networks_config` | String | The configuration options for master authorized networks feature. Deprecated: Use ControlPlaneEndpointsConfig.IPEndpointsConfig.authorized_networks_config instead. |
| `status` | String | Output only. The current status of this cluster. |
| `status_message` | String | Output only. Deprecated. Use conditions instead. Additional information about the current status of this cluster, if available. |
| `workload_identity_config` | String | Configuration for the use of Kubernetes Service Accounts in IAM policies. |
| `description` | String | An optional description of this cluster. |
| `binary_authorization` | String | Configuration for Binary Authorization. |
| `etag` | String | This checksum is computed by the server based on the value of cluster fields, and may be sent on update requests to ensure the client has an up-to-date value before proceeding. |
| `expire_time` | String | Output only. The time the cluster will be automatically deleted in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `anonymous_authentication_config` | String | Configuration for limiting anonymous access to all endpoints except the health checks. |
| `network_policy` | String | Configuration options for the NetworkPolicy feature. |
| `endpoint` | String | Output only. The IP address of this cluster's master endpoint. The endpoint can be accessed from the internet at `https://username:password@endpoint/`. See the `masterAuth` property of this resource for username and password information. |
| `pod_autoscaling` | String | The config for pod autoscaling. |
| `create_time` | String | Output only. The time the cluster was created, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `monitoring_config` | String | Monitoring configuration for the cluster. |
| `compliance_posture_config` | String | Enable/Disable Compliance Posture features for the cluster. |
| `vertical_pod_autoscaling` | String | Cluster-level Vertical Pod Autoscaling configuration. |
| `name` | String | The name of this cluster. The name must be unique within this project and location (e.g. zone or region), and can be up to 40 characters with the following restrictions: * Lowercase letters, numbers, and hyphens only. * Must start with a letter. * Must end with a number or a letter. |
| `database_encryption` | String | Configuration of etcd encryption. |
| `parent_product_config` | String | The configuration of the parent product of the cluster. This field is used by Google internal products that are built on top of the GKE cluster and take the ownership of the cluster. |
| `zone` | String | Output only. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field is deprecated, use location instead. |
| `cost_management_config` | String | Configuration for the fine-grained cost management feature. |
| `enable_kubernetes_alpha` | bool | Kubernetes alpha features are enabled on this cluster. This includes alpha API groups (e.g. v1alpha1) and features that may not be production ready in the kubernetes version of the master and nodes. The cluster has no SLA for uptime and master/node upgrades are disabled. Alpha enabled clusters are automatically deleted thirty days after creation. |
| `conditions` | Vec<String> | Which conditions caused the current cluster state. |
| `current_node_version` | String | Output only. Deprecated, use [NodePools.version](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools) instead. The current version of the node software components. If they are currently at multiple versions because they're in the process of being upgraded, this reflects the minimum version of all nodes. |
| `locations` | Vec<String> | The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the cluster's nodes should be located. This field provides a default value if [NodePool.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.FIELDS.locations) are not specified during node pool creation. Warning: changing cluster locations will update the [NodePool.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.FIELDS.locations) of all node pools and will result in nodes being added and/or removed. |
| `legacy_abac` | String | Configuration for the legacy ABAC authorization mode. |
| `shielded_nodes` | String | Shielded Nodes configuration. |
| `autopilot` | String | Autopilot configuration for the cluster. |
| `identity_service_config` | String | Configuration for Identity Service component. |
| `notification_config` | String | Notification configuration of the cluster. |
| `addons_config` | String | Configurations for the various addons available to run in the cluster. |
| `location` | String | Output only. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in which the cluster resides. |
| `rbac_binding_config` | String | RBACBindingConfig allows user to restrict ClusterRoleBindings an RoleBindings that can be created. |
| `initial_cluster_version` | String | The initial Kubernetes version for this cluster. Valid versions are those found in validMasterVersions returned by getServerConfig. The version can be upgraded over time; such upgrades are reflected in currentMasterVersion and currentNodeVersion. Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - "latest": picks the highest valid Kubernetes version - "1.X": picks the highest valid patch+gke.N patch in the 1.X version - "1.X.Y": picks the highest valid gke.N patch in the 1.X.Y version - "1.X.Y-gke.N": picks an explicit Kubernetes version - "","-": picks the default Kubernetes version |
| `resource_usage_export_config` | String | Configuration for exporting resource usages. Resource usage export is disabled when this config is unspecified. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `cluster_ipv4_cidr` | String | The IP address range of the container pods in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`). Leave blank to have one automatically chosen or specify a `/14` block in `10.0.0.0/8`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cluster
cluster = provider.container_api.Cluster {
    zone = "value"  # Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field.
    project_id = "value"  # Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field.
}

# Access cluster outputs
cluster_id = cluster.id
cluster_fleet = cluster.fleet
cluster_node_pool_defaults = cluster.node_pool_defaults
cluster_subnetwork = cluster.subnetwork
cluster_control_plane_endpoints_config = cluster.control_plane_endpoints_config
cluster_instance_group_urls = cluster.instance_group_urls
cluster_network_config = cluster.network_config
cluster_master_auth = cluster.master_auth
cluster_resource_labels = cluster.resource_labels
cluster_confidential_nodes = cluster.confidential_nodes
cluster_ip_allocation_policy = cluster.ip_allocation_policy
cluster_logging_config = cluster.logging_config
cluster_node_pool_auto_config = cluster.node_pool_auto_config
cluster_private_cluster_config = cluster.private_cluster_config
cluster_node_pools = cluster.node_pools
cluster_services_ipv4_cidr = cluster.services_ipv4_cidr
cluster_tpu_ipv4_cidr_block = cluster.tpu_ipv4_cidr_block
cluster_gke_auto_upgrade_config = cluster.gke_auto_upgrade_config
cluster_label_fingerprint = cluster.label_fingerprint
cluster_monitoring_service = cluster.monitoring_service
cluster_node_ipv4_cidr_size = cluster.node_ipv4_cidr_size
cluster_secret_manager_config = cluster.secret_manager_config
cluster_current_node_count = cluster.current_node_count
cluster_user_managed_keys_config = cluster.user_managed_keys_config
cluster_satisfies_pzs = cluster.satisfies_pzs
cluster_network = cluster.network
cluster_default_max_pods_constraint = cluster.default_max_pods_constraint
cluster_autoscaling = cluster.autoscaling
cluster_alpha_cluster_feature_gates = cluster.alpha_cluster_feature_gates
cluster_enable_tpu = cluster.enable_tpu
cluster_node_config = cluster.node_config
cluster_self_link = cluster.self_link
cluster_enable_k8s_beta_apis = cluster.enable_k8s_beta_apis
cluster_maintenance_policy = cluster.maintenance_policy
cluster_authenticator_groups_config = cluster.authenticator_groups_config
cluster_logging_service = cluster.logging_service
cluster_mesh_certificates = cluster.mesh_certificates
cluster_current_master_version = cluster.current_master_version
cluster_initial_node_count = cluster.initial_node_count
cluster_id = cluster.id
cluster_release_channel = cluster.release_channel
cluster_enterprise_config = cluster.enterprise_config
cluster_security_posture_config = cluster.security_posture_config
cluster_master_authorized_networks_config = cluster.master_authorized_networks_config
cluster_status = cluster.status
cluster_status_message = cluster.status_message
cluster_workload_identity_config = cluster.workload_identity_config
cluster_description = cluster.description
cluster_binary_authorization = cluster.binary_authorization
cluster_etag = cluster.etag
cluster_expire_time = cluster.expire_time
cluster_anonymous_authentication_config = cluster.anonymous_authentication_config
cluster_network_policy = cluster.network_policy
cluster_endpoint = cluster.endpoint
cluster_pod_autoscaling = cluster.pod_autoscaling
cluster_create_time = cluster.create_time
cluster_monitoring_config = cluster.monitoring_config
cluster_compliance_posture_config = cluster.compliance_posture_config
cluster_vertical_pod_autoscaling = cluster.vertical_pod_autoscaling
cluster_name = cluster.name
cluster_database_encryption = cluster.database_encryption
cluster_parent_product_config = cluster.parent_product_config
cluster_zone = cluster.zone
cluster_cost_management_config = cluster.cost_management_config
cluster_enable_kubernetes_alpha = cluster.enable_kubernetes_alpha
cluster_conditions = cluster.conditions
cluster_current_node_version = cluster.current_node_version
cluster_locations = cluster.locations
cluster_legacy_abac = cluster.legacy_abac
cluster_shielded_nodes = cluster.shielded_nodes
cluster_autopilot = cluster.autopilot
cluster_identity_service_config = cluster.identity_service_config
cluster_notification_config = cluster.notification_config
cluster_addons_config = cluster.addons_config
cluster_location = cluster.location
cluster_rbac_binding_config = cluster.rbac_binding_config
cluster_initial_cluster_version = cluster.initial_cluster_version
cluster_resource_usage_export_config = cluster.resource_usage_export_config
cluster_satisfies_pzi = cluster.satisfies_pzi
cluster_cluster_ipv4_cidr = cluster.cluster_ipv4_cidr
```

---


### Well_known

Gets the OIDC discovery document for the cluster. See the [OpenID Connect Discovery 1.0 specification](https://openid.net/specs/openid-connect-discovery-1_0.html) for details.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `issuer` | String | OIDC Issuer. |
| `response_types_supported` | Vec<String> | Supported response types. |
| `subject_types_supported` | Vec<String> | Supported subject types. |
| `claims_supported` | Vec<String> | Supported claims. |
| `grant_types` | Vec<String> | Supported grant types. |
| `id_token_signing_alg_values_supported` | Vec<String> | supported ID Token signing Algorithms. |
| `jwks_uri` | String | JSON Web Key uri. |
| `cache_header` | String | For HTTP requests, this field is automatically extracted into the Cache-Control HTTP header. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access well_known outputs
well_known_id = well_known.id
well_known_issuer = well_known.issuer
well_known_response_types_supported = well_known.response_types_supported
well_known_subject_types_supported = well_known.subject_types_supported
well_known_claims_supported = well_known.claims_supported
well_known_grant_types = well_known.grant_types
well_known_id_token_signing_alg_values_supported = well_known.id_token_signing_alg_values_supported
well_known_jwks_uri = well_known.jwks_uri
well_known_cache_header = well_known.cache_header
```

---


### Usable_subnetwork

Lists subnetworks that can be used for creating clusters in a project.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | This token allows you to get the next page of results for list requests. If the number of results is larger than `page_size`, use the `next_page_token` as a value for the query parameter `page_token` in the next request. The value will become empty when there are no more pages. |
| `subnetworks` | Vec<String> | A list of usable subnetworks in the specified network project. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access usable_subnetwork outputs
usable_subnetwork_id = usable_subnetwork.id
usable_subnetwork_next_page_token = usable_subnetwork.next_page_token
usable_subnetwork_subnetworks = usable_subnetwork.subnetworks
```

---


### Zone

Returns configuration info about the Google Kubernetes Engine service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channels` | Vec<String> | List of release channel configurations. |
| `valid_node_versions` | Vec<String> | List of valid node upgrade target versions, in descending order. |
| `windows_version_maps` | HashMap<String, String> | Maps of Kubernetes version and supported Windows server versions. |
| `default_cluster_version` | String | Version of Kubernetes the service deploys by default. |
| `default_image_type` | String | Default image type. |
| `valid_image_types` | Vec<String> | List of valid image types. |
| `valid_master_versions` | Vec<String> | List of valid master versions, in descending order. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access zone outputs
zone_id = zone.id
zone_channels = zone.channels
zone_valid_node_versions = zone.valid_node_versions
zone_windows_version_maps = zone.windows_version_maps
zone_default_cluster_version = zone.default_cluster_version
zone_default_image_type = zone.default_image_type
zone_valid_image_types = zone.valid_image_types
zone_valid_master_versions = zone.valid_master_versions
```

---


### Location

Fetches locations that offer Google Kubernetes Engine.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Only return ListLocationsResponse that occur after the page_token. This value should be populated from the ListLocationsResponse.next_page_token if that response token was set (which happens when listing more Locations than fit in a single ListLocationsResponse). |
| `locations` | Vec<String> | A full list of GKE locations. |


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
location_next_page_token = location.next_page_token
location_locations = location.locations
```

---


### Well_known

Gets the OIDC discovery document for the cluster. See the [OpenID Connect Discovery 1.0 specification](https://openid.net/specs/openid-connect-discovery-1_0.html) for details.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cache_header` | String | For HTTP requests, this field is automatically extracted into the Cache-Control HTTP header. |
| `jwks_uri` | String | JSON Web Key uri. |
| `grant_types` | Vec<String> | Supported grant types. |
| `claims_supported` | Vec<String> | Supported claims. |
| `subject_types_supported` | Vec<String> | Supported subject types. |
| `id_token_signing_alg_values_supported` | Vec<String> | supported ID Token signing Algorithms. |
| `response_types_supported` | Vec<String> | Supported response types. |
| `issuer` | String | OIDC Issuer. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access well_known outputs
well_known_id = well_known.id
well_known_cache_header = well_known.cache_header
well_known_jwks_uri = well_known.jwks_uri
well_known_grant_types = well_known.grant_types
well_known_claims_supported = well_known.claims_supported
well_known_subject_types_supported = well_known.subject_types_supported
well_known_id_token_signing_alg_values_supported = well_known.id_token_signing_alg_values_supported
well_known_response_types_supported = well_known.response_types_supported
well_known_issuer = well_known.issuer
```

---


### Operation

Cancels the specified operation.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name (project, location, operation id) of the operation to cancel. Specified in the format `projects/*/locations/*/operations/*`. |
| `project_id` | String |  | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field. |
| `operation_id` | String |  | Deprecated. The server-assigned `name` of the operation. This field has been deprecated and replaced by the name field. |
| `zone` | String |  | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation resides. This field has been deprecated and replaced by the name field. |
| `name` | String | ✅ | The name (project, location, operation id) of the operation to cancel. Specified in the format `projects/*/locations/*/operations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location` | String | Output only. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in which the cluster resides. |
| `target_link` | String | Output only. Server-defined URI for the target of the operation. The format of this is a URI to the resource being modified (such as a cluster, node pool, or node). For node pool repairs, there may be multiple nodes being repaired, but only one will be the target. Examples: - ## `https://container.googleapis.com/v1/projects/123/locations/us-central1/clusters/my-cluster` ## `https://container.googleapis.com/v1/projects/123/zones/us-central1-c/clusters/my-cluster/nodePools/my-np` `https://container.googleapis.com/v1/projects/123/zones/us-central1-c/clusters/my-cluster/nodePools/my-np/node/my-node` |
| `nodepool_conditions` | Vec<String> | Which conditions caused the current node pool state. Deprecated. Use field error instead. |
| `name` | String | Output only. The server-assigned ID for the operation. |
| `progress` | String | Output only. Progress information for an operation. |
| `detail` | String | Output only. Detailed operation progress, if available. |
| `start_time` | String | Output only. The time the operation started, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `operation_type` | String | Output only. The operation type. |
| `status` | String | Output only. The current status of the operation. |
| `cluster_conditions` | Vec<String> | Which conditions caused the current cluster state. Deprecated. Use field error instead. |
| `status_message` | String | Output only. If an error has occurred, a textual description of the error. Deprecated. Use field error instead. |
| `end_time` | String | Output only. The time the operation completed, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `zone` | String | Output only. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation is taking place. This field is deprecated, use location instead. |
| `self_link` | String | Output only. Server-defined URI for the operation. Example: `https://container.googleapis.com/v1alpha1/projects/123/locations/us-central1/operations/operation-123`. |
| `error` | String | The error result of the operation in case of failure. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.container_api.Operation {
    name = "value"  # The name (project, location, operation id) of the operation to cancel. Specified in the format `projects/*/locations/*/operations/*`.
}

# Access operation outputs
operation_id = operation.id
operation_location = operation.location
operation_target_link = operation.target_link
operation_nodepool_conditions = operation.nodepool_conditions
operation_name = operation.name
operation_progress = operation.progress
operation_detail = operation.detail
operation_start_time = operation.start_time
operation_operation_type = operation.operation_type
operation_status = operation.status
operation_cluster_conditions = operation.cluster_conditions
operation_status_message = operation.status_message
operation_end_time = operation.end_time
operation_zone = operation.zone
operation_self_link = operation.self_link
operation_error = operation.error
```

---


### Cluster

Creates a cluster, consisting of the specified number and type of Google Compute Engine instances. By default, the cluster is created in the project's [default network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks). One firewall is added for the cluster. After cluster creation, the kubelet creates routes for each node to allow the containers on that node to communicate with all other instances in the cluster. Finally, an entry is added to the project's global metadata indicating which CIDR range the cluster is using.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project_id` | String |  | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field. |
| `cluster` | String |  | Required. A [cluster resource](https://cloud.google.com/container-engine/reference/rest/v1beta1/projects.locations.clusters) |
| `parent` | String |  | The parent (project and location) where the cluster will be created. Specified in the format `projects/*/locations/*`. |
| `zone` | String |  | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field. |
| `project_id` | String | ✅ | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field. |
| `zone` | String | ✅ | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cost_management_config` | String | Configuration for the fine-grained cost management feature. |
| `create_time` | String | Output only. The time the cluster was created, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `conditions` | Vec<String> | Which conditions caused the current cluster state. |
| `zone` | String | Output only. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field is deprecated, use location instead. |
| `pod_security_policy_config` | String | Configuration for the PodSecurityPolicy feature. |
| `instance_group_urls` | Vec<String> | Output only. Deprecated. Use node_pools.instance_group_urls. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `workload_alts_config` | String | Configuration for direct-path (via ALTS) with workload identity. This feature is not officially supported for external customers in Kubernetes Engine when using Workload Identity. |
| `identity_service_config` | String | Configuration for Identity Service component. |
| `logging_config` | String | Logging configuration for the cluster. |
| `mesh_certificates` | String | Configuration for issuance of mTLS keys and certificates to Kubernetes pods. |
| `network_config` | String | Configuration for cluster networking. |
| `current_master_version` | String | Output only. The current software version of the master endpoint. |
| `initial_cluster_version` | String | The initial Kubernetes version for this cluster. Valid versions are those found in validMasterVersions returned by getServerConfig. The version can be upgraded over time; such upgrades are reflected in currentMasterVersion and currentNodeVersion. Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - "latest": picks the highest valid Kubernetes version - "1.X": picks the highest valid patch+gke.N patch in the 1.X version - "1.X.Y": picks the highest valid gke.N patch in the 1.X.Y version - "1.X.Y-gke.N": picks an explicit Kubernetes version - "","-": picks the default Kubernetes version |
| `addons_config` | String | Configurations for the various addons available to run in the cluster. |
| `autoscaling` | String | Cluster-level autoscaling configuration. |
| `master` | String | Configuration for master components. |
| `binary_authorization` | String | Configuration for Binary Authorization. |
| `monitoring_config` | String | Monitoring configuration for the cluster. |
| `shielded_nodes` | String | Shielded Nodes configuration. |
| `status` | String | Output only. The current status of this cluster. |
| `user_managed_keys_config` | String | The Custom keys configuration for the cluster. |
| `enable_kubernetes_alpha` | bool | Kubernetes alpha features are enabled on this cluster. This includes alpha API groups (e.g. v1beta1) and features that may not be production ready in the kubernetes version of the master and nodes. The cluster has no SLA for uptime and master/node upgrades are disabled. Alpha enabled clusters are automatically deleted thirty days after creation. |
| `node_config` | String | Parameters used in creating the cluster's nodes. For requests, this field should only be used in lieu of a "node_pool" object, since this configuration (along with the "initial_node_count") will be used to create a "NodePool" object with an auto-generated name. Do not use this and a node_pool at the same time. For responses, this field will be populated with the node configuration of the first node pool. (For configuration of each node pool, see `node_pool.config`) If unspecified, the defaults are used. This field is deprecated, use node_pool.config instead. |
| `control_plane_endpoints_config` | String | Configuration for all cluster's control plane endpoints. |
| `gke_auto_upgrade_config` | String | Configuration for GKE auto upgrades. |
| `location` | String | Output only. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in which the cluster resides. |
| `default_max_pods_constraint` | String | The default constraint on the maximum number of pods that can be run simultaneously on a node in the node pool of this cluster. Only honored if cluster created with IP Alias support. |
| `compliance_posture_config` | String | Enable/Disable Compliance Posture features for the cluster. |
| `enable_tpu` | bool | Enable the ability to use Cloud TPUs in this cluster. This field is deprecated, use tpu_config.enabled instead. This field is deprecated due to the deprecation of 2VM TPU. The end of life date for 2VM TPU is 2025-04-25. |
| `network_policy` | String | Configuration options for the NetworkPolicy feature. |
| `parent_product_config` | String | The configuration of the parent product of the cluster. This field is used by Google internal products that are built on top of the GKE cluster and take the ownership of the cluster. |
| `node_pool_defaults` | String | Default NodePool settings for the entire cluster. These settings are overridden if specified on the specific NodePool object. |
| `current_node_version` | String | Output only. Deprecated, use [NodePool.version](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/projects.locations.clusters.nodePools) instead. The current version of the node software components. If they are currently at multiple versions because they're in the process of being upgraded, this reflects the minimum version of all nodes. |
| `enable_k8s_beta_apis` | String | Kubernetes open source beta apis enabled on the cluster. Only beta apis. |
| `network` | String | The name of the Google Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the cluster is connected. If left unspecified, the `default` network will be used. On output this shows the network ID instead of the name. |
| `pod_autoscaling` | String | The config for pod autoscaling. |
| `resource_labels` | HashMap<String, String> | The resource labels for the cluster to use to annotate any related Google Compute Engine resources. |
| `security_posture_config` | String | Enable/Disable Security Posture API features for the cluster. |
| `status_message` | String | Output only. Deprecated. Use conditions instead. Additional information about the current status of this cluster, if available. |
| `confidential_nodes` | String | Configuration of Confidential Nodes. All the nodes in the cluster will be Confidential VM once enabled. |
| `workload_identity_config` | String | Configuration for the use of Kubernetes Service Accounts in IAM policies. |
| `logging_service` | String | The logging service the cluster should use to write logs. Currently available options: * `logging.googleapis.com/kubernetes` - The Cloud Logging service with a Kubernetes-native resource model * `logging.googleapis.com` - The legacy Cloud Logging service (no longer available as of GKE 1.15). * `none` - no logs will be exported from the cluster. If left as an empty string,`logging.googleapis.com/kubernetes` will be used for GKE 1.14+ or `logging.googleapis.com` for earlier versions. |
| `cluster_ipv4_cidr` | String | The IP address range of the container pods in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`). Leave blank to have one automatically chosen or specify a `/14` block in `10.0.0.0/8`. |
| `database_encryption` | String | Configuration of etcd encryption. |
| `etag` | String | This checksum is computed by the server based on the value of cluster fields, and may be sent on update requests to ensure the client has an up-to-date value before proceeding. |
| `locations` | Vec<String> | The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the cluster's nodes should be located. This field provides a default value if [NodePool.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.FIELDS.locations) are not specified during node pool creation. Warning: changing cluster locations will update the [NodePool.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.FIELDS.locations) of all node pools and will result in nodes being added and/or removed. |
| `node_pool_auto_config` | String | Node pool configs that apply to all auto-provisioned node pools in autopilot clusters and node auto-provisioning enabled clusters. |
| `endpoint` | String | Output only. The IP address of this cluster's master endpoint. The endpoint can be accessed from the internet at `https://username:password@endpoint/`. See the `masterAuth` property of this resource for username and password information. |
| `resource_usage_export_config` | String | Configuration for exporting resource usages. Resource usage export is disabled when this config unspecified. |
| `tpu_ipv4_cidr_block` | String | Output only. The IP address range of the Cloud TPUs in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `1.2.3.4/29`). This field is deprecated due to the deprecation of 2VM TPU. The end of life date for 2VM TPU is 2025-04-25. |
| `master_auth` | String | The authentication information for accessing the master endpoint. If unspecified, the defaults are used: For clusters before v1.12, if master_auth is unspecified, `username` will be set to "admin", a random password will be generated, and a client certificate will be issued. |
| `self_link` | String | Output only. Server-defined URL for the resource. |
| `protect_config` | String | Deprecated: Use SecurityPostureConfig instead. Enable/Disable Protect API features for the cluster. |
| `vertical_pod_autoscaling` | String | Cluster-level Vertical Pod Autoscaling configuration. |
| `alpha_cluster_feature_gates` | Vec<String> | The list of user specified Kubernetes feature gates. Each string represents the activation status of a feature gate (e.g. "featureX=true" or "featureX=false") |
| `secret_sync_config` | String | Configuration for sync Secret Manager secrets as k8s secrets. |
| `private_cluster_config` | String | Configuration for private cluster. |
| `ip_allocation_policy` | String | Configuration for cluster IP allocation. |
| `anonymous_authentication_config` | String | Configuration for limiting anonymous access to all endpoints except the health checks. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `cluster_telemetry` | String | Telemetry integration for the cluster. |
| `maintenance_policy` | String | Configure the maintenance policy for this cluster. |
| `node_ipv4_cidr_size` | i64 | Output only. The size of the address space on each node for hosting containers. This is provisioned from within the `container_ipv4_cidr` range. This field will only be set when cluster is in route-based network mode. |
| `services_ipv4_cidr` | String | Output only. The IP address range of the Kubernetes services in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `1.2.3.4/29`). Service addresses are typically put in the last `/16` from the container CIDR. |
| `subnetwork` | String | The name of the Google Compute Engine [subnetwork](https://cloud.google.com/compute/docs/subnetworks) to which the cluster is connected. On output this shows the subnetwork ID instead of the name. |
| `description` | String | An optional description of this cluster. |
| `notification_config` | String | Notification configuration of the cluster. |
| `secret_manager_config` | String | Secret CSI driver configuration. |
| `current_node_count` | i64 | Output only. The number of nodes currently in the cluster. Deprecated. Call Kubernetes API directly to retrieve node information. |
| `enterprise_config` | String | GKE Enterprise Configuration. Deprecated: GKE Enterprise features are now available without an Enterprise tier. |
| `master_ipv4_cidr_block` | String | The IP prefix in CIDR notation to use for the hosted master network. This prefix will be used for assigning private IP addresses to the master or set of masters, as well as the ILB VIP. This field is deprecated, use private_cluster_config.master_ipv4_cidr_block instead. |
| `name` | String | The name of this cluster. The name must be unique within this project and location (e.g. zone or region), and can be up to 40 characters with the following restrictions: * Lowercase letters, numbers, and hyphens only. * Must start with a letter. * Must end with a number or a letter. |
| `id` | String | Output only. Unique id for the cluster. |
| `private_cluster` | bool | If this is a private cluster setup. Private clusters are clusters that, by default have no external IP addresses on the nodes and where nodes and the master communicate over private IP addresses. This field is deprecated, use private_cluster_config.enable_private_nodes instead. |
| `rbac_binding_config` | String | RBACBindingConfig allows user to restrict ClusterRoleBindings an RoleBindings that can be created. |
| `release_channel` | String | Release channel configuration. If left unspecified on cluster creation and a version is specified, the cluster is enrolled in the most mature release channel where the version is available (first checking STABLE, then REGULAR, and finally RAPID). Otherwise, if no release channel configuration and no version is specified, the cluster is enrolled in the REGULAR channel with its default version. |
| `tpu_config` | String | Configuration for Cloud TPU support; This field is deprecated due to the deprecation of 2VM TPU. The end of life date for 2VM TPU is 2025-04-25. |
| `initial_node_count` | i64 | The number of nodes to create in this cluster. You must ensure that your Compute Engine [resource quota](https://cloud.google.com/compute/quotas) is sufficient for this number of instances. You must also have available firewall and routes quota. For requests, this field should only be used in lieu of a "node_pool" object, since this configuration (along with the "node_config") will be used to create a "NodePool" object with an auto-generated name. Do not use this and a node_pool at the same time. This field is deprecated, use node_pool.initial_node_count instead. |
| `authenticator_groups_config` | String | Configuration controlling RBAC group membership information. |
| `label_fingerprint` | String | The fingerprint of the set of labels for this cluster. |
| `autopilot` | String | Autopilot configuration for the cluster. |
| `monitoring_service` | String | The monitoring service the cluster should use to write metrics. Currently available options: * `monitoring.googleapis.com/kubernetes` - The Cloud Monitoring service with a Kubernetes-native resource model * `monitoring.googleapis.com` - The legacy Cloud Monitoring service (no longer available as of GKE 1.15). * `none` - No metrics will be exported from the cluster. If left as an empty string,`monitoring.googleapis.com/kubernetes` will be used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions. |
| `legacy_abac` | String | Configuration for the legacy ABAC authorization mode. |
| `master_authorized_networks_config` | String | The configuration options for master authorized networks feature. Deprecated: Use ControlPlaneEndpointsConfig.IPEndpointsConfig.authorized_networks_config instead. |
| `node_pools` | Vec<String> | The node pools associated with this cluster. This field should not be set if "node_config" or "initial_node_count" are specified. |
| `workload_certificates` | String | Configuration for issuance of mTLS keys and certificates to Kubernetes pods. |
| `fleet` | String | Fleet information for the cluster. |
| `expire_time` | String | Output only. The time the cluster will be automatically deleted in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cluster
cluster = provider.container_api.Cluster {
    project_id = "value"  # Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field.
    zone = "value"  # Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field.
}

# Access cluster outputs
cluster_id = cluster.id
cluster_cost_management_config = cluster.cost_management_config
cluster_create_time = cluster.create_time
cluster_conditions = cluster.conditions
cluster_zone = cluster.zone
cluster_pod_security_policy_config = cluster.pod_security_policy_config
cluster_instance_group_urls = cluster.instance_group_urls
cluster_satisfies_pzs = cluster.satisfies_pzs
cluster_workload_alts_config = cluster.workload_alts_config
cluster_identity_service_config = cluster.identity_service_config
cluster_logging_config = cluster.logging_config
cluster_mesh_certificates = cluster.mesh_certificates
cluster_network_config = cluster.network_config
cluster_current_master_version = cluster.current_master_version
cluster_initial_cluster_version = cluster.initial_cluster_version
cluster_addons_config = cluster.addons_config
cluster_autoscaling = cluster.autoscaling
cluster_master = cluster.master
cluster_binary_authorization = cluster.binary_authorization
cluster_monitoring_config = cluster.monitoring_config
cluster_shielded_nodes = cluster.shielded_nodes
cluster_status = cluster.status
cluster_user_managed_keys_config = cluster.user_managed_keys_config
cluster_enable_kubernetes_alpha = cluster.enable_kubernetes_alpha
cluster_node_config = cluster.node_config
cluster_control_plane_endpoints_config = cluster.control_plane_endpoints_config
cluster_gke_auto_upgrade_config = cluster.gke_auto_upgrade_config
cluster_location = cluster.location
cluster_default_max_pods_constraint = cluster.default_max_pods_constraint
cluster_compliance_posture_config = cluster.compliance_posture_config
cluster_enable_tpu = cluster.enable_tpu
cluster_network_policy = cluster.network_policy
cluster_parent_product_config = cluster.parent_product_config
cluster_node_pool_defaults = cluster.node_pool_defaults
cluster_current_node_version = cluster.current_node_version
cluster_enable_k8s_beta_apis = cluster.enable_k8s_beta_apis
cluster_network = cluster.network
cluster_pod_autoscaling = cluster.pod_autoscaling
cluster_resource_labels = cluster.resource_labels
cluster_security_posture_config = cluster.security_posture_config
cluster_status_message = cluster.status_message
cluster_confidential_nodes = cluster.confidential_nodes
cluster_workload_identity_config = cluster.workload_identity_config
cluster_logging_service = cluster.logging_service
cluster_cluster_ipv4_cidr = cluster.cluster_ipv4_cidr
cluster_database_encryption = cluster.database_encryption
cluster_etag = cluster.etag
cluster_locations = cluster.locations
cluster_node_pool_auto_config = cluster.node_pool_auto_config
cluster_endpoint = cluster.endpoint
cluster_resource_usage_export_config = cluster.resource_usage_export_config
cluster_tpu_ipv4_cidr_block = cluster.tpu_ipv4_cidr_block
cluster_master_auth = cluster.master_auth
cluster_self_link = cluster.self_link
cluster_protect_config = cluster.protect_config
cluster_vertical_pod_autoscaling = cluster.vertical_pod_autoscaling
cluster_alpha_cluster_feature_gates = cluster.alpha_cluster_feature_gates
cluster_secret_sync_config = cluster.secret_sync_config
cluster_private_cluster_config = cluster.private_cluster_config
cluster_ip_allocation_policy = cluster.ip_allocation_policy
cluster_anonymous_authentication_config = cluster.anonymous_authentication_config
cluster_satisfies_pzi = cluster.satisfies_pzi
cluster_cluster_telemetry = cluster.cluster_telemetry
cluster_maintenance_policy = cluster.maintenance_policy
cluster_node_ipv4_cidr_size = cluster.node_ipv4_cidr_size
cluster_services_ipv4_cidr = cluster.services_ipv4_cidr
cluster_subnetwork = cluster.subnetwork
cluster_description = cluster.description
cluster_notification_config = cluster.notification_config
cluster_secret_manager_config = cluster.secret_manager_config
cluster_current_node_count = cluster.current_node_count
cluster_enterprise_config = cluster.enterprise_config
cluster_master_ipv4_cidr_block = cluster.master_ipv4_cidr_block
cluster_name = cluster.name
cluster_id = cluster.id
cluster_private_cluster = cluster.private_cluster
cluster_rbac_binding_config = cluster.rbac_binding_config
cluster_release_channel = cluster.release_channel
cluster_tpu_config = cluster.tpu_config
cluster_initial_node_count = cluster.initial_node_count
cluster_authenticator_groups_config = cluster.authenticator_groups_config
cluster_label_fingerprint = cluster.label_fingerprint
cluster_autopilot = cluster.autopilot
cluster_monitoring_service = cluster.monitoring_service
cluster_legacy_abac = cluster.legacy_abac
cluster_master_authorized_networks_config = cluster.master_authorized_networks_config
cluster_node_pools = cluster.node_pools
cluster_workload_certificates = cluster.workload_certificates
cluster_fleet = cluster.fleet
cluster_expire_time = cluster.expire_time
```

---


### Node_pool

Creates a node pool for a cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String |  | The parent (project, location, cluster name) where the node pool will be created. Specified in the format `projects/*/locations/*/clusters/*`. |
| `cluster_id` | String |  | Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field. |
| `node_pool` | String |  | Required. The node pool to create. |
| `project_id` | String |  | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field. |
| `zone` | String |  | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field. |
| `project_id` | String | ✅ | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field. |
| `cluster_id` | String | ✅ | Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field. |
| `zone` | String | ✅ | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_group_urls` | Vec<String> | Output only. The resource URLs of the [managed instance groups](https://cloud.google.com/compute/docs/instance-groups/creating-groups-of-managed-instances) associated with this node pool. During the node pool blue-green upgrade operation, the URLs contain both blue and green resources. |
| `config` | String | The node configuration of the pool. |
| `best_effort_provisioning` | String | Enable best effort provisioning for nodes |
| `placement_policy` | String | Specifies the node placement policy. |
| `locations` | Vec<String> | The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the NodePool's nodes should be located. If this value is unspecified during node pool creation, the [Cluster.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters#Cluster.FIELDS.locations) value will be used, instead. Warning: changing node pool locations will result in nodes being added and/or removed. |
| `status_message` | String | Output only. Deprecated. Use conditions instead. Additional information about the current status of this node pool instance, if available. |
| `initial_node_count` | i64 | The initial node count for the pool. You must ensure that your Compute Engine [resource quota](https://cloud.google.com/compute/quotas) is sufficient for this number of instances. You must also have available firewall and routes quota. |
| `max_pods_constraint` | String | The constraint on the maximum number of pods that can be run simultaneously on a node in the node pool. |
| `update_info` | String | Output only. Update info contains relevant information during a node pool update. |
| `autopilot_config` | String | Specifies the autopilot configuration for this node pool. This field is exclusively reserved for Cluster Autoscaler. |
| `version` | String | The version of Kubernetes running on this NodePool's nodes. If unspecified, it defaults as described [here](https://cloud.google.com/kubernetes-engine/versioning#specifying_node_version). |
| `conditions` | Vec<String> | Which conditions caused the current node pool state. |
| `name` | String | The name of the node pool. |
| `network_config` | String | Networking configuration for this NodePool. If specified, it overrides the cluster-level defaults. |
| `management` | String | NodeManagement configuration for this NodePool. |
| `status` | String | Output only. The status of the nodes in this pool instance. |
| `upgrade_settings` | String | Upgrade settings control disruption and speed of the upgrade. |
| `queued_provisioning` | String | Specifies the configuration of queued provisioning. |
| `self_link` | String | Output only. Server-defined URL for the resource. |
| `etag` | String | This checksum is computed by the server based on the value of node pool fields, and may be sent on update requests to ensure the client has an up-to-date value before proceeding. |
| `autoscaling` | String | Autoscaler configuration for this NodePool. Autoscaler is enabled only if a valid configuration is present. |
| `pod_ipv4_cidr_size` | i64 | Output only. The pod CIDR block size per node in this node pool. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create node_pool
node_pool = provider.container_api.Node_pool {
    project_id = "value"  # Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field.
    cluster_id = "value"  # Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field.
    zone = "value"  # Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field.
}

# Access node_pool outputs
node_pool_id = node_pool.id
node_pool_instance_group_urls = node_pool.instance_group_urls
node_pool_config = node_pool.config
node_pool_best_effort_provisioning = node_pool.best_effort_provisioning
node_pool_placement_policy = node_pool.placement_policy
node_pool_locations = node_pool.locations
node_pool_status_message = node_pool.status_message
node_pool_initial_node_count = node_pool.initial_node_count
node_pool_max_pods_constraint = node_pool.max_pods_constraint
node_pool_update_info = node_pool.update_info
node_pool_autopilot_config = node_pool.autopilot_config
node_pool_version = node_pool.version
node_pool_conditions = node_pool.conditions
node_pool_name = node_pool.name
node_pool_network_config = node_pool.network_config
node_pool_management = node_pool.management
node_pool_status = node_pool.status
node_pool_upgrade_settings = node_pool.upgrade_settings
node_pool_queued_provisioning = node_pool.queued_provisioning
node_pool_self_link = node_pool.self_link
node_pool_etag = node_pool.etag
node_pool_autoscaling = node_pool.autoscaling
node_pool_pod_ipv4_cidr_size = node_pool.pod_ipv4_cidr_size
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
operation_0 = provider.container_api.Operation {
    name = "value-0"
}
operation_1 = provider.container_api.Operation {
    name = "value-1"
}
operation_2 = provider.container_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.container_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Container_api Documentation](https://cloud.google.com/container_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
