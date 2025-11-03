# Container_api Service



**Resources**: 14

---

## Overview

The container_api service provides access to 14 resource types:

- [Zone](#zone) [R]
- [Node_pool](#node_pool) [CRUD]
- [Well_known](#well_known) [R]
- [Location](#location) [R]
- [Operation](#operation) [CR]
- [Cluster](#cluster) [CRUD]
- [Usable_subnetwork](#usable_subnetwork) [R]
- [Usable_subnetwork](#usable_subnetwork) [R]
- [Node_pool](#node_pool) [CRUD]
- [Operation](#operation) [CR]
- [Zone](#zone) [R]
- [Cluster](#cluster) [CRUD]
- [Location](#location) [R]
- [Well_known](#well_known) [R]

---

## Resources


### Zone

Returns configuration info about the Google Kubernetes Engine service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_cluster_version` | String | Version of Kubernetes the service deploys by default. |
| `channels` | Vec<String> | List of release channel configurations. |
| `valid_image_types` | Vec<String> | List of valid image types. |
| `default_image_type` | String | Default image type. |
| `valid_master_versions` | Vec<String> | List of valid master versions, in descending order. |
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
zone_default_cluster_version = zone.default_cluster_version
zone_channels = zone.channels
zone_valid_image_types = zone.valid_image_types
zone_default_image_type = zone.default_image_type
zone_valid_master_versions = zone.valid_master_versions
zone_valid_node_versions = zone.valid_node_versions
```

---


### Node_pool

Creates a node pool for a cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `node_pool` | String |  | Required. The node pool to create. |
| `zone` | String |  | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field. |
| `cluster_id` | String |  | Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field. |
| `parent` | String |  | The parent (project, location, cluster name) where the node pool will be created. Specified in the format `projects/*/locations/*/clusters/*`. |
| `project_id` | String |  | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field. |
| `project_id` | String | ✅ | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field. |
| `cluster_id` | String | ✅ | Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field. |
| `zone` | String | ✅ | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `autoscaling` | String | Autoscaler configuration for this NodePool. Autoscaler is enabled only if a valid configuration is present. |
| `max_pods_constraint` | String | The constraint on the maximum number of pods that can be run simultaneously on a node in the node pool. |
| `update_info` | String | Output only. Update info contains relevant information during a node pool update. |
| `placement_policy` | String | Specifies the node placement policy. |
| `conditions` | Vec<String> | Which conditions caused the current node pool state. |
| `locations` | Vec<String> | The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the NodePool's nodes should be located. If this value is unspecified during node pool creation, the [Cluster.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters#Cluster.FIELDS.locations) value will be used, instead. Warning: changing node pool locations will result in nodes being added and/or removed. |
| `etag` | String | This checksum is computed by the server based on the value of node pool fields, and may be sent on update requests to ensure the client has an up-to-date value before proceeding. |
| `network_config` | String | Networking configuration for this NodePool. If specified, it overrides the cluster-level defaults. |
| `pod_ipv4_cidr_size` | i64 | Output only. The pod CIDR block size per node in this node pool. |
| `autopilot_config` | String | Specifies the autopilot configuration for this node pool. This field is exclusively reserved for Cluster Autoscaler. |
| `config` | String | The node configuration of the pool. |
| `management` | String | NodeManagement configuration for this NodePool. |
| `best_effort_provisioning` | String | Enable best effort provisioning for nodes |
| `queued_provisioning` | String | Specifies the configuration of queued provisioning. |
| `self_link` | String | Output only. Server-defined URL for the resource. |
| `upgrade_settings` | String | Upgrade settings control disruption and speed of the upgrade. |
| `status` | String | Output only. The status of the nodes in this pool instance. |
| `instance_group_urls` | Vec<String> | Output only. The resource URLs of the [managed instance groups](https://cloud.google.com/compute/docs/instance-groups/creating-groups-of-managed-instances) associated with this node pool. During the node pool blue-green upgrade operation, the URLs contain both blue and green resources. |
| `status_message` | String | Output only. Deprecated. Use conditions instead. Additional information about the current status of this node pool instance, if available. |
| `initial_node_count` | i64 | The initial node count for the pool. You must ensure that your Compute Engine [resource quota](https://cloud.google.com/compute/quotas) is sufficient for this number of instances. You must also have available firewall and routes quota. |
| `version` | String | The version of Kubernetes running on this NodePool's nodes. If unspecified, it defaults as described [here](https://cloud.google.com/kubernetes-engine/versioning#specifying_node_version). |
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
    project_id = "value"  # Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field.
    cluster_id = "value"  # Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field.
    zone = "value"  # Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field.
}

# Access node_pool outputs
node_pool_id = node_pool.id
node_pool_autoscaling = node_pool.autoscaling
node_pool_max_pods_constraint = node_pool.max_pods_constraint
node_pool_update_info = node_pool.update_info
node_pool_placement_policy = node_pool.placement_policy
node_pool_conditions = node_pool.conditions
node_pool_locations = node_pool.locations
node_pool_etag = node_pool.etag
node_pool_network_config = node_pool.network_config
node_pool_pod_ipv4_cidr_size = node_pool.pod_ipv4_cidr_size
node_pool_autopilot_config = node_pool.autopilot_config
node_pool_config = node_pool.config
node_pool_management = node_pool.management
node_pool_best_effort_provisioning = node_pool.best_effort_provisioning
node_pool_queued_provisioning = node_pool.queued_provisioning
node_pool_self_link = node_pool.self_link
node_pool_upgrade_settings = node_pool.upgrade_settings
node_pool_status = node_pool.status
node_pool_instance_group_urls = node_pool.instance_group_urls
node_pool_status_message = node_pool.status_message
node_pool_initial_node_count = node_pool.initial_node_count
node_pool_version = node_pool.version
node_pool_name = node_pool.name
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
| `id_token_signing_alg_values_supported` | Vec<String> | supported ID Token signing Algorithms. |
| `cache_header` | String | For HTTP requests, this field is automatically extracted into the Cache-Control HTTP header. |
| `claims_supported` | Vec<String> | Supported claims. |
| `jwks_uri` | String | JSON Web Key uri. |
| `response_types_supported` | Vec<String> | Supported response types. |
| `subject_types_supported` | Vec<String> | Supported subject types. |
| `grant_types` | Vec<String> | Supported grant types. |


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
well_known_id_token_signing_alg_values_supported = well_known.id_token_signing_alg_values_supported
well_known_cache_header = well_known.cache_header
well_known_claims_supported = well_known.claims_supported
well_known_jwks_uri = well_known.jwks_uri
well_known_response_types_supported = well_known.response_types_supported
well_known_subject_types_supported = well_known.subject_types_supported
well_known_grant_types = well_known.grant_types
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
| `default_cluster_version` | String | Version of Kubernetes the service deploys by default. |
| `channels` | Vec<String> | List of release channel configurations. |
| `valid_image_types` | Vec<String> | List of valid image types. |
| `default_image_type` | String | Default image type. |
| `valid_master_versions` | Vec<String> | List of valid master versions, in descending order. |
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
location_default_cluster_version = location.default_cluster_version
location_channels = location.channels
location_valid_image_types = location.valid_image_types
location_default_image_type = location.default_image_type
location_valid_master_versions = location.valid_master_versions
location_valid_node_versions = location.valid_node_versions
```

---


### Operation

Cancels the specified operation.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name (project, location, operation id) of the operation to cancel. Specified in the format `projects/*/locations/*/operations/*`. |
| `zone` | String |  | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation resides. This field has been deprecated and replaced by the name field. |
| `project_id` | String |  | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field. |
| `operation_id` | String |  | Deprecated. The server-assigned `name` of the operation. This field has been deprecated and replaced by the name field. |
| `name` | String | ✅ | The name (project, location, operation id) of the operation to cancel. Specified in the format `projects/*/locations/*/operations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `zone` | String | Output only. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation is taking place. This field is deprecated, use location instead. |
| `detail` | String | Output only. Detailed operation progress, if available. |
| `start_time` | String | Output only. The time the operation started, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `status_message` | String | Output only. If an error has occurred, a textual description of the error. Deprecated. Use the field error instead. |
| `end_time` | String | Output only. The time the operation completed, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `operation_type` | String | Output only. The operation type. |
| `name` | String | Output only. The server-assigned ID for the operation. |
| `error` | String | The error result of the operation in case of failure. |
| `progress` | String | Output only. Progress information for an operation. |
| `self_link` | String | Output only. Server-defined URI for the operation. Example: `https://container.googleapis.com/v1alpha1/projects/123/locations/us-central1/operations/operation-123`. |
| `status` | String | Output only. The current status of the operation. |
| `nodepool_conditions` | Vec<String> | Which conditions caused the current node pool state. Deprecated. Use field error instead. |
| `location` | String | Output only. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in which the cluster resides. |
| `target_link` | String | Output only. Server-defined URI for the target of the operation. The format of this is a URI to the resource being modified (such as a cluster, node pool, or node). For node pool repairs, there may be multiple nodes being repaired, but only one will be the target. Examples: - ## `https://container.googleapis.com/v1/projects/123/locations/us-central1/clusters/my-cluster` ## `https://container.googleapis.com/v1/projects/123/zones/us-central1-c/clusters/my-cluster/nodePools/my-np` `https://container.googleapis.com/v1/projects/123/zones/us-central1-c/clusters/my-cluster/nodePools/my-np/node/my-node` |
| `cluster_conditions` | Vec<String> | Which conditions caused the current cluster state. Deprecated. Use field error instead. |


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
operation_zone = operation.zone
operation_detail = operation.detail
operation_start_time = operation.start_time
operation_status_message = operation.status_message
operation_end_time = operation.end_time
operation_operation_type = operation.operation_type
operation_name = operation.name
operation_error = operation.error
operation_progress = operation.progress
operation_self_link = operation.self_link
operation_status = operation.status
operation_nodepool_conditions = operation.nodepool_conditions
operation_location = operation.location
operation_target_link = operation.target_link
operation_cluster_conditions = operation.cluster_conditions
```

---


### Cluster

Creates a cluster, consisting of the specified number and type of Google Compute Engine instances. By default, the cluster is created in the project's [default network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks). One firewall is added for the cluster. After cluster creation, the kubelet creates routes for each node to allow the containers on that node to communicate with all other instances in the cluster. Finally, an entry is added to the project's global metadata indicating which CIDR range the cluster is using.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `zone` | String |  | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field. |
| `parent` | String |  | The parent (project and location) where the cluster will be created. Specified in the format `projects/*/locations/*`. |
| `cluster` | String |  | Required. A [cluster resource](https://cloud.google.com/container-engine/reference/rest/v1/projects.locations.clusters) |
| `project_id` | String |  | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field. |
| `project_id` | String | ✅ | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field. |
| `zone` | String | ✅ | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `anonymous_authentication_config` | String | Configuration for limiting anonymous access to all endpoints except the health checks. |
| `current_node_version` | String | Output only. Deprecated, use [NodePools.version](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools) instead. The current version of the node software components. If they are currently at multiple versions because they're in the process of being upgraded, this reflects the minimum version of all nodes. |
| `logging_config` | String | Logging configuration for the cluster. |
| `authenticator_groups_config` | String | Configuration controlling RBAC group membership information. |
| `conditions` | Vec<String> | Which conditions caused the current cluster state. |
| `description` | String | An optional description of this cluster. |
| `node_ipv4_cidr_size` | i64 | Output only. The size of the address space on each node for hosting containers. This is provisioned from within the `container_ipv4_cidr` range. This field will only be set when cluster is in route-based network mode. |
| `initial_node_count` | i64 | The number of nodes to create in this cluster. You must ensure that your Compute Engine [resource quota](https://cloud.google.com/compute/quotas) is sufficient for this number of instances. You must also have available firewall and routes quota. For requests, this field should only be used in lieu of a "node_pool" object, since this configuration (along with the "node_config") will be used to create a "NodePool" object with an auto-generated name. Do not use this and a node_pool at the same time. This field is deprecated, use node_pool.initial_node_count instead. |
| `tpu_ipv4_cidr_block` | String | Output only. The IP address range of the Cloud TPUs in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `1.2.3.4/29`). This field is deprecated due to the deprecation of 2VM TPU. The end of life date for 2VM TPU is 2025-04-25. |
| `release_channel` | String | Release channel configuration. If left unspecified on cluster creation and a version is specified, the cluster is enrolled in the most mature release channel where the version is available (first checking STABLE, then REGULAR, and finally RAPID). Otherwise, if no release channel configuration and no version is specified, the cluster is enrolled in the REGULAR channel with its default version. |
| `workload_identity_config` | String | Configuration for the use of Kubernetes Service Accounts in IAM policies. |
| `monitoring_service` | String | The monitoring service the cluster should use to write metrics. Currently available options: * `monitoring.googleapis.com/kubernetes` - The Cloud Monitoring service with a Kubernetes-native resource model * `monitoring.googleapis.com` - The legacy Cloud Monitoring service (no longer available as of GKE 1.15). * `none` - No metrics will be exported from the cluster. If left as an empty string,`monitoring.googleapis.com/kubernetes` will be used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions. |
| `addons_config` | String | Configurations for the various addons available to run in the cluster. |
| `node_pools` | Vec<String> | The node pools associated with this cluster. This field should not be set if "node_config" or "initial_node_count" are specified. |
| `self_link` | String | Output only. Server-defined URL for the resource. |
| `vertical_pod_autoscaling` | String | Cluster-level Vertical Pod Autoscaling configuration. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `network_config` | String | Configuration for cluster networking. |
| `node_pool_auto_config` | String | Node pool configs that apply to all auto-provisioned node pools in autopilot clusters and node auto-provisioning enabled clusters. |
| `resource_usage_export_config` | String | Configuration for exporting resource usages. Resource usage export is disabled when this config is unspecified. |
| `endpoint` | String | Output only. The IP address of this cluster's master endpoint. The endpoint can be accessed from the internet at `https://username:password@endpoint/`. See the `masterAuth` property of this resource for username and password information. |
| `instance_group_urls` | Vec<String> | Output only. Deprecated. Use node_pools.instance_group_urls. |
| `initial_cluster_version` | String | The initial Kubernetes version for this cluster. Valid versions are those found in validMasterVersions returned by getServerConfig. The version can be upgraded over time; such upgrades are reflected in currentMasterVersion and currentNodeVersion. Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - "latest": picks the highest valid Kubernetes version - "1.X": picks the highest valid patch+gke.N patch in the 1.X version - "1.X.Y": picks the highest valid gke.N patch in the 1.X.Y version - "1.X.Y-gke.N": picks an explicit Kubernetes version - "","-": picks the default Kubernetes version |
| `private_cluster_config` | String | Configuration for private cluster. |
| `enable_k8s_beta_apis` | String | Beta APIs Config |
| `autopilot` | String | Autopilot configuration for the cluster. |
| `status` | String | Output only. The current status of this cluster. |
| `enterprise_config` | String | GKE Enterprise Configuration. Deprecated: GKE Enterprise features are now available without an Enterprise tier. |
| `logging_service` | String | The logging service the cluster should use to write logs. Currently available options: * `logging.googleapis.com/kubernetes` - The Cloud Logging service with a Kubernetes-native resource model * `logging.googleapis.com` - The legacy Cloud Logging service (no longer available as of GKE 1.15). * `none` - no logs will be exported from the cluster. If left as an empty string,`logging.googleapis.com/kubernetes` will be used for GKE 1.14+ or `logging.googleapis.com` for earlier versions. |
| `status_message` | String | Output only. Deprecated. Use conditions instead. Additional information about the current status of this cluster, if available. |
| `shielded_nodes` | String | Shielded Nodes configuration. |
| `subnetwork` | String | The name of the Google Compute Engine [subnetwork](https://cloud.google.com/compute/docs/subnetworks) to which the cluster is connected. |
| `alpha_cluster_feature_gates` | Vec<String> | The list of user specified Kubernetes feature gates. Each string represents the activation status of a feature gate (e.g. "featureX=true" or "featureX=false") |
| `cost_management_config` | String | Configuration for the fine-grained cost management feature. |
| `expire_time` | String | Output only. The time the cluster will be automatically deleted in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `services_ipv4_cidr` | String | Output only. The IP address range of the Kubernetes services in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `1.2.3.4/29`). Service addresses are typically put in the last `/16` from the container CIDR. |
| `id` | String | Output only. Unique id for the cluster. |
| `current_master_version` | String | Output only. The current software version of the master endpoint. |
| `identity_service_config` | String | Configuration for Identity Service component. |
| `database_encryption` | String | Configuration of etcd encryption. |
| `user_managed_keys_config` | String | The Custom keys configuration for the cluster. |
| `confidential_nodes` | String | Configuration of Confidential Nodes. All the nodes in the cluster will be Confidential VM once enabled. |
| `master_auth` | String | The authentication information for accessing the master endpoint. If unspecified, the defaults are used: For clusters before v1.12, if master_auth is unspecified, `username` will be set to "admin", a random password will be generated, and a client certificate will be issued. |
| `name` | String | The name of this cluster. The name must be unique within this project and location (e.g. zone or region), and can be up to 40 characters with the following restrictions: * Lowercase letters, numbers, and hyphens only. * Must start with a letter. * Must end with a number or a letter. |
| `gke_auto_upgrade_config` | String | Configuration for GKE auto upgrades. |
| `location` | String | Output only. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in which the cluster resides. |
| `parent_product_config` | String | The configuration of the parent product of the cluster. This field is used by Google internal products that are built on top of the GKE cluster and take the ownership of the cluster. |
| `resource_labels` | HashMap<String, String> | The resource labels for the cluster to use to annotate any related Google Compute Engine resources. |
| `fleet` | String | Fleet information for the cluster. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `label_fingerprint` | String | The fingerprint of the set of labels for this cluster. |
| `network` | String | The name of the Google Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the cluster is connected. If left unspecified, the `default` network will be used. |
| `node_config` | String | Parameters used in creating the cluster's nodes. For requests, this field should only be used in lieu of a "node_pool" object, since this configuration (along with the "initial_node_count") will be used to create a "NodePool" object with an auto-generated name. Do not use this and a node_pool at the same time. For responses, this field will be populated with the node configuration of the first node pool. (For configuration of each node pool, see `node_pool.config`) If unspecified, the defaults are used. This field is deprecated, use node_pool.config instead. |
| `cluster_ipv4_cidr` | String | The IP address range of the container pods in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`). Leave blank to have one automatically chosen or specify a `/14` block in `10.0.0.0/8`. |
| `etag` | String | This checksum is computed by the server based on the value of cluster fields, and may be sent on update requests to ensure the client has an up-to-date value before proceeding. |
| `pod_autoscaling` | String | The config for pod autoscaling. |
| `node_pool_defaults` | String | Default NodePool settings for the entire cluster. These settings are overridden if specified on the specific NodePool object. |
| `rbac_binding_config` | String | RBACBindingConfig allows user to restrict ClusterRoleBindings an RoleBindings that can be created. |
| `zone` | String | Output only. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field is deprecated, use location instead. |
| `autoscaling` | String | Cluster-level autoscaling configuration. |
| `secret_manager_config` | String | Secret CSI driver configuration. |
| `default_max_pods_constraint` | String | The default constraint on the maximum number of pods that can be run simultaneously on a node in the node pool of this cluster. Only honored if cluster created with IP Alias support. |
| `locations` | Vec<String> | The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the cluster's nodes should be located. This field provides a default value if [NodePool.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.FIELDS.locations) are not specified during node pool creation. Warning: changing cluster locations will update the [NodePool.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.FIELDS.locations) of all node pools and will result in nodes being added and/or removed. |
| `ip_allocation_policy` | String | Configuration for cluster IP allocation. |
| `monitoring_config` | String | Monitoring configuration for the cluster. |
| `current_node_count` | i64 | Output only. The number of nodes currently in the cluster. Deprecated. Call Kubernetes API directly to retrieve node information. |
| `maintenance_policy` | String | Configure the maintenance policy for this cluster. |
| `mesh_certificates` | String | Configuration for issuance of mTLS keys and certificates to Kubernetes pods. |
| `security_posture_config` | String | Enable/Disable Security Posture API features for the cluster. |
| `network_policy` | String | Configuration options for the NetworkPolicy feature. |
| `compliance_posture_config` | String | Enable/Disable Compliance Posture features for the cluster. |
| `enable_kubernetes_alpha` | bool | Kubernetes alpha features are enabled on this cluster. This includes alpha API groups (e.g. v1alpha1) and features that may not be production ready in the kubernetes version of the master and nodes. The cluster has no SLA for uptime and master/node upgrades are disabled. Alpha enabled clusters are automatically deleted thirty days after creation. |
| `binary_authorization` | String | Configuration for Binary Authorization. |
| `enable_tpu` | bool | Enable the ability to use Cloud TPUs in this cluster. This field is deprecated due to the deprecation of 2VM TPU. The end of life date for 2VM TPU is 2025-04-25. |
| `legacy_abac` | String | Configuration for the legacy ABAC authorization mode. |
| `notification_config` | String | Notification configuration of the cluster. |
| `master_authorized_networks_config` | String | The configuration options for master authorized networks feature. Deprecated: Use ControlPlaneEndpointsConfig.IPEndpointsConfig.authorized_networks_config instead. |
| `create_time` | String | Output only. The time the cluster was created, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `control_plane_endpoints_config` | String | Configuration for all cluster's control plane endpoints. |


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
cluster_anonymous_authentication_config = cluster.anonymous_authentication_config
cluster_current_node_version = cluster.current_node_version
cluster_logging_config = cluster.logging_config
cluster_authenticator_groups_config = cluster.authenticator_groups_config
cluster_conditions = cluster.conditions
cluster_description = cluster.description
cluster_node_ipv4_cidr_size = cluster.node_ipv4_cidr_size
cluster_initial_node_count = cluster.initial_node_count
cluster_tpu_ipv4_cidr_block = cluster.tpu_ipv4_cidr_block
cluster_release_channel = cluster.release_channel
cluster_workload_identity_config = cluster.workload_identity_config
cluster_monitoring_service = cluster.monitoring_service
cluster_addons_config = cluster.addons_config
cluster_node_pools = cluster.node_pools
cluster_self_link = cluster.self_link
cluster_vertical_pod_autoscaling = cluster.vertical_pod_autoscaling
cluster_satisfies_pzi = cluster.satisfies_pzi
cluster_network_config = cluster.network_config
cluster_node_pool_auto_config = cluster.node_pool_auto_config
cluster_resource_usage_export_config = cluster.resource_usage_export_config
cluster_endpoint = cluster.endpoint
cluster_instance_group_urls = cluster.instance_group_urls
cluster_initial_cluster_version = cluster.initial_cluster_version
cluster_private_cluster_config = cluster.private_cluster_config
cluster_enable_k8s_beta_apis = cluster.enable_k8s_beta_apis
cluster_autopilot = cluster.autopilot
cluster_status = cluster.status
cluster_enterprise_config = cluster.enterprise_config
cluster_logging_service = cluster.logging_service
cluster_status_message = cluster.status_message
cluster_shielded_nodes = cluster.shielded_nodes
cluster_subnetwork = cluster.subnetwork
cluster_alpha_cluster_feature_gates = cluster.alpha_cluster_feature_gates
cluster_cost_management_config = cluster.cost_management_config
cluster_expire_time = cluster.expire_time
cluster_services_ipv4_cidr = cluster.services_ipv4_cidr
cluster_id = cluster.id
cluster_current_master_version = cluster.current_master_version
cluster_identity_service_config = cluster.identity_service_config
cluster_database_encryption = cluster.database_encryption
cluster_user_managed_keys_config = cluster.user_managed_keys_config
cluster_confidential_nodes = cluster.confidential_nodes
cluster_master_auth = cluster.master_auth
cluster_name = cluster.name
cluster_gke_auto_upgrade_config = cluster.gke_auto_upgrade_config
cluster_location = cluster.location
cluster_parent_product_config = cluster.parent_product_config
cluster_resource_labels = cluster.resource_labels
cluster_fleet = cluster.fleet
cluster_satisfies_pzs = cluster.satisfies_pzs
cluster_label_fingerprint = cluster.label_fingerprint
cluster_network = cluster.network
cluster_node_config = cluster.node_config
cluster_cluster_ipv4_cidr = cluster.cluster_ipv4_cidr
cluster_etag = cluster.etag
cluster_pod_autoscaling = cluster.pod_autoscaling
cluster_node_pool_defaults = cluster.node_pool_defaults
cluster_rbac_binding_config = cluster.rbac_binding_config
cluster_zone = cluster.zone
cluster_autoscaling = cluster.autoscaling
cluster_secret_manager_config = cluster.secret_manager_config
cluster_default_max_pods_constraint = cluster.default_max_pods_constraint
cluster_locations = cluster.locations
cluster_ip_allocation_policy = cluster.ip_allocation_policy
cluster_monitoring_config = cluster.monitoring_config
cluster_current_node_count = cluster.current_node_count
cluster_maintenance_policy = cluster.maintenance_policy
cluster_mesh_certificates = cluster.mesh_certificates
cluster_security_posture_config = cluster.security_posture_config
cluster_network_policy = cluster.network_policy
cluster_compliance_posture_config = cluster.compliance_posture_config
cluster_enable_kubernetes_alpha = cluster.enable_kubernetes_alpha
cluster_binary_authorization = cluster.binary_authorization
cluster_enable_tpu = cluster.enable_tpu
cluster_legacy_abac = cluster.legacy_abac
cluster_notification_config = cluster.notification_config
cluster_master_authorized_networks_config = cluster.master_authorized_networks_config
cluster_create_time = cluster.create_time
cluster_control_plane_endpoints_config = cluster.control_plane_endpoints_config
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


### Node_pool

Creates a node pool for a cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `zone` | String |  | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field. |
| `parent` | String |  | The parent (project, location, cluster name) where the node pool will be created. Specified in the format `projects/*/locations/*/clusters/*`. |
| `node_pool` | String |  | Required. The node pool to create. |
| `project_id` | String |  | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field. |
| `cluster_id` | String |  | Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field. |
| `cluster_id` | String | ✅ | Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field. |
| `zone` | String | ✅ | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field. |
| `project_id` | String | ✅ | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `conditions` | Vec<String> | Which conditions caused the current node pool state. |
| `instance_group_urls` | Vec<String> | Output only. The resource URLs of the [managed instance groups](https://cloud.google.com/compute/docs/instance-groups/creating-groups-of-managed-instances) associated with this node pool. During the node pool blue-green upgrade operation, the URLs contain both blue and green resources. |
| `network_config` | String | Networking configuration for this NodePool. If specified, it overrides the cluster-level defaults. |
| `name` | String | The name of the node pool. |
| `update_info` | String | Output only. Update info contains relevant information during a node pool update. |
| `pod_ipv4_cidr_size` | i64 | Output only. The pod CIDR block size per node in this node pool. |
| `etag` | String | This checksum is computed by the server based on the value of node pool fields, and may be sent on update requests to ensure the client has an up-to-date value before proceeding. |
| `self_link` | String | Output only. Server-defined URL for the resource. |
| `management` | String | NodeManagement configuration for this NodePool. |
| `status_message` | String | Output only. Deprecated. Use conditions instead. Additional information about the current status of this node pool instance, if available. |
| `autoscaling` | String | Autoscaler configuration for this NodePool. Autoscaler is enabled only if a valid configuration is present. |
| `best_effort_provisioning` | String | Enable best effort provisioning for nodes |
| `config` | String | The node configuration of the pool. |
| `locations` | Vec<String> | The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the NodePool's nodes should be located. If this value is unspecified during node pool creation, the [Cluster.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters#Cluster.FIELDS.locations) value will be used, instead. Warning: changing node pool locations will result in nodes being added and/or removed. |
| `initial_node_count` | i64 | The initial node count for the pool. You must ensure that your Compute Engine [resource quota](https://cloud.google.com/compute/quotas) is sufficient for this number of instances. You must also have available firewall and routes quota. |
| `max_pods_constraint` | String | The constraint on the maximum number of pods that can be run simultaneously on a node in the node pool. |
| `placement_policy` | String | Specifies the node placement policy. |
| `upgrade_settings` | String | Upgrade settings control disruption and speed of the upgrade. |
| `queued_provisioning` | String | Specifies the configuration of queued provisioning. |
| `status` | String | Output only. The status of the nodes in this pool instance. |
| `version` | String | The version of Kubernetes running on this NodePool's nodes. If unspecified, it defaults as described [here](https://cloud.google.com/kubernetes-engine/versioning#specifying_node_version). |
| `autopilot_config` | String | Specifies the autopilot configuration for this node pool. This field is exclusively reserved for Cluster Autoscaler. |


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
    zone = "value"  # Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field.
    project_id = "value"  # Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field.
}

# Access node_pool outputs
node_pool_id = node_pool.id
node_pool_conditions = node_pool.conditions
node_pool_instance_group_urls = node_pool.instance_group_urls
node_pool_network_config = node_pool.network_config
node_pool_name = node_pool.name
node_pool_update_info = node_pool.update_info
node_pool_pod_ipv4_cidr_size = node_pool.pod_ipv4_cidr_size
node_pool_etag = node_pool.etag
node_pool_self_link = node_pool.self_link
node_pool_management = node_pool.management
node_pool_status_message = node_pool.status_message
node_pool_autoscaling = node_pool.autoscaling
node_pool_best_effort_provisioning = node_pool.best_effort_provisioning
node_pool_config = node_pool.config
node_pool_locations = node_pool.locations
node_pool_initial_node_count = node_pool.initial_node_count
node_pool_max_pods_constraint = node_pool.max_pods_constraint
node_pool_placement_policy = node_pool.placement_policy
node_pool_upgrade_settings = node_pool.upgrade_settings
node_pool_queued_provisioning = node_pool.queued_provisioning
node_pool_status = node_pool.status
node_pool_version = node_pool.version
node_pool_autopilot_config = node_pool.autopilot_config
```

---


### Operation

Cancels the specified operation.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project_id` | String |  | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field. |
| `operation_id` | String |  | Deprecated. The server-assigned `name` of the operation. This field has been deprecated and replaced by the name field. |
| `name` | String |  | The name (project, location, operation id) of the operation to cancel. Specified in the format `projects/*/locations/*/operations/*`. |
| `zone` | String |  | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation resides. This field has been deprecated and replaced by the name field. |
| `name` | String | ✅ | The name (project, location, operation id) of the operation to cancel. Specified in the format `projects/*/locations/*/operations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `progress` | String | Output only. Progress information for an operation. |
| `start_time` | String | Output only. The time the operation started, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `name` | String | Output only. The server-assigned ID for the operation. |
| `status` | String | Output only. The current status of the operation. |
| `self_link` | String | Output only. Server-defined URI for the operation. Example: `https://container.googleapis.com/v1alpha1/projects/123/locations/us-central1/operations/operation-123`. |
| `cluster_conditions` | Vec<String> | Which conditions caused the current cluster state. Deprecated. Use field error instead. |
| `nodepool_conditions` | Vec<String> | Which conditions caused the current node pool state. Deprecated. Use field error instead. |
| `end_time` | String | Output only. The time the operation completed, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `operation_type` | String | Output only. The operation type. |
| `target_link` | String | Output only. Server-defined URI for the target of the operation. The format of this is a URI to the resource being modified (such as a cluster, node pool, or node). For node pool repairs, there may be multiple nodes being repaired, but only one will be the target. Examples: - ## `https://container.googleapis.com/v1/projects/123/locations/us-central1/clusters/my-cluster` ## `https://container.googleapis.com/v1/projects/123/zones/us-central1-c/clusters/my-cluster/nodePools/my-np` `https://container.googleapis.com/v1/projects/123/zones/us-central1-c/clusters/my-cluster/nodePools/my-np/node/my-node` |
| `error` | String | The error result of the operation in case of failure. |
| `location` | String | Output only. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in which the cluster resides. |
| `status_message` | String | Output only. If an error has occurred, a textual description of the error. Deprecated. Use field error instead. |
| `detail` | String | Output only. Detailed operation progress, if available. |
| `zone` | String | Output only. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation is taking place. This field is deprecated, use location instead. |


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
operation_progress = operation.progress
operation_start_time = operation.start_time
operation_name = operation.name
operation_status = operation.status
operation_self_link = operation.self_link
operation_cluster_conditions = operation.cluster_conditions
operation_nodepool_conditions = operation.nodepool_conditions
operation_end_time = operation.end_time
operation_operation_type = operation.operation_type
operation_target_link = operation.target_link
operation_error = operation.error
operation_location = operation.location
operation_status_message = operation.status_message
operation_detail = operation.detail
operation_zone = operation.zone
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
| `valid_node_versions` | Vec<String> | List of valid node upgrade target versions, in descending order. |
| `default_cluster_version` | String | Version of Kubernetes the service deploys by default. |
| `default_image_type` | String | Default image type. |
| `channels` | Vec<String> | List of release channel configurations. |
| `valid_master_versions` | Vec<String> | List of valid master versions, in descending order. |
| `windows_version_maps` | HashMap<String, String> | Maps of Kubernetes version and supported Windows server versions. |
| `valid_image_types` | Vec<String> | List of valid image types. |


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
zone_valid_node_versions = zone.valid_node_versions
zone_default_cluster_version = zone.default_cluster_version
zone_default_image_type = zone.default_image_type
zone_channels = zone.channels
zone_valid_master_versions = zone.valid_master_versions
zone_windows_version_maps = zone.windows_version_maps
zone_valid_image_types = zone.valid_image_types
```

---


### Cluster

Creates a cluster, consisting of the specified number and type of Google Compute Engine instances. By default, the cluster is created in the project's [default network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks). One firewall is added for the cluster. After cluster creation, the kubelet creates routes for each node to allow the containers on that node to communicate with all other instances in the cluster. Finally, an entry is added to the project's global metadata indicating which CIDR range the cluster is using.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cluster` | String |  | Required. A [cluster resource](https://cloud.google.com/container-engine/reference/rest/v1beta1/projects.locations.clusters) |
| `project_id` | String |  | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field. |
| `parent` | String |  | The parent (project and location) where the cluster will be created. Specified in the format `projects/*/locations/*`. |
| `zone` | String |  | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field. |
| `project_id` | String | ✅ | Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field. |
| `zone` | String | ✅ | Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parent_product_config` | String | The configuration of the parent product of the cluster. This field is used by Google internal products that are built on top of the GKE cluster and take the ownership of the cluster. |
| `fleet` | String | Fleet information for the cluster. |
| `node_config` | String | Parameters used in creating the cluster's nodes. For requests, this field should only be used in lieu of a "node_pool" object, since this configuration (along with the "initial_node_count") will be used to create a "NodePool" object with an auto-generated name. Do not use this and a node_pool at the same time. For responses, this field will be populated with the node configuration of the first node pool. (For configuration of each node pool, see `node_pool.config`) If unspecified, the defaults are used. This field is deprecated, use node_pool.config instead. |
| `self_link` | String | Output only. Server-defined URL for the resource. |
| `name` | String | The name of this cluster. The name must be unique within this project and location (e.g. zone or region), and can be up to 40 characters with the following restrictions: * Lowercase letters, numbers, and hyphens only. * Must start with a letter. * Must end with a number or a letter. |
| `protect_config` | String | Deprecated: Use SecurityPostureConfig instead. Enable/Disable Protect API features for the cluster. |
| `alpha_cluster_feature_gates` | Vec<String> | The list of user specified Kubernetes feature gates. Each string represents the activation status of a feature gate (e.g. "featureX=true" or "featureX=false") |
| `zone` | String | Output only. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field is deprecated, use location instead. |
| `autopilot` | String | Autopilot configuration for the cluster. |
| `subnetwork` | String | The name of the Google Compute Engine [subnetwork](https://cloud.google.com/compute/docs/subnetworks) to which the cluster is connected. On output this shows the subnetwork ID instead of the name. |
| `control_plane_endpoints_config` | String | Configuration for all cluster's control plane endpoints. |
| `instance_group_urls` | Vec<String> | Output only. Deprecated. Use node_pools.instance_group_urls. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `current_node_version` | String | Output only. Deprecated, use [NodePool.version](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/projects.locations.clusters.nodePools) instead. The current version of the node software components. If they are currently at multiple versions because they're in the process of being upgraded, this reflects the minimum version of all nodes. |
| `default_max_pods_constraint` | String | The default constraint on the maximum number of pods that can be run simultaneously on a node in the node pool of this cluster. Only honored if cluster created with IP Alias support. |
| `master_authorized_networks_config` | String | The configuration options for master authorized networks feature. Deprecated: Use ControlPlaneEndpointsConfig.IPEndpointsConfig.authorized_networks_config instead. |
| `node_pool_defaults` | String | Default NodePool settings for the entire cluster. These settings are overridden if specified on the specific NodePool object. |
| `cost_management_config` | String | Configuration for the fine-grained cost management feature. |
| `master_ipv4_cidr_block` | String | The IP prefix in CIDR notation to use for the hosted master network. This prefix will be used for assigning private IP addresses to the master or set of masters, as well as the ILB VIP. This field is deprecated, use private_cluster_config.master_ipv4_cidr_block instead. |
| `legacy_abac` | String | Configuration for the legacy ABAC authorization mode. |
| `resource_labels` | HashMap<String, String> | The resource labels for the cluster to use to annotate any related Google Compute Engine resources. |
| `endpoint` | String | Output only. The IP address of this cluster's master endpoint. The endpoint can be accessed from the internet at `https://username:password@endpoint/`. See the `masterAuth` property of this resource for username and password information. |
| `private_cluster` | bool | If this is a private cluster setup. Private clusters are clusters that, by default have no external IP addresses on the nodes and where nodes and the master communicate over private IP addresses. This field is deprecated, use private_cluster_config.enable_private_nodes instead. |
| `locations` | Vec<String> | The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the cluster's nodes should be located. This field provides a default value if [NodePool.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.FIELDS.locations) are not specified during node pool creation. Warning: changing cluster locations will update the [NodePool.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.FIELDS.locations) of all node pools and will result in nodes being added and/or removed. |
| `mesh_certificates` | String | Configuration for issuance of mTLS keys and certificates to Kubernetes pods. |
| `services_ipv4_cidr` | String | Output only. The IP address range of the Kubernetes services in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `1.2.3.4/29`). Service addresses are typically put in the last `/16` from the container CIDR. |
| `private_cluster_config` | String | Configuration for private cluster. |
| `ip_allocation_policy` | String | Configuration for cluster IP allocation. |
| `authenticator_groups_config` | String | Configuration controlling RBAC group membership information. |
| `monitoring_config` | String | Monitoring configuration for the cluster. |
| `rbac_binding_config` | String | RBACBindingConfig allows user to restrict ClusterRoleBindings an RoleBindings that can be created. |
| `resource_usage_export_config` | String | Configuration for exporting resource usages. Resource usage export is disabled when this config unspecified. |
| `location` | String | Output only. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in which the cluster resides. |
| `enable_kubernetes_alpha` | bool | Kubernetes alpha features are enabled on this cluster. This includes alpha API groups (e.g. v1beta1) and features that may not be production ready in the kubernetes version of the master and nodes. The cluster has no SLA for uptime and master/node upgrades are disabled. Alpha enabled clusters are automatically deleted thirty days after creation. |
| `current_master_version` | String | Output only. The current software version of the master endpoint. |
| `anonymous_authentication_config` | String | Configuration for limiting anonymous access to all endpoints except the health checks. |
| `logging_service` | String | The logging service the cluster should use to write logs. Currently available options: * `logging.googleapis.com/kubernetes` - The Cloud Logging service with a Kubernetes-native resource model * `logging.googleapis.com` - The legacy Cloud Logging service (no longer available as of GKE 1.15). * `none` - no logs will be exported from the cluster. If left as an empty string,`logging.googleapis.com/kubernetes` will be used for GKE 1.14+ or `logging.googleapis.com` for earlier versions. |
| `tpu_config` | String | Configuration for Cloud TPU support; This field is deprecated due to the deprecation of 2VM TPU. The end of life date for 2VM TPU is 2025-04-25. |
| `monitoring_service` | String | The monitoring service the cluster should use to write metrics. Currently available options: * `monitoring.googleapis.com/kubernetes` - The Cloud Monitoring service with a Kubernetes-native resource model * `monitoring.googleapis.com` - The legacy Cloud Monitoring service (no longer available as of GKE 1.15). * `none` - No metrics will be exported from the cluster. If left as an empty string,`monitoring.googleapis.com/kubernetes` will be used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions. |
| `initial_node_count` | i64 | The number of nodes to create in this cluster. You must ensure that your Compute Engine [resource quota](https://cloud.google.com/compute/quotas) is sufficient for this number of instances. You must also have available firewall and routes quota. For requests, this field should only be used in lieu of a "node_pool" object, since this configuration (along with the "node_config") will be used to create a "NodePool" object with an auto-generated name. Do not use this and a node_pool at the same time. This field is deprecated, use node_pool.initial_node_count instead. |
| `conditions` | Vec<String> | Which conditions caused the current cluster state. |
| `initial_cluster_version` | String | The initial Kubernetes version for this cluster. Valid versions are those found in validMasterVersions returned by getServerConfig. The version can be upgraded over time; such upgrades are reflected in currentMasterVersion and currentNodeVersion. Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - "latest": picks the highest valid Kubernetes version - "1.X": picks the highest valid patch+gke.N patch in the 1.X version - "1.X.Y": picks the highest valid gke.N patch in the 1.X.Y version - "1.X.Y-gke.N": picks an explicit Kubernetes version - "","-": picks the default Kubernetes version |
| `gke_auto_upgrade_config` | String | Configuration for GKE auto upgrades. |
| `id` | String | Output only. Unique id for the cluster. |
| `cluster_telemetry` | String | Telemetry integration for the cluster. |
| `expire_time` | String | Output only. The time the cluster will be automatically deleted in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `status` | String | Output only. The current status of this cluster. |
| `node_pools` | Vec<String> | The node pools associated with this cluster. This field should not be set if "node_config" or "initial_node_count" are specified. |
| `secret_manager_config` | String | Secret CSI driver configuration. |
| `workload_alts_config` | String | Configuration for direct-path (via ALTS) with workload identity. This feature is not officially supported for external customers in Kubernetes Engine when using Workload Identity. |
| `enable_k8s_beta_apis` | String | Kubernetes open source beta apis enabled on the cluster. Only beta apis. |
| `user_managed_keys_config` | String | The Custom keys configuration for the cluster. |
| `pod_security_policy_config` | String | Configuration for the PodSecurityPolicy feature. |
| `master_auth` | String | The authentication information for accessing the master endpoint. If unspecified, the defaults are used: For clusters before v1.12, if master_auth is unspecified, `username` will be set to "admin", a random password will be generated, and a client certificate will be issued. |
| `network_config` | String | Configuration for cluster networking. |
| `compliance_posture_config` | String | Enable/Disable Compliance Posture features for the cluster. |
| `release_channel` | String | Release channel configuration. If left unspecified on cluster creation and a version is specified, the cluster is enrolled in the most mature release channel where the version is available (first checking STABLE, then REGULAR, and finally RAPID). Otherwise, if no release channel configuration and no version is specified, the cluster is enrolled in the REGULAR channel with its default version. |
| `cluster_ipv4_cidr` | String | The IP address range of the container pods in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`). Leave blank to have one automatically chosen or specify a `/14` block in `10.0.0.0/8`. |
| `tpu_ipv4_cidr_block` | String | Output only. The IP address range of the Cloud TPUs in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `1.2.3.4/29`). This field is deprecated due to the deprecation of 2VM TPU. The end of life date for 2VM TPU is 2025-04-25. |
| `description` | String | An optional description of this cluster. |
| `etag` | String | This checksum is computed by the server based on the value of cluster fields, and may be sent on update requests to ensure the client has an up-to-date value before proceeding. |
| `pod_autoscaling` | String | The config for pod autoscaling. |
| `workload_identity_config` | String | Configuration for the use of Kubernetes Service Accounts in IAM policies. |
| `autoscaling` | String | Cluster-level autoscaling configuration. |
| `binary_authorization` | String | Configuration for Binary Authorization. |
| `shielded_nodes` | String | Shielded Nodes configuration. |
| `logging_config` | String | Logging configuration for the cluster. |
| `create_time` | String | Output only. The time the cluster was created, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `identity_service_config` | String | Configuration for Identity Service component. |
| `label_fingerprint` | String | The fingerprint of the set of labels for this cluster. |
| `maintenance_policy` | String | Configure the maintenance policy for this cluster. |
| `status_message` | String | Output only. Deprecated. Use conditions instead. Additional information about the current status of this cluster, if available. |
| `network` | String | The name of the Google Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the cluster is connected. If left unspecified, the `default` network will be used. On output this shows the network ID instead of the name. |
| `confidential_nodes` | String | Configuration of Confidential Nodes. All the nodes in the cluster will be Confidential VM once enabled. |
| `enable_tpu` | bool | Enable the ability to use Cloud TPUs in this cluster. This field is deprecated, use tpu_config.enabled instead. This field is deprecated due to the deprecation of 2VM TPU. The end of life date for 2VM TPU is 2025-04-25. |
| `node_ipv4_cidr_size` | i64 | Output only. The size of the address space on each node for hosting containers. This is provisioned from within the `container_ipv4_cidr` range. This field will only be set when cluster is in route-based network mode. |
| `node_pool_auto_config` | String | Node pool configs that apply to all auto-provisioned node pools in autopilot clusters and node auto-provisioning enabled clusters. |
| `security_posture_config` | String | Enable/Disable Security Posture API features for the cluster. |
| `notification_config` | String | Notification configuration of the cluster. |
| `addons_config` | String | Configurations for the various addons available to run in the cluster. |
| `secret_sync_config` | String | Configuration for sync Secret Manager secrets as k8s secrets. |
| `enterprise_config` | String | GKE Enterprise Configuration. Deprecated: GKE Enterprise features are now available without an Enterprise tier. |
| `master` | String | Configuration for master components. |
| `current_node_count` | i64 | Output only. The number of nodes currently in the cluster. Deprecated. Call Kubernetes API directly to retrieve node information. |
| `database_encryption` | String | Configuration of etcd encryption. |
| `vertical_pod_autoscaling` | String | Cluster-level Vertical Pod Autoscaling configuration. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `network_policy` | String | Configuration options for the NetworkPolicy feature. |
| `workload_certificates` | String | Configuration for issuance of mTLS keys and certificates to Kubernetes pods. |


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
cluster_parent_product_config = cluster.parent_product_config
cluster_fleet = cluster.fleet
cluster_node_config = cluster.node_config
cluster_self_link = cluster.self_link
cluster_name = cluster.name
cluster_protect_config = cluster.protect_config
cluster_alpha_cluster_feature_gates = cluster.alpha_cluster_feature_gates
cluster_zone = cluster.zone
cluster_autopilot = cluster.autopilot
cluster_subnetwork = cluster.subnetwork
cluster_control_plane_endpoints_config = cluster.control_plane_endpoints_config
cluster_instance_group_urls = cluster.instance_group_urls
cluster_satisfies_pzi = cluster.satisfies_pzi
cluster_current_node_version = cluster.current_node_version
cluster_default_max_pods_constraint = cluster.default_max_pods_constraint
cluster_master_authorized_networks_config = cluster.master_authorized_networks_config
cluster_node_pool_defaults = cluster.node_pool_defaults
cluster_cost_management_config = cluster.cost_management_config
cluster_master_ipv4_cidr_block = cluster.master_ipv4_cidr_block
cluster_legacy_abac = cluster.legacy_abac
cluster_resource_labels = cluster.resource_labels
cluster_endpoint = cluster.endpoint
cluster_private_cluster = cluster.private_cluster
cluster_locations = cluster.locations
cluster_mesh_certificates = cluster.mesh_certificates
cluster_services_ipv4_cidr = cluster.services_ipv4_cidr
cluster_private_cluster_config = cluster.private_cluster_config
cluster_ip_allocation_policy = cluster.ip_allocation_policy
cluster_authenticator_groups_config = cluster.authenticator_groups_config
cluster_monitoring_config = cluster.monitoring_config
cluster_rbac_binding_config = cluster.rbac_binding_config
cluster_resource_usage_export_config = cluster.resource_usage_export_config
cluster_location = cluster.location
cluster_enable_kubernetes_alpha = cluster.enable_kubernetes_alpha
cluster_current_master_version = cluster.current_master_version
cluster_anonymous_authentication_config = cluster.anonymous_authentication_config
cluster_logging_service = cluster.logging_service
cluster_tpu_config = cluster.tpu_config
cluster_monitoring_service = cluster.monitoring_service
cluster_initial_node_count = cluster.initial_node_count
cluster_conditions = cluster.conditions
cluster_initial_cluster_version = cluster.initial_cluster_version
cluster_gke_auto_upgrade_config = cluster.gke_auto_upgrade_config
cluster_id = cluster.id
cluster_cluster_telemetry = cluster.cluster_telemetry
cluster_expire_time = cluster.expire_time
cluster_status = cluster.status
cluster_node_pools = cluster.node_pools
cluster_secret_manager_config = cluster.secret_manager_config
cluster_workload_alts_config = cluster.workload_alts_config
cluster_enable_k8s_beta_apis = cluster.enable_k8s_beta_apis
cluster_user_managed_keys_config = cluster.user_managed_keys_config
cluster_pod_security_policy_config = cluster.pod_security_policy_config
cluster_master_auth = cluster.master_auth
cluster_network_config = cluster.network_config
cluster_compliance_posture_config = cluster.compliance_posture_config
cluster_release_channel = cluster.release_channel
cluster_cluster_ipv4_cidr = cluster.cluster_ipv4_cidr
cluster_tpu_ipv4_cidr_block = cluster.tpu_ipv4_cidr_block
cluster_description = cluster.description
cluster_etag = cluster.etag
cluster_pod_autoscaling = cluster.pod_autoscaling
cluster_workload_identity_config = cluster.workload_identity_config
cluster_autoscaling = cluster.autoscaling
cluster_binary_authorization = cluster.binary_authorization
cluster_shielded_nodes = cluster.shielded_nodes
cluster_logging_config = cluster.logging_config
cluster_create_time = cluster.create_time
cluster_identity_service_config = cluster.identity_service_config
cluster_label_fingerprint = cluster.label_fingerprint
cluster_maintenance_policy = cluster.maintenance_policy
cluster_status_message = cluster.status_message
cluster_network = cluster.network
cluster_confidential_nodes = cluster.confidential_nodes
cluster_enable_tpu = cluster.enable_tpu
cluster_node_ipv4_cidr_size = cluster.node_ipv4_cidr_size
cluster_node_pool_auto_config = cluster.node_pool_auto_config
cluster_security_posture_config = cluster.security_posture_config
cluster_notification_config = cluster.notification_config
cluster_addons_config = cluster.addons_config
cluster_secret_sync_config = cluster.secret_sync_config
cluster_enterprise_config = cluster.enterprise_config
cluster_master = cluster.master
cluster_current_node_count = cluster.current_node_count
cluster_database_encryption = cluster.database_encryption
cluster_vertical_pod_autoscaling = cluster.vertical_pod_autoscaling
cluster_satisfies_pzs = cluster.satisfies_pzs
cluster_network_policy = cluster.network_policy
cluster_workload_certificates = cluster.workload_certificates
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
| `valid_node_versions` | Vec<String> | List of valid node upgrade target versions, in descending order. |
| `default_cluster_version` | String | Version of Kubernetes the service deploys by default. |
| `default_image_type` | String | Default image type. |
| `channels` | Vec<String> | List of release channel configurations. |
| `valid_master_versions` | Vec<String> | List of valid master versions, in descending order. |
| `windows_version_maps` | HashMap<String, String> | Maps of Kubernetes version and supported Windows server versions. |
| `valid_image_types` | Vec<String> | List of valid image types. |


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
location_valid_node_versions = location.valid_node_versions
location_default_cluster_version = location.default_cluster_version
location_default_image_type = location.default_image_type
location_channels = location.channels
location_valid_master_versions = location.valid_master_versions
location_windows_version_maps = location.windows_version_maps
location_valid_image_types = location.valid_image_types
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
| `jwks_uri` | String | JSON Web Key uri. |
| `subject_types_supported` | Vec<String> | Supported subject types. |
| `response_types_supported` | Vec<String> | Supported response types. |
| `id_token_signing_alg_values_supported` | Vec<String> | supported ID Token signing Algorithms. |
| `grant_types` | Vec<String> | Supported grant types. |
| `issuer` | String | OIDC Issuer. |
| `claims_supported` | Vec<String> | Supported claims. |
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
well_known_jwks_uri = well_known.jwks_uri
well_known_subject_types_supported = well_known.subject_types_supported
well_known_response_types_supported = well_known.response_types_supported
well_known_id_token_signing_alg_values_supported = well_known.id_token_signing_alg_values_supported
well_known_grant_types = well_known.grant_types
well_known_issuer = well_known.issuer
well_known_claims_supported = well_known.claims_supported
well_known_cache_header = well_known.cache_header
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple zone resources
zone_0 = provider.container_api.Zone {
}
zone_1 = provider.container_api.Zone {
}
zone_2 = provider.container_api.Zone {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    zone = provider.container_api.Zone {
    }
```

---

## Related Documentation

- [GCP Container_api Documentation](https://cloud.google.com/container_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
