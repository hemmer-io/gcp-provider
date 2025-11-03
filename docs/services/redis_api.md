# Redis_api Service



**Resources**: 12

---

## Overview

The redis_api service provides access to 12 resource types:

- [Backup](#backup) [CRD]
- [Cluster](#cluster) [CRUD]
- [Instance](#instance) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CRD]
- [Backup_collection](#backup_collection) [R]
- [Cluster](#cluster) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Backup_collection](#backup_collection) [R]
- [Instance](#instance) [CRUD]
- [Backup](#backup) [CRD]

---

## Resources


### Backup

Exports a specific backup to a customer target Cloud Storage URI.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcs_bucket` | String |  | Google Cloud Storage bucket, like "my-bucket". |
| `name` | String | ✅ | Required. Redis backup resource name using the form: `projects/{project_id}/locations/{location_id}/backupCollections/{backup_collection_id}/backups/{backup_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backup_type` | String | Output only. Type of the backup. |
| `expire_time` | String | Output only. The time when the backup will expire. |
| `cluster` | String | Output only. Cluster resource path of this backup. |
| `backup_files` | Vec<String> | Output only. List of backup files of the backup. |
| `total_size_bytes` | String | Output only. Total size of the backup in bytes. |
| `create_time` | String | Output only. The time when the backup was created. |
| `node_type` | String | Output only. Node type of the cluster. |
| `name` | String | Identifier. Full resource path of the backup. the last part of the name is the backup id with the following format: [YYYYMMDDHHMMSS]_[Shorted Cluster UID] OR customer specified while backup cluster. Example: 20240515123000_1234 |
| `replica_count` | i64 | Output only. Number of replicas for the cluster. |
| `encryption_info` | String | Output only. Encryption information of the backup. |
| `shard_count` | i64 | Output only. Number of shards for the cluster. |
| `state` | String | Output only. State of the backup. |
| `cluster_uid` | String | Output only. Cluster uid of this backup. |
| `uid` | String | Output only. System assigned unique identifier of the backup. |
| `engine_version` | String | Output only. redis-7.2, valkey-7.5 |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.redis_api.Backup {
    name = "value"  # Required. Redis backup resource name using the form: `projects/{project_id}/locations/{location_id}/backupCollections/{backup_collection_id}/backups/{backup_id}`
}

# Access backup outputs
backup_id = backup.id
backup_backup_type = backup.backup_type
backup_expire_time = backup.expire_time
backup_cluster = backup.cluster
backup_backup_files = backup.backup_files
backup_total_size_bytes = backup.total_size_bytes
backup_create_time = backup.create_time
backup_node_type = backup.node_type
backup_name = backup.name
backup_replica_count = backup.replica_count
backup_encryption_info = backup.encryption_info
backup_shard_count = backup.shard_count
backup_state = backup.state
backup_cluster_uid = backup.cluster_uid
backup_uid = backup.uid
backup_engine_version = backup.engine_version
```

---


### Cluster

Creates a Redis cluster based on the specified properties. The creation is executed asynchronously and callers may check the returned operation to track its progress. Once the operation is completed the Redis cluster will be fully functional. The completed longrunning.Operation will contain the new cluster object in the response field. The returned operation is automatically deleted after a few hours, so there is no need to call DeleteOperation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authorization_mode` | String |  | Optional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. |
| `persistence_config` | String |  | Optional. Persistence config (RDB, AOF) for the cluster. |
| `satisfies_pzs` | bool |  | Optional. Output only. Reserved for future use. |
| `simulate_maintenance_event` | bool |  | Optional. Input only. Simulate a maintenance event. |
| `backup_collection` | String |  | Optional. Output only. The backup collection full resource name. Example: projects/{project}/locations/{location}/backupCollections/{collection} |
| `replica_count` | i64 |  | Optional. The number of replica nodes per shard. |
| `discovery_endpoints` | Vec<String> |  | Output only. Endpoints created on each given network, for Redis clients to connect to the cluster. Currently only one discovery endpoint is supported. |
| `size_gb` | i64 |  | Output only. Redis memory size in GB for the entire cluster rounded up to the next integer. |
| `deletion_protection_enabled` | bool |  | Optional. The delete operation will fail when the value is set to true. |
| `state` | String |  | Output only. The current state of this cluster. Can be CREATING, READY, UPDATING, DELETING and SUSPENDED |
| `encryption_info` | String |  | Output only. Encryption information of the data at rest of the cluster. |
| `cluster_endpoints` | Vec<String> |  | Optional. A list of cluster endpoints. |
| `async_cluster_endpoints_deletion_enabled` | bool |  | Optional. If true, cluster endpoints that are created and registered by customers can be deleted asynchronously. That is, such a cluster endpoint can be de-registered before the forwarding rules in the cluster endpoint are deleted. |
| `name` | String |  | Required. Identifier. Unique name of the resource in this scope including project and location using the form: `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}` |
| `managed_backup_source` | String |  | Optional. Backups generated and managed by memorystore service. |
| `maintenance_schedule` | String |  | Output only. ClusterMaintenanceSchedule Output only Published maintenance schedule. |
| `maintenance_policy` | String |  | Optional. ClusterMaintenancePolicy determines when to allow or deny updates. |
| `transit_encryption_mode` | String |  | Optional. The in-transit encryption for the Redis cluster. If not provided, encryption is disabled for the cluster. |
| `node_type` | String |  | Optional. The type of a redis node in the cluster. NodeType determines the underlying machine-type of a redis node. |
| `allow_fewer_zones_deployment` | bool |  | Optional. Immutable. Deprecated, do not use. |
| `labels` | HashMap<String, String> |  | Optional. Labels to represent user-provided metadata. |
| `cross_cluster_replication_config` | String |  | Optional. Cross cluster replication config. |
| `automated_backup_config` | String |  | Optional. The automated backup config for the cluster. |
| `precise_size_gb` | f64 |  | Output only. Precise value of redis memory size in GB for the entire cluster. |
| `redis_configs` | HashMap<String, String> |  | Optional. Key/Value pairs of customer overrides for mutable Redis Configs |
| `state_info` | String |  | Output only. Additional information about the current state of the cluster. |
| `zone_distribution_config` | String |  | Optional. This config will be used to determine how the customer wants us to distribute cluster resources within the region. |
| `satisfies_pzi` | bool |  | Optional. Output only. Reserved for future use. |
| `effective_maintenance_version` | String |  | Output only. This field represents the actual maintenance version of the cluster. |
| `kms_key` | String |  | Optional. The KMS key used to encrypt the at-rest data of the cluster. |
| `psc_configs` | Vec<String> |  | Optional. Each PscConfig configures the consumer network where IPs will be designated to the cluster for client access through Private Service Connect Automation. Currently, only one PscConfig is supported. |
| `psc_service_attachments` | Vec<String> |  | Output only. Service attachment details to configure Psc connections |
| `uid` | String |  | Output only. System assigned, unique identifier for the cluster. |
| `maintenance_version` | String |  | Optional. This field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field. |
| `ondemand_maintenance` | bool |  | Optional. Input only. Ondemand maintenance for the cluster. This field can be used to trigger ondemand critical update on the cluster. |
| `psc_connections` | Vec<String> |  | Output only. The list of PSC connections that are auto-created through service connectivity automation. |
| `gcs_source` | String |  | Optional. Backups stored in Cloud Storage buckets. The Cloud Storage buckets need to be the same region as the clusters. Read permission is required to import from the provided Cloud Storage objects. |
| `create_time` | String |  | Output only. The timestamp associated with the cluster creation request. |
| `shard_count` | i64 |  | Optional. Number of shards for the Redis cluster. |
| `available_maintenance_versions` | Vec<String> |  | Output only. This field is used to determine the available maintenance versions for the self service update. |
| `parent` | String | ✅ | Required. The resource name of the cluster location using the form: `projects/{project_id}/locations/{location_id}` where `location_id` refers to a Google Cloud region. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authorization_mode` | String | Optional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. |
| `persistence_config` | String | Optional. Persistence config (RDB, AOF) for the cluster. |
| `satisfies_pzs` | bool | Optional. Output only. Reserved for future use. |
| `simulate_maintenance_event` | bool | Optional. Input only. Simulate a maintenance event. |
| `backup_collection` | String | Optional. Output only. The backup collection full resource name. Example: projects/{project}/locations/{location}/backupCollections/{collection} |
| `replica_count` | i64 | Optional. The number of replica nodes per shard. |
| `discovery_endpoints` | Vec<String> | Output only. Endpoints created on each given network, for Redis clients to connect to the cluster. Currently only one discovery endpoint is supported. |
| `size_gb` | i64 | Output only. Redis memory size in GB for the entire cluster rounded up to the next integer. |
| `deletion_protection_enabled` | bool | Optional. The delete operation will fail when the value is set to true. |
| `state` | String | Output only. The current state of this cluster. Can be CREATING, READY, UPDATING, DELETING and SUSPENDED |
| `encryption_info` | String | Output only. Encryption information of the data at rest of the cluster. |
| `cluster_endpoints` | Vec<String> | Optional. A list of cluster endpoints. |
| `async_cluster_endpoints_deletion_enabled` | bool | Optional. If true, cluster endpoints that are created and registered by customers can be deleted asynchronously. That is, such a cluster endpoint can be de-registered before the forwarding rules in the cluster endpoint are deleted. |
| `name` | String | Required. Identifier. Unique name of the resource in this scope including project and location using the form: `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}` |
| `managed_backup_source` | String | Optional. Backups generated and managed by memorystore service. |
| `maintenance_schedule` | String | Output only. ClusterMaintenanceSchedule Output only Published maintenance schedule. |
| `maintenance_policy` | String | Optional. ClusterMaintenancePolicy determines when to allow or deny updates. |
| `transit_encryption_mode` | String | Optional. The in-transit encryption for the Redis cluster. If not provided, encryption is disabled for the cluster. |
| `node_type` | String | Optional. The type of a redis node in the cluster. NodeType determines the underlying machine-type of a redis node. |
| `allow_fewer_zones_deployment` | bool | Optional. Immutable. Deprecated, do not use. |
| `labels` | HashMap<String, String> | Optional. Labels to represent user-provided metadata. |
| `cross_cluster_replication_config` | String | Optional. Cross cluster replication config. |
| `automated_backup_config` | String | Optional. The automated backup config for the cluster. |
| `precise_size_gb` | f64 | Output only. Precise value of redis memory size in GB for the entire cluster. |
| `redis_configs` | HashMap<String, String> | Optional. Key/Value pairs of customer overrides for mutable Redis Configs |
| `state_info` | String | Output only. Additional information about the current state of the cluster. |
| `zone_distribution_config` | String | Optional. This config will be used to determine how the customer wants us to distribute cluster resources within the region. |
| `satisfies_pzi` | bool | Optional. Output only. Reserved for future use. |
| `effective_maintenance_version` | String | Output only. This field represents the actual maintenance version of the cluster. |
| `kms_key` | String | Optional. The KMS key used to encrypt the at-rest data of the cluster. |
| `psc_configs` | Vec<String> | Optional. Each PscConfig configures the consumer network where IPs will be designated to the cluster for client access through Private Service Connect Automation. Currently, only one PscConfig is supported. |
| `psc_service_attachments` | Vec<String> | Output only. Service attachment details to configure Psc connections |
| `uid` | String | Output only. System assigned, unique identifier for the cluster. |
| `maintenance_version` | String | Optional. This field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field. |
| `ondemand_maintenance` | bool | Optional. Input only. Ondemand maintenance for the cluster. This field can be used to trigger ondemand critical update on the cluster. |
| `psc_connections` | Vec<String> | Output only. The list of PSC connections that are auto-created through service connectivity automation. |
| `gcs_source` | String | Optional. Backups stored in Cloud Storage buckets. The Cloud Storage buckets need to be the same region as the clusters. Read permission is required to import from the provided Cloud Storage objects. |
| `create_time` | String | Output only. The timestamp associated with the cluster creation request. |
| `shard_count` | i64 | Optional. Number of shards for the Redis cluster. |
| `available_maintenance_versions` | Vec<String> | Output only. This field is used to determine the available maintenance versions for the self service update. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cluster
cluster = provider.redis_api.Cluster {
    parent = "value"  # Required. The resource name of the cluster location using the form: `projects/{project_id}/locations/{location_id}` where `location_id` refers to a Google Cloud region.
}

# Access cluster outputs
cluster_id = cluster.id
cluster_authorization_mode = cluster.authorization_mode
cluster_persistence_config = cluster.persistence_config
cluster_satisfies_pzs = cluster.satisfies_pzs
cluster_simulate_maintenance_event = cluster.simulate_maintenance_event
cluster_backup_collection = cluster.backup_collection
cluster_replica_count = cluster.replica_count
cluster_discovery_endpoints = cluster.discovery_endpoints
cluster_size_gb = cluster.size_gb
cluster_deletion_protection_enabled = cluster.deletion_protection_enabled
cluster_state = cluster.state
cluster_encryption_info = cluster.encryption_info
cluster_cluster_endpoints = cluster.cluster_endpoints
cluster_async_cluster_endpoints_deletion_enabled = cluster.async_cluster_endpoints_deletion_enabled
cluster_name = cluster.name
cluster_managed_backup_source = cluster.managed_backup_source
cluster_maintenance_schedule = cluster.maintenance_schedule
cluster_maintenance_policy = cluster.maintenance_policy
cluster_transit_encryption_mode = cluster.transit_encryption_mode
cluster_node_type = cluster.node_type
cluster_allow_fewer_zones_deployment = cluster.allow_fewer_zones_deployment
cluster_labels = cluster.labels
cluster_cross_cluster_replication_config = cluster.cross_cluster_replication_config
cluster_automated_backup_config = cluster.automated_backup_config
cluster_precise_size_gb = cluster.precise_size_gb
cluster_redis_configs = cluster.redis_configs
cluster_state_info = cluster.state_info
cluster_zone_distribution_config = cluster.zone_distribution_config
cluster_satisfies_pzi = cluster.satisfies_pzi
cluster_effective_maintenance_version = cluster.effective_maintenance_version
cluster_kms_key = cluster.kms_key
cluster_psc_configs = cluster.psc_configs
cluster_psc_service_attachments = cluster.psc_service_attachments
cluster_uid = cluster.uid
cluster_maintenance_version = cluster.maintenance_version
cluster_ondemand_maintenance = cluster.ondemand_maintenance
cluster_psc_connections = cluster.psc_connections
cluster_gcs_source = cluster.gcs_source
cluster_create_time = cluster.create_time
cluster_shard_count = cluster.shard_count
cluster_available_maintenance_versions = cluster.available_maintenance_versions
```

---


### Instance

Creates a Redis instance based on the specified tier and memory size. By default, the instance is accessible from the project's [default network](https://cloud.google.com/vpc/docs/vpc). The creation is executed asynchronously and callers may check the returned operation to track its progress. Once the operation is completed the Redis instance will be fully functional. Completed longrunning.Operation will contain the new instance object in the response field. The returned operation is automatically deleted after a few hours, so there is no need to call DeleteOperation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `suspension_reasons` | Vec<String> |  | Optional. reasons that causes instance in "SUSPENDED" state. |
| `available_maintenance_versions` | Vec<String> |  | Optional. The available maintenance versions that an instance could update to. |
| `location_id` | String |  | Optional. The zone where the instance will be provisioned. If not provided, the service will choose a zone from the specified region for the instance. For standard tier, additional nodes will be added across multiple zones for protection against zonal failures. If specified, at least one node will be provisioned in this zone. |
| `status_message` | String |  | Output only. Additional information about the current status of this instance, if available. |
| `alternative_location_id` | String |  | Optional. If specified, at least one node will be provisioned in this zone in addition to the zone specified in location_id. Only applicable to standard tier. If provided, it must be a different zone from the one provided in [location_id]. Additional nodes beyond the first 2 will be placed in zones selected by the service. |
| `nodes` | Vec<String> |  | Output only. Info per node. |
| `read_replicas_mode` | String |  | Optional. Read replicas mode for the instance. Defaults to READ_REPLICAS_DISABLED. |
| `maintenance_schedule` | String |  | Output only. Date and time of upcoming maintenance events which have been scheduled. |
| `name` | String |  | Required. Unique name of the resource in this scope including project and location using the form: `projects/{project_id}/locations/{location_id}/instances/{instance_id}` Note: Redis instances are managed and addressed at regional level so location_id here refers to a GCP region; however, users may choose which specific zone (or collection of zones for cross-zone instances) an instance should be provisioned in. Refer to location_id and alternative_location_id fields for more details. |
| `auth_enabled` | bool |  | Optional. Indicates whether OSS Redis AUTH is enabled for the instance. If set to "true" AUTH is enabled on the instance. Default value is "false" meaning AUTH is disabled. |
| `current_location_id` | String |  | Output only. The current zone where the Redis primary node is located. In basic tier, this will always be the same as [location_id]. In standard tier, this can be the zone of any node in the instance. |
| `redis_version` | String |  | Optional. The version of Redis software. If not provided, the default version will be used. Currently, the supported values are: * `REDIS_3_2` for Redis 3.2 compatibility * `REDIS_4_0` for Redis 4.0 compatibility * `REDIS_5_0` for Redis 5.0 compatibility * `REDIS_6_X` for Redis 6.x compatibility * `REDIS_7_0` for Redis 7.0 compatibility (default) * `REDIS_7_2` for Redis 7.2 compatibility |
| `replica_count` | i64 |  | Optional. The number of replica nodes. The valid range for the Standard Tier with read replicas enabled is [1-5] and defaults to 2. If read replicas are not enabled for a Standard Tier instance, the only valid value is 1 and the default is 1. The valid value for basic tier is 0 and the default is also 0. |
| `tier` | String |  | Required. The service tier of the instance. |
| `redis_configs` | HashMap<String, String> |  | Optional. Redis configuration parameters, according to http://redis.io/topics/config. Currently, the only supported parameters are: Redis version 3.2 and newer: * maxmemory-policy * notify-keyspace-events Redis version 4.0 and newer: * activedefrag * lfu-decay-time * lfu-log-factor * maxmemory-gb Redis version 5.0 and newer: * stream-node-max-bytes * stream-node-max-entries |
| `create_time` | String |  | Output only. The time the instance was created. |
| `maintenance_version` | String |  | Optional. The self service update maintenance version. The version is date based such as "20210712_00_00". |
| `state` | String |  | Output only. The current state of this instance. |
| `authorized_network` | String |  | Optional. The full name of the Google Compute Engine [network](https://cloud.google.com/vpc/docs/vpc) to which the instance is connected. If left unspecified, the `default` network will be used. |
| `connect_mode` | String |  | Optional. The network connect mode of the Redis instance. If not provided, the connect mode defaults to DIRECT_PEERING. |
| `memory_size_gb` | i64 |  | Required. Redis memory size in GiB. |
| `port` | i64 |  | Output only. The port number of the exposed Redis endpoint. |
| `display_name` | String |  | An arbitrary and optional user-provided name for the instance. |
| `server_ca_certs` | Vec<String> |  | Output only. List of server CA certificates for the instance. |
| `reserved_ip_range` | String |  | Optional. For DIRECT_PEERING mode, the CIDR range of internal addresses that are reserved for this instance. Range must be unique and non-overlapping with existing subnets in an authorized network. For PRIVATE_SERVICE_ACCESS mode, the name of one allocated IP address ranges associated with this private service access connection. If not provided, the service will choose an unused /29 block, for example, 10.0.0.0/29 or 192.168.0.0/29. For READ_REPLICAS_ENABLED the default block size is /28. |
| `host` | String |  | Output only. Hostname or IP address of the exposed Redis endpoint used by clients to connect to the service. |
| `transit_encryption_mode` | String |  | Optional. The TLS mode of the Redis instance. If not provided, TLS is disabled for the instance. |
| `persistence_config` | String |  | Optional. Persistence configuration parameters |
| `read_endpoint` | String |  | Output only. Hostname or IP address of the exposed readonly Redis endpoint. Standard tier only. Targets all healthy replica nodes in instance. Replication is asynchronous and replica nodes will exhibit some lag behind the primary. Write requests must target 'host'. |
| `persistence_iam_identity` | String |  | Output only. Cloud IAM identity used by import / export operations to transfer data to/from Cloud Storage. Format is "serviceAccount:". The value may change over time for a given instance so should be checked before each import/export operation. |
| `secondary_ip_range` | String |  | Optional. Additional IP range for node placement. Required when enabling read replicas on an existing instance. For DIRECT_PEERING mode value must be a CIDR range of size /28, or "auto". For PRIVATE_SERVICE_ACCESS mode value must be the name of an allocated address range associated with the private service access connection, or "auto". |
| `customer_managed_key` | String |  | Optional. The KMS key reference that the customer provides when trying to create the instance. |
| `read_endpoint_port` | i64 |  | Output only. The port number of the exposed readonly redis endpoint. Standard tier only. Write requests should target 'port'. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata |
| `maintenance_policy` | String |  | Optional. The maintenance policy for the instance. If not provided, maintenance events can be performed at any time. |
| `satisfies_pzs` | bool |  | Optional. Output only. Reserved for future use. |
| `satisfies_pzi` | bool |  | Optional. Output only. Reserved for future use. |
| `parent` | String | ✅ | Required. The resource name of the instance location using the form: `projects/{project_id}/locations/{location_id}` where `location_id` refers to a GCP region. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `suspension_reasons` | Vec<String> | Optional. reasons that causes instance in "SUSPENDED" state. |
| `available_maintenance_versions` | Vec<String> | Optional. The available maintenance versions that an instance could update to. |
| `location_id` | String | Optional. The zone where the instance will be provisioned. If not provided, the service will choose a zone from the specified region for the instance. For standard tier, additional nodes will be added across multiple zones for protection against zonal failures. If specified, at least one node will be provisioned in this zone. |
| `status_message` | String | Output only. Additional information about the current status of this instance, if available. |
| `alternative_location_id` | String | Optional. If specified, at least one node will be provisioned in this zone in addition to the zone specified in location_id. Only applicable to standard tier. If provided, it must be a different zone from the one provided in [location_id]. Additional nodes beyond the first 2 will be placed in zones selected by the service. |
| `nodes` | Vec<String> | Output only. Info per node. |
| `read_replicas_mode` | String | Optional. Read replicas mode for the instance. Defaults to READ_REPLICAS_DISABLED. |
| `maintenance_schedule` | String | Output only. Date and time of upcoming maintenance events which have been scheduled. |
| `name` | String | Required. Unique name of the resource in this scope including project and location using the form: `projects/{project_id}/locations/{location_id}/instances/{instance_id}` Note: Redis instances are managed and addressed at regional level so location_id here refers to a GCP region; however, users may choose which specific zone (or collection of zones for cross-zone instances) an instance should be provisioned in. Refer to location_id and alternative_location_id fields for more details. |
| `auth_enabled` | bool | Optional. Indicates whether OSS Redis AUTH is enabled for the instance. If set to "true" AUTH is enabled on the instance. Default value is "false" meaning AUTH is disabled. |
| `current_location_id` | String | Output only. The current zone where the Redis primary node is located. In basic tier, this will always be the same as [location_id]. In standard tier, this can be the zone of any node in the instance. |
| `redis_version` | String | Optional. The version of Redis software. If not provided, the default version will be used. Currently, the supported values are: * `REDIS_3_2` for Redis 3.2 compatibility * `REDIS_4_0` for Redis 4.0 compatibility * `REDIS_5_0` for Redis 5.0 compatibility * `REDIS_6_X` for Redis 6.x compatibility * `REDIS_7_0` for Redis 7.0 compatibility (default) * `REDIS_7_2` for Redis 7.2 compatibility |
| `replica_count` | i64 | Optional. The number of replica nodes. The valid range for the Standard Tier with read replicas enabled is [1-5] and defaults to 2. If read replicas are not enabled for a Standard Tier instance, the only valid value is 1 and the default is 1. The valid value for basic tier is 0 and the default is also 0. |
| `tier` | String | Required. The service tier of the instance. |
| `redis_configs` | HashMap<String, String> | Optional. Redis configuration parameters, according to http://redis.io/topics/config. Currently, the only supported parameters are: Redis version 3.2 and newer: * maxmemory-policy * notify-keyspace-events Redis version 4.0 and newer: * activedefrag * lfu-decay-time * lfu-log-factor * maxmemory-gb Redis version 5.0 and newer: * stream-node-max-bytes * stream-node-max-entries |
| `create_time` | String | Output only. The time the instance was created. |
| `maintenance_version` | String | Optional. The self service update maintenance version. The version is date based such as "20210712_00_00". |
| `state` | String | Output only. The current state of this instance. |
| `authorized_network` | String | Optional. The full name of the Google Compute Engine [network](https://cloud.google.com/vpc/docs/vpc) to which the instance is connected. If left unspecified, the `default` network will be used. |
| `connect_mode` | String | Optional. The network connect mode of the Redis instance. If not provided, the connect mode defaults to DIRECT_PEERING. |
| `memory_size_gb` | i64 | Required. Redis memory size in GiB. |
| `port` | i64 | Output only. The port number of the exposed Redis endpoint. |
| `display_name` | String | An arbitrary and optional user-provided name for the instance. |
| `server_ca_certs` | Vec<String> | Output only. List of server CA certificates for the instance. |
| `reserved_ip_range` | String | Optional. For DIRECT_PEERING mode, the CIDR range of internal addresses that are reserved for this instance. Range must be unique and non-overlapping with existing subnets in an authorized network. For PRIVATE_SERVICE_ACCESS mode, the name of one allocated IP address ranges associated with this private service access connection. If not provided, the service will choose an unused /29 block, for example, 10.0.0.0/29 or 192.168.0.0/29. For READ_REPLICAS_ENABLED the default block size is /28. |
| `host` | String | Output only. Hostname or IP address of the exposed Redis endpoint used by clients to connect to the service. |
| `transit_encryption_mode` | String | Optional. The TLS mode of the Redis instance. If not provided, TLS is disabled for the instance. |
| `persistence_config` | String | Optional. Persistence configuration parameters |
| `read_endpoint` | String | Output only. Hostname or IP address of the exposed readonly Redis endpoint. Standard tier only. Targets all healthy replica nodes in instance. Replication is asynchronous and replica nodes will exhibit some lag behind the primary. Write requests must target 'host'. |
| `persistence_iam_identity` | String | Output only. Cloud IAM identity used by import / export operations to transfer data to/from Cloud Storage. Format is "serviceAccount:". The value may change over time for a given instance so should be checked before each import/export operation. |
| `secondary_ip_range` | String | Optional. Additional IP range for node placement. Required when enabling read replicas on an existing instance. For DIRECT_PEERING mode value must be a CIDR range of size /28, or "auto". For PRIVATE_SERVICE_ACCESS mode value must be the name of an allocated address range associated with the private service access connection, or "auto". |
| `customer_managed_key` | String | Optional. The KMS key reference that the customer provides when trying to create the instance. |
| `read_endpoint_port` | i64 | Output only. The port number of the exposed readonly redis endpoint. Standard tier only. Write requests should target 'port'. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata |
| `maintenance_policy` | String | Optional. The maintenance policy for the instance. If not provided, maintenance events can be performed at any time. |
| `satisfies_pzs` | bool | Optional. Output only. Reserved for future use. |
| `satisfies_pzi` | bool | Optional. Output only. Reserved for future use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.redis_api.Instance {
    parent = "value"  # Required. The resource name of the instance location using the form: `projects/{project_id}/locations/{location_id}` where `location_id` refers to a GCP region.
}

# Access instance outputs
instance_id = instance.id
instance_suspension_reasons = instance.suspension_reasons
instance_available_maintenance_versions = instance.available_maintenance_versions
instance_location_id = instance.location_id
instance_status_message = instance.status_message
instance_alternative_location_id = instance.alternative_location_id
instance_nodes = instance.nodes
instance_read_replicas_mode = instance.read_replicas_mode
instance_maintenance_schedule = instance.maintenance_schedule
instance_name = instance.name
instance_auth_enabled = instance.auth_enabled
instance_current_location_id = instance.current_location_id
instance_redis_version = instance.redis_version
instance_replica_count = instance.replica_count
instance_tier = instance.tier
instance_redis_configs = instance.redis_configs
instance_create_time = instance.create_time
instance_maintenance_version = instance.maintenance_version
instance_state = instance.state
instance_authorized_network = instance.authorized_network
instance_connect_mode = instance.connect_mode
instance_memory_size_gb = instance.memory_size_gb
instance_port = instance.port
instance_display_name = instance.display_name
instance_server_ca_certs = instance.server_ca_certs
instance_reserved_ip_range = instance.reserved_ip_range
instance_host = instance.host
instance_transit_encryption_mode = instance.transit_encryption_mode
instance_persistence_config = instance.persistence_config
instance_read_endpoint = instance.read_endpoint
instance_persistence_iam_identity = instance.persistence_iam_identity
instance_secondary_ip_range = instance.secondary_ip_range
instance_customer_managed_key = instance.customer_managed_key
instance_read_endpoint_port = instance.read_endpoint_port
instance_tags = instance.tags
instance_labels = instance.labels
instance_maintenance_policy = instance.maintenance_policy
instance_satisfies_pzs = instance.satisfies_pzs
instance_satisfies_pzi = instance.satisfies_pzi
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
| `location_id` | String | Resource ID for the region. For example: "us-east1". |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Full resource name for the region. For example: "projects/example-project/locations/us-east1". |
| `metadata` | HashMap<String, String> | Output only. The set of available zones in the location. The map is keyed by the lowercase ID of each zone, as defined by Compute Engine. These keys can be specified in `location_id` or `alternative_location_id` fields when creating a Redis instance. |


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
location_labels = location.labels
location_name = location.name
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | { `createTime`: The time the operation was created. `endTime`: The time the operation finished running. `target`: Server-defined resource path for the target of the operation. `verb`: Name of the verb executed by the operation. `statusDetail`: Human-readable status of the operation, if any. `cancelRequested`: Identifies whether the user has requested cancellation of the operation. Operations that have successfully been cancelled have Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`. `apiVersion`: API version used to start the operation. } |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.redis_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
operation_error = operation.error
```

---


### Backup_collection

Get a backup collection.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. System assigned unique identifier of the backup collection. |
| `last_backup_time` | String | Output only. The last time a backup was created in the backup collection. |
| `cluster_uid` | String | Output only. The cluster uid of the backup collection. |
| `total_backup_size_bytes` | String | Output only. Total size of all backups in the backup collection. |
| `create_time` | String | Output only. The time when the backup collection was created. |
| `name` | String | Identifier. Full resource path of the backup collection. |
| `total_backup_count` | String | Output only. Total number of backups in the backup collection. |
| `kms_key` | String | Output only. The KMS key used to encrypt the backups under this backup collection. |
| `cluster` | String | Output only. The full resource path of the cluster the backup collection belongs to. Example: projects/{project}/locations/{location}/clusters/{cluster} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access backup_collection outputs
backup_collection_id = backup_collection.id
backup_collection_uid = backup_collection.uid
backup_collection_last_backup_time = backup_collection.last_backup_time
backup_collection_cluster_uid = backup_collection.cluster_uid
backup_collection_total_backup_size_bytes = backup_collection.total_backup_size_bytes
backup_collection_create_time = backup_collection.create_time
backup_collection_name = backup_collection.name
backup_collection_total_backup_count = backup_collection.total_backup_count
backup_collection_kms_key = backup_collection.kms_key
backup_collection_cluster = backup_collection.cluster
```

---


### Cluster

Creates a Redis cluster based on the specified properties. The creation is executed asynchronously and callers may check the returned operation to track its progress. Once the operation is completed the Redis cluster will be fully functional. The completed longrunning.Operation will contain the new cluster object in the response field. The returned operation is automatically deleted after a few hours, so there is no need to call DeleteOperation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The current state of this cluster. Can be CREATING, READY, UPDATING, DELETING and SUSPENDED |
| `uid` | String |  | Output only. System assigned, unique identifier for the cluster. |
| `maintenance_policy` | String |  | Optional. ClusterMaintenancePolicy determines when to allow or deny updates. |
| `effective_maintenance_version` | String |  | Output only. This field represents the actual maintenance version of the cluster. |
| `psc_service_attachments` | Vec<String> |  | Output only. Service attachment details to configure Psc connections |
| `psc_configs` | Vec<String> |  | Optional. Each PscConfig configures the consumer network where IPs will be designated to the cluster for client access through Private Service Connect Automation. Currently, only one PscConfig is supported. |
| `available_maintenance_versions` | Vec<String> |  | Output only. This field is used to determine the available maintenance versions for the self service update. |
| `state_info` | String |  | Output only. Additional information about the current state of the cluster. |
| `precise_size_gb` | f64 |  | Output only. Precise value of redis memory size in GB for the entire cluster. |
| `authorization_mode` | String |  | Optional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. |
| `async_cluster_endpoints_deletion_enabled` | bool |  | Optional. If true, cluster endpoints that are created and registered by customers can be deleted asynchronously. That is, such a cluster endpoint can be de-registered before the forwarding rules in the cluster endpoint are deleted. |
| `encryption_info` | String |  | Output only. Encryption information of the data at rest of the cluster. |
| `maintenance_version` | String |  | Optional. This field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field. |
| `discovery_endpoints` | Vec<String> |  | Output only. Endpoints created on each given network, for Redis clients to connect to the cluster. Currently only one discovery endpoint is supported. |
| `ondemand_maintenance` | bool |  | Optional. Input only. Ondemand maintenance for the cluster. This field can be used to trigger ondemand critical update on the cluster. |
| `persistence_config` | String |  | Optional. Persistence config (RDB, AOF) for the cluster. |
| `create_time` | String |  | Output only. The timestamp associated with the cluster creation request. |
| `redis_configs` | HashMap<String, String> |  | Optional. Key/Value pairs of customer overrides for mutable Redis Configs |
| `allow_fewer_zones_deployment` | bool |  | Optional. Immutable. Deprecated, do not use. |
| `transit_encryption_mode` | String |  | Optional. The in-transit encryption for the Redis cluster. If not provided, encryption is disabled for the cluster. |
| `psc_connections` | Vec<String> |  | Output only. The list of PSC connections that are auto-created through service connectivity automation. |
| `satisfies_pzs` | bool |  | Optional. Output only. Reserved for future use. |
| `automated_backup_config` | String |  | Optional. The automated backup config for the cluster. |
| `replica_count` | i64 |  | Optional. The number of replica nodes per shard. |
| `kms_key` | String |  | Optional. The KMS key used to encrypt the at-rest data of the cluster. |
| `shard_count` | i64 |  | Optional. Number of shards for the Redis cluster. |
| `cross_cluster_replication_config` | String |  | Optional. Cross cluster replication config. |
| `size_gb` | i64 |  | Output only. Redis memory size in GB for the entire cluster rounded up to the next integer. |
| `deletion_protection_enabled` | bool |  | Optional. The delete operation will fail when the value is set to true. |
| `zone_distribution_config` | String |  | Optional. This config will be used to determine how the customer wants us to distribute cluster resources within the region. |
| `node_type` | String |  | Optional. The type of a redis node in the cluster. NodeType determines the underlying machine-type of a redis node. |
| `backup_collection` | String |  | Optional. Output only. The backup collection full resource name. Example: projects/{project}/locations/{location}/backupCollections/{collection} |
| `maintenance_schedule` | String |  | Output only. ClusterMaintenanceSchedule Output only Published maintenance schedule. |
| `managed_backup_source` | String |  | Optional. Backups generated and managed by memorystore service. |
| `cluster_endpoints` | Vec<String> |  | Optional. A list of cluster endpoints. |
| `gcs_source` | String |  | Optional. Backups stored in Cloud Storage buckets. The Cloud Storage buckets need to be the same region as the clusters. Read permission is required to import from the provided Cloud Storage objects. |
| `labels` | HashMap<String, String> |  | Optional. Labels to represent user-provided metadata. |
| `name` | String |  | Required. Identifier. Unique name of the resource in this scope including project and location using the form: `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}` |
| `satisfies_pzi` | bool |  | Optional. Output only. Reserved for future use. |
| `simulate_maintenance_event` | bool |  | Optional. Input only. Simulate a maintenance event. |
| `parent` | String | ✅ | Required. The resource name of the cluster location using the form: `projects/{project_id}/locations/{location_id}` where `location_id` refers to a Google Cloud region. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The current state of this cluster. Can be CREATING, READY, UPDATING, DELETING and SUSPENDED |
| `uid` | String | Output only. System assigned, unique identifier for the cluster. |
| `maintenance_policy` | String | Optional. ClusterMaintenancePolicy determines when to allow or deny updates. |
| `effective_maintenance_version` | String | Output only. This field represents the actual maintenance version of the cluster. |
| `psc_service_attachments` | Vec<String> | Output only. Service attachment details to configure Psc connections |
| `psc_configs` | Vec<String> | Optional. Each PscConfig configures the consumer network where IPs will be designated to the cluster for client access through Private Service Connect Automation. Currently, only one PscConfig is supported. |
| `available_maintenance_versions` | Vec<String> | Output only. This field is used to determine the available maintenance versions for the self service update. |
| `state_info` | String | Output only. Additional information about the current state of the cluster. |
| `precise_size_gb` | f64 | Output only. Precise value of redis memory size in GB for the entire cluster. |
| `authorization_mode` | String | Optional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. |
| `async_cluster_endpoints_deletion_enabled` | bool | Optional. If true, cluster endpoints that are created and registered by customers can be deleted asynchronously. That is, such a cluster endpoint can be de-registered before the forwarding rules in the cluster endpoint are deleted. |
| `encryption_info` | String | Output only. Encryption information of the data at rest of the cluster. |
| `maintenance_version` | String | Optional. This field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field. |
| `discovery_endpoints` | Vec<String> | Output only. Endpoints created on each given network, for Redis clients to connect to the cluster. Currently only one discovery endpoint is supported. |
| `ondemand_maintenance` | bool | Optional. Input only. Ondemand maintenance for the cluster. This field can be used to trigger ondemand critical update on the cluster. |
| `persistence_config` | String | Optional. Persistence config (RDB, AOF) for the cluster. |
| `create_time` | String | Output only. The timestamp associated with the cluster creation request. |
| `redis_configs` | HashMap<String, String> | Optional. Key/Value pairs of customer overrides for mutable Redis Configs |
| `allow_fewer_zones_deployment` | bool | Optional. Immutable. Deprecated, do not use. |
| `transit_encryption_mode` | String | Optional. The in-transit encryption for the Redis cluster. If not provided, encryption is disabled for the cluster. |
| `psc_connections` | Vec<String> | Output only. The list of PSC connections that are auto-created through service connectivity automation. |
| `satisfies_pzs` | bool | Optional. Output only. Reserved for future use. |
| `automated_backup_config` | String | Optional. The automated backup config for the cluster. |
| `replica_count` | i64 | Optional. The number of replica nodes per shard. |
| `kms_key` | String | Optional. The KMS key used to encrypt the at-rest data of the cluster. |
| `shard_count` | i64 | Optional. Number of shards for the Redis cluster. |
| `cross_cluster_replication_config` | String | Optional. Cross cluster replication config. |
| `size_gb` | i64 | Output only. Redis memory size in GB for the entire cluster rounded up to the next integer. |
| `deletion_protection_enabled` | bool | Optional. The delete operation will fail when the value is set to true. |
| `zone_distribution_config` | String | Optional. This config will be used to determine how the customer wants us to distribute cluster resources within the region. |
| `node_type` | String | Optional. The type of a redis node in the cluster. NodeType determines the underlying machine-type of a redis node. |
| `backup_collection` | String | Optional. Output only. The backup collection full resource name. Example: projects/{project}/locations/{location}/backupCollections/{collection} |
| `maintenance_schedule` | String | Output only. ClusterMaintenanceSchedule Output only Published maintenance schedule. |
| `managed_backup_source` | String | Optional. Backups generated and managed by memorystore service. |
| `cluster_endpoints` | Vec<String> | Optional. A list of cluster endpoints. |
| `gcs_source` | String | Optional. Backups stored in Cloud Storage buckets. The Cloud Storage buckets need to be the same region as the clusters. Read permission is required to import from the provided Cloud Storage objects. |
| `labels` | HashMap<String, String> | Optional. Labels to represent user-provided metadata. |
| `name` | String | Required. Identifier. Unique name of the resource in this scope including project and location using the form: `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}` |
| `satisfies_pzi` | bool | Optional. Output only. Reserved for future use. |
| `simulate_maintenance_event` | bool | Optional. Input only. Simulate a maintenance event. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cluster
cluster = provider.redis_api.Cluster {
    parent = "value"  # Required. The resource name of the cluster location using the form: `projects/{project_id}/locations/{location_id}` where `location_id` refers to a Google Cloud region.
}

# Access cluster outputs
cluster_id = cluster.id
cluster_state = cluster.state
cluster_uid = cluster.uid
cluster_maintenance_policy = cluster.maintenance_policy
cluster_effective_maintenance_version = cluster.effective_maintenance_version
cluster_psc_service_attachments = cluster.psc_service_attachments
cluster_psc_configs = cluster.psc_configs
cluster_available_maintenance_versions = cluster.available_maintenance_versions
cluster_state_info = cluster.state_info
cluster_precise_size_gb = cluster.precise_size_gb
cluster_authorization_mode = cluster.authorization_mode
cluster_async_cluster_endpoints_deletion_enabled = cluster.async_cluster_endpoints_deletion_enabled
cluster_encryption_info = cluster.encryption_info
cluster_maintenance_version = cluster.maintenance_version
cluster_discovery_endpoints = cluster.discovery_endpoints
cluster_ondemand_maintenance = cluster.ondemand_maintenance
cluster_persistence_config = cluster.persistence_config
cluster_create_time = cluster.create_time
cluster_redis_configs = cluster.redis_configs
cluster_allow_fewer_zones_deployment = cluster.allow_fewer_zones_deployment
cluster_transit_encryption_mode = cluster.transit_encryption_mode
cluster_psc_connections = cluster.psc_connections
cluster_satisfies_pzs = cluster.satisfies_pzs
cluster_automated_backup_config = cluster.automated_backup_config
cluster_replica_count = cluster.replica_count
cluster_kms_key = cluster.kms_key
cluster_shard_count = cluster.shard_count
cluster_cross_cluster_replication_config = cluster.cross_cluster_replication_config
cluster_size_gb = cluster.size_gb
cluster_deletion_protection_enabled = cluster.deletion_protection_enabled
cluster_zone_distribution_config = cluster.zone_distribution_config
cluster_node_type = cluster.node_type
cluster_backup_collection = cluster.backup_collection
cluster_maintenance_schedule = cluster.maintenance_schedule
cluster_managed_backup_source = cluster.managed_backup_source
cluster_cluster_endpoints = cluster.cluster_endpoints
cluster_gcs_source = cluster.gcs_source
cluster_labels = cluster.labels
cluster_name = cluster.name
cluster_satisfies_pzi = cluster.satisfies_pzi
cluster_simulate_maintenance_event = cluster.simulate_maintenance_event
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
| `metadata` | HashMap<String, String> | { `createTime`: The time the operation was created. `endTime`: The time the operation finished running. `target`: Server-defined resource path for the target of the operation. `verb`: Name of the verb executed by the operation. `statusDetail`: Human-readable status of the operation, if any. `cancelRequested`: Identifies whether the user has requested cancellation of the operation. Operations that have successfully been cancelled have Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`. `apiVersion`: API version used to start the operation. } |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.redis_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
operation_error = operation.error
operation_response = operation.response
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
| `location_id` | String | Resource ID for the region. For example: "us-east1". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Full resource name for the region. For example: "projects/example-project/locations/us-east1". |
| `metadata` | HashMap<String, String> | Output only. The set of available zones in the location. The map is keyed by the lowercase ID of each zone, as defined by Compute Engine. These keys can be specified in `location_id` or `alternative_location_id` fields when creating a Redis instance. |


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
location_location_id = location.location_id
location_labels = location.labels
location_name = location.name
location_metadata = location.metadata
```

---


### Backup_collection

Get a backup collection.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cluster` | String | Output only. The full resource path of the cluster the backup collection belongs to. Example: projects/{project}/locations/{location}/clusters/{cluster} |
| `total_backup_size_bytes` | String | Output only. Total size of all backups in the backup collection. |
| `last_backup_time` | String | Output only. The last time a backup was created in the backup collection. |
| `uid` | String | Output only. System assigned unique identifier of the backup collection. |
| `kms_key` | String | Output only. The KMS key used to encrypt the backups under this backup collection. |
| `create_time` | String | Output only. The time when the backup collection was created. |
| `total_backup_count` | String | Output only. Total number of backups in the backup collection. |
| `cluster_uid` | String | Output only. The cluster uid of the backup collection. |
| `name` | String | Identifier. Full resource path of the backup collection. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access backup_collection outputs
backup_collection_id = backup_collection.id
backup_collection_cluster = backup_collection.cluster
backup_collection_total_backup_size_bytes = backup_collection.total_backup_size_bytes
backup_collection_last_backup_time = backup_collection.last_backup_time
backup_collection_uid = backup_collection.uid
backup_collection_kms_key = backup_collection.kms_key
backup_collection_create_time = backup_collection.create_time
backup_collection_total_backup_count = backup_collection.total_backup_count
backup_collection_cluster_uid = backup_collection.cluster_uid
backup_collection_name = backup_collection.name
```

---


### Instance

Creates a Redis instance based on the specified tier and memory size. By default, the instance is accessible from the project's [default network](https://cloud.google.com/vpc/docs/vpc). The creation is executed asynchronously and callers may check the returned operation to track its progress. Once the operation is completed the Redis instance will be fully functional. The completed longrunning.Operation will contain the new instance object in the response field. The returned operation is automatically deleted after a few hours, so there is no need to call DeleteOperation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `redis_configs` | HashMap<String, String> |  | Optional. Redis configuration parameters, according to http://redis.io/topics/config. Currently, the only supported parameters are: Redis version 3.2 and newer: * maxmemory-policy * notify-keyspace-events Redis version 4.0 and newer: * activedefrag * lfu-decay-time * lfu-log-factor * maxmemory-gb Redis version 5.0 and newer: * stream-node-max-bytes * stream-node-max-entries |
| `status_message` | String |  | Output only. Additional information about the current status of this instance, if available. |
| `host` | String |  | Output only. Hostname or IP address of the exposed Redis endpoint used by clients to connect to the service. |
| `maintenance_policy` | String |  | Optional. The maintenance policy for the instance. If not provided, maintenance events can be performed at any time. |
| `display_name` | String |  | An arbitrary and optional user-provided name for the instance. |
| `read_replicas_mode` | String |  | Optional. Read replicas mode for the instance. Defaults to READ_REPLICAS_DISABLED. |
| `connect_mode` | String |  | Optional. The network connect mode of the Redis instance. If not provided, the connect mode defaults to DIRECT_PEERING. |
| `read_endpoint_port` | i64 |  | Output only. The port number of the exposed readonly redis endpoint. Standard tier only. Write requests should target 'port'. |
| `read_endpoint` | String |  | Output only. Hostname or IP address of the exposed readonly Redis endpoint. Standard tier only. Targets all healthy replica nodes in instance. Replication is asynchronous and replica nodes will exhibit some lag behind the primary. Write requests must target 'host'. |
| `replica_count` | i64 |  | Optional. The number of replica nodes. The valid range for the Standard Tier with read replicas enabled is [1-5] and defaults to 2. If read replicas are not enabled for a Standard Tier instance, the only valid value is 1 and the default is 1. The valid value for basic tier is 0 and the default is also 0. |
| `customer_managed_key` | String |  | Optional. The KMS key reference that the customer provides when trying to create the instance. |
| `server_ca_certs` | Vec<String> |  | Output only. List of server CA certificates for the instance. |
| `available_maintenance_versions` | Vec<String> |  | Optional. The available maintenance versions that an instance could update to. |
| `secondary_ip_range` | String |  | Optional. Additional IP range for node placement. Required when enabling read replicas on an existing instance. For DIRECT_PEERING mode value must be a CIDR range of size /28, or "auto". For PRIVATE_SERVICE_ACCESS mode value must be the name of an allocated address range associated with the private service access connection, or "auto". |
| `transit_encryption_mode` | String |  | Optional. The TLS mode of the Redis instance. If not provided, TLS is disabled for the instance. |
| `memory_size_gb` | i64 |  | Required. Redis memory size in GiB. |
| `redis_version` | String |  | Optional. The version of Redis software. If not provided, latest supported version will be used. Currently, the supported values are: * `REDIS_3_2` for Redis 3.2 compatibility * `REDIS_4_0` for Redis 4.0 compatibility (default) * `REDIS_5_0` for Redis 5.0 compatibility * `REDIS_6_X` for Redis 6.x compatibility * `REDIS_7_0` for Redis 7.0 compatibility |
| `maintenance_version` | String |  | Optional. The self service update maintenance version. The version is date based such as "20210712_00_00". |
| `current_location_id` | String |  | Output only. The current zone where the Redis primary node is located. In basic tier, this will always be the same as [location_id]. In standard tier, this can be the zone of any node in the instance. |
| `alternative_location_id` | String |  | Optional. If specified, at least one node will be provisioned in this zone in addition to the zone specified in location_id. Only applicable to standard tier. If provided, it must be a different zone from the one provided in [location_id]. Additional nodes beyond the first 2 will be placed in zones selected by the service. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata |
| `nodes` | Vec<String> |  | Output only. Info per node. |
| `persistence_iam_identity` | String |  | Output only. Cloud IAM identity used by import / export operations to transfer data to/from Cloud Storage. Format is "serviceAccount:". The value may change over time for a given instance so should be checked before each import/export operation. |
| `create_time` | String |  | Output only. The time the instance was created. |
| `port` | i64 |  | Output only. The port number of the exposed Redis endpoint. |
| `tier` | String |  | Required. The service tier of the instance. |
| `reserved_ip_range` | String |  | Optional. For DIRECT_PEERING mode, the CIDR range of internal addresses that are reserved for this instance. Range must be unique and non-overlapping with existing subnets in an authorized network. For PRIVATE_SERVICE_ACCESS mode, the name of one allocated IP address ranges associated with this private service access connection. If not provided, the service will choose an unused /29 block, for example, 10.0.0.0/29 or 192.168.0.0/29. For READ_REPLICAS_ENABLED the default block size is /28. |
| `suspension_reasons` | Vec<String> |  | Optional. reasons that causes instance in "SUSPENDED" state. |
| `satisfies_pzs` | bool |  | Optional. Output only. Reserved for future use. |
| `authorized_network` | String |  | Optional. The full name of the Google Compute Engine [network](https://cloud.google.com/vpc/docs/vpc) to which the instance is connected. If left unspecified, the `default` network will be used. |
| `satisfies_pzi` | bool |  | Optional. Output only. Reserved for future use. |
| `auth_enabled` | bool |  | Optional. Indicates whether OSS Redis AUTH is enabled for the instance. If set to "true" AUTH is enabled on the instance. Default value is "false" meaning AUTH is disabled. |
| `persistence_config` | String |  | Optional. Persistence configuration parameters |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `location_id` | String |  | Optional. The zone where the instance will be provisioned. If not provided, the service will choose a zone from the specified region for the instance. For standard tier, additional nodes will be added across multiple zones for protection against zonal failures. If specified, at least one node will be provisioned in this zone. |
| `state` | String |  | Output only. The current state of this instance. |
| `maintenance_schedule` | String |  | Output only. Date and time of upcoming maintenance events which have been scheduled. |
| `name` | String |  | Required. Unique name of the resource in this scope including project and location using the form: `projects/{project_id}/locations/{location_id}/instances/{instance_id}` Note: Redis instances are managed and addressed at regional level so location_id here refers to a GCP region; however, users may choose which specific zone (or collection of zones for cross-zone instances) an instance should be provisioned in. Refer to location_id and alternative_location_id fields for more details. |
| `parent` | String | ✅ | Required. The resource name of the instance location using the form: `projects/{project_id}/locations/{location_id}` where `location_id` refers to a GCP region. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `redis_configs` | HashMap<String, String> | Optional. Redis configuration parameters, according to http://redis.io/topics/config. Currently, the only supported parameters are: Redis version 3.2 and newer: * maxmemory-policy * notify-keyspace-events Redis version 4.0 and newer: * activedefrag * lfu-decay-time * lfu-log-factor * maxmemory-gb Redis version 5.0 and newer: * stream-node-max-bytes * stream-node-max-entries |
| `status_message` | String | Output only. Additional information about the current status of this instance, if available. |
| `host` | String | Output only. Hostname or IP address of the exposed Redis endpoint used by clients to connect to the service. |
| `maintenance_policy` | String | Optional. The maintenance policy for the instance. If not provided, maintenance events can be performed at any time. |
| `display_name` | String | An arbitrary and optional user-provided name for the instance. |
| `read_replicas_mode` | String | Optional. Read replicas mode for the instance. Defaults to READ_REPLICAS_DISABLED. |
| `connect_mode` | String | Optional. The network connect mode of the Redis instance. If not provided, the connect mode defaults to DIRECT_PEERING. |
| `read_endpoint_port` | i64 | Output only. The port number of the exposed readonly redis endpoint. Standard tier only. Write requests should target 'port'. |
| `read_endpoint` | String | Output only. Hostname or IP address of the exposed readonly Redis endpoint. Standard tier only. Targets all healthy replica nodes in instance. Replication is asynchronous and replica nodes will exhibit some lag behind the primary. Write requests must target 'host'. |
| `replica_count` | i64 | Optional. The number of replica nodes. The valid range for the Standard Tier with read replicas enabled is [1-5] and defaults to 2. If read replicas are not enabled for a Standard Tier instance, the only valid value is 1 and the default is 1. The valid value for basic tier is 0 and the default is also 0. |
| `customer_managed_key` | String | Optional. The KMS key reference that the customer provides when trying to create the instance. |
| `server_ca_certs` | Vec<String> | Output only. List of server CA certificates for the instance. |
| `available_maintenance_versions` | Vec<String> | Optional. The available maintenance versions that an instance could update to. |
| `secondary_ip_range` | String | Optional. Additional IP range for node placement. Required when enabling read replicas on an existing instance. For DIRECT_PEERING mode value must be a CIDR range of size /28, or "auto". For PRIVATE_SERVICE_ACCESS mode value must be the name of an allocated address range associated with the private service access connection, or "auto". |
| `transit_encryption_mode` | String | Optional. The TLS mode of the Redis instance. If not provided, TLS is disabled for the instance. |
| `memory_size_gb` | i64 | Required. Redis memory size in GiB. |
| `redis_version` | String | Optional. The version of Redis software. If not provided, latest supported version will be used. Currently, the supported values are: * `REDIS_3_2` for Redis 3.2 compatibility * `REDIS_4_0` for Redis 4.0 compatibility (default) * `REDIS_5_0` for Redis 5.0 compatibility * `REDIS_6_X` for Redis 6.x compatibility * `REDIS_7_0` for Redis 7.0 compatibility |
| `maintenance_version` | String | Optional. The self service update maintenance version. The version is date based such as "20210712_00_00". |
| `current_location_id` | String | Output only. The current zone where the Redis primary node is located. In basic tier, this will always be the same as [location_id]. In standard tier, this can be the zone of any node in the instance. |
| `alternative_location_id` | String | Optional. If specified, at least one node will be provisioned in this zone in addition to the zone specified in location_id. Only applicable to standard tier. If provided, it must be a different zone from the one provided in [location_id]. Additional nodes beyond the first 2 will be placed in zones selected by the service. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata |
| `nodes` | Vec<String> | Output only. Info per node. |
| `persistence_iam_identity` | String | Output only. Cloud IAM identity used by import / export operations to transfer data to/from Cloud Storage. Format is "serviceAccount:". The value may change over time for a given instance so should be checked before each import/export operation. |
| `create_time` | String | Output only. The time the instance was created. |
| `port` | i64 | Output only. The port number of the exposed Redis endpoint. |
| `tier` | String | Required. The service tier of the instance. |
| `reserved_ip_range` | String | Optional. For DIRECT_PEERING mode, the CIDR range of internal addresses that are reserved for this instance. Range must be unique and non-overlapping with existing subnets in an authorized network. For PRIVATE_SERVICE_ACCESS mode, the name of one allocated IP address ranges associated with this private service access connection. If not provided, the service will choose an unused /29 block, for example, 10.0.0.0/29 or 192.168.0.0/29. For READ_REPLICAS_ENABLED the default block size is /28. |
| `suspension_reasons` | Vec<String> | Optional. reasons that causes instance in "SUSPENDED" state. |
| `satisfies_pzs` | bool | Optional. Output only. Reserved for future use. |
| `authorized_network` | String | Optional. The full name of the Google Compute Engine [network](https://cloud.google.com/vpc/docs/vpc) to which the instance is connected. If left unspecified, the `default` network will be used. |
| `satisfies_pzi` | bool | Optional. Output only. Reserved for future use. |
| `auth_enabled` | bool | Optional. Indicates whether OSS Redis AUTH is enabled for the instance. If set to "true" AUTH is enabled on the instance. Default value is "false" meaning AUTH is disabled. |
| `persistence_config` | String | Optional. Persistence configuration parameters |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `location_id` | String | Optional. The zone where the instance will be provisioned. If not provided, the service will choose a zone from the specified region for the instance. For standard tier, additional nodes will be added across multiple zones for protection against zonal failures. If specified, at least one node will be provisioned in this zone. |
| `state` | String | Output only. The current state of this instance. |
| `maintenance_schedule` | String | Output only. Date and time of upcoming maintenance events which have been scheduled. |
| `name` | String | Required. Unique name of the resource in this scope including project and location using the form: `projects/{project_id}/locations/{location_id}/instances/{instance_id}` Note: Redis instances are managed and addressed at regional level so location_id here refers to a GCP region; however, users may choose which specific zone (or collection of zones for cross-zone instances) an instance should be provisioned in. Refer to location_id and alternative_location_id fields for more details. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.redis_api.Instance {
    parent = "value"  # Required. The resource name of the instance location using the form: `projects/{project_id}/locations/{location_id}` where `location_id` refers to a GCP region.
}

# Access instance outputs
instance_id = instance.id
instance_redis_configs = instance.redis_configs
instance_status_message = instance.status_message
instance_host = instance.host
instance_maintenance_policy = instance.maintenance_policy
instance_display_name = instance.display_name
instance_read_replicas_mode = instance.read_replicas_mode
instance_connect_mode = instance.connect_mode
instance_read_endpoint_port = instance.read_endpoint_port
instance_read_endpoint = instance.read_endpoint
instance_replica_count = instance.replica_count
instance_customer_managed_key = instance.customer_managed_key
instance_server_ca_certs = instance.server_ca_certs
instance_available_maintenance_versions = instance.available_maintenance_versions
instance_secondary_ip_range = instance.secondary_ip_range
instance_transit_encryption_mode = instance.transit_encryption_mode
instance_memory_size_gb = instance.memory_size_gb
instance_redis_version = instance.redis_version
instance_maintenance_version = instance.maintenance_version
instance_current_location_id = instance.current_location_id
instance_alternative_location_id = instance.alternative_location_id
instance_labels = instance.labels
instance_nodes = instance.nodes
instance_persistence_iam_identity = instance.persistence_iam_identity
instance_create_time = instance.create_time
instance_port = instance.port
instance_tier = instance.tier
instance_reserved_ip_range = instance.reserved_ip_range
instance_suspension_reasons = instance.suspension_reasons
instance_satisfies_pzs = instance.satisfies_pzs
instance_authorized_network = instance.authorized_network
instance_satisfies_pzi = instance.satisfies_pzi
instance_auth_enabled = instance.auth_enabled
instance_persistence_config = instance.persistence_config
instance_tags = instance.tags
instance_location_id = instance.location_id
instance_state = instance.state
instance_maintenance_schedule = instance.maintenance_schedule
instance_name = instance.name
```

---


### Backup

Exports a specific backup to a customer target Cloud Storage URI.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcs_bucket` | String |  | Google Cloud Storage bucket, like "my-bucket". |
| `name` | String | ✅ | Required. Redis backup resource name using the form: `projects/{project_id}/locations/{location_id}/backupCollections/{backup_collection_id}/backups/{backup_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the backup. |
| `total_size_bytes` | String | Output only. Total size of the backup in bytes. |
| `backup_files` | Vec<String> | Output only. List of backup files of the backup. |
| `create_time` | String | Output only. The time when the backup was created. |
| `cluster_uid` | String | Output only. Cluster uid of this backup. |
| `cluster` | String | Output only. Cluster resource path of this backup. |
| `backup_type` | String | Output only. Type of the backup. |
| `node_type` | String | Output only. Node type of the cluster. |
| `shard_count` | i64 | Output only. Number of shards for the cluster. |
| `uid` | String | Output only. System assigned unique identifier of the backup. |
| `name` | String | Identifier. Full resource path of the backup. the last part of the name is the backup id with the following format: [YYYYMMDDHHMMSS]_[Shorted Cluster UID] OR customer specified while backup cluster. Example: 20240515123000_1234 |
| `replica_count` | i64 | Output only. Number of replicas for the cluster. |
| `engine_version` | String | Output only. redis-7.2, valkey-7.5 |
| `expire_time` | String | Output only. The time when the backup will expire. |
| `encryption_info` | String | Output only. Encryption information of the backup. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.redis_api.Backup {
    name = "value"  # Required. Redis backup resource name using the form: `projects/{project_id}/locations/{location_id}/backupCollections/{backup_collection_id}/backups/{backup_id}`
}

# Access backup outputs
backup_id = backup.id
backup_state = backup.state
backup_total_size_bytes = backup.total_size_bytes
backup_backup_files = backup.backup_files
backup_create_time = backup.create_time
backup_cluster_uid = backup.cluster_uid
backup_cluster = backup.cluster
backup_backup_type = backup.backup_type
backup_node_type = backup.node_type
backup_shard_count = backup.shard_count
backup_uid = backup.uid
backup_name = backup.name
backup_replica_count = backup.replica_count
backup_engine_version = backup.engine_version
backup_expire_time = backup.expire_time
backup_encryption_info = backup.encryption_info
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple backup resources
backup_0 = provider.redis_api.Backup {
    name = "value-0"
}
backup_1 = provider.redis_api.Backup {
    name = "value-1"
}
backup_2 = provider.redis_api.Backup {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    backup = provider.redis_api.Backup {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Redis_api Documentation](https://cloud.google.com/redis_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
