# Testing_api Service



**Resources**: 4

---

## Overview

The testing_api service provides access to 4 resource types:

- [Test_matrice](#test_matrice) [CR]
- [Device_session](#device_session) [CRU]
- [Test_environment_catalog](#test_environment_catalog) [R]
- [Application_detail_service](#application_detail_service) [C]

---

## Resources


### Test_matrice

Creates and runs a matrix of tests according to the given specifications. Unsupported environments will be returned in the state UNSUPPORTED. A test matrix is limited to use at most 2000 devices in parallel. The returned matrix will not yet contain the executions that will be created for this matrix. Execution creation happens later on and will require a call to GetTestMatrix. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed or if the matrix tries to use too many simultaneous devices.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fail_fast` | bool |  | If true, only a single attempt at most will be made to run each execution/shard in the matrix. Flaky test attempts are not affected. Normally, 2 or more attempts are made if a potential infrastructure issue is detected. This feature is for latency sensitive workloads. The incidence of execution failures may be significantly greater for fail-fast matrices and support is more limited because of that expectation. |
| `flaky_test_attempts` | i64 |  | The number of times a TestExecution should be re-attempted if one or more of its test cases fail for any reason. The maximum number of reruns allowed is 10. Default is 0, which implies no reruns. |
| `test_executions` | Vec<String> |  | Output only. The list of test executions that the service creates for this matrix. |
| `outcome_summary` | String |  | Output Only. The overall outcome of the test. Only set when the test matrix state is FINISHED. |
| `client_info` | String |  | Information about the client which invoked the test. |
| `extended_invalid_matrix_details` | Vec<String> |  | Output only. Details about why a matrix was deemed invalid. If multiple checks can be safely performed, they will be reported but no assumptions should be made about the length of this list. |
| `state` | String |  | Output only. Indicates the current progress of the test matrix. |
| `invalid_matrix_details` | String |  | Output only. Describes why the matrix is considered invalid. Only useful for matrices in the INVALID state. |
| `test_specification` | String |  | Required. How to run the test. |
| `timestamp` | String |  | Output only. The time this test matrix was initially created. |
| `test_matrix_id` | String |  | Output only. Unique id set by the service. |
| `environment_matrix` | String |  | Required. The devices the tests are being executed on. |
| `result_storage` | String |  | Required. Where the results for the matrix are written. |
| `project_id` | String |  | The cloud project that owns the test matrix. |
| `project_id` | String | ✅ | The GCE project under which this job will run. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fail_fast` | bool | If true, only a single attempt at most will be made to run each execution/shard in the matrix. Flaky test attempts are not affected. Normally, 2 or more attempts are made if a potential infrastructure issue is detected. This feature is for latency sensitive workloads. The incidence of execution failures may be significantly greater for fail-fast matrices and support is more limited because of that expectation. |
| `flaky_test_attempts` | i64 | The number of times a TestExecution should be re-attempted if one or more of its test cases fail for any reason. The maximum number of reruns allowed is 10. Default is 0, which implies no reruns. |
| `test_executions` | Vec<String> | Output only. The list of test executions that the service creates for this matrix. |
| `outcome_summary` | String | Output Only. The overall outcome of the test. Only set when the test matrix state is FINISHED. |
| `client_info` | String | Information about the client which invoked the test. |
| `extended_invalid_matrix_details` | Vec<String> | Output only. Details about why a matrix was deemed invalid. If multiple checks can be safely performed, they will be reported but no assumptions should be made about the length of this list. |
| `state` | String | Output only. Indicates the current progress of the test matrix. |
| `invalid_matrix_details` | String | Output only. Describes why the matrix is considered invalid. Only useful for matrices in the INVALID state. |
| `test_specification` | String | Required. How to run the test. |
| `timestamp` | String | Output only. The time this test matrix was initially created. |
| `test_matrix_id` | String | Output only. Unique id set by the service. |
| `environment_matrix` | String | Required. The devices the tests are being executed on. |
| `result_storage` | String | Required. Where the results for the matrix are written. |
| `project_id` | String | The cloud project that owns the test matrix. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create test_matrice
test_matrice = provider.testing_api.Test_matrice {
    project_id = "value"  # The GCE project under which this job will run.
}

# Access test_matrice outputs
test_matrice_id = test_matrice.id
test_matrice_fail_fast = test_matrice.fail_fast
test_matrice_flaky_test_attempts = test_matrice.flaky_test_attempts
test_matrice_test_executions = test_matrice.test_executions
test_matrice_outcome_summary = test_matrice.outcome_summary
test_matrice_client_info = test_matrice.client_info
test_matrice_extended_invalid_matrix_details = test_matrice.extended_invalid_matrix_details
test_matrice_state = test_matrice.state
test_matrice_invalid_matrix_details = test_matrice.invalid_matrix_details
test_matrice_test_specification = test_matrice.test_specification
test_matrice_timestamp = test_matrice.timestamp
test_matrice_test_matrix_id = test_matrice.test_matrix_id
test_matrice_environment_matrix = test_matrice.environment_matrix
test_matrice_result_storage = test_matrice.result_storage
test_matrice_project_id = test_matrice.project_id
```

---


### Device_session

POST /v1/projects/{project_id}/deviceSessions

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `active_start_time` | String |  | Output only. The timestamp that the session first became ACTIVE. |
| `android_device` | String |  | Required. The requested device |
| `name` | String |  | Optional. Name of the DeviceSession, e.g. "projects/{project_id}/deviceSessions/{session_id}" |
| `state_histories` | Vec<String> |  | Output only. The historical state transitions of the session_state message including the current session state. |
| `ttl` | String |  | Optional. The amount of time that a device will be initially allocated for. This can eventually be extended with the UpdateDeviceSession RPC. Default: 15 minutes. |
| `create_time` | String |  | Output only. The time that the Session was created. |
| `inactivity_timeout` | String |  | Output only. The interval of time that this device must be interacted with before it transitions from ACTIVE to TIMEOUT_INACTIVITY. |
| `display_name` | String |  | Output only. The title of the DeviceSession to be presented in the UI. |
| `expire_time` | String |  | Optional. If the device is still in use at this time, any connections will be ended and the SessionState will transition from ACTIVE to FINISHED. |
| `state` | String |  | Output only. Current state of the DeviceSession. |
| `parent` | String | ✅ | Required. The Compute Engine project under which this device will be allocated. "projects/{project_id}" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `active_start_time` | String | Output only. The timestamp that the session first became ACTIVE. |
| `android_device` | String | Required. The requested device |
| `name` | String | Optional. Name of the DeviceSession, e.g. "projects/{project_id}/deviceSessions/{session_id}" |
| `state_histories` | Vec<String> | Output only. The historical state transitions of the session_state message including the current session state. |
| `ttl` | String | Optional. The amount of time that a device will be initially allocated for. This can eventually be extended with the UpdateDeviceSession RPC. Default: 15 minutes. |
| `create_time` | String | Output only. The time that the Session was created. |
| `inactivity_timeout` | String | Output only. The interval of time that this device must be interacted with before it transitions from ACTIVE to TIMEOUT_INACTIVITY. |
| `display_name` | String | Output only. The title of the DeviceSession to be presented in the UI. |
| `expire_time` | String | Optional. If the device is still in use at this time, any connections will be ended and the SessionState will transition from ACTIVE to FINISHED. |
| `state` | String | Output only. Current state of the DeviceSession. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create device_session
device_session = provider.testing_api.Device_session {
    parent = "value"  # Required. The Compute Engine project under which this device will be allocated. "projects/{project_id}"
}

# Access device_session outputs
device_session_id = device_session.id
device_session_active_start_time = device_session.active_start_time
device_session_android_device = device_session.android_device
device_session_name = device_session.name
device_session_state_histories = device_session.state_histories
device_session_ttl = device_session.ttl
device_session_create_time = device_session.create_time
device_session_inactivity_timeout = device_session.inactivity_timeout
device_session_display_name = device_session.display_name
device_session_expire_time = device_session.expire_time
device_session_state = device_session.state
```

---


### Test_environment_catalog

Gets the catalog of supported test environments. May return any of the following canonical error codes: - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the environment type does not exist - INTERNAL - if an internal error occurred

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `android_device_catalog` | String | Supported Android devices. |
| `software_catalog` | String | The software test environment provided by TestExecutionService. |
| `ios_device_catalog` | String | Supported iOS devices. |
| `device_ip_block_catalog` | String | The IP blocks used by devices in the test environment. |
| `network_configuration_catalog` | String | Supported network configurations. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access test_environment_catalog outputs
test_environment_catalog_id = test_environment_catalog.id
test_environment_catalog_android_device_catalog = test_environment_catalog.android_device_catalog
test_environment_catalog_software_catalog = test_environment_catalog.software_catalog
test_environment_catalog_ios_device_catalog = test_environment_catalog.ios_device_catalog
test_environment_catalog_device_ip_block_catalog = test_environment_catalog.device_ip_block_catalog
test_environment_catalog_network_configuration_catalog = test_environment_catalog.network_configuration_catalog
```

---


### Application_detail_service

Gets the details of an Android application APK.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcs_path` | String |  | A path to a file in Google Cloud Storage. Example: gs://build-app-1414623860166/app%40debug-unaligned.apk These paths are expected to be url encoded (percent encoding) |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create application_detail_service
application_detail_service = provider.testing_api.Application_detail_service {
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

# Create multiple test_matrice resources
test_matrice_0 = provider.testing_api.Test_matrice {
    project_id = "value-0"
}
test_matrice_1 = provider.testing_api.Test_matrice {
    project_id = "value-1"
}
test_matrice_2 = provider.testing_api.Test_matrice {
    project_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    test_matrice = provider.testing_api.Test_matrice {
        project_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Testing_api Documentation](https://cloud.google.com/testing_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
