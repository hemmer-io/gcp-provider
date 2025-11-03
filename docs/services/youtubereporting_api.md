# Youtubereporting_api Service



**Resources**: 4

---

## Overview

The youtubereporting_api service provides access to 4 resource types:

- [Report](#report) [R]
- [Job](#job) [CRD]
- [Media](#media) [R]
- [Report_type](#report_type) [R]

---

## Resources


### Report

Gets the metadata of a specific report.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | The end of the time period that the report instance covers. The value is exclusive. |
| `create_time` | String | The date/time when this report was created. |
| `job_expire_time` | String | The date/time when the job this report belongs to will expire/expired. |
| `start_time` | String | The start of the time period that the report instance covers. The value is inclusive. |
| `job_id` | String | The ID of the job that created this report. |
| `download_url` | String | The URL from which the report can be downloaded (max. 1000 characters). |
| `id` | String | The server-generated ID of the report. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access report outputs
report_id = report.id
report_end_time = report.end_time
report_create_time = report.create_time
report_job_expire_time = report.job_expire_time
report_start_time = report.start_time
report_job_id = report.job_id
report_download_url = report.download_url
report_id = report.id
```

---


### Job

Creates a job and returns it.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | The creation date/time of the job. |
| `report_type_id` | String |  | The type of reports this job creates. Corresponds to the ID of a ReportType. |
| `system_managed` | bool |  | True if this a system-managed job that cannot be modified by the user; otherwise false. |
| `expire_time` | String |  | The date/time when this job will expire/expired. After a job expired, no new reports are generated. |
| `id` | String |  | The server-generated ID of the job (max. 40 characters). |
| `name` | String |  | The name of the job (max. 100 characters). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | The creation date/time of the job. |
| `report_type_id` | String | The type of reports this job creates. Corresponds to the ID of a ReportType. |
| `system_managed` | bool | True if this a system-managed job that cannot be modified by the user; otherwise false. |
| `expire_time` | String | The date/time when this job will expire/expired. After a job expired, no new reports are generated. |
| `id` | String | The server-generated ID of the job (max. 40 characters). |
| `name` | String | The name of the job (max. 100 characters). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.youtubereporting_api.Job {
}

# Access job outputs
job_id = job.id
job_create_time = job.create_time
job_report_type_id = job.report_type_id
job_system_managed = job.system_managed
job_expire_time = job.expire_time
job_id = job.id
job_name = job.name
```

---


### Media

Method for media download. Download is supported on the URI `/v1/media/{+name}?alt=media`.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `filename` | String | gdata |
| `diff_version_response` | String | gdata |
| `blob_ref` | String | gdata |
| `content_type_info` | String | gdata |
| `is_potential_retry` | bool | gdata |
| `object_id` | String | gdata |
| `reference_type` | String | gdata |
| `timestamp` | String | gdata |
| `token` | String | gdata |
| `sha1_hash` | String | gdata |
| `content_type` | String | gdata |
| `diff_checksums_response` | String | gdata |
| `sha256_hash` | String | gdata |
| `blobstore2_info` | String | gdata |
| `hash` | String | gdata |
| `composite_media` | Vec<String> | gdata |
| `md5_hash` | String | gdata |
| `crc32c_hash` | i64 | gdata |
| `length` | String | gdata |
| `diff_download_response` | String | gdata |
| `diff_upload_request` | String | gdata |
| `inline` | String | gdata |
| `path` | String | gdata |
| `diff_upload_response` | String | gdata |
| `download_parameters` | String | gdata |
| `media_id` | String | gdata |
| `hash_verified` | bool | gdata |
| `bigstore_object_ref` | String | gdata |
| `cosmo_binary_reference` | String | gdata |
| `algorithm` | String | gdata |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access media outputs
media_id = media.id
media_filename = media.filename
media_diff_version_response = media.diff_version_response
media_blob_ref = media.blob_ref
media_content_type_info = media.content_type_info
media_is_potential_retry = media.is_potential_retry
media_object_id = media.object_id
media_reference_type = media.reference_type
media_timestamp = media.timestamp
media_token = media.token
media_sha1_hash = media.sha1_hash
media_content_type = media.content_type
media_diff_checksums_response = media.diff_checksums_response
media_sha256_hash = media.sha256_hash
media_blobstore2_info = media.blobstore2_info
media_hash = media.hash
media_composite_media = media.composite_media
media_md5_hash = media.md5_hash
media_crc32c_hash = media.crc32c_hash
media_length = media.length
media_diff_download_response = media.diff_download_response
media_diff_upload_request = media.diff_upload_request
media_inline = media.inline
media_path = media.path
media_diff_upload_response = media.diff_upload_response
media_download_parameters = media.download_parameters
media_media_id = media.media_id
media_hash_verified = media.hash_verified
media_bigstore_object_ref = media.bigstore_object_ref
media_cosmo_binary_reference = media.cosmo_binary_reference
media_algorithm = media.algorithm
```

---


### Report_type

Lists report types.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `report_types` | Vec<String> | The list of report types. |
| `next_page_token` | String | A token to retrieve next page of results. Pass this value in the ListReportTypesRequest.page_token field in the subsequent call to `ListReportTypes` method to retrieve the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access report_type outputs
report_type_id = report_type.id
report_type_report_types = report_type.report_types
report_type_next_page_token = report_type.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple report resources
report_0 = provider.youtubereporting_api.Report {
}
report_1 = provider.youtubereporting_api.Report {
}
report_2 = provider.youtubereporting_api.Report {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    report = provider.youtubereporting_api.Report {
    }
```

---

## Related Documentation

- [GCP Youtubereporting_api Documentation](https://cloud.google.com/youtubereporting_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
