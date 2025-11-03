# Firebaseappdistribution_api Service



**Resources**: 16

---

## Overview

The firebaseappdistribution_api service provides access to 16 resource types:

- [Group](#group) [CRUD]
- [Feedback_report](#feedback_report) [RD]
- [Release](#release) [CRU]
- [Tester](#tester) [CRU]
- [App](#app) [R]
- [Operation](#operation) [CRD]
- [Media](#media) [C]
- [App](#app) [RU]
- [Release](#release) [C]
- [Tester](#tester) [R]
- [Test](#test) [CR]
- [Note](#note) [C]
- [Release_by_hash](#release_by_hash) [R]
- [Project](#project) [R]
- [Test_case](#test_case) [CRUD]
- [Upload_statu](#upload_statu) [R]

---

## Resources


### Group

Create a group.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `invite_link_count` | i64 |  | Output only. The number of invite links for this group. |
| `tester_count` | i64 |  | Output only. The number of testers who are members of this group. |
| `display_name` | String |  | Required. The display name of the group. |
| `name` | String |  | The name of the group resource. Format: `projects/{project_number}/groups/{group_alias}` |
| `release_count` | i64 |  | Output only. The number of releases this group is permitted to access. |
| `parent` | String | ✅ | Required. The name of the project resource, which is the parent of the group resource. Format: `projects/{project_number}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `invite_link_count` | i64 | Output only. The number of invite links for this group. |
| `tester_count` | i64 | Output only. The number of testers who are members of this group. |
| `display_name` | String | Required. The display name of the group. |
| `name` | String | The name of the group resource. Format: `projects/{project_number}/groups/{group_alias}` |
| `release_count` | i64 | Output only. The number of releases this group is permitted to access. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create group
group = provider.firebaseappdistribution_api.Group {
    parent = "value"  # Required. The name of the project resource, which is the parent of the group resource. Format: `projects/{project_number}`
}

# Access group outputs
group_id = group.id
group_invite_link_count = group.invite_link_count
group_tester_count = group.tester_count
group_display_name = group.display_name
group_name = group.name
group_release_count = group.release_count
```

---


### Feedback_report

Gets a feedback report.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the feedback report was created. |
| `text` | String | Output only. The text of the feedback report. |
| `firebase_console_uri` | String | Output only. A link to the Firebase console displaying the feedback report. |
| `name` | String | The name of the feedback report resource. Format: `projects/{project_number}/apps/{app}/releases/{release}/feedbackReports/{feedback_report}` |
| `tester` | String | Output only. The resource name of the tester who submitted the feedback report. |
| `screenshot_uri` | String | Output only. A signed link (which expires in one hour) that lets you directly download the screenshot. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access feedback_report outputs
feedback_report_id = feedback_report.id
feedback_report_create_time = feedback_report.create_time
feedback_report_text = feedback_report.text
feedback_report_firebase_console_uri = feedback_report.firebase_console_uri
feedback_report_name = feedback_report.name
feedback_report_tester = feedback_report.tester
feedback_report_screenshot_uri = feedback_report.screenshot_uri
```

---


### Release

Deletes releases. A maximum of 100 releases can be deleted per request.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `names` | Vec<String> |  | Required. The names of the release resources to delete. Format: `projects/{project_number}/apps/{app_id}/releases/{release_id}` A maximum of 100 releases can be deleted per request. |
| `parent` | String | ✅ | Required. The name of the app resource, which is the parent of the release resources. Format: `projects/{project_number}/apps/{app_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `build_version` | String | Output only. Build version of the release. For an Android release, the build version is the `versionCode`. For an iOS release, the build version is the `CFBundleVersion`. |
| `display_version` | String | Output only. Display version of the release. For an Android release, the display version is the `versionName`. For an iOS release, the display version is the `CFBundleShortVersionString`. |
| `expire_time` | String | Output only. The time the release will expire. |
| `name` | String | The name of the release resource. Format: `projects/{project_number}/apps/{app_id}/releases/{release_id}` |
| `testing_uri` | String | Output only. A link to the release in the tester web clip or Android app that lets testers (which were granted access to the app) view release notes and install the app onto their devices. |
| `create_time` | String | Output only. The time the release was created. |
| `update_time` | String | Output only. The time the release was last updated. |
| `binary_download_uri` | String | Output only. A signed link (which expires in one hour) to directly download the app binary (IPA/APK/AAB) file. |
| `firebase_console_uri` | String | Output only. A link to the Firebase console displaying a single release. |
| `release_notes` | String | Notes of the release. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create release
release = provider.firebaseappdistribution_api.Release {
    parent = "value"  # Required. The name of the app resource, which is the parent of the release resources. Format: `projects/{project_number}/apps/{app_id}`
}

# Access release outputs
release_id = release.id
release_build_version = release.build_version
release_display_version = release.display_version
release_expire_time = release.expire_time
release_name = release.name
release_testing_uri = release.testing_uri
release_create_time = release.create_time
release_update_time = release.update_time
release_binary_download_uri = release.binary_download_uri
release_firebase_console_uri = release.firebase_console_uri
release_release_notes = release.release_notes
```

---


### Tester

Batch adds testers. This call adds testers for the specified emails if they don't already exist. Returns all testers specified in the request, including newly created and previously existing testers. This action is idempotent.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `emails` | Vec<String> |  | Required. The email addresses of the tester resources to create. A maximum of 999 and a minimum of 1 tester can be created in a batch. |
| `project` | String | ✅ | Required. The name of the project resource. Format: `projects/{project_number}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A short-lived token, which can be sent as `pageToken` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `testers` | Vec<String> | The testers listed. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tester
tester = provider.firebaseappdistribution_api.Tester {
    project = "value"  # Required. The name of the project resource. Format: `projects/{project_number}`
}

# Access tester outputs
tester_id = tester.id
tester_next_page_token = tester.next_page_token
tester_testers = tester.testers
```

---


### App

Gets Android App Bundle (AAB) information for a Firebase app.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `test_certificate` | String | App bundle test certificate generated for the app. Set after the first app bundle is uploaded for this app. |
| `name` | String | The name of the `AabInfo` resource. Format: `projects/{project_number}/apps/{app}/aabInfo` |
| `integration_state` | String | App bundle integration state. Only valid for android apps. |


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
app_test_certificate = app.test_certificate
app_name = app.name
app_integration_state = app.integration_state
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.firebaseappdistribution_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_response = operation.response
operation_error = operation.error
operation_done = operation.done
operation_name = operation.name
```

---


### Media

Uploads a binary. Uploading a binary can result in a new release being created, an update to an existing release, or a no-op if a release with the same binary already exists.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `blob` | String |  | Binary to upload |
| `app` | String | ✅ | Required. The name of the app resource. Format: `projects/{project_number}/apps/{app_id}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create media
media = provider.firebaseappdistribution_api.Media {
    app = "value"  # Required. The name of the app resource. Format: `projects/{project_number}/apps/{app_id}`
}

```

---


### App

Get the app, if it exists

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `test_devices` | Vec<String> |  | Optional. Tests will be run on this list of devices |
| `name` | String |  | Identifier. The name of the test configuration resource. Format: `projects/{project_number}/apps/{app_id}/testConfig` |
| `display_name` | String |  | Optional. Display name of the AI driven test. Required if the release test is created with multiple goals. |
| `robo_crawler` | String |  | Optional. Configuration for Robo crawler |
| `name` | String | ✅ | Identifier. The name of the test configuration resource. Format: `projects/{project_number}/apps/{app_id}/testConfig` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `aab_state` | String | App bundle state. Only valid for android apps. The app_view field in the request must be set to FULL in order for this to be populated. |
| `bundle_id` | String | Bundle identifier |
| `platform` | String | iOS or Android |
| `contact_email` | String | Developer contact email for testers to reach out to about privacy or support issues. |
| `project_number` | String | Project number of the Firebase project, for example 300830567303. |
| `aab_certificate` | String | App bundle test certificate generated for the app. |
| `app_id` | String | Firebase gmp app id |


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
app_aab_state = app.aab_state
app_bundle_id = app.bundle_id
app_platform = app.platform
app_contact_email = app.contact_email
app_project_number = app.project_number
app_aab_certificate = app.aab_certificate
app_app_id = app.app_id
```

---


### Release

Enable access on a release for testers.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_version` | String |  | Optional. Ignored. Used to be display version of the app release if an instance identifier was provided for the release_id. |
| `emails` | Vec<String> |  | Optional. An email address which should get access to this release, for example rebeccahe@google.com |
| `build_version` | String |  | Optional. Ignored. Used to be build version of the app release if an instance identifier was provided for the release_id. |
| `group_ids` | Vec<String> |  | Optional. A repeated list of group aliases to enable access to a release for Note: This field is misnamed, but can't be changed because we need to maintain compatibility with old build tools |
| `release_id` | String | ✅ | Required. Release identifier |
| `mobilesdk_app_id` | String | ✅ | Required. Unique id for a Firebase app of the format: {version}:{project_number}:{platform}:{hash(bundle_id)} Example: 1:581234567376:android:aa0a3c7b135e90289 |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create release
release = provider.firebaseappdistribution_api.Release {
    release_id = "value"  # Required. Release identifier
    mobilesdk_app_id = "value"  # Required. Unique id for a Firebase app of the format: {version}:{project_number}:{platform}:{hash(bundle_id)} Example: 1:581234567376:android:aa0a3c7b135e90289
}

```

---


### Tester

Get UDIDs of tester iOS devices in a project

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tester_udids` | Vec<String> | The UDIDs of tester iOS devices in a project |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access tester outputs
tester_id = tester.id
tester_tester_udids = tester.tester_udids
```

---


### Test

Run automated test(s) on release.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Timestamp when the test was run. |
| `ai_instructions` | String |  | Optional. Instructions for AI driven test. |
| `display_name` | String |  | Optional. Display name of the release test. Required if the release test is created with multiple goals. |
| `login_credential` | String |  | Optional. Input only. Login credentials for the test. Input only. |
| `name` | String |  | The name of the release test resource. Format: `projects/{project_number}/apps/{app_id}/releases/{release_id}/tests/{test_id}` |
| `test_state` | String |  | Output only. The state of the release test. |
| `test_case` | String |  | Optional. The test case that was used to generate this release test. Note: The test case may have changed or been deleted since the release test was created. Format: `projects/{project_number}/apps/{app}/testCases/{test_case}` |
| `device_executions` | Vec<String> |  | Required. The results of the test on each device. |
| `parent` | String | ✅ | Required. The name of the release resource, which is the parent of the test Format: `projects/{project_number}/apps/{app_id}/releases/{release_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Timestamp when the test was run. |
| `ai_instructions` | String | Optional. Instructions for AI driven test. |
| `display_name` | String | Optional. Display name of the release test. Required if the release test is created with multiple goals. |
| `login_credential` | String | Optional. Input only. Login credentials for the test. Input only. |
| `name` | String | The name of the release test resource. Format: `projects/{project_number}/apps/{app_id}/releases/{release_id}/tests/{test_id}` |
| `test_state` | String | Output only. The state of the release test. |
| `test_case` | String | Optional. The test case that was used to generate this release test. Note: The test case may have changed or been deleted since the release test was created. Format: `projects/{project_number}/apps/{app}/testCases/{test_case}` |
| `device_executions` | Vec<String> | Required. The results of the test on each device. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create test
test = provider.firebaseappdistribution_api.Test {
    parent = "value"  # Required. The name of the release resource, which is the parent of the test Format: `projects/{project_number}/apps/{app_id}/releases/{release_id}`
}

# Access test outputs
test_id = test.id
test_create_time = test.create_time
test_ai_instructions = test.ai_instructions
test_display_name = test.display_name
test_login_credential = test.login_credential
test_name = test.name
test_test_state = test.test_state
test_test_case = test.test_case
test_device_executions = test.device_executions
```

---


### Note

Create release notes on a release.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `release_notes` | String |  | The actual release notes body from the user |
| `mobilesdk_app_id` | String | ✅ | Required. Unique id for a Firebase app of the format: {version}:{project_number}:{platform}:{hash(bundle_id)} Example: 1:581234567376:android:aa0a3c7b135e90289 |
| `release_id` | String | ✅ | Required. Release identifier |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create note
note = provider.firebaseappdistribution_api.Note {
    mobilesdk_app_id = "value"  # Required. Unique id for a Firebase app of the format: {version}:{project_number}:{platform}:{hash(bundle_id)} Example: 1:581234567376:android:aa0a3c7b135e90289
    release_id = "value"  # Required. Release identifier
}

```

---


### Release_by_hash

GET Release by binary upload hash

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `release` | String | Release object |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access release_by_hash outputs
release_by_hash_id = release_by_hash.id
release_by_hash_release = release_by_hash.release
```

---


### Project

Get information about the quota for `ReleaseTests`.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The name of the `TestQuota` resource. Format: `projects/{project_number}/testQuota` |
| `usage` | String | Output only. Number of `ReleaseTests` run in the current month |
| `limit` | String | Output only. Maximum number of `ReleaseTests` allotted for the current month. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access project outputs
project_id = project.id
project_name = project.name
project_usage = project.usage
project_limit = project.limit
```

---


### Test_case

Create a new test case.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dependent_test_cases` | Vec<String> |  | Output only. Other test cases that depend on this test case as a prerequisite. |
| `name` | String |  | Identifier. The name of the test case resource. Format: `projects/{project_number}/apps/{app_id}/testCases/{test_case_id}` |
| `prerequisite_test_case` | String |  | Optional. Test case that must be run before this test case. |
| `ai_instructions` | String |  | Optional. Instructions for AI driven test. |
| `create_time` | String |  | Output only. Timestamp when the test case was created |
| `display_name` | String |  | Required. Display name of the test case. |
| `parent` | String | ✅ | Required. The parent resource where this test case will be created. Format: `projects/{project_number}/apps/{app_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dependent_test_cases` | Vec<String> | Output only. Other test cases that depend on this test case as a prerequisite. |
| `name` | String | Identifier. The name of the test case resource. Format: `projects/{project_number}/apps/{app_id}/testCases/{test_case_id}` |
| `prerequisite_test_case` | String | Optional. Test case that must be run before this test case. |
| `ai_instructions` | String | Optional. Instructions for AI driven test. |
| `create_time` | String | Output only. Timestamp when the test case was created |
| `display_name` | String | Required. Display name of the test case. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create test_case
test_case = provider.firebaseappdistribution_api.Test_case {
    parent = "value"  # Required. The parent resource where this test case will be created. Format: `projects/{project_number}/apps/{app_id}`
}

# Access test_case outputs
test_case_id = test_case.id
test_case_dependent_test_cases = test_case.dependent_test_cases
test_case_name = test_case.name
test_case_prerequisite_test_case = test_case.prerequisite_test_case
test_case_ai_instructions = test_case.ai_instructions
test_case_create_time = test_case.create_time
test_case_display_name = test_case.display_name
```

---


### Upload_statu

GET Binary upload status by token

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `release` | String | The release that was created from the upload (only set on "SUCCESS") |
| `status` | String | The status of the upload |
| `error_code` | String | The error code associated with (only set on "FAILURE") |
| `message` | String | Any additional context for the given upload status (e.g. error message) Meant to be displayed to the client |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access upload_statu outputs
upload_statu_id = upload_statu.id
upload_statu_release = upload_statu.release
upload_statu_status = upload_statu.status
upload_statu_error_code = upload_statu.error_code
upload_statu_message = upload_statu.message
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple group resources
group_0 = provider.firebaseappdistribution_api.Group {
    parent = "value-0"
}
group_1 = provider.firebaseappdistribution_api.Group {
    parent = "value-1"
}
group_2 = provider.firebaseappdistribution_api.Group {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    group = provider.firebaseappdistribution_api.Group {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Firebaseappdistribution_api Documentation](https://cloud.google.com/firebaseappdistribution_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
