# Cloudtrace_api Service



**Resources**: 5

---

## Overview

The cloudtrace_api service provides access to 5 resource types:

- [Project](#project) [U]
- [Trace](#trace) [R]
- [Trace_sink](#trace_sink) [CRUD]
- [Trace](#trace) [C]
- [Span](#span) [C]

---

## Resources


### Project



**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `traces` | Vec<String> |  | List of traces. |
| `project_id` | String | ✅ | Required. ID of the Cloud project where the trace data is stored. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

```

---


### Trace

Gets a single trace by its ID.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `project_id` | String | Project ID of the Cloud project where the trace data is stored. |
| `spans` | Vec<String> | Collection of spans in the trace. |
| `trace_id` | String | Globally unique identifier for the trace. This identifier is a 128-bit numeric value formatted as a 32-byte hex string. For example, `382d4f4c6b7bb2f4a972559d9085001d`. The numeric value should not be zero. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access trace outputs
trace_id = trace.id
trace_project_id = trace.project_id
trace_spans = trace.spans
trace_trace_id = trace.trace_id
```

---


### Trace_sink

Creates a sink that exports trace spans to a destination. The export of newly-ingested traces begins immediately, unless the sink's `writer_identity` is not permitted to write to the destination. A sink can export traces only from the resource owning the sink (the 'parent').

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `writer_identity` | String |  | Output only. A service account name for exporting the data. This field is set by sinks.create and sinks.update. The service account will need to be granted write access to the destination specified in the output configuration, see [Granting access for a resource](/iam/docs/granting-roles-to-service-accounts#granting_access_to_a_service_account_for_a_resource). To create tables and to write data, this account needs the `dataEditor` role. Read more about roles in the [BigQuery documentation](https://cloud.google.com/bigquery/docs/access-control). E.g.: "service-00000001@00000002.iam.gserviceaccount.com" |
| `output_config` | String |  | Required. The export destination. |
| `name` | String |  | Identifier. The canonical sink resource name, unique within the project. Must be of the form: projects/[PROJECT_NUMBER]/traceSinks/[SINK_ID]. E.g.: `"projects/12345/traceSinks/my-project-trace-sink"`. Sink identifiers are limited to 256 characters and can include only the following characters: upper and lower-case alphanumeric characters, underscores, hyphens, and periods. |
| `parent` | String | ✅ | Required. The resource in which to create the sink (currently only project sinks are supported): "projects/[PROJECT_ID]" Examples: `"projects/my-trace-project"`, `"projects/123456789"`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `writer_identity` | String | Output only. A service account name for exporting the data. This field is set by sinks.create and sinks.update. The service account will need to be granted write access to the destination specified in the output configuration, see [Granting access for a resource](/iam/docs/granting-roles-to-service-accounts#granting_access_to_a_service_account_for_a_resource). To create tables and to write data, this account needs the `dataEditor` role. Read more about roles in the [BigQuery documentation](https://cloud.google.com/bigquery/docs/access-control). E.g.: "service-00000001@00000002.iam.gserviceaccount.com" |
| `output_config` | String | Required. The export destination. |
| `name` | String | Identifier. The canonical sink resource name, unique within the project. Must be of the form: projects/[PROJECT_NUMBER]/traceSinks/[SINK_ID]. E.g.: `"projects/12345/traceSinks/my-project-trace-sink"`. Sink identifiers are limited to 256 characters and can include only the following characters: upper and lower-case alphanumeric characters, underscores, hyphens, and periods. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create trace_sink
trace_sink = provider.cloudtrace_api.Trace_sink {
    parent = "value"  # Required. The resource in which to create the sink (currently only project sinks are supported): "projects/[PROJECT_ID]" Examples: `"projects/my-trace-project"`, `"projects/123456789"`.
}

# Access trace_sink outputs
trace_sink_id = trace_sink.id
trace_sink_writer_identity = trace_sink.writer_identity
trace_sink_output_config = trace_sink.output_config
trace_sink_name = trace_sink.name
```

---


### Trace

Batch writes new spans to new or existing traces. You cannot update existing spans. If a span ID already exists, an additional copy of the span will be stored.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `spans` | Vec<String> |  | Required. A list of new spans. The span names must not match existing spans, otherwise the results are undefined. |
| `name` | String | ✅ | Required. The name of the project where the spans belong. The format is `projects/[PROJECT_ID]`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create trace
trace = provider.cloudtrace_api.Trace {
    name = "value"  # Required. The name of the project where the spans belong. The format is `projects/[PROJECT_ID]`.
}

```

---


### Span

Creates a new span. If a span ID already exists, an additional copy of the span will be stored.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `same_process_as_parent_span` | bool |  | Optional. Set this parameter to indicate whether this span is in the same process as its parent. If you do not set this parameter, Trace is unable to take advantage of this helpful information. |
| `parent_span_id` | String |  | The `[SPAN_ID]` of this span's parent span. If this is a root span, then this field must be empty. |
| `name` | String |  | Required. The resource name of the span in the following format: * `projects/[PROJECT_ID]/traces/[TRACE_ID]/spans/[SPAN_ID]` `[TRACE_ID]` is a unique identifier for a trace within a project; it is a 32-character hexadecimal encoding of a 16-byte array. It should not be zero. `[SPAN_ID]` is a unique identifier for a span within a trace; it is a 16-character hexadecimal encoding of an 8-byte array. It should not be zero. . |
| `display_name` | String |  | Required. A description of the span's operation (up to 128 bytes). Cloud Trace displays the description in the Cloud console. For example, the display name can be a qualified method name or a file name and a line number where the operation is called. A best practice is to use the same display name within an application and at the same call point. This makes it easier to correlate spans in different traces. |
| `attributes` | String |  | A set of attributes on the span. You can have up to 32 attributes per span. |
| `end_time` | String |  | Required. The end time of the span. On the client side, this is the time kept by the local machine where the span execution ends. On the server side, this is the time when the server application handler stops running. |
| `span_kind` | String |  | Optional. Distinguishes between spans generated in a particular context. For example, two spans with the same name may be distinguished using `CLIENT` (caller) and `SERVER` (callee) to identify an RPC call. |
| `stack_trace` | String |  | Stack trace captured at the start of the span. |
| `child_span_count` | i64 |  | Optional. The number of child spans that were generated while this span was active. If set, allows implementation to detect missing child spans. |
| `links` | String |  | Links associated with the span. You can have up to 128 links per Span. |
| `start_time` | String |  | Required. The start time of the span. On the client side, this is the time kept by the local machine where the span execution starts. On the server side, this is the time when the server's application handler starts running. |
| `status` | String |  | Optional. The final status for this span. |
| `span_id` | String |  | Required. The `[SPAN_ID]` portion of the span's resource name. |
| `time_events` | String |  | A set of time events. You can have up to 32 annotations and 128 message events per span. |
| `name` | String | ✅ | Required. The resource name of the span in the following format: * `projects/[PROJECT_ID]/traces/[TRACE_ID]/spans/[SPAN_ID]` `[TRACE_ID]` is a unique identifier for a trace within a project; it is a 32-character hexadecimal encoding of a 16-byte array. It should not be zero. `[SPAN_ID]` is a unique identifier for a span within a trace; it is a 16-character hexadecimal encoding of an 8-byte array. It should not be zero. . |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create span
span = provider.cloudtrace_api.Span {
    name = "value"  # Required. The resource name of the span in the following format: * `projects/[PROJECT_ID]/traces/[TRACE_ID]/spans/[SPAN_ID]` `[TRACE_ID]` is a unique identifier for a trace within a project; it is a 32-character hexadecimal encoding of a 16-byte array. It should not be zero. `[SPAN_ID]` is a unique identifier for a span within a trace; it is a 16-character hexadecimal encoding of an 8-byte array. It should not be zero. .
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

# Create multiple project resources
project_0 = provider.cloudtrace_api.Project {
    project_id = "value-0"
}
project_1 = provider.cloudtrace_api.Project {
    project_id = "value-1"
}
project_2 = provider.cloudtrace_api.Project {
    project_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    project = provider.cloudtrace_api.Project {
        project_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Cloudtrace_api Documentation](https://cloud.google.com/cloudtrace_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
