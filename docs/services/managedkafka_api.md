# Managedkafka_api Service



**Resources**: 18

---

## Overview

The managedkafka_api service provides access to 18 resource types:

- [Consumer_group](#consumer_group) [RUD]
- [Subject](#subject) [CRD]
- [Mode](#mode) [RUD]
- [Cluster](#cluster) [CRUD]
- [Connector](#connector) [CRUD]
- [Config](#config) [RUD]
- [Type](#type) [R]
- [Schema_registrie](#schema_registrie) [CRD]
- [Compatibility](#compatibility) [C]
- [Version](#version) [CRD]
- [Schema](#schema) [R]
- [Topic](#topic) [CRUD]
- [Context](#context) [R]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Connect_cluster](#connect_cluster) [CRUD]
- [Acl](#acl) [CRUD]
- [Referencedby](#referencedby) [R]

---

## Resources


### Consumer_group

Returns the properties of a single consumer group.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `topics` | HashMap<String, String> |  | Optional. Metadata for this consumer group for all topics it has metadata for. The key of the map is a topic name, structured like: projects/{project}/locations/{location}/clusters/{cluster}/topics/{topic} |
| `name` | String |  | Identifier. The name of the consumer group. The `consumer_group` segment is used when connecting directly to the cluster. Structured like: projects/{project}/locations/{location}/clusters/{cluster}/consumerGroups/{consumer_group} |
| `name` | String | ✅ | Identifier. The name of the consumer group. The `consumer_group` segment is used when connecting directly to the cluster. Structured like: projects/{project}/locations/{location}/clusters/{cluster}/consumerGroups/{consumer_group} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `topics` | HashMap<String, String> | Optional. Metadata for this consumer group for all topics it has metadata for. The key of the map is a topic name, structured like: projects/{project}/locations/{location}/clusters/{cluster}/topics/{topic} |
| `name` | String | Identifier. The name of the consumer group. The `consumer_group` segment is used when connecting directly to the cluster. Structured like: projects/{project}/locations/{location}/clusters/{cluster}/consumerGroups/{consumer_group} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access consumer_group outputs
consumer_group_id = consumer_group.id
consumer_group_topics = consumer_group.topics
consumer_group_name = consumer_group.name
```

---


### Subject

Lookup a schema under the specified subject.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_type` | String |  | Optional. The schema type of the schema. |
| `schema` | String |  | Required. The schema payload |
| `normalize` | bool |  | Optional. If true, the schema will be normalized before being looked up. The default is false. |
| `deleted` | bool |  | Optional. If true, soft-deleted versions will be included in lookup, no matter if the subject is active or soft-deleted. If false, soft-deleted versions will be excluded. The default is false. |
| `references` | Vec<String> |  | Optional. The schema references used by the schema. |
| `parent` | String | ✅ | Required. The subject to lookup the schema in. Structured like: `projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/subjects/{subject}` or `projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/contexts/{context}/subjects/{subject}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The HTTP request/response body as raw binary. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create subject
subject = provider.managedkafka_api.Subject {
    parent = "value"  # Required. The subject to lookup the schema in. Structured like: `projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/subjects/{subject}` or `projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/contexts/{context}/subjects/{subject}`
}

# Access subject outputs
subject_id = subject.id
subject_data = subject.data
subject_content_type = subject.content_type
subject_extensions = subject.extensions
```

---


### Mode

Get mode at global level or for a subject.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `mode` | String |  | Required. The mode type. |
| `name` | String | ✅ | Required. The resource name of the mode. The format is * projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/mode/{subject}: mode for a schema registry, or * projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/contexts/{context}/mode/{subject}: mode for a specific subject in a specific context |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mode` | String | Required. The mode type of a schema registry (READWRITE by default) or of a subject (unset by default, which means use the global schema registry setting). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access mode outputs
mode_id = mode.id
mode_mode = mode.mode
```

---


### Cluster

Creates a new cluster in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `update_options` | String |  | Optional. UpdateOptions represents options that control how updates to the cluster are applied. |
| `gcp_config` | String |  | Required. Configuration properties for a Kafka cluster deployed to Google Cloud Platform. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `create_time` | String |  | Output only. The time when the cluster was created. |
| `name` | String |  | Identifier. The name of the cluster. Structured like: projects/{project_number}/locations/{location}/clusters/{cluster_id} |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `rebalance_config` | String |  | Optional. Rebalance configuration for the Kafka cluster. |
| `state` | String |  | Output only. The current state of the cluster. |
| `tls_config` | String |  | Optional. TLS configuration for the Kafka cluster. |
| `capacity_config` | String |  | Required. Capacity configuration for the Kafka cluster. |
| `update_time` | String |  | Output only. The time when the cluster was last updated. |
| `parent` | String | ✅ | Required. The parent region in which to create the cluster. Structured like `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `update_options` | String | Optional. UpdateOptions represents options that control how updates to the cluster are applied. |
| `gcp_config` | String | Required. Configuration properties for a Kafka cluster deployed to Google Cloud Platform. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `create_time` | String | Output only. The time when the cluster was created. |
| `name` | String | Identifier. The name of the cluster. Structured like: projects/{project_number}/locations/{location}/clusters/{cluster_id} |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `rebalance_config` | String | Optional. Rebalance configuration for the Kafka cluster. |
| `state` | String | Output only. The current state of the cluster. |
| `tls_config` | String | Optional. TLS configuration for the Kafka cluster. |
| `capacity_config` | String | Required. Capacity configuration for the Kafka cluster. |
| `update_time` | String | Output only. The time when the cluster was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cluster
cluster = provider.managedkafka_api.Cluster {
    parent = "value"  # Required. The parent region in which to create the cluster. Structured like `projects/{project}/locations/{location}`.
}

# Access cluster outputs
cluster_id = cluster.id
cluster_satisfies_pzs = cluster.satisfies_pzs
cluster_update_options = cluster.update_options
cluster_gcp_config = cluster.gcp_config
cluster_satisfies_pzi = cluster.satisfies_pzi
cluster_create_time = cluster.create_time
cluster_name = cluster.name
cluster_labels = cluster.labels
cluster_rebalance_config = cluster.rebalance_config
cluster_state = cluster.state
cluster_tls_config = cluster.tls_config
cluster_capacity_config = cluster.capacity_config
cluster_update_time = cluster.update_time
```

---


### Connector

Creates a new connector in a given Connect cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The name of the connector. Structured like: projects/{project}/locations/{location}/connectClusters/{connect_cluster}/connectors/{connector} |
| `state` | String |  | Output only. The current state of the connector. |
| `configs` | HashMap<String, String> |  | Optional. Connector config as keys/values. The keys of the map are connector property names, for example: `connector.class`, `tasks.max`, `key.converter`. |
| `task_restart_policy` | String |  | Optional. Restarts the individual tasks of a Connector. |
| `parent` | String | ✅ | Required. The parent Connect cluster in which to create the connector. Structured like `projects/{project}/locations/{location}/connectClusters/{connect_cluster_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The name of the connector. Structured like: projects/{project}/locations/{location}/connectClusters/{connect_cluster}/connectors/{connector} |
| `state` | String | Output only. The current state of the connector. |
| `configs` | HashMap<String, String> | Optional. Connector config as keys/values. The keys of the map are connector property names, for example: `connector.class`, `tasks.max`, `key.converter`. |
| `task_restart_policy` | String | Optional. Restarts the individual tasks of a Connector. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connector
connector = provider.managedkafka_api.Connector {
    parent = "value"  # Required. The parent Connect cluster in which to create the connector. Structured like `projects/{project}/locations/{location}/connectClusters/{connect_cluster_id}`.
}

# Access connector outputs
connector_id = connector.id
connector_name = connector.name
connector_state = connector.state
connector_configs = connector.configs
connector_task_restart_policy = connector.task_restart_policy
```

---


### Config

Get schema config at global level or for a subject.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `compatibility` | String |  | Required. The compatibility type of the schemas. Cannot be unset for a SchemaRegistry-level SchemaConfig. If unset on a SchemaSubject-level SchemaConfig, removes the compatibility field for the SchemaConfig. |
| `normalize` | bool |  | Optional. If true, the schema will be normalized before being stored or looked up. The default is false. Cannot be unset for a SchemaRegistry-level SchemaConfig. If unset on a SchemaSubject-level SchemaConfig, removes the normalize field for the SchemaConfig. |
| `name` | String | ✅ | Required. The resource name to update the config for. It can be either of following: * projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/config: Update config at global level. * projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/config/{subject}: Update config for a specific subject. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `alias` | String | Optional. The subject to which this subject is an alias of. Only applicable for subject config. |
| `compatibility` | String | Required. The compatibility type of the schema. The default value is BACKWARD. If unset in a SchemaSubject-level SchemaConfig, defaults to the global value. If unset in a SchemaRegistry-level SchemaConfig, reverts to the default value. |
| `normalize` | bool | Optional. If true, the schema will be normalized before being stored or looked up. The default is false. If unset in a SchemaSubject-level SchemaConfig, the global value will be used. If unset in a SchemaRegistry-level SchemaConfig, reverts to the default value. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access config outputs
config_id = config.id
config_alias = config.alias
config_compatibility = config.compatibility
config_normalize = config.normalize
```

---


### Type

List the supported schema types. The response will be an array of schema types.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The HTTP request/response body as raw binary. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access type outputs
type_id = type.id
type_data = type.data
type_content_type = type.content_type
type_extensions = type.extensions
```

---


### Schema_registrie

Create a schema registry instance.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_registry` | String |  | Required. The schema registry instance to create. The name field is ignored. |
| `schema_registry_id` | String |  | Required. The schema registry instance ID to use for this schema registry. The ID must contain only letters (a-z, A-Z), numbers (0-9), and underscores (-). The maximum length is 63 characters. The ID must not start with a number. |
| `parent` | String | ✅ | Required. The parent whose schema registry instance is to be created. Structured like: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `contexts` | Vec<String> | Output only. The contexts of the schema registry instance. |
| `name` | String | Identifier. The name of the schema registry instance. Structured like: `projects/{project}/locations/{location}/schemaRegistries/{schema_registry}` The instance name {schema_registry} can contain the following: * Up to 255 characters. * Letters (uppercase or lowercase), numbers, and underscores. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create schema_registrie
schema_registrie = provider.managedkafka_api.Schema_registrie {
    parent = "value"  # Required. The parent whose schema registry instance is to be created. Structured like: `projects/{project}/locations/{location}`
}

# Access schema_registrie outputs
schema_registrie_id = schema_registrie.id
schema_registrie_contexts = schema_registrie.contexts
schema_registrie_name = schema_registrie.name
```

---


### Compatibility

Check compatibility of a schema with all versions or a specific version of a subject.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_type` | String |  | Optional. The schema type of the schema. |
| `references` | Vec<String> |  | Optional. The schema references used by the schema. |
| `schema` | String |  | Required. The schema payload |
| `verbose` | bool |  | Optional. If true, the response will contain the compatibility check result with reasons for failed checks. The default is false. |
| `name` | String | ✅ | Required. The name of the resource to check compatibility for. The format is either of following: * projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/compatibility/subjects/*/versions: Check compatibility with one or more versions of the specified subject. * projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/compatibility/subjects/{subject}/versions/{version}: Check compatibility with a specific version of the subject. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create compatibility
compatibility = provider.managedkafka_api.Compatibility {
    name = "value"  # Required. The name of the resource to check compatibility for. The format is either of following: * projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/compatibility/subjects/*/versions: Check compatibility with one or more versions of the specified subject. * projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/compatibility/subjects/{subject}/versions/{version}: Check compatibility with a specific version of the subject.
}

```

---


### Version

Register a new version under a given subject with the given schema.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_type` | String |  | Optional. The type of the schema. It is optional. If not specified, the schema type will be AVRO. |
| `id` | i64 |  | Optional. The schema ID of the schema. If not specified, the schema ID will be generated by the server. If the schema ID is specified, it must not be used by an existing schema that is different from the schema to be created. |
| `normalize` | bool |  | Optional. If true, the schema will be normalized before being stored. The default is false. |
| `version` | i64 |  | Optional. The version to create. It is optional. If not specified, the version will be created with the max version ID of the subject increased by 1. If the version ID is specified, it will be used as the new version ID and must not be used by an existing version of the subject. |
| `references` | Vec<String> |  | Optional. The schema references used by the schema. |
| `schema` | String |  | Required. The schema payload |
| `parent` | String | ✅ | Required. The subject to create the version for. Structured like: `projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/subjects/{subject}` or `projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/contexts/{context}/subjects/{subject}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `schema` | String | Required. The schema payload. |
| `version` | i64 | Required. The version ID |
| `id` | i64 | Required. The schema ID. |
| `schema_type` | String | Optional. The schema type of the schema. |
| `subject` | String | Required. The subject of the version. |
| `references` | Vec<String> | Optional. The schema references used by the schema. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.managedkafka_api.Version {
    parent = "value"  # Required. The subject to create the version for. Structured like: `projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/subjects/{subject}` or `projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/contexts/{context}/subjects/{subject}`
}

# Access version outputs
version_id = version.id
version_schema = version.schema
version_version = version.version
version_id = version.id
version_schema_type = version.schema_type
version_subject = version.subject
version_references = version.references
```

---


### Schema

Get the schema for the given schema id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `schema_type` | String | Optional. The schema type of the schema. |
| `schema` | String | The schema payload. |
| `references` | Vec<String> | Optional. The schema references used by the schema. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access schema outputs
schema_id = schema.id
schema_schema_type = schema.schema_type
schema_schema = schema.schema
schema_references = schema.references
```

---


### Topic

Creates a new topic in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `partition_count` | i64 |  | Required. The number of partitions this topic has. The partition count can only be increased, not decreased. Please note that if partitions are increased for a topic that has a key, the partitioning logic or the ordering of the messages will be affected. |
| `configs` | HashMap<String, String> |  | Optional. Configurations for the topic that are overridden from the cluster defaults. The key of the map is a Kafka topic property name, for example: `cleanup.policy`, `compression.type`. |
| `replication_factor` | i64 |  | Required. Immutable. The number of replicas of each partition. A replication factor of 3 is recommended for high availability. |
| `name` | String |  | Identifier. The name of the topic. The `topic` segment is used when connecting directly to the cluster. Structured like: projects/{project}/locations/{location}/clusters/{cluster}/topics/{topic} |
| `parent` | String | ✅ | Required. The parent cluster in which to create the topic. Structured like `projects/{project}/locations/{location}/clusters/{cluster}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `partition_count` | i64 | Required. The number of partitions this topic has. The partition count can only be increased, not decreased. Please note that if partitions are increased for a topic that has a key, the partitioning logic or the ordering of the messages will be affected. |
| `configs` | HashMap<String, String> | Optional. Configurations for the topic that are overridden from the cluster defaults. The key of the map is a Kafka topic property name, for example: `cleanup.policy`, `compression.type`. |
| `replication_factor` | i64 | Required. Immutable. The number of replicas of each partition. A replication factor of 3 is recommended for high availability. |
| `name` | String | Identifier. The name of the topic. The `topic` segment is used when connecting directly to the cluster. Structured like: projects/{project}/locations/{location}/clusters/{cluster}/topics/{topic} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create topic
topic = provider.managedkafka_api.Topic {
    parent = "value"  # Required. The parent cluster in which to create the topic. Structured like `projects/{project}/locations/{location}/clusters/{cluster}`.
}

# Access topic outputs
topic_id = topic.id
topic_partition_count = topic.partition_count
topic_configs = topic.configs
topic_replication_factor = topic.replication_factor
topic_name = topic.name
```

---


### Context

Get the context.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subjects` | Vec<String> | Optional. The subjects of the context. |
| `name` | String | Identifier. The name of the context. Structured like: `projects/{project}/locations/{location}/schemaRegistries/{schema_registry}/contexts/{context}` The context name {context} can contain the following: * Up to 255 characters. * Allowed characters: letters (uppercase or lowercase), numbers, and the following special characters: `.`, `-`, `_`, `+`, `%`, and `~`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access context outputs
context_id = context.id
context_subjects = context.subjects
context_name = context.name
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation = provider.managedkafka_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_done = operation.done
operation_metadata = operation.metadata
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
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
location_display_name = location.display_name
location_location_id = location.location_id
location_labels = location.labels
location_name = location.name
location_metadata = location.metadata
```

---


### Connect_cluster

Creates a new Kafka Connect cluster in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `capacity_config` | String |  | Required. Capacity configuration for the Kafka Connect cluster. |
| `state` | String |  | Output only. The current state of the Kafka Connect cluster. |
| `update_time` | String |  | Output only. The time when the cluster was last updated. |
| `kafka_cluster` | String |  | Required. Immutable. The name of the Kafka cluster this Kafka Connect cluster is attached to. Structured like: projects/{project}/locations/{location}/clusters/{cluster} |
| `name` | String |  | Identifier. The name of the Kafka Connect cluster. Structured like: projects/{project_number}/locations/{location}/connectClusters/{connect_cluster_id} |
| `create_time` | String |  | Output only. The time when the cluster was created. |
| `config` | HashMap<String, String> |  | Optional. Configurations for the worker that are overridden from the defaults. The key of the map is a Kafka Connect worker property name, for example: `exactly.once.source.support`. |
| `gcp_config` | String |  | Required. Configuration properties for a Kafka Connect cluster deployed to Google Cloud Platform. |
| `parent` | String | ✅ | Required. The parent project/location in which to create the Kafka Connect cluster. Structured like `projects/{project}/locations/{location}/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `capacity_config` | String | Required. Capacity configuration for the Kafka Connect cluster. |
| `state` | String | Output only. The current state of the Kafka Connect cluster. |
| `update_time` | String | Output only. The time when the cluster was last updated. |
| `kafka_cluster` | String | Required. Immutable. The name of the Kafka cluster this Kafka Connect cluster is attached to. Structured like: projects/{project}/locations/{location}/clusters/{cluster} |
| `name` | String | Identifier. The name of the Kafka Connect cluster. Structured like: projects/{project_number}/locations/{location}/connectClusters/{connect_cluster_id} |
| `create_time` | String | Output only. The time when the cluster was created. |
| `config` | HashMap<String, String> | Optional. Configurations for the worker that are overridden from the defaults. The key of the map is a Kafka Connect worker property name, for example: `exactly.once.source.support`. |
| `gcp_config` | String | Required. Configuration properties for a Kafka Connect cluster deployed to Google Cloud Platform. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connect_cluster
connect_cluster = provider.managedkafka_api.Connect_cluster {
    parent = "value"  # Required. The parent project/location in which to create the Kafka Connect cluster. Structured like `projects/{project}/locations/{location}/`.
}

# Access connect_cluster outputs
connect_cluster_id = connect_cluster.id
connect_cluster_satisfies_pzi = connect_cluster.satisfies_pzi
connect_cluster_labels = connect_cluster.labels
connect_cluster_satisfies_pzs = connect_cluster.satisfies_pzs
connect_cluster_capacity_config = connect_cluster.capacity_config
connect_cluster_state = connect_cluster.state
connect_cluster_update_time = connect_cluster.update_time
connect_cluster_kafka_cluster = connect_cluster.kafka_cluster
connect_cluster_name = connect_cluster.name
connect_cluster_create_time = connect_cluster.create_time
connect_cluster_config = connect_cluster.config
connect_cluster_gcp_config = connect_cluster.gcp_config
```

---


### Acl

Creates a new acl in the given project, location, and cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_name` | String |  | Output only. The ACL resource name derived from the name. For cluster resource_type, this is always "kafka-cluster". Can be the wildcard literal "*". |
| `name` | String |  | Identifier. The name for the acl. Represents a single Resource Pattern. Structured like: projects/{project}/locations/{location}/clusters/{cluster}/acls/{acl_id} The structure of `acl_id` defines the Resource Pattern (resource_type, resource_name, pattern_type) of the acl. `acl_id` is structured like one of the following: For acls on the cluster: `cluster` For acls on a single resource within the cluster: `topic/{resource_name}` `consumerGroup/{resource_name}` `transactionalId/{resource_name}` For acls on all resources that match a prefix: `topicPrefixed/{resource_name}` `consumerGroupPrefixed/{resource_name}` `transactionalIdPrefixed/{resource_name}` For acls on all resources of a given type (i.e. the wildcard literal "*"): `allTopics` (represents `topic/*`) `allConsumerGroups` (represents `consumerGroup/*`) `allTransactionalIds` (represents `transactionalId/*`) |
| `resource_type` | String |  | Output only. The ACL resource type derived from the name. One of: CLUSTER, TOPIC, GROUP, TRANSACTIONAL_ID. |
| `pattern_type` | String |  | Output only. The ACL pattern type derived from the name. One of: LITERAL, PREFIXED. |
| `acl_entries` | Vec<String> |  | Required. The ACL entries that apply to the resource pattern. The maximum number of allowed entries 100. |
| `etag` | String |  | Optional. `etag` is used for concurrency control. An `etag` is returned in the response to `GetAcl` and `CreateAcl`. Callers are required to put that etag in the request to `UpdateAcl` to ensure that their change will be applied to the same version of the acl that exists in the Kafka Cluster. A terminal 'T' character in the etag indicates that the AclEntries were truncated; more entries for the Acl exist on the Kafka Cluster, but can't be returned in the Acl due to repeated field limits. |
| `parent` | String | ✅ | Required. The parent cluster in which to create the acl. Structured like `projects/{project}/locations/{location}/clusters/{cluster}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_name` | String | Output only. The ACL resource name derived from the name. For cluster resource_type, this is always "kafka-cluster". Can be the wildcard literal "*". |
| `name` | String | Identifier. The name for the acl. Represents a single Resource Pattern. Structured like: projects/{project}/locations/{location}/clusters/{cluster}/acls/{acl_id} The structure of `acl_id` defines the Resource Pattern (resource_type, resource_name, pattern_type) of the acl. `acl_id` is structured like one of the following: For acls on the cluster: `cluster` For acls on a single resource within the cluster: `topic/{resource_name}` `consumerGroup/{resource_name}` `transactionalId/{resource_name}` For acls on all resources that match a prefix: `topicPrefixed/{resource_name}` `consumerGroupPrefixed/{resource_name}` `transactionalIdPrefixed/{resource_name}` For acls on all resources of a given type (i.e. the wildcard literal "*"): `allTopics` (represents `topic/*`) `allConsumerGroups` (represents `consumerGroup/*`) `allTransactionalIds` (represents `transactionalId/*`) |
| `resource_type` | String | Output only. The ACL resource type derived from the name. One of: CLUSTER, TOPIC, GROUP, TRANSACTIONAL_ID. |
| `pattern_type` | String | Output only. The ACL pattern type derived from the name. One of: LITERAL, PREFIXED. |
| `acl_entries` | Vec<String> | Required. The ACL entries that apply to the resource pattern. The maximum number of allowed entries 100. |
| `etag` | String | Optional. `etag` is used for concurrency control. An `etag` is returned in the response to `GetAcl` and `CreateAcl`. Callers are required to put that etag in the request to `UpdateAcl` to ensure that their change will be applied to the same version of the acl that exists in the Kafka Cluster. A terminal 'T' character in the etag indicates that the AclEntries were truncated; more entries for the Acl exist on the Kafka Cluster, but can't be returned in the Acl due to repeated field limits. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create acl
acl = provider.managedkafka_api.Acl {
    parent = "value"  # Required. The parent cluster in which to create the acl. Structured like `projects/{project}/locations/{location}/clusters/{cluster}`.
}

# Access acl outputs
acl_id = acl.id
acl_resource_name = acl.resource_name
acl_name = acl.name
acl_resource_type = acl.resource_type
acl_pattern_type = acl.pattern_type
acl_acl_entries = acl.acl_entries
acl_etag = acl.etag
```

---


### Referencedby

Get a list of IDs of schemas that reference the schema with the given subject and version.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The HTTP request/response body as raw binary. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access referencedby outputs
referencedby_id = referencedby.id
referencedby_data = referencedby.data
referencedby_content_type = referencedby.content_type
referencedby_extensions = referencedby.extensions
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple consumer_group resources
consumer_group_0 = provider.managedkafka_api.Consumer_group {
    name = "value-0"
}
consumer_group_1 = provider.managedkafka_api.Consumer_group {
    name = "value-1"
}
consumer_group_2 = provider.managedkafka_api.Consumer_group {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    consumer_group = provider.managedkafka_api.Consumer_group {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Managedkafka_api Documentation](https://cloud.google.com/managedkafka_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
