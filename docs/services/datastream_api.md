# Datastream_api Service



**Resources**: 14

---

## Overview

The datastream_api service provides access to 14 resource types:

- [Operation](#operation) [CRD]
- [Object](#object) [CR]
- [Stream](#stream) [CRUD]
- [Private_connection](#private_connection) [CRD]
- [Route](#route) [CRD]
- [Connection_profile](#connection_profile) [CRUD]
- [Location](#location) [R]
- [Connection_profile](#connection_profile) [CRUD]
- [Location](#location) [R]
- [Private_connection](#private_connection) [CRD]
- [Stream](#stream) [CRUD]
- [Operation](#operation) [CRD]
- [Route](#route) [CRD]
- [Object](#object) [CR]

---

## Resources


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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.datastream_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
operation_response = operation.response
```

---


### Object

Use this method to start a backfill job for the specified stream object.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_filter` | String |  | Optional. Optional event filter. If not set, or empty, the backfill will be performed on the entire object. This is currently used for partial backfill and only supported for SQL Server sources. |
| `object` | String | ✅ | Required. The name of the stream object resource to start a backfill job for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. Display name. |
| `backfill_job` | String | The latest backfill job that was initiated for the stream object. |
| `errors` | Vec<String> | Output only. Active errors on the object. |
| `update_time` | String | Output only. The last update time of the object. |
| `name` | String | Output only. Identifier. The object resource's name. |
| `create_time` | String | Output only. The creation time of the object. |
| `source_object` | String | The object identifier in the data source. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create object
object = provider.datastream_api.Object {
    object = "value"  # Required. The name of the stream object resource to start a backfill job for.
}

# Access object outputs
object_id = object.id
object_display_name = object.display_name
object_backfill_job = object.backfill_job
object_errors = object.errors
object_update_time = object.update_time
object_name = object.name
object_create_time = object.create_time
object_source_object = object.source_object
```

---


### Stream

Use this method to create a stream.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `backfill_none` | String |  | Do not automatically backfill any objects. |
| `destination_config` | String |  | Required. Destination connection profile configuration. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `name` | String |  | Output only. Identifier. The stream's name. |
| `last_recovery_time` | String |  | Output only. If the stream was recovered, the time of the last recovery. Note: This field is currently experimental. |
| `display_name` | String |  | Required. Display name. |
| `state` | String |  | The state of the stream. |
| `update_time` | String |  | Output only. The last update time of the stream. |
| `source_config` | String |  | Required. Source connection profile configuration. |
| `customer_managed_encryption_key` | String |  | Immutable. A reference to a KMS encryption key. If provided, it will be used to encrypt the data. If left blank, data will be encrypted using an internal Stream-specific encryption key provisioned through KMS. |
| `create_time` | String |  | Output only. The creation time of the stream. |
| `backfill_all` | String |  | Automatically backfill objects included in the stream source configuration. Specific objects can be excluded. |
| `errors` | Vec<String> |  | Output only. Errors on the Stream. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> |  | Labels. |
| `parent` | String | ✅ | Required. The parent that owns the collection of streams. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backfill_none` | String | Do not automatically backfill any objects. |
| `destination_config` | String | Required. Destination connection profile configuration. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `name` | String | Output only. Identifier. The stream's name. |
| `last_recovery_time` | String | Output only. If the stream was recovered, the time of the last recovery. Note: This field is currently experimental. |
| `display_name` | String | Required. Display name. |
| `state` | String | The state of the stream. |
| `update_time` | String | Output only. The last update time of the stream. |
| `source_config` | String | Required. Source connection profile configuration. |
| `customer_managed_encryption_key` | String | Immutable. A reference to a KMS encryption key. If provided, it will be used to encrypt the data. If left blank, data will be encrypted using an internal Stream-specific encryption key provisioned through KMS. |
| `create_time` | String | Output only. The creation time of the stream. |
| `backfill_all` | String | Automatically backfill objects included in the stream source configuration. Specific objects can be excluded. |
| `errors` | Vec<String> | Output only. Errors on the Stream. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> | Labels. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create stream
stream = provider.datastream_api.Stream {
    parent = "value"  # Required. The parent that owns the collection of streams.
}

# Access stream outputs
stream_id = stream.id
stream_backfill_none = stream.backfill_none
stream_destination_config = stream.destination_config
stream_satisfies_pzs = stream.satisfies_pzs
stream_name = stream.name
stream_last_recovery_time = stream.last_recovery_time
stream_display_name = stream.display_name
stream_state = stream.state
stream_update_time = stream.update_time
stream_source_config = stream.source_config
stream_customer_managed_encryption_key = stream.customer_managed_encryption_key
stream_create_time = stream.create_time
stream_backfill_all = stream.backfill_all
stream_errors = stream.errors
stream_satisfies_pzi = stream.satisfies_pzi
stream_labels = stream.labels
```

---


### Private_connection

Use this method to create a private connectivity configuration.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Identifier. The resource's name. |
| `vpc_peering_config` | String |  | VPC Peering Config. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> |  | Labels. |
| `display_name` | String |  | Required. Display name. |
| `create_time` | String |  | Output only. The create time of the resource. |
| `error` | String |  | Output only. In case of error, the details of the error in a user-friendly format. |
| `psc_interface_config` | String |  | PSC Interface Config. |
| `state` | String |  | Output only. The state of the Private Connection. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. The update time of the resource. |
| `parent` | String | ✅ | Required. The parent that owns the collection of PrivateConnections. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Identifier. The resource's name. |
| `vpc_peering_config` | String | VPC Peering Config. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> | Labels. |
| `display_name` | String | Required. Display name. |
| `create_time` | String | Output only. The create time of the resource. |
| `error` | String | Output only. In case of error, the details of the error in a user-friendly format. |
| `psc_interface_config` | String | PSC Interface Config. |
| `state` | String | Output only. The state of the Private Connection. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. The update time of the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create private_connection
private_connection = provider.datastream_api.Private_connection {
    parent = "value"  # Required. The parent that owns the collection of PrivateConnections.
}

# Access private_connection outputs
private_connection_id = private_connection.id
private_connection_name = private_connection.name
private_connection_vpc_peering_config = private_connection.vpc_peering_config
private_connection_satisfies_pzi = private_connection.satisfies_pzi
private_connection_labels = private_connection.labels
private_connection_display_name = private_connection.display_name
private_connection_create_time = private_connection.create_time
private_connection_error = private_connection.error
private_connection_psc_interface_config = private_connection.psc_interface_config
private_connection_state = private_connection.state
private_connection_satisfies_pzs = private_connection.satisfies_pzs
private_connection_update_time = private_connection.update_time
```

---


### Route

Use this method to create a route for a private connectivity configuration in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The update time of the resource. |
| `labels` | HashMap<String, String> |  | Labels. |
| `create_time` | String |  | Output only. The create time of the resource. |
| `display_name` | String |  | Required. Display name. |
| `destination_address` | String |  | Required. Destination address for connection |
| `name` | String |  | Output only. Identifier. The resource's name. |
| `destination_port` | i64 |  | Destination port for connection |
| `parent` | String | ✅ | Required. The parent that owns the collection of Routes. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The update time of the resource. |
| `labels` | HashMap<String, String> | Labels. |
| `create_time` | String | Output only. The create time of the resource. |
| `display_name` | String | Required. Display name. |
| `destination_address` | String | Required. Destination address for connection |
| `name` | String | Output only. Identifier. The resource's name. |
| `destination_port` | i64 | Destination port for connection |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create route
route = provider.datastream_api.Route {
    parent = "value"  # Required. The parent that owns the collection of Routes.
}

# Access route outputs
route_id = route.id
route_update_time = route.update_time
route_labels = route.labels
route_create_time = route.create_time
route_display_name = route.display_name
route_destination_address = route.destination_address
route_name = route.name
route_destination_port = route.destination_port
```

---


### Connection_profile

Use this method to create a connection profile in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcs_profile` | String |  | Cloud Storage ConnectionProfile configuration. |
| `postgresql_profile` | String |  | PostgreSQL Connection Profile configuration. |
| `salesforce_profile` | String |  | Salesforce Connection Profile configuration. |
| `private_connectivity` | String |  | Private connectivity. |
| `oracle_profile` | String |  | Oracle ConnectionProfile configuration. |
| `bigquery_profile` | String |  | BigQuery Connection Profile configuration. |
| `create_time` | String |  | Output only. The create time of the resource. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. The update time of the resource. |
| `forward_ssh_connectivity` | String |  | Forward SSH tunnel connectivity. |
| `static_service_ip_connectivity` | String |  | Static Service IP connectivity. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> |  | Labels. |
| `mysql_profile` | String |  | MySQL ConnectionProfile configuration. |
| `name` | String |  | Output only. Identifier. The resource's name. |
| `mongodb_profile` | String |  | MongoDB Connection Profile configuration. |
| `sql_server_profile` | String |  | SQLServer Connection Profile configuration. |
| `display_name` | String |  | Required. Display name. |
| `parent` | String | ✅ | Required. The parent that owns the collection of ConnectionProfiles. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gcs_profile` | String | Cloud Storage ConnectionProfile configuration. |
| `postgresql_profile` | String | PostgreSQL Connection Profile configuration. |
| `salesforce_profile` | String | Salesforce Connection Profile configuration. |
| `private_connectivity` | String | Private connectivity. |
| `oracle_profile` | String | Oracle ConnectionProfile configuration. |
| `bigquery_profile` | String | BigQuery Connection Profile configuration. |
| `create_time` | String | Output only. The create time of the resource. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. The update time of the resource. |
| `forward_ssh_connectivity` | String | Forward SSH tunnel connectivity. |
| `static_service_ip_connectivity` | String | Static Service IP connectivity. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> | Labels. |
| `mysql_profile` | String | MySQL ConnectionProfile configuration. |
| `name` | String | Output only. Identifier. The resource's name. |
| `mongodb_profile` | String | MongoDB Connection Profile configuration. |
| `sql_server_profile` | String | SQLServer Connection Profile configuration. |
| `display_name` | String | Required. Display name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connection_profile
connection_profile = provider.datastream_api.Connection_profile {
    parent = "value"  # Required. The parent that owns the collection of ConnectionProfiles.
}

# Access connection_profile outputs
connection_profile_id = connection_profile.id
connection_profile_gcs_profile = connection_profile.gcs_profile
connection_profile_postgresql_profile = connection_profile.postgresql_profile
connection_profile_salesforce_profile = connection_profile.salesforce_profile
connection_profile_private_connectivity = connection_profile.private_connectivity
connection_profile_oracle_profile = connection_profile.oracle_profile
connection_profile_bigquery_profile = connection_profile.bigquery_profile
connection_profile_create_time = connection_profile.create_time
connection_profile_satisfies_pzi = connection_profile.satisfies_pzi
connection_profile_update_time = connection_profile.update_time
connection_profile_forward_ssh_connectivity = connection_profile.forward_ssh_connectivity
connection_profile_static_service_ip_connectivity = connection_profile.static_service_ip_connectivity
connection_profile_satisfies_pzs = connection_profile.satisfies_pzs
connection_profile_labels = connection_profile.labels
connection_profile_mysql_profile = connection_profile.mysql_profile
connection_profile_name = connection_profile.name
connection_profile_mongodb_profile = connection_profile.mongodb_profile
connection_profile_sql_server_profile = connection_profile.sql_server_profile
connection_profile_display_name = connection_profile.display_name
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_display_name = location.display_name
location_location_id = location.location_id
location_metadata = location.metadata
location_labels = location.labels
location_name = location.name
```

---


### Connection_profile

Use this method to create a connection profile in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. Display name. |
| `private_connectivity` | String |  | Private connectivity. |
| `static_service_ip_connectivity` | String |  | Static Service IP connectivity. |
| `no_connectivity` | String |  | No connectivity option chosen. |
| `oracle_profile` | String |  | Oracle ConnectionProfile configuration. |
| `create_time` | String |  | Output only. The create time of the resource. |
| `update_time` | String |  | Output only. The update time of the resource. |
| `forward_ssh_connectivity` | String |  | Forward SSH tunnel connectivity. |
| `name` | String |  | Output only. The resource's name. |
| `gcs_profile` | String |  | Cloud Storage ConnectionProfile configuration. |
| `labels` | HashMap<String, String> |  | Labels. |
| `mysql_profile` | String |  | MySQL ConnectionProfile configuration. |
| `parent` | String | ✅ | Required. The parent that owns the collection of ConnectionProfiles. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. Display name. |
| `private_connectivity` | String | Private connectivity. |
| `static_service_ip_connectivity` | String | Static Service IP connectivity. |
| `no_connectivity` | String | No connectivity option chosen. |
| `oracle_profile` | String | Oracle ConnectionProfile configuration. |
| `create_time` | String | Output only. The create time of the resource. |
| `update_time` | String | Output only. The update time of the resource. |
| `forward_ssh_connectivity` | String | Forward SSH tunnel connectivity. |
| `name` | String | Output only. The resource's name. |
| `gcs_profile` | String | Cloud Storage ConnectionProfile configuration. |
| `labels` | HashMap<String, String> | Labels. |
| `mysql_profile` | String | MySQL ConnectionProfile configuration. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connection_profile
connection_profile = provider.datastream_api.Connection_profile {
    parent = "value"  # Required. The parent that owns the collection of ConnectionProfiles.
}

# Access connection_profile outputs
connection_profile_id = connection_profile.id
connection_profile_display_name = connection_profile.display_name
connection_profile_private_connectivity = connection_profile.private_connectivity
connection_profile_static_service_ip_connectivity = connection_profile.static_service_ip_connectivity
connection_profile_no_connectivity = connection_profile.no_connectivity
connection_profile_oracle_profile = connection_profile.oracle_profile
connection_profile_create_time = connection_profile.create_time
connection_profile_update_time = connection_profile.update_time
connection_profile_forward_ssh_connectivity = connection_profile.forward_ssh_connectivity
connection_profile_name = connection_profile.name
connection_profile_gcs_profile = connection_profile.gcs_profile
connection_profile_labels = connection_profile.labels
connection_profile_mysql_profile = connection_profile.mysql_profile
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |


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
location_metadata = location.metadata
location_name = location.name
location_display_name = location.display_name
location_labels = location.labels
```

---


### Private_connection

Use this method to create a private connectivity configuration.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The update time of the resource. |
| `error` | String |  | Output only. In case of error, the details of the error in a user-friendly format. |
| `display_name` | String |  | Required. Display name. |
| `name` | String |  | Output only. The resource's name. |
| `create_time` | String |  | Output only. The create time of the resource. |
| `labels` | HashMap<String, String> |  | Labels. |
| `state` | String |  | Output only. The state of the Private Connection. |
| `vpc_peering_config` | String |  | VPC Peering Config |
| `parent` | String | ✅ | Required. The parent that owns the collection of PrivateConnections. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The update time of the resource. |
| `error` | String | Output only. In case of error, the details of the error in a user-friendly format. |
| `display_name` | String | Required. Display name. |
| `name` | String | Output only. The resource's name. |
| `create_time` | String | Output only. The create time of the resource. |
| `labels` | HashMap<String, String> | Labels. |
| `state` | String | Output only. The state of the Private Connection. |
| `vpc_peering_config` | String | VPC Peering Config |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create private_connection
private_connection = provider.datastream_api.Private_connection {
    parent = "value"  # Required. The parent that owns the collection of PrivateConnections.
}

# Access private_connection outputs
private_connection_id = private_connection.id
private_connection_update_time = private_connection.update_time
private_connection_error = private_connection.error
private_connection_display_name = private_connection.display_name
private_connection_name = private_connection.name
private_connection_create_time = private_connection.create_time
private_connection_labels = private_connection.labels
private_connection_state = private_connection.state
private_connection_vpc_peering_config = private_connection.vpc_peering_config
```

---


### Stream

Use this method to create a stream.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `backfill_none` | String |  | Do not automatically backfill any objects. |
| `backfill_all` | String |  | Automatically backfill objects included in the stream source configuration. Specific objects can be excluded. |
| `update_time` | String |  | Output only. The last update time of the stream. |
| `customer_managed_encryption_key` | String |  | Immutable. A reference to a KMS encryption key. If provided, it will be used to encrypt the data. If left blank, data will be encrypted using an internal Stream-specific encryption key provisioned through KMS. |
| `source_config` | String |  | Required. Source connection profile configuration. |
| `state` | String |  | The state of the stream. |
| `create_time` | String |  | Output only. The creation time of the stream. |
| `display_name` | String |  | Required. Display name. |
| `destination_config` | String |  | Required. Destination connection profile configuration. |
| `errors` | Vec<String> |  | Output only. Errors on the Stream. |
| `labels` | HashMap<String, String> |  | Labels. |
| `name` | String |  | Output only. The stream's name. |
| `parent` | String | ✅ | Required. The parent that owns the collection of streams. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backfill_none` | String | Do not automatically backfill any objects. |
| `backfill_all` | String | Automatically backfill objects included in the stream source configuration. Specific objects can be excluded. |
| `update_time` | String | Output only. The last update time of the stream. |
| `customer_managed_encryption_key` | String | Immutable. A reference to a KMS encryption key. If provided, it will be used to encrypt the data. If left blank, data will be encrypted using an internal Stream-specific encryption key provisioned through KMS. |
| `source_config` | String | Required. Source connection profile configuration. |
| `state` | String | The state of the stream. |
| `create_time` | String | Output only. The creation time of the stream. |
| `display_name` | String | Required. Display name. |
| `destination_config` | String | Required. Destination connection profile configuration. |
| `errors` | Vec<String> | Output only. Errors on the Stream. |
| `labels` | HashMap<String, String> | Labels. |
| `name` | String | Output only. The stream's name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create stream
stream = provider.datastream_api.Stream {
    parent = "value"  # Required. The parent that owns the collection of streams.
}

# Access stream outputs
stream_id = stream.id
stream_backfill_none = stream.backfill_none
stream_backfill_all = stream.backfill_all
stream_update_time = stream.update_time
stream_customer_managed_encryption_key = stream.customer_managed_encryption_key
stream_source_config = stream.source_config
stream_state = stream.state
stream_create_time = stream.create_time
stream_display_name = stream.display_name
stream_destination_config = stream.destination_config
stream_errors = stream.errors
stream_labels = stream.labels
stream_name = stream.name
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.datastream_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_response = operation.response
operation_name = operation.name
operation_done = operation.done
operation_metadata = operation.metadata
```

---


### Route

Use this method to create a route for a private connectivity in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The create time of the resource. |
| `labels` | HashMap<String, String> |  | Labels. |
| `update_time` | String |  | Output only. The update time of the resource. |
| `display_name` | String |  | Required. Display name. |
| `name` | String |  | Output only. The resource's name. |
| `destination_address` | String |  | Required. Destination address for connection |
| `destination_port` | i64 |  | Destination port for connection |
| `parent` | String | ✅ | Required. The parent that owns the collection of Routes. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The create time of the resource. |
| `labels` | HashMap<String, String> | Labels. |
| `update_time` | String | Output only. The update time of the resource. |
| `display_name` | String | Required. Display name. |
| `name` | String | Output only. The resource's name. |
| `destination_address` | String | Required. Destination address for connection |
| `destination_port` | i64 | Destination port for connection |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create route
route = provider.datastream_api.Route {
    parent = "value"  # Required. The parent that owns the collection of Routes.
}

# Access route outputs
route_id = route.id
route_create_time = route.create_time
route_labels = route.labels
route_update_time = route.update_time
route_display_name = route.display_name
route_name = route.name
route_destination_address = route.destination_address
route_destination_port = route.destination_port
```

---


### Object

Starts backfill job for the specified stream object.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `object` | String | ✅ | Required. The name of the stream object resource to start a backfill job for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `source_object` | String | The object identifier in the data source. |
| `update_time` | String | Output only. The last update time of the object. |
| `display_name` | String | Required. Display name. |
| `backfill_job` | String | The latest backfill job that was initiated for the stream object. |
| `errors` | Vec<String> | Output only. Active errors on the object. |
| `name` | String | Output only. The object's name. |
| `create_time` | String | Output only. The creation time of the object. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create object
object = provider.datastream_api.Object {
    object = "value"  # Required. The name of the stream object resource to start a backfill job for.
}

# Access object outputs
object_id = object.id
object_source_object = object.source_object
object_update_time = object.update_time
object_display_name = object.display_name
object_backfill_job = object.backfill_job
object_errors = object.errors
object_name = object.name
object_create_time = object.create_time
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
operation_0 = provider.datastream_api.Operation {
    name = "value-0"
}
operation_1 = provider.datastream_api.Operation {
    name = "value-1"
}
operation_2 = provider.datastream_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.datastream_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Datastream_api Documentation](https://cloud.google.com/datastream_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
