# Pubsublite_api Service



**Resources**: 5

---

## Overview

The pubsublite_api service provides access to 5 resource types:

- [Subscription](#subscription) [CRUD]
- [Reservation](#reservation) [CRUD]
- [Operation](#operation) [CRD]
- [Cursor](#cursor) [R]
- [Topic](#topic) [CRUD]

---

## Resources


### Subscription

Creates a new subscription.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delivery_config` | String |  | The settings for this subscription's message delivery. |
| `name` | String |  | The name of the subscription. Structured like: projects/{project_number}/locations/{location}/subscriptions/{subscription_id} |
| `topic` | String |  | The name of the topic this subscription is attached to. Structured like: projects/{project_number}/locations/{location}/topics/{topic_id} |
| `export_config` | String |  | If present, messages are automatically written from the Pub/Sub Lite topic associated with this subscription to a destination. |
| `parent` | String | ✅ | Required. The parent location in which to create the subscription. Structured like `projects/{project_number}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delivery_config` | String | The settings for this subscription's message delivery. |
| `name` | String | The name of the subscription. Structured like: projects/{project_number}/locations/{location}/subscriptions/{subscription_id} |
| `topic` | String | The name of the topic this subscription is attached to. Structured like: projects/{project_number}/locations/{location}/topics/{topic_id} |
| `export_config` | String | If present, messages are automatically written from the Pub/Sub Lite topic associated with this subscription to a destination. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create subscription
subscription = provider.pubsublite_api.Subscription {
    parent = "value"  # Required. The parent location in which to create the subscription. Structured like `projects/{project_number}/locations/{location}`.
}

# Access subscription outputs
subscription_id = subscription.id
subscription_delivery_config = subscription.delivery_config
subscription_name = subscription.name
subscription_topic = subscription.topic
subscription_export_config = subscription.export_config
```

---


### Reservation

Creates a new reservation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the reservation. Structured like: projects/{project_number}/locations/{location}/reservations/{reservation_id} |
| `throughput_capacity` | String |  | The reserved throughput capacity. Every unit of throughput capacity is equivalent to 1 MiB/s of published messages or 2 MiB/s of subscribed messages. Any topics which are declared as using capacity from a Reservation will consume resources from this reservation instead of being charged individually. |
| `parent` | String | ✅ | Required. The parent location in which to create the reservation. Structured like `projects/{project_number}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the reservation. Structured like: projects/{project_number}/locations/{location}/reservations/{reservation_id} |
| `throughput_capacity` | String | The reserved throughput capacity. Every unit of throughput capacity is equivalent to 1 MiB/s of published messages or 2 MiB/s of subscribed messages. Any topics which are declared as using capacity from a Reservation will consume resources from this reservation instead of being charged individually. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create reservation
reservation = provider.pubsublite_api.Reservation {
    parent = "value"  # Required. The parent location in which to create the reservation. Structured like `projects/{project_number}/locations/{location}`.
}

# Access reservation outputs
reservation_id = reservation.id
reservation_name = reservation.name
reservation_throughput_capacity = reservation.throughput_capacity
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation = provider.pubsublite_api.Operation {
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


### Cursor

Returns all committed cursor information for a subscription.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `partition_cursors` | Vec<String> | The partition cursors from this request. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access cursor outputs
cursor_id = cursor.id
cursor_partition_cursors = cursor.partition_cursors
cursor_next_page_token = cursor.next_page_token
```

---


### Topic

Creates a new topic.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `partition_config` | String |  | The settings for this topic's partitions. |
| `retention_config` | String |  | The settings for this topic's message retention. |
| `name` | String |  | The name of the topic. Structured like: projects/{project_number}/locations/{location}/topics/{topic_id} |
| `reservation_config` | String |  | The settings for this topic's Reservation usage. |
| `parent` | String | ✅ | Required. The parent location in which to create the topic. Structured like `projects/{project_number}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `partition_config` | String | The settings for this topic's partitions. |
| `retention_config` | String | The settings for this topic's message retention. |
| `name` | String | The name of the topic. Structured like: projects/{project_number}/locations/{location}/topics/{topic_id} |
| `reservation_config` | String | The settings for this topic's Reservation usage. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create topic
topic = provider.pubsublite_api.Topic {
    parent = "value"  # Required. The parent location in which to create the topic. Structured like `projects/{project_number}/locations/{location}`.
}

# Access topic outputs
topic_id = topic.id
topic_partition_config = topic.partition_config
topic_retention_config = topic.retention_config
topic_name = topic.name
topic_reservation_config = topic.reservation_config
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple subscription resources
subscription_0 = provider.pubsublite_api.Subscription {
    parent = "value-0"
}
subscription_1 = provider.pubsublite_api.Subscription {
    parent = "value-1"
}
subscription_2 = provider.pubsublite_api.Subscription {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    subscription = provider.pubsublite_api.Subscription {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Pubsublite_api Documentation](https://cloud.google.com/pubsublite_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
