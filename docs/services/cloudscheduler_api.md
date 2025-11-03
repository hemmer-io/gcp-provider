# Cloudscheduler_api Service



**Resources**: 6

---

## Overview

The cloudscheduler_api service provides access to 6 resource types:

- [Operation](#operation) [CRD]
- [Location](#location) [RU]
- [Job](#job) [CRUD]
- [Job](#job) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CRD]

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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.cloudscheduler_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kms_key_name` | String |  | Optional. Resource name of the Cloud KMS key, of the form `projects/PROJECT_ID/locations/LOCATION_ID/keyRings/KEY_RING_ID/cryptoKeys/KEY_ID`, that will be used to encrypt Jobs in the region. Setting this as blank will turn off CMEK encryption. |
| `name` | String |  | Identifier. The config resource name which includes the project and location and must end in 'cmekConfig', in the format projects/PROJECT_ID/locations/LOCATION_ID/cmekConfig` |
| `name` | String | ✅ | Identifier. The config resource name which includes the project and location and must end in 'cmekConfig', in the format projects/PROJECT_ID/locations/LOCATION_ID/cmekConfig` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |


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
location_metadata = location.metadata
location_name = location.name
location_display_name = location.display_name
location_labels = location.labels
location_location_id = location.location_id
```

---


### Job

Creates a job.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attempt_deadline` | String |  | The deadline for job attempts. If the request handler does not respond by this deadline then the request is cancelled and the attempt is marked as a `DEADLINE_EXCEEDED` failure. The failed attempt can be viewed in execution logs. Cloud Scheduler will retry the job according to the RetryConfig. The default and the allowed values depend on the type of target: * For HTTP targets, the default is 3 minutes. The deadline must be in the interval [15 seconds, 30 minutes]. * For App Engine HTTP targets, 0 indicates that the request has the default deadline. The default deadline depends on the scaling type of the service: 10 minutes for standard apps with automatic scaling, 24 hours for standard apps with manual and basic scaling, and 60 minutes for flex apps. If the request deadline is set, it must be in the interval [15 seconds, 24 hours 15 seconds]. * For Pub/Sub targets, this field is ignored. |
| `state` | String |  | Output only. State of the job. |
| `retry_config` | String |  | Settings that determine the retry behavior. |
| `user_update_time` | String |  | Output only. The creation time of the job. |
| `satisfies_pzs` | bool |  | Output only. Whether or not this Job satisfies the requirements of physical zone separation |
| `pubsub_target` | String |  | Pub/Sub target. |
| `schedule` | String |  | Required, except when used with UpdateJob. Describes the schedule on which the job will be executed. The schedule can be either of the following types: * [Crontab](https://en.wikipedia.org/wiki/Cron#Overview) * English-like [schedule](/scheduler/docs/configuring/cron-job-schedules) As a general rule, execution `n + 1` of a job will not begin until execution `n` has finished. Cloud Scheduler will never allow two simultaneously outstanding executions. For example, this implies that if the `n+1`th execution is scheduled to run at 16:00 but the `n`th execution takes until 16:15, the `n+1`th execution will not start until `16:15`. A scheduled start time will be delayed if the previous execution has not ended when its scheduled time occurs. If retry_count > 0 and a job attempt fails, the job will be tried a total of retry_count times, with exponential backoff, until the next scheduled start time. If retry_count is 0, a job attempt will not be retried if it fails. Instead the Cloud Scheduler system will wait for the next scheduled execution time. Setting retry_count to 0 does not prevent failed jobs from running according to schedule after the failure. |
| `time_zone` | String |  | Specifies the time zone to be used in interpreting schedule. The value of this field must be a time zone name from the [tz database](http://en.wikipedia.org/wiki/Tz_database). Note that some time zones include a provision for daylight savings time. The rules for daylight saving time are determined by the chosen tz. For UTC use the string "utc". If a time zone is not specified, the default will be in UTC (also known as GMT). |
| `app_engine_http_target` | String |  | App Engine HTTP target. |
| `last_attempt_time` | String |  | Output only. The time the last job attempt started. |
| `http_target` | String |  | HTTP target. |
| `schedule_time` | String |  | Output only. The next time the job is scheduled. Note that this may be a retry of a previously failed attempt or the next execution time according to the schedule. |
| `status` | String |  | Output only. The response from the target for the last attempted execution. |
| `description` | String |  | Optionally caller-specified in CreateJob or UpdateJob. A human-readable description for the job. This string must not contain more than 500 characters. |
| `name` | String |  | Optionally caller-specified in CreateJob, after which it becomes output only. The job name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/jobs/JOB_ID`. * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), or periods (.). For more information, see [Identifying projects](/resource-manager/docs/creating-managing-projects#identifying_projects) * `LOCATION_ID` is the canonical ID for the job's location. The list of available locations can be obtained by calling [locations.list](/scheduler/docs/reference/rest/v1/projects.locations/list). For more information, see [Cloud Scheduler locations](/scheduler/docs/locations). * `JOB_ID` can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), or underscores (_). The maximum length is 500 characters. |
| `parent` | String | ✅ | Required. The location name. For example: `projects/PROJECT_ID/locations/LOCATION_ID`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attempt_deadline` | String | The deadline for job attempts. If the request handler does not respond by this deadline then the request is cancelled and the attempt is marked as a `DEADLINE_EXCEEDED` failure. The failed attempt can be viewed in execution logs. Cloud Scheduler will retry the job according to the RetryConfig. The default and the allowed values depend on the type of target: * For HTTP targets, the default is 3 minutes. The deadline must be in the interval [15 seconds, 30 minutes]. * For App Engine HTTP targets, 0 indicates that the request has the default deadline. The default deadline depends on the scaling type of the service: 10 minutes for standard apps with automatic scaling, 24 hours for standard apps with manual and basic scaling, and 60 minutes for flex apps. If the request deadline is set, it must be in the interval [15 seconds, 24 hours 15 seconds]. * For Pub/Sub targets, this field is ignored. |
| `state` | String | Output only. State of the job. |
| `retry_config` | String | Settings that determine the retry behavior. |
| `user_update_time` | String | Output only. The creation time of the job. |
| `satisfies_pzs` | bool | Output only. Whether or not this Job satisfies the requirements of physical zone separation |
| `pubsub_target` | String | Pub/Sub target. |
| `schedule` | String | Required, except when used with UpdateJob. Describes the schedule on which the job will be executed. The schedule can be either of the following types: * [Crontab](https://en.wikipedia.org/wiki/Cron#Overview) * English-like [schedule](/scheduler/docs/configuring/cron-job-schedules) As a general rule, execution `n + 1` of a job will not begin until execution `n` has finished. Cloud Scheduler will never allow two simultaneously outstanding executions. For example, this implies that if the `n+1`th execution is scheduled to run at 16:00 but the `n`th execution takes until 16:15, the `n+1`th execution will not start until `16:15`. A scheduled start time will be delayed if the previous execution has not ended when its scheduled time occurs. If retry_count > 0 and a job attempt fails, the job will be tried a total of retry_count times, with exponential backoff, until the next scheduled start time. If retry_count is 0, a job attempt will not be retried if it fails. Instead the Cloud Scheduler system will wait for the next scheduled execution time. Setting retry_count to 0 does not prevent failed jobs from running according to schedule after the failure. |
| `time_zone` | String | Specifies the time zone to be used in interpreting schedule. The value of this field must be a time zone name from the [tz database](http://en.wikipedia.org/wiki/Tz_database). Note that some time zones include a provision for daylight savings time. The rules for daylight saving time are determined by the chosen tz. For UTC use the string "utc". If a time zone is not specified, the default will be in UTC (also known as GMT). |
| `app_engine_http_target` | String | App Engine HTTP target. |
| `last_attempt_time` | String | Output only. The time the last job attempt started. |
| `http_target` | String | HTTP target. |
| `schedule_time` | String | Output only. The next time the job is scheduled. Note that this may be a retry of a previously failed attempt or the next execution time according to the schedule. |
| `status` | String | Output only. The response from the target for the last attempted execution. |
| `description` | String | Optionally caller-specified in CreateJob or UpdateJob. A human-readable description for the job. This string must not contain more than 500 characters. |
| `name` | String | Optionally caller-specified in CreateJob, after which it becomes output only. The job name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/jobs/JOB_ID`. * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), or periods (.). For more information, see [Identifying projects](/resource-manager/docs/creating-managing-projects#identifying_projects) * `LOCATION_ID` is the canonical ID for the job's location. The list of available locations can be obtained by calling [locations.list](/scheduler/docs/reference/rest/v1/projects.locations/list). For more information, see [Cloud Scheduler locations](/scheduler/docs/locations). * `JOB_ID` can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), or underscores (_). The maximum length is 500 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.cloudscheduler_api.Job {
    parent = "value"  # Required. The location name. For example: `projects/PROJECT_ID/locations/LOCATION_ID`.
}

# Access job outputs
job_id = job.id
job_attempt_deadline = job.attempt_deadline
job_state = job.state
job_retry_config = job.retry_config
job_user_update_time = job.user_update_time
job_satisfies_pzs = job.satisfies_pzs
job_pubsub_target = job.pubsub_target
job_schedule = job.schedule
job_time_zone = job.time_zone
job_app_engine_http_target = job.app_engine_http_target
job_last_attempt_time = job.last_attempt_time
job_http_target = job.http_target
job_schedule_time = job.schedule_time
job_status = job.status
job_description = job.description
job_name = job.name
```

---


### Job

Creates a job.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status` | String |  | Output only. The response from the target for the last attempted execution. |
| `last_attempt_time` | String |  | Output only. The time the last job attempt started. |
| `name` | String |  | Optionally caller-specified in CreateJob, after which it becomes output only. The job name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/jobs/JOB_ID`. * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), or periods (.). For more information, see [Identifying projects](/resource-manager/docs/creating-managing-projects#identifying_projects) * `LOCATION_ID` is the canonical ID for the job's location. The list of available locations can be obtained by calling [locations.list](/scheduler/docs/reference/rest/v1beta1/projects.locations/list). For more information, see [Cloud Scheduler locations](/scheduler/docs/locations). * `JOB_ID` can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), or underscores (_). The maximum length is 500 characters. |
| `legacy_app_engine_cron` | bool |  | Immutable. This field is used to manage the legacy App Engine Cron jobs using the Cloud Scheduler API. If the field is set to true, the job will be considered a legacy job. Note that App Engine Cron jobs have fewer features than Cloud Scheduler jobs, e.g., are only limited to App Engine targets. |
| `schedule` | String |  | Required, except when used with UpdateJob. Describes the schedule on which the job will be executed. The schedule can be either of the following types: * [Crontab](https://en.wikipedia.org/wiki/Cron#Overview) * English-like [schedule](/scheduler/docs/configuring/cron-job-schedules) As a general rule, execution `n + 1` of a job will not begin until execution `n` has finished. Cloud Scheduler will never allow two simultaneously outstanding executions. For example, this implies that if the `n+1`th execution is scheduled to run at 16:00 but the `n`th execution takes until 16:15, the `n+1`th execution will not start until `16:15`. A scheduled start time will be delayed if the previous execution has not ended when its scheduled time occurs. If retry_count > 0 and a job attempt fails, the job will be tried a total of retry_count times, with exponential backoff, until the next scheduled start time. If retry_count is 0, a job attempt will not be retried if it fails. Instead the Cloud Scheduler system will wait for the next scheduled execution time. Setting retry_count to 0 does not prevent failed jobs from running according to schedule after the failure. |
| `pubsub_target` | String |  | Pub/Sub target. |
| `time_zone` | String |  | Specifies the time zone to be used in interpreting schedule. The value of this field must be a time zone name from the [tz database](http://en.wikipedia.org/wiki/Tz_database). Note that some time zones include a provision for daylight savings time. The rules for daylight saving time are determined by the chosen tz. For UTC use the string "utc". If a time zone is not specified, the default will be in UTC (also known as GMT). |
| `schedule_time` | String |  | Output only. The next time the job is scheduled. Note that this may be a retry of a previously failed attempt or the next execution time according to the schedule. |
| `state` | String |  | Output only. State of the job. |
| `attempt_deadline` | String |  | The deadline for job attempts. If the request handler does not respond by this deadline then the request is cancelled and the attempt is marked as a `DEADLINE_EXCEEDED` failure. The failed attempt can be viewed in execution logs. Cloud Scheduler will retry the job according to the RetryConfig. The default and the allowed values depend on the type of target: * For HTTP targets, the default is 3 minutes. The deadline must be in the interval [15 seconds, 30 minutes]. * For App Engine HTTP targets, 0 indicates that the request has the default deadline. The default deadline depends on the scaling type of the service: 10 minutes for standard apps with automatic scaling, 24 hours for standard apps with manual and basic scaling, and 60 minutes for flex apps. If the request deadline is set, it must be in the interval [15 seconds, 24 hours 15 seconds]. * For Pub/Sub targets, this field is ignored. |
| `retry_config` | String |  | Settings that determine the retry behavior. |
| `user_update_time` | String |  | Output only. The creation time of the job. |
| `description` | String |  | Optionally caller-specified in CreateJob or UpdateJob. A human-readable description for the job. This string must not contain more than 500 characters. |
| `satisfies_pzs` | bool |  | Output only. Whether or not this Job satisfies the requirements of physical zone separation |
| `app_engine_http_target` | String |  | App Engine HTTP target. |
| `http_target` | String |  | HTTP target. |
| `parent` | String | ✅ | Required. The location name. For example: `projects/PROJECT_ID/locations/LOCATION_ID`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | Output only. The response from the target for the last attempted execution. |
| `last_attempt_time` | String | Output only. The time the last job attempt started. |
| `name` | String | Optionally caller-specified in CreateJob, after which it becomes output only. The job name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/jobs/JOB_ID`. * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), or periods (.). For more information, see [Identifying projects](/resource-manager/docs/creating-managing-projects#identifying_projects) * `LOCATION_ID` is the canonical ID for the job's location. The list of available locations can be obtained by calling [locations.list](/scheduler/docs/reference/rest/v1beta1/projects.locations/list). For more information, see [Cloud Scheduler locations](/scheduler/docs/locations). * `JOB_ID` can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), or underscores (_). The maximum length is 500 characters. |
| `legacy_app_engine_cron` | bool | Immutable. This field is used to manage the legacy App Engine Cron jobs using the Cloud Scheduler API. If the field is set to true, the job will be considered a legacy job. Note that App Engine Cron jobs have fewer features than Cloud Scheduler jobs, e.g., are only limited to App Engine targets. |
| `schedule` | String | Required, except when used with UpdateJob. Describes the schedule on which the job will be executed. The schedule can be either of the following types: * [Crontab](https://en.wikipedia.org/wiki/Cron#Overview) * English-like [schedule](/scheduler/docs/configuring/cron-job-schedules) As a general rule, execution `n + 1` of a job will not begin until execution `n` has finished. Cloud Scheduler will never allow two simultaneously outstanding executions. For example, this implies that if the `n+1`th execution is scheduled to run at 16:00 but the `n`th execution takes until 16:15, the `n+1`th execution will not start until `16:15`. A scheduled start time will be delayed if the previous execution has not ended when its scheduled time occurs. If retry_count > 0 and a job attempt fails, the job will be tried a total of retry_count times, with exponential backoff, until the next scheduled start time. If retry_count is 0, a job attempt will not be retried if it fails. Instead the Cloud Scheduler system will wait for the next scheduled execution time. Setting retry_count to 0 does not prevent failed jobs from running according to schedule after the failure. |
| `pubsub_target` | String | Pub/Sub target. |
| `time_zone` | String | Specifies the time zone to be used in interpreting schedule. The value of this field must be a time zone name from the [tz database](http://en.wikipedia.org/wiki/Tz_database). Note that some time zones include a provision for daylight savings time. The rules for daylight saving time are determined by the chosen tz. For UTC use the string "utc". If a time zone is not specified, the default will be in UTC (also known as GMT). |
| `schedule_time` | String | Output only. The next time the job is scheduled. Note that this may be a retry of a previously failed attempt or the next execution time according to the schedule. |
| `state` | String | Output only. State of the job. |
| `attempt_deadline` | String | The deadline for job attempts. If the request handler does not respond by this deadline then the request is cancelled and the attempt is marked as a `DEADLINE_EXCEEDED` failure. The failed attempt can be viewed in execution logs. Cloud Scheduler will retry the job according to the RetryConfig. The default and the allowed values depend on the type of target: * For HTTP targets, the default is 3 minutes. The deadline must be in the interval [15 seconds, 30 minutes]. * For App Engine HTTP targets, 0 indicates that the request has the default deadline. The default deadline depends on the scaling type of the service: 10 minutes for standard apps with automatic scaling, 24 hours for standard apps with manual and basic scaling, and 60 minutes for flex apps. If the request deadline is set, it must be in the interval [15 seconds, 24 hours 15 seconds]. * For Pub/Sub targets, this field is ignored. |
| `retry_config` | String | Settings that determine the retry behavior. |
| `user_update_time` | String | Output only. The creation time of the job. |
| `description` | String | Optionally caller-specified in CreateJob or UpdateJob. A human-readable description for the job. This string must not contain more than 500 characters. |
| `satisfies_pzs` | bool | Output only. Whether or not this Job satisfies the requirements of physical zone separation |
| `app_engine_http_target` | String | App Engine HTTP target. |
| `http_target` | String | HTTP target. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.cloudscheduler_api.Job {
    parent = "value"  # Required. The location name. For example: `projects/PROJECT_ID/locations/LOCATION_ID`.
}

# Access job outputs
job_id = job.id
job_status = job.status
job_last_attempt_time = job.last_attempt_time
job_name = job.name
job_legacy_app_engine_cron = job.legacy_app_engine_cron
job_schedule = job.schedule
job_pubsub_target = job.pubsub_target
job_time_zone = job.time_zone
job_schedule_time = job.schedule_time
job_state = job.state
job_attempt_deadline = job.attempt_deadline
job_retry_config = job.retry_config
job_user_update_time = job.user_update_time
job_description = job.description
job_satisfies_pzs = job.satisfies_pzs
job_app_engine_http_target = job.app_engine_http_target
job_http_target = job.http_target
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_metadata = location.metadata
location_display_name = location.display_name
location_name = location.name
location_location_id = location.location_id
location_labels = location.labels
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

# Create operation
operation = provider.cloudscheduler_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_response = operation.response
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

# Create multiple operation resources
operation_0 = provider.cloudscheduler_api.Operation {
    name = "value-0"
}
operation_1 = provider.cloudscheduler_api.Operation {
    name = "value-1"
}
operation_2 = provider.cloudscheduler_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.cloudscheduler_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Cloudscheduler_api Documentation](https://cloud.google.com/cloudscheduler_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
