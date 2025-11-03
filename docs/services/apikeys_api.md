# Apikeys_api Service



**Resources**: 2

---

## Overview

The apikeys_api service provides access to 2 resource types:

- [Operation](#operation) [R]
- [Key](#key) [CRUD]

---

## Resources


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
operation_error = operation.error
operation_response = operation.response
```

---


### Key

Creates a new API key. NOTE: Key is a global resource; hence the only supported value for location is `global`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_account_email` | String |  | Optional. The email address of [the service account](https://cloud.google.com/iam/docs/service-accounts) the key is bound to. |
| `display_name` | String |  | Human-readable display name of this key that you can modify. The maximum length is 63 characters. |
| `etag` | String |  | Output only. A checksum computed by the server based on the current value of the Key resource. This may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. See https://google.aip.dev/154. |
| `key_string` | String |  | Output only. An encrypted and signed value held by this key. This field can be accessed only through the `GetKeyString` method. |
| `delete_time` | String |  | Output only. A timestamp when this key was deleted. If the resource is not deleted, this must be empty. |
| `annotations` | HashMap<String, String> |  | Annotations is an unstructured key-value map stored with a policy that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `uid` | String |  | Output only. Unique id in UUID4 format. |
| `name` | String |  | Output only. The resource name of the key. The `name` has the form: `projects//locations/global/keys/`. For example: `projects/123456867718/locations/global/keys/b7ff1f9f-8275-410a-94dd-3855ee9b5dd2` NOTE: Key is a global resource; hence the only supported value for location is `global`. |
| `restrictions` | String |  | Key restrictions. |
| `create_time` | String |  | Output only. A timestamp identifying the time this key was originally created. |
| `update_time` | String |  | Output only. A timestamp identifying the time this key was last updated. |
| `parent` | String | ✅ | Required. The project in which the API key is created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_account_email` | String | Optional. The email address of [the service account](https://cloud.google.com/iam/docs/service-accounts) the key is bound to. |
| `display_name` | String | Human-readable display name of this key that you can modify. The maximum length is 63 characters. |
| `etag` | String | Output only. A checksum computed by the server based on the current value of the Key resource. This may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. See https://google.aip.dev/154. |
| `key_string` | String | Output only. An encrypted and signed value held by this key. This field can be accessed only through the `GetKeyString` method. |
| `delete_time` | String | Output only. A timestamp when this key was deleted. If the resource is not deleted, this must be empty. |
| `annotations` | HashMap<String, String> | Annotations is an unstructured key-value map stored with a policy that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `uid` | String | Output only. Unique id in UUID4 format. |
| `name` | String | Output only. The resource name of the key. The `name` has the form: `projects//locations/global/keys/`. For example: `projects/123456867718/locations/global/keys/b7ff1f9f-8275-410a-94dd-3855ee9b5dd2` NOTE: Key is a global resource; hence the only supported value for location is `global`. |
| `restrictions` | String | Key restrictions. |
| `create_time` | String | Output only. A timestamp identifying the time this key was originally created. |
| `update_time` | String | Output only. A timestamp identifying the time this key was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create key
key = provider.apikeys_api.Key {
    parent = "value"  # Required. The project in which the API key is created.
}

# Access key outputs
key_id = key.id
key_service_account_email = key.service_account_email
key_display_name = key.display_name
key_etag = key.etag
key_key_string = key.key_string
key_delete_time = key.delete_time
key_annotations = key.annotations
key_uid = key.uid
key_name = key.name
key_restrictions = key.restrictions
key_create_time = key.create_time
key_update_time = key.update_time
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
operation_0 = provider.apikeys_api.Operation {
}
operation_1 = provider.apikeys_api.Operation {
}
operation_2 = provider.apikeys_api.Operation {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.apikeys_api.Operation {
    }
```

---

## Related Documentation

- [GCP Apikeys_api Documentation](https://cloud.google.com/apikeys_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
