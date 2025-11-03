# Storagetransfer_api Service



**Resources**: 4

---

## Overview

The storagetransfer_api service provides access to 4 resource types:

- [Agent_pool](#agent_pool) [CRUD]
- [Google_service_account](#google_service_account) [R]
- [Transfer_operation](#transfer_operation) [CR]
- [Transfer_job](#transfer_job) [CRUD]

---

## Resources


### Agent_pool

Creates an agent pool resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. Specifies the state of the AgentPool. |
| `display_name` | String |  | Specifies the client-specified AgentPool description. |
| `name` | String |  | Required. Specifies a unique string that identifies the agent pool. Format: `projects/{project_id}/agentPools/{agent_pool_id}` |
| `bandwidth_limit` | String |  | Specifies the bandwidth limit details. If this field is unspecified, the default value is set as 'No Limit'. |
| `project_id` | String | ✅ | Required. The ID of the Google Cloud project that owns the agent pool. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. Specifies the state of the AgentPool. |
| `display_name` | String | Specifies the client-specified AgentPool description. |
| `name` | String | Required. Specifies a unique string that identifies the agent pool. Format: `projects/{project_id}/agentPools/{agent_pool_id}` |
| `bandwidth_limit` | String | Specifies the bandwidth limit details. If this field is unspecified, the default value is set as 'No Limit'. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create agent_pool
agent_pool = provider.storagetransfer_api.Agent_pool {
    project_id = "value"  # Required. The ID of the Google Cloud project that owns the agent pool.
}

# Access agent_pool outputs
agent_pool_id = agent_pool.id
agent_pool_state = agent_pool.state
agent_pool_display_name = agent_pool.display_name
agent_pool_name = agent_pool.name
agent_pool_bandwidth_limit = agent_pool.bandwidth_limit
```

---


### Google_service_account

Returns the Google service account that is used by Storage Transfer Service to access buckets in the project where transfers run or in other projects. Each Google service account is associated with one Google Cloud project. Users should add this service account to the Google Cloud Storage bucket ACLs to grant access to Storage Transfer Service. This service account is created and owned by Storage Transfer Service and can only be used by Storage Transfer Service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subject_id` | String | Unique identifier for the service account. |
| `account_email` | String | Email address of the service account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access google_service_account outputs
google_service_account_id = google_service_account.id
google_service_account_subject_id = google_service_account.subject_id
google_service_account_account_email = google_service_account.account_email
```

---


### Transfer_operation

Pauses a transfer operation.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The name of the transfer operation. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned unique name. The format of `name` is `transferOperations/some/unique/name`. |
| `metadata` | HashMap<String, String> | Represents the transfer operation object. To request a TransferOperation object, use transferOperations.get. |
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

# Create transfer_operation
transfer_operation = provider.storagetransfer_api.Transfer_operation {
    name = "value"  # Required. The name of the transfer operation.
}

# Access transfer_operation outputs
transfer_operation_id = transfer_operation.id
transfer_operation_done = transfer_operation.done
transfer_operation_name = transfer_operation.name
transfer_operation_metadata = transfer_operation.metadata
transfer_operation_error = transfer_operation.error
transfer_operation_response = transfer_operation.response
```

---


### Transfer_job

Creates a transfer job that runs periodically.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `creation_time` | String |  | Output only. The time that the transfer job was created. |
| `name` | String |  | A unique name (within the transfer project) assigned when the job is created. If this field is empty in a CreateTransferJobRequest, Storage Transfer Service assigns a unique name. Otherwise, the specified name is used as the unique name for this job. If the specified name is in use by a job, the creation request fails with an ALREADY_EXISTS error. This name must start with `"transferJobs/"` prefix and end with a letter or a number, and should be no more than 128 characters. For transfers involving PosixFilesystem, this name must start with `transferJobs/OPI` specifically. For all other transfer types, this name must not start with `transferJobs/OPI`. Non-PosixFilesystem example: `"transferJobs/^(?!OPI)[A-Za-z0-9-._~]*[A-Za-z0-9]$"` PosixFilesystem example: `"transferJobs/OPI^[A-Za-z0-9-._~]*[A-Za-z0-9]$"` Applications must not rely on the enforcement of naming requirements involving OPI. Invalid job names fail with an INVALID_ARGUMENT error. |
| `last_modification_time` | String |  | Output only. The time that the transfer job was last modified. |
| `event_stream` | String |  | Specifies the event stream for the transfer job for event-driven transfers. When EventStream is specified, the Schedule fields are ignored. |
| `notification_config` | String |  | Notification configuration. |
| `service_account` | String |  | Optional. The user-managed service account to which to delegate service agent permissions. You can grant Cloud Storage bucket permissions to this service account instead of to the Transfer Service service agent. Format is `projects/-/serviceAccounts/ACCOUNT_EMAIL_OR_UNIQUEID` Either the service account email (`SERVICE_ACCOUNT_NAME@PROJECT_ID.iam.gserviceaccount.com`) or the unique ID (`123456789012345678901`) are accepted in the string. The `-` wildcard character is required; replacing it with a project ID is invalid. See https://cloud.google.com//storage-transfer/docs/delegate-service-agent-permissions for required permissions. |
| `schedule` | String |  | Specifies schedule for the transfer job. This is an optional field. When the field is not set, the job never executes a transfer, unless you invoke RunTransferJob or update the job to have a non-empty schedule. |
| `deletion_time` | String |  | Output only. The time that the transfer job was deleted. |
| `replication_spec` | String |  | Replication specification. |
| `transfer_spec` | String |  | Transfer specification. |
| `description` | String |  | A description provided by the user for the job. Its max length is 1024 bytes when Unicode-encoded. |
| `logging_config` | String |  | Logging configuration. |
| `project_id` | String |  | The ID of the Google Cloud project that owns the job. |
| `status` | String |  | Status of the job. This value MUST be specified for `CreateTransferJobRequests`. **Note:** The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation. |
| `latest_operation_name` | String |  | The name of the most recently started TransferOperation of this JobConfig. Present if a TransferOperation has been created for this JobConfig. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | Output only. The time that the transfer job was created. |
| `name` | String | A unique name (within the transfer project) assigned when the job is created. If this field is empty in a CreateTransferJobRequest, Storage Transfer Service assigns a unique name. Otherwise, the specified name is used as the unique name for this job. If the specified name is in use by a job, the creation request fails with an ALREADY_EXISTS error. This name must start with `"transferJobs/"` prefix and end with a letter or a number, and should be no more than 128 characters. For transfers involving PosixFilesystem, this name must start with `transferJobs/OPI` specifically. For all other transfer types, this name must not start with `transferJobs/OPI`. Non-PosixFilesystem example: `"transferJobs/^(?!OPI)[A-Za-z0-9-._~]*[A-Za-z0-9]$"` PosixFilesystem example: `"transferJobs/OPI^[A-Za-z0-9-._~]*[A-Za-z0-9]$"` Applications must not rely on the enforcement of naming requirements involving OPI. Invalid job names fail with an INVALID_ARGUMENT error. |
| `last_modification_time` | String | Output only. The time that the transfer job was last modified. |
| `event_stream` | String | Specifies the event stream for the transfer job for event-driven transfers. When EventStream is specified, the Schedule fields are ignored. |
| `notification_config` | String | Notification configuration. |
| `service_account` | String | Optional. The user-managed service account to which to delegate service agent permissions. You can grant Cloud Storage bucket permissions to this service account instead of to the Transfer Service service agent. Format is `projects/-/serviceAccounts/ACCOUNT_EMAIL_OR_UNIQUEID` Either the service account email (`SERVICE_ACCOUNT_NAME@PROJECT_ID.iam.gserviceaccount.com`) or the unique ID (`123456789012345678901`) are accepted in the string. The `-` wildcard character is required; replacing it with a project ID is invalid. See https://cloud.google.com//storage-transfer/docs/delegate-service-agent-permissions for required permissions. |
| `schedule` | String | Specifies schedule for the transfer job. This is an optional field. When the field is not set, the job never executes a transfer, unless you invoke RunTransferJob or update the job to have a non-empty schedule. |
| `deletion_time` | String | Output only. The time that the transfer job was deleted. |
| `replication_spec` | String | Replication specification. |
| `transfer_spec` | String | Transfer specification. |
| `description` | String | A description provided by the user for the job. Its max length is 1024 bytes when Unicode-encoded. |
| `logging_config` | String | Logging configuration. |
| `project_id` | String | The ID of the Google Cloud project that owns the job. |
| `status` | String | Status of the job. This value MUST be specified for `CreateTransferJobRequests`. **Note:** The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation. |
| `latest_operation_name` | String | The name of the most recently started TransferOperation of this JobConfig. Present if a TransferOperation has been created for this JobConfig. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create transfer_job
transfer_job = provider.storagetransfer_api.Transfer_job {
}

# Access transfer_job outputs
transfer_job_id = transfer_job.id
transfer_job_creation_time = transfer_job.creation_time
transfer_job_name = transfer_job.name
transfer_job_last_modification_time = transfer_job.last_modification_time
transfer_job_event_stream = transfer_job.event_stream
transfer_job_notification_config = transfer_job.notification_config
transfer_job_service_account = transfer_job.service_account
transfer_job_schedule = transfer_job.schedule
transfer_job_deletion_time = transfer_job.deletion_time
transfer_job_replication_spec = transfer_job.replication_spec
transfer_job_transfer_spec = transfer_job.transfer_spec
transfer_job_description = transfer_job.description
transfer_job_logging_config = transfer_job.logging_config
transfer_job_project_id = transfer_job.project_id
transfer_job_status = transfer_job.status
transfer_job_latest_operation_name = transfer_job.latest_operation_name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple agent_pool resources
agent_pool_0 = provider.storagetransfer_api.Agent_pool {
    project_id = "value-0"
}
agent_pool_1 = provider.storagetransfer_api.Agent_pool {
    project_id = "value-1"
}
agent_pool_2 = provider.storagetransfer_api.Agent_pool {
    project_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    agent_pool = provider.storagetransfer_api.Agent_pool {
        project_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Storagetransfer_api Documentation](https://cloud.google.com/storagetransfer_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
