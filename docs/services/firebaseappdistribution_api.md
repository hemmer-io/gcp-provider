# Firebaseappdistribution_api Service



**Resources**: 16

---

## Overview

The firebaseappdistribution_api service provides access to 16 resource types:

- [Media](#media) [C]
- [Group](#group) [CRUD]
- [App](#app) [R]
- [Tester](#tester) [CRU]
- [Release](#release) [CRU]
- [Feedback_report](#feedback_report) [RD]
- [Operation](#operation) [CRD]
- [Release](#release) [C]
- [Tester](#tester) [R]
- [Test_case](#test_case) [CRUD]
- [Note](#note) [C]
- [Release_by_hash](#release_by_hash) [R]
- [Project](#project) [R]
- [Test](#test) [CR]
- [Upload_statu](#upload_statu) [R]
- [App](#app) [RU]

---

## Resources


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


### Group

Create a group.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `release_count` | i64 |  | Output only. The number of releases this group is permitted to access. |
| `display_name` | String |  | Required. The display name of the group. |
| `invite_link_count` | i64 |  | Output only. The number of invite links for this group. |
| `name` | String |  | The name of the group resource. Format: `projects/{project_number}/groups/{group_alias}` |
| `tester_count` | i64 |  | Output only. The number of testers who are members of this group. |
| `parent` | String | ✅ | Required. The name of the project resource, which is the parent of the group resource. Format: `projects/{project_number}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `release_count` | i64 | Output only. The number of releases this group is permitted to access. |
| `display_name` | String | Required. The display name of the group. |
| `invite_link_count` | i64 | Output only. The number of invite links for this group. |
| `name` | String | The name of the group resource. Format: `projects/{project_number}/groups/{group_alias}` |
| `tester_count` | i64 | Output only. The number of testers who are members of this group. |


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
group_release_count = group.release_count
group_display_name = group.display_name
group_invite_link_count = group.invite_link_count
group_name = group.name
group_tester_count = group.tester_count
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


### Tester

Batch removes testers. If found, this call deletes testers for the specified emails. Returns all deleted testers.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `emails` | Vec<String> |  | Required. The email addresses of the tester resources to removed. A maximum of 999 and a minimum of 1 testers can be deleted in a batch. |
| `project` | String | ✅ | Required. The name of the project resource. Format: `projects/{project_number}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `testers` | Vec<String> | The testers listed. |
| `next_page_token` | String | A short-lived token, which can be sent as `pageToken` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


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
tester_testers = tester.testers
tester_next_page_token = tester.next_page_token
```

---


### Release

Distributes a release to testers. This call does the following: 1. Creates testers for the specified emails, if none exist. 2. Adds the testers and groups to the release. 3. Sends new testers an invitation email. 4. Sends existing testers a new release email. The request will fail with a `INVALID_ARGUMENT` if it contains a group that doesn't exist.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group_aliases` | Vec<String> |  | Optional. A list of group aliases (IDs) to be given access to this release. A combined maximum of 999 `testerEmails` and `groupAliases` can be specified in a single request. |
| `tester_emails` | Vec<String> |  | Optional. A list of tester email addresses to be given access to this release. A combined maximum of 999 `testerEmails` and `groupAliases` can be specified in a single request. |
| `name` | String | ✅ | Required. The name of the release resource to distribute. Format: `projects/{project_number}/apps/{app_id}/releases/{release_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expire_time` | String | Output only. The time the release will expire. |
| `create_time` | String | Output only. The time the release was created. |
| `testing_uri` | String | Output only. A link to the release in the tester web clip or Android app that lets testers (which were granted access to the app) view release notes and install the app onto their devices. |
| `firebase_console_uri` | String | Output only. A link to the Firebase console displaying a single release. |
| `update_time` | String | Output only. The time the release was last updated. |
| `release_notes` | String | Notes of the release. |
| `build_version` | String | Output only. Build version of the release. For an Android release, the build version is the `versionCode`. For an iOS release, the build version is the `CFBundleVersion`. |
| `binary_download_uri` | String | Output only. A signed link (which expires in one hour) to directly download the app binary (IPA/APK/AAB) file. |
| `display_version` | String | Output only. Display version of the release. For an Android release, the display version is the `versionName`. For an iOS release, the display version is the `CFBundleShortVersionString`. |
| `name` | String | The name of the release resource. Format: `projects/{project_number}/apps/{app_id}/releases/{release_id}` |


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
    name = "value"  # Required. The name of the release resource to distribute. Format: `projects/{project_number}/apps/{app_id}/releases/{release_id}`
}

# Access release outputs
release_id = release.id
release_expire_time = release.expire_time
release_create_time = release.create_time
release_testing_uri = release.testing_uri
release_firebase_console_uri = release.firebase_console_uri
release_update_time = release.update_time
release_release_notes = release.release_notes
release_build_version = release.build_version
release_binary_download_uri = release.binary_download_uri
release_display_version = release.display_version
release_name = release.name
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
| `firebase_console_uri` | String | Output only. A link to the Firebase console displaying the feedback report. |
| `screenshot_uri` | String | Output only. A signed link (which expires in one hour) that lets you directly download the screenshot. |
| `create_time` | String | Output only. The time when the feedback report was created. |
| `tester` | String | Output only. The resource name of the tester who submitted the feedback report. |
| `text` | String | Output only. The text of the feedback report. |
| `name` | String | The name of the feedback report resource. Format: `projects/{project_number}/apps/{app}/releases/{release}/feedbackReports/{feedback_report}` |


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
feedback_report_firebase_console_uri = feedback_report.firebase_console_uri
feedback_report_screenshot_uri = feedback_report.screenshot_uri
feedback_report_create_time = feedback_report.create_time
feedback_report_tester = feedback_report.tester
feedback_report_text = feedback_report.text
feedback_report_name = feedback_report.name
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.firebaseappdistribution_api.Operation {
    name = "value"  # The name of the operation resource to wait on.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_response = operation.response
operation_name = operation.name
operation_done = operation.done
operation_metadata = operation.metadata
```

---


### Release

Enable access on a release for testers.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_version` | String |  | Optional. Ignored. Used to be display version of the app release if an instance identifier was provided for the release_id. |
| `build_version` | String |  | Optional. Ignored. Used to be build version of the app release if an instance identifier was provided for the release_id. |
| `group_ids` | Vec<String> |  | Optional. A repeated list of group aliases to enable access to a release for Note: This field is misnamed, but can't be changed because we need to maintain compatibility with old build tools |
| `emails` | Vec<String> |  | Optional. An email address which should get access to this release, for example rebeccahe@google.com |
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


### Test_case

Create a new test case.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The name of the test case resource. Format: `projects/{project_number}/apps/{app_id}/testCases/{test_case_id}` |
| `dependent_test_cases` | Vec<String> |  | Output only. Other test cases that depend on this test case as a prerequisite. |
| `ai_instructions` | String |  | Optional. Instructions for AI driven test. |
| `display_name` | String |  | Required. Display name of the test case. |
| `prerequisite_test_case` | String |  | Optional. Test case that must be run before this test case. |
| `create_time` | String |  | Output only. Timestamp when the test case was created |
| `parent` | String | ✅ | Required. The parent resource where this test case will be created. Format: `projects/{project_number}/apps/{app_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The name of the test case resource. Format: `projects/{project_number}/apps/{app_id}/testCases/{test_case_id}` |
| `dependent_test_cases` | Vec<String> | Output only. Other test cases that depend on this test case as a prerequisite. |
| `ai_instructions` | String | Optional. Instructions for AI driven test. |
| `display_name` | String | Required. Display name of the test case. |
| `prerequisite_test_case` | String | Optional. Test case that must be run before this test case. |
| `create_time` | String | Output only. Timestamp when the test case was created |


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
test_case_name = test_case.name
test_case_dependent_test_cases = test_case.dependent_test_cases
test_case_ai_instructions = test_case.ai_instructions
test_case_display_name = test_case.display_name
test_case_prerequisite_test_case = test_case.prerequisite_test_case
test_case_create_time = test_case.create_time
```

---


### Note

Create release notes on a release.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `release_notes` | String |  | The actual release notes body from the user |
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

# Create note
note = provider.firebaseappdistribution_api.Note {
    release_id = "value"  # Required. Release identifier
    mobilesdk_app_id = "value"  # Required. Unique id for a Firebase app of the format: {version}:{project_number}:{platform}:{hash(bundle_id)} Example: 1:581234567376:android:aa0a3c7b135e90289
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
| `usage` | String | Output only. Number of `ReleaseTests` run in the current month |
| `limit` | String | Output only. Maximum number of `ReleaseTests` allotted for the current month. |
| `name` | String | Identifier. The name of the `TestQuota` resource. Format: `projects/{project_number}/testQuota` |


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
project_usage = project.usage
project_limit = project.limit
project_name = project.name
```

---


### Test

Run automated test(s) on release.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `test_state` | String |  | Output only. The state of the release test. |
| `ai_instructions` | String |  | Optional. Instructions for AI driven test. |
| `create_time` | String |  | Output only. Timestamp when the test was run. |
| `device_executions` | Vec<String> |  | Required. The results of the test on each device. |
| `test_case` | String |  | Optional. The test case that was used to generate this release test. Note: The test case may have changed or been deleted since the release test was created. Format: `projects/{project_number}/apps/{app}/testCases/{test_case}` |
| `login_credential` | String |  | Optional. Input only. Login credentials for the test. Input only. |
| `display_name` | String |  | Optional. Display name of the release test. Required if the release test is created with multiple goals. |
| `name` | String |  | The name of the release test resource. Format: `projects/{project_number}/apps/{app_id}/releases/{release_id}/tests/{test_id}` |
| `parent` | String | ✅ | Required. The name of the release resource, which is the parent of the test Format: `projects/{project_number}/apps/{app_id}/releases/{release_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `test_state` | String | Output only. The state of the release test. |
| `ai_instructions` | String | Optional. Instructions for AI driven test. |
| `create_time` | String | Output only. Timestamp when the test was run. |
| `device_executions` | Vec<String> | Required. The results of the test on each device. |
| `test_case` | String | Optional. The test case that was used to generate this release test. Note: The test case may have changed or been deleted since the release test was created. Format: `projects/{project_number}/apps/{app}/testCases/{test_case}` |
| `login_credential` | String | Optional. Input only. Login credentials for the test. Input only. |
| `display_name` | String | Optional. Display name of the release test. Required if the release test is created with multiple goals. |
| `name` | String | The name of the release test resource. Format: `projects/{project_number}/apps/{app_id}/releases/{release_id}/tests/{test_id}` |


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
test_test_state = test.test_state
test_ai_instructions = test.ai_instructions
test_create_time = test.create_time
test_device_executions = test.device_executions
test_test_case = test.test_case
test_login_credential = test.login_credential
test_display_name = test.display_name
test_name = test.name
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
| `status` | String | The status of the upload |
| `message` | String | Any additional context for the given upload status (e.g. error message) Meant to be displayed to the client |
| `error_code` | String | The error code associated with (only set on "FAILURE") |
| `release` | String | The release that was created from the upload (only set on "SUCCESS") |


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
upload_statu_status = upload_statu.status
upload_statu_message = upload_statu.message
upload_statu_error_code = upload_statu.error_code
upload_statu_release = upload_statu.release
```

---


### App

Get the app, if it exists

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The name of the test configuration resource. Format: `projects/{project_number}/apps/{app_id}/testConfig` |
| `robo_crawler` | String |  | Optional. Configuration for Robo crawler |
| `test_devices` | Vec<String> |  | Optional. Tests will be run on this list of devices |
| `display_name` | String |  | Optional. Display name of the AI driven test. Required if the release test is created with multiple goals. |
| `name` | String | ✅ | Identifier. The name of the test configuration resource. Format: `projects/{project_number}/apps/{app_id}/testConfig` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bundle_id` | String | Bundle identifier |
| `platform` | String | iOS or Android |
| `app_id` | String | Firebase gmp app id |
| `aab_certificate` | String | App bundle test certificate generated for the app. |
| `aab_state` | String | App bundle state. Only valid for android apps. The app_view field in the request must be set to FULL in order for this to be populated. |
| `contact_email` | String | Developer contact email for testers to reach out to about privacy or support issues. |
| `project_number` | String | Project number of the Firebase project, for example 300830567303. |


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
app_bundle_id = app.bundle_id
app_platform = app.platform
app_app_id = app.app_id
app_aab_certificate = app.aab_certificate
app_aab_state = app.aab_state
app_contact_email = app.contact_email
app_project_number = app.project_number
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
media_0 = provider.firebaseappdistribution_api.Media {
    app = "value-0"
}
media_1 = provider.firebaseappdistribution_api.Media {
    app = "value-1"
}
media_2 = provider.firebaseappdistribution_api.Media {
    app = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    media = provider.firebaseappdistribution_api.Media {
        app = "production-value"
    }
```

---

## Related Documentation

- [GCP Firebaseappdistribution_api Documentation](https://cloud.google.com/firebaseappdistribution_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
