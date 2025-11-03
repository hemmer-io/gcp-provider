# Integrations_api Service



**Resources**: 34

---

## Overview

The integrations_api service provides access to 34 resource types:

- [Sfdc_instance](#sfdc_instance) [CRUD]
- [Executionsnapshot](#executionsnapshot) [R]
- [Location](#location) [R]
- [Auth_config](#auth_config) [CRUD]
- [Project](#project) [R]
- [Cloud_function](#cloud_function) [C]
- [Runtime_action_schema](#runtime_action_schema) [R]
- [Runtime_entity_schema](#runtime_entity_schema) [R]
- [Version](#version) [CRUD]
- [Integration](#integration) [CRD]
- [Product](#product) [CR]
- [Suspension](#suspension) [CR]
- [Apps_script_project](#apps_script_project) [C]
- [Callback](#callback) [R]
- [Client](#client) [C]
- [Sfdc_channel](#sfdc_channel) [CRUD]
- [Execution](#execution) [CR]
- [Connection](#connection) [R]
- [Certificate](#certificate) [CRUD]
- [Connector_platform_region](#connector_platform_region) [R]
- [Apps_script_project](#apps_script_project) [C]
- [Callback](#callback) [R]
- [Certificate](#certificate) [CRUD]
- [Execution](#execution) [CR]
- [Connector_platform_region](#connector_platform_region) [R]
- [Suspension](#suspension) [CR]
- [Sfdc_instance](#sfdc_instance) [CRUD]
- [Runtime_action_schema](#runtime_action_schema) [R]
- [Version](#version) [CRUD]
- [Auth_config](#auth_config) [CRUD]
- [Runtime_entity_schema](#runtime_entity_schema) [R]
- [Connection](#connection) [R]
- [Integration](#integration) [CRD]
- [Sfdc_channel](#sfdc_channel) [CRUD]

---

## Resources


### Sfdc_instance

Creates an sfdc instance record. Store the sfdc instance in Spanner. Returns the sfdc instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time when the instance is created |
| `auth_config_id` | Vec<String> |  | A list of AuthConfigs that can be tried to open the channel to SFDC |
| `display_name` | String |  | User selected unique name/alias to easily reference an instance. |
| `name` | String |  | Resource name of the SFDC instance projects/{project}/locations/{location}/sfdcInstances/{sfdcInstance}. |
| `delete_time` | String |  | Output only. Time when the instance was deleted. Empty if not deleted. |
| `service_authority` | String |  | URL used for API calls after authentication (the login authority is configured within the referenced AuthConfig). |
| `update_time` | String |  | Output only. Time when the instance was last updated |
| `sfdc_org_id` | String |  | The SFDC Org Id. This is defined in salesforce. |
| `description` | String |  | A description of the sfdc instance. |
| `parent` | String | ✅ | Required. "projects/{project}/locations/{location}" format. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time when the instance is created |
| `auth_config_id` | Vec<String> | A list of AuthConfigs that can be tried to open the channel to SFDC |
| `display_name` | String | User selected unique name/alias to easily reference an instance. |
| `name` | String | Resource name of the SFDC instance projects/{project}/locations/{location}/sfdcInstances/{sfdcInstance}. |
| `delete_time` | String | Output only. Time when the instance was deleted. Empty if not deleted. |
| `service_authority` | String | URL used for API calls after authentication (the login authority is configured within the referenced AuthConfig). |
| `update_time` | String | Output only. Time when the instance was last updated |
| `sfdc_org_id` | String | The SFDC Org Id. This is defined in salesforce. |
| `description` | String | A description of the sfdc instance. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sfdc_instance
sfdc_instance = provider.integrations_api.Sfdc_instance {
    parent = "value"  # Required. "projects/{project}/locations/{location}" format.
}

# Access sfdc_instance outputs
sfdc_instance_id = sfdc_instance.id
sfdc_instance_create_time = sfdc_instance.create_time
sfdc_instance_auth_config_id = sfdc_instance.auth_config_id
sfdc_instance_display_name = sfdc_instance.display_name
sfdc_instance_name = sfdc_instance.name
sfdc_instance_delete_time = sfdc_instance.delete_time
sfdc_instance_service_authority = sfdc_instance.service_authority
sfdc_instance_update_time = sfdc_instance.update_time
sfdc_instance_sfdc_org_id = sfdc_instance.sfdc_org_id
sfdc_instance_description = sfdc_instance.description
```

---


### Executionsnapshot

Lists the snapshots of a given integration executions. This RPC is not being used.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The token returned in the previous response. |
| `execution_snapshots` | Vec<String> | Required. The detailed information for the execution snapshot. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access executionsnapshot outputs
executionsnapshot_id = executionsnapshot.id
executionsnapshot_next_page_token = executionsnapshot.next_page_token
executionsnapshot_execution_snapshots = executionsnapshot.execution_snapshots
```

---


### Location

Gets the client configuration for the given project and location resource name

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `client` | String | Required. Required: The client configuration that was requested |


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
location_client = location.client
```

---


### Auth_config

Creates an auth config record. Fetch corresponding credentials for specific auth types, e.g. access token for OAuth 2.0, JWT token for JWT. Encrypt the auth config with Cloud KMS and store the encrypted credentials in Spanner. Returns the encrypted auth config.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `certificate_id` | String |  | Certificate id for client certificate |
| `description` | String |  | A description of the auth config. |
| `state` | String |  | The status of the auth config. |
| `credential_type` | String |  | Credential type of the encrypted credential. |
| `update_time` | String |  | Output only. The timestamp when the auth config is modified. |
| `creator_email` | String |  | The creator's email address. Generated based on the End User Credentials/LOAS role of the user making the call. |
| `visibility` | String |  | The visibility of the auth config. |
| `valid_time` | String |  | The time until the auth config is valid. Empty or max value is considered the auth config won't expire. |
| `override_valid_time` | String |  | User provided expiry time to override. For the example of Salesforce, username/password credentials can be valid for 6 months depending on the instance settings. |
| `encrypted_credential` | String |  | Auth credential encrypted by Cloud KMS. Can be decrypted as Credential with proper KMS key. |
| `decrypted_credential` | String |  | Raw auth credentials. |
| `reason` | String |  | The reason / details of the current status. |
| `create_time` | String |  | Output only. The timestamp when the auth config is created. |
| `last_modifier_email` | String |  | The last modifier's email address. Generated based on the End User Credentials/LOAS role of the user making the call. |
| `name` | String |  | Resource name of the SFDC instance projects/{project}/locations/{location}/authConfigs/{authConfig}. |
| `expiry_notification_duration` | Vec<String> |  | User can define the time to receive notification after which the auth config becomes invalid. Support up to 30 days. Support granularity in hours. |
| `display_name` | String |  | The name of the auth config. |
| `parent` | String | ✅ | Required. "projects/{project}/locations/{location}" format. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certificate_id` | String | Certificate id for client certificate |
| `description` | String | A description of the auth config. |
| `state` | String | The status of the auth config. |
| `credential_type` | String | Credential type of the encrypted credential. |
| `update_time` | String | Output only. The timestamp when the auth config is modified. |
| `creator_email` | String | The creator's email address. Generated based on the End User Credentials/LOAS role of the user making the call. |
| `visibility` | String | The visibility of the auth config. |
| `valid_time` | String | The time until the auth config is valid. Empty or max value is considered the auth config won't expire. |
| `override_valid_time` | String | User provided expiry time to override. For the example of Salesforce, username/password credentials can be valid for 6 months depending on the instance settings. |
| `encrypted_credential` | String | Auth credential encrypted by Cloud KMS. Can be decrypted as Credential with proper KMS key. |
| `decrypted_credential` | String | Raw auth credentials. |
| `reason` | String | The reason / details of the current status. |
| `create_time` | String | Output only. The timestamp when the auth config is created. |
| `last_modifier_email` | String | The last modifier's email address. Generated based on the End User Credentials/LOAS role of the user making the call. |
| `name` | String | Resource name of the SFDC instance projects/{project}/locations/{location}/authConfigs/{authConfig}. |
| `expiry_notification_duration` | Vec<String> | User can define the time to receive notification after which the auth config becomes invalid. Support up to 30 days. Support granularity in hours. |
| `display_name` | String | The name of the auth config. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create auth_config
auth_config = provider.integrations_api.Auth_config {
    parent = "value"  # Required. "projects/{project}/locations/{location}" format.
}

# Access auth_config outputs
auth_config_id = auth_config.id
auth_config_certificate_id = auth_config.certificate_id
auth_config_description = auth_config.description
auth_config_state = auth_config.state
auth_config_credential_type = auth_config.credential_type
auth_config_update_time = auth_config.update_time
auth_config_creator_email = auth_config.creator_email
auth_config_visibility = auth_config.visibility
auth_config_valid_time = auth_config.valid_time
auth_config_override_valid_time = auth_config.override_valid_time
auth_config_encrypted_credential = auth_config.encrypted_credential
auth_config_decrypted_credential = auth_config.decrypted_credential
auth_config_reason = auth_config.reason
auth_config_create_time = auth_config.create_time
auth_config_last_modifier_email = auth_config.last_modifier_email
auth_config_name = auth_config.name
auth_config_expiry_notification_duration = auth_config.expiry_notification_duration
auth_config_display_name = auth_config.display_name
```

---


### Project

Gets the metadata info for the requested client

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `properties` | String | Required. Required: The client configuration that was requested |


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
project_properties = project.properties
```

---


### Cloud_function

Creates an cloud function project.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `function_region` | String |  | The function region of CF to be created |
| `function_name` | String |  | The function name of CF to be created |
| `project_id` | String |  | Indicates the id of the GCP project that the function will be created in. |
| `parent` | String | ✅ | Required. The project that the executed integration belongs to. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cloud_function
cloud_function = provider.integrations_api.Cloud_function {
    parent = "value"  # Required. The project that the executed integration belongs to.
}

```

---


### Runtime_action_schema

Lists the JSON schemas for the inputs and outputs of actions, filtered by action name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `runtime_action_schemas` | Vec<String> | Runtime action schemas. |
| `next_page_token` | String | Next page token. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access runtime_action_schema outputs
runtime_action_schema_id = runtime_action_schema.id
runtime_action_schema_runtime_action_schemas = runtime_action_schema.runtime_action_schemas
runtime_action_schema_next_page_token = runtime_action_schema.next_page_token
```

---


### Runtime_entity_schema

Lists the JSON schemas for the properties of runtime entities, filtered by entity name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Next page token. |
| `runtime_entity_schemas` | Vec<String> | Runtime entity schemas. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access runtime_entity_schema outputs
runtime_entity_schema_id = runtime_entity_schema.id
runtime_entity_schema_next_page_token = runtime_entity_schema.next_page_token
runtime_entity_schema_runtime_entity_schemas = runtime_entity_schema.runtime_entity_schemas
```

---


### Version

Create a integration with a draft version in the specified project.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. The integration description. |
| `status` | String |  | Output only. Generated by eventbus. User should not set it as an input. |
| `teardown` | String |  | Optional. Contains a graph of tasks that will be executed before putting the event in a terminal state (SUCCEEDED/FAILED/FATAL), regardless of success or failure, similar to "finally" in code. |
| `task_configs` | Vec<String> |  | Optional. Task configuration for the integration. It's optional, but the integration doesn't do anything without task_configs. |
| `database_persistence_policy` | String |  | Optional. Flag to disable database persistence for execution data, including event execution info, execution export info, execution metadata index and execution param index. |
| `state` | String |  | Output only. User should not set it as an input. |
| `integration_parameters_internal` | String |  | Optional. Parameters that are expected to be passed to the integration when an event is triggered. This consists of all the parameters that are expected in the integration execution. This gives the user the ability to provide default values, add information like PII and also provide data types of each parameter. |
| `task_configs_internal` | Vec<String> |  | Optional. Task configuration for the integration. It's optional, but the integration doesn't do anything without task_configs. |
| `trigger_configs` | Vec<String> |  | Optional. Trigger configurations. |
| `last_modifier_email` | String |  | Optional. The last modifier's email address. Generated based on the End User Credentials/LOAS role of the user making the call. |
| `snapshot_number` | String |  | Optional. An increasing sequence that is set when a new snapshot is created. The last created snapshot can be identified by [workflow_name, org_id latest(snapshot_number)]. However, last created snapshot need not be same as the HEAD. So users should always use "HEAD" tag to identify the head. |
| `origin` | String |  | Optional. The origin that indicates where this integration is coming from. |
| `update_time` | String |  | Output only. Auto-generated. |
| `lock_holder` | String |  | Optional. The edit lock holder's email address. Generated based on the End User Credentials/LOAS role of the user making the call. |
| `trigger_configs_internal` | Vec<String> |  | Optional. Trigger configurations. |
| `name` | String |  | Output only. Auto-generated primary key. |
| `parent_template_id` | String |  | Optional. The id of the template which was used to create this integration_version. |
| `integration_parameters` | Vec<String> |  | Optional. Parameters that are expected to be passed to the integration when an event is triggered. This consists of all the parameters that are expected in the integration execution. This gives the user the ability to provide default values, add information like PII and also provide data types of each parameter. |
| `create_time` | String |  | Output only. Auto-generated. |
| `user_label` | String |  | Optional. A user-defined label that annotates an integration version. Typically, this is only set when the integration version is created. |
| `parent` | String | ✅ | Required. The parent resource where this version will be created. Format: projects/{project}/locations/{location}/integrations/{integration} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. The integration description. |
| `status` | String | Output only. Generated by eventbus. User should not set it as an input. |
| `teardown` | String | Optional. Contains a graph of tasks that will be executed before putting the event in a terminal state (SUCCEEDED/FAILED/FATAL), regardless of success or failure, similar to "finally" in code. |
| `task_configs` | Vec<String> | Optional. Task configuration for the integration. It's optional, but the integration doesn't do anything without task_configs. |
| `database_persistence_policy` | String | Optional. Flag to disable database persistence for execution data, including event execution info, execution export info, execution metadata index and execution param index. |
| `state` | String | Output only. User should not set it as an input. |
| `integration_parameters_internal` | String | Optional. Parameters that are expected to be passed to the integration when an event is triggered. This consists of all the parameters that are expected in the integration execution. This gives the user the ability to provide default values, add information like PII and also provide data types of each parameter. |
| `task_configs_internal` | Vec<String> | Optional. Task configuration for the integration. It's optional, but the integration doesn't do anything without task_configs. |
| `trigger_configs` | Vec<String> | Optional. Trigger configurations. |
| `last_modifier_email` | String | Optional. The last modifier's email address. Generated based on the End User Credentials/LOAS role of the user making the call. |
| `snapshot_number` | String | Optional. An increasing sequence that is set when a new snapshot is created. The last created snapshot can be identified by [workflow_name, org_id latest(snapshot_number)]. However, last created snapshot need not be same as the HEAD. So users should always use "HEAD" tag to identify the head. |
| `origin` | String | Optional. The origin that indicates where this integration is coming from. |
| `update_time` | String | Output only. Auto-generated. |
| `lock_holder` | String | Optional. The edit lock holder's email address. Generated based on the End User Credentials/LOAS role of the user making the call. |
| `trigger_configs_internal` | Vec<String> | Optional. Trigger configurations. |
| `name` | String | Output only. Auto-generated primary key. |
| `parent_template_id` | String | Optional. The id of the template which was used to create this integration_version. |
| `integration_parameters` | Vec<String> | Optional. Parameters that are expected to be passed to the integration when an event is triggered. This consists of all the parameters that are expected in the integration execution. This gives the user the ability to provide default values, add information like PII and also provide data types of each parameter. |
| `create_time` | String | Output only. Auto-generated. |
| `user_label` | String | Optional. A user-defined label that annotates an integration version. Typically, this is only set when the integration version is created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.integrations_api.Version {
    parent = "value"  # Required. The parent resource where this version will be created. Format: projects/{project}/locations/{location}/integrations/{integration}
}

# Access version outputs
version_id = version.id
version_description = version.description
version_status = version.status
version_teardown = version.teardown
version_task_configs = version.task_configs
version_database_persistence_policy = version.database_persistence_policy
version_state = version.state
version_integration_parameters_internal = version.integration_parameters_internal
version_task_configs_internal = version.task_configs_internal
version_trigger_configs = version.trigger_configs
version_last_modifier_email = version.last_modifier_email
version_snapshot_number = version.snapshot_number
version_origin = version.origin
version_update_time = version.update_time
version_lock_holder = version.lock_holder
version_trigger_configs_internal = version.trigger_configs_internal
version_name = version.name
version_parent_template_id = version.parent_template_id
version_integration_parameters = version.integration_parameters
version_create_time = version.create_time
version_user_label = version.user_label
```

---


### Integration

Schedules an integration for execution by passing the trigger id and the scheduled time in the request body.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameters` | String |  | Passed in as parameters to each integration execution. |
| `input_parameters` | HashMap<String, String> |  | Optional. Input parameters used by integration execution. |
| `request_id` | String |  | This is used to de-dup incoming request: if the duplicate request was detected, the response from the previous execution is returned. |
| `parameter_entries` | Vec<String> |  | Parameters are a part of Event and can be used to communicate between different tasks that are part of the same integration execution. |
| `schedule_time` | String |  | The time that the integration should be executed. If the time is less or equal to the current time, the integration is executed immediately. |
| `trigger_id` | String |  | Matched against all {@link TriggerConfig}s across all integrations. i.e. TriggerConfig.trigger_id.equals(trigger_id) |
| `name` | String | ✅ | The integration resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `integrations` | Vec<String> | The integrations which match the request. |
| `next_page_token` | String | The next page token for the response. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create integration
integration = provider.integrations_api.Integration {
    name = "value"  # The integration resource name.
}

# Access integration outputs
integration_id = integration.id
integration_integrations = integration.integrations
integration_next_page_token = integration.next_page_token
```

---


### Product

PROTECT WITH A VISIBILITY LABEL. THIS METHOD WILL BE MOVED TO A SEPARATE SERVICE. Create a bundle.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bundle_id` | String |  | Required. name of the bundle that will be created |
| `integrations` | Vec<String> |  | A list of integrations that can be executed by the bundle |
| `secondary_customer_org_id` | String |  | Optional. The prefix for the SA, it should be in the format "o". This is an optional field, and if empty service account will be created per project, where we are creating bundle. This should only be used as the org ID for which we want to run the integrations in the bundle. |
| `parent` | String | ✅ | Required. The location resource of the request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `task_entities` | Vec<String> | The list of the tasks. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create product
product = provider.integrations_api.Product {
    parent = "value"  # Required. The location resource of the request.
}

# Access product outputs
product_id = product.id
product_task_entities = product.task_entities
```

---


### Suspension

* Resolves (lifts/rejects) any number of suspensions. If the integration is already running, only the status of the suspension is updated. Otherwise, the suspended integration will begin execution again.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `suspension` | String |  | Suspension, containing the event_execution_info_id, task_id, and state to set on the corresponding suspension record. |
| `name` | String | ✅ | Required. projects/{gcp_project_id}/locations/{location}/products/{product}/integrations/{integration_name}/executions/{execution_name}/suspensions/{suspension_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to retrieve the next page of results. |
| `suspensions` | Vec<String> | The suspensions for the relevant execution which the caller has permissions to view and resolve. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create suspension
suspension = provider.integrations_api.Suspension {
    name = "value"  # Required. projects/{gcp_project_id}/locations/{location}/products/{product}/integrations/{integration_name}/executions/{execution_name}/suspensions/{suspension_id}
}

# Access suspension outputs
suspension_id = suspension.id
suspension_next_page_token = suspension.next_page_token
suspension_suspensions = suspension.suspensions
```

---


### Apps_script_project

Creates an Apps Script project.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `apps_script_project` | String |  | The name of the Apps Script project to be created. |
| `auth_config_id` | String |  | The auth config id necessary to fetch the necessary credentials to create the project for external clients |
| `parent` | String | ✅ | Required. The project that the executed integration belongs to. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create apps_script_project
apps_script_project = provider.integrations_api.Apps_script_project {
    parent = "value"  # Required. The project that the executed integration belongs to.
}

```

---


### Callback

Receives the auth code and auth config id to combine that with the client id and secret to retrieve access tokens from the token endpoint. Returns either a success or error message when it's done.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `message` | String | The message that notifies the user if the request succeeded or not. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access callback outputs
callback_id = callback.id
callback_message = callback.message
```

---


### Client

Perform the deprovisioning steps to disable a user GCP project to use IP and purge all related data in a wipeout-compliant way.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | Required. Required: The ID of the GCP Project to be deprovisioned. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create client
client = provider.integrations_api.Client {
    parent = "value"  # Required. Required: The ID of the GCP Project to be deprovisioned.
}

```

---


### Sfdc_channel

Creates an sfdc channel record. Store the sfdc channel in Spanner. Returns the sfdc channel.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Resource name of the SFDC channel projects/{project}/locations/{location}/sfdcInstances/{sfdc_instance}/sfdcChannels/{sfdc_channel}. |
| `last_replay_id` | String |  | Last sfdc messsage replay id for channel |
| `channel_topic` | String |  | The Channel topic defined by salesforce once an channel is opened |
| `create_time` | String |  | Output only. Time when the channel is created |
| `is_active` | bool |  | Indicated if a channel has any active integrations referencing it. Set to false when the channel is created, and set to true if there is any integration published with the channel configured in it. |
| `update_time` | String |  | Output only. Time when the channel was last updated |
| `description` | String |  | The description for this channel |
| `delete_time` | String |  | Output only. Time when the channel was deleted. Empty if not deleted. |
| `display_name` | String |  | Client level unique name/alias to easily reference a channel. |
| `parent` | String | ✅ | Required. "projects/{project}/locations/{location}" format. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name of the SFDC channel projects/{project}/locations/{location}/sfdcInstances/{sfdc_instance}/sfdcChannels/{sfdc_channel}. |
| `last_replay_id` | String | Last sfdc messsage replay id for channel |
| `channel_topic` | String | The Channel topic defined by salesforce once an channel is opened |
| `create_time` | String | Output only. Time when the channel is created |
| `is_active` | bool | Indicated if a channel has any active integrations referencing it. Set to false when the channel is created, and set to true if there is any integration published with the channel configured in it. |
| `update_time` | String | Output only. Time when the channel was last updated |
| `description` | String | The description for this channel |
| `delete_time` | String | Output only. Time when the channel was deleted. Empty if not deleted. |
| `display_name` | String | Client level unique name/alias to easily reference a channel. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sfdc_channel
sfdc_channel = provider.integrations_api.Sfdc_channel {
    parent = "value"  # Required. "projects/{project}/locations/{location}" format.
}

# Access sfdc_channel outputs
sfdc_channel_id = sfdc_channel.id
sfdc_channel_name = sfdc_channel.name
sfdc_channel_last_replay_id = sfdc_channel.last_replay_id
sfdc_channel_channel_topic = sfdc_channel.channel_topic
sfdc_channel_create_time = sfdc_channel.create_time
sfdc_channel_is_active = sfdc_channel.is_active
sfdc_channel_update_time = sfdc_channel.update_time
sfdc_channel_description = sfdc_channel.description
sfdc_channel_delete_time = sfdc_channel.delete_time
sfdc_channel_display_name = sfdc_channel.display_name
```

---


### Execution

Cancellation of an execution

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The execution resource name. Format: projects/{gcp_project_id}/locations/{location}/products/{product}/integrations/{integration_id}/executions/{execution_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `execution_method` | String | The ways user posts this event. |
| `name` | String | Auto-generated primary key. |
| `trigger_id` | String | The trigger id of the integration trigger config. If both trigger_id and client_id is present, the integration is executed from the start tasks provided by the matching trigger config otherwise it is executed from the default start tasks. |
| `update_time` | String | Output only. Last modified time of the execution. |
| `direct_sub_executions` | Vec<String> | Direct sub executions of the following Execution. |
| `response_parameters` | HashMap<String, String> | Event parameters returned as part of the response. |
| `request_parameters` | HashMap<String, String> | Event parameters come in as part of the request. |
| `event_execution_details` | String | The execution info about this event. |
| `create_time` | String | Output only. Created time of the execution. |
| `response_params` | Vec<String> | Event parameters come out as part of the response. |
| `execution_details` | String | Detailed info of this execution. |
| `request_params` | Vec<String> | Event parameters come in as part of the request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create execution
execution = provider.integrations_api.Execution {
    name = "value"  # Required. The execution resource name. Format: projects/{gcp_project_id}/locations/{location}/products/{product}/integrations/{integration_id}/executions/{execution_id}
}

# Access execution outputs
execution_id = execution.id
execution_execution_method = execution.execution_method
execution_name = execution.name
execution_trigger_id = execution.trigger_id
execution_update_time = execution.update_time
execution_direct_sub_executions = execution.direct_sub_executions
execution_response_parameters = execution.response_parameters
execution_request_parameters = execution.request_parameters
execution_event_execution_details = execution.event_execution_details
execution_create_time = execution.create_time
execution_response_params = execution.response_params
execution_execution_details = execution.execution_details
execution_request_params = execution.request_params
```

---


### Connection

Lists the available entities and actions associated with a Connection.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `actions` | Vec<String> | List of actions. |
| `entities` | Vec<String> | List of entity names. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access connection outputs
connection_id = connection.id
connection_actions = connection.actions
connection_entities = connection.entities
```

---


### Certificate

Creates a new certificate. The certificate will be registered to the trawler service and will be encrypted using cloud KMS and stored in Spanner Returns the certificate.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `credential_id` | String |  | Immutable. Credential id that will be used to register with trawler INTERNAL_ONLY |
| `description` | String |  | Description of the certificate |
| `certificate_status` | String |  | Status of the certificate |
| `display_name` | String |  | Name of the certificate |
| `requestor_id` | String |  | Immutable. Requestor ID to be used to register certificate with trawler |
| `valid_start_time` | String |  | Output only. The timestamp after which certificate will be valid |
| `raw_certificate` | String |  | Input only. Raw client certificate which would be registered with trawler |
| `name` | String |  | Output only. Auto generated primary key |
| `valid_end_time` | String |  | Output only. The timestamp after which certificate will expire |
| `parent` | String | ✅ | Required. "projects/{project}/locations/{location}" format. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `credential_id` | String | Immutable. Credential id that will be used to register with trawler INTERNAL_ONLY |
| `description` | String | Description of the certificate |
| `certificate_status` | String | Status of the certificate |
| `display_name` | String | Name of the certificate |
| `requestor_id` | String | Immutable. Requestor ID to be used to register certificate with trawler |
| `valid_start_time` | String | Output only. The timestamp after which certificate will be valid |
| `raw_certificate` | String | Input only. Raw client certificate which would be registered with trawler |
| `name` | String | Output only. Auto generated primary key |
| `valid_end_time` | String | Output only. The timestamp after which certificate will expire |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create certificate
certificate = provider.integrations_api.Certificate {
    parent = "value"  # Required. "projects/{project}/locations/{location}" format.
}

# Access certificate outputs
certificate_id = certificate.id
certificate_credential_id = certificate.credential_id
certificate_description = certificate.description
certificate_certificate_status = certificate.certificate_status
certificate_display_name = certificate.display_name
certificate_requestor_id = certificate.requestor_id
certificate_valid_start_time = certificate.valid_start_time
certificate_raw_certificate = certificate.raw_certificate
certificate_name = certificate.name
certificate_valid_end_time = certificate.valid_end_time
```

---


### Connector_platform_region

Enumerates the regions for which Connector Platform is provisioned.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `regions` | Vec<String> | All regions where Connector Platform is provisioned. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access connector_platform_region outputs
connector_platform_region_id = connector_platform_region.id
connector_platform_region_regions = connector_platform_region.regions
```

---


### Apps_script_project

Creates an Apps Script project.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `apps_script_project` | String |  | The name of the Apps Script project to be created. |
| `auth_config_id` | String |  | The auth config id necessary to fetch the necessary credentials to create the project for external clients |
| `parent` | String | ✅ | Required. The project that the executed integration belongs to. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create apps_script_project
apps_script_project = provider.integrations_api.Apps_script_project {
    parent = "value"  # Required. The project that the executed integration belongs to.
}

```

---


### Callback

Receives the auth code and auth config id to combine that with the client id and secret to retrieve access tokens from the token endpoint. Returns either a success or error message when it's done.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `message` | String | The message that notifies the user if the request succeeded or not. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access callback outputs
callback_id = callback.id
callback_message = callback.message
```

---


### Certificate

Creates a new certificate. The certificate will be registered to the trawler service and will be encrypted using cloud KMS and stored in Spanner Returns the certificate.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `valid_end_time` | String |  | Output only. The timestamp after which certificate will expire |
| `valid_start_time` | String |  | Output only. The timestamp after which certificate will be valid |
| `certificate_status` | String |  | Status of the certificate |
| `raw_certificate` | String |  | Input only. Raw client certificate which would be registered with trawler |
| `display_name` | String |  | Name of the certificate |
| `credential_id` | String |  | Immutable. Credential id that will be used to register with trawler INTERNAL_ONLY |
| `name` | String |  | Output only. Auto generated primary key |
| `description` | String |  | Description of the certificate |
| `requestor_id` | String |  | Immutable. Requestor ID to be used to register certificate with trawler |
| `parent` | String | ✅ | Required. "projects/{project}/locations/{location}" format. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `valid_end_time` | String | Output only. The timestamp after which certificate will expire |
| `valid_start_time` | String | Output only. The timestamp after which certificate will be valid |
| `certificate_status` | String | Status of the certificate |
| `raw_certificate` | String | Input only. Raw client certificate which would be registered with trawler |
| `display_name` | String | Name of the certificate |
| `credential_id` | String | Immutable. Credential id that will be used to register with trawler INTERNAL_ONLY |
| `name` | String | Output only. Auto generated primary key |
| `description` | String | Description of the certificate |
| `requestor_id` | String | Immutable. Requestor ID to be used to register certificate with trawler |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create certificate
certificate = provider.integrations_api.Certificate {
    parent = "value"  # Required. "projects/{project}/locations/{location}" format.
}

# Access certificate outputs
certificate_id = certificate.id
certificate_valid_end_time = certificate.valid_end_time
certificate_valid_start_time = certificate.valid_start_time
certificate_certificate_status = certificate.certificate_status
certificate_raw_certificate = certificate.raw_certificate
certificate_display_name = certificate.display_name
certificate_credential_id = certificate.credential_id
certificate_name = certificate.name
certificate_description = certificate.description
certificate_requestor_id = certificate.requestor_id
```

---


### Execution

Cancellation of an execution

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The execution resource name. Format: projects/{gcp_project_id}/locations/{location}/products/{product}/integrations/{integration_id}/executions/{execution_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `execution_details` | String | Detailed info of this execution. |
| `response_parameters` | HashMap<String, String> | Event parameters returned as part of the response. |
| `trigger_id` | String | The trigger id of the integration trigger config. If both trigger_id and client_id is present, the integration is executed from the start tasks provided by the matching trigger config otherwise it is executed from the default start tasks. |
| `execution_method` | String | The ways user posts this event. |
| `name` | String | Auto-generated primary key. |
| `update_time` | String | Output only. Last modified time of the execution. |
| `request_params` | Vec<String> | Event parameters come in as part of the request. |
| `direct_sub_executions` | Vec<String> | Direct sub executions of the following Execution. |
| `request_parameters` | HashMap<String, String> | Event parameters come in as part of the request. |
| `create_time` | String | Output only. Created time of the execution. |
| `response_params` | Vec<String> | Event parameters come out as part of the response. |
| `event_execution_details` | String | The execution info about this event. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create execution
execution = provider.integrations_api.Execution {
    name = "value"  # Required. The execution resource name. Format: projects/{gcp_project_id}/locations/{location}/products/{product}/integrations/{integration_id}/executions/{execution_id}
}

# Access execution outputs
execution_id = execution.id
execution_execution_details = execution.execution_details
execution_response_parameters = execution.response_parameters
execution_trigger_id = execution.trigger_id
execution_execution_method = execution.execution_method
execution_name = execution.name
execution_update_time = execution.update_time
execution_request_params = execution.request_params
execution_direct_sub_executions = execution.direct_sub_executions
execution_request_parameters = execution.request_parameters
execution_create_time = execution.create_time
execution_response_params = execution.response_params
execution_event_execution_details = execution.event_execution_details
```

---


### Connector_platform_region

Enumerates the regions for which Connector Platform is provisioned.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `regions` | Vec<String> | All regions where Connector Platform is provisioned. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access connector_platform_region outputs
connector_platform_region_id = connector_platform_region.id
connector_platform_region_regions = connector_platform_region.regions
```

---


### Suspension

* Resolves (lifts/rejects) any number of suspensions. If the integration is already running, only the status of the suspension is updated. Otherwise, the suspended integration will begin execution again.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `suspension` | String |  | Suspension, containing the event_execution_info_id, task_id, and state to set on the corresponding suspension record. |
| `name` | String | ✅ | Required. projects/{gcp_project_id}/locations/{location}/products/{product}/integrations/{integration_name}/executions/{execution_name}/suspensions/{suspension_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to retrieve the next page of results. |
| `suspensions` | Vec<String> | The suspensions for the relevant execution which the caller has permissions to view and resolve. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create suspension
suspension = provider.integrations_api.Suspension {
    name = "value"  # Required. projects/{gcp_project_id}/locations/{location}/products/{product}/integrations/{integration_name}/executions/{execution_name}/suspensions/{suspension_id}
}

# Access suspension outputs
suspension_id = suspension.id
suspension_next_page_token = suspension.next_page_token
suspension_suspensions = suspension.suspensions
```

---


### Sfdc_instance

Creates an sfdc instance record. Store the sfdc instance in Spanner. Returns the sfdc instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auth_config_id` | Vec<String> |  | A list of AuthConfigs that can be tried to open the channel to SFDC |
| `sfdc_org_id` | String |  | The SFDC Org Id. This is defined in salesforce. |
| `create_time` | String |  | Output only. Time when the instance is created |
| `description` | String |  | A description of the sfdc instance. |
| `service_authority` | String |  | URL used for API calls after authentication (the login authority is configured within the referenced AuthConfig). |
| `update_time` | String |  | Output only. Time when the instance was last updated |
| `delete_time` | String |  | Output only. Time when the instance was deleted. Empty if not deleted. |
| `display_name` | String |  | User selected unique name/alias to easily reference an instance. |
| `name` | String |  | Resource name of the SFDC instance projects/{project}/locations/{location}/sfdcInstances/{sfdcInstance}. |
| `parent` | String | ✅ | Required. "projects/{project}/locations/{location}" format. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auth_config_id` | Vec<String> | A list of AuthConfigs that can be tried to open the channel to SFDC |
| `sfdc_org_id` | String | The SFDC Org Id. This is defined in salesforce. |
| `create_time` | String | Output only. Time when the instance is created |
| `description` | String | A description of the sfdc instance. |
| `service_authority` | String | URL used for API calls after authentication (the login authority is configured within the referenced AuthConfig). |
| `update_time` | String | Output only. Time when the instance was last updated |
| `delete_time` | String | Output only. Time when the instance was deleted. Empty if not deleted. |
| `display_name` | String | User selected unique name/alias to easily reference an instance. |
| `name` | String | Resource name of the SFDC instance projects/{project}/locations/{location}/sfdcInstances/{sfdcInstance}. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sfdc_instance
sfdc_instance = provider.integrations_api.Sfdc_instance {
    parent = "value"  # Required. "projects/{project}/locations/{location}" format.
}

# Access sfdc_instance outputs
sfdc_instance_id = sfdc_instance.id
sfdc_instance_auth_config_id = sfdc_instance.auth_config_id
sfdc_instance_sfdc_org_id = sfdc_instance.sfdc_org_id
sfdc_instance_create_time = sfdc_instance.create_time
sfdc_instance_description = sfdc_instance.description
sfdc_instance_service_authority = sfdc_instance.service_authority
sfdc_instance_update_time = sfdc_instance.update_time
sfdc_instance_delete_time = sfdc_instance.delete_time
sfdc_instance_display_name = sfdc_instance.display_name
sfdc_instance_name = sfdc_instance.name
```

---


### Runtime_action_schema

Lists the JSON schemas for the inputs and outputs of actions, filtered by action name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Next page token. |
| `runtime_action_schemas` | Vec<String> | Runtime action schemas. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access runtime_action_schema outputs
runtime_action_schema_id = runtime_action_schema.id
runtime_action_schema_next_page_token = runtime_action_schema.next_page_token
runtime_action_schema_runtime_action_schemas = runtime_action_schema.runtime_action_schemas
```

---


### Version

Creates an IntegrationTemplateVersion.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `task_configs` | Vec<String> |  | Optional. Task configuration for the IntegrationTemplateVersion. It's optional, but the IntegrationTemplateVersion doesn't do anything without task_configs. |
| `template_parameters` | String |  | Optional. Parameters that are expected to be passed to the IntegrationTemplateVersion when an event is triggered. This consists of all the parameters that are expected in the IntegrationTemplateVersion execution. This gives the user the ability to provide default values, add information like PII and also provide data types of each parameter. |
| `description` | String |  | Optional. The templateversion description. Permitted format is alphanumeric with underscores and no spaces. |
| `error_catcher_configs` | Vec<String> |  | Optional. Error Catch Task configuration for the IntegrationTemplateVersion. It's optional. |
| `user_label` | String |  | Optional. A user-defined label that annotates an integration version. Typically, this is only set when the integration version is created. |
| `update_time` | String |  | Output only. Auto-generated. |
| `status` | String |  | Optional. Generated by eventbus. User should not set it as an input. |
| `snapshot_number` | String |  | Output only. An increasing sequence that is set when a new snapshot is created. |
| `database_persistence_policy` | String |  | Optional. Flag to disable database persistence for execution data, including event execution info, execution export info, execution metadata index and execution param index. |
| `create_time` | String |  | Output only. Auto-generated. |
| `last_modifier_email` | String |  | Optional. The last modifier's email address. Generated based on the End User Credentials/LOAS role of the user making the call. |
| `teardown` | String |  | Optional. Contains a graph of tasks that will be executed before putting the event in a terminal state (SUCCEEDED/FAILED/FATAL), regardless of success or failure, similar to "finally" in code. |
| `trigger_configs` | Vec<String> |  | Optional. Trigger configurations. |
| `parent_integration_version_id` | String |  | Optional. ID of the IntegrationVersion that was used to create this IntegrationTemplateVersion |
| `name` | String |  | Output only. Auto-generated primary key. Format: projects/{project}/locations/{location}/products/{product}/integrationtemplates/{integrationtemplate}/versions/{version} |
| `parent` | String | ✅ | Required. The parent resource where this TemplateVersion will be created. Format: projects/{project}/location/{location}/product/{product}/integrationtemplates/{integrationtemplate} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `task_configs` | Vec<String> | Optional. Task configuration for the IntegrationTemplateVersion. It's optional, but the IntegrationTemplateVersion doesn't do anything without task_configs. |
| `template_parameters` | String | Optional. Parameters that are expected to be passed to the IntegrationTemplateVersion when an event is triggered. This consists of all the parameters that are expected in the IntegrationTemplateVersion execution. This gives the user the ability to provide default values, add information like PII and also provide data types of each parameter. |
| `description` | String | Optional. The templateversion description. Permitted format is alphanumeric with underscores and no spaces. |
| `error_catcher_configs` | Vec<String> | Optional. Error Catch Task configuration for the IntegrationTemplateVersion. It's optional. |
| `user_label` | String | Optional. A user-defined label that annotates an integration version. Typically, this is only set when the integration version is created. |
| `update_time` | String | Output only. Auto-generated. |
| `status` | String | Optional. Generated by eventbus. User should not set it as an input. |
| `snapshot_number` | String | Output only. An increasing sequence that is set when a new snapshot is created. |
| `database_persistence_policy` | String | Optional. Flag to disable database persistence for execution data, including event execution info, execution export info, execution metadata index and execution param index. |
| `create_time` | String | Output only. Auto-generated. |
| `last_modifier_email` | String | Optional. The last modifier's email address. Generated based on the End User Credentials/LOAS role of the user making the call. |
| `teardown` | String | Optional. Contains a graph of tasks that will be executed before putting the event in a terminal state (SUCCEEDED/FAILED/FATAL), regardless of success or failure, similar to "finally" in code. |
| `trigger_configs` | Vec<String> | Optional. Trigger configurations. |
| `parent_integration_version_id` | String | Optional. ID of the IntegrationVersion that was used to create this IntegrationTemplateVersion |
| `name` | String | Output only. Auto-generated primary key. Format: projects/{project}/locations/{location}/products/{product}/integrationtemplates/{integrationtemplate}/versions/{version} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.integrations_api.Version {
    parent = "value"  # Required. The parent resource where this TemplateVersion will be created. Format: projects/{project}/location/{location}/product/{product}/integrationtemplates/{integrationtemplate}
}

# Access version outputs
version_id = version.id
version_task_configs = version.task_configs
version_template_parameters = version.template_parameters
version_description = version.description
version_error_catcher_configs = version.error_catcher_configs
version_user_label = version.user_label
version_update_time = version.update_time
version_status = version.status
version_snapshot_number = version.snapshot_number
version_database_persistence_policy = version.database_persistence_policy
version_create_time = version.create_time
version_last_modifier_email = version.last_modifier_email
version_teardown = version.teardown
version_trigger_configs = version.trigger_configs
version_parent_integration_version_id = version.parent_integration_version_id
version_name = version.name
```

---


### Auth_config

Creates an auth config record. Fetch corresponding credentials for specific auth types, e.g. access token for OAuth 2.0, JWT token for JWT. Encrypt the auth config with Cloud KMS and store the encrypted credentials in Spanner. Returns the encrypted auth config.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | The name of the auth config. |
| `valid_time` | String |  | The time until the auth config is valid. Empty or max value is considered the auth config won't expire. |
| `decrypted_credential` | String |  | Raw auth credentials. |
| `encrypted_credential` | String |  | Auth credential encrypted by Cloud KMS. Can be decrypted as Credential with proper KMS key. |
| `expiry_notification_duration` | Vec<String> |  | User can define the time to receive notification after which the auth config becomes invalid. Support up to 30 days. Support granularity in hours. |
| `state` | String |  | The status of the auth config. |
| `credential_type` | String |  | Credential type of the encrypted credential. |
| `name` | String |  | Resource name of the SFDC instance projects/{project}/locations/{location}/authConfigs/{authConfig}. |
| `last_modifier_email` | String |  | The last modifier's email address. Generated based on the End User Credentials/LOAS role of the user making the call. |
| `certificate_id` | String |  | Certificate id for client certificate |
| `update_time` | String |  | Output only. The timestamp when the auth config is modified. |
| `creator_email` | String |  | The creator's email address. Generated based on the End User Credentials/LOAS role of the user making the call. |
| `description` | String |  | A description of the auth config. |
| `override_valid_time` | String |  | User provided expiry time to override. For the example of Salesforce, username/password credentials can be valid for 6 months depending on the instance settings. |
| `reason` | String |  | The reason / details of the current status. |
| `visibility` | String |  | The visibility of the auth config. |
| `create_time` | String |  | Output only. The timestamp when the auth config is created. |
| `parent` | String | ✅ | Required. "projects/{project}/locations/{location}" format. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The name of the auth config. |
| `valid_time` | String | The time until the auth config is valid. Empty or max value is considered the auth config won't expire. |
| `decrypted_credential` | String | Raw auth credentials. |
| `encrypted_credential` | String | Auth credential encrypted by Cloud KMS. Can be decrypted as Credential with proper KMS key. |
| `expiry_notification_duration` | Vec<String> | User can define the time to receive notification after which the auth config becomes invalid. Support up to 30 days. Support granularity in hours. |
| `state` | String | The status of the auth config. |
| `credential_type` | String | Credential type of the encrypted credential. |
| `name` | String | Resource name of the SFDC instance projects/{project}/locations/{location}/authConfigs/{authConfig}. |
| `last_modifier_email` | String | The last modifier's email address. Generated based on the End User Credentials/LOAS role of the user making the call. |
| `certificate_id` | String | Certificate id for client certificate |
| `update_time` | String | Output only. The timestamp when the auth config is modified. |
| `creator_email` | String | The creator's email address. Generated based on the End User Credentials/LOAS role of the user making the call. |
| `description` | String | A description of the auth config. |
| `override_valid_time` | String | User provided expiry time to override. For the example of Salesforce, username/password credentials can be valid for 6 months depending on the instance settings. |
| `reason` | String | The reason / details of the current status. |
| `visibility` | String | The visibility of the auth config. |
| `create_time` | String | Output only. The timestamp when the auth config is created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create auth_config
auth_config = provider.integrations_api.Auth_config {
    parent = "value"  # Required. "projects/{project}/locations/{location}" format.
}

# Access auth_config outputs
auth_config_id = auth_config.id
auth_config_display_name = auth_config.display_name
auth_config_valid_time = auth_config.valid_time
auth_config_decrypted_credential = auth_config.decrypted_credential
auth_config_encrypted_credential = auth_config.encrypted_credential
auth_config_expiry_notification_duration = auth_config.expiry_notification_duration
auth_config_state = auth_config.state
auth_config_credential_type = auth_config.credential_type
auth_config_name = auth_config.name
auth_config_last_modifier_email = auth_config.last_modifier_email
auth_config_certificate_id = auth_config.certificate_id
auth_config_update_time = auth_config.update_time
auth_config_creator_email = auth_config.creator_email
auth_config_description = auth_config.description
auth_config_override_valid_time = auth_config.override_valid_time
auth_config_reason = auth_config.reason
auth_config_visibility = auth_config.visibility
auth_config_create_time = auth_config.create_time
```

---


### Runtime_entity_schema

Lists the JSON schemas for the properties of runtime entities, filtered by entity name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `runtime_entity_schemas` | Vec<String> | Runtime entity schemas. |
| `next_page_token` | String | Next page token. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access runtime_entity_schema outputs
runtime_entity_schema_id = runtime_entity_schema.id
runtime_entity_schema_runtime_entity_schemas = runtime_entity_schema.runtime_entity_schemas
runtime_entity_schema_next_page_token = runtime_entity_schema.next_page_token
```

---


### Connection

Lists Connections in a given project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Next page token. |
| `connections` | Vec<String> | Connections. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access connection outputs
connection_id = connection.id
connection_next_page_token = connection.next_page_token
connection_connections = connection.connections
```

---


### Integration

Executes an integration on receiving events from Integration Connector triggers, Eventarc or CPS Trigger. The details about integration are derived from CloudEvent request body.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cloud_event` | String |  | Required. Details about the trigger which is to be executed and values for input variable to the integration. |
| `name` | String | ✅ | Required. The integration resource name. Format: projects/{gcp_project_id}/locations/{location}/integrations/{integration_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The next page token for the response. |
| `integrations` | Vec<String> | The integrations which match the request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create integration
integration = provider.integrations_api.Integration {
    name = "value"  # Required. The integration resource name. Format: projects/{gcp_project_id}/locations/{location}/integrations/{integration_id}
}

# Access integration outputs
integration_id = integration.id
integration_next_page_token = integration.next_page_token
integration_integrations = integration.integrations
```

---


### Sfdc_channel

Creates an sfdc channel record. Store the sfdc channel in Spanner. Returns the sfdc channel.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delete_time` | String |  | Output only. Time when the channel was deleted. Empty if not deleted. |
| `last_replay_id` | String |  | Last sfdc messsage replay id for channel |
| `name` | String |  | Resource name of the SFDC channel projects/{project}/locations/{location}/sfdcInstances/{sfdc_instance}/sfdcChannels/{sfdc_channel}. |
| `update_time` | String |  | Output only. Time when the channel was last updated |
| `description` | String |  | The description for this channel |
| `display_name` | String |  | Client level unique name/alias to easily reference a channel. |
| `is_active` | bool |  | Indicated if a channel has any active integrations referencing it. Set to false when the channel is created, and set to true if there is any integration published with the channel configured in it. |
| `channel_topic` | String |  | The Channel topic defined by salesforce once an channel is opened |
| `create_time` | String |  | Output only. Time when the channel is created |
| `parent` | String | ✅ | Required. "projects/{project}/locations/{location}" format. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delete_time` | String | Output only. Time when the channel was deleted. Empty if not deleted. |
| `last_replay_id` | String | Last sfdc messsage replay id for channel |
| `name` | String | Resource name of the SFDC channel projects/{project}/locations/{location}/sfdcInstances/{sfdc_instance}/sfdcChannels/{sfdc_channel}. |
| `update_time` | String | Output only. Time when the channel was last updated |
| `description` | String | The description for this channel |
| `display_name` | String | Client level unique name/alias to easily reference a channel. |
| `is_active` | bool | Indicated if a channel has any active integrations referencing it. Set to false when the channel is created, and set to true if there is any integration published with the channel configured in it. |
| `channel_topic` | String | The Channel topic defined by salesforce once an channel is opened |
| `create_time` | String | Output only. Time when the channel is created |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sfdc_channel
sfdc_channel = provider.integrations_api.Sfdc_channel {
    parent = "value"  # Required. "projects/{project}/locations/{location}" format.
}

# Access sfdc_channel outputs
sfdc_channel_id = sfdc_channel.id
sfdc_channel_delete_time = sfdc_channel.delete_time
sfdc_channel_last_replay_id = sfdc_channel.last_replay_id
sfdc_channel_name = sfdc_channel.name
sfdc_channel_update_time = sfdc_channel.update_time
sfdc_channel_description = sfdc_channel.description
sfdc_channel_display_name = sfdc_channel.display_name
sfdc_channel_is_active = sfdc_channel.is_active
sfdc_channel_channel_topic = sfdc_channel.channel_topic
sfdc_channel_create_time = sfdc_channel.create_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple sfdc_instance resources
sfdc_instance_0 = provider.integrations_api.Sfdc_instance {
    parent = "value-0"
}
sfdc_instance_1 = provider.integrations_api.Sfdc_instance {
    parent = "value-1"
}
sfdc_instance_2 = provider.integrations_api.Sfdc_instance {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    sfdc_instance = provider.integrations_api.Sfdc_instance {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Integrations_api Documentation](https://cloud.google.com/integrations_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
