# Checks_api Service



**Resources**: 6

---

## Overview

The checks_api service provides access to 6 resource types:

- [Media](#media) [C]
- [Report](#report) [R]
- [Scan](#scan) [CR]
- [App](#app) [R]
- [Operation](#operation) [CRD]
- [Aisafety](#aisafety) [C]

---

## Resources


### Media

Analyzes the uploaded app bundle and returns a google.longrunning.Operation containing the generated Report. ## Example (upload only) Send a regular POST request with the header `X-Goog-Upload-Protocol: raw`. ``` POST https://checks.googleapis.com/upload/v1alpha/{parent=accounts/*/apps/*}/reports:analyzeUpload HTTP/1.1 X-Goog-Upload-Protocol: raw Content-Length: Content-Type: application/octet-stream ``` ## Example (upload with metadata) Send a multipart POST request where the first body part contains the metadata JSON and the second body part contains the binary upload. Include the header `X-Goog-Upload-Protocol: multipart`. ``` POST https://checks.googleapis.com/upload/v1alpha/{parent=accounts/*/apps/*}/reports:analyzeUpload HTTP/1.1 X-Goog-Upload-Protocol: multipart Content-Length: ? Content-Type: multipart/related; boundary=BOUNDARY --BOUNDARY Content-Type: application/json {"code_reference_id":"db5bcc20f94055fb5bc08cbb9b0e7a5530308786"} --BOUNDARY --BOUNDARY-- ``` *Note:* Metadata-only requests are not supported. 

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `code_reference_id` | String |  | Optional. Git commit hash or changelist number associated with the upload. |
| `app_binary_file_type` | String |  | Optional. The type of the uploaded app binary. If not provided, the server assumes APK file for Android and IPA file for iOS. |
| `parent` | String | ✅ | Required. Resource name of the app. Example: `accounts/123/apps/456` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create media
media = provider.checks_api.Media {
    parent = "value"  # Required. Resource name of the app. Example: `accounts/123/apps/456`
}

```

---


### Report

Gets a report. By default, only the name and results_uri fields are returned. You can include other fields by listing them in the `fields` URL query parameter. For example, `?fields=name,checks` will return the name and checks fields.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name of the report. |
| `results_uri` | String | A URL to view results. |
| `app_bundle` | String | Information about the analyzed app bundle. |
| `data_monitoring` | String | Information related to data monitoring. |
| `checks` | Vec<String> | List of checks that were run on the app bundle. |


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
report_name = report.name
report_results_uri = report.results_uri
report_app_bundle = report.app_bundle
report_data_monitoring = report.data_monitoring
report_checks = report.checks
```

---


### Scan

Uploads the results of local Code Compliance analysis and generates a scan of privacy issues. Returns a google.longrunning.Operation containing analysis and findings.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scm_metadata` | String |  | Required. SCM metadata. |
| `local_scan_path` | String |  | Required. Local scan path. |
| `cli_version` | String |  | Required. CLI version. |
| `cli_analysis` | String |  | Required. CLI analysis results. |
| `parent` | String | ✅ | Required. Resource name of the repo. Example: `accounts/123/repos/456` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Resource name of the scan. |
| `scm_metadata` | String | SCM metadata. |
| `sources` | Vec<String> | Data sources detected. |
| `cli_version` | String | CLI version. |
| `results_uri` | String | A URL to view results. |
| `local_scan_path` | String | Local scan path. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create scan
scan = provider.checks_api.Scan {
    parent = "value"  # Required. Resource name of the repo. Example: `accounts/123/repos/456`
}

# Access scan outputs
scan_id = scan.id
scan_name = scan.name
scan_scm_metadata = scan.scm_metadata
scan_sources = scan.sources
scan_cli_version = scan.cli_version
scan_results_uri = scan.results_uri
scan_local_scan_path = scan.local_scan_path
```

---


### App

Gets an app.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `title` | String | The app's title. |
| `name` | String | The resource name of the app. Example: `accounts/123/apps/456` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access app outputs
app_id = app.id
app_title = app.title
app_name = app.name
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.checks_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_metadata = operation.metadata
operation_done = operation.done
operation_error = operation.error
operation_name = operation.name
```

---


### Aisafety

Analyze a piece of content with the provided set of policies.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `context` | String |  | Optional. Context about the input that will be used to help on the classification. |
| `classifier_version` | String |  | Optional. Version of the classifier to use. If not specified, the latest version will be used. |
| `input` | String |  | Required. Content to be classified. |
| `policies` | Vec<String> |  | Required. List of policies to classify against. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create aisafety
aisafety = provider.checks_api.Aisafety {
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

# Create multiple media resources
media_0 = provider.checks_api.Media {
    parent = "value-0"
}
media_1 = provider.checks_api.Media {
    parent = "value-1"
}
media_2 = provider.checks_api.Media {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    media = provider.checks_api.Media {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Checks_api Documentation](https://cloud.google.com/checks_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
