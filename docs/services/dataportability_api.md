# Dataportability_api Service



**Resources**: 8

---

## Overview

The dataportability_api service provides access to 8 resource types:

- [Authorization](#authorization) [C]
- [Archive_job](#archive_job) [CR]
- [Portability_archive](#portability_archive) [C]
- [Access_type](#access_type) [C]
- [Archive_job](#archive_job) [CR]
- [Portability_archive](#portability_archive) [C]
- [Access_type](#access_type) [C]
- [Authorization](#authorization) [C]

---

## Resources


### Authorization

Revokes OAuth tokens and resets exhausted scopes for a user/project pair. This method allows you to initiate a request after a new consent is granted. This method also indicates that previous archives can be garbage collected. You should call this method when all jobs are complete and all archives are downloaded. Do not call it only when you start a new job.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create authorization
authorization = provider.dataportability_api.Authorization {
}

```

---


### Archive_job

Retries a failed Portability Archive job.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The Archive job ID you're retrying. This is returned by the InitiatePortabilityArchiveResponse. Retrying is only executed if the initial job failed. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | The timestamp that represents the starting point for the data you are exporting. This field is set only if the start_time field is specified in the InitiatePortabilityArchiveRequest. |
| `state` | String | Resource that represents the state of the Archive job. |
| `urls` | Vec<String> | If the state is complete, this method returns the signed URLs of the objects in the Cloud Storage bucket. |
| `name` | String | The resource name of ArchiveJob's PortabilityArchiveState singleton. The format is: archiveJobs/{archive_job}/portabilityArchiveState. archive_job is the job ID provided in the request. |
| `export_time` | String | The timestamp that represents the end point for the data you are exporting. If the end_time value is set in the InitiatePortabilityArchiveRequest, this field is set to that value. If end_time is not set, this value is set to the time the export was requested. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create archive_job
archive_job = provider.dataportability_api.Archive_job {
    name = "value"  # Required. The Archive job ID you're retrying. This is returned by the InitiatePortabilityArchiveResponse. Retrying is only executed if the initial job failed.
}

# Access archive_job outputs
archive_job_id = archive_job.id
archive_job_start_time = archive_job.start_time
archive_job_state = archive_job.state
archive_job_urls = archive_job.urls
archive_job_name = archive_job.name
archive_job_export_time = archive_job.export_time
```

---


### Portability_archive

Initiates a new Archive job for the Portability API.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resources` | Vec<String> |  | The resources from which you're exporting data. These values have a 1:1 correspondence with the OAuth scopes. |
| `end_time` | String |  | Optional. The timestamp that represents the end point for the data you are exporting. If the end_time is not specified in the InitiatePortabilityArchiveRequest, this field is set to the latest available data. |
| `start_time` | String |  | Optional. The timestamp that represents the starting point for the data you are exporting. If the start_time is not specified in the InitiatePortabilityArchiveRequest, the field is set to the earliest available data. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create portability_archive
portability_archive = provider.dataportability_api.Portability_archive {
}

```

---


### Access_type

Gets the access type of the token.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create access_type
access_type = provider.dataportability_api.Access_type {
}

```

---


### Archive_job

Retries a failed Portability Archive job.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The Archive job ID you're retrying. This is returned by the InitiatePortabilityArchiveResponse. Retrying is only executed if the initial job failed. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `export_time` | String | The timestamp that represents the end point for the data you are exporting. If the end_time value is set in the InitiatePortabilityArchiveRequest, this field is set to that value. If end_time is not set, this value is set to the time the export was requested. |
| `start_time` | String | The timestamp that represents the starting point for the data you are exporting. This field is set only if the start_time field is specified in the InitiatePortabilityArchiveRequest. |
| `state` | String | Resource that represents the state of the Archive job. |
| `urls` | Vec<String> | If the state is complete, this method returns the signed URLs of the objects in the Cloud Storage bucket. |
| `name` | String | The resource name of ArchiveJob's PortabilityArchiveState singleton. The format is: archiveJobs/{archive_job}/portabilityArchiveState. archive_job is the job ID provided in the request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create archive_job
archive_job = provider.dataportability_api.Archive_job {
    name = "value"  # Required. The Archive job ID you're retrying. This is returned by the InitiatePortabilityArchiveResponse. Retrying is only executed if the initial job failed.
}

# Access archive_job outputs
archive_job_id = archive_job.id
archive_job_export_time = archive_job.export_time
archive_job_start_time = archive_job.start_time
archive_job_state = archive_job.state
archive_job_urls = archive_job.urls
archive_job_name = archive_job.name
```

---


### Portability_archive

Initiates a new Archive job for the Portability API.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `start_time` | String |  | Optional. The timestamp that represents the starting point for the data you are exporting. If the start_time is not specified in the InitiatePortabilityArchiveRequest, the field is set to the earliest available data. |
| `resources` | Vec<String> |  | The resources from which you're exporting data. These values have a 1:1 correspondence with the OAuth scopes. |
| `end_time` | String |  | Optional. The timestamp that represents the end point for the data you are exporting. If the end_time is not specified in the InitiatePortabilityArchiveRequest, this field is set to the latest available data. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create portability_archive
portability_archive = provider.dataportability_api.Portability_archive {
}

```

---


### Access_type

Gets the access type of the token.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create access_type
access_type = provider.dataportability_api.Access_type {
}

```

---


### Authorization

Revokes OAuth tokens and resets exhausted scopes for a user/project pair. This method allows you to initiate a request after a new consent is granted. This method also indicates that previous archives can be garbage collected. You should call this method when all jobs are complete and all archives are downloaded. Do not call it only when you start a new job.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create authorization
authorization = provider.dataportability_api.Authorization {
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

# Create multiple authorization resources
authorization_0 = provider.dataportability_api.Authorization {
}
authorization_1 = provider.dataportability_api.Authorization {
}
authorization_2 = provider.dataportability_api.Authorization {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    authorization = provider.dataportability_api.Authorization {
    }
```

---

## Related Documentation

- [GCP Dataportability_api Documentation](https://cloud.google.com/dataportability_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
