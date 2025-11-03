# Tracing_api Service



**Resources**: 2

---

## Overview

The tracing_api service provides access to 2 resource types:

- [Span](#span) [C]
- [Trace](#trace) [CR]

---

## Resources


### Span

Creates a new Span.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `links` | String |  | A maximum of 128 links are allowed per Span. |
| `start_time` | String |  | Start time of the span.
On the client side, this is the local machine clock time at which the span
execution was started; on the server
side, this is the time at which the server application handler started
running. |
| `span_id` | String |  | Unique identifier for a span within a trace. It is a base 16-encoded,
case-insensitive string of a 8-bytes array and is required. |
| `name` | String |  | The resource name of Span in the format
`projects/PROJECT_ID/traces/TRACE_ID/spans/SPAN_ID`.
`TRACE_ID` is a unique identifier for a trace within a project and is a
base16-encoded, case-insensitive string and is required to be 32 char long.
`SPAN_ID` is a unique identifier for a span within a trace. It is a
base 16-encoded, case-insensitive string of a 8-bytes array and is required
to be 16 char long. |
| `parent_span_id` | String |  | ID of parent span which is a base 16-encoded, case-insensitive string of
a 8-bytes array and is required to be 16 char long. If this is a root span,
the value must be empty. |
| `time_events` | String |  | A maximum of 32 annotations and 128 network events are allowed per Span. |
| `status` | String |  | An optional final status for this span. |
| `stack_trace` | String |  | Stack trace captured at the start of the span. |
| `attributes` | String |  | A set of attributes on the span. A maximum of 32 attributes are allowed per
Span. |
| `end_time` | String |  | End time of the span.
On the client side, this is the local machine clock time at which the span
execution was ended; on the server
side, this is the time at which the server application handler stopped
running. |
| `display_name` | String |  | Description of the operation in the span. It is sanitized and displayed in
the Stackdriver Trace tool in the
{% dynamic print site_values.console_name %}.
The display_name may be a method name or some other per-call site
name. For the same executable and the same call point, a best practice is
to use a consistent operation name, which makes it easier to correlate
cross-trace spans.
The maximum length for the display_name is 128 bytes. |
| `name` | String | ✅ | The resource name of Span in the format
`projects/PROJECT_ID/traces/TRACE_ID/spans/SPAN_ID`.
`TRACE_ID` is a unique identifier for a trace within a project and is a
base16-encoded, case-insensitive string and is required to be 32 char long.
`SPAN_ID` is a unique identifier for a span within a trace. It is a
base 16-encoded, case-insensitive string of a 8-bytes array and is required
to be 16 char long. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create span
span = provider.tracing_api.Span {
    name = "value"  # The resource name of Span in the format
`projects/PROJECT_ID/traces/TRACE_ID/spans/SPAN_ID`.
`TRACE_ID` is a unique identifier for a trace within a project and is a
base16-encoded, case-insensitive string and is required to be 32 char long.
`SPAN_ID` is a unique identifier for a span within a trace. It is a
base 16-encoded, case-insensitive string of a 8-bytes array and is required
to be 16 char long.
}

```

---


### Trace

Sends new spans to Stackdriver Trace or updates existing traces. If the
name of a trace that you send matches that of an existing trace, new spans
are added to the existing trace. Attempt to update existing spans results
undefined behavior. If the name does not match, a new trace is created
with given set of spans.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `spans` | Vec<String> |  | A collection of spans. |
| `name` | String | ✅ | Name of the project where the spans belong to. Format is
`projects/PROJECT_ID`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `traces` | Vec<String> | List of trace records returned. |
| `next_page_token` | String | If defined, indicates that there are more traces that match the request
and that this value should be passed to the next request to continue
retrieving additional traces. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create trace
trace = provider.tracing_api.Trace {
    name = "value"  # Name of the project where the spans belong to. Format is
`projects/PROJECT_ID`.
}

# Access trace outputs
trace_id = trace.id
trace_traces = trace.traces
trace_next_page_token = trace.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple span resources
span_0 = provider.tracing_api.Span {
    name = "value-0"
}
span_1 = provider.tracing_api.Span {
    name = "value-1"
}
span_2 = provider.tracing_api.Span {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    span = provider.tracing_api.Span {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Tracing_api Documentation](https://cloud.google.com/tracing_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
