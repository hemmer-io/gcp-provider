# Cloudtasks_api Service



**Resources**: 9

---

## Overview

The cloudtasks_api service provides access to 9 resource types:

- [Task](#task) [CRD]
- [Queue](#queue) [CRUD]
- [Location](#location) [RU]
- [Queue](#queue) [CRUD]
- [Location](#location) [RU]
- [Task](#task) [CRD]
- [Task](#task) [CRD]
- [Location](#location) [RU]
- [Queue](#queue) [CRUD]

---

## Resources


### Task

Creates a task and adds it to a queue. Tasks cannot be updated after creation; there is no UpdateTask command. * The maximum task size is 100KB.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `response_view` | String |  | The response_view specifies which subset of the Task will be returned. By default response_view is BASIC; not all information is retrieved by default because some data, such as payloads, might be desirable to return only when needed because of its large size or because of the sensitivity of data that it contains. Authorization for FULL requires `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/) permission on the Task resource. |
| `task` | String |  | Required. The task to add. Task names have the following format: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`. The user can optionally specify a task name. If a name is not specified then the system will generate a random unique task id, which will be set in the task returned in the response. If schedule_time is not set or is in the past then Cloud Tasks will set it to the current time. Task De-duplication: Explicitly specifying a task ID enables task de-duplication. If a task's ID is identical to that of an existing task or a task that was deleted or executed recently then the call will fail with ALREADY_EXISTS. The IDs of deleted tasks are not immediately available for reuse. It can take up to 24 hours (or 9 days if the task's queue was created using a queue.yaml or queue.xml) for the task ID to be released and made available again. Because there is an extra lookup cost to identify duplicate task names, these CreateTask calls have significantly increased latency. Using hashed strings for the task id or for the prefix of the task id is recommended. Choosing task ids that are sequential or have sequential prefixes, for example using a timestamp, causes an increase in latency and error rates in all task commands. The infrastructure relies on an approximately uniform distribution of task ids to store and serve tasks efficiently. |
| `parent` | String | ✅ | Required. The queue name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID` The queue must already exist. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dispatch_deadline` | String | The deadline for requests sent to the worker. If the worker does not respond by this deadline then the request is cancelled and the attempt is marked as a `DEADLINE_EXCEEDED` failure. Cloud Tasks will retry the task according to the RetryConfig. Note that when the request is cancelled, Cloud Tasks will stop listening for the response, but whether the worker stops processing depends on the worker. For example, if the worker is stuck, it may not react to cancelled requests. The default and maximum values depend on the type of request: * For HTTP tasks, the default is 10 minutes. The deadline must be in the interval [15 seconds, 30 minutes]. * For App Engine tasks, 0 indicates that the request has the default deadline. The default deadline depends on the [scaling type](https://cloud.google.com/appengine/docs/standard/go/how-instances-are-managed#instance_scaling) of the service: 10 minutes for standard apps with automatic scaling, 24 hours for standard apps with manual and basic scaling, and 60 minutes for flex apps. If the request deadline is set, it must be in the interval [15 seconds, 24 hours 15 seconds]. Regardless of the task's `dispatch_deadline`, the app handler will not run for longer than than the service's timeout. We recommend setting the `dispatch_deadline` to at most a few seconds more than the app handler's timeout. For more information see [Timeouts](https://cloud.google.com/tasks/docs/creating-appengine-handlers#timeouts). The value must be given as a string that indicates the length of time (in seconds) followed by `s` (for "seconds"). For more information on the format, see the documentation for [Duration](https://protobuf.dev/reference/protobuf/google.protobuf/#duration). `dispatch_deadline` will be truncated to the nearest millisecond. The deadline is an approximate deadline. |
| `response_count` | i64 | Output only. The number of attempts which have received a response. |
| `first_attempt` | String | Output only. The status of the task's first attempt. Only dispatch_time will be set. The other Attempt information is not retained by Cloud Tasks. |
| `schedule_time` | String | The time when the task is scheduled to be attempted or retried. `schedule_time` will be truncated to the nearest microsecond. |
| `http_request` | String | HTTP request that is sent to the worker. An HTTP task is a task that has HttpRequest set. |
| `dispatch_count` | i64 | Output only. The number of attempts dispatched. This count includes attempts which have been dispatched but haven't received a response. |
| `last_attempt` | String | Output only. The status of the task's last attempt. |
| `name` | String | Optionally caller-specified in CreateTask. The task name. The task name must have the following format: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID` * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), or periods (.). For more information, see [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects) * `LOCATION_ID` is the canonical ID for the task's location. The list of available locations can be obtained by calling ListLocations. For more information, see https://cloud.google.com/about/locations/. * `QUEUE_ID` can contain letters ([A-Za-z]), numbers ([0-9]), or hyphens (-). The maximum length is 100 characters. * `TASK_ID` can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), or underscores (_). The maximum length is 500 characters. |
| `view` | String | Output only. The view specifies which subset of the Task has been returned. |
| `app_engine_http_request` | String | HTTP request that is sent to the App Engine app handler. An App Engine task is a task that has AppEngineHttpRequest set. |
| `create_time` | String | Output only. The time that the task was created. `create_time` will be truncated to the nearest second. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create task
task = provider.cloudtasks_api.Task {
    parent = "value"  # Required. The queue name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID` The queue must already exist.
}

# Access task outputs
task_id = task.id
task_dispatch_deadline = task.dispatch_deadline
task_response_count = task.response_count
task_first_attempt = task.first_attempt
task_schedule_time = task.schedule_time
task_http_request = task.http_request
task_dispatch_count = task.dispatch_count
task_last_attempt = task.last_attempt
task_name = task.name
task_view = task.view
task_app_engine_http_request = task.app_engine_http_request
task_create_time = task.create_time
```

---


### Queue

Creates a queue. Queues created with this method allow tasks to live for a maximum of 31 days. After a task is 31 days old, the task will be deleted regardless of whether it was dispatched or not. WARNING: Using this method may have unintended side effects if you are using an App Engine `queue.yaml` or `queue.xml` file to manage your queues. Read [Overview of Queue Management and queue.yaml](https://cloud.google.com/tasks/docs/queue-yaml) before using this method.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `retry_config` | String |  | Settings that determine the retry behavior. * For tasks created using Cloud Tasks: the queue-level retry settings apply to all tasks in the queue that were created using Cloud Tasks. Retry settings cannot be set on individual tasks. * For tasks created using the App Engine SDK: the queue-level retry settings apply to all tasks in the queue which do not have retry settings explicitly set on the task and were created by the App Engine SDK. See [App Engine documentation](https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/retrying-tasks). |
| `rate_limits` | String |  | Rate limits for task dispatches. rate_limits and retry_config are related because they both control task attempts. However they control task attempts in different ways: * rate_limits controls the total rate of dispatches from a queue (i.e. all traffic dispatched from the queue, regardless of whether the dispatch is from a first attempt or a retry). * retry_config controls what happens to particular a task after its first attempt fails. That is, retry_config controls task retries (the second attempt, third attempt, etc). The queue's actual dispatch rate is the result of: * Number of tasks in the queue * User-specified throttling: rate_limits, retry_config, and the queue's state. * System throttling due to `429` (Too Many Requests) or `503` (Service Unavailable) responses from the worker, high error rates, or to smooth sudden large traffic spikes. |
| `app_engine_routing_override` | String |  | Overrides for task-level app_engine_routing. These settings apply only to App Engine tasks in this queue. Http tasks are not affected. If set, `app_engine_routing_override` is used for all App Engine tasks in the queue, no matter what the setting is for the task-level app_engine_routing. |
| `purge_time` | String |  | Output only. The last time this queue was purged. All tasks that were created before this time were purged. A queue can be purged using PurgeQueue, the [App Engine Task Queue SDK, or the Cloud Console](https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/deleting-tasks-and-queues#purging_all_tasks_from_a_queue). Purge time will be truncated to the nearest microsecond. Purge time will be unset if the queue has never been purged. |
| `state` | String |  | Output only. The state of the queue. `state` can only be changed by calling PauseQueue, ResumeQueue, or uploading [queue.yaml/xml](https://cloud.google.com/appengine/docs/python/config/queueref). UpdateQueue cannot be used to change `state`. |
| `stackdriver_logging_config` | String |  | Configuration options for writing logs to [Stackdriver Logging](https://cloud.google.com/logging/docs/). If this field is unset, then no logs are written. |
| `name` | String |  | Caller-specified and required in CreateQueue, after which it becomes output only. The queue name. The queue name must have the following format: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID` * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), or periods (.). For more information, see [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects) * `LOCATION_ID` is the canonical ID for the queue's location. The list of available locations can be obtained by calling ListLocations. For more information, see https://cloud.google.com/about/locations/. * `QUEUE_ID` can contain letters ([A-Za-z]), numbers ([0-9]), or hyphens (-). The maximum length is 100 characters. |
| `http_target` | String |  | Modifies HTTP target for HTTP tasks. |
| `parent` | String | ✅ | Required. The location name in which the queue will be created. For example: `projects/PROJECT_ID/locations/LOCATION_ID` The list of allowed locations can be obtained by calling Cloud Tasks' implementation of ListLocations. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `retry_config` | String | Settings that determine the retry behavior. * For tasks created using Cloud Tasks: the queue-level retry settings apply to all tasks in the queue that were created using Cloud Tasks. Retry settings cannot be set on individual tasks. * For tasks created using the App Engine SDK: the queue-level retry settings apply to all tasks in the queue which do not have retry settings explicitly set on the task and were created by the App Engine SDK. See [App Engine documentation](https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/retrying-tasks). |
| `rate_limits` | String | Rate limits for task dispatches. rate_limits and retry_config are related because they both control task attempts. However they control task attempts in different ways: * rate_limits controls the total rate of dispatches from a queue (i.e. all traffic dispatched from the queue, regardless of whether the dispatch is from a first attempt or a retry). * retry_config controls what happens to particular a task after its first attempt fails. That is, retry_config controls task retries (the second attempt, third attempt, etc). The queue's actual dispatch rate is the result of: * Number of tasks in the queue * User-specified throttling: rate_limits, retry_config, and the queue's state. * System throttling due to `429` (Too Many Requests) or `503` (Service Unavailable) responses from the worker, high error rates, or to smooth sudden large traffic spikes. |
| `app_engine_routing_override` | String | Overrides for task-level app_engine_routing. These settings apply only to App Engine tasks in this queue. Http tasks are not affected. If set, `app_engine_routing_override` is used for all App Engine tasks in the queue, no matter what the setting is for the task-level app_engine_routing. |
| `purge_time` | String | Output only. The last time this queue was purged. All tasks that were created before this time were purged. A queue can be purged using PurgeQueue, the [App Engine Task Queue SDK, or the Cloud Console](https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/deleting-tasks-and-queues#purging_all_tasks_from_a_queue). Purge time will be truncated to the nearest microsecond. Purge time will be unset if the queue has never been purged. |
| `state` | String | Output only. The state of the queue. `state` can only be changed by calling PauseQueue, ResumeQueue, or uploading [queue.yaml/xml](https://cloud.google.com/appengine/docs/python/config/queueref). UpdateQueue cannot be used to change `state`. |
| `stackdriver_logging_config` | String | Configuration options for writing logs to [Stackdriver Logging](https://cloud.google.com/logging/docs/). If this field is unset, then no logs are written. |
| `name` | String | Caller-specified and required in CreateQueue, after which it becomes output only. The queue name. The queue name must have the following format: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID` * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), or periods (.). For more information, see [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects) * `LOCATION_ID` is the canonical ID for the queue's location. The list of available locations can be obtained by calling ListLocations. For more information, see https://cloud.google.com/about/locations/. * `QUEUE_ID` can contain letters ([A-Za-z]), numbers ([0-9]), or hyphens (-). The maximum length is 100 characters. |
| `http_target` | String | Modifies HTTP target for HTTP tasks. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create queue
queue = provider.cloudtasks_api.Queue {
    parent = "value"  # Required. The location name in which the queue will be created. For example: `projects/PROJECT_ID/locations/LOCATION_ID` The list of allowed locations can be obtained by calling Cloud Tasks' implementation of ListLocations.
}

# Access queue outputs
queue_id = queue.id
queue_retry_config = queue.retry_config
queue_rate_limits = queue.rate_limits
queue_app_engine_routing_override = queue.app_engine_routing_override
queue_purge_time = queue.purge_time
queue_state = queue.state
queue_stackdriver_logging_config = queue.stackdriver_logging_config
queue_name = queue.name
queue_http_target = queue.http_target
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The config resource name which includes the project and location and must end in 'cmekConfig', in the format projects/PROJECT_ID/locations/LOCATION_ID/cmekConfig` |
| `kms_key` | String |  | Resource name of the Cloud KMS key, of the form `projects/PROJECT_ID/locations/LOCATION_ID/keyRings/KEY_RING_ID/cryptoKeys/KEY_ID`, that will be used to encrypt the Queues & Tasks in the region. Setting this as blank will turn off CMEK encryption. |
| `name` | String | ✅ | Output only. The config resource name which includes the project and location and must end in 'cmekConfig', in the format projects/PROJECT_ID/locations/LOCATION_ID/cmekConfig` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
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
location_name = location.name
location_labels = location.labels
location_metadata = location.metadata
location_display_name = location.display_name
location_location_id = location.location_id
```

---


### Queue

Creates a queue. Queues created with this method allow tasks to live for a maximum of 31 days. After a task is 31 days old, the task will be deleted regardless of whether it was dispatched or not. WARNING: Using this method may have unintended side effects if you are using an App Engine `queue.yaml` or `queue.xml` file to manage your queues. Read [Overview of Queue Management and queue.yaml](https://cloud.google.com/tasks/docs/queue-yaml) before using this method.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `stats` | String |  | Output only. The realtime, informational statistics for a queue. In order to receive the statistics the caller should include this field in the FieldMask. |
| `name` | String |  | Caller-specified and required in CreateQueue, after which it becomes output only. The queue name. The queue name must have the following format: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID` * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), or periods (.). For more information, see [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects) * `LOCATION_ID` is the canonical ID for the queue's location. The list of available locations can be obtained by calling ListLocations. For more information, see https://cloud.google.com/about/locations/. * `QUEUE_ID` can contain letters ([A-Za-z]), numbers ([0-9]), or hyphens (-). The maximum length is 100 characters. |
| `pull_target` | String |  | Pull target. A pull queue is a queue that has a PullTarget. |
| `rate_limits` | String |  | Rate limits for task dispatches. rate_limits and retry_config are related because they both control task attempts however they control how tasks are attempted in different ways: * rate_limits controls the total rate of dispatches from a queue (i.e. all traffic dispatched from the queue, regardless of whether the dispatch is from a first attempt or a retry). * retry_config controls what happens to particular a task after its first attempt fails. That is, retry_config controls task retries (the second attempt, third attempt, etc). |
| `task_ttl` | String |  | The maximum amount of time that a task will be retained in this queue. Queues created by Cloud Tasks have a default `task_ttl` of 31 days. After a task has lived for `task_ttl`, the task will be deleted regardless of whether it was dispatched or not. The `task_ttl` for queues created via queue.yaml/xml is equal to the maximum duration because there is a [storage quota](https://cloud.google.com/appengine/quotas#Task_Queue) for these queues. To view the maximum valid duration, see the documentation for Duration. |
| `app_engine_http_target` | String |  | App Engine HTTP target. An App Engine queue is a queue that has an AppEngineHttpTarget. |
| `purge_time` | String |  | Output only. The last time this queue was purged. All tasks that were created before this time were purged. A queue can be purged using PurgeQueue, the [App Engine Task Queue SDK, or the Cloud Console](https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/deleting-tasks-and-queues#purging_all_tasks_from_a_queue). Purge time will be truncated to the nearest microsecond. Purge time will be unset if the queue has never been purged. |
| `state` | String |  | Output only. The state of the queue. `state` can only be changed by called PauseQueue, ResumeQueue, or uploading [queue.yaml/xml](https://cloud.google.com/appengine/docs/python/config/queueref). UpdateQueue cannot be used to change `state`. |
| `tombstone_ttl` | String |  | The task tombstone time to live (TTL). After a task is deleted or completed, the task's tombstone is retained for the length of time specified by `tombstone_ttl`. The tombstone is used by task de-duplication; another task with the same name can't be created until the tombstone has expired. For more information about task de-duplication, see the documentation for CreateTaskRequest. Queues created by Cloud Tasks have a default `tombstone_ttl` of 1 hour. |
| `http_target` | String |  | An http_target is used to override the target values for HTTP tasks. |
| `retry_config` | String |  | Settings that determine the retry behavior. * For tasks created using Cloud Tasks: the queue-level retry settings apply to all tasks in the queue that were created using Cloud Tasks. Retry settings cannot be set on individual tasks. * For tasks created using the App Engine SDK: the queue-level retry settings apply to all tasks in the queue which do not have retry settings explicitly set on the task and were created by the App Engine SDK. See [App Engine documentation](https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/retrying-tasks). |
| `parent` | String | ✅ | Required. The location name in which the queue will be created. For example: `projects/PROJECT_ID/locations/LOCATION_ID` The list of allowed locations can be obtained by calling Cloud Tasks' implementation of ListLocations. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stats` | String | Output only. The realtime, informational statistics for a queue. In order to receive the statistics the caller should include this field in the FieldMask. |
| `name` | String | Caller-specified and required in CreateQueue, after which it becomes output only. The queue name. The queue name must have the following format: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID` * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), or periods (.). For more information, see [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects) * `LOCATION_ID` is the canonical ID for the queue's location. The list of available locations can be obtained by calling ListLocations. For more information, see https://cloud.google.com/about/locations/. * `QUEUE_ID` can contain letters ([A-Za-z]), numbers ([0-9]), or hyphens (-). The maximum length is 100 characters. |
| `pull_target` | String | Pull target. A pull queue is a queue that has a PullTarget. |
| `rate_limits` | String | Rate limits for task dispatches. rate_limits and retry_config are related because they both control task attempts however they control how tasks are attempted in different ways: * rate_limits controls the total rate of dispatches from a queue (i.e. all traffic dispatched from the queue, regardless of whether the dispatch is from a first attempt or a retry). * retry_config controls what happens to particular a task after its first attempt fails. That is, retry_config controls task retries (the second attempt, third attempt, etc). |
| `task_ttl` | String | The maximum amount of time that a task will be retained in this queue. Queues created by Cloud Tasks have a default `task_ttl` of 31 days. After a task has lived for `task_ttl`, the task will be deleted regardless of whether it was dispatched or not. The `task_ttl` for queues created via queue.yaml/xml is equal to the maximum duration because there is a [storage quota](https://cloud.google.com/appengine/quotas#Task_Queue) for these queues. To view the maximum valid duration, see the documentation for Duration. |
| `app_engine_http_target` | String | App Engine HTTP target. An App Engine queue is a queue that has an AppEngineHttpTarget. |
| `purge_time` | String | Output only. The last time this queue was purged. All tasks that were created before this time were purged. A queue can be purged using PurgeQueue, the [App Engine Task Queue SDK, or the Cloud Console](https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/deleting-tasks-and-queues#purging_all_tasks_from_a_queue). Purge time will be truncated to the nearest microsecond. Purge time will be unset if the queue has never been purged. |
| `state` | String | Output only. The state of the queue. `state` can only be changed by called PauseQueue, ResumeQueue, or uploading [queue.yaml/xml](https://cloud.google.com/appengine/docs/python/config/queueref). UpdateQueue cannot be used to change `state`. |
| `tombstone_ttl` | String | The task tombstone time to live (TTL). After a task is deleted or completed, the task's tombstone is retained for the length of time specified by `tombstone_ttl`. The tombstone is used by task de-duplication; another task with the same name can't be created until the tombstone has expired. For more information about task de-duplication, see the documentation for CreateTaskRequest. Queues created by Cloud Tasks have a default `tombstone_ttl` of 1 hour. |
| `http_target` | String | An http_target is used to override the target values for HTTP tasks. |
| `retry_config` | String | Settings that determine the retry behavior. * For tasks created using Cloud Tasks: the queue-level retry settings apply to all tasks in the queue that were created using Cloud Tasks. Retry settings cannot be set on individual tasks. * For tasks created using the App Engine SDK: the queue-level retry settings apply to all tasks in the queue which do not have retry settings explicitly set on the task and were created by the App Engine SDK. See [App Engine documentation](https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/retrying-tasks). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create queue
queue = provider.cloudtasks_api.Queue {
    parent = "value"  # Required. The location name in which the queue will be created. For example: `projects/PROJECT_ID/locations/LOCATION_ID` The list of allowed locations can be obtained by calling Cloud Tasks' implementation of ListLocations.
}

# Access queue outputs
queue_id = queue.id
queue_stats = queue.stats
queue_name = queue.name
queue_pull_target = queue.pull_target
queue_rate_limits = queue.rate_limits
queue_task_ttl = queue.task_ttl
queue_app_engine_http_target = queue.app_engine_http_target
queue_purge_time = queue.purge_time
queue_state = queue.state
queue_tombstone_ttl = queue.tombstone_ttl
queue_http_target = queue.http_target
queue_retry_config = queue.retry_config
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The config resource name which includes the project and location and must end in 'cmekConfig', in the format projects/PROJECT_ID/locations/LOCATION_ID/cmekConfig` |
| `kms_key` | String |  | Resource name of the Cloud KMS key, of the form `projects/PROJECT_ID/locations/LOCATION_ID/keyRings/KEY_RING_ID/cryptoKeys/KEY_ID`, that will be used to encrypt the Queues & Tasks in the region. Setting this as blank will turn off CMEK encryption. |
| `name` | String | ✅ | Output only. The config resource name which includes the project and location and must end in 'cmekConfig', in the format projects/PROJECT_ID/locations/LOCATION_ID/cmekConfig` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |


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
location_metadata = location.metadata
location_name = location.name
location_location_id = location.location_id
location_display_name = location.display_name
```

---


### Task

Creates a task and adds it to a queue. Tasks cannot be updated after creation; there is no UpdateTask command. * For App Engine queues, the maximum task size is 100KB. * For pull queues, the maximum task size is 1MB.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `response_view` | String |  | The response_view specifies which subset of the Task will be returned. By default response_view is BASIC; not all information is retrieved by default because some data, such as payloads, might be desirable to return only when needed because of its large size or because of the sensitivity of data that it contains. Authorization for FULL requires `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/) permission on the Task resource. |
| `task` | String |  | Required. The task to add. Task names have the following format: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`. The user can optionally specify a task name. If a name is not specified then the system will generate a random unique task id, which will be set in the task returned in the response. If schedule_time is not set or is in the past then Cloud Tasks will set it to the current time. Task De-duplication: Explicitly specifying a task ID enables task de-duplication. If a task's ID is identical to that of an existing task or a task that was deleted or completed recently then the call will fail with ALREADY_EXISTS. The IDs of deleted tasks are not immediately available for reuse. It can take up to 24 hours (or 9 days if the task's queue was created using a queue.yaml or queue.xml) for the task ID to be released and made available again. Because there is an extra lookup cost to identify duplicate task names, these CreateTask calls have significantly increased latency. Using hashed strings for the task id or for the prefix of the task id is recommended. Choosing task ids that are sequential or have sequential prefixes, for example using a timestamp, causes an increase in latency and error rates in all task commands. The infrastructure relies on an approximately uniform distribution of task ids to store and serve tasks efficiently. |
| `parent` | String | ✅ | Required. The queue name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID` The queue must already exist. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `http_request` | String | HTTP request that is sent to the task's target. An HTTP task is a task that has HttpRequest set. |
| `create_time` | String | Output only. The time that the task was created. `create_time` will be truncated to the nearest second. |
| `status` | String | Output only. The task status. |
| `app_engine_http_request` | String | App Engine HTTP request that is sent to the task's target. Can be set only if app_engine_http_target is set on the queue. An App Engine task is a task that has AppEngineHttpRequest set. |
| `view` | String | Output only. The view specifies which subset of the Task has been returned. |
| `name` | String | Optionally caller-specified in CreateTask. The task name. The task name must have the following format: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID` * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), or periods (.). For more information, see [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects) * `LOCATION_ID` is the canonical ID for the task's location. The list of available locations can be obtained by calling ListLocations. For more information, see https://cloud.google.com/about/locations/. * `QUEUE_ID` can contain letters ([A-Za-z]), numbers ([0-9]), or hyphens (-). The maximum length is 100 characters. * `TASK_ID` can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), or underscores (_). The maximum length is 500 characters. |
| `schedule_time` | String | The time when the task is scheduled to be attempted. For App Engine queues, this is when the task will be attempted or retried. For pull queues, this is the time when the task is available to be leased; if a task is currently leased, this is the time when the current lease expires, that is, the time that the task was leased plus the lease_duration. `schedule_time` will be truncated to the nearest microsecond. |
| `pull_message` | String | LeaseTasks to process the task. Can be set only if pull_target is set on the queue. A pull task is a task that has PullMessage set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create task
task = provider.cloudtasks_api.Task {
    parent = "value"  # Required. The queue name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID` The queue must already exist.
}

# Access task outputs
task_id = task.id
task_http_request = task.http_request
task_create_time = task.create_time
task_status = task.status
task_app_engine_http_request = task.app_engine_http_request
task_view = task.view
task_name = task.name
task_schedule_time = task.schedule_time
task_pull_message = task.pull_message
```

---


### Task

Creates a task and adds it to a queue. Tasks cannot be updated after creation; there is no UpdateTask command. * The maximum task size is 100KB.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `response_view` | String |  | The response_view specifies which subset of the Task will be returned. By default response_view is BASIC; not all information is retrieved by default because some data, such as payloads, might be desirable to return only when needed because of its large size or because of the sensitivity of data that it contains. Authorization for FULL requires `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/) permission on the Task resource. |
| `task` | String |  | Required. The task to add. Task names have the following format: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`. The user can optionally specify a task name. If a name is not specified then the system will generate a random unique task id, which will be set in the task returned in the response. If schedule_time is not set or is in the past then Cloud Tasks will set it to the current time. Task De-duplication: Explicitly specifying a task ID enables task de-duplication. If a task's ID is identical to that of an existing task or a task that was deleted or executed recently then the call will fail with ALREADY_EXISTS. The IDs of deleted tasks are not immediately available for reuse. It can take up to 24 hours (or 9 days if the task's queue was created using a queue.yaml or queue.xml) for the task ID to be released and made available again. Because there is an extra lookup cost to identify duplicate task names, these CreateTask calls have significantly increased latency. Using hashed strings for the task id or for the prefix of the task id is recommended. Choosing task ids that are sequential or have sequential prefixes, for example using a timestamp, causes an increase in latency and error rates in all task commands. The infrastructure relies on an approximately uniform distribution of task ids to store and serve tasks efficiently. |
| `parent` | String | ✅ | Required. The queue name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID` The queue must already exist. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_engine_http_request` | String | HTTP request that is sent to the App Engine app handler. An App Engine task is a task that has AppEngineHttpRequest set. |
| `schedule_time` | String | The time when the task is scheduled to be attempted. For App Engine queues, this is when the task will be attempted or retried. `schedule_time` will be truncated to the nearest microsecond. |
| `response_count` | i64 | Output only. The number of attempts which have received a response. |
| `first_attempt` | String | Output only. The status of the task's first attempt. Only dispatch_time will be set. The other Attempt information is not retained by Cloud Tasks. |
| `dispatch_deadline` | String | The deadline for requests sent to the worker. If the worker does not respond by this deadline then the request is cancelled and the attempt is marked as a `DEADLINE_EXCEEDED` failure. Cloud Tasks will retry the task according to the RetryConfig. Note that when the request is cancelled, Cloud Tasks will stop listening for the response, but whether the worker stops processing depends on the worker. For example, if the worker is stuck, it may not react to cancelled requests. The default and maximum values depend on the type of request: * For HTTP tasks, the default is 10 minutes. The deadline must be in the interval [15 seconds, 30 minutes]. * For App Engine tasks, 0 indicates that the request has the default deadline. The default deadline depends on the [scaling type](https://cloud.google.com/appengine/docs/standard/go/how-instances-are-managed#instance_scaling) of the service: 10 minutes for standard apps with automatic scaling, 24 hours for standard apps with manual and basic scaling, and 60 minutes for flex apps. If the request deadline is set, it must be in the interval [15 seconds, 24 hours 15 seconds]. Regardless of the task's `dispatch_deadline`, the app handler will not run for longer than than the service's timeout. We recommend setting the `dispatch_deadline` to at most a few seconds more than the app handler's timeout. For more information see [Timeouts](https://cloud.google.com/tasks/docs/creating-appengine-handlers#timeouts). The value must be given as a string that indicates the length of time (in seconds) followed by `s` (for "seconds"). For more information on the format, see the documentation for [Duration](https://protobuf.dev/reference/protobuf/google.protobuf/#duration). `dispatch_deadline` will be truncated to the nearest millisecond. The deadline is an approximate deadline. |
| `dispatch_count` | i64 | Output only. The number of attempts dispatched. This count includes attempts which have been dispatched but haven't received a response. |
| `last_attempt` | String | Output only. The status of the task's last attempt. |
| `create_time` | String | Output only. The time that the task was created. `create_time` will be truncated to the nearest second. |
| `http_request` | String | HTTP request that is sent to the task's target. An HTTP task is a task that has HttpRequest set. |
| `pull_message` | String | Pull Message contained in a task in a PULL queue type. This payload type cannot be explicitly set through Cloud Tasks API. Its purpose, currently is to provide backward compatibility with App Engine Task Queue [pull](https://cloud.google.com/appengine/docs/standard/java/taskqueue/pull/) queues to provide a way to inspect contents of pull tasks through the CloudTasks.GetTask. |
| `view` | String | Output only. The view specifies which subset of the Task has been returned. |
| `name` | String | Optionally caller-specified in CreateTask. The task name. The task name must have the following format: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID` * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), or periods (.). For more information, see [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects) * `LOCATION_ID` is the canonical ID for the task's location. The list of available locations can be obtained by calling ListLocations. For more information, see https://cloud.google.com/about/locations/. * `QUEUE_ID` can contain letters ([A-Za-z]), numbers ([0-9]), or hyphens (-). The maximum length is 100 characters. * `TASK_ID` can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), or underscores (_). The maximum length is 500 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create task
task = provider.cloudtasks_api.Task {
    parent = "value"  # Required. The queue name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID` The queue must already exist.
}

# Access task outputs
task_id = task.id
task_app_engine_http_request = task.app_engine_http_request
task_schedule_time = task.schedule_time
task_response_count = task.response_count
task_first_attempt = task.first_attempt
task_dispatch_deadline = task.dispatch_deadline
task_dispatch_count = task.dispatch_count
task_last_attempt = task.last_attempt
task_create_time = task.create_time
task_http_request = task.http_request
task_pull_message = task.pull_message
task_view = task.view
task_name = task.name
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kms_key` | String |  | Resource name of the Cloud KMS key, of the form `projects/PROJECT_ID/locations/LOCATION_ID/keyRings/KEY_RING_ID/cryptoKeys/KEY_ID`, that will be used to encrypt the Queues & Tasks in the region. Setting this as blank will turn off CMEK encryption. |
| `name` | String |  | Output only. The config resource name which includes the project and location and must end in 'cmekConfig', in the format projects/PROJECT_ID/locations/LOCATION_ID/cmekConfig` |
| `name` | String | ✅ | Output only. The config resource name which includes the project and location and must end in 'cmekConfig', in the format projects/PROJECT_ID/locations/LOCATION_ID/cmekConfig` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
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
location_location_id = location.location_id
location_labels = location.labels
location_name = location.name
location_display_name = location.display_name
location_metadata = location.metadata
```

---


### Queue

Creates a queue. Queues created with this method allow tasks to live for a maximum of 31 days. After a task is 31 days old, the task will be deleted regardless of whether it was dispatched or not. WARNING: Using this method may have unintended side effects if you are using an App Engine `queue.yaml` or `queue.xml` file to manage your queues. Read [Overview of Queue Management and queue.yaml](https://cloud.google.com/tasks/docs/queue-yaml) before using this method.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_engine_http_queue` | String |  | AppEngineHttpQueue settings apply only to App Engine tasks in this queue. Http tasks are not affected by this proto. |
| `rate_limits` | String |  | Rate limits for task dispatches. rate_limits and retry_config are related because they both control task attempts. However they control task attempts in different ways: * rate_limits controls the total rate of dispatches from a queue (i.e. all traffic dispatched from the queue, regardless of whether the dispatch is from a first attempt or a retry). * retry_config controls what happens to particular a task after its first attempt fails. That is, retry_config controls task retries (the second attempt, third attempt, etc). The queue's actual dispatch rate is the result of: * Number of tasks in the queue * User-specified throttling: rate_limits, retry_config, and the queue's state. * System throttling due to `429` (Too Many Requests) or `503` (Service Unavailable) responses from the worker, high error rates, or to smooth sudden large traffic spikes. |
| `state` | String |  | Output only. The state of the queue. `state` can only be changed by called PauseQueue, ResumeQueue, or uploading [queue.yaml/xml](https://cloud.google.com/appengine/docs/python/config/queueref). UpdateQueue cannot be used to change `state`. |
| `task_ttl` | String |  | The maximum amount of time that a task will be retained in this queue. After a task has lived for `task_ttl`, the task will be deleted regardless of whether it was dispatched or not. The minimum value is 10 days. The maximum value is 10 years. The value must be given as a string that indicates the length of time (in seconds) followed by `s` (for "seconds"). For more information on the format, see the documentation for [Duration](https://protobuf.dev/reference/protobuf/google.protobuf/#duration). Queues created by Cloud Tasks have a default `task_ttl` of 31 days. . Queues created by queue.yaml/xml have a fixed `task_ttl` of the maximum duration, because there is a [storage quota](https://cloud.google.com/appengine/quotas#Task_Queue) for these queues. |
| `tombstone_ttl` | String |  | The task tombstone time to live (TTL). After a task is deleted or executed, the task's tombstone is retained for the length of time specified by `tombstone_ttl`. The tombstone is used by task de-duplication; another task with the same name can't be created until the tombstone has expired. For more information about task de-duplication, see the documentation for CreateTaskRequest. The minimum value is 1 hour. The maximum value is 9 days. The value must be given as a string that indicates the length of time (in seconds) followed by `s` (for "seconds"). For more information on the format, see the documentation for [Duration](https://protobuf.dev/reference/protobuf/google.protobuf/#duration). Queues created by Cloud Tasks have a default `tombstone_ttl` of 1 hour. |
| `name` | String |  | Caller-specified and required in CreateQueue, after which it becomes output only. The queue name. The queue name must have the following format: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID` * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), or periods (.). For more information, see [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects) * `LOCATION_ID` is the canonical ID for the queue's location. The list of available locations can be obtained by calling ListLocations. For more information, see https://cloud.google.com/about/locations/. * `QUEUE_ID` can contain letters ([A-Za-z]), numbers ([0-9]), or hyphens (-). The maximum length is 100 characters. |
| `retry_config` | String |  | Settings that determine the retry behavior. * For tasks created using Cloud Tasks: the queue-level retry settings apply to all tasks in the queue that were created using Cloud Tasks. Retry settings cannot be set on individual tasks. * For tasks created using the App Engine SDK: the queue-level retry settings apply to all tasks in the queue which do not have retry settings explicitly set on the task and were created by the App Engine SDK. See [App Engine documentation](https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/retrying-tasks). |
| `purge_time` | String |  | Output only. The last time this queue was purged. All tasks that were created before this time were purged. A queue can be purged using PurgeQueue, the [App Engine Task Queue SDK, or the Cloud Console](https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/deleting-tasks-and-queues#purging_all_tasks_from_a_queue). Purge time will be truncated to the nearest microsecond. Purge time will be unset if the queue has never been purged. |
| `stackdriver_logging_config` | String |  | Configuration options for writing logs to [Stackdriver Logging](https://cloud.google.com/logging/docs/). If this field is unset, then no logs are written. |
| `http_target` | String |  | Modifies HTTP target for HTTP tasks. |
| `stats` | String |  | Output only. The realtime, informational statistics for a queue. In order to receive the statistics the caller should include this field in the FieldMask. |
| `type` | String |  | Immutable. The type of a queue (push or pull). `Queue.type` is an immutable property of the queue that is set at the queue creation time. When left unspecified, the default value of `PUSH` is selected. |
| `parent` | String | ✅ | Required. The location name in which the queue will be created. For example: `projects/PROJECT_ID/locations/LOCATION_ID` The list of allowed locations can be obtained by calling Cloud Tasks' implementation of ListLocations. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_engine_http_queue` | String | AppEngineHttpQueue settings apply only to App Engine tasks in this queue. Http tasks are not affected by this proto. |
| `rate_limits` | String | Rate limits for task dispatches. rate_limits and retry_config are related because they both control task attempts. However they control task attempts in different ways: * rate_limits controls the total rate of dispatches from a queue (i.e. all traffic dispatched from the queue, regardless of whether the dispatch is from a first attempt or a retry). * retry_config controls what happens to particular a task after its first attempt fails. That is, retry_config controls task retries (the second attempt, third attempt, etc). The queue's actual dispatch rate is the result of: * Number of tasks in the queue * User-specified throttling: rate_limits, retry_config, and the queue's state. * System throttling due to `429` (Too Many Requests) or `503` (Service Unavailable) responses from the worker, high error rates, or to smooth sudden large traffic spikes. |
| `state` | String | Output only. The state of the queue. `state` can only be changed by called PauseQueue, ResumeQueue, or uploading [queue.yaml/xml](https://cloud.google.com/appengine/docs/python/config/queueref). UpdateQueue cannot be used to change `state`. |
| `task_ttl` | String | The maximum amount of time that a task will be retained in this queue. After a task has lived for `task_ttl`, the task will be deleted regardless of whether it was dispatched or not. The minimum value is 10 days. The maximum value is 10 years. The value must be given as a string that indicates the length of time (in seconds) followed by `s` (for "seconds"). For more information on the format, see the documentation for [Duration](https://protobuf.dev/reference/protobuf/google.protobuf/#duration). Queues created by Cloud Tasks have a default `task_ttl` of 31 days. . Queues created by queue.yaml/xml have a fixed `task_ttl` of the maximum duration, because there is a [storage quota](https://cloud.google.com/appengine/quotas#Task_Queue) for these queues. |
| `tombstone_ttl` | String | The task tombstone time to live (TTL). After a task is deleted or executed, the task's tombstone is retained for the length of time specified by `tombstone_ttl`. The tombstone is used by task de-duplication; another task with the same name can't be created until the tombstone has expired. For more information about task de-duplication, see the documentation for CreateTaskRequest. The minimum value is 1 hour. The maximum value is 9 days. The value must be given as a string that indicates the length of time (in seconds) followed by `s` (for "seconds"). For more information on the format, see the documentation for [Duration](https://protobuf.dev/reference/protobuf/google.protobuf/#duration). Queues created by Cloud Tasks have a default `tombstone_ttl` of 1 hour. |
| `name` | String | Caller-specified and required in CreateQueue, after which it becomes output only. The queue name. The queue name must have the following format: `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID` * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), or periods (.). For more information, see [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects) * `LOCATION_ID` is the canonical ID for the queue's location. The list of available locations can be obtained by calling ListLocations. For more information, see https://cloud.google.com/about/locations/. * `QUEUE_ID` can contain letters ([A-Za-z]), numbers ([0-9]), or hyphens (-). The maximum length is 100 characters. |
| `retry_config` | String | Settings that determine the retry behavior. * For tasks created using Cloud Tasks: the queue-level retry settings apply to all tasks in the queue that were created using Cloud Tasks. Retry settings cannot be set on individual tasks. * For tasks created using the App Engine SDK: the queue-level retry settings apply to all tasks in the queue which do not have retry settings explicitly set on the task and were created by the App Engine SDK. See [App Engine documentation](https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/retrying-tasks). |
| `purge_time` | String | Output only. The last time this queue was purged. All tasks that were created before this time were purged. A queue can be purged using PurgeQueue, the [App Engine Task Queue SDK, or the Cloud Console](https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/deleting-tasks-and-queues#purging_all_tasks_from_a_queue). Purge time will be truncated to the nearest microsecond. Purge time will be unset if the queue has never been purged. |
| `stackdriver_logging_config` | String | Configuration options for writing logs to [Stackdriver Logging](https://cloud.google.com/logging/docs/). If this field is unset, then no logs are written. |
| `http_target` | String | Modifies HTTP target for HTTP tasks. |
| `stats` | String | Output only. The realtime, informational statistics for a queue. In order to receive the statistics the caller should include this field in the FieldMask. |
| `type` | String | Immutable. The type of a queue (push or pull). `Queue.type` is an immutable property of the queue that is set at the queue creation time. When left unspecified, the default value of `PUSH` is selected. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create queue
queue = provider.cloudtasks_api.Queue {
    parent = "value"  # Required. The location name in which the queue will be created. For example: `projects/PROJECT_ID/locations/LOCATION_ID` The list of allowed locations can be obtained by calling Cloud Tasks' implementation of ListLocations.
}

# Access queue outputs
queue_id = queue.id
queue_app_engine_http_queue = queue.app_engine_http_queue
queue_rate_limits = queue.rate_limits
queue_state = queue.state
queue_task_ttl = queue.task_ttl
queue_tombstone_ttl = queue.tombstone_ttl
queue_name = queue.name
queue_retry_config = queue.retry_config
queue_purge_time = queue.purge_time
queue_stackdriver_logging_config = queue.stackdriver_logging_config
queue_http_target = queue.http_target
queue_stats = queue.stats
queue_type = queue.type
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple task resources
task_0 = provider.cloudtasks_api.Task {
    parent = "value-0"
}
task_1 = provider.cloudtasks_api.Task {
    parent = "value-1"
}
task_2 = provider.cloudtasks_api.Task {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    task = provider.cloudtasks_api.Task {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Cloudtasks_api Documentation](https://cloud.google.com/cloudtasks_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
