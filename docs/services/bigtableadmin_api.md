# Bigtableadmin_api Service



**Resources**: 12

---

## Overview

The bigtableadmin_api service provides access to 12 resource types:

- [App_profile](#app_profile) [CRUD]
- [Location](#location) [R]
- [Authorized_view](#authorized_view) [CRUD]
- [Hot_tablet](#hot_tablet) [R]
- [Logical_view](#logical_view) [CRUD]
- [Table](#table) [CRUD]
- [Schema_bundle](#schema_bundle) [CRUD]
- [Materialized_view](#materialized_view) [CRUD]
- [Cluster](#cluster) [CRUD]
- [Instance](#instance) [CRUD]
- [Operation](#operation) [R]
- [Backup](#backup) [CRUD]

---

## Resources


### App_profile

Creates an app profile within an instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Long form description of the use case for this AppProfile. |
| `priority` | String |  | This field has been deprecated in favor of `standard_isolation.priority`. If you set this field, `standard_isolation.priority` will be set instead. The priority of requests sent using this app profile. |
| `name` | String |  | The unique name of the app profile, up to 50 characters long. Values are of the form `projects/{project}/instances/{instance}/appProfiles/_a-zA-Z0-9*`. |
| `etag` | String |  | Strongly validated etag for optimistic concurrency control. Preserve the value returned from `GetAppProfile` when calling `UpdateAppProfile` to fail the request if there has been a modification in the mean time. The `update_mask` of the request need not include `etag` for this protection to apply. See [Wikipedia](https://en.wikipedia.org/wiki/HTTP_ETag) and [RFC 7232](https://tools.ietf.org/html/rfc7232#section-2.3) for more details. |
| `single_cluster_routing` | String |  | Use a single-cluster routing policy. |
| `standard_isolation` | String |  | The standard options used for isolating this app profile's traffic from other use cases. |
| `multi_cluster_routing_use_any` | String |  | Use a multi-cluster routing policy. |
| `data_boost_isolation_read_only` | String |  | Specifies that this app profile is intended for read-only usage via the Data Boost feature. |
| `parent` | String | ✅ | Required. The unique name of the instance in which to create the new app profile. Values are of the form `projects/{project}/instances/{instance}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Long form description of the use case for this AppProfile. |
| `priority` | String | This field has been deprecated in favor of `standard_isolation.priority`. If you set this field, `standard_isolation.priority` will be set instead. The priority of requests sent using this app profile. |
| `name` | String | The unique name of the app profile, up to 50 characters long. Values are of the form `projects/{project}/instances/{instance}/appProfiles/_a-zA-Z0-9*`. |
| `etag` | String | Strongly validated etag for optimistic concurrency control. Preserve the value returned from `GetAppProfile` when calling `UpdateAppProfile` to fail the request if there has been a modification in the mean time. The `update_mask` of the request need not include `etag` for this protection to apply. See [Wikipedia](https://en.wikipedia.org/wiki/HTTP_ETag) and [RFC 7232](https://tools.ietf.org/html/rfc7232#section-2.3) for more details. |
| `single_cluster_routing` | String | Use a single-cluster routing policy. |
| `standard_isolation` | String | The standard options used for isolating this app profile's traffic from other use cases. |
| `multi_cluster_routing_use_any` | String | Use a multi-cluster routing policy. |
| `data_boost_isolation_read_only` | String | Specifies that this app profile is intended for read-only usage via the Data Boost feature. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create app_profile
app_profile = provider.bigtableadmin_api.App_profile {
    parent = "value"  # Required. The unique name of the instance in which to create the new app profile. Values are of the form `projects/{project}/instances/{instance}`.
}

# Access app_profile outputs
app_profile_id = app_profile.id
app_profile_description = app_profile.description
app_profile_priority = app_profile.priority
app_profile_name = app_profile.name
app_profile_etag = app_profile.etag
app_profile_single_cluster_routing = app_profile.single_cluster_routing
app_profile_standard_isolation = app_profile.standard_isolation
app_profile_multi_cluster_routing_use_any = app_profile.multi_cluster_routing_use_any
app_profile_data_boost_isolation_read_only = app_profile.data_boost_isolation_read_only
```

---


### Location

Lists information about the supported locations for this service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `locations` | Vec<String> | A list of locations that matches the specified filter in the request. |
| `next_page_token` | String | The standard List next-page token. |


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
location_locations = location.locations
location_next_page_token = location.next_page_token
```

---


### Authorized_view

Creates a new AuthorizedView in a table.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The name of this AuthorizedView. Values are of the form `projects/{project}/instances/{instance}/tables/{table}/authorizedViews/{authorized_view}` |
| `deletion_protection` | bool |  | Set to true to make the AuthorizedView protected against deletion. The parent Table and containing Instance cannot be deleted if an AuthorizedView has this bit set. |
| `etag` | String |  | The etag for this AuthorizedView. If this is provided on update, it must match the server's etag. The server returns ABORTED error on a mismatched etag. |
| `subset_view` | String |  | An AuthorizedView permitting access to an explicit subset of a Table. |
| `parent` | String | ✅ | Required. This is the name of the table the AuthorizedView belongs to. Values are of the form `projects/{project}/instances/{instance}/tables/{table}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The name of this AuthorizedView. Values are of the form `projects/{project}/instances/{instance}/tables/{table}/authorizedViews/{authorized_view}` |
| `deletion_protection` | bool | Set to true to make the AuthorizedView protected against deletion. The parent Table and containing Instance cannot be deleted if an AuthorizedView has this bit set. |
| `etag` | String | The etag for this AuthorizedView. If this is provided on update, it must match the server's etag. The server returns ABORTED error on a mismatched etag. |
| `subset_view` | String | An AuthorizedView permitting access to an explicit subset of a Table. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create authorized_view
authorized_view = provider.bigtableadmin_api.Authorized_view {
    parent = "value"  # Required. This is the name of the table the AuthorizedView belongs to. Values are of the form `projects/{project}/instances/{instance}/tables/{table}`.
}

# Access authorized_view outputs
authorized_view_id = authorized_view.id
authorized_view_name = authorized_view.name
authorized_view_deletion_protection = authorized_view.deletion_protection
authorized_view_etag = authorized_view.etag
authorized_view_subset_view = authorized_view.subset_view
```

---


### Hot_tablet

Lists hot tablets in a cluster, within the time range provided. Hot tablets are ordered based on CPU usage.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hot_tablets` | Vec<String> | List of hot tablets in the tables of the requested cluster that fall within the requested time range. Hot tablets are ordered by node cpu usage percent. If there are multiple hot tablets that correspond to the same tablet within a 15-minute interval, only the hot tablet with the highest node cpu usage will be included in the response. |
| `next_page_token` | String | Set if not all hot tablets could be returned in a single response. Pass this value to `page_token` in another request to get the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access hot_tablet outputs
hot_tablet_id = hot_tablet.id
hot_tablet_hot_tablets = hot_tablet.hot_tablets
hot_tablet_next_page_token = hot_tablet.next_page_token
```

---


### Logical_view

Creates a logical view within an instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Optional. The etag for this logical view. This may be sent on update requests to ensure that the client has an up-to-date value before proceeding. The server returns an ABORTED error on a mismatched etag. |
| `name` | String |  | Identifier. The unique name of the logical view. Format: `projects/{project}/instances/{instance}/logicalViews/{logical_view}` |
| `deletion_protection` | bool |  | Optional. Set to true to make the LogicalView protected against deletion. |
| `query` | String |  | Required. The logical view's select query. |
| `parent` | String | ✅ | Required. The parent instance where this logical view will be created. Format: `projects/{project}/instances/{instance}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Optional. The etag for this logical view. This may be sent on update requests to ensure that the client has an up-to-date value before proceeding. The server returns an ABORTED error on a mismatched etag. |
| `name` | String | Identifier. The unique name of the logical view. Format: `projects/{project}/instances/{instance}/logicalViews/{logical_view}` |
| `deletion_protection` | bool | Optional. Set to true to make the LogicalView protected against deletion. |
| `query` | String | Required. The logical view's select query. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create logical_view
logical_view = provider.bigtableadmin_api.Logical_view {
    parent = "value"  # Required. The parent instance where this logical view will be created. Format: `projects/{project}/instances/{instance}`.
}

# Access logical_view outputs
logical_view_id = logical_view.id
logical_view_etag = logical_view.etag
logical_view_name = logical_view.name
logical_view_deletion_protection = logical_view.deletion_protection
logical_view_query = logical_view.query
```

---


### Table

Creates a new table in the specified instance. The table can be created with a full set of initial column families, specified in the request.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `table` | String |  | Required. The Table to create. |
| `table_id` | String |  | Required. The name by which the new table should be referred to within the parent instance, e.g., `foobar` rather than `{parent}/tables/foobar`. Maximum 50 characters. |
| `initial_splits` | Vec<String> |  | The optional list of row keys that will be used to initially split the table into several tablets (tablets are similar to HBase regions). Given two split keys, `s1` and `s2`, three tablets will be created, spanning the key ranges: `[, s1), [s1, s2), [s2, )`. Example: * Row keys := `["a", "apple", "custom", "customer_1", "customer_2",` `"other", "zz"]` * initial_split_keys := `["apple", "customer_1", "customer_2", "other"]` * Key assignment: - Tablet 1 `[, apple) => {"a"}.` - Tablet 2 `[apple, customer_1) => {"apple", "custom"}.` - Tablet 3 `[customer_1, customer_2) => {"customer_1"}.` - Tablet 4 `[customer_2, other) => {"customer_2"}.` - Tablet 5 `[other, ) => {"other", "zz"}.` |
| `parent` | String | ✅ | Required. The unique name of the instance in which to create the table. Values are of the form `projects/{project}/instances/{instance}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The unique name of the table. Values are of the form `projects/{project}/instances/{instance}/tables/_a-zA-Z0-9*`. Views: `NAME_ONLY`, `SCHEMA_VIEW`, `REPLICATION_VIEW`, `STATS_VIEW`, `FULL` |
| `granularity` | String | Immutable. The granularity (i.e. `MILLIS`) at which timestamps are stored in this table. Timestamps not matching the granularity will be rejected. If unspecified at creation time, the value will be set to `MILLIS`. Views: `SCHEMA_VIEW`, `FULL`. |
| `restore_info` | String | Output only. If this table was restored from another data source (e.g. a backup), this field will be populated with information about the restore. |
| `row_key_schema` | String | The row key schema for this table. The schema is used to decode the raw row key bytes into a structured format. The order of field declarations in this schema is important, as it reflects how the raw row key bytes are structured. Currently, this only affects how the key is read via a GoogleSQL query from the ExecuteQuery API. For a SQL query, the _key column is still read as raw bytes. But queries can reference the key fields by name, which will be decoded from _key using provided type and encoding. Queries that reference key fields will fail if they encounter an invalid row key. For example, if _key = "some_id#2024-04-30#\x00\x13\x00\xf3" with the following schema: { fields { field_name: "id" type { string { encoding: utf8_bytes {} } } } fields { field_name: "date" type { string { encoding: utf8_bytes {} } } } fields { field_name: "product_code" type { int64 { encoding: big_endian_bytes {} } } } encoding { delimited_bytes { delimiter: "#" } } } The decoded key parts would be: id = "some_id", date = "2024-04-30", product_code = 1245427 The query "SELECT _key, product_code FROM table" will return two columns: /------------------------------------------------------\ | _key | product_code | | --------------------------------------|--------------| | "some_id#2024-04-30#\x00\x13\x00\xf3" | 1245427 | \------------------------------------------------------/ The schema has the following invariants: (1) The decoded field values are order-preserved. For read, the field values will be decoded in sorted mode from the raw bytes. (2) Every field in the schema must specify a non-empty name. (3) Every field must specify a type with an associated encoding. The type is limited to scalar types only: Array, Map, Aggregate, and Struct are not allowed. (4) The field names must not collide with existing column family names and reserved keywords "_key" and "_timestamp". The following update operations are allowed for row_key_schema: - Update from an empty schema to a new schema. - Remove the existing schema. This operation requires setting the `ignore_warnings` flag to `true`, since it might be a backward incompatible change. Without the flag, the update request will fail with an INVALID_ARGUMENT error. Any other row key schema update operation (e.g. update existing schema columns names or types) is currently unsupported. |
| `automated_backup_policy` | String | If specified, automated backups are enabled for this table. Otherwise, automated backups are disabled. |
| `cluster_states` | HashMap<String, String> | Output only. Map from cluster ID to per-cluster table state. If it could not be determined whether or not the table has data in a particular cluster (for example, if its zone is unavailable), then there will be an entry for the cluster with UNKNOWN `replication_status`. Views: `REPLICATION_VIEW`, `ENCRYPTION_VIEW`, `FULL` |
| `tiered_storage_config` | String | Rules to specify what data is stored in each storage tier. Different tiers store data differently, providing different trade-offs between cost and performance. Different parts of a table can be stored separately on different tiers. If a config is specified, tiered storage is enabled for this table. Otherwise, tiered storage is disabled. Only SSD instances can configure tiered storage. |
| `stats` | String | Output only. Only available with STATS_VIEW, this includes summary statistics about the entire table contents. For statistics about a specific column family, see ColumnFamilyStats in the mapped ColumnFamily collection above. |
| `column_families` | HashMap<String, String> | The column families configured for this table, mapped by column family ID. Views: `SCHEMA_VIEW`, `STATS_VIEW`, `FULL` |
| `deletion_protection` | bool | Set to true to make the table protected against data loss. i.e. deleting the following resources through Admin APIs are prohibited: * The table. * The column families in the table. * The instance containing the table. Note one can still delete the data stored in the table through Data APIs. |
| `change_stream_config` | String | If specified, enable the change stream on this table. Otherwise, the change stream is disabled and the change stream is not retained. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create table
table = provider.bigtableadmin_api.Table {
    parent = "value"  # Required. The unique name of the instance in which to create the table. Values are of the form `projects/{project}/instances/{instance}`.
}

# Access table outputs
table_id = table.id
table_name = table.name
table_granularity = table.granularity
table_restore_info = table.restore_info
table_row_key_schema = table.row_key_schema
table_automated_backup_policy = table.automated_backup_policy
table_cluster_states = table.cluster_states
table_tiered_storage_config = table.tiered_storage_config
table_stats = table.stats
table_column_families = table.column_families
table_deletion_protection = table.deletion_protection
table_change_stream_config = table.change_stream_config
```

---


### Schema_bundle

Creates a new schema bundle in the specified table.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `proto_schema` | String |  | Schema for Protobufs. |
| `name` | String |  | Identifier. The unique name identifying this schema bundle. Values are of the form `projects/{project}/instances/{instance}/tables/{table}/schemaBundles/{schema_bundle}` |
| `etag` | String |  | Optional. The etag for this schema bundle. This may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. The server returns an ABORTED error on a mismatched etag. |
| `parent` | String | ✅ | Required. The parent resource where this schema bundle will be created. Values are of the form `projects/{project}/instances/{instance}/tables/{table}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `proto_schema` | String | Schema for Protobufs. |
| `name` | String | Identifier. The unique name identifying this schema bundle. Values are of the form `projects/{project}/instances/{instance}/tables/{table}/schemaBundles/{schema_bundle}` |
| `etag` | String | Optional. The etag for this schema bundle. This may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. The server returns an ABORTED error on a mismatched etag. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create schema_bundle
schema_bundle = provider.bigtableadmin_api.Schema_bundle {
    parent = "value"  # Required. The parent resource where this schema bundle will be created. Values are of the form `projects/{project}/instances/{instance}/tables/{table}`.
}

# Access schema_bundle outputs
schema_bundle_id = schema_bundle.id
schema_bundle_proto_schema = schema_bundle.proto_schema
schema_bundle_name = schema_bundle.name
schema_bundle_etag = schema_bundle.etag
```

---


### Materialized_view

Creates a materialized view within an instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Optional. The etag for this materialized view. This may be sent on update requests to ensure that the client has an up-to-date value before proceeding. The server returns an ABORTED error on a mismatched etag. Views: `SCHEMA_VIEW`, `REPLICATION_VIEW`, `FULL`. |
| `name` | String |  | Identifier. The unique name of the materialized view. Format: `projects/{project}/instances/{instance}/materializedViews/{materialized_view}` Views: `SCHEMA_VIEW`, `REPLICATION_VIEW`, `FULL`. |
| `cluster_states` | HashMap<String, String> |  | Output only. Map from cluster ID to per-cluster materialized view state. If it could not be determined whether or not the materialized view has data in a particular cluster (for example, if its zone is unavailable), then there will be an entry for the cluster with `STATE_NOT_KNOWN` state. Views: `REPLICATION_VIEW`, `FULL`. |
| `deletion_protection` | bool |  | Set to true to make the MaterializedView protected against deletion. Views: `SCHEMA_VIEW`, `REPLICATION_VIEW`, `FULL`. |
| `query` | String |  | Required. Immutable. The materialized view's select query. Views: `SCHEMA_VIEW`, `FULL`. |
| `parent` | String | ✅ | Required. The parent instance where this materialized view will be created. Format: `projects/{project}/instances/{instance}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Optional. The etag for this materialized view. This may be sent on update requests to ensure that the client has an up-to-date value before proceeding. The server returns an ABORTED error on a mismatched etag. Views: `SCHEMA_VIEW`, `REPLICATION_VIEW`, `FULL`. |
| `name` | String | Identifier. The unique name of the materialized view. Format: `projects/{project}/instances/{instance}/materializedViews/{materialized_view}` Views: `SCHEMA_VIEW`, `REPLICATION_VIEW`, `FULL`. |
| `cluster_states` | HashMap<String, String> | Output only. Map from cluster ID to per-cluster materialized view state. If it could not be determined whether or not the materialized view has data in a particular cluster (for example, if its zone is unavailable), then there will be an entry for the cluster with `STATE_NOT_KNOWN` state. Views: `REPLICATION_VIEW`, `FULL`. |
| `deletion_protection` | bool | Set to true to make the MaterializedView protected against deletion. Views: `SCHEMA_VIEW`, `REPLICATION_VIEW`, `FULL`. |
| `query` | String | Required. Immutable. The materialized view's select query. Views: `SCHEMA_VIEW`, `FULL`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create materialized_view
materialized_view = provider.bigtableadmin_api.Materialized_view {
    parent = "value"  # Required. The parent instance where this materialized view will be created. Format: `projects/{project}/instances/{instance}`.
}

# Access materialized_view outputs
materialized_view_id = materialized_view.id
materialized_view_etag = materialized_view.etag
materialized_view_name = materialized_view.name
materialized_view_cluster_states = materialized_view.cluster_states
materialized_view_deletion_protection = materialized_view.deletion_protection
materialized_view_query = materialized_view.query
```

---


### Cluster

Creates a cluster within an instance. Note that exactly one of Cluster.serve_nodes and Cluster.cluster_config.cluster_autoscaling_config can be set. If serve_nodes is set to non-zero, then the cluster is manually scaled. If cluster_config.cluster_autoscaling_config is non-empty, then autoscaling is enabled.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `serve_nodes` | i64 |  | The number of nodes in the cluster. If no value is set, Cloud Bigtable automatically allocates nodes based on your data footprint and optimized for 50% storage utilization. |
| `name` | String |  | The unique name of the cluster. Values are of the form `projects/{project}/instances/{instance}/clusters/a-z*`. |
| `default_storage_type` | String |  | Immutable. The type of storage used by this cluster to serve its parent instance's tables, unless explicitly overridden. |
| `encryption_config` | String |  | Immutable. The encryption configuration for CMEK-protected clusters. |
| `node_scaling_factor` | String |  | Immutable. The node scaling factor of this cluster. |
| `state` | String |  | Output only. The current state of the cluster. |
| `cluster_config` | String |  | Configuration for this cluster. |
| `location` | String |  | Immutable. The location where this cluster's nodes and storage reside. For best performance, clients should be located as close as possible to this cluster. Currently only zones are supported, so values should be of the form `projects/{project}/locations/{zone}`. |
| `parent` | String | ✅ | Required. The unique name of the instance in which to create the new cluster. Values are of the form `projects/{project}/instances/{instance}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `serve_nodes` | i64 | The number of nodes in the cluster. If no value is set, Cloud Bigtable automatically allocates nodes based on your data footprint and optimized for 50% storage utilization. |
| `name` | String | The unique name of the cluster. Values are of the form `projects/{project}/instances/{instance}/clusters/a-z*`. |
| `default_storage_type` | String | Immutable. The type of storage used by this cluster to serve its parent instance's tables, unless explicitly overridden. |
| `encryption_config` | String | Immutable. The encryption configuration for CMEK-protected clusters. |
| `node_scaling_factor` | String | Immutable. The node scaling factor of this cluster. |
| `state` | String | Output only. The current state of the cluster. |
| `cluster_config` | String | Configuration for this cluster. |
| `location` | String | Immutable. The location where this cluster's nodes and storage reside. For best performance, clients should be located as close as possible to this cluster. Currently only zones are supported, so values should be of the form `projects/{project}/locations/{zone}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cluster
cluster = provider.bigtableadmin_api.Cluster {
    parent = "value"  # Required. The unique name of the instance in which to create the new cluster. Values are of the form `projects/{project}/instances/{instance}`.
}

# Access cluster outputs
cluster_id = cluster.id
cluster_serve_nodes = cluster.serve_nodes
cluster_name = cluster.name
cluster_default_storage_type = cluster.default_storage_type
cluster_encryption_config = cluster.encryption_config
cluster_node_scaling_factor = cluster.node_scaling_factor
cluster_state = cluster.state
cluster_cluster_config = cluster.cluster_config
cluster_location = cluster.location
```

---


### Instance

Create an instance within a project. Note that exactly one of Cluster.serve_nodes and Cluster.cluster_config.cluster_autoscaling_config can be set. If serve_nodes is set to non-zero, then the cluster is manually scaled. If cluster_config.cluster_autoscaling_config is non-empty, then autoscaling is enabled.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `clusters` | HashMap<String, String> |  | Required. The clusters to be created within the instance, mapped by desired cluster ID, e.g., just `mycluster` rather than `projects/myproject/instances/myinstance/clusters/mycluster`. Fields marked `OutputOnly` must be left blank. |
| `parent` | String |  | Required. The unique name of the project in which to create the new instance. Values are of the form `projects/{project}`. |
| `instance` | String |  | Required. The instance to create. Fields marked `OutputOnly` must be left blank. |
| `instance_id` | String |  | Required. The ID to be used when referring to the new instance within its project, e.g., just `myinstance` rather than `projects/myproject/instances/myinstance`. |
| `parent` | String | ✅ | Required. The unique name of the project in which to create the new instance. Values are of the form `projects/{project}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The descriptive name for this instance as it appears in UIs. Can be changed at any time, but should be kept globally unique to avoid confusion. |
| `state` | String | Output only. The current state of the instance. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: - "123/environment": "production", - "123/costCenter": "marketing" Tags and Labels (above) are both used to bind metadata to resources, with different use-cases. See https://cloud.google.com/resource-manager/docs/tags/tags-overview for an in-depth overview on the difference between tags and labels. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `type` | String | The type of the instance. Defaults to `PRODUCTION`. |
| `create_time` | String | Output only. A commit timestamp representing when this Instance was created. For instances created before this field was added (August 2021), this value is `seconds: 0, nanos: 1`. |
| `labels` | HashMap<String, String> | Labels are a flexible and lightweight mechanism for organizing cloud resources into groups that reflect a customer's organizational needs and deployment strategies. They can be used to filter resources and aggregate metrics. * Label keys must be between 1 and 63 characters long and must conform to the regular expression: `\p{Ll}\p{Lo}{0,62}`. * Label values must be between 0 and 63 characters long and must conform to the regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}`. * No more than 64 labels can be associated with a given resource. * Keys and values must both be under 128 bytes. |
| `name` | String | The unique name of the instance. Values are of the form `projects/{project}/instances/a-z+[a-z0-9]`. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.bigtableadmin_api.Instance {
    parent = "value"  # Required. The unique name of the project in which to create the new instance. Values are of the form `projects/{project}`.
}

# Access instance outputs
instance_id = instance.id
instance_display_name = instance.display_name
instance_state = instance.state
instance_tags = instance.tags
instance_satisfies_pzs = instance.satisfies_pzs
instance_type = instance.type
instance_create_time = instance.create_time
instance_labels = instance.labels
instance_name = instance.name
instance_satisfies_pzi = instance.satisfies_pzi
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


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

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
```

---


### Backup

Starts creating a new Cloud Bigtable Backup. The returned backup long-running operation can be used to track creation of the backup. The metadata field type is CreateBackupMetadata. The response field type is Backup, if successful. Cancelling the returned operation will stop the creation and delete the backup.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_time` | String |  | Output only. `end_time` is the time that the backup was finished. The row data in the backup will be no newer than this timestamp. |
| `hot_to_standard_time` | String |  | The time at which the hot backup will be converted to a standard backup. Once the `hot_to_standard_time` has passed, Cloud Bigtable will convert the hot backup to a standard backup. This value must be greater than the backup creation time by: - At least 24 hours This field only applies for hot backups. When creating or updating a standard backup, attempting to set this field will fail the request. |
| `expire_time` | String |  | Required. The expiration time of the backup. When creating a backup or updating its `expire_time`, the value must be greater than the backup creation time by: - At least 6 hours - At most 90 days Once the `expire_time` has passed, Cloud Bigtable will delete the backup. |
| `name` | String |  | A globally unique identifier for the backup which cannot be changed. Values are of the form `projects/{project}/instances/{instance}/clusters/{cluster}/ backups/_a-zA-Z0-9*` The final segment of the name must be between 1 and 50 characters in length. The backup is stored in the cluster identified by the prefix of the backup name of the form `projects/{project}/instances/{instance}/clusters/{cluster}`. |
| `encryption_info` | String |  | Output only. The encryption information for the backup. |
| `backup_type` | String |  | Indicates the backup type of the backup. |
| `size_bytes` | String |  | Output only. Size of the backup in bytes. |
| `source_backup` | String |  | Output only. Name of the backup from which this backup was copied. If a backup is not created by copying a backup, this field will be empty. Values are of the form: projects//instances//clusters//backups/ |
| `source_table` | String |  | Required. Immutable. Name of the table from which this backup was created. This needs to be in the same instance as the backup. Values are of the form `projects/{project}/instances/{instance}/tables/{source_table}`. |
| `start_time` | String |  | Output only. `start_time` is the time that the backup was started (i.e. approximately the time the CreateBackup request is received). The row data in this backup will be no older than this timestamp. |
| `state` | String |  | Output only. The current state of the backup. |
| `parent` | String | ✅ | Required. This must be one of the clusters in the instance in which this table is located. The backup will be stored in this cluster. Values are of the form `projects/{project}/instances/{instance}/clusters/{cluster}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Output only. `end_time` is the time that the backup was finished. The row data in the backup will be no newer than this timestamp. |
| `hot_to_standard_time` | String | The time at which the hot backup will be converted to a standard backup. Once the `hot_to_standard_time` has passed, Cloud Bigtable will convert the hot backup to a standard backup. This value must be greater than the backup creation time by: - At least 24 hours This field only applies for hot backups. When creating or updating a standard backup, attempting to set this field will fail the request. |
| `expire_time` | String | Required. The expiration time of the backup. When creating a backup or updating its `expire_time`, the value must be greater than the backup creation time by: - At least 6 hours - At most 90 days Once the `expire_time` has passed, Cloud Bigtable will delete the backup. |
| `name` | String | A globally unique identifier for the backup which cannot be changed. Values are of the form `projects/{project}/instances/{instance}/clusters/{cluster}/ backups/_a-zA-Z0-9*` The final segment of the name must be between 1 and 50 characters in length. The backup is stored in the cluster identified by the prefix of the backup name of the form `projects/{project}/instances/{instance}/clusters/{cluster}`. |
| `encryption_info` | String | Output only. The encryption information for the backup. |
| `backup_type` | String | Indicates the backup type of the backup. |
| `size_bytes` | String | Output only. Size of the backup in bytes. |
| `source_backup` | String | Output only. Name of the backup from which this backup was copied. If a backup is not created by copying a backup, this field will be empty. Values are of the form: projects//instances//clusters//backups/ |
| `source_table` | String | Required. Immutable. Name of the table from which this backup was created. This needs to be in the same instance as the backup. Values are of the form `projects/{project}/instances/{instance}/tables/{source_table}`. |
| `start_time` | String | Output only. `start_time` is the time that the backup was started (i.e. approximately the time the CreateBackup request is received). The row data in this backup will be no older than this timestamp. |
| `state` | String | Output only. The current state of the backup. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.bigtableadmin_api.Backup {
    parent = "value"  # Required. This must be one of the clusters in the instance in which this table is located. The backup will be stored in this cluster. Values are of the form `projects/{project}/instances/{instance}/clusters/{cluster}`.
}

# Access backup outputs
backup_id = backup.id
backup_end_time = backup.end_time
backup_hot_to_standard_time = backup.hot_to_standard_time
backup_expire_time = backup.expire_time
backup_name = backup.name
backup_encryption_info = backup.encryption_info
backup_backup_type = backup.backup_type
backup_size_bytes = backup.size_bytes
backup_source_backup = backup.source_backup
backup_source_table = backup.source_table
backup_start_time = backup.start_time
backup_state = backup.state
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple app_profile resources
app_profile_0 = provider.bigtableadmin_api.App_profile {
    parent = "value-0"
}
app_profile_1 = provider.bigtableadmin_api.App_profile {
    parent = "value-1"
}
app_profile_2 = provider.bigtableadmin_api.App_profile {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    app_profile = provider.bigtableadmin_api.App_profile {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Bigtableadmin_api Documentation](https://cloud.google.com/bigtableadmin_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
