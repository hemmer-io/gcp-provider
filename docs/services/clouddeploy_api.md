# Clouddeploy_api Service



**Resources**: 11

---

## Overview

The clouddeploy_api service provides access to 11 resource types:

- [Custom_target_type](#custom_target_type) [CRUD]
- [Job_run](#job_run) [CR]
- [Rollout](#rollout) [CR]
- [Deploy_policie](#deploy_policie) [CRUD]
- [Automation_run](#automation_run) [CR]
- [Target](#target) [CRUD]
- [Delivery_pipeline](#delivery_pipeline) [CRUD]
- [Operation](#operation) [CRD]
- [Release](#release) [CR]
- [Automation](#automation) [CRUD]
- [Location](#location) [R]

---

## Resources


### Custom_target_type

Creates a new CustomTargetType in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time at which the `CustomTargetType` was created. |
| `etag` | String |  | Optional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `custom_actions` | String |  | Optional. Configures render and deploy for the `CustomTargetType` using Skaffold custom actions. |
| `labels` | HashMap<String, String> |  | Optional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes. |
| `update_time` | String |  | Output only. Most recent time at which the `CustomTargetType` was updated. |
| `custom_target_type_id` | String |  | Output only. Resource id of the `CustomTargetType`. |
| `annotations` | HashMap<String, String> |  | Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations. |
| `name` | String |  | Identifier. Name of the `CustomTargetType`. Format is `projects/{project}/locations/{location}/customTargetTypes/{customTargetType}`. The `customTargetType` component must match `[a-z]([a-z0-9-]{0,61}[a-z0-9])?` |
| `description` | String |  | Optional. Description of the `CustomTargetType`. Max length is 255 characters. |
| `uid` | String |  | Output only. Unique identifier of the `CustomTargetType`. |
| `parent` | String | ✅ | Required. The parent collection in which the `CustomTargetType` must be created. The format is `projects/{project_id}/locations/{location_name}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time at which the `CustomTargetType` was created. |
| `etag` | String | Optional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `custom_actions` | String | Optional. Configures render and deploy for the `CustomTargetType` using Skaffold custom actions. |
| `labels` | HashMap<String, String> | Optional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes. |
| `update_time` | String | Output only. Most recent time at which the `CustomTargetType` was updated. |
| `custom_target_type_id` | String | Output only. Resource id of the `CustomTargetType`. |
| `annotations` | HashMap<String, String> | Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations. |
| `name` | String | Identifier. Name of the `CustomTargetType`. Format is `projects/{project}/locations/{location}/customTargetTypes/{customTargetType}`. The `customTargetType` component must match `[a-z]([a-z0-9-]{0,61}[a-z0-9])?` |
| `description` | String | Optional. Description of the `CustomTargetType`. Max length is 255 characters. |
| `uid` | String | Output only. Unique identifier of the `CustomTargetType`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_target_type
custom_target_type = provider.clouddeploy_api.Custom_target_type {
    parent = "value"  # Required. The parent collection in which the `CustomTargetType` must be created. The format is `projects/{project_id}/locations/{location_name}`.
}

# Access custom_target_type outputs
custom_target_type_id = custom_target_type.id
custom_target_type_create_time = custom_target_type.create_time
custom_target_type_etag = custom_target_type.etag
custom_target_type_custom_actions = custom_target_type.custom_actions
custom_target_type_labels = custom_target_type.labels
custom_target_type_update_time = custom_target_type.update_time
custom_target_type_custom_target_type_id = custom_target_type.custom_target_type_id
custom_target_type_annotations = custom_target_type.annotations
custom_target_type_name = custom_target_type.name
custom_target_type_description = custom_target_type.description
custom_target_type_uid = custom_target_type.uid
```

---


### Job_run

Terminates a Job Run in a given project and location.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `override_deploy_policy` | Vec<String> |  | Optional. Deploy policies to override. Format is `projects/{project}/locations/{location}/deployPolicies/{deployPolicy}`. |
| `name` | String | ✅ | Required. Name of the `JobRun`. Format must be `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/{rollout}/jobRuns/{jobRun}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The current state of the `JobRun`. |
| `predeploy_job_run` | String | Output only. Information specific to a predeploy `JobRun`. |
| `job_id` | String | Output only. ID of the `Rollout` job this `JobRun` corresponds to. |
| `verify_job_run` | String | Output only. Information specific to a verify `JobRun`. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `deploy_job_run` | String | Output only. Information specific to a deploy `JobRun`. |
| `postdeploy_job_run` | String | Output only. Information specific to a postdeploy `JobRun`. |
| `end_time` | String | Output only. Time at which the `JobRun` ended. |
| `uid` | String | Output only. Unique identifier of the `JobRun`. |
| `create_child_rollout_job_run` | String | Output only. Information specific to a createChildRollout `JobRun`. |
| `advance_child_rollout_job_run` | String | Output only. Information specific to an advanceChildRollout `JobRun` |
| `start_time` | String | Output only. Time at which the `JobRun` was started. |
| `phase_id` | String | Output only. ID of the `Rollout` phase this `JobRun` belongs in. |
| `create_time` | String | Output only. Time at which the `JobRun` was created. |
| `name` | String | Output only. Name of the `JobRun`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{releases}/rollouts/{rollouts}/jobRuns/{uuid}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job_run
job_run = provider.clouddeploy_api.Job_run {
    name = "value"  # Required. Name of the `JobRun`. Format must be `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/{rollout}/jobRuns/{jobRun}`.
}

# Access job_run outputs
job_run_id = job_run.id
job_run_state = job_run.state
job_run_predeploy_job_run = job_run.predeploy_job_run
job_run_job_id = job_run.job_id
job_run_verify_job_run = job_run.verify_job_run
job_run_etag = job_run.etag
job_run_deploy_job_run = job_run.deploy_job_run
job_run_postdeploy_job_run = job_run.postdeploy_job_run
job_run_end_time = job_run.end_time
job_run_uid = job_run.uid
job_run_create_child_rollout_job_run = job_run.create_child_rollout_job_run
job_run_advance_child_rollout_job_run = job_run.advance_child_rollout_job_run
job_run_start_time = job_run.start_time
job_run_phase_id = job_run.phase_id
job_run_create_time = job_run.create_time
job_run_name = job_run.name
```

---


### Rollout

Creates a new Rollout in a given project and location.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. Description of the `Rollout` for user purposes. Max length is 255 characters. |
| `active_repair_automation_run` | String |  | Output only. The AutomationRun actively repairing the rollout. |
| `phases` | Vec<String> |  | Output only. The phases that represent the workflows of this `Rollout`. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `approve_time` | String |  | Output only. Time at which the `Rollout` was approved. |
| `deploy_start_time` | String |  | Output only. Time at which the `Rollout` started deploying. |
| `target_id` | String |  | Required. The ID of Target to which this `Rollout` is deploying. |
| `enqueue_time` | String |  | Output only. Time at which the `Rollout` was enqueued. |
| `metadata` | String |  | Output only. Metadata contains information about the rollout. |
| `approval_state` | String |  | Output only. Approval state of the `Rollout`. |
| `rollback_of_rollout` | String |  | Output only. Name of the `Rollout` that is rolled back by this `Rollout`. Empty if this `Rollout` wasn't created as a rollback. |
| `name` | String |  | Identifier. Name of the `Rollout`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/{rollout}`. The `rollout` component must match `[a-z]([a-z0-9-]{0,61}[a-z0-9])?` |
| `controller_rollout` | String |  | Output only. Name of the `ControllerRollout`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/{rollout}`. |
| `uid` | String |  | Output only. Unique identifier of the `Rollout`. |
| `deploy_failure_cause` | String |  | Output only. The reason this rollout failed. This will always be unspecified while the rollout is in progress. |
| `deploy_end_time` | String |  | Output only. Time at which the `Rollout` finished deploying. |
| `create_time` | String |  | Output only. Time at which the `Rollout` was created. |
| `labels` | HashMap<String, String> |  | Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes. |
| `rolled_back_by_rollouts` | Vec<String> |  | Output only. Names of `Rollouts` that rolled back this `Rollout`. |
| `state` | String |  | Output only. Current state of the `Rollout`. |
| `failure_reason` | String |  | Output only. Additional information about the rollout failure, if available. |
| `annotations` | HashMap<String, String> |  | Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations. |
| `deploying_build` | String |  | Output only. The resource name of the Cloud Build `Build` object that is used to deploy the Rollout. Format is `projects/{project}/locations/{location}/builds/{build}`. |
| `parent` | String | ✅ | Required. The parent collection in which the `Rollout` must be created. The format is `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}/releases/{release_name}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. Description of the `Rollout` for user purposes. Max length is 255 characters. |
| `active_repair_automation_run` | String | Output only. The AutomationRun actively repairing the rollout. |
| `phases` | Vec<String> | Output only. The phases that represent the workflows of this `Rollout`. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `approve_time` | String | Output only. Time at which the `Rollout` was approved. |
| `deploy_start_time` | String | Output only. Time at which the `Rollout` started deploying. |
| `target_id` | String | Required. The ID of Target to which this `Rollout` is deploying. |
| `enqueue_time` | String | Output only. Time at which the `Rollout` was enqueued. |
| `metadata` | String | Output only. Metadata contains information about the rollout. |
| `approval_state` | String | Output only. Approval state of the `Rollout`. |
| `rollback_of_rollout` | String | Output only. Name of the `Rollout` that is rolled back by this `Rollout`. Empty if this `Rollout` wasn't created as a rollback. |
| `name` | String | Identifier. Name of the `Rollout`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/{rollout}`. The `rollout` component must match `[a-z]([a-z0-9-]{0,61}[a-z0-9])?` |
| `controller_rollout` | String | Output only. Name of the `ControllerRollout`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/{rollout}`. |
| `uid` | String | Output only. Unique identifier of the `Rollout`. |
| `deploy_failure_cause` | String | Output only. The reason this rollout failed. This will always be unspecified while the rollout is in progress. |
| `deploy_end_time` | String | Output only. Time at which the `Rollout` finished deploying. |
| `create_time` | String | Output only. Time at which the `Rollout` was created. |
| `labels` | HashMap<String, String> | Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes. |
| `rolled_back_by_rollouts` | Vec<String> | Output only. Names of `Rollouts` that rolled back this `Rollout`. |
| `state` | String | Output only. Current state of the `Rollout`. |
| `failure_reason` | String | Output only. Additional information about the rollout failure, if available. |
| `annotations` | HashMap<String, String> | Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations. |
| `deploying_build` | String | Output only. The resource name of the Cloud Build `Build` object that is used to deploy the Rollout. Format is `projects/{project}/locations/{location}/builds/{build}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rollout
rollout = provider.clouddeploy_api.Rollout {
    parent = "value"  # Required. The parent collection in which the `Rollout` must be created. The format is `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}/releases/{release_name}`.
}

# Access rollout outputs
rollout_id = rollout.id
rollout_description = rollout.description
rollout_active_repair_automation_run = rollout.active_repair_automation_run
rollout_phases = rollout.phases
rollout_etag = rollout.etag
rollout_approve_time = rollout.approve_time
rollout_deploy_start_time = rollout.deploy_start_time
rollout_target_id = rollout.target_id
rollout_enqueue_time = rollout.enqueue_time
rollout_metadata = rollout.metadata
rollout_approval_state = rollout.approval_state
rollout_rollback_of_rollout = rollout.rollback_of_rollout
rollout_name = rollout.name
rollout_controller_rollout = rollout.controller_rollout
rollout_uid = rollout.uid
rollout_deploy_failure_cause = rollout.deploy_failure_cause
rollout_deploy_end_time = rollout.deploy_end_time
rollout_create_time = rollout.create_time
rollout_labels = rollout.labels
rollout_rolled_back_by_rollouts = rollout.rolled_back_by_rollouts
rollout_state = rollout.state
rollout_failure_reason = rollout.failure_reason
rollout_annotations = rollout.annotations
rollout_deploying_build = rollout.deploying_build
```

---


### Deploy_policie

Creates a new DeployPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. Description of the `DeployPolicy`. Max length is 255 characters. |
| `labels` | HashMap<String, String> |  | Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes. |
| `create_time` | String |  | Output only. Time at which the deploy policy was created. |
| `selectors` | Vec<String> |  | Required. Selected resources to which the policy will be applied. At least one selector is required. If one selector matches the resource the policy applies. For example, if there are two selectors and the action being attempted matches one of them, the policy will apply to that action. |
| `update_time` | String |  | Output only. Most recent time at which the deploy policy was updated. |
| `uid` | String |  | Output only. Unique identifier of the `DeployPolicy`. |
| `annotations` | HashMap<String, String> |  | Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash (`/`). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character (`[a-z0-9A-Z]`) with dashes (`-`), underscores (`_`), dots (`.`), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots(`.`), not longer than 253 characters in total, followed by a slash (`/`). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details. |
| `suspended` | bool |  | Optional. When suspended, the policy will not prevent actions from occurring, even if the action violates the policy. |
| `name` | String |  | Output only. Name of the `DeployPolicy`. Format is `projects/{project}/locations/{location}/deployPolicies/{deployPolicy}`. The `deployPolicy` component must match `[a-z]([a-z0-9-]{0,61}[a-z0-9])?` |
| `rules` | Vec<String> |  | Required. Rules to apply. At least one rule must be present. |
| `etag` | String |  | The weak etag of the `DeployPolicy` resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `parent` | String | ✅ | Required. The parent collection in which the `DeployPolicy` must be created. The format is `projects/{project_id}/locations/{location_name}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. Description of the `DeployPolicy`. Max length is 255 characters. |
| `labels` | HashMap<String, String> | Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes. |
| `create_time` | String | Output only. Time at which the deploy policy was created. |
| `selectors` | Vec<String> | Required. Selected resources to which the policy will be applied. At least one selector is required. If one selector matches the resource the policy applies. For example, if there are two selectors and the action being attempted matches one of them, the policy will apply to that action. |
| `update_time` | String | Output only. Most recent time at which the deploy policy was updated. |
| `uid` | String | Output only. Unique identifier of the `DeployPolicy`. |
| `annotations` | HashMap<String, String> | Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash (`/`). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character (`[a-z0-9A-Z]`) with dashes (`-`), underscores (`_`), dots (`.`), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots(`.`), not longer than 253 characters in total, followed by a slash (`/`). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details. |
| `suspended` | bool | Optional. When suspended, the policy will not prevent actions from occurring, even if the action violates the policy. |
| `name` | String | Output only. Name of the `DeployPolicy`. Format is `projects/{project}/locations/{location}/deployPolicies/{deployPolicy}`. The `deployPolicy` component must match `[a-z]([a-z0-9-]{0,61}[a-z0-9])?` |
| `rules` | Vec<String> | Required. Rules to apply. At least one rule must be present. |
| `etag` | String | The weak etag of the `DeployPolicy` resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create deploy_policie
deploy_policie = provider.clouddeploy_api.Deploy_policie {
    parent = "value"  # Required. The parent collection in which the `DeployPolicy` must be created. The format is `projects/{project_id}/locations/{location_name}`.
}

# Access deploy_policie outputs
deploy_policie_id = deploy_policie.id
deploy_policie_description = deploy_policie.description
deploy_policie_labels = deploy_policie.labels
deploy_policie_create_time = deploy_policie.create_time
deploy_policie_selectors = deploy_policie.selectors
deploy_policie_update_time = deploy_policie.update_time
deploy_policie_uid = deploy_policie.uid
deploy_policie_annotations = deploy_policie.annotations
deploy_policie_suspended = deploy_policie.suspended
deploy_policie_name = deploy_policie.name
deploy_policie_rules = deploy_policie.rules
deploy_policie_etag = deploy_policie.etag
```

---


### Automation_run

Cancels an AutomationRun. The `state` of the `AutomationRun` after cancelling is `CANCELLED`. `CancelAutomationRun` can be called on AutomationRun in the state `IN_PROGRESS` and `PENDING`; AutomationRun in a different state returns an `FAILED_PRECONDITION` error.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. Name of the `AutomationRun`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{delivery_pipeline}/automationRuns/{automation_run}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time at which the `AutomationRun` was created. |
| `automation_id` | String | Output only. The ID of the automation that initiated the operation. |
| `name` | String | Output only. Name of the `AutomationRun`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{delivery_pipeline}/automationRuns/{automation_run}`. |
| `service_account` | String | Output only. Email address of the user-managed IAM service account that performs the operations against Cloud Deploy resources. |
| `automation_snapshot` | String | Output only. Snapshot of the Automation taken at AutomationRun creation time. |
| `expire_time` | String | Output only. Time the `AutomationRun` expires. An `AutomationRun` expires after 14 days from its creation date. |
| `rule_id` | String | Output only. The ID of the automation rule that initiated the operation. |
| `advance_rollout_operation` | String | Output only. Advances a rollout to the next phase. |
| `state_description` | String | Output only. Explains the current state of the `AutomationRun`. Present only when an explanation is needed. |
| `state` | String | Output only. Current state of the `AutomationRun`. |
| `target_id` | String | Output only. The ID of the source target that initiates the `AutomationRun`. The value of this field is the last segment of a target name. |
| `timed_promote_release_operation` | String | Output only. Promotes a release to a specified 'Target' as defined in a Timed Promote Release rule. |
| `update_time` | String | Output only. Time at which the automationRun was updated. |
| `wait_until_time` | String | Output only. Earliest time the `AutomationRun` will attempt to resume. Wait-time is configured by `wait` in automation rule. |
| `promote_release_operation` | String | Output only. Promotes a release to a specified 'Target'. |
| `repair_rollout_operation` | String | Output only. Repairs a failed 'Rollout'. |
| `etag` | String | Output only. The weak etag of the `AutomationRun` resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `policy_violation` | String | Output only. Contains information about what policies prevented the `AutomationRun` from proceeding. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create automation_run
automation_run = provider.clouddeploy_api.Automation_run {
    name = "value"  # Required. Name of the `AutomationRun`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{delivery_pipeline}/automationRuns/{automation_run}`.
}

# Access automation_run outputs
automation_run_id = automation_run.id
automation_run_create_time = automation_run.create_time
automation_run_automation_id = automation_run.automation_id
automation_run_name = automation_run.name
automation_run_service_account = automation_run.service_account
automation_run_automation_snapshot = automation_run.automation_snapshot
automation_run_expire_time = automation_run.expire_time
automation_run_rule_id = automation_run.rule_id
automation_run_advance_rollout_operation = automation_run.advance_rollout_operation
automation_run_state_description = automation_run.state_description
automation_run_state = automation_run.state
automation_run_target_id = automation_run.target_id
automation_run_timed_promote_release_operation = automation_run.timed_promote_release_operation
automation_run_update_time = automation_run.update_time
automation_run_wait_until_time = automation_run.wait_until_time
automation_run_promote_release_operation = automation_run.promote_release_operation
automation_run_repair_rollout_operation = automation_run.repair_rollout_operation
automation_run_etag = automation_run.etag
automation_run_policy_violation = automation_run.policy_violation
```

---


### Target

Creates a new Target in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `anthos_cluster` | String |  | Optional. Information specifying an Anthos Cluster. |
| `target_id` | String |  | Output only. Resource id of the `Target`. |
| `associated_entities` | HashMap<String, String> |  | Optional. Map of entity IDs to their associated entities. Associated entities allows specifying places other than the deployment target for specific features. For example, the Gateway API canary can be configured to deploy the HTTPRoute to a different cluster(s) than the deployment cluster using associated entities. An entity ID must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$`. |
| `run` | String |  | Optional. Information specifying a Cloud Run deployment target. |
| `update_time` | String |  | Output only. Most recent time at which the `Target` was updated. |
| `create_time` | String |  | Output only. Time at which the `Target` was created. |
| `custom_target` | String |  | Optional. Information specifying a Custom Target. |
| `execution_configs` | Vec<String> |  | Optional. Configurations for all execution that relates to this `Target`. Each `ExecutionEnvironmentUsage` value may only be used in a single configuration; using the same value multiple times is an error. When one or more configurations are specified, they must include the `RENDER` and `DEPLOY` `ExecutionEnvironmentUsage` values. When no configurations are specified, execution will use the default specified in `DefaultPool`. |
| `require_approval` | bool |  | Optional. Whether or not the `Target` requires approval. |
| `deploy_parameters` | HashMap<String, String> |  | Optional. The deploy parameters to use for this target. |
| `description` | String |  | Optional. Description of the `Target`. Max length is 255 characters. |
| `name` | String |  | Identifier. Name of the `Target`. Format is `projects/{project}/locations/{location}/targets/{target}`. The `target` component must match `[a-z]([a-z0-9-]{0,61}[a-z0-9])?` |
| `gke` | String |  | Optional. Information specifying a GKE Cluster. |
| `multi_target` | String |  | Optional. Information specifying a multiTarget. |
| `uid` | String |  | Output only. Unique identifier of the `Target`. |
| `labels` | HashMap<String, String> |  | Optional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes. |
| `annotations` | HashMap<String, String> |  | Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations. |
| `etag` | String |  | Optional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `parent` | String | ✅ | Required. The parent collection in which the `Target` must be created. The format is `projects/{project_id}/locations/{location_name}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `anthos_cluster` | String | Optional. Information specifying an Anthos Cluster. |
| `target_id` | String | Output only. Resource id of the `Target`. |
| `associated_entities` | HashMap<String, String> | Optional. Map of entity IDs to their associated entities. Associated entities allows specifying places other than the deployment target for specific features. For example, the Gateway API canary can be configured to deploy the HTTPRoute to a different cluster(s) than the deployment cluster using associated entities. An entity ID must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$`. |
| `run` | String | Optional. Information specifying a Cloud Run deployment target. |
| `update_time` | String | Output only. Most recent time at which the `Target` was updated. |
| `create_time` | String | Output only. Time at which the `Target` was created. |
| `custom_target` | String | Optional. Information specifying a Custom Target. |
| `execution_configs` | Vec<String> | Optional. Configurations for all execution that relates to this `Target`. Each `ExecutionEnvironmentUsage` value may only be used in a single configuration; using the same value multiple times is an error. When one or more configurations are specified, they must include the `RENDER` and `DEPLOY` `ExecutionEnvironmentUsage` values. When no configurations are specified, execution will use the default specified in `DefaultPool`. |
| `require_approval` | bool | Optional. Whether or not the `Target` requires approval. |
| `deploy_parameters` | HashMap<String, String> | Optional. The deploy parameters to use for this target. |
| `description` | String | Optional. Description of the `Target`. Max length is 255 characters. |
| `name` | String | Identifier. Name of the `Target`. Format is `projects/{project}/locations/{location}/targets/{target}`. The `target` component must match `[a-z]([a-z0-9-]{0,61}[a-z0-9])?` |
| `gke` | String | Optional. Information specifying a GKE Cluster. |
| `multi_target` | String | Optional. Information specifying a multiTarget. |
| `uid` | String | Output only. Unique identifier of the `Target`. |
| `labels` | HashMap<String, String> | Optional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes. |
| `annotations` | HashMap<String, String> | Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations. |
| `etag` | String | Optional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create target
target = provider.clouddeploy_api.Target {
    parent = "value"  # Required. The parent collection in which the `Target` must be created. The format is `projects/{project_id}/locations/{location_name}`.
}

# Access target outputs
target_id = target.id
target_anthos_cluster = target.anthos_cluster
target_target_id = target.target_id
target_associated_entities = target.associated_entities
target_run = target.run
target_update_time = target.update_time
target_create_time = target.create_time
target_custom_target = target.custom_target
target_execution_configs = target.execution_configs
target_require_approval = target.require_approval
target_deploy_parameters = target.deploy_parameters
target_description = target.description
target_name = target.name
target_gke = target.gke
target_multi_target = target.multi_target
target_uid = target.uid
target_labels = target.labels
target_annotations = target.annotations
target_etag = target.etag
```

---


### Delivery_pipeline

Creates a new DeliveryPipeline in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `serial_pipeline` | String |  | Optional. SerialPipeline defines a sequential set of stages for a `DeliveryPipeline`. |
| `description` | String |  | Optional. Description of the `DeliveryPipeline`. Max length is 255 characters. |
| `update_time` | String |  | Output only. Most recent time at which the pipeline was updated. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `annotations` | HashMap<String, String> |  | Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. |
| `name` | String |  | Identifier. Name of the `DeliveryPipeline`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}`. The `deliveryPipeline` component must match `[a-z]([a-z0-9-]{0,61}[a-z0-9])?` |
| `suspended` | bool |  | Optional. When suspended, no new releases or rollouts can be created, but in-progress ones will complete. |
| `condition` | String |  | Output only. Information around the state of the Delivery Pipeline. |
| `uid` | String |  | Output only. Unique identifier of the `DeliveryPipeline`. |
| `create_time` | String |  | Output only. Time at which the pipeline was created. |
| `labels` | HashMap<String, String> |  | Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes. |
| `parent` | String | ✅ | Required. The parent collection in which the `DeliveryPipeline` must be created. The format is `projects/{project_id}/locations/{location_name}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `serial_pipeline` | String | Optional. SerialPipeline defines a sequential set of stages for a `DeliveryPipeline`. |
| `description` | String | Optional. Description of the `DeliveryPipeline`. Max length is 255 characters. |
| `update_time` | String | Output only. Most recent time at which the pipeline was updated. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `annotations` | HashMap<String, String> | Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. |
| `name` | String | Identifier. Name of the `DeliveryPipeline`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}`. The `deliveryPipeline` component must match `[a-z]([a-z0-9-]{0,61}[a-z0-9])?` |
| `suspended` | bool | Optional. When suspended, no new releases or rollouts can be created, but in-progress ones will complete. |
| `condition` | String | Output only. Information around the state of the Delivery Pipeline. |
| `uid` | String | Output only. Unique identifier of the `DeliveryPipeline`. |
| `create_time` | String | Output only. Time at which the pipeline was created. |
| `labels` | HashMap<String, String> | Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create delivery_pipeline
delivery_pipeline = provider.clouddeploy_api.Delivery_pipeline {
    parent = "value"  # Required. The parent collection in which the `DeliveryPipeline` must be created. The format is `projects/{project_id}/locations/{location_name}`.
}

# Access delivery_pipeline outputs
delivery_pipeline_id = delivery_pipeline.id
delivery_pipeline_serial_pipeline = delivery_pipeline.serial_pipeline
delivery_pipeline_description = delivery_pipeline.description
delivery_pipeline_update_time = delivery_pipeline.update_time
delivery_pipeline_etag = delivery_pipeline.etag
delivery_pipeline_annotations = delivery_pipeline.annotations
delivery_pipeline_name = delivery_pipeline.name
delivery_pipeline_suspended = delivery_pipeline.suspended
delivery_pipeline_condition = delivery_pipeline.condition
delivery_pipeline_uid = delivery_pipeline.uid
delivery_pipeline_create_time = delivery_pipeline.create_time
delivery_pipeline_labels = delivery_pipeline.labels
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.clouddeploy_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_error = operation.error
operation_name = operation.name
operation_done = operation.done
operation_metadata = operation.metadata
```

---


### Release

Creates a new Release in a given project and location.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `annotations` | HashMap<String, String> |  | Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations. |
| `name` | String |  | Identifier. Name of the `Release`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}`. The `release` component must match `[a-z]([a-z0-9-]{0,61}[a-z0-9])?` |
| `abandoned` | bool |  | Output only. Indicates whether this is an abandoned release. |
| `delivery_pipeline_snapshot` | String |  | Output only. Snapshot of the parent pipeline taken at release creation time. |
| `render_end_time` | String |  | Output only. Time at which the render completed. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `custom_target_type_snapshots` | Vec<String> |  | Output only. Snapshot of the custom target types referenced by the targets taken at release creation time. |
| `description` | String |  | Optional. Description of the `Release`. Max length is 255 characters. |
| `render_state` | String |  | Output only. Current state of the render operation. |
| `target_artifacts` | HashMap<String, String> |  | Output only. Map from target ID to the target artifacts created during the render operation. |
| `target_renders` | HashMap<String, String> |  | Output only. Map from target ID to details of the render operation for that target. |
| `target_snapshots` | Vec<String> |  | Output only. Snapshot of the targets taken at release creation time. |
| `build_artifacts` | Vec<String> |  | Optional. List of artifacts to pass through to Skaffold command. |
| `labels` | HashMap<String, String> |  | Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes. |
| `skaffold_config_path` | String |  | Optional. Filepath of the Skaffold config inside of the config URI. |
| `skaffold_config_uri` | String |  | Optional. Cloud Storage URI of tar.gz archive containing Skaffold configuration. |
| `condition` | String |  | Output only. Information around the state of the Release. |
| `create_time` | String |  | Output only. Time at which the `Release` was created. |
| `deploy_parameters` | HashMap<String, String> |  | Optional. The deploy parameters to use for all targets in this release. |
| `uid` | String |  | Output only. Unique identifier of the `Release`. |
| `skaffold_version` | String |  | Optional. The Skaffold version to use when operating on this release, such as "1.20.0". Not all versions are valid; Cloud Deploy supports a specific set of versions. If unset, the most recent supported Skaffold version will be used. |
| `render_start_time` | String |  | Output only. Time at which the render began. |
| `parent` | String | ✅ | Required. The parent collection in which the `Release` is created. The format is `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotations` | HashMap<String, String> | Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations. |
| `name` | String | Identifier. Name of the `Release`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}`. The `release` component must match `[a-z]([a-z0-9-]{0,61}[a-z0-9])?` |
| `abandoned` | bool | Output only. Indicates whether this is an abandoned release. |
| `delivery_pipeline_snapshot` | String | Output only. Snapshot of the parent pipeline taken at release creation time. |
| `render_end_time` | String | Output only. Time at which the render completed. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `custom_target_type_snapshots` | Vec<String> | Output only. Snapshot of the custom target types referenced by the targets taken at release creation time. |
| `description` | String | Optional. Description of the `Release`. Max length is 255 characters. |
| `render_state` | String | Output only. Current state of the render operation. |
| `target_artifacts` | HashMap<String, String> | Output only. Map from target ID to the target artifacts created during the render operation. |
| `target_renders` | HashMap<String, String> | Output only. Map from target ID to details of the render operation for that target. |
| `target_snapshots` | Vec<String> | Output only. Snapshot of the targets taken at release creation time. |
| `build_artifacts` | Vec<String> | Optional. List of artifacts to pass through to Skaffold command. |
| `labels` | HashMap<String, String> | Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes. |
| `skaffold_config_path` | String | Optional. Filepath of the Skaffold config inside of the config URI. |
| `skaffold_config_uri` | String | Optional. Cloud Storage URI of tar.gz archive containing Skaffold configuration. |
| `condition` | String | Output only. Information around the state of the Release. |
| `create_time` | String | Output only. Time at which the `Release` was created. |
| `deploy_parameters` | HashMap<String, String> | Optional. The deploy parameters to use for all targets in this release. |
| `uid` | String | Output only. Unique identifier of the `Release`. |
| `skaffold_version` | String | Optional. The Skaffold version to use when operating on this release, such as "1.20.0". Not all versions are valid; Cloud Deploy supports a specific set of versions. If unset, the most recent supported Skaffold version will be used. |
| `render_start_time` | String | Output only. Time at which the render began. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create release
release = provider.clouddeploy_api.Release {
    parent = "value"  # Required. The parent collection in which the `Release` is created. The format is `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}`.
}

# Access release outputs
release_id = release.id
release_annotations = release.annotations
release_name = release.name
release_abandoned = release.abandoned
release_delivery_pipeline_snapshot = release.delivery_pipeline_snapshot
release_render_end_time = release.render_end_time
release_etag = release.etag
release_custom_target_type_snapshots = release.custom_target_type_snapshots
release_description = release.description
release_render_state = release.render_state
release_target_artifacts = release.target_artifacts
release_target_renders = release.target_renders
release_target_snapshots = release.target_snapshots
release_build_artifacts = release.build_artifacts
release_labels = release.labels
release_skaffold_config_path = release.skaffold_config_path
release_skaffold_config_uri = release.skaffold_config_uri
release_condition = release.condition
release_create_time = release.create_time
release_deploy_parameters = release.deploy_parameters
release_uid = release.uid
release_skaffold_version = release.skaffold_version
release_render_start_time = release.render_start_time
```

---


### Automation

Creates a new Automation in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rules` | Vec<String> |  | Required. List of Automation rules associated with the Automation resource. Must have at least one rule and limited to 250 rules per Delivery Pipeline. Note: the order of the rules here is not the same as the order of execution. |
| `selector` | String |  | Required. Selected resources to which the automation will be applied. |
| `uid` | String |  | Output only. Unique identifier of the `Automation`. |
| `annotations` | HashMap<String, String> |  | Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash (`/`). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character (`[a-z0-9A-Z]`) with dashes (`-`), underscores (`_`), dots (`.`), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots(`.`), not longer than 253 characters in total, followed by a slash (`/`). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details. |
| `create_time` | String |  | Output only. Time at which the automation was created. |
| `etag` | String |  | Optional. The weak etag of the `Automation` resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `labels` | HashMap<String, String> |  | Optional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 63 characters. |
| `service_account` | String |  | Required. Email address of the user-managed IAM service account that creates Cloud Deploy release and rollout resources. |
| `suspended` | bool |  | Optional. When Suspended, automation is deactivated from execution. |
| `update_time` | String |  | Output only. Time at which the automation was updated. |
| `name` | String |  | Output only. Name of the `Automation`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{delivery_pipeline}/automations/{automation}`. |
| `description` | String |  | Optional. Description of the `Automation`. Max length is 255 characters. |
| `parent` | String | ✅ | Required. The parent collection in which the `Automation` must be created. The format is `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rules` | Vec<String> | Required. List of Automation rules associated with the Automation resource. Must have at least one rule and limited to 250 rules per Delivery Pipeline. Note: the order of the rules here is not the same as the order of execution. |
| `selector` | String | Required. Selected resources to which the automation will be applied. |
| `uid` | String | Output only. Unique identifier of the `Automation`. |
| `annotations` | HashMap<String, String> | Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash (`/`). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character (`[a-z0-9A-Z]`) with dashes (`-`), underscores (`_`), dots (`.`), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots(`.`), not longer than 253 characters in total, followed by a slash (`/`). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details. |
| `create_time` | String | Output only. Time at which the automation was created. |
| `etag` | String | Optional. The weak etag of the `Automation` resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `labels` | HashMap<String, String> | Optional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 63 characters. |
| `service_account` | String | Required. Email address of the user-managed IAM service account that creates Cloud Deploy release and rollout resources. |
| `suspended` | bool | Optional. When Suspended, automation is deactivated from execution. |
| `update_time` | String | Output only. Time at which the automation was updated. |
| `name` | String | Output only. Name of the `Automation`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{delivery_pipeline}/automations/{automation}`. |
| `description` | String | Optional. Description of the `Automation`. Max length is 255 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create automation
automation = provider.clouddeploy_api.Automation {
    parent = "value"  # Required. The parent collection in which the `Automation` must be created. The format is `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}`.
}

# Access automation outputs
automation_id = automation.id
automation_rules = automation.rules
automation_selector = automation.selector
automation_uid = automation.uid
automation_annotations = automation.annotations
automation_create_time = automation.create_time
automation_etag = automation.etag
automation_labels = automation.labels
automation_service_account = automation.service_account
automation_suspended = automation.suspended
automation_update_time = automation.update_time
automation_name = automation.name
automation_description = automation.description
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access location outputs
location_id = location.id
location_labels = location.labels
location_name = location.name
location_display_name = location.display_name
location_location_id = location.location_id
location_metadata = location.metadata
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple custom_target_type resources
custom_target_type_0 = provider.clouddeploy_api.Custom_target_type {
    parent = "value-0"
}
custom_target_type_1 = provider.clouddeploy_api.Custom_target_type {
    parent = "value-1"
}
custom_target_type_2 = provider.clouddeploy_api.Custom_target_type {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    custom_target_type = provider.clouddeploy_api.Custom_target_type {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Clouddeploy_api Documentation](https://cloud.google.com/clouddeploy_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
