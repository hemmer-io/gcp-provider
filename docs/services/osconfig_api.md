# Osconfig_api Service



**Resources**: 24

---

## Overview

The osconfig_api service provides access to 24 resource types:

- [Os_policy_assignment](#os_policy_assignment) [CRUD]
- [Global](#global) [RU]
- [Operation](#operation) [CR]
- [Inventorie](#inventorie) [R]
- [Patch_job](#patch_job) [CR]
- [Report](#report) [R]
- [Vulnerability_report](#vulnerability_report) [R]
- [Instance_detail](#instance_detail) [R]
- [Patch_deployment](#patch_deployment) [CRUD]
- [Vulnerability_report](#vulnerability_report) [R]
- [Report](#report) [R]
- [Os_policy_assignment](#os_policy_assignment) [CRUD]
- [Instance_os_policies_compliance](#instance_os_policies_compliance) [R]
- [Operation](#operation) [CR]
- [Inventorie](#inventorie) [R]
- [Operation](#operation) [CRD]
- [Policy_orchestrator](#policy_orchestrator) [CRUD]
- [Operation](#operation) [CRD]
- [Policy_orchestrator](#policy_orchestrator) [CRUD]
- [Guest_policie](#guest_policie) [CRUD]
- [Patch_job](#patch_job) [CR]
- [Instance_detail](#instance_detail) [R]
- [Instance](#instance) [C]
- [Patch_deployment](#patch_deployment) [CRUD]

---

## Resources


### Os_policy_assignment

Create an OS policy assignment. This method also creates the first revision of the OS policy assignment. This method returns a long running operation (LRO) that contains the rollout details. The rollout can be cancelled by cancelling the LRO. For more information, see [Method: projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1/projects.locations.osPolicyAssignments.operations/cancel).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reconciling` | bool |  | Output only. Indicates that reconciliation is in progress for the revision. This value is `true` when the `rollout_state` is one of: * IN_PROGRESS * CANCELLING |
| `revision_create_time` | String |  | Output only. The timestamp that the revision was created. |
| `name` | String |  | Resource name. Format: `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id}` This field is ignored when you create an OS policy assignment. |
| `revision_id` | String |  | Output only. The assignment revision ID A new revision is committed whenever a rollout is triggered for a OS policy assignment |
| `rollout` | String |  | Required. Rollout to deploy the OS policy assignment. A rollout is triggered in the following situations: 1) OSPolicyAssignment is created. 2) OSPolicyAssignment is updated and the update contains changes to one of the following fields: - instance_filter - os_policies 3) OSPolicyAssignment is deleted. |
| `rollout_state` | String |  | Output only. OS policy assignment rollout state |
| `baseline` | bool |  | Output only. Indicates that this revision has been successfully rolled out in this zone and new VMs will be assigned OS policies from this revision. For a given OS policy assignment, there is only one revision with a value of `true` for this field. |
| `etag` | String |  | The etag for this OS policy assignment. If this is provided on update, it must match the server's etag. |
| `instance_filter` | String |  | Required. Filter to select VMs. |
| `os_policies` | Vec<String> |  | Required. List of OS policies to be applied to the VMs. |
| `uid` | String |  | Output only. Server generated unique id for the OS policy assignment resource. |
| `deleted` | bool |  | Output only. Indicates that this revision deletes the OS policy assignment. |
| `description` | String |  | OS policy assignment description. Length of the description is limited to 1024 characters. |
| `parent` | String | ✅ | Required. The parent resource name in the form: projects/{project}/locations/{location}. Note: Specify the zone of your VMs as the location. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reconciling` | bool | Output only. Indicates that reconciliation is in progress for the revision. This value is `true` when the `rollout_state` is one of: * IN_PROGRESS * CANCELLING |
| `revision_create_time` | String | Output only. The timestamp that the revision was created. |
| `name` | String | Resource name. Format: `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id}` This field is ignored when you create an OS policy assignment. |
| `revision_id` | String | Output only. The assignment revision ID A new revision is committed whenever a rollout is triggered for a OS policy assignment |
| `rollout` | String | Required. Rollout to deploy the OS policy assignment. A rollout is triggered in the following situations: 1) OSPolicyAssignment is created. 2) OSPolicyAssignment is updated and the update contains changes to one of the following fields: - instance_filter - os_policies 3) OSPolicyAssignment is deleted. |
| `rollout_state` | String | Output only. OS policy assignment rollout state |
| `baseline` | bool | Output only. Indicates that this revision has been successfully rolled out in this zone and new VMs will be assigned OS policies from this revision. For a given OS policy assignment, there is only one revision with a value of `true` for this field. |
| `etag` | String | The etag for this OS policy assignment. If this is provided on update, it must match the server's etag. |
| `instance_filter` | String | Required. Filter to select VMs. |
| `os_policies` | Vec<String> | Required. List of OS policies to be applied to the VMs. |
| `uid` | String | Output only. Server generated unique id for the OS policy assignment resource. |
| `deleted` | bool | Output only. Indicates that this revision deletes the OS policy assignment. |
| `description` | String | OS policy assignment description. Length of the description is limited to 1024 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create os_policy_assignment
os_policy_assignment = provider.osconfig_api.Os_policy_assignment {
    parent = "value"  # Required. The parent resource name in the form: projects/{project}/locations/{location}. Note: Specify the zone of your VMs as the location.
}

# Access os_policy_assignment outputs
os_policy_assignment_id = os_policy_assignment.id
os_policy_assignment_reconciling = os_policy_assignment.reconciling
os_policy_assignment_revision_create_time = os_policy_assignment.revision_create_time
os_policy_assignment_name = os_policy_assignment.name
os_policy_assignment_revision_id = os_policy_assignment.revision_id
os_policy_assignment_rollout = os_policy_assignment.rollout
os_policy_assignment_rollout_state = os_policy_assignment.rollout_state
os_policy_assignment_baseline = os_policy_assignment.baseline
os_policy_assignment_etag = os_policy_assignment.etag
os_policy_assignment_instance_filter = os_policy_assignment.instance_filter
os_policy_assignment_os_policies = os_policy_assignment.os_policies
os_policy_assignment_uid = os_policy_assignment.uid
os_policy_assignment_deleted = os_policy_assignment.deleted
os_policy_assignment_description = os_policy_assignment.description
```

---


### Global

GetProjectFeatureSettings returns the VM Manager feature settings for a project.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. Immutable. Name specifies the URL for the ProjectFeatureSettings resource: projects/project_id/locations/global/projectFeatureSettings. |
| `patch_and_config_feature_set` | String |  | Set PatchAndConfigFeatureSet for the project. |
| `name` | String | ✅ | Required. Immutable. Name specifies the URL for the ProjectFeatureSettings resource: projects/project_id/locations/global/projectFeatureSettings. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. Immutable. Name specifies the URL for the ProjectFeatureSettings resource: projects/project_id/locations/global/projectFeatureSettings. |
| `patch_and_config_feature_set` | String | Set PatchAndConfigFeatureSet for the project. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access global outputs
global_id = global.id
global_name = global.name
global_patch_and_config_feature_set = global.patch_and_config_feature_set
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of `1`, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.osconfig_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_error = operation.error
operation_done = operation.done
operation_response = operation.response
operation_name = operation.name
```

---


### Inventorie

Get inventory data for the specified VM instance. If the VM has no associated inventory, the message `NOT_FOUND` is returned.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `os_info` | String | Base level operating system information for the VM. |
| `items` | HashMap<String, String> | Inventory items related to the VM keyed by an opaque unique identifier for each inventory item. The identifier is unique to each distinct and addressable inventory item and will change, when there is a new package version. |
| `update_time` | String | Output only. Timestamp of the last reported inventory for the VM. |
| `name` | String | Output only. The `Inventory` API resource name. Format: `projects/{project_number}/locations/{location}/instances/{instance_id}/inventory` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access inventorie outputs
inventorie_id = inventorie.id
inventorie_os_info = inventorie.os_info
inventorie_items = inventorie.items
inventorie_update_time = inventorie.update_time
inventorie_name = inventorie.name
```

---


### Patch_job

Cancel a patch job. The patch job must be active. Canceled patch jobs cannot be restarted.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. Name of the patch in the form `projects/*/patchJobs/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Unique identifier for this patch job in the form `projects/*/patchJobs/*` |
| `rollout` | String | Rollout strategy being applied. |
| `error_message` | String | If this patch job failed, this message provides information about the failure. |
| `description` | String | Description of the patch job. Length of the description is limited to 1024 characters. |
| `instance_filter` | String | Instances to patch. |
| `percent_complete` | f64 | Reflects the overall progress of the patch job in the range of 0.0 being no progress to 100.0 being complete. |
| `duration` | String | Duration of the patch job. After the duration ends, the patch job times out. |
| `patch_config` | String | Patch configuration being applied. |
| `patch_deployment` | String | Output only. Name of the patch deployment that created this patch job. |
| `create_time` | String | Time this patch job was created. |
| `instance_details_summary` | String | Summary of instance details. |
| `state` | String | The current state of the PatchJob. |
| `display_name` | String | Display name for this patch job. This is not a unique identifier. |
| `update_time` | String | Last time this patch job was updated. |
| `dry_run` | bool | If this patch job is a dry run, the agent reports that it has finished without running any updates on the VM instance. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create patch_job
patch_job = provider.osconfig_api.Patch_job {
    name = "value"  # Required. Name of the patch in the form `projects/*/patchJobs/*`
}

# Access patch_job outputs
patch_job_id = patch_job.id
patch_job_name = patch_job.name
patch_job_rollout = patch_job.rollout
patch_job_error_message = patch_job.error_message
patch_job_description = patch_job.description
patch_job_instance_filter = patch_job.instance_filter
patch_job_percent_complete = patch_job.percent_complete
patch_job_duration = patch_job.duration
patch_job_patch_config = patch_job.patch_config
patch_job_patch_deployment = patch_job.patch_deployment
patch_job_create_time = patch_job.create_time
patch_job_instance_details_summary = patch_job.instance_details_summary
patch_job_state = patch_job.state
patch_job_display_name = patch_job.display_name
patch_job_update_time = patch_job.update_time
patch_job_dry_run = patch_job.dry_run
```

---


### Report

Get the OS policy assignment report for the specified Compute Engine VM instance.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The `OSPolicyAssignmentReport` API resource name. Format: `projects/{project_number}/locations/{location}/instances/{instance_id}/osPolicyAssignments/{os_policy_assignment_id}/report` |
| `instance` | String | The Compute Engine VM instance name. |
| `last_run_id` | String | Unique identifier of the last attempted run to apply the OS policies associated with this assignment on the VM. This ID is logged by the OS Config agent while applying the OS policies associated with this assignment on the VM. NOTE: If the service is unable to successfully connect to the agent for this run, then this id will not be available in the agent logs. |
| `update_time` | String | Timestamp for when the report was last generated. |
| `os_policy_compliances` | Vec<String> | Compliance data for each `OSPolicy` that is applied to the VM. |
| `os_policy_assignment` | String | Reference to the `OSPolicyAssignment` API resource that the `OSPolicy` belongs to. Format: `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id@revision_id}` |


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
report_instance = report.instance
report_last_run_id = report.last_run_id
report_update_time = report.update_time
report_os_policy_compliances = report.os_policy_compliances
report_os_policy_assignment = report.os_policy_assignment
```

---


### Vulnerability_report

Gets the vulnerability report for the specified VM instance. Only VMs with inventory data have vulnerability reports associated with them.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vulnerabilities` | Vec<String> | Output only. List of vulnerabilities affecting the VM. |
| `highest_upgradable_cve_severity` | String | Output only. Highest level of severity among all the upgradable vulnerabilities with CVEs attached. |
| `name` | String | Output only. The `vulnerabilityReport` API resource name. Format: `projects/{project_number}/locations/{location}/instances/{instance_id}/vulnerabilityReport` |
| `update_time` | String | Output only. The timestamp for when the last vulnerability report was generated for the VM. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access vulnerability_report outputs
vulnerability_report_id = vulnerability_report.id
vulnerability_report_vulnerabilities = vulnerability_report.vulnerabilities
vulnerability_report_highest_upgradable_cve_severity = vulnerability_report.highest_upgradable_cve_severity
vulnerability_report_name = vulnerability_report.name
vulnerability_report_update_time = vulnerability_report.update_time
```

---


### Instance_detail

Get a list of instance details for a given patch job.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `patch_job_instance_details` | Vec<String> | A list of instance status. |
| `next_page_token` | String | A pagination token that can be used to get the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access instance_detail outputs
instance_detail_id = instance_detail.id
instance_detail_patch_job_instance_details = instance_detail.patch_job_instance_details
instance_detail_next_page_token = instance_detail.next_page_token
```

---


### Patch_deployment

Create an OS Config patch deployment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time the patch deployment was created. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `last_execute_time` | String |  | Output only. The last time a patch job was started by this deployment. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `recurring_schedule` | String |  | Required. Schedule recurring executions. |
| `patch_config` | String |  | Optional. Patch configuration that is applied. |
| `state` | String |  | Output only. Current state of the patch deployment. |
| `description` | String |  | Optional. Description of the patch deployment. Length of the description is limited to 1024 characters. |
| `duration` | String |  | Optional. Duration of the patch. After the duration ends, the patch times out. |
| `name` | String |  | Unique name for the patch deployment resource in a project. The patch deployment name is in the form: `projects/{project_id}/patchDeployments/{patch_deployment_id}`. This field is ignored when you create a new patch deployment. |
| `instance_filter` | String |  | Required. VM instances to patch. |
| `rollout` | String |  | Optional. Rollout strategy of the patch job. |
| `one_time_schedule` | String |  | Required. Schedule a one-time execution. |
| `update_time` | String |  | Output only. Time the patch deployment was last updated. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `parent` | String | ✅ | Required. The project to apply this patch deployment to in the form `projects/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time the patch deployment was created. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `last_execute_time` | String | Output only. The last time a patch job was started by this deployment. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `recurring_schedule` | String | Required. Schedule recurring executions. |
| `patch_config` | String | Optional. Patch configuration that is applied. |
| `state` | String | Output only. Current state of the patch deployment. |
| `description` | String | Optional. Description of the patch deployment. Length of the description is limited to 1024 characters. |
| `duration` | String | Optional. Duration of the patch. After the duration ends, the patch times out. |
| `name` | String | Unique name for the patch deployment resource in a project. The patch deployment name is in the form: `projects/{project_id}/patchDeployments/{patch_deployment_id}`. This field is ignored when you create a new patch deployment. |
| `instance_filter` | String | Required. VM instances to patch. |
| `rollout` | String | Optional. Rollout strategy of the patch job. |
| `one_time_schedule` | String | Required. Schedule a one-time execution. |
| `update_time` | String | Output only. Time the patch deployment was last updated. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create patch_deployment
patch_deployment = provider.osconfig_api.Patch_deployment {
    parent = "value"  # Required. The project to apply this patch deployment to in the form `projects/*`.
}

# Access patch_deployment outputs
patch_deployment_id = patch_deployment.id
patch_deployment_create_time = patch_deployment.create_time
patch_deployment_last_execute_time = patch_deployment.last_execute_time
patch_deployment_recurring_schedule = patch_deployment.recurring_schedule
patch_deployment_patch_config = patch_deployment.patch_config
patch_deployment_state = patch_deployment.state
patch_deployment_description = patch_deployment.description
patch_deployment_duration = patch_deployment.duration
patch_deployment_name = patch_deployment.name
patch_deployment_instance_filter = patch_deployment.instance_filter
patch_deployment_rollout = patch_deployment.rollout
patch_deployment_one_time_schedule = patch_deployment.one_time_schedule
patch_deployment_update_time = patch_deployment.update_time
```

---


### Vulnerability_report

Gets the vulnerability report for the specified VM instance. Only VMs with inventory data have vulnerability reports associated with them.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `highest_upgradable_cve_severity` | String | Output only. Highest level of severity among all the upgradable vulnerabilities with CVEs attached. |
| `vulnerabilities` | Vec<String> | Output only. List of vulnerabilities affecting the VM. |
| `update_time` | String | Output only. The timestamp for when the last vulnerability report was generated for the VM. |
| `name` | String | Output only. The `vulnerabilityReport` API resource name. Format: `projects/{project_number}/locations/{location}/instances/{instance_id}/vulnerabilityReport` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access vulnerability_report outputs
vulnerability_report_id = vulnerability_report.id
vulnerability_report_highest_upgradable_cve_severity = vulnerability_report.highest_upgradable_cve_severity
vulnerability_report_vulnerabilities = vulnerability_report.vulnerabilities
vulnerability_report_update_time = vulnerability_report.update_time
vulnerability_report_name = vulnerability_report.name
```

---


### Report

Get the OS policy assignment report for the specified Compute Engine VM instance.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Timestamp for when the report was last generated. |
| `name` | String | The `OSPolicyAssignmentReport` API resource name. Format: `projects/{project_number}/locations/{location}/instances/{instance_id}/osPolicyAssignments/{os_policy_assignment_id}/report` |
| `os_policy_compliances` | Vec<String> | Compliance data for each `OSPolicy` that is applied to the VM. |
| `instance` | String | The Compute Engine VM instance name. |
| `last_run_id` | String | Unique identifier of the last attempted run to apply the OS policies associated with this assignment on the VM. This ID is logged by the OS Config agent while applying the OS policies associated with this assignment on the VM. NOTE: If the service is unable to successfully connect to the agent for this run, then this id will not be available in the agent logs. |
| `os_policy_assignment` | String | Reference to the `OSPolicyAssignment` API resource that the `OSPolicy` belongs to. Format: `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id@revision_id}` |


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
report_update_time = report.update_time
report_name = report.name
report_os_policy_compliances = report.os_policy_compliances
report_instance = report.instance
report_last_run_id = report.last_run_id
report_os_policy_assignment = report.os_policy_assignment
```

---


### Os_policy_assignment

Create an OS policy assignment. This method also creates the first revision of the OS policy assignment. This method returns a long running operation (LRO) that contains the rollout details. The rollout can be cancelled by cancelling the LRO. For more information, see [Method: projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1alpha/projects.locations.osPolicyAssignments.operations/cancel).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uid` | String |  | Output only. Server generated unique id for the OS policy assignment resource. |
| `rollout_state` | String |  | Output only. OS policy assignment rollout state |
| `instance_filter` | String |  | Required. Filter to select VMs. |
| `description` | String |  | OS policy assignment description. Length of the description is limited to 1024 characters. |
| `name` | String |  | Resource name. Format: `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id}` This field is ignored when you create an OS policy assignment. |
| `baseline` | bool |  | Output only. Indicates that this revision has been successfully rolled out in this zone and new VMs will be assigned OS policies from this revision. For a given OS policy assignment, there is only one revision with a value of `true` for this field. |
| `revision_create_time` | String |  | Output only. The timestamp that the revision was created. |
| `etag` | String |  | The etag for this OS policy assignment. If this is provided on update, it must match the server's etag. |
| `deleted` | bool |  | Output only. Indicates that this revision deletes the OS policy assignment. |
| `os_policies` | Vec<String> |  | Required. List of OS policies to be applied to the VMs. |
| `reconciling` | bool |  | Output only. Indicates that reconciliation is in progress for the revision. This value is `true` when the `rollout_state` is one of: * IN_PROGRESS * CANCELLING |
| `revision_id` | String |  | Output only. The assignment revision ID A new revision is committed whenever a rollout is triggered for a OS policy assignment |
| `rollout` | String |  | Required. Rollout to deploy the OS policy assignment. A rollout is triggered in the following situations: 1) OSPolicyAssignment is created. 2) OSPolicyAssignment is updated and the update contains changes to one of the following fields: - instance_filter - os_policies 3) OSPolicyAssignment is deleted. |
| `parent` | String | ✅ | Required. The parent resource name in the form: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. Server generated unique id for the OS policy assignment resource. |
| `rollout_state` | String | Output only. OS policy assignment rollout state |
| `instance_filter` | String | Required. Filter to select VMs. |
| `description` | String | OS policy assignment description. Length of the description is limited to 1024 characters. |
| `name` | String | Resource name. Format: `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id}` This field is ignored when you create an OS policy assignment. |
| `baseline` | bool | Output only. Indicates that this revision has been successfully rolled out in this zone and new VMs will be assigned OS policies from this revision. For a given OS policy assignment, there is only one revision with a value of `true` for this field. |
| `revision_create_time` | String | Output only. The timestamp that the revision was created. |
| `etag` | String | The etag for this OS policy assignment. If this is provided on update, it must match the server's etag. |
| `deleted` | bool | Output only. Indicates that this revision deletes the OS policy assignment. |
| `os_policies` | Vec<String> | Required. List of OS policies to be applied to the VMs. |
| `reconciling` | bool | Output only. Indicates that reconciliation is in progress for the revision. This value is `true` when the `rollout_state` is one of: * IN_PROGRESS * CANCELLING |
| `revision_id` | String | Output only. The assignment revision ID A new revision is committed whenever a rollout is triggered for a OS policy assignment |
| `rollout` | String | Required. Rollout to deploy the OS policy assignment. A rollout is triggered in the following situations: 1) OSPolicyAssignment is created. 2) OSPolicyAssignment is updated and the update contains changes to one of the following fields: - instance_filter - os_policies 3) OSPolicyAssignment is deleted. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create os_policy_assignment
os_policy_assignment = provider.osconfig_api.Os_policy_assignment {
    parent = "value"  # Required. The parent resource name in the form: projects/{project}/locations/{location}
}

# Access os_policy_assignment outputs
os_policy_assignment_id = os_policy_assignment.id
os_policy_assignment_uid = os_policy_assignment.uid
os_policy_assignment_rollout_state = os_policy_assignment.rollout_state
os_policy_assignment_instance_filter = os_policy_assignment.instance_filter
os_policy_assignment_description = os_policy_assignment.description
os_policy_assignment_name = os_policy_assignment.name
os_policy_assignment_baseline = os_policy_assignment.baseline
os_policy_assignment_revision_create_time = os_policy_assignment.revision_create_time
os_policy_assignment_etag = os_policy_assignment.etag
os_policy_assignment_deleted = os_policy_assignment.deleted
os_policy_assignment_os_policies = os_policy_assignment.os_policies
os_policy_assignment_reconciling = os_policy_assignment.reconciling
os_policy_assignment_revision_id = os_policy_assignment.revision_id
os_policy_assignment_rollout = os_policy_assignment.rollout
```

---


### Instance_os_policies_compliance

Get OS policies compliance data for the specified Compute Engine VM instance.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance` | String | Output only. The Compute Engine VM instance name. |
| `os_policy_compliances` | Vec<String> | Output only. Compliance data for each `OSPolicy` that is applied to the VM. |
| `last_compliance_check_time` | String | Output only. Timestamp of the last compliance check for the VM. |
| `last_compliance_run_id` | String | Output only. Unique identifier for the last compliance run. This id will be logged by the OS config agent during a compliance run and can be used for debugging and tracing purpose. |
| `name` | String | Output only. The `InstanceOSPoliciesCompliance` API resource name. Format: `projects/{project_number}/locations/{location}/instanceOSPoliciesCompliances/{instance_id}` |
| `state` | String | Output only. Compliance state of the VM. |
| `detailed_state_reason` | String | Output only. The reason for the `detailed_state` of the VM (if any). |
| `detailed_state` | String | Output only. Detailed compliance state of the VM. This field is populated only when compliance state is `UNKNOWN`. It may contain one of the following values: * `no-compliance-data`: Compliance data is not available for this VM. * `no-agent-detected`: OS Config agent is not detected for this VM. * `config-not-supported-by-agent`: The version of the OS Config agent running on this VM does not support configuration management. * `inactive`: VM is not running. * `internal-service-errors`: There were internal service errors encountered while enforcing compliance. * `agent-errors`: OS config agent encountered errors while enforcing compliance. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access instance_os_policies_compliance outputs
instance_os_policies_compliance_id = instance_os_policies_compliance.id
instance_os_policies_compliance_instance = instance_os_policies_compliance.instance
instance_os_policies_compliance_os_policy_compliances = instance_os_policies_compliance.os_policy_compliances
instance_os_policies_compliance_last_compliance_check_time = instance_os_policies_compliance.last_compliance_check_time
instance_os_policies_compliance_last_compliance_run_id = instance_os_policies_compliance.last_compliance_run_id
instance_os_policies_compliance_name = instance_os_policies_compliance.name
instance_os_policies_compliance_state = instance_os_policies_compliance.state
instance_os_policies_compliance_detailed_state_reason = instance_os_policies_compliance.detailed_state_reason
instance_os_policies_compliance_detailed_state = instance_os_policies_compliance.detailed_state
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of `1`, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.osconfig_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_response = operation.response
operation_name = operation.name
operation_done = operation.done
operation_error = operation.error
```

---


### Inventorie

Get inventory data for the specified VM instance. If the VM has no associated inventory, the message `NOT_FOUND` is returned.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | HashMap<String, String> | Output only. Inventory items related to the VM keyed by an opaque unique identifier for each inventory item. The identifier is unique to each distinct and addressable inventory item and will change, when there is a new package version. |
| `os_info` | String | Output only. Base level operating system information for the VM. |
| `update_time` | String | Output only. Timestamp of the last reported inventory for the VM. |
| `name` | String | Output only. The `Inventory` API resource name. Format: `projects/{project_number}/locations/{location}/instances/{instance_id}/inventory` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access inventorie outputs
inventorie_id = inventorie.id
inventorie_items = inventorie.items
inventorie_os_info = inventorie.os_info
inventorie_update_time = inventorie.update_time
inventorie_name = inventorie.name
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
operation = provider.osconfig_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_metadata = operation.metadata
operation_error = operation.error
operation_done = operation.done
operation_name = operation.name
```

---


### Policy_orchestrator

Creates a new policy orchestrator under the given folder resource. `name` field of the given orchestrator are ignored and instead replaced by a product of `parent` and `policy_orchestrator_id`. Orchestrator state field might be only set to `ACTIVE`, `STOPPED` or omitted (in which case, the created resource will be in `ACTIVE` state anyway).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Optional. State of the orchestrator. Can be updated to change orchestrator behaviour. Allowed values: - `ACTIVE` - orchestrator is actively looking for actions to be taken. - `STOPPED` - orchestrator won't make any changes. Note: There might be more states added in the future. We use string here instead of an enum, to avoid the need of propagating new states to all the client code. |
| `update_time` | String |  | Output only. Timestamp when the policy orchestrator resource was last modified. |
| `description` | String |  | Optional. Freeform text describing the purpose of the resource. |
| `orchestration_state` | String |  | Output only. State of the orchestration. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `action` | String |  | Required. Action to be done by the orchestrator in `projects/{project_id}/zones/{zone_id}` locations defined by the `orchestration_scope`. Allowed values: - `UPSERT` - Orchestrator will create or update target resources. - `DELETE` - Orchestrator will delete target resources, if they exist |
| `name` | String |  | Immutable. Identifier. In form of * `organizations/{organization_id}/locations/global/policyOrchestrators/{orchestrator_id}` * `folders/{folder_id}/locations/global/policyOrchestrators/{orchestrator_id}` * `projects/{project_id_or_number}/locations/global/policyOrchestrators/{orchestrator_id}` |
| `create_time` | String |  | Output only. Timestamp when the policy orchestrator resource was created. |
| `orchestration_scope` | String |  | Optional. Defines scope for the orchestration, in context of the enclosing PolicyOrchestrator resource. Scope is expanded into a list of pairs, in which the rollout action will take place. Expansion starts with a Folder resource parenting the PolicyOrchestrator resource: - All the descendant projects are listed. - List of project is cross joined with a list of all available zones. - Resulting list of pairs is filtered according to the selectors. |
| `orchestrated_resource` | String |  | Required. Resource to be orchestrated by the policy orchestrator. |
| `reconciling` | bool |  | Output only. Set to true, if the there are ongoing changes being applied by the orchestrator. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `parent` | String | ✅ | Required. The parent resource name in the form of: * `organizations/{organization_id}/locations/global` * `folders/{folder_id}/locations/global` * `projects/{project_id_or_number}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Optional. State of the orchestrator. Can be updated to change orchestrator behaviour. Allowed values: - `ACTIVE` - orchestrator is actively looking for actions to be taken. - `STOPPED` - orchestrator won't make any changes. Note: There might be more states added in the future. We use string here instead of an enum, to avoid the need of propagating new states to all the client code. |
| `update_time` | String | Output only. Timestamp when the policy orchestrator resource was last modified. |
| `description` | String | Optional. Freeform text describing the purpose of the resource. |
| `orchestration_state` | String | Output only. State of the orchestration. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `action` | String | Required. Action to be done by the orchestrator in `projects/{project_id}/zones/{zone_id}` locations defined by the `orchestration_scope`. Allowed values: - `UPSERT` - Orchestrator will create or update target resources. - `DELETE` - Orchestrator will delete target resources, if they exist |
| `name` | String | Immutable. Identifier. In form of * `organizations/{organization_id}/locations/global/policyOrchestrators/{orchestrator_id}` * `folders/{folder_id}/locations/global/policyOrchestrators/{orchestrator_id}` * `projects/{project_id_or_number}/locations/global/policyOrchestrators/{orchestrator_id}` |
| `create_time` | String | Output only. Timestamp when the policy orchestrator resource was created. |
| `orchestration_scope` | String | Optional. Defines scope for the orchestration, in context of the enclosing PolicyOrchestrator resource. Scope is expanded into a list of pairs, in which the rollout action will take place. Expansion starts with a Folder resource parenting the PolicyOrchestrator resource: - All the descendant projects are listed. - List of project is cross joined with a list of all available zones. - Resulting list of pairs is filtered according to the selectors. |
| `orchestrated_resource` | String | Required. Resource to be orchestrated by the policy orchestrator. |
| `reconciling` | bool | Output only. Set to true, if the there are ongoing changes being applied by the orchestrator. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policy_orchestrator
policy_orchestrator = provider.osconfig_api.Policy_orchestrator {
    parent = "value"  # Required. The parent resource name in the form of: * `organizations/{organization_id}/locations/global` * `folders/{folder_id}/locations/global` * `projects/{project_id_or_number}/locations/global`
}

# Access policy_orchestrator outputs
policy_orchestrator_id = policy_orchestrator.id
policy_orchestrator_state = policy_orchestrator.state
policy_orchestrator_update_time = policy_orchestrator.update_time
policy_orchestrator_description = policy_orchestrator.description
policy_orchestrator_orchestration_state = policy_orchestrator.orchestration_state
policy_orchestrator_etag = policy_orchestrator.etag
policy_orchestrator_action = policy_orchestrator.action
policy_orchestrator_name = policy_orchestrator.name
policy_orchestrator_create_time = policy_orchestrator.create_time
policy_orchestrator_orchestration_scope = policy_orchestrator.orchestration_scope
policy_orchestrator_orchestrated_resource = policy_orchestrator.orchestrated_resource
policy_orchestrator_reconciling = policy_orchestrator.reconciling
policy_orchestrator_labels = policy_orchestrator.labels
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation = provider.osconfig_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_response = operation.response
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
```

---


### Policy_orchestrator

Creates a new policy orchestrator under the given folder resource. `name` field of the given orchestrator are ignored and instead replaced by a product of `parent` and `policy_orchestrator_id`. Orchestrator state field might be only set to `ACTIVE`, `STOPPED` or omitted (in which case, the created resource will be in `ACTIVE` state anyway).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Timestamp when the policy orchestrator resource was created. |
| `state` | String |  | Optional. State of the orchestrator. Can be updated to change orchestrator behaviour. Allowed values: - `ACTIVE` - orchestrator is actively looking for actions to be taken. - `STOPPED` - orchestrator won't make any changes. Note: There might be more states added in the future. We use string here instead of an enum, to avoid the need of propagating new states to all the client code. |
| `name` | String |  | Immutable. Identifier. In form of * `organizations/{organization_id}/locations/global/policyOrchestrators/{orchestrator_id}` * `folders/{folder_id}/locations/global/policyOrchestrators/{orchestrator_id}` * `projects/{project_id_or_number}/locations/global/policyOrchestrators/{orchestrator_id}` |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `orchestrated_resource` | String |  | Required. Resource to be orchestrated by the policy orchestrator. |
| `orchestration_state` | String |  | Output only. State of the orchestration. |
| `reconciling` | bool |  | Output only. Set to true, if the there are ongoing changes being applied by the orchestrator. |
| `update_time` | String |  | Output only. Timestamp when the policy orchestrator resource was last modified. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `action` | String |  | Required. Action to be done by the orchestrator in `projects/{project_id}/zones/{zone_id}` locations defined by the `orchestration_scope`. Allowed values: - `UPSERT` - Orchestrator will create or update target resources. - `DELETE` - Orchestrator will delete target resources, if they exist |
| `description` | String |  | Optional. Freeform text describing the purpose of the resource. |
| `orchestration_scope` | String |  | Optional. Defines scope for the orchestration, in context of the enclosing PolicyOrchestrator resource. Scope is expanded into a list of pairs, in which the rollout action will take place. Expansion starts with a Folder resource parenting the PolicyOrchestrator resource: - All the descendant projects are listed. - List of project is cross joined with a list of all available zones. - Resulting list of pairs is filtered according to the selectors. |
| `parent` | String | ✅ | Required. The parent resource name in the form of: * `organizations/{organization_id}/locations/global` * `folders/{folder_id}/locations/global` * `projects/{project_id_or_number}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Timestamp when the policy orchestrator resource was created. |
| `state` | String | Optional. State of the orchestrator. Can be updated to change orchestrator behaviour. Allowed values: - `ACTIVE` - orchestrator is actively looking for actions to be taken. - `STOPPED` - orchestrator won't make any changes. Note: There might be more states added in the future. We use string here instead of an enum, to avoid the need of propagating new states to all the client code. |
| `name` | String | Immutable. Identifier. In form of * `organizations/{organization_id}/locations/global/policyOrchestrators/{orchestrator_id}` * `folders/{folder_id}/locations/global/policyOrchestrators/{orchestrator_id}` * `projects/{project_id_or_number}/locations/global/policyOrchestrators/{orchestrator_id}` |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |
| `orchestrated_resource` | String | Required. Resource to be orchestrated by the policy orchestrator. |
| `orchestration_state` | String | Output only. State of the orchestration. |
| `reconciling` | bool | Output only. Set to true, if the there are ongoing changes being applied by the orchestrator. |
| `update_time` | String | Output only. Timestamp when the policy orchestrator resource was last modified. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `action` | String | Required. Action to be done by the orchestrator in `projects/{project_id}/zones/{zone_id}` locations defined by the `orchestration_scope`. Allowed values: - `UPSERT` - Orchestrator will create or update target resources. - `DELETE` - Orchestrator will delete target resources, if they exist |
| `description` | String | Optional. Freeform text describing the purpose of the resource. |
| `orchestration_scope` | String | Optional. Defines scope for the orchestration, in context of the enclosing PolicyOrchestrator resource. Scope is expanded into a list of pairs, in which the rollout action will take place. Expansion starts with a Folder resource parenting the PolicyOrchestrator resource: - All the descendant projects are listed. - List of project is cross joined with a list of all available zones. - Resulting list of pairs is filtered according to the selectors. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policy_orchestrator
policy_orchestrator = provider.osconfig_api.Policy_orchestrator {
    parent = "value"  # Required. The parent resource name in the form of: * `organizations/{organization_id}/locations/global` * `folders/{folder_id}/locations/global` * `projects/{project_id_or_number}/locations/global`
}

# Access policy_orchestrator outputs
policy_orchestrator_id = policy_orchestrator.id
policy_orchestrator_create_time = policy_orchestrator.create_time
policy_orchestrator_state = policy_orchestrator.state
policy_orchestrator_name = policy_orchestrator.name
policy_orchestrator_labels = policy_orchestrator.labels
policy_orchestrator_orchestrated_resource = policy_orchestrator.orchestrated_resource
policy_orchestrator_orchestration_state = policy_orchestrator.orchestration_state
policy_orchestrator_reconciling = policy_orchestrator.reconciling
policy_orchestrator_update_time = policy_orchestrator.update_time
policy_orchestrator_etag = policy_orchestrator.etag
policy_orchestrator_action = policy_orchestrator.action
policy_orchestrator_description = policy_orchestrator.description
policy_orchestrator_orchestration_scope = policy_orchestrator.orchestration_scope
```

---


### Guest_policie

Create an OS Config guest policy.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `assignment` | String |  | Required. Specifies the VM instances that are assigned to this policy. This allows you to target sets or groups of VM instances by different parameters such as labels, names, OS, or zones. If left empty, all VM instances underneath this policy are targeted. At the same level in the resource hierarchy (that is within a project), the service prevents the creation of multiple policies that conflict with each other. For more information, see how the service [handles assignment conflicts](/compute/docs/os-config-management/create-guest-policy#handle-conflicts). |
| `etag` | String |  | The etag for this guest policy. If this is provided on update, it must match the server's etag. |
| `create_time` | String |  | Output only. Time this guest policy was created. |
| `name` | String |  | Required. Unique name of the resource in this project using one of the following forms: `projects/{project_number}/guestPolicies/{guest_policy_id}`. |
| `package_repositories` | Vec<String> |  | A list of package repositories to configure on the VM instance. This is done before any other configs are applied so they can use these repos. Package repositories are only configured if the corresponding package manager(s) are available. |
| `description` | String |  | Description of the guest policy. Length of the description is limited to 1024 characters. |
| `packages` | Vec<String> |  | The software packages to be managed by this policy. |
| `update_time` | String |  | Output only. Last time this guest policy was updated. |
| `recipes` | Vec<String> |  | A list of Recipes to install on the VM instance. |
| `parent` | String | ✅ | Required. The resource name of the parent using one of the following forms: `projects/{project_number}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assignment` | String | Required. Specifies the VM instances that are assigned to this policy. This allows you to target sets or groups of VM instances by different parameters such as labels, names, OS, or zones. If left empty, all VM instances underneath this policy are targeted. At the same level in the resource hierarchy (that is within a project), the service prevents the creation of multiple policies that conflict with each other. For more information, see how the service [handles assignment conflicts](/compute/docs/os-config-management/create-guest-policy#handle-conflicts). |
| `etag` | String | The etag for this guest policy. If this is provided on update, it must match the server's etag. |
| `create_time` | String | Output only. Time this guest policy was created. |
| `name` | String | Required. Unique name of the resource in this project using one of the following forms: `projects/{project_number}/guestPolicies/{guest_policy_id}`. |
| `package_repositories` | Vec<String> | A list of package repositories to configure on the VM instance. This is done before any other configs are applied so they can use these repos. Package repositories are only configured if the corresponding package manager(s) are available. |
| `description` | String | Description of the guest policy. Length of the description is limited to 1024 characters. |
| `packages` | Vec<String> | The software packages to be managed by this policy. |
| `update_time` | String | Output only. Last time this guest policy was updated. |
| `recipes` | Vec<String> | A list of Recipes to install on the VM instance. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create guest_policie
guest_policie = provider.osconfig_api.Guest_policie {
    parent = "value"  # Required. The resource name of the parent using one of the following forms: `projects/{project_number}`.
}

# Access guest_policie outputs
guest_policie_id = guest_policie.id
guest_policie_assignment = guest_policie.assignment
guest_policie_etag = guest_policie.etag
guest_policie_create_time = guest_policie.create_time
guest_policie_name = guest_policie.name
guest_policie_package_repositories = guest_policie.package_repositories
guest_policie_description = guest_policie.description
guest_policie_packages = guest_policie.packages
guest_policie_update_time = guest_policie.update_time
guest_policie_recipes = guest_policie.recipes
```

---


### Patch_job

Cancel a patch job. The patch job must be active. Canceled patch jobs cannot be restarted.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. Name of the patch in the form `projects/*/patchJobs/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Unique identifier for this patch job in the form `projects/*/patchJobs/*` |
| `create_time` | String | Time this patch job was created. |
| `description` | String | Description of the patch job. Length of the description is limited to 1024 characters. |
| `instance_filter` | String | Instances to patch. |
| `duration` | String | Duration of the patch job. After the duration ends, the patch job times out. |
| `patch_config` | String | Patch configuration being applied. |
| `error_message` | String | If this patch job failed, this message provides information about the failure. |
| `patch_deployment` | String | Output only. Name of the patch deployment that created this patch job. |
| `state` | String | The current state of the PatchJob. |
| `display_name` | String | Display name for this patch job. This is not a unique identifier. |
| `percent_complete` | f64 | Reflects the overall progress of the patch job in the range of 0.0 being no progress to 100.0 being complete. |
| `instance_details_summary` | String | Summary of instance details. |
| `rollout` | String | Rollout strategy being applied. |
| `dry_run` | bool | If this patch job is a dry run, the agent reports that it has finished without running any updates on the VM instance. |
| `update_time` | String | Last time this patch job was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create patch_job
patch_job = provider.osconfig_api.Patch_job {
    name = "value"  # Required. Name of the patch in the form `projects/*/patchJobs/*`
}

# Access patch_job outputs
patch_job_id = patch_job.id
patch_job_name = patch_job.name
patch_job_create_time = patch_job.create_time
patch_job_description = patch_job.description
patch_job_instance_filter = patch_job.instance_filter
patch_job_duration = patch_job.duration
patch_job_patch_config = patch_job.patch_config
patch_job_error_message = patch_job.error_message
patch_job_patch_deployment = patch_job.patch_deployment
patch_job_state = patch_job.state
patch_job_display_name = patch_job.display_name
patch_job_percent_complete = patch_job.percent_complete
patch_job_instance_details_summary = patch_job.instance_details_summary
patch_job_rollout = patch_job.rollout
patch_job_dry_run = patch_job.dry_run
patch_job_update_time = patch_job.update_time
```

---


### Instance_detail

Get a list of instance details for a given patch job.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A pagination token that can be used to get the next page of results. |
| `patch_job_instance_details` | Vec<String> | A list of instance status. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access instance_detail outputs
instance_detail_id = instance_detail.id
instance_detail_next_page_token = instance_detail.next_page_token
instance_detail_patch_job_instance_details = instance_detail.patch_job_instance_details
```

---


### Instance

Lookup the effective guest policy that applies to a VM instance. This lookup merges all policies that are assigned to the instance ancestry.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `os_architecture` | String |  | Architecture of OS running on the instance. The OS Config agent only provides this field for targeting if OS Inventory is enabled for that instance. |
| `os_short_name` | String |  | Short name of the OS running on the instance. The OS Config agent only provides this field for targeting if OS Inventory is enabled for that instance. |
| `os_version` | String |  | Version of the OS running on the instance. The OS Config agent only provides this field for targeting if OS Inventory is enabled for that VM instance. |
| `instance` | String | ✅ | Required. The VM instance whose policies are being looked up. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.osconfig_api.Instance {
    instance = "value"  # Required. The VM instance whose policies are being looked up.
}

```

---


### Patch_deployment

Create an OS Config patch deployment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time the patch deployment was created. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `name` | String |  | Unique name for the patch deployment resource in a project. The patch deployment name is in the form: `projects/{project_id}/patchDeployments/{patch_deployment_id}`. This field is ignored when you create a new patch deployment. |
| `instance_filter` | String |  | Required. VM instances to patch. |
| `state` | String |  | Output only. Current state of the patch deployment. |
| `update_time` | String |  | Output only. Time the patch deployment was last updated. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `duration` | String |  | Optional. Duration of the patch. After the duration ends, the patch times out. |
| `description` | String |  | Optional. Description of the patch deployment. Length of the description is limited to 1024 characters. |
| `one_time_schedule` | String |  | Required. Schedule a one-time execution. |
| `recurring_schedule` | String |  | Required. Schedule recurring executions. |
| `last_execute_time` | String |  | Output only. The last time a patch job was started by this deployment. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `rollout` | String |  | Optional. Rollout strategy of the patch job. |
| `patch_config` | String |  | Optional. Patch configuration that is applied. |
| `parent` | String | ✅ | Required. The project to apply this patch deployment to in the form `projects/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time the patch deployment was created. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `name` | String | Unique name for the patch deployment resource in a project. The patch deployment name is in the form: `projects/{project_id}/patchDeployments/{patch_deployment_id}`. This field is ignored when you create a new patch deployment. |
| `instance_filter` | String | Required. VM instances to patch. |
| `state` | String | Output only. Current state of the patch deployment. |
| `update_time` | String | Output only. Time the patch deployment was last updated. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `duration` | String | Optional. Duration of the patch. After the duration ends, the patch times out. |
| `description` | String | Optional. Description of the patch deployment. Length of the description is limited to 1024 characters. |
| `one_time_schedule` | String | Required. Schedule a one-time execution. |
| `recurring_schedule` | String | Required. Schedule recurring executions. |
| `last_execute_time` | String | Output only. The last time a patch job was started by this deployment. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format. |
| `rollout` | String | Optional. Rollout strategy of the patch job. |
| `patch_config` | String | Optional. Patch configuration that is applied. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create patch_deployment
patch_deployment = provider.osconfig_api.Patch_deployment {
    parent = "value"  # Required. The project to apply this patch deployment to in the form `projects/*`.
}

# Access patch_deployment outputs
patch_deployment_id = patch_deployment.id
patch_deployment_create_time = patch_deployment.create_time
patch_deployment_name = patch_deployment.name
patch_deployment_instance_filter = patch_deployment.instance_filter
patch_deployment_state = patch_deployment.state
patch_deployment_update_time = patch_deployment.update_time
patch_deployment_duration = patch_deployment.duration
patch_deployment_description = patch_deployment.description
patch_deployment_one_time_schedule = patch_deployment.one_time_schedule
patch_deployment_recurring_schedule = patch_deployment.recurring_schedule
patch_deployment_last_execute_time = patch_deployment.last_execute_time
patch_deployment_rollout = patch_deployment.rollout
patch_deployment_patch_config = patch_deployment.patch_config
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple os_policy_assignment resources
os_policy_assignment_0 = provider.osconfig_api.Os_policy_assignment {
    parent = "value-0"
}
os_policy_assignment_1 = provider.osconfig_api.Os_policy_assignment {
    parent = "value-1"
}
os_policy_assignment_2 = provider.osconfig_api.Os_policy_assignment {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    os_policy_assignment = provider.osconfig_api.Os_policy_assignment {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Osconfig_api Documentation](https://cloud.google.com/osconfig_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
