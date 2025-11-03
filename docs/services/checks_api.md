# Checks_api Service



**Resources**: 6

---

## Overview

The checks_api service provides access to 6 resource types:

- [Aisafety](#aisafety) [C]
- [App](#app) [R]
- [Media](#media) [C]
- [Scan](#scan) [CR]
- [Operation](#operation) [CRD]
- [Report](#report) [R]

---

## Resources


### Aisafety

Analyze a piece of content with the provided set of policies.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `classifier_version` | String |  | Optional. Version of the classifier to use. If not specified, the latest version will be used. |
| `input` | String |  | Required. Content to be classified. |
| `policies` | Vec<String> |  | Required. List of policies to classify against. |
| `context` | String |  | Optional. Context about the input that will be used to help on the classification. |



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


### App

Gets an app.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name of the app. Example: `accounts/123/apps/456` |
| `title` | String | The app's title. |


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
app_name = app.name
app_title = app.title
```

---


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


### Scan

Uploads the results of local Code Compliance analysis and generates a scan of privacy issues. Returns a google.longrunning.Operation containing analysis and findings.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cli_version` | String |  | Required. CLI version. |
| `local_scan_path` | String |  | Required. Local scan path. |
| `cli_analysis` | String |  | Required. CLI analysis results. |
| `scm_metadata` | String |  | Required. SCM metadata. |
| `parent` | String | ✅ | Required. Resource name of the repo. Example: `accounts/123/repos/456` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scm_metadata` | String | SCM metadata. |
| `sources` | Vec<String> | Data sources detected. |
| `cli_version` | String | CLI version. |
| `results_uri` | String | A URL to view results. |
| `local_scan_path` | String | Local scan path. |
| `name` | String | Identifier. Resource name of the scan. |


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
scan_scm_metadata = scan.scm_metadata
scan_sources = scan.sources
scan_cli_version = scan.cli_version
scan_results_uri = scan.results_uri
scan_local_scan_path = scan.local_scan_path
scan_name = scan.name
```

---


### Operation

Waits until the specified long-running operation is done or reaches at most a specified timeout, returning the latest state. If the operation is already done, the latest state is immediately returned. If the timeout specified is greater than the default HTTP/RPC timeout, the HTTP/RPC timeout is used. If the server does not support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Note that this method is on a best-effort basis. It may return the latest state before the specified timeout (including immediately), meaning even an immediate response is no guarantee that the operation is done.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `timeout` | String |  | The maximum duration to wait before timing out. If left blank, the wait will be at most the time permitted by the underlying HTTP/RPC protocol. If RPC context deadline is also specified, the shorter one will be used. |
| `name` | String | ✅ | The name of the operation resource to wait on. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |


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
    name = "value"  # The name of the operation resource to wait on.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_error = operation.error
operation_done = operation.done
operation_name = operation.name
operation_metadata = operation.metadata
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
| `app_bundle` | String | Information about the analyzed app bundle. |
| `checks` | Vec<String> | List of checks that were run on the app bundle. |
| `name` | String | Resource name of the report. |
| `results_uri` | String | A URL to view results. |
| `data_monitoring` | String | Information related to data monitoring. |


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
report_app_bundle = report.app_bundle
report_checks = report.checks
report_name = report.name
report_results_uri = report.results_uri
report_data_monitoring = report.data_monitoring
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple aisafety resources
aisafety_0 = provider.checks_api.Aisafety {
}
aisafety_1 = provider.checks_api.Aisafety {
}
aisafety_2 = provider.checks_api.Aisafety {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    aisafety = provider.checks_api.Aisafety {
    }
```

---

## Related Documentation

- [GCP Checks_api Documentation](https://cloud.google.com/checks_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
