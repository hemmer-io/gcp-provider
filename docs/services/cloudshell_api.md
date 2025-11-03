# Cloudshell_api Service



**Resources**: 4

---

## Overview

The cloudshell_api service provides access to 4 resource types:

- [Environment](#environment) [CR]
- [Operation](#operation) [CRD]
- [Environment](#environment) [CRU]
- [Public_key](#public_key) [CD]

---

## Resources


### Environment

Adds a public SSH key to an environment, allowing clients with the corresponding private key to connect to that environment via SSH. If a key with the same content already exists, this will error with ALREADY_EXISTS.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key` | String |  | Key that should be added to the environment. Supported formats are `ssh-dss` (see RFC4253), `ssh-rsa` (see RFC4253), `ecdsa-sha2-nistp256` (see RFC5656), `ecdsa-sha2-nistp384` (see RFC5656) and `ecdsa-sha2-nistp521` (see RFC5656). It should be structured as <format> <content>, where <content> part is encoded with Base64. |
| `environment` | String | ✅ | Environment this key should be added to, e.g. `users/me/environments/default`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `docker_image` | String | Required. Immutable. Full path to the Docker image used to run this environment, e.g. "gcr.io/dev-con/cloud-devshell:latest". |
| `web_host` | String | Output only. Host to which clients can connect to initiate HTTPS or WSS connections with the environment. |
| `ssh_port` | i64 | Output only. Port to which clients can connect to initiate SSH sessions with the environment. |
| `ssh_host` | String | Output only. Host to which clients can connect to initiate SSH sessions with the environment. |
| `id` | String | Output only. The environment's identifier, unique among the user's environments. |
| `name` | String | Immutable. Full name of this resource, in the format `users/{owner_email}/environments/{environment_id}`. `{owner_email}` is the email address of the user to whom this environment belongs, and `{environment_id}` is the identifier of this environment. For example, `users/someone@example.com/environments/default`. |
| `ssh_username` | String | Output only. Username that clients should use when initiating SSH sessions with the environment. |
| `public_keys` | Vec<String> | Output only. Public keys associated with the environment. Clients can connect to this environment via SSH only if they possess a private key corresponding to at least one of these public keys. Keys can be added to or removed from the environment using the AddPublicKey and RemovePublicKey methods. |
| `state` | String | Output only. Current execution state of this environment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create environment
environment = provider.cloudshell_api.Environment {
    environment = "value"  # Environment this key should be added to, e.g. `users/me/environments/default`.
}

# Access environment outputs
environment_id = environment.id
environment_docker_image = environment.docker_image
environment_web_host = environment.web_host
environment_ssh_port = environment.ssh_port
environment_ssh_host = environment.ssh_host
environment_id = environment.id
environment_name = environment.name
environment_ssh_username = environment.ssh_username
environment_public_keys = environment.public_keys
environment_state = environment.state
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.cloudshell_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
operation_metadata = operation.metadata
```

---


### Environment

Sends OAuth credentials to a running environment on behalf of a user. When this completes, the environment will be authorized to run various Google Cloud command line tools without requiring the user to manually authenticate.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expire_time` | String |  | The time when the credentials expire. If not set, defaults to one hour from when the server received the request. |
| `id_token` | String |  | The OAuth ID token that should be sent to the environment. |
| `access_token` | String |  | The OAuth access token that should be sent to the environment. |
| `name` | String | ✅ | Name of the resource that should receive the credentials, for example `users/me/environments/default` or `users/someone@example.com/environments/default`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ssh_host` | String | Output only. Host to which clients can connect to initiate SSH sessions with the environment. |
| `ssh_port` | i64 | Output only. Port to which clients can connect to initiate SSH sessions with the environment. |
| `state` | String | Output only. Current execution state of this environment. |
| `name` | String | Output only. Full name of this resource, in the format `users/{owner_email}/environments/{environment_id}`. `{owner_email}` is the email address of the user to whom this environment belongs, and `{environment_id}` is the identifier of this environment. For example, `users/someone@example.com/environments/default`. |
| `docker_image` | String | Required. Full path to the Docker image used to run this environment, e.g. "gcr.io/dev-con/cloud-devshell:latest". |
| `web_host` | String | Output only. Host to which clients can connect to initiate HTTPS or WSS connections with the environment. |
| `public_keys` | Vec<String> | Output only. Public keys associated with the environment. Clients can connect to this environment via SSH only if they possess a private key corresponding to at least one of these public keys. Keys can be added to or removed from the environment using the CreatePublicKey and DeletePublicKey methods. |
| `web_ports` | Vec<i64> | Output only. Ports to which clients can connect to initiate HTTPS or WSS connections with the environment. |
| `vm_size_expire_time` | String | Output only. The time when the Environment will expire back to the default VM size. |
| `id` | String | Output only. The environment's identifier, unique among the user's environments. |
| `ssh_username` | String | Output only. Username that clients should use when initiating SSH sessions with the environment. |
| `size` | String | Indicates the size of the backing VM running the environment. If set to something other than DEFAULT, it will be reverted to the default VM size after vm_size_expire_time. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create environment
environment = provider.cloudshell_api.Environment {
    name = "value"  # Name of the resource that should receive the credentials, for example `users/me/environments/default` or `users/someone@example.com/environments/default`.
}

# Access environment outputs
environment_id = environment.id
environment_ssh_host = environment.ssh_host
environment_ssh_port = environment.ssh_port
environment_state = environment.state
environment_name = environment.name
environment_docker_image = environment.docker_image
environment_web_host = environment.web_host
environment_public_keys = environment.public_keys
environment_web_ports = environment.web_ports
environment_vm_size_expire_time = environment.vm_size_expire_time
environment_id = environment.id
environment_ssh_username = environment.ssh_username
environment_size = environment.size
```

---


### Public_key

Adds a public SSH key to an environment, allowing clients with the corresponding private key to connect to that environment via SSH. If a key with the same format and content already exists, this will return the existing key.

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key` | String |  | Key that should be added to the environment. |
| `parent` | String | ✅ | Parent resource name, e.g. `users/me/environments/default`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create public_key
public_key = provider.cloudshell_api.Public_key {
    parent = "value"  # Parent resource name, e.g. `users/me/environments/default`.
}

```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple environment resources
environment_0 = provider.cloudshell_api.Environment {
    environment = "value-0"
}
environment_1 = provider.cloudshell_api.Environment {
    environment = "value-1"
}
environment_2 = provider.cloudshell_api.Environment {
    environment = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    environment = provider.cloudshell_api.Environment {
        environment = "production-value"
    }
```

---

## Related Documentation

- [GCP Cloudshell_api Documentation](https://cloud.google.com/cloudshell_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
