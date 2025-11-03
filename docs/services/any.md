# Any Service



**Resources**: 5

---

## Overview

The any service provides access to 5 resource types:

- [Log](#log) [RD]
- [Entrie](#entrie) [C]
- [Log_service](#log_service) [R]
- [Indexe](#indexe) [R]
- [Sink](#sink) [CRUD]

---

## Resources


### Log

Lists log resources belonging to the specified project.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `logs` | Vec<String> | A list of log resources. |
| `next_page_token` | String | If there are more results, then `nextPageToken` is returned in the response. To get the next batch of logs, use the value of `nextPageToken` as `pageToken` in the next call of `ListLogs`. If `nextPageToken` is empty, then there are no more results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access log outputs
log_id = log.id
log_logs = log.logs
log_next_page_token = log.next_page_token
```

---


### Entrie

Creates one or more log entries in a log. You must supply a list of `LogEntry` objects, named `entries`. Each `LogEntry` object must contain a payload object and a `LogEntryMetadata` object that describes the entry. You must fill in all the fields of the entry, metadata, and payload. You can also supply a map, `commonLabels`, that supplies default (key, value) data for the `entries[].metadata.labels` maps, saving you the trouble of creating identical copies for each entry.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `common_labels` | HashMap<String, String> |  | Metadata labels that apply to all entries in this request. If one of the log entries contains a (key, value) with the same key that is in `commonLabels`, then the entry's (key, value) overrides the one in `commonLabels`. |
| `entries` | Vec<String> |  | Log entries to insert. |
| `logs_id` | String | ✅ | Part of `logName`. See documentation of `projectsId`. |
| `projects_id` | String | ✅ | Part of `logName`. The name of the log resource into which to insert the log entries. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entrie
entrie = provider.any.Entrie {
    logs_id = "value"  # Part of `logName`. See documentation of `projectsId`.
    projects_id = "value"  # Part of `logName`. The name of the log resource into which to insert the log entries.
}

```

---


### Log_service

Lists log services associated with log entries ingested for a project.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `log_services` | Vec<String> | A list of log services. |
| `next_page_token` | String | If there are more results, then `nextPageToken` is returned in the response. To get the next batch of services, use the value of `nextPageToken` as `pageToken` in the next call of `ListLogServices`. If `nextPageToken` is empty, then there are no more results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access log_service outputs
log_service_id = log_service.id
log_service_log_services = log_service.log_services
log_service_next_page_token = log_service.next_page_token
```

---


### Indexe

Lists log service indexes associated with a log service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_index_prefixes` | Vec<String> | A list of log service index prefixes. |
| `next_page_token` | String | If there are more results, then `nextPageToken` is returned in the response. To get the next batch of indexes, use the value of `nextPageToken` as `pageToken` in the next call of `ListLogServiceIndexess`. If `nextPageToken` is empty, then there are no more results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access indexe outputs
indexe_id = indexe.id
indexe_service_index_prefixes = indexe.service_index_prefixes
indexe_next_page_token = indexe.next_page_token
```

---


### Sink

Creates the specified log service sink resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `errors` | Vec<String> |  | _Output only._ All active errors found for this sink. |
| `destination` | String |  | The resource to send log entries to. The supported sink resource types are: + Google Cloud Storage: `storage.googleapis.com/BUCKET` or `BUCKET.storage.googleapis.com/` + Google BigQuery: `bigquery.googleapis.com/projects/PROJECT/datasets/DATASET` Currently the Cloud Logging API supports at most one sink for each resource type per log or log service resource. |
| `name` | String |  | The name of this sink. This is a client-assigned identifier for the resource. This is ignored by UpdateLogSink and UpdateLogServicesSink. |
| `log_services_id` | String | ✅ | Part of `serviceName`. See documentation of `projectsId`. |
| `projects_id` | String | ✅ | Part of `serviceName`. The name of the service in which to create a sink. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `errors` | Vec<String> | _Output only._ All active errors found for this sink. |
| `destination` | String | The resource to send log entries to. The supported sink resource types are: + Google Cloud Storage: `storage.googleapis.com/BUCKET` or `BUCKET.storage.googleapis.com/` + Google BigQuery: `bigquery.googleapis.com/projects/PROJECT/datasets/DATASET` Currently the Cloud Logging API supports at most one sink for each resource type per log or log service resource. |
| `name` | String | The name of this sink. This is a client-assigned identifier for the resource. This is ignored by UpdateLogSink and UpdateLogServicesSink. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sink
sink = provider.any.Sink {
    log_services_id = "value"  # Part of `serviceName`. See documentation of `projectsId`.
    projects_id = "value"  # Part of `serviceName`. The name of the service in which to create a sink.
}

# Access sink outputs
sink_id = sink.id
sink_errors = sink.errors
sink_destination = sink.destination
sink_name = sink.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple log resources
log_0 = provider.any.Log {
}
log_1 = provider.any.Log {
}
log_2 = provider.any.Log {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    log = provider.any.Log {
    }
```

---

## Related Documentation

- [GCP Any Documentation](https://cloud.google.com/any/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
