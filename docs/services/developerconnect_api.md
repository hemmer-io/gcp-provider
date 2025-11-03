# Developerconnect_api Service



**Resources**: 7

---

## Overview

The developerconnect_api service provides access to 7 resource types:

- [Account_connector](#account_connector) [CRUD]
- [User](#user) [CRD]
- [Connection](#connection) [CRUD]
- [Git_repository_link](#git_repository_link) [CRD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Insights_config](#insights_config) [CRUD]

---

## Resources


### Account_connector

Creates a new AccountConnector in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `annotations` | HashMap<String, String> |  | Optional. Allows users to store small amounts of arbitrary data. |
| `update_time` | String |  | Output only. The timestamp when the accountConnector was updated. |
| `oauth_start_uri` | String |  | Output only. Start OAuth flow by clicking on this URL. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `name` | String |  | Identifier. The resource name of the accountConnector, in the format `projects/{project}/locations/{location}/accountConnectors/{account_connector_id}`. |
| `etag` | String |  | Optional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `provider_oauth_config` | String |  | Provider OAuth config. |
| `create_time` | String |  | Output only. The timestamp when the accountConnector was created. |
| `parent` | String | ✅ | Required. Location resource name as the account_connector’s parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotations` | HashMap<String, String> | Optional. Allows users to store small amounts of arbitrary data. |
| `update_time` | String | Output only. The timestamp when the accountConnector was updated. |
| `oauth_start_uri` | String | Output only. Start OAuth flow by clicking on this URL. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |
| `name` | String | Identifier. The resource name of the accountConnector, in the format `projects/{project}/locations/{location}/accountConnectors/{account_connector_id}`. |
| `etag` | String | Optional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `provider_oauth_config` | String | Provider OAuth config. |
| `create_time` | String | Output only. The timestamp when the accountConnector was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create account_connector
account_connector = provider.developerconnect_api.Account_connector {
    parent = "value"  # Required. Location resource name as the account_connector’s parent.
}

# Access account_connector outputs
account_connector_id = account_connector.id
account_connector_annotations = account_connector.annotations
account_connector_update_time = account_connector.update_time
account_connector_oauth_start_uri = account_connector.oauth_start_uri
account_connector_labels = account_connector.labels
account_connector_name = account_connector.name
account_connector_etag = account_connector.etag
account_connector_provider_oauth_config = account_connector.provider_oauth_config
account_connector_create_time = account_connector.create_time
```

---


### User

Fetches OAuth access token based on end user credentials.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_connector` | String | ✅ | Required. The resource name of the AccountConnector in the format `projects/*/locations/*/accountConnectors/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Output only. Developer Connect automatically converts user identity to some human readable description, e.g., email address. |
| `last_token_request_time` | String | Output only. The timestamp when the token was last requested. |
| `name` | String | Identifier. Resource name of the user, in the format `projects/*/locations/*/accountConnectors/*/users/*`. |
| `create_time` | String | Output only. The timestamp when the user was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user
user = provider.developerconnect_api.User {
    account_connector = "value"  # Required. The resource name of the AccountConnector in the format `projects/*/locations/*/accountConnectors/*`.
}

# Access user outputs
user_id = user.id
user_display_name = user.display_name
user_last_token_request_time = user.last_token_request_time
user_name = user.name
user_create_time = user.create_time
```

---


### Connection

Creates a new Connection in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `annotations` | HashMap<String, String> |  | Optional. Allows clients to store small amounts of arbitrary data. |
| `delete_time` | String |  | Output only. [Output only] Delete timestamp |
| `uid` | String |  | Output only. A system-assigned unique identifier for the Connection. |
| `installation_state` | String |  | Output only. Installation state of the Connection. |
| `update_time` | String |  | Output only. [Output only] Update timestamp |
| `bitbucket_cloud_config` | String |  | Configuration for connections to an instance of Bitbucket Clouds. |
| `bitbucket_data_center_config` | String |  | Configuration for connections to an instance of Bitbucket Data Center. |
| `github_enterprise_config` | String |  | Configuration for connections to an instance of GitHub Enterprise. |
| `reconciling` | bool |  | Output only. Set to true when the connection is being set up or updated in the background. |
| `gitlab_enterprise_config` | String |  | Configuration for connections to an instance of GitLab Enterprise. |
| `disabled` | bool |  | Optional. If disabled is set to true, functionality is disabled for this connection. Repository based API methods and webhooks processing for repositories in this connection will be disabled. |
| `crypto_key_config` | String |  | Optional. The crypto key configuration. This field is used by the Customer-Managed Encryption Keys (CMEK) feature. |
| `git_proxy_config` | String |  | Optional. Configuration for the git proxy feature. Enabling the git proxy allows clients to perform git operations on the repositories linked in the connection. |
| `gitlab_config` | String |  | Configuration for connections to gitlab.com. |
| `create_time` | String |  | Output only. [Output only] Create timestamp |
| `etag` | String |  | Optional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `github_config` | String |  | Configuration for connections to github.com. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `name` | String |  | Identifier. The resource name of the connection, in the format `projects/{project}/locations/{location}/connections/{connection_id}`. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotations` | HashMap<String, String> | Optional. Allows clients to store small amounts of arbitrary data. |
| `delete_time` | String | Output only. [Output only] Delete timestamp |
| `uid` | String | Output only. A system-assigned unique identifier for the Connection. |
| `installation_state` | String | Output only. Installation state of the Connection. |
| `update_time` | String | Output only. [Output only] Update timestamp |
| `bitbucket_cloud_config` | String | Configuration for connections to an instance of Bitbucket Clouds. |
| `bitbucket_data_center_config` | String | Configuration for connections to an instance of Bitbucket Data Center. |
| `github_enterprise_config` | String | Configuration for connections to an instance of GitHub Enterprise. |
| `reconciling` | bool | Output only. Set to true when the connection is being set up or updated in the background. |
| `gitlab_enterprise_config` | String | Configuration for connections to an instance of GitLab Enterprise. |
| `disabled` | bool | Optional. If disabled is set to true, functionality is disabled for this connection. Repository based API methods and webhooks processing for repositories in this connection will be disabled. |
| `crypto_key_config` | String | Optional. The crypto key configuration. This field is used by the Customer-Managed Encryption Keys (CMEK) feature. |
| `git_proxy_config` | String | Optional. Configuration for the git proxy feature. Enabling the git proxy allows clients to perform git operations on the repositories linked in the connection. |
| `gitlab_config` | String | Configuration for connections to gitlab.com. |
| `create_time` | String | Output only. [Output only] Create timestamp |
| `etag` | String | Optional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `github_config` | String | Configuration for connections to github.com. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |
| `name` | String | Identifier. The resource name of the connection, in the format `projects/{project}/locations/{location}/connections/{connection_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connection
connection = provider.developerconnect_api.Connection {
    parent = "value"  # Required. Value for parent.
}

# Access connection outputs
connection_id = connection.id
connection_annotations = connection.annotations
connection_delete_time = connection.delete_time
connection_uid = connection.uid
connection_installation_state = connection.installation_state
connection_update_time = connection.update_time
connection_bitbucket_cloud_config = connection.bitbucket_cloud_config
connection_bitbucket_data_center_config = connection.bitbucket_data_center_config
connection_github_enterprise_config = connection.github_enterprise_config
connection_reconciling = connection.reconciling
connection_gitlab_enterprise_config = connection.gitlab_enterprise_config
connection_disabled = connection.disabled
connection_crypto_key_config = connection.crypto_key_config
connection_git_proxy_config = connection.git_proxy_config
connection_gitlab_config = connection.gitlab_config
connection_create_time = connection.create_time
connection_etag = connection.etag
connection_github_config = connection.github_config
connection_labels = connection.labels
connection_name = connection.name
```

---


### Git_repository_link

Creates a GitRepositoryLink. Upon linking a Git Repository, Developer Connect will configure the Git Repository to send webhook events to Developer Connect. Connections that use Firebase GitHub Application will have events forwarded to the Firebase service. Connections that use Gemini Code Assist will have events forwarded to Gemini Code Assist service. All other Connections will have events forwarded to Cloud Build.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uid` | String |  | Output only. A system-assigned unique identifier for the GitRepositoryLink. |
| `clone_uri` | String |  | Required. Git Clone URI. |
| `git_proxy_uri` | String |  | Output only. URI to access the linked repository through the Git Proxy. This field is only populated if the git proxy is enabled for the connection. |
| `annotations` | HashMap<String, String> |  | Optional. Allows clients to store small amounts of arbitrary data. |
| `name` | String |  | Identifier. Resource name of the repository, in the format `projects/*/locations/*/connections/*/gitRepositoryLinks/*`. |
| `reconciling` | bool |  | Output only. Set to true when the connection is being set up or updated in the background. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `delete_time` | String |  | Output only. [Output only] Delete timestamp |
| `etag` | String |  | Optional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `create_time` | String |  | Output only. [Output only] Create timestamp |
| `webhook_id` | String |  | Output only. External ID of the webhook created for the repository. |
| `update_time` | String |  | Output only. [Output only] Update timestamp |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. A system-assigned unique identifier for the GitRepositoryLink. |
| `clone_uri` | String | Required. Git Clone URI. |
| `git_proxy_uri` | String | Output only. URI to access the linked repository through the Git Proxy. This field is only populated if the git proxy is enabled for the connection. |
| `annotations` | HashMap<String, String> | Optional. Allows clients to store small amounts of arbitrary data. |
| `name` | String | Identifier. Resource name of the repository, in the format `projects/*/locations/*/connections/*/gitRepositoryLinks/*`. |
| `reconciling` | bool | Output only. Set to true when the connection is being set up or updated in the background. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |
| `delete_time` | String | Output only. [Output only] Delete timestamp |
| `etag` | String | Optional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `create_time` | String | Output only. [Output only] Create timestamp |
| `webhook_id` | String | Output only. External ID of the webhook created for the repository. |
| `update_time` | String | Output only. [Output only] Update timestamp |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create git_repository_link
git_repository_link = provider.developerconnect_api.Git_repository_link {
    parent = "value"  # Required. Value for parent.
}

# Access git_repository_link outputs
git_repository_link_id = git_repository_link.id
git_repository_link_uid = git_repository_link.uid
git_repository_link_clone_uri = git_repository_link.clone_uri
git_repository_link_git_proxy_uri = git_repository_link.git_proxy_uri
git_repository_link_annotations = git_repository_link.annotations
git_repository_link_name = git_repository_link.name
git_repository_link_reconciling = git_repository_link.reconciling
git_repository_link_labels = git_repository_link.labels
git_repository_link_delete_time = git_repository_link.delete_time
git_repository_link_etag = git_repository_link.etag
git_repository_link_create_time = git_repository_link.create_time
git_repository_link_webhook_id = git_repository_link.webhook_id
git_repository_link_update_time = git_repository_link.update_time
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.developerconnect_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_error = operation.error
operation_response = operation.response
operation_done = operation.done
operation_name = operation.name
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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
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
location_labels = location.labels
location_location_id = location.location_id
location_metadata = location.metadata
location_display_name = location.display_name
location_name = location.name
```

---


### Insights_config

Creates a new InsightsConfig in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_hub_application` | String |  | Optional. The name of the App Hub Application. Format: projects/{project}/locations/{location}/applications/{application} |
| `annotations` | HashMap<String, String> |  | Optional. User specified annotations. See https://google.aip.dev/148#annotations for more details such as format and size limitations. |
| `create_time` | String |  | Output only. [Output only] Create timestamp |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with an InsightsConfig. |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of InsightsConfig does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `state` | String |  | Optional. Output only. The state of the InsightsConfig. |
| `errors` | Vec<String> |  | Output only. Any errors that occurred while setting up the InsightsConfig. Each error will be in the format: `field_name: error_message`, e.g. GetAppHubApplication: Permission denied while getting App Hub application. Please grant permissions to the P4SA. |
| `artifact_configs` | Vec<String> |  | Optional. The artifact configurations of the artifacts that are deployed. |
| `name` | String |  | Identifier. The name of the InsightsConfig. Format: projects/{project}/locations/{location}/insightsConfigs/{insightsConfig} |
| `runtime_configs` | Vec<String> |  | Output only. The runtime configurations where the application is deployed. |
| `update_time` | String |  | Output only. [Output only] Update timestamp |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_hub_application` | String | Optional. The name of the App Hub Application. Format: projects/{project}/locations/{location}/applications/{application} |
| `annotations` | HashMap<String, String> | Optional. User specified annotations. See https://google.aip.dev/148#annotations for more details such as format and size limitations. |
| `create_time` | String | Output only. [Output only] Create timestamp |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with an InsightsConfig. |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of InsightsConfig does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `state` | String | Optional. Output only. The state of the InsightsConfig. |
| `errors` | Vec<String> | Output only. Any errors that occurred while setting up the InsightsConfig. Each error will be in the format: `field_name: error_message`, e.g. GetAppHubApplication: Permission denied while getting App Hub application. Please grant permissions to the P4SA. |
| `artifact_configs` | Vec<String> | Optional. The artifact configurations of the artifacts that are deployed. |
| `name` | String | Identifier. The name of the InsightsConfig. Format: projects/{project}/locations/{location}/insightsConfigs/{insightsConfig} |
| `runtime_configs` | Vec<String> | Output only. The runtime configurations where the application is deployed. |
| `update_time` | String | Output only. [Output only] Update timestamp |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create insights_config
insights_config = provider.developerconnect_api.Insights_config {
    parent = "value"  # Required. Value for parent.
}

# Access insights_config outputs
insights_config_id = insights_config.id
insights_config_app_hub_application = insights_config.app_hub_application
insights_config_annotations = insights_config.annotations
insights_config_create_time = insights_config.create_time
insights_config_labels = insights_config.labels
insights_config_reconciling = insights_config.reconciling
insights_config_state = insights_config.state
insights_config_errors = insights_config.errors
insights_config_artifact_configs = insights_config.artifact_configs
insights_config_name = insights_config.name
insights_config_runtime_configs = insights_config.runtime_configs
insights_config_update_time = insights_config.update_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple account_connector resources
account_connector_0 = provider.developerconnect_api.Account_connector {
    parent = "value-0"
}
account_connector_1 = provider.developerconnect_api.Account_connector {
    parent = "value-1"
}
account_connector_2 = provider.developerconnect_api.Account_connector {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    account_connector = provider.developerconnect_api.Account_connector {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Developerconnect_api Documentation](https://cloud.google.com/developerconnect_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
