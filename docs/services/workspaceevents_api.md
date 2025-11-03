# Workspaceevents_api Service



**Resources**: 2

---

## Overview

The workspaceevents_api service provides access to 2 resource types:

- [Subscription](#subscription) [CRUD]
- [Operation](#operation) [R]

---

## Resources


### Subscription

Creates a Google Workspace subscription. To learn how to use this method, see [Create a Google Workspace subscription](https://developers.google.com/workspace/events/guides/create-subscription). For a subscription on a [Chat target resource](https://developers.google.com/workspace/events/guides/events-chat), you can create a subscription as: - A Chat app by specifying an authorization scope that begins with `chat.app` and getting one-time administrator approval ([Developer Preview](https://developers.google.com/workspace/preview)). To learn more, see [Authorize as a Chat app with administrator approval](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app). - A user by specifying an authorization scope that doesn't include `app` in its name. To learn more, see [Authorize as a Chat user](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The last time that the subscription is updated. |
| `event_types` | Vec<String> |  | Required. Unordered list. Input for creating a subscription. Otherwise, output only. One or more types of events to receive about the target resource. Formatted according to the CloudEvents specification. The supported event types depend on the target resource of your subscription. For details, see [Supported Google Workspace events](https://developers.google.com/workspace/events/guides#supported-events). By default, you also receive events about the [lifecycle of your subscription](https://developers.google.com/workspace/events/guides/events-lifecycle). You don't need to specify lifecycle events for this field. If you specify an event type that doesn't exist for the target resource, the request returns an HTTP `400 Bad Request` status code. |
| `target_resource` | String |  | Required. Immutable. The Google Workspace resource that's monitored for events, formatted as the [full resource name](https://google.aip.dev/122#full-resource-names). To learn about target resources and the events that they support, see [Supported Google Workspace events](https://developers.google.com/workspace/events#supported-events). A user can only authorize your app to create one subscription for a given target resource. If your app tries to create another subscription with the same user credentials, the request returns an `ALREADY_EXISTS` error. |
| `etag` | String |  | Optional. This checksum is computed by the server based on the value of other fields, and might be sent on update requests to ensure the client has an up-to-date value before proceeding. |
| `expire_time` | String |  | Non-empty default. The timestamp in UTC when the subscription expires. Always displayed on output, regardless of what was used on input. |
| `ttl` | String |  | Input only. The time-to-live (TTL) or duration for the subscription. If unspecified or set to `0`, uses the maximum possible duration. |
| `authority` | String |  | Output only. The user who authorized the creation of the subscription. When a user authorizes the subscription, this field and the `user_authority` field have the same value and the format is: Format: `users/{user}` For Google Workspace users, the `{user}` value is the [`user.id`](https://developers.google.com/admin-sdk/directory/reference/rest/v1/users#User.FIELDS.ids) field from the Directory API. When a Chat app authorizes the subscription, only `service_account_authority` field populates and this field is empty. |
| `notification_endpoint` | String |  | Required. Immutable. The endpoint where the subscription delivers events, such as a Pub/Sub topic. |
| `name` | String |  | Identifier. Resource name of the subscription. Format: `subscriptions/{subscription}` |
| `uid` | String |  | Output only. System-assigned unique identifier for the subscription. |
| `payload_options` | String |  | Optional. Options about what data to include in the event payload. Only supported for Google Chat and Google Drive events. |
| `create_time` | String |  | Output only. The time when the subscription is created. |
| `reconciling` | bool |  | Output only. If `true`, the subscription is in the process of being updated. |
| `state` | String |  | Output only. The state of the subscription. Determines whether the subscription can receive events and deliver them to the notification endpoint. |
| `suspension_reason` | String |  | Output only. The error that suspended the subscription. To reactivate the subscription, resolve the error and call the `ReactivateSubscription` method. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The last time that the subscription is updated. |
| `event_types` | Vec<String> | Required. Unordered list. Input for creating a subscription. Otherwise, output only. One or more types of events to receive about the target resource. Formatted according to the CloudEvents specification. The supported event types depend on the target resource of your subscription. For details, see [Supported Google Workspace events](https://developers.google.com/workspace/events/guides#supported-events). By default, you also receive events about the [lifecycle of your subscription](https://developers.google.com/workspace/events/guides/events-lifecycle). You don't need to specify lifecycle events for this field. If you specify an event type that doesn't exist for the target resource, the request returns an HTTP `400 Bad Request` status code. |
| `target_resource` | String | Required. Immutable. The Google Workspace resource that's monitored for events, formatted as the [full resource name](https://google.aip.dev/122#full-resource-names). To learn about target resources and the events that they support, see [Supported Google Workspace events](https://developers.google.com/workspace/events#supported-events). A user can only authorize your app to create one subscription for a given target resource. If your app tries to create another subscription with the same user credentials, the request returns an `ALREADY_EXISTS` error. |
| `etag` | String | Optional. This checksum is computed by the server based on the value of other fields, and might be sent on update requests to ensure the client has an up-to-date value before proceeding. |
| `expire_time` | String | Non-empty default. The timestamp in UTC when the subscription expires. Always displayed on output, regardless of what was used on input. |
| `ttl` | String | Input only. The time-to-live (TTL) or duration for the subscription. If unspecified or set to `0`, uses the maximum possible duration. |
| `authority` | String | Output only. The user who authorized the creation of the subscription. When a user authorizes the subscription, this field and the `user_authority` field have the same value and the format is: Format: `users/{user}` For Google Workspace users, the `{user}` value is the [`user.id`](https://developers.google.com/admin-sdk/directory/reference/rest/v1/users#User.FIELDS.ids) field from the Directory API. When a Chat app authorizes the subscription, only `service_account_authority` field populates and this field is empty. |
| `notification_endpoint` | String | Required. Immutable. The endpoint where the subscription delivers events, such as a Pub/Sub topic. |
| `name` | String | Identifier. Resource name of the subscription. Format: `subscriptions/{subscription}` |
| `uid` | String | Output only. System-assigned unique identifier for the subscription. |
| `payload_options` | String | Optional. Options about what data to include in the event payload. Only supported for Google Chat and Google Drive events. |
| `create_time` | String | Output only. The time when the subscription is created. |
| `reconciling` | bool | Output only. If `true`, the subscription is in the process of being updated. |
| `state` | String | Output only. The state of the subscription. Determines whether the subscription can receive events and deliver them to the notification endpoint. |
| `suspension_reason` | String | Output only. The error that suspended the subscription. To reactivate the subscription, resolve the error and call the `ReactivateSubscription` method. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create subscription
subscription = provider.workspaceevents_api.Subscription {
}

# Access subscription outputs
subscription_id = subscription.id
subscription_update_time = subscription.update_time
subscription_event_types = subscription.event_types
subscription_target_resource = subscription.target_resource
subscription_etag = subscription.etag
subscription_expire_time = subscription.expire_time
subscription_ttl = subscription.ttl
subscription_authority = subscription.authority
subscription_notification_endpoint = subscription.notification_endpoint
subscription_name = subscription.name
subscription_uid = subscription.uid
subscription_payload_options = subscription.payload_options
subscription_create_time = subscription.create_time
subscription_reconciling = subscription.reconciling
subscription_state = subscription.state
subscription_suspension_reason = subscription.suspension_reason
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
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
subscription_0 = provider.workspaceevents_api.Subscription {
}
subscription_1 = provider.workspaceevents_api.Subscription {
}
subscription_2 = provider.workspaceevents_api.Subscription {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    subscription = provider.workspaceevents_api.Subscription {
    }
```

---

## Related Documentation

- [GCP Workspaceevents_api Documentation](https://cloud.google.com/workspaceevents_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
