# Storage_api Service



**Resources**: 23

---

## Overview

The storage_api service provides access to 23 resource types:

- [Bucket_access_control](#bucket_access_control) [CRUD]
- [Default_object_access_control](#default_object_access_control) [CRUD]
- [Operation](#operation) [CR]
- [Folder](#folder) [CRD]
- [Object_access_control](#object_access_control) [CRUD]
- [Managed_folder](#managed_folder) [CRUD]
- [Channel](#channel) [C]
- [Service_account](#service_account) [R]
- [Notification](#notification) [CRD]
- [Object](#object) [CRUD]
- [Hmac_key](#hmac_key) [CRUD]
- [Bucket](#bucket) [CRUD]
- [Anywhere_cache](#anywhere_cache) [CRU]
- [Bucket_access_control](#bucket_access_control) [CRUD]
- [Bucket](#bucket) [CRUD]
- [Default_object_access_control](#default_object_access_control) [CRUD]
- [Channel](#channel) [C]
- [Object_access_control](#object_access_control) [CRUD]
- [Object](#object) [CRUD]
- [Bucket_access_control](#bucket_access_control) [CRUD]
- [Object](#object) [CRUD]
- [Bucket](#bucket) [CRUD]
- [Object_access_control](#object_access_control) [CRUD]

---

## Resources


### Bucket_access_control

Creates a new ACL entry on the specified bucket.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entity_id` | String |  | The ID for the entity, if any. |
| `bucket` | String |  | The name of the bucket. |
| `email` | String |  | The email address associated with the entity, if any. |
| `id` | String |  | The ID of the access-control entry. |
| `self_link` | String |  | The link to this access-control entry. |
| `project_team` | String |  | The project team associated with the entity, if any. |
| `etag` | String |  | HTTP 1.1 Entity tag for the access-control entry. |
| `kind` | String |  | The kind of item this is. For bucket access control entries, this is always storage#bucketAccessControl. |
| `role` | String |  | The access permission for the entity. |
| `domain` | String |  | The domain associated with the entity, if any. |
| `entity` | String |  | The entity holding the permission, in one of the following forms: 
- user-userId 
- user-email 
- group-groupId 
- group-email 
- domain-domain 
- project-team-projectId 
- allUsers 
- allAuthenticatedUsers Examples: 
- The user liz@example.com would be user-liz@example.com. 
- The group example@googlegroups.com would be group-example@googlegroups.com. 
- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com. |
| `bucket` | String | ✅ | Name of a bucket. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entity_id` | String | The ID for the entity, if any. |
| `bucket` | String | The name of the bucket. |
| `email` | String | The email address associated with the entity, if any. |
| `id` | String | The ID of the access-control entry. |
| `self_link` | String | The link to this access-control entry. |
| `project_team` | String | The project team associated with the entity, if any. |
| `etag` | String | HTTP 1.1 Entity tag for the access-control entry. |
| `kind` | String | The kind of item this is. For bucket access control entries, this is always storage#bucketAccessControl. |
| `role` | String | The access permission for the entity. |
| `domain` | String | The domain associated with the entity, if any. |
| `entity` | String | The entity holding the permission, in one of the following forms: 
- user-userId 
- user-email 
- group-groupId 
- group-email 
- domain-domain 
- project-team-projectId 
- allUsers 
- allAuthenticatedUsers Examples: 
- The user liz@example.com would be user-liz@example.com. 
- The group example@googlegroups.com would be group-example@googlegroups.com. 
- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create bucket_access_control
bucket_access_control = provider.storage_api.Bucket_access_control {
    bucket = "value"  # Name of a bucket.
}

# Access bucket_access_control outputs
bucket_access_control_id = bucket_access_control.id
bucket_access_control_entity_id = bucket_access_control.entity_id
bucket_access_control_bucket = bucket_access_control.bucket
bucket_access_control_email = bucket_access_control.email
bucket_access_control_id = bucket_access_control.id
bucket_access_control_self_link = bucket_access_control.self_link
bucket_access_control_project_team = bucket_access_control.project_team
bucket_access_control_etag = bucket_access_control.etag
bucket_access_control_kind = bucket_access_control.kind
bucket_access_control_role = bucket_access_control.role
bucket_access_control_domain = bucket_access_control.domain
bucket_access_control_entity = bucket_access_control.entity
```

---


### Default_object_access_control

Creates a new default object ACL entry on the specified bucket.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email` | String |  | The email address associated with the entity, if any. |
| `entity` | String |  | The entity holding the permission, in one of the following forms: 
- user-userId 
- user-email 
- group-groupId 
- group-email 
- domain-domain 
- project-team-projectId 
- allUsers 
- allAuthenticatedUsers Examples: 
- The user liz@example.com would be user-liz@example.com. 
- The group example@googlegroups.com would be group-example@googlegroups.com. 
- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com. |
| `entity_id` | String |  | The ID for the entity, if any. |
| `object` | String |  | The name of the object, if applied to an object. |
| `domain` | String |  | The domain associated with the entity, if any. |
| `kind` | String |  | The kind of item this is. For object access control entries, this is always storage#objectAccessControl. |
| `generation` | String |  | The content generation of the object, if applied to an object. |
| `project_team` | String |  | The project team associated with the entity, if any. |
| `id` | String |  | The ID of the access-control entry. |
| `role` | String |  | The access permission for the entity. |
| `bucket` | String |  | The name of the bucket. |
| `self_link` | String |  | The link to this access-control entry. |
| `etag` | String |  | HTTP 1.1 Entity tag for the access-control entry. |
| `bucket` | String | ✅ | Name of a bucket. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `email` | String | The email address associated with the entity, if any. |
| `entity` | String | The entity holding the permission, in one of the following forms: 
- user-userId 
- user-email 
- group-groupId 
- group-email 
- domain-domain 
- project-team-projectId 
- allUsers 
- allAuthenticatedUsers Examples: 
- The user liz@example.com would be user-liz@example.com. 
- The group example@googlegroups.com would be group-example@googlegroups.com. 
- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com. |
| `entity_id` | String | The ID for the entity, if any. |
| `object` | String | The name of the object, if applied to an object. |
| `domain` | String | The domain associated with the entity, if any. |
| `kind` | String | The kind of item this is. For object access control entries, this is always storage#objectAccessControl. |
| `generation` | String | The content generation of the object, if applied to an object. |
| `project_team` | String | The project team associated with the entity, if any. |
| `id` | String | The ID of the access-control entry. |
| `role` | String | The access permission for the entity. |
| `bucket` | String | The name of the bucket. |
| `self_link` | String | The link to this access-control entry. |
| `etag` | String | HTTP 1.1 Entity tag for the access-control entry. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create default_object_access_control
default_object_access_control = provider.storage_api.Default_object_access_control {
    bucket = "value"  # Name of a bucket.
}

# Access default_object_access_control outputs
default_object_access_control_id = default_object_access_control.id
default_object_access_control_email = default_object_access_control.email
default_object_access_control_entity = default_object_access_control.entity
default_object_access_control_entity_id = default_object_access_control.entity_id
default_object_access_control_object = default_object_access_control.object
default_object_access_control_domain = default_object_access_control.domain
default_object_access_control_kind = default_object_access_control.kind
default_object_access_control_generation = default_object_access_control.generation
default_object_access_control_project_team = default_object_access_control.project_team
default_object_access_control_id = default_object_access_control.id
default_object_access_control_role = default_object_access_control.role
default_object_access_control_bucket = default_object_access_control.bucket
default_object_access_control_self_link = default_object_access_control.self_link
default_object_access_control_etag = default_object_access_control.etag
```

---


### Operation

Starts asynchronous advancement of the relocate bucket operation in the case of required write downtime, to allow it to lock the bucket at the source location, and proceed with the bucket location swap. The server makes a best effort to advance the relocate bucket operation, but success is not guaranteed.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ttl` | String |  | Specifies the duration after which the relocation will revert to the sync stage if the relocation hasn't succeeded. Optional, if not supplied, a default value of 12h will be used. |
| `expire_time` | String |  | Specifies the time when the relocation will revert to the sync stage if the relocation hasn't succeeded. |
| `operation_id` | String | ✅ | ID of the operation resource. |
| `bucket` | String | ✅ | Name of the bucket to advance the relocate for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | The kind of item this is. For operations, this is always storage#operation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as "Delete", the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type "XxxResponse", where "Xxx" is the original method name. For example, if the original method name is "TakeSnapshot()", the inferred response type is "TakeSnapshotResponse". |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the "name" should be a resource name ending with "operations/{operationId}". |
| `done` | bool | If the value is "false", it means the operation is still in progress. If "true", the operation is completed, and either "error" or "response" is available. |
| `self_link` | String | The link to this long running operation. |
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
operation = provider.storage_api.Operation {
    operation_id = "value"  # ID of the operation resource.
    bucket = "value"  # Name of the bucket to advance the relocate for.
}

# Access operation outputs
operation_id = operation.id
operation_kind = operation.kind
operation_metadata = operation.metadata
operation_response = operation.response
operation_name = operation.name
operation_done = operation.done
operation_self_link = operation.self_link
operation_error = operation.error
```

---


### Folder

Creates a new folder. Only applicable to buckets with hierarchical namespace enabled.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the folder. Required if not specified by URL parameter. |
| `create_time` | String |  | The creation time of the folder in RFC 3339 format. |
| `self_link` | String |  | The link to this folder. |
| `id` | String |  | The ID of the folder, including the bucket name, folder name. |
| `bucket` | String |  | The name of the bucket containing this folder. |
| `kind` | String |  | The kind of item this is. For folders, this is always storage#folder. |
| `pending_rename_info` | String |  | Only present if the folder is part of an ongoing rename folder operation. Contains information which can be used to query the operation status. |
| `metageneration` | String |  | The version of the metadata for this folder. Used for preconditions and for detecting changes in metadata. |
| `update_time` | String |  | The modification time of the folder metadata in RFC 3339 format. |
| `bucket` | String | ✅ | Name of the bucket in which the folder resides. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the folder. Required if not specified by URL parameter. |
| `create_time` | String | The creation time of the folder in RFC 3339 format. |
| `self_link` | String | The link to this folder. |
| `id` | String | The ID of the folder, including the bucket name, folder name. |
| `bucket` | String | The name of the bucket containing this folder. |
| `kind` | String | The kind of item this is. For folders, this is always storage#folder. |
| `pending_rename_info` | String | Only present if the folder is part of an ongoing rename folder operation. Contains information which can be used to query the operation status. |
| `metageneration` | String | The version of the metadata for this folder. Used for preconditions and for detecting changes in metadata. |
| `update_time` | String | The modification time of the folder metadata in RFC 3339 format. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create folder
folder = provider.storage_api.Folder {
    bucket = "value"  # Name of the bucket in which the folder resides.
}

# Access folder outputs
folder_id = folder.id
folder_name = folder.name
folder_create_time = folder.create_time
folder_self_link = folder.self_link
folder_id = folder.id
folder_bucket = folder.bucket
folder_kind = folder.kind
folder_pending_rename_info = folder.pending_rename_info
folder_metageneration = folder.metageneration
folder_update_time = folder.update_time
```

---


### Object_access_control

Creates a new ACL entry on the specified object.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email` | String |  | The email address associated with the entity, if any. |
| `entity` | String |  | The entity holding the permission, in one of the following forms: 
- user-userId 
- user-email 
- group-groupId 
- group-email 
- domain-domain 
- project-team-projectId 
- allUsers 
- allAuthenticatedUsers Examples: 
- The user liz@example.com would be user-liz@example.com. 
- The group example@googlegroups.com would be group-example@googlegroups.com. 
- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com. |
| `entity_id` | String |  | The ID for the entity, if any. |
| `object` | String |  | The name of the object, if applied to an object. |
| `domain` | String |  | The domain associated with the entity, if any. |
| `kind` | String |  | The kind of item this is. For object access control entries, this is always storage#objectAccessControl. |
| `generation` | String |  | The content generation of the object, if applied to an object. |
| `project_team` | String |  | The project team associated with the entity, if any. |
| `id` | String |  | The ID of the access-control entry. |
| `role` | String |  | The access permission for the entity. |
| `bucket` | String |  | The name of the bucket. |
| `self_link` | String |  | The link to this access-control entry. |
| `etag` | String |  | HTTP 1.1 Entity tag for the access-control entry. |
| `object` | String | ✅ | Name of the object. For information about how to URL encode object names to be path safe, see [Encoding URI Path Parts](https://cloud.google.com/storage/docs/request-endpoints#encoding). |
| `bucket` | String | ✅ | Name of a bucket. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `email` | String | The email address associated with the entity, if any. |
| `entity` | String | The entity holding the permission, in one of the following forms: 
- user-userId 
- user-email 
- group-groupId 
- group-email 
- domain-domain 
- project-team-projectId 
- allUsers 
- allAuthenticatedUsers Examples: 
- The user liz@example.com would be user-liz@example.com. 
- The group example@googlegroups.com would be group-example@googlegroups.com. 
- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com. |
| `entity_id` | String | The ID for the entity, if any. |
| `object` | String | The name of the object, if applied to an object. |
| `domain` | String | The domain associated with the entity, if any. |
| `kind` | String | The kind of item this is. For object access control entries, this is always storage#objectAccessControl. |
| `generation` | String | The content generation of the object, if applied to an object. |
| `project_team` | String | The project team associated with the entity, if any. |
| `id` | String | The ID of the access-control entry. |
| `role` | String | The access permission for the entity. |
| `bucket` | String | The name of the bucket. |
| `self_link` | String | The link to this access-control entry. |
| `etag` | String | HTTP 1.1 Entity tag for the access-control entry. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create object_access_control
object_access_control = provider.storage_api.Object_access_control {
    object = "value"  # Name of the object. For information about how to URL encode object names to be path safe, see [Encoding URI Path Parts](https://cloud.google.com/storage/docs/request-endpoints#encoding).
    bucket = "value"  # Name of a bucket.
}

# Access object_access_control outputs
object_access_control_id = object_access_control.id
object_access_control_email = object_access_control.email
object_access_control_entity = object_access_control.entity
object_access_control_entity_id = object_access_control.entity_id
object_access_control_object = object_access_control.object
object_access_control_domain = object_access_control.domain
object_access_control_kind = object_access_control.kind
object_access_control_generation = object_access_control.generation
object_access_control_project_team = object_access_control.project_team
object_access_control_id = object_access_control.id
object_access_control_role = object_access_control.role
object_access_control_bucket = object_access_control.bucket
object_access_control_self_link = object_access_control.self_link
object_access_control_etag = object_access_control.etag
```

---


### Managed_folder

Creates a new managed folder.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `self_link` | String |  | The link to this managed folder. |
| `id` | String |  | The ID of the managed folder, including the bucket name and managed folder name. |
| `create_time` | String |  | The creation time of the managed folder in RFC 3339 format. |
| `kind` | String |  | The kind of item this is. For managed folders, this is always storage#managedFolder. |
| `bucket` | String |  | The name of the bucket containing this managed folder. |
| `metageneration` | String |  | The version of the metadata for this managed folder. Used for preconditions and for detecting changes in metadata. |
| `update_time` | String |  | The last update time of the managed folder metadata in RFC 3339 format. |
| `name` | String |  | The name of the managed folder. Required if not specified by URL parameter. |
| `bucket` | String | ✅ | Name of the bucket containing the managed folder. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `self_link` | String | The link to this managed folder. |
| `id` | String | The ID of the managed folder, including the bucket name and managed folder name. |
| `create_time` | String | The creation time of the managed folder in RFC 3339 format. |
| `kind` | String | The kind of item this is. For managed folders, this is always storage#managedFolder. |
| `bucket` | String | The name of the bucket containing this managed folder. |
| `metageneration` | String | The version of the metadata for this managed folder. Used for preconditions and for detecting changes in metadata. |
| `update_time` | String | The last update time of the managed folder metadata in RFC 3339 format. |
| `name` | String | The name of the managed folder. Required if not specified by URL parameter. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create managed_folder
managed_folder = provider.storage_api.Managed_folder {
    bucket = "value"  # Name of the bucket containing the managed folder.
}

# Access managed_folder outputs
managed_folder_id = managed_folder.id
managed_folder_self_link = managed_folder.self_link
managed_folder_id = managed_folder.id
managed_folder_create_time = managed_folder.create_time
managed_folder_kind = managed_folder.kind
managed_folder_bucket = managed_folder.bucket
managed_folder_metageneration = managed_folder.metageneration
managed_folder_update_time = managed_folder.update_time
managed_folder_name = managed_folder.name
```

---


### Channel

Stop watching resources through this channel

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `token` | String |  | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `id` | String |  | A UUID or similar unique string that identifies this channel. |
| `resource_id` | String |  | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |
| `kind` | String |  | Identifies this as a notification channel used to watch for changes to a resource, which is "api#channel". |
| `params` | HashMap<String, String> |  | Additional parameters controlling delivery channel behavior. Optional. |
| `payload` | bool |  | A Boolean value to indicate whether payload is wanted. Optional. |
| `resource_uri` | String |  | A version-specific identifier for the watched resource. |
| `type` | String |  | The type of delivery mechanism used for this channel. |
| `address` | String |  | The address where notifications are delivered for this channel. |
| `expiration` | String |  | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create channel
channel = provider.storage_api.Channel {
}

```

---


### Service_account

Get the email address of this project's Google Cloud Storage service account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `email_address` | String | The ID of the notification. |
| `kind` | String | The kind of item this is. For notifications, this is always storage#notification. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access service_account outputs
service_account_id = service_account.id
service_account_email_address = service_account.email_address
service_account_kind = service_account.kind
```

---


### Notification

Creates a notification subscription for a given bucket.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `topic` | String |  | The Cloud PubSub topic to which this subscription publishes. Formatted as: '//pubsub.googleapis.com/projects/{project-identifier}/topics/{my-topic}' |
| `etag` | String |  | HTTP 1.1 Entity tag for this subscription notification. |
| `payload_format` | String |  | The desired content of the Payload. |
| `kind` | String |  | The kind of item this is. For notifications, this is always storage#notification. |
| `self_link` | String |  | The canonical URL of this notification. |
| `event_types` | Vec<String> |  | If present, only send notifications about listed event types. If empty, sent notifications for all event types. |
| `custom_attributes` | HashMap<String, String> |  | An optional list of additional attributes to attach to each Cloud PubSub message published for this notification subscription. |
| `id` | String |  | The ID of the notification. |
| `object_name_prefix` | String |  | If present, only apply this notification configuration to object names that begin with this prefix. |
| `bucket` | String | ✅ | The parent bucket of the notification. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `topic` | String | The Cloud PubSub topic to which this subscription publishes. Formatted as: '//pubsub.googleapis.com/projects/{project-identifier}/topics/{my-topic}' |
| `etag` | String | HTTP 1.1 Entity tag for this subscription notification. |
| `payload_format` | String | The desired content of the Payload. |
| `kind` | String | The kind of item this is. For notifications, this is always storage#notification. |
| `self_link` | String | The canonical URL of this notification. |
| `event_types` | Vec<String> | If present, only send notifications about listed event types. If empty, sent notifications for all event types. |
| `custom_attributes` | HashMap<String, String> | An optional list of additional attributes to attach to each Cloud PubSub message published for this notification subscription. |
| `id` | String | The ID of the notification. |
| `object_name_prefix` | String | If present, only apply this notification configuration to object names that begin with this prefix. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create notification
notification = provider.storage_api.Notification {
    bucket = "value"  # The parent bucket of the notification.
}

# Access notification outputs
notification_id = notification.id
notification_topic = notification.topic
notification_etag = notification.etag
notification_payload_format = notification.payload_format
notification_kind = notification.kind
notification_self_link = notification.self_link
notification_event_types = notification.event_types
notification_custom_attributes = notification.custom_attributes
notification_id = notification.id
notification_object_name_prefix = notification.object_name_prefix
```

---


### Object

Stores a new object and metadata.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the object. Required if not specified by URL parameter. |
| `temporary_hold` | bool |  | Whether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites. A common use case of this flag is regulatory investigations where objects need to be retained while the investigation is ongoing. Note that unlike event-based hold, temporary hold does not impact retention expiration time of an object. |
| `md5_hash` | String |  | MD5 hash of the data; encoded using base64. For more information about using the MD5 hash, see [Data Validation and Change Detection](https://cloud.google.com/storage/docs/data-validation). |
| `content_type` | String |  | Content-Type of the object data. If an object is stored without a Content-Type, it is served as application/octet-stream. |
| `media_link` | String |  | Media download link. |
| `acl` | Vec<String> |  | Access controls on the object. |
| `kms_key_name` | String |  | Not currently supported. Specifying the parameter causes the request to fail with status code 400 - Bad Request. |
| `time_deleted` | String |  | The time at which the object became noncurrent in RFC 3339 format. Will be returned if and only if this version of the object has been deleted. |
| `storage_class` | String |  | Storage class of the object. |
| `metageneration` | String |  | The version of the metadata for this object at this generation. Used for preconditions and for detecting changes in metadata. A metageneration number is only meaningful in the context of a particular generation of a particular object. |
| `time_finalized` | String |  | The time when the object was finalized. |
| `retention_expiration_time` | String |  | A server-determined value that specifies the earliest time that the object's retention period expires. This value is in RFC 3339 format. Note 1: This field is not provided for objects with an active event-based hold, since retention expiration is unknown until the hold is removed. Note 2: This value can be provided even when temporary hold is set (so that the user can reason about policy without having to first unset the temporary hold). |
| `hard_delete_time` | String |  | This is the time (in the future) when the soft-deleted object will no longer be restorable. It is equal to the soft delete time plus the current soft delete retention duration of the bucket. |
| `retention` | String |  | A collection of object level retention parameters. |
| `custom_time` | String |  | A timestamp in RFC 3339 format specified by the user for an object. |
| `size` | String |  | Content-Length of the data in bytes. |
| `contexts` | String |  | User-defined or system-defined object contexts. Each object context is a key-payload pair, where the key provides the identification and the payload holds the associated value and additional metadata. |
| `soft_delete_time` | String |  | The time at which the object became soft-deleted in RFC 3339 format. |
| `restore_token` | String |  | Restore token used to differentiate deleted objects with the same name and generation. This field is only returned for deleted objects in hierarchical namespace buckets. |
| `event_based_hold` | bool |  | Whether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is the loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false. |
| `etag` | String |  | HTTP 1.1 Entity tag for the object. |
| `generation` | String |  | The content generation of this object. Used for object versioning. |
| `cache_control` | String |  | Cache-Control directive for the object data. If omitted, and the object is accessible to all anonymous users, the default will be public, max-age=3600. |
| `owner` | String |  | The owner of the object. This will always be the uploader of the object. |
| `content_language` | String |  | Content-Language of the object data. |
| `content_encoding` | String |  | Content-Encoding of the object data. |
| `customer_encryption` | String |  | Metadata of customer-supplied encryption key, if the object is encrypted by such a key. |
| `kind` | String |  | The kind of item this is. For objects, this is always storage#object. |
| `time_storage_class_updated` | String |  | The time at which the object's storage class was last changed. When the object is initially created, it will be set to timeCreated. |
| `updated` | String |  | The modification time of the object metadata in RFC 3339 format. Set initially to object creation time and then updated whenever any metadata of the object changes. This includes changes made by a requester, such as modifying custom metadata, as well as changes made by Cloud Storage on behalf of a requester, such as changing the storage class based on an Object Lifecycle Configuration. |
| `crc32c` | String |  | CRC32c checksum, as described in RFC 4960, Appendix B; encoded using base64 in big-endian byte order. For more information about using the CRC32c checksum, see [Data Validation and Change Detection](https://cloud.google.com/storage/docs/data-validation). |
| `id` | String |  | The ID of the object, including the bucket name, object name, and generation number. |
| `metadata` | HashMap<String, String> |  | User-provided metadata, in key/value pairs. |
| `self_link` | String |  | The link to this object. |
| `time_created` | String |  | The creation time of the object in RFC 3339 format. |
| `component_count` | i64 |  | Number of underlying components that make up this object. Components are accumulated by compose operations. |
| `content_disposition` | String |  | Content-Disposition of the object data. |
| `bucket` | String |  | The name of the bucket containing this object. |
| `bucket` | String | ✅ | Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the object. Required if not specified by URL parameter. |
| `temporary_hold` | bool | Whether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites. A common use case of this flag is regulatory investigations where objects need to be retained while the investigation is ongoing. Note that unlike event-based hold, temporary hold does not impact retention expiration time of an object. |
| `md5_hash` | String | MD5 hash of the data; encoded using base64. For more information about using the MD5 hash, see [Data Validation and Change Detection](https://cloud.google.com/storage/docs/data-validation). |
| `content_type` | String | Content-Type of the object data. If an object is stored without a Content-Type, it is served as application/octet-stream. |
| `media_link` | String | Media download link. |
| `acl` | Vec<String> | Access controls on the object. |
| `kms_key_name` | String | Not currently supported. Specifying the parameter causes the request to fail with status code 400 - Bad Request. |
| `time_deleted` | String | The time at which the object became noncurrent in RFC 3339 format. Will be returned if and only if this version of the object has been deleted. |
| `storage_class` | String | Storage class of the object. |
| `metageneration` | String | The version of the metadata for this object at this generation. Used for preconditions and for detecting changes in metadata. A metageneration number is only meaningful in the context of a particular generation of a particular object. |
| `time_finalized` | String | The time when the object was finalized. |
| `retention_expiration_time` | String | A server-determined value that specifies the earliest time that the object's retention period expires. This value is in RFC 3339 format. Note 1: This field is not provided for objects with an active event-based hold, since retention expiration is unknown until the hold is removed. Note 2: This value can be provided even when temporary hold is set (so that the user can reason about policy without having to first unset the temporary hold). |
| `hard_delete_time` | String | This is the time (in the future) when the soft-deleted object will no longer be restorable. It is equal to the soft delete time plus the current soft delete retention duration of the bucket. |
| `retention` | String | A collection of object level retention parameters. |
| `custom_time` | String | A timestamp in RFC 3339 format specified by the user for an object. |
| `size` | String | Content-Length of the data in bytes. |
| `contexts` | String | User-defined or system-defined object contexts. Each object context is a key-payload pair, where the key provides the identification and the payload holds the associated value and additional metadata. |
| `soft_delete_time` | String | The time at which the object became soft-deleted in RFC 3339 format. |
| `restore_token` | String | Restore token used to differentiate deleted objects with the same name and generation. This field is only returned for deleted objects in hierarchical namespace buckets. |
| `event_based_hold` | bool | Whether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is the loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false. |
| `etag` | String | HTTP 1.1 Entity tag for the object. |
| `generation` | String | The content generation of this object. Used for object versioning. |
| `cache_control` | String | Cache-Control directive for the object data. If omitted, and the object is accessible to all anonymous users, the default will be public, max-age=3600. |
| `owner` | String | The owner of the object. This will always be the uploader of the object. |
| `content_language` | String | Content-Language of the object data. |
| `content_encoding` | String | Content-Encoding of the object data. |
| `customer_encryption` | String | Metadata of customer-supplied encryption key, if the object is encrypted by such a key. |
| `kind` | String | The kind of item this is. For objects, this is always storage#object. |
| `time_storage_class_updated` | String | The time at which the object's storage class was last changed. When the object is initially created, it will be set to timeCreated. |
| `updated` | String | The modification time of the object metadata in RFC 3339 format. Set initially to object creation time and then updated whenever any metadata of the object changes. This includes changes made by a requester, such as modifying custom metadata, as well as changes made by Cloud Storage on behalf of a requester, such as changing the storage class based on an Object Lifecycle Configuration. |
| `crc32c` | String | CRC32c checksum, as described in RFC 4960, Appendix B; encoded using base64 in big-endian byte order. For more information about using the CRC32c checksum, see [Data Validation and Change Detection](https://cloud.google.com/storage/docs/data-validation). |
| `id` | String | The ID of the object, including the bucket name, object name, and generation number. |
| `metadata` | HashMap<String, String> | User-provided metadata, in key/value pairs. |
| `self_link` | String | The link to this object. |
| `time_created` | String | The creation time of the object in RFC 3339 format. |
| `component_count` | i64 | Number of underlying components that make up this object. Components are accumulated by compose operations. |
| `content_disposition` | String | Content-Disposition of the object data. |
| `bucket` | String | The name of the bucket containing this object. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create object
object = provider.storage_api.Object {
    bucket = "value"  # Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.
}

# Access object outputs
object_id = object.id
object_name = object.name
object_temporary_hold = object.temporary_hold
object_md5_hash = object.md5_hash
object_content_type = object.content_type
object_media_link = object.media_link
object_acl = object.acl
object_kms_key_name = object.kms_key_name
object_time_deleted = object.time_deleted
object_storage_class = object.storage_class
object_metageneration = object.metageneration
object_time_finalized = object.time_finalized
object_retention_expiration_time = object.retention_expiration_time
object_hard_delete_time = object.hard_delete_time
object_retention = object.retention
object_custom_time = object.custom_time
object_size = object.size
object_contexts = object.contexts
object_soft_delete_time = object.soft_delete_time
object_restore_token = object.restore_token
object_event_based_hold = object.event_based_hold
object_etag = object.etag
object_generation = object.generation
object_cache_control = object.cache_control
object_owner = object.owner
object_content_language = object.content_language
object_content_encoding = object.content_encoding
object_customer_encryption = object.customer_encryption
object_kind = object.kind
object_time_storage_class_updated = object.time_storage_class_updated
object_updated = object.updated
object_crc32c = object.crc32c
object_id = object.id
object_metadata = object.metadata
object_self_link = object.self_link
object_time_created = object.time_created
object_component_count = object.component_count
object_content_disposition = object.content_disposition
object_bucket = object.bucket
```

---


### Hmac_key

Creates a new HMAC key for the specified service account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project_id` | String | ✅ | Project ID owning the service account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | HTTP 1.1 Entity tag for the HMAC key. |
| `access_id` | String | The ID of the HMAC Key. |
| `updated` | String | The last modification time of the HMAC key metadata in RFC 3339 format. |
| `state` | String | The state of the key. Can be one of ACTIVE, INACTIVE, or DELETED. |
| `id` | String | The ID of the HMAC key, including the Project ID and the Access ID. |
| `kind` | String | The kind of item this is. For HMAC Key metadata, this is always storage#hmacKeyMetadata. |
| `self_link` | String | The link to this resource. |
| `project_id` | String | Project ID owning the service account to which the key authenticates. |
| `service_account_email` | String | The email address of the key's associated service account. |
| `time_created` | String | The creation time of the HMAC key in RFC 3339 format. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create hmac_key
hmac_key = provider.storage_api.Hmac_key {
    project_id = "value"  # Project ID owning the service account.
}

# Access hmac_key outputs
hmac_key_id = hmac_key.id
hmac_key_etag = hmac_key.etag
hmac_key_access_id = hmac_key.access_id
hmac_key_updated = hmac_key.updated
hmac_key_state = hmac_key.state
hmac_key_id = hmac_key.id
hmac_key_kind = hmac_key.kind
hmac_key_self_link = hmac_key.self_link
hmac_key_project_id = hmac_key.project_id
hmac_key_service_account_email = hmac_key.service_account_email
hmac_key_time_created = hmac_key.time_created
```

---


### Bucket

Creates a new bucket.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `satisfies_pzs` | bool |  | Reserved for future use. |
| `time_created` | String |  | The creation time of the bucket in RFC 3339 format. |
| `billing` | String |  | The bucket's billing configuration. |
| `default_object_acl` | Vec<String> |  | Default access controls to apply to new objects when no ACL is provided. |
| `owner` | String |  | The owner of the bucket. This is always the project team's owner group. |
| `hard_delete_time` | String |  | The hard delete time of the bucket in RFC 3339 format. |
| `object_retention` | String |  | The bucket's object retention config. |
| `versioning` | String |  | The bucket's versioning configuration. |
| `website` | String |  | The bucket's website configuration, controlling how the service behaves when accessing bucket contents as a web site. See the [Static Website Examples](https://cloud.google.com/storage/docs/static-website) for more information. |
| `rpo` | String |  | The Recovery Point Objective (RPO) of this bucket. Set to ASYNC_TURBO to turn on Turbo Replication on a bucket. |
| `metageneration` | String |  | The metadata generation of this bucket. |
| `hierarchical_namespace` | String |  | The bucket's hierarchical namespace configuration. |
| `location` | String |  | The location of the bucket. Object data for objects in the bucket resides in physical storage within this region. Defaults to US. See the [Developer's Guide](https://cloud.google.com/storage/docs/locations) for the authoritative list. |
| `default_event_based_hold` | bool |  | The default value for event-based hold on newly created objects in this bucket. Event-based hold is a way to retain objects indefinitely until an event occurs, signified by the hold's release. After being released, such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false. Objects under event-based hold cannot be deleted, overwritten or archived until the hold is removed. |
| `custom_placement_config` | String |  | The bucket's custom placement configuration for Custom Dual Regions. |
| `labels` | HashMap<String, String> |  | User-provided labels, in key/value pairs. |
| `soft_delete_policy` | String |  | The bucket's soft delete policy, which defines the period of time that soft-deleted objects will be retained, and cannot be permanently deleted. |
| `etag` | String |  | HTTP 1.1 Entity tag for the bucket. |
| `id` | String |  | The ID of the bucket. For buckets, the id and name properties are the same. |
| `ip_filter` | String |  | The bucket's IP filter configuration. Specifies the network sources that are allowed to access the operations on the bucket, as well as its underlying objects. Only enforced when the mode is set to 'Enabled'. |
| `acl` | Vec<String> |  | Access controls on the bucket. |
| `autoclass` | String |  | The bucket's Autoclass configuration. |
| `updated` | String |  | The modification time of the bucket in RFC 3339 format. |
| `kind` | String |  | The kind of item this is. For buckets, this is always storage#bucket. |
| `name` | String |  | The name of the bucket. |
| `encryption` | String |  | Encryption configuration for a bucket. |
| `project_number` | String |  | The project number of the project the bucket belongs to. |
| `self_link` | String |  | The URI of this bucket. |
| `logging` | String |  | The bucket's logging configuration, which defines the destination bucket and optional name prefix for the current bucket's logs. |
| `location_type` | String |  | The type of the bucket location. |
| `retention_policy` | String |  | The bucket's retention policy. The retention policy enforces a minimum retention time for all objects contained in the bucket, based on their creation time. Any attempt to overwrite or delete objects younger than the retention period will result in a PERMISSION_DENIED error. An unlocked retention policy can be modified or removed from the bucket via a storage.buckets.update operation. A locked retention policy cannot be removed or shortened in duration for the lifetime of the bucket. Attempting to remove or decrease period of a locked retention policy will result in a PERMISSION_DENIED error. |
| `satisfies_pzi` | bool |  | Reserved for future use. |
| `lifecycle` | String |  | The bucket's lifecycle configuration. See [Lifecycle Management](https://cloud.google.com/storage/docs/lifecycle) for more information. |
| `iam_configuration` | String |  | The bucket's IAM configuration. |
| `cors` | Vec<String> |  | The bucket's Cross-Origin Resource Sharing (CORS) configuration. |
| `generation` | String |  | The generation of this bucket. |
| `soft_delete_time` | String |  | The soft delete time of the bucket in RFC 3339 format. |
| `storage_class` | String |  | The bucket's default storage class, used whenever no storageClass is specified for a newly-created object. This defines how objects in the bucket are stored and determines the SLA and the cost of storage. Values include MULTI_REGIONAL, REGIONAL, STANDARD, NEARLINE, COLDLINE, ARCHIVE, and DURABLE_REDUCED_AVAILABILITY. If this value is not specified when the bucket is created, it will default to STANDARD. For more information, see [Storage Classes](https://cloud.google.com/storage/docs/storage-classes). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzs` | bool | Reserved for future use. |
| `time_created` | String | The creation time of the bucket in RFC 3339 format. |
| `billing` | String | The bucket's billing configuration. |
| `default_object_acl` | Vec<String> | Default access controls to apply to new objects when no ACL is provided. |
| `owner` | String | The owner of the bucket. This is always the project team's owner group. |
| `hard_delete_time` | String | The hard delete time of the bucket in RFC 3339 format. |
| `object_retention` | String | The bucket's object retention config. |
| `versioning` | String | The bucket's versioning configuration. |
| `website` | String | The bucket's website configuration, controlling how the service behaves when accessing bucket contents as a web site. See the [Static Website Examples](https://cloud.google.com/storage/docs/static-website) for more information. |
| `rpo` | String | The Recovery Point Objective (RPO) of this bucket. Set to ASYNC_TURBO to turn on Turbo Replication on a bucket. |
| `metageneration` | String | The metadata generation of this bucket. |
| `hierarchical_namespace` | String | The bucket's hierarchical namespace configuration. |
| `location` | String | The location of the bucket. Object data for objects in the bucket resides in physical storage within this region. Defaults to US. See the [Developer's Guide](https://cloud.google.com/storage/docs/locations) for the authoritative list. |
| `default_event_based_hold` | bool | The default value for event-based hold on newly created objects in this bucket. Event-based hold is a way to retain objects indefinitely until an event occurs, signified by the hold's release. After being released, such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false. Objects under event-based hold cannot be deleted, overwritten or archived until the hold is removed. |
| `custom_placement_config` | String | The bucket's custom placement configuration for Custom Dual Regions. |
| `labels` | HashMap<String, String> | User-provided labels, in key/value pairs. |
| `soft_delete_policy` | String | The bucket's soft delete policy, which defines the period of time that soft-deleted objects will be retained, and cannot be permanently deleted. |
| `etag` | String | HTTP 1.1 Entity tag for the bucket. |
| `id` | String | The ID of the bucket. For buckets, the id and name properties are the same. |
| `ip_filter` | String | The bucket's IP filter configuration. Specifies the network sources that are allowed to access the operations on the bucket, as well as its underlying objects. Only enforced when the mode is set to 'Enabled'. |
| `acl` | Vec<String> | Access controls on the bucket. |
| `autoclass` | String | The bucket's Autoclass configuration. |
| `updated` | String | The modification time of the bucket in RFC 3339 format. |
| `kind` | String | The kind of item this is. For buckets, this is always storage#bucket. |
| `name` | String | The name of the bucket. |
| `encryption` | String | Encryption configuration for a bucket. |
| `project_number` | String | The project number of the project the bucket belongs to. |
| `self_link` | String | The URI of this bucket. |
| `logging` | String | The bucket's logging configuration, which defines the destination bucket and optional name prefix for the current bucket's logs. |
| `location_type` | String | The type of the bucket location. |
| `retention_policy` | String | The bucket's retention policy. The retention policy enforces a minimum retention time for all objects contained in the bucket, based on their creation time. Any attempt to overwrite or delete objects younger than the retention period will result in a PERMISSION_DENIED error. An unlocked retention policy can be modified or removed from the bucket via a storage.buckets.update operation. A locked retention policy cannot be removed or shortened in duration for the lifetime of the bucket. Attempting to remove or decrease period of a locked retention policy will result in a PERMISSION_DENIED error. |
| `satisfies_pzi` | bool | Reserved for future use. |
| `lifecycle` | String | The bucket's lifecycle configuration. See [Lifecycle Management](https://cloud.google.com/storage/docs/lifecycle) for more information. |
| `iam_configuration` | String | The bucket's IAM configuration. |
| `cors` | Vec<String> | The bucket's Cross-Origin Resource Sharing (CORS) configuration. |
| `generation` | String | The generation of this bucket. |
| `soft_delete_time` | String | The soft delete time of the bucket in RFC 3339 format. |
| `storage_class` | String | The bucket's default storage class, used whenever no storageClass is specified for a newly-created object. This defines how objects in the bucket are stored and determines the SLA and the cost of storage. Values include MULTI_REGIONAL, REGIONAL, STANDARD, NEARLINE, COLDLINE, ARCHIVE, and DURABLE_REDUCED_AVAILABILITY. If this value is not specified when the bucket is created, it will default to STANDARD. For more information, see [Storage Classes](https://cloud.google.com/storage/docs/storage-classes). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create bucket
bucket = provider.storage_api.Bucket {
}

# Access bucket outputs
bucket_id = bucket.id
bucket_satisfies_pzs = bucket.satisfies_pzs
bucket_time_created = bucket.time_created
bucket_billing = bucket.billing
bucket_default_object_acl = bucket.default_object_acl
bucket_owner = bucket.owner
bucket_hard_delete_time = bucket.hard_delete_time
bucket_object_retention = bucket.object_retention
bucket_versioning = bucket.versioning
bucket_website = bucket.website
bucket_rpo = bucket.rpo
bucket_metageneration = bucket.metageneration
bucket_hierarchical_namespace = bucket.hierarchical_namespace
bucket_location = bucket.location
bucket_default_event_based_hold = bucket.default_event_based_hold
bucket_custom_placement_config = bucket.custom_placement_config
bucket_labels = bucket.labels
bucket_soft_delete_policy = bucket.soft_delete_policy
bucket_etag = bucket.etag
bucket_id = bucket.id
bucket_ip_filter = bucket.ip_filter
bucket_acl = bucket.acl
bucket_autoclass = bucket.autoclass
bucket_updated = bucket.updated
bucket_kind = bucket.kind
bucket_name = bucket.name
bucket_encryption = bucket.encryption
bucket_project_number = bucket.project_number
bucket_self_link = bucket.self_link
bucket_logging = bucket.logging
bucket_location_type = bucket.location_type
bucket_retention_policy = bucket.retention_policy
bucket_satisfies_pzi = bucket.satisfies_pzi
bucket_lifecycle = bucket.lifecycle
bucket_iam_configuration = bucket.iam_configuration
bucket_cors = bucket.cors
bucket_generation = bucket.generation
bucket_soft_delete_time = bucket.soft_delete_time
bucket_storage_class = bucket.storage_class
```

---


### Anywhere_cache

Creates an Anywhere Cache instance.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `anywhere_cache_id` | String |  | The ID of the Anywhere cache instance. |
| `update_time` | String |  | The modification time of the cache instance metadata in RFC 3339 format. |
| `pending_update` | bool |  | True if the cache instance has an active Update long-running operation. |
| `create_time` | String |  | The creation time of the cache instance in RFC 3339 format. |
| `zone` | String |  | The zone in which the cache instance is running. For example, us-central1-a. |
| `id` | String |  | The ID of the resource, including the project number, bucket name and anywhere cache ID. |
| `state` | String |  | The current state of the cache instance. |
| `bucket` | String |  | The name of the bucket containing this cache instance. |
| `kind` | String |  | The kind of item this is. For Anywhere Cache, this is always storage#anywhereCache. |
| `ttl` | String |  | The TTL of all cache entries in whole seconds. e.g., "7200s".  |
| `self_link` | String |  | The link to this cache instance. |
| `admission_policy` | String |  | The cache-level entry admission policy. |
| `bucket` | String | ✅ | Name of the parent bucket. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `anywhere_cache_id` | String | The ID of the Anywhere cache instance. |
| `update_time` | String | The modification time of the cache instance metadata in RFC 3339 format. |
| `pending_update` | bool | True if the cache instance has an active Update long-running operation. |
| `create_time` | String | The creation time of the cache instance in RFC 3339 format. |
| `zone` | String | The zone in which the cache instance is running. For example, us-central1-a. |
| `id` | String | The ID of the resource, including the project number, bucket name and anywhere cache ID. |
| `state` | String | The current state of the cache instance. |
| `bucket` | String | The name of the bucket containing this cache instance. |
| `kind` | String | The kind of item this is. For Anywhere Cache, this is always storage#anywhereCache. |
| `ttl` | String | The TTL of all cache entries in whole seconds. e.g., "7200s".  |
| `self_link` | String | The link to this cache instance. |
| `admission_policy` | String | The cache-level entry admission policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create anywhere_cache
anywhere_cache = provider.storage_api.Anywhere_cache {
    bucket = "value"  # Name of the parent bucket.
}

# Access anywhere_cache outputs
anywhere_cache_id = anywhere_cache.id
anywhere_cache_anywhere_cache_id = anywhere_cache.anywhere_cache_id
anywhere_cache_update_time = anywhere_cache.update_time
anywhere_cache_pending_update = anywhere_cache.pending_update
anywhere_cache_create_time = anywhere_cache.create_time
anywhere_cache_zone = anywhere_cache.zone
anywhere_cache_id = anywhere_cache.id
anywhere_cache_state = anywhere_cache.state
anywhere_cache_bucket = anywhere_cache.bucket
anywhere_cache_kind = anywhere_cache.kind
anywhere_cache_ttl = anywhere_cache.ttl
anywhere_cache_self_link = anywhere_cache.self_link
anywhere_cache_admission_policy = anywhere_cache.admission_policy
```

---


### Bucket_access_control

Creates a new ACL entry on the specified bucket.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `self_link` | String |  | The link to this access-control entry. |
| `bucket` | String |  | The name of the bucket. |
| `etag` | String |  | HTTP 1.1 Entity tag for the access-control entry. |
| `email` | String |  | The email address associated with the entity, if any. |
| `entity_id` | String |  | The ID for the entity, if any. |
| `entity` | String |  | The entity holding the permission, in one of the following forms: 
- user-userId 
- user-email 
- group-groupId 
- group-email 
- domain-domain 
- allUsers 
- allAuthenticatedUsers Examples: 
- The user liz@example.com would be user-liz@example.com. 
- The group example@googlegroups.com would be group-example@googlegroups.com. 
- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com. |
| `id` | String |  | The ID of the access-control entry. |
| `kind` | String |  | The kind of item this is. For bucket access control entries, this is always storage#bucketAccessControl. |
| `domain` | String |  | The domain associated with the entity, if any. |
| `role` | String |  | The access permission for the entity. Can be READER, WRITER, or OWNER. |
| `bucket` | String | ✅ | Name of a bucket. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `self_link` | String | The link to this access-control entry. |
| `bucket` | String | The name of the bucket. |
| `etag` | String | HTTP 1.1 Entity tag for the access-control entry. |
| `email` | String | The email address associated with the entity, if any. |
| `entity_id` | String | The ID for the entity, if any. |
| `entity` | String | The entity holding the permission, in one of the following forms: 
- user-userId 
- user-email 
- group-groupId 
- group-email 
- domain-domain 
- allUsers 
- allAuthenticatedUsers Examples: 
- The user liz@example.com would be user-liz@example.com. 
- The group example@googlegroups.com would be group-example@googlegroups.com. 
- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com. |
| `id` | String | The ID of the access-control entry. |
| `kind` | String | The kind of item this is. For bucket access control entries, this is always storage#bucketAccessControl. |
| `domain` | String | The domain associated with the entity, if any. |
| `role` | String | The access permission for the entity. Can be READER, WRITER, or OWNER. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create bucket_access_control
bucket_access_control = provider.storage_api.Bucket_access_control {
    bucket = "value"  # Name of a bucket.
}

# Access bucket_access_control outputs
bucket_access_control_id = bucket_access_control.id
bucket_access_control_self_link = bucket_access_control.self_link
bucket_access_control_bucket = bucket_access_control.bucket
bucket_access_control_etag = bucket_access_control.etag
bucket_access_control_email = bucket_access_control.email
bucket_access_control_entity_id = bucket_access_control.entity_id
bucket_access_control_entity = bucket_access_control.entity
bucket_access_control_id = bucket_access_control.id
bucket_access_control_kind = bucket_access_control.kind
bucket_access_control_domain = bucket_access_control.domain
bucket_access_control_role = bucket_access_control.role
```

---


### Bucket

Creates a new bucket.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cors` | Vec<String> |  | The bucket's Cross-Origin Resource Sharing (CORS) configuration. |
| `id` | String |  | The ID of the bucket. |
| `lifecycle` | String |  | The bucket's lifecycle configuration. See object lifecycle management for more information. |
| `location` | String |  | The location of the bucket. Object data for objects in the bucket resides in physical storage within this region. Typical values are US and EU. Defaults to US. See the developer's guide for the authoritative list. |
| `name` | String |  | The name of the bucket. |
| `acl` | Vec<String> |  | Access controls on the bucket. |
| `time_created` | String |  | Creation time of the bucket in RFC 3339 format. |
| `logging` | String |  | The bucket's logging configuration, which defines the destination bucket and optional name prefix for the current bucket's logs. |
| `etag` | String |  | HTTP 1.1 Entity tag for the bucket. |
| `kind` | String |  | The kind of item this is. For buckets, this is always storage#bucket. |
| `owner` | String |  | The owner of the bucket. This is always the project team's owner group. |
| `default_object_acl` | Vec<String> |  | Default access controls to apply to new objects when no ACL is provided. |
| `storage_class` | String |  | The bucket's storage class. This defines how objects in the bucket are stored and determines the SLA and the cost of storage. Typical values are STANDARD and DURABLE_REDUCED_AVAILABILITY. Defaults to STANDARD. See the developer's guide for the authoritative list. |
| `website` | String |  | The bucket's website configuration. |
| `self_link` | String |  | The URI of this bucket. |
| `metageneration` | String |  | The metadata generation of this bucket. |
| `versioning` | String |  | The bucket's versioning configuration. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cors` | Vec<String> | The bucket's Cross-Origin Resource Sharing (CORS) configuration. |
| `id` | String | The ID of the bucket. |
| `lifecycle` | String | The bucket's lifecycle configuration. See object lifecycle management for more information. |
| `location` | String | The location of the bucket. Object data for objects in the bucket resides in physical storage within this region. Typical values are US and EU. Defaults to US. See the developer's guide for the authoritative list. |
| `name` | String | The name of the bucket. |
| `acl` | Vec<String> | Access controls on the bucket. |
| `time_created` | String | Creation time of the bucket in RFC 3339 format. |
| `logging` | String | The bucket's logging configuration, which defines the destination bucket and optional name prefix for the current bucket's logs. |
| `etag` | String | HTTP 1.1 Entity tag for the bucket. |
| `kind` | String | The kind of item this is. For buckets, this is always storage#bucket. |
| `owner` | String | The owner of the bucket. This is always the project team's owner group. |
| `default_object_acl` | Vec<String> | Default access controls to apply to new objects when no ACL is provided. |
| `storage_class` | String | The bucket's storage class. This defines how objects in the bucket are stored and determines the SLA and the cost of storage. Typical values are STANDARD and DURABLE_REDUCED_AVAILABILITY. Defaults to STANDARD. See the developer's guide for the authoritative list. |
| `website` | String | The bucket's website configuration. |
| `self_link` | String | The URI of this bucket. |
| `metageneration` | String | The metadata generation of this bucket. |
| `versioning` | String | The bucket's versioning configuration. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create bucket
bucket = provider.storage_api.Bucket {
}

# Access bucket outputs
bucket_id = bucket.id
bucket_cors = bucket.cors
bucket_id = bucket.id
bucket_lifecycle = bucket.lifecycle
bucket_location = bucket.location
bucket_name = bucket.name
bucket_acl = bucket.acl
bucket_time_created = bucket.time_created
bucket_logging = bucket.logging
bucket_etag = bucket.etag
bucket_kind = bucket.kind
bucket_owner = bucket.owner
bucket_default_object_acl = bucket.default_object_acl
bucket_storage_class = bucket.storage_class
bucket_website = bucket.website
bucket_self_link = bucket.self_link
bucket_metageneration = bucket.metageneration
bucket_versioning = bucket.versioning
```

---


### Default_object_access_control

Creates a new default object ACL entry on the specified bucket.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | HTTP 1.1 Entity tag for the access-control entry. |
| `email` | String |  | The email address associated with the entity, if any. |
| `kind` | String |  | The kind of item this is. For object access control entries, this is always storage#objectAccessControl. |
| `object` | String |  | The name of the object. |
| `self_link` | String |  | The link to this access-control entry. |
| `entity_id` | String |  | The ID for the entity, if any. |
| `id` | String |  | The ID of the access-control entry. |
| `role` | String |  | The access permission for the entity. Can be READER or OWNER. |
| `generation` | String |  | The content generation of the object. |
| `bucket` | String |  | The name of the bucket. |
| `entity` | String |  | The entity holding the permission, in one of the following forms: 
- user-userId 
- user-email 
- group-groupId 
- group-email 
- domain-domain 
- allUsers 
- allAuthenticatedUsers Examples: 
- The user liz@example.com would be user-liz@example.com. 
- The group example@googlegroups.com would be group-example@googlegroups.com. 
- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com. |
| `domain` | String |  | The domain associated with the entity, if any. |
| `bucket` | String | ✅ | Name of a bucket. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | HTTP 1.1 Entity tag for the access-control entry. |
| `email` | String | The email address associated with the entity, if any. |
| `kind` | String | The kind of item this is. For object access control entries, this is always storage#objectAccessControl. |
| `object` | String | The name of the object. |
| `self_link` | String | The link to this access-control entry. |
| `entity_id` | String | The ID for the entity, if any. |
| `id` | String | The ID of the access-control entry. |
| `role` | String | The access permission for the entity. Can be READER or OWNER. |
| `generation` | String | The content generation of the object. |
| `bucket` | String | The name of the bucket. |
| `entity` | String | The entity holding the permission, in one of the following forms: 
- user-userId 
- user-email 
- group-groupId 
- group-email 
- domain-domain 
- allUsers 
- allAuthenticatedUsers Examples: 
- The user liz@example.com would be user-liz@example.com. 
- The group example@googlegroups.com would be group-example@googlegroups.com. 
- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com. |
| `domain` | String | The domain associated with the entity, if any. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create default_object_access_control
default_object_access_control = provider.storage_api.Default_object_access_control {
    bucket = "value"  # Name of a bucket.
}

# Access default_object_access_control outputs
default_object_access_control_id = default_object_access_control.id
default_object_access_control_etag = default_object_access_control.etag
default_object_access_control_email = default_object_access_control.email
default_object_access_control_kind = default_object_access_control.kind
default_object_access_control_object = default_object_access_control.object
default_object_access_control_self_link = default_object_access_control.self_link
default_object_access_control_entity_id = default_object_access_control.entity_id
default_object_access_control_id = default_object_access_control.id
default_object_access_control_role = default_object_access_control.role
default_object_access_control_generation = default_object_access_control.generation
default_object_access_control_bucket = default_object_access_control.bucket
default_object_access_control_entity = default_object_access_control.entity
default_object_access_control_domain = default_object_access_control.domain
```

---


### Channel

Stop watching resources through this channel

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expiration` | String |  | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `resource_uri` | String |  | A version-specific identifier for the watched resource. |
| `token` | String |  | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `id` | String |  | A UUID or similar unique string that identifies this channel. |
| `type` | String |  | The type of delivery mechanism used for this channel. |
| `address` | String |  | The address where notifications are delivered for this channel. |
| `params` | HashMap<String, String> |  | Additional parameters controlling delivery channel behavior. Optional. |
| `payload` | bool |  | A Boolean value to indicate whether payload is wanted. Optional. |
| `resource_id` | String |  | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |
| `kind` | String |  | Identifies this as a notification channel used to watch for changes to a resource, which is "api#channel". |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create channel
channel = provider.storage_api.Channel {
}

```

---


### Object_access_control

Creates a new ACL entry on the specified object.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | HTTP 1.1 Entity tag for the access-control entry. |
| `email` | String |  | The email address associated with the entity, if any. |
| `kind` | String |  | The kind of item this is. For object access control entries, this is always storage#objectAccessControl. |
| `object` | String |  | The name of the object. |
| `self_link` | String |  | The link to this access-control entry. |
| `entity_id` | String |  | The ID for the entity, if any. |
| `id` | String |  | The ID of the access-control entry. |
| `role` | String |  | The access permission for the entity. Can be READER or OWNER. |
| `generation` | String |  | The content generation of the object. |
| `bucket` | String |  | The name of the bucket. |
| `entity` | String |  | The entity holding the permission, in one of the following forms: 
- user-userId 
- user-email 
- group-groupId 
- group-email 
- domain-domain 
- allUsers 
- allAuthenticatedUsers Examples: 
- The user liz@example.com would be user-liz@example.com. 
- The group example@googlegroups.com would be group-example@googlegroups.com. 
- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com. |
| `domain` | String |  | The domain associated with the entity, if any. |
| `object` | String | ✅ | Name of the object. |
| `bucket` | String | ✅ | Name of a bucket. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | HTTP 1.1 Entity tag for the access-control entry. |
| `email` | String | The email address associated with the entity, if any. |
| `kind` | String | The kind of item this is. For object access control entries, this is always storage#objectAccessControl. |
| `object` | String | The name of the object. |
| `self_link` | String | The link to this access-control entry. |
| `entity_id` | String | The ID for the entity, if any. |
| `id` | String | The ID of the access-control entry. |
| `role` | String | The access permission for the entity. Can be READER or OWNER. |
| `generation` | String | The content generation of the object. |
| `bucket` | String | The name of the bucket. |
| `entity` | String | The entity holding the permission, in one of the following forms: 
- user-userId 
- user-email 
- group-groupId 
- group-email 
- domain-domain 
- allUsers 
- allAuthenticatedUsers Examples: 
- The user liz@example.com would be user-liz@example.com. 
- The group example@googlegroups.com would be group-example@googlegroups.com. 
- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com. |
| `domain` | String | The domain associated with the entity, if any. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create object_access_control
object_access_control = provider.storage_api.Object_access_control {
    object = "value"  # Name of the object.
    bucket = "value"  # Name of a bucket.
}

# Access object_access_control outputs
object_access_control_id = object_access_control.id
object_access_control_etag = object_access_control.etag
object_access_control_email = object_access_control.email
object_access_control_kind = object_access_control.kind
object_access_control_object = object_access_control.object
object_access_control_self_link = object_access_control.self_link
object_access_control_entity_id = object_access_control.entity_id
object_access_control_id = object_access_control.id
object_access_control_role = object_access_control.role
object_access_control_generation = object_access_control.generation
object_access_control_bucket = object_access_control.bucket
object_access_control_entity = object_access_control.entity
object_access_control_domain = object_access_control.domain
```

---


### Object

Stores new data blobs and associated metadata.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content_language` | String |  | Content-Language of the object data. |
| `generation` | String |  | The content generation of this object. Used for object versioning. |
| `self_link` | String |  | The link to this object. |
| `time_deleted` | String |  | Deletion time of the object in RFC 3339 format. Will be returned if and only if this version of the object has been deleted. |
| `content_encoding` | String |  | Content-Encoding of the object data. |
| `component_count` | i64 |  | Number of underlying components that make up this object. Components are accumulated by compose operations and are limited to a count of 32. |
| `metageneration` | String |  | The generation of the metadata for this object at this generation. Used for metadata versioning. Has no meaning outside of the context of this generation. |
| `size` | String |  | Content-Length of the data in bytes. |
| `kind` | String |  | The kind of item this is. For objects, this is always storage#object. |
| `md5_hash` | String |  | MD5 hash of the data; encoded using base64. |
| `acl` | Vec<String> |  | Access controls on the object. |
| `content_disposition` | String |  | Content-Disposition of the object data. |
| `bucket` | String |  | The bucket containing this object. |
| `etag` | String |  | HTTP 1.1 Entity tag for the object. |
| `updated` | String |  | Modification time of the object metadata in RFC 3339 format. |
| `owner` | String |  | The owner of the object. This will always be the uploader of the object. |
| `storage_class` | String |  | Storage class of the object. |
| `crc32c` | String |  | CRC32c checksum, as described in RFC 4960, Appendix B; encoded using base64. |
| `cache_control` | String |  | Cache-Control directive for the object data. |
| `content_type` | String |  | Content-Type of the object data. |
| `id` | String |  | The ID of the object. |
| `media_link` | String |  | Media download link. |
| `metadata` | HashMap<String, String> |  | User-provided metadata, in key/value pairs. |
| `name` | String |  | The name of this object. Required if not specified by URL parameter. |
| `bucket` | String | ✅ | Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_language` | String | Content-Language of the object data. |
| `generation` | String | The content generation of this object. Used for object versioning. |
| `self_link` | String | The link to this object. |
| `time_deleted` | String | Deletion time of the object in RFC 3339 format. Will be returned if and only if this version of the object has been deleted. |
| `content_encoding` | String | Content-Encoding of the object data. |
| `component_count` | i64 | Number of underlying components that make up this object. Components are accumulated by compose operations and are limited to a count of 32. |
| `metageneration` | String | The generation of the metadata for this object at this generation. Used for metadata versioning. Has no meaning outside of the context of this generation. |
| `size` | String | Content-Length of the data in bytes. |
| `kind` | String | The kind of item this is. For objects, this is always storage#object. |
| `md5_hash` | String | MD5 hash of the data; encoded using base64. |
| `acl` | Vec<String> | Access controls on the object. |
| `content_disposition` | String | Content-Disposition of the object data. |
| `bucket` | String | The bucket containing this object. |
| `etag` | String | HTTP 1.1 Entity tag for the object. |
| `updated` | String | Modification time of the object metadata in RFC 3339 format. |
| `owner` | String | The owner of the object. This will always be the uploader of the object. |
| `storage_class` | String | Storage class of the object. |
| `crc32c` | String | CRC32c checksum, as described in RFC 4960, Appendix B; encoded using base64. |
| `cache_control` | String | Cache-Control directive for the object data. |
| `content_type` | String | Content-Type of the object data. |
| `id` | String | The ID of the object. |
| `media_link` | String | Media download link. |
| `metadata` | HashMap<String, String> | User-provided metadata, in key/value pairs. |
| `name` | String | The name of this object. Required if not specified by URL parameter. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create object
object = provider.storage_api.Object {
    bucket = "value"  # Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.
}

# Access object outputs
object_id = object.id
object_content_language = object.content_language
object_generation = object.generation
object_self_link = object.self_link
object_time_deleted = object.time_deleted
object_content_encoding = object.content_encoding
object_component_count = object.component_count
object_metageneration = object.metageneration
object_size = object.size
object_kind = object.kind
object_md5_hash = object.md5_hash
object_acl = object.acl
object_content_disposition = object.content_disposition
object_bucket = object.bucket
object_etag = object.etag
object_updated = object.updated
object_owner = object.owner
object_storage_class = object.storage_class
object_crc32c = object.crc32c
object_cache_control = object.cache_control
object_content_type = object.content_type
object_id = object.id
object_media_link = object.media_link
object_metadata = object.metadata
object_name = object.name
```

---


### Bucket_access_control

Creates a new ACL entry on the specified bucket.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | The kind of item this is. For bucket access control entries, this is always storage#bucketAccessControl. |
| `id` | String |  | The ID of the access-control entry. |
| `entity` | String |  | The entity holding the permission, in one of the following forms: 
- user-userId 
- user-email 
- group-groupId 
- group-email 
- domain-domain 
- allUsers 
- allAuthenticatedUsers Examples: 
- The user liz@example.com would be user-liz@example.com. 
- The group example@googlegroups.com would be group-example@googlegroups.com. 
- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com. |
| `role` | String |  | The access permission for the entity. Can be READER, WRITER, or OWNER. |
| `self_link` | String |  | The link to this access-control entry. |
| `bucket` | String |  | The name of the bucket. |
| `domain` | String |  | The domain associated with the entity, if any. |
| `email` | String |  | The email address associated with the entity, if any. |
| `entity_id` | String |  | The ID for the entity, if any. |
| `bucket` | String | ✅ | Name of a bucket. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | The kind of item this is. For bucket access control entries, this is always storage#bucketAccessControl. |
| `id` | String | The ID of the access-control entry. |
| `entity` | String | The entity holding the permission, in one of the following forms: 
- user-userId 
- user-email 
- group-groupId 
- group-email 
- domain-domain 
- allUsers 
- allAuthenticatedUsers Examples: 
- The user liz@example.com would be user-liz@example.com. 
- The group example@googlegroups.com would be group-example@googlegroups.com. 
- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com. |
| `role` | String | The access permission for the entity. Can be READER, WRITER, or OWNER. |
| `self_link` | String | The link to this access-control entry. |
| `bucket` | String | The name of the bucket. |
| `domain` | String | The domain associated with the entity, if any. |
| `email` | String | The email address associated with the entity, if any. |
| `entity_id` | String | The ID for the entity, if any. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create bucket_access_control
bucket_access_control = provider.storage_api.Bucket_access_control {
    bucket = "value"  # Name of a bucket.
}

# Access bucket_access_control outputs
bucket_access_control_id = bucket_access_control.id
bucket_access_control_kind = bucket_access_control.kind
bucket_access_control_id = bucket_access_control.id
bucket_access_control_entity = bucket_access_control.entity
bucket_access_control_role = bucket_access_control.role
bucket_access_control_self_link = bucket_access_control.self_link
bucket_access_control_bucket = bucket_access_control.bucket
bucket_access_control_domain = bucket_access_control.domain
bucket_access_control_email = bucket_access_control.email
bucket_access_control_entity_id = bucket_access_control.entity_id
```

---


### Object

Stores new data blobs and associated metadata.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bucket` | String |  | The bucket containing this object. |
| `kind` | String |  | The kind of item this is. For objects, this is always storage#object. |
| `metadata` | HashMap<String, String> |  | User-provided metadata, in key/value pairs. |
| `name` | String |  | The name of this object. Required if not specified by URL parameter. |
| `content_language` | String |  | Content-Language of the object data. |
| `cache_control` | String |  | Cache-Control directive for the object data. |
| `content_disposition` | String |  | Content-Disposition of the object data. |
| `self_link` | String |  | The link to this object. |
| `media` | String |  | Object media data. Provided on your behalf when uploading raw media or multipart/related with an auxiliary media part. |
| `acl` | Vec<String> |  | Access controls on the object. |
| `owner` | String |  | The owner of the object. This will always be the uploader of the object. |
| `id` | String |  | The ID of the object. |
| `content_encoding` | String |  | Content-Encoding of the object data. |
| `bucket` | String | ✅ | Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bucket` | String | The bucket containing this object. |
| `kind` | String | The kind of item this is. For objects, this is always storage#object. |
| `metadata` | HashMap<String, String> | User-provided metadata, in key/value pairs. |
| `name` | String | The name of this object. Required if not specified by URL parameter. |
| `content_language` | String | Content-Language of the object data. |
| `cache_control` | String | Cache-Control directive for the object data. |
| `content_disposition` | String | Content-Disposition of the object data. |
| `self_link` | String | The link to this object. |
| `media` | String | Object media data. Provided on your behalf when uploading raw media or multipart/related with an auxiliary media part. |
| `acl` | Vec<String> | Access controls on the object. |
| `owner` | String | The owner of the object. This will always be the uploader of the object. |
| `id` | String | The ID of the object. |
| `content_encoding` | String | Content-Encoding of the object data. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create object
object = provider.storage_api.Object {
    bucket = "value"  # Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.
}

# Access object outputs
object_id = object.id
object_bucket = object.bucket
object_kind = object.kind
object_metadata = object.metadata
object_name = object.name
object_content_language = object.content_language
object_cache_control = object.cache_control
object_content_disposition = object.content_disposition
object_self_link = object.self_link
object_media = object.media
object_acl = object.acl
object_owner = object.owner
object_id = object.id
object_content_encoding = object.content_encoding
```

---


### Bucket

Creates a new bucket.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `acl` | Vec<String> |  | Access controls on the bucket. |
| `project_id` | String |  | The project the bucket belongs to. |
| `location` | String |  | The location of the bucket. Object data for objects in the bucket resides in physical storage in this location. Can be US or EU. Defaults to US. |
| `kind` | String |  | The kind of item this is. For buckets, this is always storage#bucket. |
| `self_link` | String |  | The URI of this bucket. |
| `owner` | String |  | The owner of the bucket. This will always be the project team's owner group. |
| `website` | String |  | The bucket's website configuration. |
| `id` | String |  | The name of the bucket. |
| `time_created` | String |  | Creation time of the bucket in RFC 3339 format. |
| `default_object_acl` | Vec<String> |  | Default access controls to apply to new objects when no ACL is provided. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `acl` | Vec<String> | Access controls on the bucket. |
| `project_id` | String | The project the bucket belongs to. |
| `location` | String | The location of the bucket. Object data for objects in the bucket resides in physical storage in this location. Can be US or EU. Defaults to US. |
| `kind` | String | The kind of item this is. For buckets, this is always storage#bucket. |
| `self_link` | String | The URI of this bucket. |
| `owner` | String | The owner of the bucket. This will always be the project team's owner group. |
| `website` | String | The bucket's website configuration. |
| `id` | String | The name of the bucket. |
| `time_created` | String | Creation time of the bucket in RFC 3339 format. |
| `default_object_acl` | Vec<String> | Default access controls to apply to new objects when no ACL is provided. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create bucket
bucket = provider.storage_api.Bucket {
}

# Access bucket outputs
bucket_id = bucket.id
bucket_acl = bucket.acl
bucket_project_id = bucket.project_id
bucket_location = bucket.location
bucket_kind = bucket.kind
bucket_self_link = bucket.self_link
bucket_owner = bucket.owner
bucket_website = bucket.website
bucket_id = bucket.id
bucket_time_created = bucket.time_created
bucket_default_object_acl = bucket.default_object_acl
```

---


### Object_access_control

Creates a new ACL entry on the specified object.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `object` | String |  | The name of the object. |
| `entity` | String |  | The entity holding the permission, in one of the following forms: 
- user-userId 
- user-email 
- group-groupId 
- group-email 
- domain-domain 
- allUsers 
- allAuthenticatedUsers Examples: 
- The user liz@example.com would be user-liz@example.com. 
- The group example@googlegroups.com would be group-example@googlegroups.com. 
- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com. |
| `id` | String |  | The ID of the access-control entry. |
| `email` | String |  | The email address associated with the entity, if any. |
| `kind` | String |  | The kind of item this is. For object access control entries, this is always storage#objectAccessControl. |
| `domain` | String |  | The domain associated with the entity, if any. |
| `bucket` | String |  | The name of the bucket. |
| `role` | String |  | The access permission for the entity. Can be READER or OWNER. |
| `entity_id` | String |  | The ID for the entity, if any. |
| `self_link` | String |  | The link to this access-control entry. |
| `bucket` | String | ✅ | Name of a bucket. |
| `object` | String | ✅ | Name of the object. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `object` | String | The name of the object. |
| `entity` | String | The entity holding the permission, in one of the following forms: 
- user-userId 
- user-email 
- group-groupId 
- group-email 
- domain-domain 
- allUsers 
- allAuthenticatedUsers Examples: 
- The user liz@example.com would be user-liz@example.com. 
- The group example@googlegroups.com would be group-example@googlegroups.com. 
- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com. |
| `id` | String | The ID of the access-control entry. |
| `email` | String | The email address associated with the entity, if any. |
| `kind` | String | The kind of item this is. For object access control entries, this is always storage#objectAccessControl. |
| `domain` | String | The domain associated with the entity, if any. |
| `bucket` | String | The name of the bucket. |
| `role` | String | The access permission for the entity. Can be READER or OWNER. |
| `entity_id` | String | The ID for the entity, if any. |
| `self_link` | String | The link to this access-control entry. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create object_access_control
object_access_control = provider.storage_api.Object_access_control {
    bucket = "value"  # Name of a bucket.
    object = "value"  # Name of the object.
}

# Access object_access_control outputs
object_access_control_id = object_access_control.id
object_access_control_object = object_access_control.object
object_access_control_entity = object_access_control.entity
object_access_control_id = object_access_control.id
object_access_control_email = object_access_control.email
object_access_control_kind = object_access_control.kind
object_access_control_domain = object_access_control.domain
object_access_control_bucket = object_access_control.bucket
object_access_control_role = object_access_control.role
object_access_control_entity_id = object_access_control.entity_id
object_access_control_self_link = object_access_control.self_link
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple bucket_access_control resources
bucket_access_control_0 = provider.storage_api.Bucket_access_control {
    bucket = "value-0"
}
bucket_access_control_1 = provider.storage_api.Bucket_access_control {
    bucket = "value-1"
}
bucket_access_control_2 = provider.storage_api.Bucket_access_control {
    bucket = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    bucket_access_control = provider.storage_api.Bucket_access_control {
        bucket = "production-value"
    }
```

---

## Related Documentation

- [GCP Storage_api Documentation](https://cloud.google.com/storage_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
