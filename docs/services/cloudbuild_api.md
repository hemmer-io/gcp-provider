# Cloudbuild_api Service



**Resources**: 22

---

## Overview

The cloudbuild_api service provides access to 22 resource types:

- [Git_lab_config](#git_lab_config) [CRUD]
- [Github_enterprise_config](#github_enterprise_config) [CRUD]
- [Build](#build) [CR]
- [Trigger](#trigger) [CRUD]
- [Location](#location) [CR]
- [Connected_repositorie](#connected_repositorie) [C]
- [Repo](#repo) [R]
- [Operation](#operation) [CR]
- [Github_dot_com_webhook](#github_dot_com_webhook) [C]
- [Cloudbuild](#cloudbuild) [C]
- [Worker_pool](#worker_pool) [CRUD]
- [Bitbucket_server_config](#bitbucket_server_config) [CRUD]
- [Worker_pool](#worker_pool) [CRUD]
- [Operation](#operation) [CR]
- [Operation](#operation) [CR]
- [Repositorie](#repositorie) [CRD]
- [Connection](#connection) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CR]
- [Worker_pool](#worker_pool) [CRUD]
- [Operation](#operation) [CR]
- [Worker_pool](#worker_pool) [CRUD]

---

## Resources


### Git_lab_config

Creates a new `GitLabConfig`. This API is experimental

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The resource name for the config. |
| `secrets` | String |  | Required. Secret Manager secrets needed by the config. |
| `username` | String |  | Username of the GitLab.com or GitLab Enterprise account Cloud Build will use. |
| `webhook_key` | String |  | Output only. UUID included in webhook requests. The UUID is used to look up the corresponding config. |
| `create_time` | String |  | Output only. Time when the config was created. |
| `connected_repositories` | Vec<String> |  | Connected GitLab.com or GitLabEnterprise repositories for this config. |
| `enterprise_config` | String |  | Optional. GitLabEnterprise config. |
| `parent` | String | ✅ | Required. Name of the parent resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name for the config. |
| `secrets` | String | Required. Secret Manager secrets needed by the config. |
| `username` | String | Username of the GitLab.com or GitLab Enterprise account Cloud Build will use. |
| `webhook_key` | String | Output only. UUID included in webhook requests. The UUID is used to look up the corresponding config. |
| `create_time` | String | Output only. Time when the config was created. |
| `connected_repositories` | Vec<String> | Connected GitLab.com or GitLabEnterprise repositories for this config. |
| `enterprise_config` | String | Optional. GitLabEnterprise config. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create git_lab_config
git_lab_config = provider.cloudbuild_api.Git_lab_config {
    parent = "value"  # Required. Name of the parent resource.
}

# Access git_lab_config outputs
git_lab_config_id = git_lab_config.id
git_lab_config_name = git_lab_config.name
git_lab_config_secrets = git_lab_config.secrets
git_lab_config_username = git_lab_config.username
git_lab_config_webhook_key = git_lab_config.webhook_key
git_lab_config_create_time = git_lab_config.create_time
git_lab_config_connected_repositories = git_lab_config.connected_repositories
git_lab_config_enterprise_config = git_lab_config.enterprise_config
```

---


### Github_enterprise_config

Create an association between a GCP project and a GitHub Enterprise server.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `webhook_key` | String |  | The key that should be attached to webhook calls to the ReceiveWebhook endpoint. |
| `peered_network` | String |  | Optional. The network to be used when reaching out to the GitHub Enterprise server. The VPC network must be enabled for private service connection. This should be set if the GitHub Enterprise server is hosted on-premises and not reachable by public internet. If this field is left empty, no network peering will occur and calls to the GitHub Enterprise server will be made over the public internet. Must be in the format `projects/{project}/global/networks/{network}`, where {project} is a project number or id and {network} is the name of a VPC network in the project. |
| `display_name` | String |  | Optional. Name to display for this config. |
| `name` | String |  | The full resource name for the GitHubEnterpriseConfig For example: "projects/{$project_id}/locations/{$location_id}/githubEnterpriseConfigs/{$config_id}" |
| `app_id` | String |  | Required. The GitHub app id of the Cloud Build app on the GitHub Enterprise server. |
| `create_time` | String |  | Output only. Time when the installation was associated with the project. |
| `host_url` | String |  | The URL of the github enterprise host the configuration is for. |
| `ssl_ca` | String |  | Optional. SSL certificate to use for requests to GitHub Enterprise. |
| `secrets` | String |  | Optional. Names of secrets in Secret Manager. |
| `parent` | String | ✅ | Name of the parent project. For example: projects/{$project_number} or projects/{$project_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `webhook_key` | String | The key that should be attached to webhook calls to the ReceiveWebhook endpoint. |
| `peered_network` | String | Optional. The network to be used when reaching out to the GitHub Enterprise server. The VPC network must be enabled for private service connection. This should be set if the GitHub Enterprise server is hosted on-premises and not reachable by public internet. If this field is left empty, no network peering will occur and calls to the GitHub Enterprise server will be made over the public internet. Must be in the format `projects/{project}/global/networks/{network}`, where {project} is a project number or id and {network} is the name of a VPC network in the project. |
| `display_name` | String | Optional. Name to display for this config. |
| `name` | String | The full resource name for the GitHubEnterpriseConfig For example: "projects/{$project_id}/locations/{$location_id}/githubEnterpriseConfigs/{$config_id}" |
| `app_id` | String | Required. The GitHub app id of the Cloud Build app on the GitHub Enterprise server. |
| `create_time` | String | Output only. Time when the installation was associated with the project. |
| `host_url` | String | The URL of the github enterprise host the configuration is for. |
| `ssl_ca` | String | Optional. SSL certificate to use for requests to GitHub Enterprise. |
| `secrets` | String | Optional. Names of secrets in Secret Manager. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create github_enterprise_config
github_enterprise_config = provider.cloudbuild_api.Github_enterprise_config {
    parent = "value"  # Name of the parent project. For example: projects/{$project_number} or projects/{$project_id}
}

# Access github_enterprise_config outputs
github_enterprise_config_id = github_enterprise_config.id
github_enterprise_config_webhook_key = github_enterprise_config.webhook_key
github_enterprise_config_peered_network = github_enterprise_config.peered_network
github_enterprise_config_display_name = github_enterprise_config.display_name
github_enterprise_config_name = github_enterprise_config.name
github_enterprise_config_app_id = github_enterprise_config.app_id
github_enterprise_config_create_time = github_enterprise_config.create_time
github_enterprise_config_host_url = github_enterprise_config.host_url
github_enterprise_config_ssl_ca = github_enterprise_config.ssl_ca
github_enterprise_config_secrets = github_enterprise_config.secrets
```

---


### Build

Starts a build with the specified configuration. This method returns a long-running `Operation`, which includes the build ID. Pass the build ID to `GetBuild` to determine the build status (such as `SUCCESS` or `FAILURE`).

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `failure_info` | String |  | Output only. Contains information about the build when status=FAILURE. |
| `git_config` | String |  | Optional. Configuration for git operations. |
| `timing` | HashMap<String, String> |  | Output only. Stores timing information for phases of the build. Valid keys are: * BUILD: time to execute all build steps. * PUSH: time to push all artifacts including docker images and non docker artifacts. * FETCHSOURCE: time to fetch source. * SETUPBUILD: time to set up build. If the build does not specify source or images, these keys will not be included. |
| `options` | String |  | Special options for this build. |
| `build_trigger_id` | String |  | Output only. The ID of the `BuildTrigger` that triggered this build, if it was triggered automatically. |
| `finish_time` | String |  | Output only. Time at which execution of the build was finished. The difference between finish_time and start_time is the duration of the build's execution. |
| `start_time` | String |  | Output only. Time at which execution of the build was started. |
| `logs_bucket` | String |  | Cloud Storage bucket where logs should be written (see [Bucket Name Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)). Logs file names will be of the format `${logs_bucket}/log-${build_id}.txt`. |
| `steps` | Vec<String> |  | Required. The operations to be performed on the workspace. |
| `images` | Vec<String> |  | A list of images to be pushed upon the successful completion of all build steps. The images are pushed using the builder service account's credentials. The digests of the pushed images will be stored in the `Build` resource's results field. If any of the images fail to be pushed, the build status is marked `FAILURE`. |
| `timeout` | String |  | Amount of time that this build should be allowed to run, to second granularity. If this amount of time elapses, work on the build will cease and the build status will be `TIMEOUT`. `timeout` starts ticking from `startTime`. Default time is 60 minutes. |
| `create_time` | String |  | Output only. Time at which the request to create the build was received. |
| `project_id` | String |  | Output only. ID of the project. |
| `source_provenance` | String |  | Output only. A permanent fixed identifier for source. |
| `queue_ttl` | String |  | TTL in queue for this build. If provided and the build is enqueued longer than this value, the build will expire and the build status will be `EXPIRED`. The TTL starts ticking from create_time. |
| `results` | String |  | Output only. Results of the build. |
| `warnings` | Vec<String> |  | Output only. Non-fatal problems encountered during the execution of the build. |
| `name` | String |  | Output only. The 'Build' name with format: `projects/{project}/locations/{location}/builds/{build}`, where {build} is a unique identifier generated by the service. |
| `substitutions` | HashMap<String, String> |  | Substitutions data for `Build` resource. |
| `dependencies` | Vec<String> |  | Optional. Dependencies that the Cloud Build worker will fetch before executing user steps. |
| `available_secrets` | String |  | Secrets and secret environment variables. |
| `source` | String |  | Optional. The location of the source files to build. |
| `log_url` | String |  | Output only. URL to logs for this build in Google Cloud Console. |
| `artifacts` | String |  | Artifacts produced by the build that should be uploaded upon successful completion of all build steps. |
| `status_detail` | String |  | Output only. Customer-readable message about the current status. |
| `status` | String |  | Output only. Status of the build. |
| `secrets` | Vec<String> |  | Secrets to decrypt using Cloud Key Management Service. Note: Secret Manager is the recommended technique for managing sensitive data with Cloud Build. Use `available_secrets` to configure builds to access secrets from Secret Manager. For instructions, see: https://cloud.google.com/cloud-build/docs/securing-builds/use-secrets |
| `approval` | String |  | Output only. Describes this build's approval configuration, status, and result. |
| `tags` | Vec<String> |  | Tags for annotation of a `Build`. These are not docker tags. |
| `id` | String |  | Output only. Unique identifier of the build. |
| `service_account` | String |  | IAM service account whose credentials will be used at build runtime. Must be of the format `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`. ACCOUNT can be email address or uniqueId of the service account.  |
| `parent` | String | ✅ | The parent resource where this build will be created. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `failure_info` | String | Output only. Contains information about the build when status=FAILURE. |
| `git_config` | String | Optional. Configuration for git operations. |
| `timing` | HashMap<String, String> | Output only. Stores timing information for phases of the build. Valid keys are: * BUILD: time to execute all build steps. * PUSH: time to push all artifacts including docker images and non docker artifacts. * FETCHSOURCE: time to fetch source. * SETUPBUILD: time to set up build. If the build does not specify source or images, these keys will not be included. |
| `options` | String | Special options for this build. |
| `build_trigger_id` | String | Output only. The ID of the `BuildTrigger` that triggered this build, if it was triggered automatically. |
| `finish_time` | String | Output only. Time at which execution of the build was finished. The difference between finish_time and start_time is the duration of the build's execution. |
| `start_time` | String | Output only. Time at which execution of the build was started. |
| `logs_bucket` | String | Cloud Storage bucket where logs should be written (see [Bucket Name Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)). Logs file names will be of the format `${logs_bucket}/log-${build_id}.txt`. |
| `steps` | Vec<String> | Required. The operations to be performed on the workspace. |
| `images` | Vec<String> | A list of images to be pushed upon the successful completion of all build steps. The images are pushed using the builder service account's credentials. The digests of the pushed images will be stored in the `Build` resource's results field. If any of the images fail to be pushed, the build status is marked `FAILURE`. |
| `timeout` | String | Amount of time that this build should be allowed to run, to second granularity. If this amount of time elapses, work on the build will cease and the build status will be `TIMEOUT`. `timeout` starts ticking from `startTime`. Default time is 60 minutes. |
| `create_time` | String | Output only. Time at which the request to create the build was received. |
| `project_id` | String | Output only. ID of the project. |
| `source_provenance` | String | Output only. A permanent fixed identifier for source. |
| `queue_ttl` | String | TTL in queue for this build. If provided and the build is enqueued longer than this value, the build will expire and the build status will be `EXPIRED`. The TTL starts ticking from create_time. |
| `results` | String | Output only. Results of the build. |
| `warnings` | Vec<String> | Output only. Non-fatal problems encountered during the execution of the build. |
| `name` | String | Output only. The 'Build' name with format: `projects/{project}/locations/{location}/builds/{build}`, where {build} is a unique identifier generated by the service. |
| `substitutions` | HashMap<String, String> | Substitutions data for `Build` resource. |
| `dependencies` | Vec<String> | Optional. Dependencies that the Cloud Build worker will fetch before executing user steps. |
| `available_secrets` | String | Secrets and secret environment variables. |
| `source` | String | Optional. The location of the source files to build. |
| `log_url` | String | Output only. URL to logs for this build in Google Cloud Console. |
| `artifacts` | String | Artifacts produced by the build that should be uploaded upon successful completion of all build steps. |
| `status_detail` | String | Output only. Customer-readable message about the current status. |
| `status` | String | Output only. Status of the build. |
| `secrets` | Vec<String> | Secrets to decrypt using Cloud Key Management Service. Note: Secret Manager is the recommended technique for managing sensitive data with Cloud Build. Use `available_secrets` to configure builds to access secrets from Secret Manager. For instructions, see: https://cloud.google.com/cloud-build/docs/securing-builds/use-secrets |
| `approval` | String | Output only. Describes this build's approval configuration, status, and result. |
| `tags` | Vec<String> | Tags for annotation of a `Build`. These are not docker tags. |
| `id` | String | Output only. Unique identifier of the build. |
| `service_account` | String | IAM service account whose credentials will be used at build runtime. Must be of the format `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`. ACCOUNT can be email address or uniqueId of the service account.  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create build
build = provider.cloudbuild_api.Build {
    parent = "value"  # The parent resource where this build will be created. Format: `projects/{project}/locations/{location}`
}

# Access build outputs
build_id = build.id
build_failure_info = build.failure_info
build_git_config = build.git_config
build_timing = build.timing
build_options = build.options
build_build_trigger_id = build.build_trigger_id
build_finish_time = build.finish_time
build_start_time = build.start_time
build_logs_bucket = build.logs_bucket
build_steps = build.steps
build_images = build.images
build_timeout = build.timeout
build_create_time = build.create_time
build_project_id = build.project_id
build_source_provenance = build.source_provenance
build_queue_ttl = build.queue_ttl
build_results = build.results
build_warnings = build.warnings
build_name = build.name
build_substitutions = build.substitutions
build_dependencies = build.dependencies
build_available_secrets = build.available_secrets
build_source = build.source
build_log_url = build.log_url
build_artifacts = build.artifacts
build_status_detail = build.status_detail
build_status = build.status
build_secrets = build.secrets
build_approval = build.approval
build_tags = build.tags
build_id = build.id
build_service_account = build.service_account
```

---


### Trigger

Creates a new `BuildTrigger`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `approval_config` | String |  | Configuration for manual approval to start a build invocation of this BuildTrigger. |
| `bitbucket_server_trigger_config` | String |  | BitbucketServerTriggerConfig describes the configuration of a trigger that creates a build whenever a Bitbucket Server event is received. |
| `event_type` | String |  | EventType allows the user to explicitly set the type of event to which this BuildTrigger should respond. This field will be validated against the rest of the configuration if it is set. |
| `developer_connect_event_config` | String |  | Optional. The configuration of a trigger that creates a build whenever an event from the DeveloperConnect API is received. |
| `ignored_files` | Vec<String> |  | ignored_files and included_files are file glob matches using https://golang.org/pkg/path/filepath/#Match extended with support for "**". If ignored_files and changed files are both empty, then they are not used to determine whether or not to trigger a build. If ignored_files is not empty, then we ignore any files that match any of the ignored_file globs. If the change has no files that are outside of the ignored_files globs, then we do not trigger a build. |
| `name` | String |  | User-assigned name of the trigger. Must be unique within the project. Trigger names must meet the following requirements: + They must contain only alphanumeric characters and dashes. + They can be 1-64 characters long. + They must begin and end with an alphanumeric character. |
| `id` | String |  | Output only. Unique identifier of the trigger. |
| `resource_name` | String |  | The `Trigger` name with format: `projects/{project}/locations/{location}/triggers/{trigger}`, where {trigger} is a unique identifier generated by the service. |
| `github` | String |  | GitHubEventsConfig describes the configuration of a trigger that creates a build whenever a GitHub event is received. Mutually exclusive with `trigger_template`. |
| `substitutions` | HashMap<String, String> |  | Substitutions for Build resource. The keys must match the following regular expression: `^_[A-Z0-9_]+$`. |
| `build` | String |  | Contents of the build template. |
| `filename` | String |  | Path, from the source root, to the build configuration file (i.e. cloudbuild.yaml). |
| `disabled` | bool |  | If true, the trigger will never automatically execute a build. |
| `git_file_source` | String |  | The file source describing the local or remote Build template. |
| `create_time` | String |  | Output only. Time when the trigger was created. |
| `include_build_logs` | String |  | If set to INCLUDE_BUILD_LOGS_WITH_STATUS, log url will be shown on GitHub page when build status is final. Setting this field to INCLUDE_BUILD_LOGS_WITH_STATUS for non GitHub triggers results in INVALID_ARGUMENT error. |
| `pubsub_config` | String |  | PubsubConfig describes the configuration of a trigger that creates a build whenever a Pub/Sub message is published. |
| `service_account` | String |  | The service account used for all user-controlled operations including UpdateBuildTrigger, RunBuildTrigger, CreateBuild, and CancelBuild. If no service account is set and the legacy Cloud Build service account ([PROJECT_NUM]@cloudbuild.gserviceaccount.com) is the default for the project then it will be used instead. Format: `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}` |
| `trigger_template` | String |  | Template describing the types of source changes to trigger a build. Branch and tag names in trigger templates are interpreted as regular expressions. Any branch or tag change that matches that regular expression will trigger a build. Mutually exclusive with `github`. |
| `included_files` | Vec<String> |  | If any of the files altered in the commit pass the ignored_files filter and included_files is empty, then as far as this filter is concerned, we should trigger the build. If any of the files altered in the commit pass the ignored_files filter and included_files is not empty, then we make sure that at least one of those files matches a included_files glob. If not, then we do not trigger a build. |
| `webhook_config` | String |  | WebhookConfig describes the configuration of a trigger that creates a build whenever a webhook is sent to a trigger's webhook URL. |
| `tags` | Vec<String> |  | Tags for annotation of a `BuildTrigger` |
| `gitlab_enterprise_events_config` | String |  | GitLabEnterpriseEventsConfig describes the configuration of a trigger that creates a build whenever a GitLab Enterprise event is received. |
| `description` | String |  | Human-readable description of this trigger. |
| `autodetect` | bool |  | Autodetect build configuration. The following precedence is used (case insensitive): 1. cloudbuild.yaml 2. cloudbuild.yml 3. cloudbuild.json 4. Dockerfile Currently only available for GitHub App Triggers. |
| `source_to_build` | String |  | The repo and ref of the repository from which to build. This field is used only for those triggers that do not respond to SCM events. Triggers that respond to such events build source at whatever commit caused the event. This field is currently only used by Webhook, Pub/Sub, Manual, and Cron triggers. |
| `repository_event_config` | String |  | The configuration of a trigger that creates a build whenever an event from Repo API is received. |
| `filter` | String |  | A Common Expression Language string. |
| `parent` | String | ✅ | The parent resource where this trigger will be created. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `approval_config` | String | Configuration for manual approval to start a build invocation of this BuildTrigger. |
| `bitbucket_server_trigger_config` | String | BitbucketServerTriggerConfig describes the configuration of a trigger that creates a build whenever a Bitbucket Server event is received. |
| `event_type` | String | EventType allows the user to explicitly set the type of event to which this BuildTrigger should respond. This field will be validated against the rest of the configuration if it is set. |
| `developer_connect_event_config` | String | Optional. The configuration of a trigger that creates a build whenever an event from the DeveloperConnect API is received. |
| `ignored_files` | Vec<String> | ignored_files and included_files are file glob matches using https://golang.org/pkg/path/filepath/#Match extended with support for "**". If ignored_files and changed files are both empty, then they are not used to determine whether or not to trigger a build. If ignored_files is not empty, then we ignore any files that match any of the ignored_file globs. If the change has no files that are outside of the ignored_files globs, then we do not trigger a build. |
| `name` | String | User-assigned name of the trigger. Must be unique within the project. Trigger names must meet the following requirements: + They must contain only alphanumeric characters and dashes. + They can be 1-64 characters long. + They must begin and end with an alphanumeric character. |
| `id` | String | Output only. Unique identifier of the trigger. |
| `resource_name` | String | The `Trigger` name with format: `projects/{project}/locations/{location}/triggers/{trigger}`, where {trigger} is a unique identifier generated by the service. |
| `github` | String | GitHubEventsConfig describes the configuration of a trigger that creates a build whenever a GitHub event is received. Mutually exclusive with `trigger_template`. |
| `substitutions` | HashMap<String, String> | Substitutions for Build resource. The keys must match the following regular expression: `^_[A-Z0-9_]+$`. |
| `build` | String | Contents of the build template. |
| `filename` | String | Path, from the source root, to the build configuration file (i.e. cloudbuild.yaml). |
| `disabled` | bool | If true, the trigger will never automatically execute a build. |
| `git_file_source` | String | The file source describing the local or remote Build template. |
| `create_time` | String | Output only. Time when the trigger was created. |
| `include_build_logs` | String | If set to INCLUDE_BUILD_LOGS_WITH_STATUS, log url will be shown on GitHub page when build status is final. Setting this field to INCLUDE_BUILD_LOGS_WITH_STATUS for non GitHub triggers results in INVALID_ARGUMENT error. |
| `pubsub_config` | String | PubsubConfig describes the configuration of a trigger that creates a build whenever a Pub/Sub message is published. |
| `service_account` | String | The service account used for all user-controlled operations including UpdateBuildTrigger, RunBuildTrigger, CreateBuild, and CancelBuild. If no service account is set and the legacy Cloud Build service account ([PROJECT_NUM]@cloudbuild.gserviceaccount.com) is the default for the project then it will be used instead. Format: `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}` |
| `trigger_template` | String | Template describing the types of source changes to trigger a build. Branch and tag names in trigger templates are interpreted as regular expressions. Any branch or tag change that matches that regular expression will trigger a build. Mutually exclusive with `github`. |
| `included_files` | Vec<String> | If any of the files altered in the commit pass the ignored_files filter and included_files is empty, then as far as this filter is concerned, we should trigger the build. If any of the files altered in the commit pass the ignored_files filter and included_files is not empty, then we make sure that at least one of those files matches a included_files glob. If not, then we do not trigger a build. |
| `webhook_config` | String | WebhookConfig describes the configuration of a trigger that creates a build whenever a webhook is sent to a trigger's webhook URL. |
| `tags` | Vec<String> | Tags for annotation of a `BuildTrigger` |
| `gitlab_enterprise_events_config` | String | GitLabEnterpriseEventsConfig describes the configuration of a trigger that creates a build whenever a GitLab Enterprise event is received. |
| `description` | String | Human-readable description of this trigger. |
| `autodetect` | bool | Autodetect build configuration. The following precedence is used (case insensitive): 1. cloudbuild.yaml 2. cloudbuild.yml 3. cloudbuild.json 4. Dockerfile Currently only available for GitHub App Triggers. |
| `source_to_build` | String | The repo and ref of the repository from which to build. This field is used only for those triggers that do not respond to SCM events. Triggers that respond to such events build source at whatever commit caused the event. This field is currently only used by Webhook, Pub/Sub, Manual, and Cron triggers. |
| `repository_event_config` | String | The configuration of a trigger that creates a build whenever an event from Repo API is received. |
| `filter` | String | A Common Expression Language string. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create trigger
trigger = provider.cloudbuild_api.Trigger {
    parent = "value"  # The parent resource where this trigger will be created. Format: `projects/{project}/locations/{location}`
}

# Access trigger outputs
trigger_id = trigger.id
trigger_approval_config = trigger.approval_config
trigger_bitbucket_server_trigger_config = trigger.bitbucket_server_trigger_config
trigger_event_type = trigger.event_type
trigger_developer_connect_event_config = trigger.developer_connect_event_config
trigger_ignored_files = trigger.ignored_files
trigger_name = trigger.name
trigger_id = trigger.id
trigger_resource_name = trigger.resource_name
trigger_github = trigger.github
trigger_substitutions = trigger.substitutions
trigger_build = trigger.build
trigger_filename = trigger.filename
trigger_disabled = trigger.disabled
trigger_git_file_source = trigger.git_file_source
trigger_create_time = trigger.create_time
trigger_include_build_logs = trigger.include_build_logs
trigger_pubsub_config = trigger.pubsub_config
trigger_service_account = trigger.service_account
trigger_trigger_template = trigger.trigger_template
trigger_included_files = trigger.included_files
trigger_webhook_config = trigger.webhook_config
trigger_tags = trigger.tags
trigger_gitlab_enterprise_events_config = trigger.gitlab_enterprise_events_config
trigger_description = trigger.description
trigger_autodetect = trigger.autodetect
trigger_source_to_build = trigger.source_to_build
trigger_repository_event_config = trigger.repository_event_config
trigger_filter = trigger.filter
```

---


### Location

ReceiveRegionalWebhook is called when the API receives a regional GitHub webhook.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data` | String |  | The HTTP request/response body as raw binary. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `location` | String | ✅ | Required. The location where the webhook should be sent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Format: `projects/{project}/locations/{location}/defaultServiceAccount |
| `service_account_email` | String | Output only. The email address of the service account identity that will be used for a build by default. This is returned in the format `projects/{project}/serviceAccounts/{service_account}` where `{service_account}` could be the legacy Cloud Build SA, in the format [PROJECT_NUMBER]@cloudbuild.gserviceaccount.com or the Compute SA, in the format [PROJECT_NUMBER]-compute@developer.gserviceaccount.com. If no service account will be used by default, this will be empty. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.cloudbuild_api.Location {
    location = "value"  # Required. The location where the webhook should be sent.
}

# Access location outputs
location_id = location.id
location_name = location.name
location_service_account_email = location.service_account_email
```

---


### Connected_repositorie

Batch connecting GitLab repositories to Cloud Build. This API is experimental.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Required. Requests to connect GitLab repositories. |
| `parent` | String | ✅ | The name of the `GitLabConfig` that adds connected repositories. Format: `projects/{project}/locations/{location}/gitLabConfigs/{config}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connected_repositorie
connected_repositorie = provider.cloudbuild_api.Connected_repositorie {
    parent = "value"  # The name of the `GitLabConfig` that adds connected repositories. Format: `projects/{project}/locations/{location}/gitLabConfigs/{config}`
}

```

---


### Repo

List all repositories for a given `GitLabConfig`. This API is experimental

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gitlab_repositories` | Vec<String> | List of GitLab repositories |
| `next_page_token` | String | A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access repo outputs
repo_id = repo.id
repo_gitlab_repositories = repo.gitlab_repositories
repo_next_page_token = repo.next_page_token
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.cloudbuild_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
operation_done = operation.done
operation_response = operation.response
```

---


### Github_dot_com_webhook

ReceiveGitHubDotComWebhook is called when the API receives a github.com webhook.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data` | String |  | The HTTP request/response body as raw binary. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response for streaming APIs. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create github_dot_com_webhook
github_dot_com_webhook = provider.cloudbuild_api.Github_dot_com_webhook {
}

```

---


### Cloudbuild

ReceiveWebhook is called when the API receives a GitHub webhook.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data` | String |  | The HTTP request/response body as raw binary. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response for streaming APIs. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cloudbuild
cloudbuild = provider.cloudbuild_api.Cloudbuild {
}

```

---


### Worker_pool

Creates a `WorkerPool`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time at which the request to create the `WorkerPool` was received. |
| `update_time` | String |  | Output only. Time at which the request to update the `WorkerPool` was received. |
| `display_name` | String |  | A user-specified, human-readable name for the `WorkerPool`. If provided, this value must be 1-63 characters. |
| `private_pool_v1_config` | String |  | Private Pool configuration. |
| `delete_time` | String |  | Output only. Time at which the request to delete the `WorkerPool` was received. |
| `uid` | String |  | Output only. A unique identifier for the `WorkerPool`. |
| `etag` | String |  | Output only. Checksum computed by the server. May be sent on update and delete requests to ensure that the client has an up-to-date value before proceeding. |
| `annotations` | HashMap<String, String> |  | User specified annotations. See https://google.aip.dev/128#annotations for more details such as format and size limitations. |
| `state` | String |  | Output only. `WorkerPool` state. |
| `name` | String |  | Output only. The resource name of the `WorkerPool`, with format `projects/{project}/locations/{location}/workerPools/{worker_pool}`. The value of `{worker_pool}` is provided by `worker_pool_id` in `CreateWorkerPool` request and the value of `{location}` is determined by the endpoint accessed. |
| `parent` | String | ✅ | Required. The parent resource where this worker pool will be created. Format: `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time at which the request to create the `WorkerPool` was received. |
| `update_time` | String | Output only. Time at which the request to update the `WorkerPool` was received. |
| `display_name` | String | A user-specified, human-readable name for the `WorkerPool`. If provided, this value must be 1-63 characters. |
| `private_pool_v1_config` | String | Private Pool configuration. |
| `delete_time` | String | Output only. Time at which the request to delete the `WorkerPool` was received. |
| `uid` | String | Output only. A unique identifier for the `WorkerPool`. |
| `etag` | String | Output only. Checksum computed by the server. May be sent on update and delete requests to ensure that the client has an up-to-date value before proceeding. |
| `annotations` | HashMap<String, String> | User specified annotations. See https://google.aip.dev/128#annotations for more details such as format and size limitations. |
| `state` | String | Output only. `WorkerPool` state. |
| `name` | String | Output only. The resource name of the `WorkerPool`, with format `projects/{project}/locations/{location}/workerPools/{worker_pool}`. The value of `{worker_pool}` is provided by `worker_pool_id` in `CreateWorkerPool` request and the value of `{location}` is determined by the endpoint accessed. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create worker_pool
worker_pool = provider.cloudbuild_api.Worker_pool {
    parent = "value"  # Required. The parent resource where this worker pool will be created. Format: `projects/{project}/locations/{location}`.
}

# Access worker_pool outputs
worker_pool_id = worker_pool.id
worker_pool_create_time = worker_pool.create_time
worker_pool_update_time = worker_pool.update_time
worker_pool_display_name = worker_pool.display_name
worker_pool_private_pool_v1_config = worker_pool.private_pool_v1_config
worker_pool_delete_time = worker_pool.delete_time
worker_pool_uid = worker_pool.uid
worker_pool_etag = worker_pool.etag
worker_pool_annotations = worker_pool.annotations
worker_pool_state = worker_pool.state
worker_pool_name = worker_pool.name
```

---


### Bitbucket_server_config

Creates a new `BitbucketServerConfig`. This API is experimental.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connected_repositories` | Vec<String> |  | Output only. Connected Bitbucket Server repositories for this config. |
| `host_uri` | String |  | Required. Immutable. The URI of the Bitbucket Server host. Once this field has been set, it cannot be changed. If you need to change it, please create another BitbucketServerConfig. |
| `create_time` | String |  | Time when the config was created. |
| `peered_network` | String |  | Optional. The network to be used when reaching out to the Bitbucket Server instance. The VPC network must be enabled for private service connection. This should be set if the Bitbucket Server instance is hosted on-premises and not reachable by public internet. If this field is left empty, no network peering will occur and calls to the Bitbucket Server instance will be made over the public internet. Must be in the format `projects/{project}/global/networks/{network}`, where {project} is a project number or id and {network} is the name of a VPC network in the project. |
| `secrets` | String |  | Required. Secret Manager secrets needed by the config. |
| `name` | String |  | The resource name for the config. |
| `username` | String |  | Username of the account Cloud Build will use on Bitbucket Server. |
| `webhook_key` | String |  | Output only. UUID included in webhook requests. The UUID is used to look up the corresponding config. |
| `peered_network_ip_range` | String |  | Immutable. IP range within the peered network. This is specified in CIDR notation with a slash and the subnet prefix size. You can optionally specify an IP address before the subnet prefix value. e.g. `192.168.0.0/29` would specify an IP range starting at 192.168.0.0 with a 29 bit prefix size. `/16` would specify a prefix size of 16 bits, with an automatically determined IP within the peered VPC. If unspecified, a value of `/24` will be used. The field only has an effect if peered_network is set. |
| `ssl_ca` | String |  | Optional. SSL certificate to use for requests to Bitbucket Server. The format should be PEM format but the extension can be one of .pem, .cer, or .crt. |
| `api_key` | String |  | Required. Immutable. API Key that will be attached to webhook. Once this field has been set, it cannot be changed. If you need to change it, please create another BitbucketServerConfig. |
| `parent` | String | ✅ | Required. Name of the parent resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connected_repositories` | Vec<String> | Output only. Connected Bitbucket Server repositories for this config. |
| `host_uri` | String | Required. Immutable. The URI of the Bitbucket Server host. Once this field has been set, it cannot be changed. If you need to change it, please create another BitbucketServerConfig. |
| `create_time` | String | Time when the config was created. |
| `peered_network` | String | Optional. The network to be used when reaching out to the Bitbucket Server instance. The VPC network must be enabled for private service connection. This should be set if the Bitbucket Server instance is hosted on-premises and not reachable by public internet. If this field is left empty, no network peering will occur and calls to the Bitbucket Server instance will be made over the public internet. Must be in the format `projects/{project}/global/networks/{network}`, where {project} is a project number or id and {network} is the name of a VPC network in the project. |
| `secrets` | String | Required. Secret Manager secrets needed by the config. |
| `name` | String | The resource name for the config. |
| `username` | String | Username of the account Cloud Build will use on Bitbucket Server. |
| `webhook_key` | String | Output only. UUID included in webhook requests. The UUID is used to look up the corresponding config. |
| `peered_network_ip_range` | String | Immutable. IP range within the peered network. This is specified in CIDR notation with a slash and the subnet prefix size. You can optionally specify an IP address before the subnet prefix value. e.g. `192.168.0.0/29` would specify an IP range starting at 192.168.0.0 with a 29 bit prefix size. `/16` would specify a prefix size of 16 bits, with an automatically determined IP within the peered VPC. If unspecified, a value of `/24` will be used. The field only has an effect if peered_network is set. |
| `ssl_ca` | String | Optional. SSL certificate to use for requests to Bitbucket Server. The format should be PEM format but the extension can be one of .pem, .cer, or .crt. |
| `api_key` | String | Required. Immutable. API Key that will be attached to webhook. Once this field has been set, it cannot be changed. If you need to change it, please create another BitbucketServerConfig. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create bitbucket_server_config
bitbucket_server_config = provider.cloudbuild_api.Bitbucket_server_config {
    parent = "value"  # Required. Name of the parent resource.
}

# Access bitbucket_server_config outputs
bitbucket_server_config_id = bitbucket_server_config.id
bitbucket_server_config_connected_repositories = bitbucket_server_config.connected_repositories
bitbucket_server_config_host_uri = bitbucket_server_config.host_uri
bitbucket_server_config_create_time = bitbucket_server_config.create_time
bitbucket_server_config_peered_network = bitbucket_server_config.peered_network
bitbucket_server_config_secrets = bitbucket_server_config.secrets
bitbucket_server_config_name = bitbucket_server_config.name
bitbucket_server_config_username = bitbucket_server_config.username
bitbucket_server_config_webhook_key = bitbucket_server_config.webhook_key
bitbucket_server_config_peered_network_ip_range = bitbucket_server_config.peered_network_ip_range
bitbucket_server_config_ssl_ca = bitbucket_server_config.ssl_ca
bitbucket_server_config_api_key = bitbucket_server_config.api_key
```

---


### Worker_pool

Creates a `WorkerPool` to run the builds, and returns the new worker pool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time at which the request to create the `WorkerPool` was received. |
| `update_time` | String |  | Output only. Time at which the request to update the `WorkerPool` was received. |
| `network_config` | String |  | Network configuration for the `WorkerPool`. |
| `name` | String |  | Output only. The resource name of the `WorkerPool`. Format of the name is `projects/{project_id}/workerPools/{worker_pool_id}`, where the value of {worker_pool_id} is provided in the CreateWorkerPool request. |
| `region` | String |  | Required. Immutable. The region where the `WorkerPool` runs. Only "us-central1" is currently supported. Note that `region` cannot be changed once the `WorkerPool` is created. |
| `worker_config` | String |  | Worker configuration for the `WorkerPool`. |
| `delete_time` | String |  | Output only. Time at which the request to delete the `WorkerPool` was received. |
| `state` | String |  | Output only. WorkerPool state. |
| `parent` | String | ✅ | Required. The parent resource where this book will be created. Format: projects/{project} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time at which the request to create the `WorkerPool` was received. |
| `update_time` | String | Output only. Time at which the request to update the `WorkerPool` was received. |
| `network_config` | String | Network configuration for the `WorkerPool`. |
| `name` | String | Output only. The resource name of the `WorkerPool`. Format of the name is `projects/{project_id}/workerPools/{worker_pool_id}`, where the value of {worker_pool_id} is provided in the CreateWorkerPool request. |
| `region` | String | Required. Immutable. The region where the `WorkerPool` runs. Only "us-central1" is currently supported. Note that `region` cannot be changed once the `WorkerPool` is created. |
| `worker_config` | String | Worker configuration for the `WorkerPool`. |
| `delete_time` | String | Output only. Time at which the request to delete the `WorkerPool` was received. |
| `state` | String | Output only. WorkerPool state. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create worker_pool
worker_pool = provider.cloudbuild_api.Worker_pool {
    parent = "value"  # Required. The parent resource where this book will be created. Format: projects/{project}
}

# Access worker_pool outputs
worker_pool_id = worker_pool.id
worker_pool_create_time = worker_pool.create_time
worker_pool_update_time = worker_pool.update_time
worker_pool_network_config = worker_pool.network_config
worker_pool_name = worker_pool.name
worker_pool_region = worker_pool.region
worker_pool_worker_config = worker_pool.worker_config
worker_pool_delete_time = worker_pool.delete_time
worker_pool_state = worker_pool.state
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.cloudbuild_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_name = operation.name
operation_metadata = operation.metadata
operation_response = operation.response
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation = provider.cloudbuild_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
operation_error = operation.error
```

---


### Repositorie

Creates a Repository.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Server assigned timestamp for when the connection was created. |
| `annotations` | HashMap<String, String> |  | Optional. Allows clients to store small amounts of arbitrary data. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `name` | String |  | Immutable. Resource name of the repository, in the format `projects/*/locations/*/connections/*/repositories/*`. |
| `remote_uri` | String |  | Required. Git Clone HTTPS URI. |
| `webhook_id` | String |  | Output only. External ID of the webhook created for the repository. |
| `update_time` | String |  | Output only. Server assigned timestamp for when the connection was updated. |
| `parent` | String | ✅ | Required. The connection to contain the repository. If the request is part of a BatchCreateRepositoriesRequest, this field should be empty or match the parent specified there. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Server assigned timestamp for when the connection was created. |
| `annotations` | HashMap<String, String> | Optional. Allows clients to store small amounts of arbitrary data. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `name` | String | Immutable. Resource name of the repository, in the format `projects/*/locations/*/connections/*/repositories/*`. |
| `remote_uri` | String | Required. Git Clone HTTPS URI. |
| `webhook_id` | String | Output only. External ID of the webhook created for the repository. |
| `update_time` | String | Output only. Server assigned timestamp for when the connection was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create repositorie
repositorie = provider.cloudbuild_api.Repositorie {
    parent = "value"  # Required. The connection to contain the repository. If the request is part of a BatchCreateRepositoriesRequest, this field should be empty or match the parent specified there.
}

# Access repositorie outputs
repositorie_id = repositorie.id
repositorie_create_time = repositorie.create_time
repositorie_annotations = repositorie.annotations
repositorie_etag = repositorie.etag
repositorie_name = repositorie.name
repositorie_remote_uri = repositorie.remote_uri
repositorie_webhook_id = repositorie.webhook_id
repositorie_update_time = repositorie.update_time
```

---


### Connection

Creates a Connection.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `installation_state` | String |  | Output only. Installation state of the Connection. |
| `annotations` | HashMap<String, String> |  | Optional. Allows clients to store small amounts of arbitrary data. |
| `bitbucket_cloud_config` | String |  | Configuration for connections to Bitbucket Cloud. |
| `update_time` | String |  | Output only. Server assigned timestamp for when the connection was updated. |
| `create_time` | String |  | Output only. Server assigned timestamp for when the connection was created. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `gitlab_config` | String |  | Configuration for connections to gitlab.com or an instance of GitLab Enterprise. |
| `bitbucket_data_center_config` | String |  | Configuration for connections to Bitbucket Data Center. |
| `disabled` | bool |  | Optional. If disabled is set to true, functionality is disabled for this connection. Repository based API methods and webhooks processing for repositories in this connection will be disabled. |
| `reconciling` | bool |  | Output only. Set to true when the connection is being set up or updated in the background. |
| `github_enterprise_config` | String |  | Configuration for connections to an instance of GitHub Enterprise. |
| `name` | String |  | Immutable. The resource name of the connection, in the format `projects/{project}/locations/{location}/connections/{connection_id}`. |
| `github_config` | String |  | Configuration for connections to github.com. |
| `parent` | String | ✅ | Required. Project and location where the connection will be created. Format: `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `installation_state` | String | Output only. Installation state of the Connection. |
| `annotations` | HashMap<String, String> | Optional. Allows clients to store small amounts of arbitrary data. |
| `bitbucket_cloud_config` | String | Configuration for connections to Bitbucket Cloud. |
| `update_time` | String | Output only. Server assigned timestamp for when the connection was updated. |
| `create_time` | String | Output only. Server assigned timestamp for when the connection was created. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `gitlab_config` | String | Configuration for connections to gitlab.com or an instance of GitLab Enterprise. |
| `bitbucket_data_center_config` | String | Configuration for connections to Bitbucket Data Center. |
| `disabled` | bool | Optional. If disabled is set to true, functionality is disabled for this connection. Repository based API methods and webhooks processing for repositories in this connection will be disabled. |
| `reconciling` | bool | Output only. Set to true when the connection is being set up or updated in the background. |
| `github_enterprise_config` | String | Configuration for connections to an instance of GitHub Enterprise. |
| `name` | String | Immutable. The resource name of the connection, in the format `projects/{project}/locations/{location}/connections/{connection_id}`. |
| `github_config` | String | Configuration for connections to github.com. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connection
connection = provider.cloudbuild_api.Connection {
    parent = "value"  # Required. Project and location where the connection will be created. Format: `projects/*/locations/*`.
}

# Access connection outputs
connection_id = connection.id
connection_installation_state = connection.installation_state
connection_annotations = connection.annotations
connection_bitbucket_cloud_config = connection.bitbucket_cloud_config
connection_update_time = connection.update_time
connection_create_time = connection.create_time
connection_etag = connection.etag
connection_gitlab_config = connection.gitlab_config
connection_bitbucket_data_center_config = connection.bitbucket_data_center_config
connection_disabled = connection.disabled
connection_reconciling = connection.reconciling
connection_github_enterprise_config = connection.github_enterprise_config
connection_name = connection.name
connection_github_config = connection.github_config
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |


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
location_metadata = location.metadata
location_labels = location.labels
location_name = location.name
location_display_name = location.display_name
location_location_id = location.location_id
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.

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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.cloudbuild_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_error = operation.error
operation_done = operation.done
operation_name = operation.name
operation_response = operation.response
```

---


### Worker_pool

Creates a `WorkerPool` to run the builds, and returns the new worker pool. NOTE: As of now, this method returns an `Operation` that is always complete.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `annotations` | HashMap<String, String> |  | User specified annotations. See https://google.aip.dev/128#annotations for more details such as format and size limitations. |
| `create_time` | String |  | Output only. Time at which the request to create the `WorkerPool` was received. |
| `update_time` | String |  | Output only. Time at which the request to update the `WorkerPool` was received. |
| `network_config` | String |  | Network configuration for the `WorkerPool`. |
| `delete_time` | String |  | Output only. Time at which the request to delete the `WorkerPool` was received. |
| `display_name` | String |  | A user-specified, human-readable name for the `WorkerPool`. If provided, this value must be 1-63 characters. |
| `etag` | String |  | Output only. Checksum computed by the server. May be sent on update and delete requests to ensure that the client has an up-to-date value before proceeding. |
| `uid` | String |  | Output only. A unique identifier for the `WorkerPool`. |
| `worker_config` | String |  | Worker configuration for the `WorkerPool`. |
| `state` | String |  | Output only. `WorkerPool` state. |
| `name` | String |  | Output only. The resource name of the `WorkerPool`, with format `projects/{project}/locations/{location}/workerPools/{worker_pool}`. The value of `{worker_pool}` is provided by `worker_pool_id` in `CreateWorkerPool` request and the value of `{location}` is determined by the endpoint accessed. |
| `parent` | String | ✅ | Required. The parent resource where this worker pool will be created. Format: `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotations` | HashMap<String, String> | User specified annotations. See https://google.aip.dev/128#annotations for more details such as format and size limitations. |
| `create_time` | String | Output only. Time at which the request to create the `WorkerPool` was received. |
| `update_time` | String | Output only. Time at which the request to update the `WorkerPool` was received. |
| `network_config` | String | Network configuration for the `WorkerPool`. |
| `delete_time` | String | Output only. Time at which the request to delete the `WorkerPool` was received. |
| `display_name` | String | A user-specified, human-readable name for the `WorkerPool`. If provided, this value must be 1-63 characters. |
| `etag` | String | Output only. Checksum computed by the server. May be sent on update and delete requests to ensure that the client has an up-to-date value before proceeding. |
| `uid` | String | Output only. A unique identifier for the `WorkerPool`. |
| `worker_config` | String | Worker configuration for the `WorkerPool`. |
| `state` | String | Output only. `WorkerPool` state. |
| `name` | String | Output only. The resource name of the `WorkerPool`, with format `projects/{project}/locations/{location}/workerPools/{worker_pool}`. The value of `{worker_pool}` is provided by `worker_pool_id` in `CreateWorkerPool` request and the value of `{location}` is determined by the endpoint accessed. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create worker_pool
worker_pool = provider.cloudbuild_api.Worker_pool {
    parent = "value"  # Required. The parent resource where this worker pool will be created. Format: `projects/{project}/locations/{location}`.
}

# Access worker_pool outputs
worker_pool_id = worker_pool.id
worker_pool_annotations = worker_pool.annotations
worker_pool_create_time = worker_pool.create_time
worker_pool_update_time = worker_pool.update_time
worker_pool_network_config = worker_pool.network_config
worker_pool_delete_time = worker_pool.delete_time
worker_pool_display_name = worker_pool.display_name
worker_pool_etag = worker_pool.etag
worker_pool_uid = worker_pool.uid
worker_pool_worker_config = worker_pool.worker_config
worker_pool_state = worker_pool.state
worker_pool_name = worker_pool.name
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.cloudbuild_api.Operation {
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


### Worker_pool

Creates a `WorkerPool` to run the builds, and returns the new worker pool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `worker_config` | String |  | Configuration to be used for a creating workers in the `WorkerPool`. |
| `status` | String |  | Output only. WorkerPool Status. |
| `project_id` | String |  | The project ID of the GCP project for which the `WorkerPool` is created. |
| `regions` | Vec<String> |  | List of regions to create the `WorkerPool`. Regions can't be empty. If Cloud Build adds a new GCP region in the future, the existing `WorkerPool` will not be enabled in the new region automatically; you must add the new region to the `regions` field to enable the `WorkerPool` in that region. |
| `create_time` | String |  | Output only. Time at which the request to create the `WorkerPool` was received. |
| `service_account_email` | String |  | Output only. The service account used to manage the `WorkerPool`. The service account must have the Compute Instance Admin (Beta) permission at the project level. |
| `worker_count` | String |  | Total number of workers to be created across all requested regions. |
| `name` | String |  | User-defined name of the `WorkerPool`. |
| `delete_time` | String |  | Output only. Time at which the request to delete the `WorkerPool` was received. |
| `update_time` | String |  | Output only. Time at which the request to update the `WorkerPool` was received. |
| `parent` | String | ✅ | ID of the parent project. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `worker_config` | String | Configuration to be used for a creating workers in the `WorkerPool`. |
| `status` | String | Output only. WorkerPool Status. |
| `project_id` | String | The project ID of the GCP project for which the `WorkerPool` is created. |
| `regions` | Vec<String> | List of regions to create the `WorkerPool`. Regions can't be empty. If Cloud Build adds a new GCP region in the future, the existing `WorkerPool` will not be enabled in the new region automatically; you must add the new region to the `regions` field to enable the `WorkerPool` in that region. |
| `create_time` | String | Output only. Time at which the request to create the `WorkerPool` was received. |
| `service_account_email` | String | Output only. The service account used to manage the `WorkerPool`. The service account must have the Compute Instance Admin (Beta) permission at the project level. |
| `worker_count` | String | Total number of workers to be created across all requested regions. |
| `name` | String | User-defined name of the `WorkerPool`. |
| `delete_time` | String | Output only. Time at which the request to delete the `WorkerPool` was received. |
| `update_time` | String | Output only. Time at which the request to update the `WorkerPool` was received. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create worker_pool
worker_pool = provider.cloudbuild_api.Worker_pool {
    parent = "value"  # ID of the parent project.
}

# Access worker_pool outputs
worker_pool_id = worker_pool.id
worker_pool_worker_config = worker_pool.worker_config
worker_pool_status = worker_pool.status
worker_pool_project_id = worker_pool.project_id
worker_pool_regions = worker_pool.regions
worker_pool_create_time = worker_pool.create_time
worker_pool_service_account_email = worker_pool.service_account_email
worker_pool_worker_count = worker_pool.worker_count
worker_pool_name = worker_pool.name
worker_pool_delete_time = worker_pool.delete_time
worker_pool_update_time = worker_pool.update_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple git_lab_config resources
git_lab_config_0 = provider.cloudbuild_api.Git_lab_config {
    parent = "value-0"
}
git_lab_config_1 = provider.cloudbuild_api.Git_lab_config {
    parent = "value-1"
}
git_lab_config_2 = provider.cloudbuild_api.Git_lab_config {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    git_lab_config = provider.cloudbuild_api.Git_lab_config {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Cloudbuild_api Documentation](https://cloud.google.com/cloudbuild_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
