# Dialogflow_api Service



**Resources**: 101

---

## Overview

The dialogflow_api service provides access to 101 resource types:

- [Operation](#operation) [CR]
- [Result](#result) [R]
- [Playbook](#playbook) [CRUD]
- [Example](#example) [CRUD]
- [Transition_route_group](#transition_route_group) [CRUD]
- [Entity_type](#entity_type) [CRUD]
- [Deployment](#deployment) [R]
- [Location](#location) [R]
- [Environment](#environment) [CRUD]
- [Version](#version) [CRUD]
- [Conversation](#conversation) [RD]
- [Webhook](#webhook) [CRUD]
- [Generator](#generator) [CRUD]
- [Changelog](#changelog) [R]
- [Intent](#intent) [CRUD]
- [Agent](#agent) [CRUD]
- [Flow](#flow) [CRUD]
- [Page](#page) [CRUD]
- [Tool](#tool) [CRUD]
- [Session](#session) [C]
- [Security_setting](#security_setting) [CRUD]
- [Test_case](#test_case) [CRU]
- [Experiment](#experiment) [CRUD]
- [Continuous_test_result](#continuous_test_result) [R]
- [Answer_record](#answer_record) [RU]
- [Document](#document) [CRUD]
- [Entity_type](#entity_type) [CRUD]
- [Evaluation](#evaluation) [CRD]
- [Suggestion](#suggestion) [CR]
- [Generator](#generator) [CRUD]
- [Conversation_profile](#conversation_profile) [CRUD]
- [Project](#project) [CRD]
- [Session](#session) [CD]
- [Version](#version) [CRUD]
- [Phone_number](#phone_number) [CRUD]
- [Participant](#participant) [CRU]
- [Encryption_spec](#encryption_spec) [C]
- [Sip_trunk](#sip_trunk) [CRUD]
- [Agent](#agent) [CRU]
- [Intent](#intent) [CRUD]
- [Entitie](#entitie) [C]
- [Operation](#operation) [CR]
- [Tool](#tool) [CRUD]
- [Conversation](#conversation) [CR]
- [Message](#message) [CR]
- [Stateless_suggestion](#stateless_suggestion) [C]
- [Location](#location) [CRD]
- [Knowledge_base](#knowledge_base) [CRUD]
- [Environment](#environment) [CRUD]
- [Context](#context) [CRUD]
- [Location](#location) [CRD]
- [Conversation_dataset](#conversation_dataset) [CRD]
- [Version](#version) [CRUD]
- [Conversation_model](#conversation_model) [CRD]
- [Sip_trunk](#sip_trunk) [CRUD]
- [Knowledge_base](#knowledge_base) [CRUD]
- [Evaluation](#evaluation) [CRD]
- [Document](#document) [CRUD]
- [Suggestion](#suggestion) [C]
- [Entity_type](#entity_type) [CRUD]
- [Participant](#participant) [CRU]
- [Stateless_suggestion](#stateless_suggestion) [C]
- [Encryption_spec](#encryption_spec) [C]
- [Operation](#operation) [CR]
- [Context](#context) [CRUD]
- [Conversation_profile](#conversation_profile) [CRUD]
- [Conversation](#conversation) [CR]
- [Message](#message) [R]
- [Environment](#environment) [CRUD]
- [Generator](#generator) [CRUD]
- [Session](#session) [CD]
- [Intent](#intent) [CRUD]
- [Project](#project) [CRD]
- [Agent](#agent) [CRU]
- [Tool](#tool) [CRUD]
- [Answer_record](#answer_record) [RU]
- [Entitie](#entitie) [C]
- [Deployment](#deployment) [R]
- [Tool](#tool) [CRUD]
- [Page](#page) [CRUD]
- [Security_setting](#security_setting) [CRUD]
- [Agent](#agent) [CRUD]
- [Example](#example) [CRUD]
- [Location](#location) [R]
- [Experiment](#experiment) [CRUD]
- [Test_case](#test_case) [CRU]
- [Generator](#generator) [CRUD]
- [Operation](#operation) [CR]
- [Version](#version) [CRUD]
- [Continuous_test_result](#continuous_test_result) [R]
- [Session](#session) [C]
- [Flow](#flow) [CRUD]
- [Playbook](#playbook) [CRUD]
- [Environment](#environment) [CRUD]
- [Webhook](#webhook) [CRUD]
- [Entity_type](#entity_type) [CRUD]
- [Transition_route_group](#transition_route_group) [CRUD]
- [Intent](#intent) [CRUD]
- [Result](#result) [R]
- [Changelog](#changelog) [R]
- [Operation](#operation) [CR]

---

## Resources


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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.dialogflow_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
```

---


### Result

Gets a test case result.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `environment` | String | Environment where the test was run. If not set, it indicates the draft environment. |
| `name` | String | The resource name for the test case result. Format: `projects//locations//agents//testCases//results/`. |
| `test_result` | String | Whether the test case passed in the agent environment. |
| `conversation_turns` | Vec<String> | The conversation turns uttered during the test case replay in chronological order. |
| `test_time` | String | The time that the test was run. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access result outputs
result_id = result.id
result_environment = result.environment
result_name = result.name
result_test_result = result.test_result
result_conversation_turns = result.conversation_turns
result_test_time = result.test_time
```

---


### Playbook

Creates a playbook in a specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `output_parameter_definitions` | Vec<String> |  | Optional. Defined structured output parameters for this playbook. |
| `referenced_flows` | Vec<String> |  | Output only. The resource name of flows referenced by the current playbook in the instructions. |
| `create_time` | String |  | Output only. The timestamp of initial playbook creation. |
| `playbook_type` | String |  | Optional. Type of the playbook. |
| `code_block` | String |  | Optional. The playbook's scoped code block, which may implement handlers and actions. |
| `update_time` | String |  | Output only. Last time the playbook version was updated. |
| `instruction` | String |  | Instruction to accomplish target goal. |
| `handlers` | Vec<String> |  | Optional. A list of registered handlers to execute based on the specified triggers. |
| `goal` | String |  | Required. High level description of the goal the playbook intend to accomplish. A goal should be concise since it's visible to other playbooks that may reference this playbook. |
| `display_name` | String |  | Required. The human-readable name of the playbook, unique within an agent. |
| `llm_model_settings` | String |  | Optional. Llm model settings for the playbook. |
| `input_parameter_definitions` | Vec<String> |  | Optional. Defined structured input parameters for this playbook. |
| `inline_actions` | Vec<String> |  | Optional. Output only. Names of inline actions scoped to this playbook. These actions are in addition to those belonging to referenced tools, child playbooks, and flows, e.g. actions that are defined in the playbook's code block. |
| `token_count` | String |  | Output only. Estimated number of tokes current playbook takes when sent to the LLM. |
| `referenced_playbooks` | Vec<String> |  | Output only. The resource name of other playbooks referenced by the current playbook in the instructions. |
| `referenced_tools` | Vec<String> |  | Optional. The resource name of tools referenced by the current playbook in the instructions. If not provided explicitly, they are will be implied using the tool being referenced in goal and steps. |
| `name` | String |  | The unique identifier of the playbook. Format: `projects//locations//agents//playbooks/`. |
| `speech_settings` | String |  | Optional. Playbook level Settings for speech to text detection. |
| `parent` | String | ✅ | Required. The agent to create a playbook for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `output_parameter_definitions` | Vec<String> | Optional. Defined structured output parameters for this playbook. |
| `referenced_flows` | Vec<String> | Output only. The resource name of flows referenced by the current playbook in the instructions. |
| `create_time` | String | Output only. The timestamp of initial playbook creation. |
| `playbook_type` | String | Optional. Type of the playbook. |
| `code_block` | String | Optional. The playbook's scoped code block, which may implement handlers and actions. |
| `update_time` | String | Output only. Last time the playbook version was updated. |
| `instruction` | String | Instruction to accomplish target goal. |
| `handlers` | Vec<String> | Optional. A list of registered handlers to execute based on the specified triggers. |
| `goal` | String | Required. High level description of the goal the playbook intend to accomplish. A goal should be concise since it's visible to other playbooks that may reference this playbook. |
| `display_name` | String | Required. The human-readable name of the playbook, unique within an agent. |
| `llm_model_settings` | String | Optional. Llm model settings for the playbook. |
| `input_parameter_definitions` | Vec<String> | Optional. Defined structured input parameters for this playbook. |
| `inline_actions` | Vec<String> | Optional. Output only. Names of inline actions scoped to this playbook. These actions are in addition to those belonging to referenced tools, child playbooks, and flows, e.g. actions that are defined in the playbook's code block. |
| `token_count` | String | Output only. Estimated number of tokes current playbook takes when sent to the LLM. |
| `referenced_playbooks` | Vec<String> | Output only. The resource name of other playbooks referenced by the current playbook in the instructions. |
| `referenced_tools` | Vec<String> | Optional. The resource name of tools referenced by the current playbook in the instructions. If not provided explicitly, they are will be implied using the tool being referenced in goal and steps. |
| `name` | String | The unique identifier of the playbook. Format: `projects//locations//agents//playbooks/`. |
| `speech_settings` | String | Optional. Playbook level Settings for speech to text detection. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create playbook
playbook = provider.dialogflow_api.Playbook {
    parent = "value"  # Required. The agent to create a playbook for. Format: `projects//locations//agents/`.
}

# Access playbook outputs
playbook_id = playbook.id
playbook_output_parameter_definitions = playbook.output_parameter_definitions
playbook_referenced_flows = playbook.referenced_flows
playbook_create_time = playbook.create_time
playbook_playbook_type = playbook.playbook_type
playbook_code_block = playbook.code_block
playbook_update_time = playbook.update_time
playbook_instruction = playbook.instruction
playbook_handlers = playbook.handlers
playbook_goal = playbook.goal
playbook_display_name = playbook.display_name
playbook_llm_model_settings = playbook.llm_model_settings
playbook_input_parameter_definitions = playbook.input_parameter_definitions
playbook_inline_actions = playbook.inline_actions
playbook_token_count = playbook.token_count
playbook_referenced_playbooks = playbook.referenced_playbooks
playbook_referenced_tools = playbook.referenced_tools
playbook_name = playbook.name
playbook_speech_settings = playbook.speech_settings
```

---


### Example

Creates an example in the specified playbook.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The unique identifier of the playbook example. Format: `projects//locations//agents//playbooks//examples/`. |
| `conversation_state` | String |  | Required. Example's output state. |
| `token_count` | String |  | Output only. Estimated number of tokes current example takes when sent to the LLM. |
| `description` | String |  | Optional. The high level concise description of the example. The max number of characters is 200. |
| `update_time` | String |  | Output only. Last time the example was updated. |
| `create_time` | String |  | Output only. The timestamp of initial example creation. |
| `actions` | Vec<String> |  | Required. The ordered list of actions performed by the end user and the Dialogflow agent. |
| `playbook_output` | String |  | Optional. The output of the playbook in the example. |
| `playbook_input` | String |  | Optional. The input to the playbook in the example. |
| `display_name` | String |  | Required. The display name of the example. |
| `language_code` | String |  | Optional. The language code of the example. If not specified, the agent's default language is used. Note: languages must be enabled in the agent before they can be used. Note: example's language code is not currently used in dialogflow agents. |
| `parent` | String | ✅ | Required. The playbook to create an example for. Format: `projects//locations//agents//playbooks/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The unique identifier of the playbook example. Format: `projects//locations//agents//playbooks//examples/`. |
| `conversation_state` | String | Required. Example's output state. |
| `token_count` | String | Output only. Estimated number of tokes current example takes when sent to the LLM. |
| `description` | String | Optional. The high level concise description of the example. The max number of characters is 200. |
| `update_time` | String | Output only. Last time the example was updated. |
| `create_time` | String | Output only. The timestamp of initial example creation. |
| `actions` | Vec<String> | Required. The ordered list of actions performed by the end user and the Dialogflow agent. |
| `playbook_output` | String | Optional. The output of the playbook in the example. |
| `playbook_input` | String | Optional. The input to the playbook in the example. |
| `display_name` | String | Required. The display name of the example. |
| `language_code` | String | Optional. The language code of the example. If not specified, the agent's default language is used. Note: languages must be enabled in the agent before they can be used. Note: example's language code is not currently used in dialogflow agents. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create example
example = provider.dialogflow_api.Example {
    parent = "value"  # Required. The playbook to create an example for. Format: `projects//locations//agents//playbooks/`.
}

# Access example outputs
example_id = example.id
example_name = example.name
example_conversation_state = example.conversation_state
example_token_count = example.token_count
example_description = example.description
example_update_time = example.update_time
example_create_time = example.create_time
example_actions = example.actions
example_playbook_output = example.playbook_output
example_playbook_input = example.playbook_input
example_display_name = example.display_name
example_language_code = example.language_code
```

---


### Transition_route_group

Creates an TransitionRouteGroup in the specified flow. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The human-readable name of the transition route group, unique within the flow. The display name can be no longer than 30 characters. |
| `transition_routes` | Vec<String> |  | Transition routes associated with the TransitionRouteGroup. |
| `name` | String |  | The unique identifier of the transition route group. TransitionRouteGroups.CreateTransitionRouteGroup populates the name automatically. Format: `projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `parent` | String | ✅ | Required. The flow to create an TransitionRouteGroup for. Format: `projects//locations//agents//flows/` or `projects//locations//agents/` for agent-level groups. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The human-readable name of the transition route group, unique within the flow. The display name can be no longer than 30 characters. |
| `transition_routes` | Vec<String> | Transition routes associated with the TransitionRouteGroup. |
| `name` | String | The unique identifier of the transition route group. TransitionRouteGroups.CreateTransitionRouteGroup populates the name automatically. Format: `projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create transition_route_group
transition_route_group = provider.dialogflow_api.Transition_route_group {
    parent = "value"  # Required. The flow to create an TransitionRouteGroup for. Format: `projects//locations//agents//flows/` or `projects//locations//agents/` for agent-level groups.
}

# Access transition_route_group outputs
transition_route_group_id = transition_route_group.id
transition_route_group_display_name = transition_route_group.display_name
transition_route_group_transition_routes = transition_route_group.transition_routes
transition_route_group_name = transition_route_group.name
```

---


### Entity_type

Creates an entity type in the specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `redact` | bool |  | Indicates whether parameters of the entity type should be redacted in log. If redaction is enabled, page parameters and intent parameters referring to the entity type will be replaced by parameter name during logging. |
| `kind` | String |  | Required. Indicates the kind of entity type. |
| `entities` | Vec<String> |  | The collection of entity entries associated with the entity type. |
| `display_name` | String |  | Required. The human-readable name of the entity type, unique within the agent. |
| `excluded_phrases` | Vec<String> |  | Collection of exceptional words and phrases that shouldn't be matched. For example, if you have a size entity type with entry `giant`(an adjective), you might consider adding `giants`(a noun) as an exclusion. If the kind of entity type is `KIND_MAP`, then the phrases specified by entities and excluded phrases should be mutually exclusive. |
| `enable_fuzzy_extraction` | bool |  | Enables fuzzy entity extraction during classification. |
| `name` | String |  | The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType. Format: `projects//locations//agents//entityTypes/`. |
| `auto_expansion_mode` | String |  | Indicates whether the entity type can be automatically expanded. |
| `parent` | String | ✅ | Required. The agent to create a entity type for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `redact` | bool | Indicates whether parameters of the entity type should be redacted in log. If redaction is enabled, page parameters and intent parameters referring to the entity type will be replaced by parameter name during logging. |
| `kind` | String | Required. Indicates the kind of entity type. |
| `entities` | Vec<String> | The collection of entity entries associated with the entity type. |
| `display_name` | String | Required. The human-readable name of the entity type, unique within the agent. |
| `excluded_phrases` | Vec<String> | Collection of exceptional words and phrases that shouldn't be matched. For example, if you have a size entity type with entry `giant`(an adjective), you might consider adding `giants`(a noun) as an exclusion. If the kind of entity type is `KIND_MAP`, then the phrases specified by entities and excluded phrases should be mutually exclusive. |
| `enable_fuzzy_extraction` | bool | Enables fuzzy entity extraction during classification. |
| `name` | String | The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType. Format: `projects//locations//agents//entityTypes/`. |
| `auto_expansion_mode` | String | Indicates whether the entity type can be automatically expanded. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entity_type
entity_type = provider.dialogflow_api.Entity_type {
    parent = "value"  # Required. The agent to create a entity type for. Format: `projects//locations//agents/`.
}

# Access entity_type outputs
entity_type_id = entity_type.id
entity_type_redact = entity_type.redact
entity_type_kind = entity_type.kind
entity_type_entities = entity_type.entities
entity_type_display_name = entity_type.display_name
entity_type_excluded_phrases = entity_type.excluded_phrases
entity_type_enable_fuzzy_extraction = entity_type.enable_fuzzy_extraction
entity_type_name = entity_type.name
entity_type_auto_expansion_mode = entity_type.auto_expansion_mode
```

---


### Deployment

Retrieves the specified Deployment.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the deployment. Format: projects//locations//agents//environments//deployments/. |
| `end_time` | String | End time of this deployment. |
| `result` | String | Result of the deployment. |
| `start_time` | String | Start time of this deployment. |
| `state` | String | The current state of the deployment. |
| `flow_version` | String | The name of the flow version for this deployment. Format: projects//locations//agents//flows//versions/. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access deployment outputs
deployment_id = deployment.id
deployment_name = deployment.name
deployment_end_time = deployment.end_time
deployment_result = deployment.result
deployment_start_time = deployment.start_time
deployment_state = deployment.state
deployment_flow_version = deployment.flow_version
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |


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
location_location_id = location.location_id
location_metadata = location.metadata
location_display_name = location.display_name
location_labels = location.labels
location_name = location.name
```

---


### Environment

Creates an Environment in the specified Agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: Environment

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Update time of this environment. |
| `version_configs` | Vec<String> |  | A list of configurations for flow versions. You should include version configs for all flows that are reachable from `Start Flow` in the agent. Otherwise, an error will be returned. |
| `webhook_config` | String |  | The webhook configuration for this environment. |
| `display_name` | String |  | Required. The human-readable name of the environment (unique in an agent). Limit of 64 characters. |
| `name` | String |  | The name of the environment. Format: `projects//locations//agents//environments/`. |
| `test_cases_config` | String |  | The test cases config for continuous tests of this environment. |
| `description` | String |  | The human-readable description of the environment. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `parent` | String | ✅ | Required. The Agent to create an Environment for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Update time of this environment. |
| `version_configs` | Vec<String> | A list of configurations for flow versions. You should include version configs for all flows that are reachable from `Start Flow` in the agent. Otherwise, an error will be returned. |
| `webhook_config` | String | The webhook configuration for this environment. |
| `display_name` | String | Required. The human-readable name of the environment (unique in an agent). Limit of 64 characters. |
| `name` | String | The name of the environment. Format: `projects//locations//agents//environments/`. |
| `test_cases_config` | String | The test cases config for continuous tests of this environment. |
| `description` | String | The human-readable description of the environment. The maximum length is 500 characters. If exceeded, the request is rejected. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create environment
environment = provider.dialogflow_api.Environment {
    parent = "value"  # Required. The Agent to create an Environment for. Format: `projects//locations//agents/`.
}

# Access environment outputs
environment_id = environment.id
environment_update_time = environment.update_time
environment_version_configs = environment.version_configs
environment_webhook_config = environment.webhook_config
environment_display_name = environment.display_name
environment_name = environment.name
environment_test_cases_config = environment.test_cases_config
environment_description = environment.description
```

---


### Version

Creates a version for the specified Tool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The display name of the tool version. |
| `name` | String |  | Identifier. The unique identifier of the tool version. Format: `projects//locations//agents//tools//versions/`. |
| `tool` | String |  | Required. Snapshot of the tool to be associated with this version. |
| `create_time` | String |  | Output only. Last time the tool version was created or modified. |
| `update_time` | String |  | Output only. Last time the tool version was created or modified. |
| `parent` | String | ✅ | Required. The tool to create a version for. Format: `projects//locations//agents//tools/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The display name of the tool version. |
| `name` | String | Identifier. The unique identifier of the tool version. Format: `projects//locations//agents//tools//versions/`. |
| `tool` | String | Required. Snapshot of the tool to be associated with this version. |
| `create_time` | String | Output only. Last time the tool version was created or modified. |
| `update_time` | String | Output only. Last time the tool version was created or modified. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.dialogflow_api.Version {
    parent = "value"  # Required. The tool to create a version for. Format: `projects//locations//agents//tools/`.
}

# Access version outputs
version_id = version.id
version_display_name = version.display_name
version_name = version.name
version_tool = version.tool
version_create_time = version.create_time
version_update_time = version.update_time
```

---


### Conversation

Retrieves the specified conversation.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | The type of the conversation. |
| `flows` | Vec<String> | All the Flow the conversation has went through. Only `name` and `display_name` are filled in this message. |
| `interactions` | Vec<String> | Interactions of the conversation. Only populated for `GetConversation` and empty for `ListConversations`. |
| `name` | String | Identifier. The identifier of the conversation. If conversation ID is reused, interactions happened later than 48 hours of the conversation's create time will be ignored. Format: `projects//locations//agents//conversations/` |
| `environment` | String | Environment of the conversation. Only `name` and `display_name` are filled in this message. |
| `language_code` | String | The language of the conversation, which is the language of the first request in the conversation. |
| `intents` | Vec<String> | All the matched Intent in the conversation. Only `name` and `display_name` are filled in this message. |
| `metrics` | String | Conversation metrics. |
| `start_time` | String | Start time of the conversation, which is the time of the first request of the conversation. |
| `pages` | Vec<String> | All the Page the conversation has went through. Only `name` and `display_name` are filled in this message. |
| `duration` | String | Duration of the conversation. |
| `flow_versions` | HashMap<String, String> | Flow versions used in the conversation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access conversation outputs
conversation_id = conversation.id
conversation_type = conversation.type
conversation_flows = conversation.flows
conversation_interactions = conversation.interactions
conversation_name = conversation.name
conversation_environment = conversation.environment
conversation_language_code = conversation.language_code
conversation_intents = conversation.intents
conversation_metrics = conversation.metrics
conversation_start_time = conversation.start_time
conversation_pages = conversation.pages
conversation_duration = conversation.duration
conversation_flow_versions = conversation.flow_versions
```

---


### Webhook

Creates a webhook in the specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The human-readable name of the webhook, unique within the agent. |
| `disabled` | bool |  | Indicates whether the webhook is disabled. |
| `timeout` | String |  | Webhook execution timeout. Execution is considered failed if Dialogflow doesn't receive a response from webhook at the end of the timeout period. Defaults to 5 seconds, maximum allowed timeout is 30 seconds. |
| `generic_web_service` | String |  | Configuration for a generic web service. |
| `name` | String |  | The unique identifier of the webhook. Required for the Webhooks.UpdateWebhook method. Webhooks.CreateWebhook populates the name automatically. Format: `projects//locations//agents//webhooks/`. |
| `service_directory` | String |  | Configuration for a [Service Directory](https://cloud.google.com/service-directory) service. |
| `parent` | String | ✅ | Required. The agent to create a webhook for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The human-readable name of the webhook, unique within the agent. |
| `disabled` | bool | Indicates whether the webhook is disabled. |
| `timeout` | String | Webhook execution timeout. Execution is considered failed if Dialogflow doesn't receive a response from webhook at the end of the timeout period. Defaults to 5 seconds, maximum allowed timeout is 30 seconds. |
| `generic_web_service` | String | Configuration for a generic web service. |
| `name` | String | The unique identifier of the webhook. Required for the Webhooks.UpdateWebhook method. Webhooks.CreateWebhook populates the name automatically. Format: `projects//locations//agents//webhooks/`. |
| `service_directory` | String | Configuration for a [Service Directory](https://cloud.google.com/service-directory) service. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create webhook
webhook = provider.dialogflow_api.Webhook {
    parent = "value"  # Required. The agent to create a webhook for. Format: `projects//locations//agents/`.
}

# Access webhook outputs
webhook_id = webhook.id
webhook_display_name = webhook.display_name
webhook_disabled = webhook.disabled
webhook_timeout = webhook.timeout
webhook_generic_web_service = webhook.generic_web_service
webhook_name = webhook.name
webhook_service_directory = webhook.service_directory
```

---


### Generator

Creates a generator in the specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `prompt_text` | String |  | Required. Prompt for the LLM model. |
| `llm_model_settings` | String |  | The LLM model settings. |
| `model_parameter` | String |  | Parameters passed to the LLM to configure its behavior. |
| `display_name` | String |  | Required. The human-readable name of the generator, unique within the agent. The prompt contains pre-defined parameters such as $conversation, $last-user-utterance, etc. populated by Dialogflow. It can also contain custom placeholders which will be resolved during fulfillment. |
| `name` | String |  | The unique identifier of the generator. Must be set for the Generators.UpdateGenerator method. Generators.CreateGenerate populates the name automatically. Format: `projects//locations//agents//generators/`. |
| `placeholders` | Vec<String> |  | Optional. List of custom placeholders in the prompt text. |
| `parent` | String | ✅ | Required. The agent to create a generator for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `prompt_text` | String | Required. Prompt for the LLM model. |
| `llm_model_settings` | String | The LLM model settings. |
| `model_parameter` | String | Parameters passed to the LLM to configure its behavior. |
| `display_name` | String | Required. The human-readable name of the generator, unique within the agent. The prompt contains pre-defined parameters such as $conversation, $last-user-utterance, etc. populated by Dialogflow. It can also contain custom placeholders which will be resolved during fulfillment. |
| `name` | String | The unique identifier of the generator. Must be set for the Generators.UpdateGenerator method. Generators.CreateGenerate populates the name automatically. Format: `projects//locations//agents//generators/`. |
| `placeholders` | Vec<String> | Optional. List of custom placeholders in the prompt text. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create generator
generator = provider.dialogflow_api.Generator {
    parent = "value"  # Required. The agent to create a generator for. Format: `projects//locations//agents/`.
}

# Access generator outputs
generator_id = generator.id
generator_prompt_text = generator.prompt_text
generator_llm_model_settings = generator.llm_model_settings
generator_model_parameter = generator.model_parameter
generator_display_name = generator.display_name
generator_name = generator.name
generator_placeholders = generator.placeholders
```

---


### Changelog

Retrieves the specified Changelog.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The affected resource display name of the change. |
| `create_time` | String | The timestamp of the change. |
| `action` | String | The action of the change. |
| `user_email` | String | Email address of the authenticated user. |
| `resource` | String | The affected resource name of the change. |
| `type` | String | The affected resource type. |
| `language_code` | String | The affected language code of the change. |
| `name` | String | The unique identifier of the changelog. Format: `projects//locations//agents//changelogs/`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access changelog outputs
changelog_id = changelog.id
changelog_display_name = changelog.display_name
changelog_create_time = changelog.create_time
changelog_action = changelog.action
changelog_user_email = changelog.user_email
changelog_resource = changelog.resource
changelog_type = changelog.type
changelog_language_code = changelog.language_code
changelog_name = changelog.name
```

---


### Intent

Creates an intent in the specified agent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The unique identifier of the intent. Required for the Intents.UpdateIntent method. Intents.CreateIntent populates the name automatically. Format: `projects//locations//agents//intents/`. |
| `display_name` | String |  | Required. The human-readable name of the intent, unique within the agent. |
| `parameters` | Vec<String> |  | The collection of parameters associated with the intent. |
| `training_phrases` | Vec<String> |  | The collection of training phrases the agent is trained on to identify the intent. |
| `priority` | i64 |  | The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests. |
| `is_fallback` | bool |  | Indicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation. Adding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event. |
| `description` | String |  | Human readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters. |
| `labels` | HashMap<String, String> |  | The key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes. Prefix "sys-" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys-head * sys-contextual The above labels do not require value. "sys-head" means the intent is a head intent. "sys-contextual" means the intent is a contextual intent. |
| `parent` | String | ✅ | Required. The agent to create an intent for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The unique identifier of the intent. Required for the Intents.UpdateIntent method. Intents.CreateIntent populates the name automatically. Format: `projects//locations//agents//intents/`. |
| `display_name` | String | Required. The human-readable name of the intent, unique within the agent. |
| `parameters` | Vec<String> | The collection of parameters associated with the intent. |
| `training_phrases` | Vec<String> | The collection of training phrases the agent is trained on to identify the intent. |
| `priority` | i64 | The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests. |
| `is_fallback` | bool | Indicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation. Adding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event. |
| `description` | String | Human readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters. |
| `labels` | HashMap<String, String> | The key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes. Prefix "sys-" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys-head * sys-contextual The above labels do not require value. "sys-head" means the intent is a head intent. "sys-contextual" means the intent is a contextual intent. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create intent
intent = provider.dialogflow_api.Intent {
    parent = "value"  # Required. The agent to create an intent for. Format: `projects//locations//agents/`.
}

# Access intent outputs
intent_id = intent.id
intent_name = intent.name
intent_display_name = intent.display_name
intent_parameters = intent.parameters
intent_training_phrases = intent.training_phrases
intent_priority = intent.priority
intent_is_fallback = intent.is_fallback
intent_description = intent.description
intent_labels = intent.labels
```

---


### Agent

Creates an agent in the specified location. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_settings` | String |  | Name of the SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`. |
| `display_name` | String |  | Required. The human-readable name of the agent, unique within the location. |
| `enable_spell_correction` | bool |  | Indicates if automatic spell correction is enabled in detect intent requests. |
| `satisfies_pzi` | bool |  | Optional. Output only. A read only boolean field reflecting Zone Isolation status of the agent. |
| `start_flow` | String |  | Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//flows/`. Currently only the default start flow with id "00000000-0000-0000-0000-000000000000" is allowed. |
| `advanced_settings` | String |  | Hierarchical advanced settings for this agent. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `gen_app_builder_settings` | String |  | Gen App Builder-related agent-level settings. |
| `default_language_code` | String |  | Required. Immutable. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the Agents.UpdateAgent method. |
| `start_playbook` | String |  | Name of the start playbook in this agent. A start playbook will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//playbooks/`. Currently only the default playbook with id "00000000-0000-0000-0000-000000000000" is allowed. |
| `description` | String |  | The description of the agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `supported_language_codes` | Vec<String> |  | The list of all languages supported by the agent (except for the `default_language_code`). |
| `text_to_speech_settings` | String |  | Settings on instructing the speech synthesizer on how to generate the output audio content. |
| `avatar_uri` | String |  | The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `bigquery_export_settings` | String |  | Optional. The BigQuery export settings for this agent. The conversation data will be exported to BigQuery tables if it is enabled. By default, BigQuery export settings will not be exported with agent. You need to set include_bigquery_export_settings to include it in the exported agent. |
| `name` | String |  | The unique identifier of the agent. Required for the Agents.UpdateAgent method. Agents.CreateAgent populates the name automatically. Format: `projects//locations//agents/`. |
| `personalization_settings` | String |  | Optional. Settings for end user personalization. |
| `enable_stackdriver_logging` | bool |  | Indicates if stackdriver logging is enabled for the agent. Please use agent.advanced_settings instead. |
| `git_integration_settings` | String |  | Git integration settings for this agent. |
| `locked` | bool |  | Indicates whether the agent is locked for changes. If the agent is locked, modifications to the agent will be rejected except for RestoreAgent. |
| `speech_to_text_settings` | String |  | Speech recognition related settings. |
| `time_zone` | String |  | Required. The time zone of the agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `client_certificate_settings` | String |  | Optional. Settings for custom client certificates. |
| `answer_feedback_settings` | String |  | Optional. Answer feedback collection settings. |
| `enable_multi_language_training` | bool |  | Optional. Enable training multi-lingual models for this agent. These models will be trained on all the languages supported by the agent. |
| `satisfies_pzs` | bool |  | Optional. Output only. A read only boolean field reflecting Zone Separation status of the agent. |
| `parent` | String | ✅ | Required. The location to create a agent for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `security_settings` | String | Name of the SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`. |
| `display_name` | String | Required. The human-readable name of the agent, unique within the location. |
| `enable_spell_correction` | bool | Indicates if automatic spell correction is enabled in detect intent requests. |
| `satisfies_pzi` | bool | Optional. Output only. A read only boolean field reflecting Zone Isolation status of the agent. |
| `start_flow` | String | Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//flows/`. Currently only the default start flow with id "00000000-0000-0000-0000-000000000000" is allowed. |
| `advanced_settings` | String | Hierarchical advanced settings for this agent. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `gen_app_builder_settings` | String | Gen App Builder-related agent-level settings. |
| `default_language_code` | String | Required. Immutable. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the Agents.UpdateAgent method. |
| `start_playbook` | String | Name of the start playbook in this agent. A start playbook will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//playbooks/`. Currently only the default playbook with id "00000000-0000-0000-0000-000000000000" is allowed. |
| `description` | String | The description of the agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `supported_language_codes` | Vec<String> | The list of all languages supported by the agent (except for the `default_language_code`). |
| `text_to_speech_settings` | String | Settings on instructing the speech synthesizer on how to generate the output audio content. |
| `avatar_uri` | String | The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `bigquery_export_settings` | String | Optional. The BigQuery export settings for this agent. The conversation data will be exported to BigQuery tables if it is enabled. By default, BigQuery export settings will not be exported with agent. You need to set include_bigquery_export_settings to include it in the exported agent. |
| `name` | String | The unique identifier of the agent. Required for the Agents.UpdateAgent method. Agents.CreateAgent populates the name automatically. Format: `projects//locations//agents/`. |
| `personalization_settings` | String | Optional. Settings for end user personalization. |
| `enable_stackdriver_logging` | bool | Indicates if stackdriver logging is enabled for the agent. Please use agent.advanced_settings instead. |
| `git_integration_settings` | String | Git integration settings for this agent. |
| `locked` | bool | Indicates whether the agent is locked for changes. If the agent is locked, modifications to the agent will be rejected except for RestoreAgent. |
| `speech_to_text_settings` | String | Speech recognition related settings. |
| `time_zone` | String | Required. The time zone of the agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `client_certificate_settings` | String | Optional. Settings for custom client certificates. |
| `answer_feedback_settings` | String | Optional. Answer feedback collection settings. |
| `enable_multi_language_training` | bool | Optional. Enable training multi-lingual models for this agent. These models will be trained on all the languages supported by the agent. |
| `satisfies_pzs` | bool | Optional. Output only. A read only boolean field reflecting Zone Separation status of the agent. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create agent
agent = provider.dialogflow_api.Agent {
    parent = "value"  # Required. The location to create a agent for. Format: `projects//locations/`.
}

# Access agent outputs
agent_id = agent.id
agent_security_settings = agent.security_settings
agent_display_name = agent.display_name
agent_enable_spell_correction = agent.enable_spell_correction
agent_satisfies_pzi = agent.satisfies_pzi
agent_start_flow = agent.start_flow
agent_advanced_settings = agent.advanced_settings
agent_gen_app_builder_settings = agent.gen_app_builder_settings
agent_default_language_code = agent.default_language_code
agent_start_playbook = agent.start_playbook
agent_description = agent.description
agent_supported_language_codes = agent.supported_language_codes
agent_text_to_speech_settings = agent.text_to_speech_settings
agent_avatar_uri = agent.avatar_uri
agent_bigquery_export_settings = agent.bigquery_export_settings
agent_name = agent.name
agent_personalization_settings = agent.personalization_settings
agent_enable_stackdriver_logging = agent.enable_stackdriver_logging
agent_git_integration_settings = agent.git_integration_settings
agent_locked = agent.locked
agent_speech_to_text_settings = agent.speech_to_text_settings
agent_time_zone = agent.time_zone
agent_client_certificate_settings = agent.client_certificate_settings
agent_answer_feedback_settings = agent.answer_feedback_settings
agent_enable_multi_language_training = agent.enable_multi_language_training
agent_satisfies_pzs = agent.satisfies_pzs
```

---


### Flow

Creates a flow in the specified agent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `nlu_settings` | String |  | NLU related settings of the flow. |
| `name` | String |  | The unique identifier of the flow. Format: `projects//locations//agents//flows/`. |
| `event_handlers` | Vec<String> |  | A flow's event handlers serve two purposes: * They are responsible for handling events (e.g. no match, webhook errors) in the flow. * They are inherited by every page's event handlers, which can be used to handle common events regardless of the current page. Event handlers defined in the page have higher priority than those defined in the flow. Unlike transition_routes, these handlers are evaluated on a first-match basis. The first one that matches the event get executed, with the rest being ignored. |
| `locked` | bool |  | Indicates whether the flow is locked for changes. If the flow is locked, modifications to the flow will be rejected. |
| `multi_language_settings` | String |  | Optional. Multi-lingual agent settings for this flow. |
| `output_parameter_definitions` | Vec<String> |  | Optional. Defined structured output parameters for this flow. |
| `input_parameter_definitions` | Vec<String> |  | Optional. Defined structured input parameters for this flow. |
| `advanced_settings` | String |  | Hierarchical advanced settings for this flow. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `transition_routes` | Vec<String> |  | A flow's transition routes serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition routes and can support use cases such as the user saying "help" or "can I talk to a human?", which can be handled in a common way regardless of the current page. Transition routes defined in the page have higher priority than those defined in the flow. TransitionRoutes are evaluated in the following order: * TransitionRoutes with intent specified. * TransitionRoutes with only condition specified. TransitionRoutes with intent specified are inherited by pages in the flow. |
| `knowledge_connector_settings` | String |  | Optional. Knowledge connector configuration. |
| `description` | String |  | The description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `transition_route_groups` | Vec<String> |  | A flow's transition route group serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition route groups. Transition route groups defined in the page have higher priority than those defined in the flow. Format:`projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `display_name` | String |  | Required. The human-readable name of the flow. |
| `parent` | String | ✅ | Required. The agent to create a flow for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `nlu_settings` | String | NLU related settings of the flow. |
| `name` | String | The unique identifier of the flow. Format: `projects//locations//agents//flows/`. |
| `event_handlers` | Vec<String> | A flow's event handlers serve two purposes: * They are responsible for handling events (e.g. no match, webhook errors) in the flow. * They are inherited by every page's event handlers, which can be used to handle common events regardless of the current page. Event handlers defined in the page have higher priority than those defined in the flow. Unlike transition_routes, these handlers are evaluated on a first-match basis. The first one that matches the event get executed, with the rest being ignored. |
| `locked` | bool | Indicates whether the flow is locked for changes. If the flow is locked, modifications to the flow will be rejected. |
| `multi_language_settings` | String | Optional. Multi-lingual agent settings for this flow. |
| `output_parameter_definitions` | Vec<String> | Optional. Defined structured output parameters for this flow. |
| `input_parameter_definitions` | Vec<String> | Optional. Defined structured input parameters for this flow. |
| `advanced_settings` | String | Hierarchical advanced settings for this flow. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `transition_routes` | Vec<String> | A flow's transition routes serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition routes and can support use cases such as the user saying "help" or "can I talk to a human?", which can be handled in a common way regardless of the current page. Transition routes defined in the page have higher priority than those defined in the flow. TransitionRoutes are evaluated in the following order: * TransitionRoutes with intent specified. * TransitionRoutes with only condition specified. TransitionRoutes with intent specified are inherited by pages in the flow. |
| `knowledge_connector_settings` | String | Optional. Knowledge connector configuration. |
| `description` | String | The description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `transition_route_groups` | Vec<String> | A flow's transition route group serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition route groups. Transition route groups defined in the page have higher priority than those defined in the flow. Format:`projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `display_name` | String | Required. The human-readable name of the flow. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create flow
flow = provider.dialogflow_api.Flow {
    parent = "value"  # Required. The agent to create a flow for. Format: `projects//locations//agents/`.
}

# Access flow outputs
flow_id = flow.id
flow_nlu_settings = flow.nlu_settings
flow_name = flow.name
flow_event_handlers = flow.event_handlers
flow_locked = flow.locked
flow_multi_language_settings = flow.multi_language_settings
flow_output_parameter_definitions = flow.output_parameter_definitions
flow_input_parameter_definitions = flow.input_parameter_definitions
flow_advanced_settings = flow.advanced_settings
flow_transition_routes = flow.transition_routes
flow_knowledge_connector_settings = flow.knowledge_connector_settings
flow_description = flow.description
flow_transition_route_groups = flow.transition_route_groups
flow_display_name = flow.display_name
```

---


### Page

Creates a page in the specified flow.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | The description of the page. The maximum length is 500 characters. |
| `display_name` | String |  | Required. The human-readable name of the page, unique within the flow. |
| `event_handlers` | Vec<String> |  | Handlers associated with the page to handle events such as webhook errors, no match or no input. |
| `form` | String |  | The form associated with the page, used for collecting parameters relevant to the page. |
| `entry_fulfillment` | String |  | The fulfillment to call when the session is entering the page. |
| `name` | String |  | The unique identifier of the page. Required for the Pages.UpdatePage method. Pages.CreatePage populates the name automatically. Format: `projects//locations//agents//flows//pages/`. |
| `transition_route_groups` | Vec<String> |  | Ordered list of `TransitionRouteGroups` added to the page. Transition route groups must be unique within a page. If the page links both flow-level transition route groups and agent-level transition route groups, the flow-level ones will have higher priority and will be put before the agent-level ones. * If multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes. * If multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence. Format:`projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `advanced_settings` | String |  | Hierarchical advanced settings for this page. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `knowledge_connector_settings` | String |  | Optional. Knowledge connector configuration. |
| `transition_routes` | Vec<String> |  | A list of transitions for the transition rules of this page. They route the conversation to another page in the same flow, or another flow. When we are in a certain page, the TransitionRoutes are evaluated in the following order: * TransitionRoutes defined in the page with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in flow with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in the page with only condition specified. * TransitionRoutes defined in the transition route groups with only condition specified. |
| `parent` | String | ✅ | Required. The flow to create a page for. Format: `projects//locations//agents//flows/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | The description of the page. The maximum length is 500 characters. |
| `display_name` | String | Required. The human-readable name of the page, unique within the flow. |
| `event_handlers` | Vec<String> | Handlers associated with the page to handle events such as webhook errors, no match or no input. |
| `form` | String | The form associated with the page, used for collecting parameters relevant to the page. |
| `entry_fulfillment` | String | The fulfillment to call when the session is entering the page. |
| `name` | String | The unique identifier of the page. Required for the Pages.UpdatePage method. Pages.CreatePage populates the name automatically. Format: `projects//locations//agents//flows//pages/`. |
| `transition_route_groups` | Vec<String> | Ordered list of `TransitionRouteGroups` added to the page. Transition route groups must be unique within a page. If the page links both flow-level transition route groups and agent-level transition route groups, the flow-level ones will have higher priority and will be put before the agent-level ones. * If multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes. * If multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence. Format:`projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `advanced_settings` | String | Hierarchical advanced settings for this page. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `knowledge_connector_settings` | String | Optional. Knowledge connector configuration. |
| `transition_routes` | Vec<String> | A list of transitions for the transition rules of this page. They route the conversation to another page in the same flow, or another flow. When we are in a certain page, the TransitionRoutes are evaluated in the following order: * TransitionRoutes defined in the page with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in flow with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in the page with only condition specified. * TransitionRoutes defined in the transition route groups with only condition specified. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create page
page = provider.dialogflow_api.Page {
    parent = "value"  # Required. The flow to create a page for. Format: `projects//locations//agents//flows/`.
}

# Access page outputs
page_id = page.id
page_description = page.description
page_display_name = page.display_name
page_event_handlers = page.event_handlers
page_form = page.form
page_entry_fulfillment = page.entry_fulfillment
page_name = page.name
page_transition_route_groups = page.transition_route_groups
page_advanced_settings = page.advanced_settings
page_knowledge_connector_settings = page.knowledge_connector_settings
page_transition_routes = page.transition_routes
```

---


### Tool

Creates a Tool in the specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_store_spec` | String |  | Data store search tool specification. |
| `tool_type` | String |  | Output only. The tool type. |
| `extension_spec` | String |  | Vertex extension tool specification. |
| `function_spec` | String |  | Client side executed function specification. |
| `connector_spec` | String |  | Integration connectors tool specification. |
| `description` | String |  | Required. High level description of the Tool and its usage. |
| `display_name` | String |  | Required. The human-readable name of the Tool, unique within an agent. |
| `open_api_spec` | String |  | OpenAPI specification of the Tool. |
| `name` | String |  | The unique identifier of the Tool. Format: `projects//locations//agents//tools/`. |
| `parent` | String | ✅ | Required. The agent to create a Tool for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_store_spec` | String | Data store search tool specification. |
| `tool_type` | String | Output only. The tool type. |
| `extension_spec` | String | Vertex extension tool specification. |
| `function_spec` | String | Client side executed function specification. |
| `connector_spec` | String | Integration connectors tool specification. |
| `description` | String | Required. High level description of the Tool and its usage. |
| `display_name` | String | Required. The human-readable name of the Tool, unique within an agent. |
| `open_api_spec` | String | OpenAPI specification of the Tool. |
| `name` | String | The unique identifier of the Tool. Format: `projects//locations//agents//tools/`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tool
tool = provider.dialogflow_api.Tool {
    parent = "value"  # Required. The agent to create a Tool for. Format: `projects//locations//agents/`.
}

# Access tool outputs
tool_id = tool.id
tool_data_store_spec = tool.data_store_spec
tool_tool_type = tool.tool_type
tool_extension_spec = tool.extension_spec
tool_function_spec = tool.function_spec
tool_connector_spec = tool.connector_spec
tool_description = tool.description
tool_display_name = tool.display_name
tool_open_api_spec = tool.open_api_spec
tool_name = tool.name
```

---


### Session

Fulfills a matched intent returned by MatchIntent. Must be called after MatchIntent, with input from MatchIntentResponse. Otherwise, the behavior is undefined.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `match_intent_request` | String |  | Must be same as the corresponding MatchIntent request, otherwise the behavior is undefined. |
| `match` | String |  | The matched intent/event to fulfill. |
| `output_audio_config` | String |  | Instructs the speech synthesizer how to generate output audio. |
| `session` | String | ✅ | Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session). |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create session
session = provider.dialogflow_api.Session {
    session = "value"  # Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session).
}

```

---


### Security_setting

Create security settings in the specified location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `redaction_scope` | String |  | Defines the data for which Dialogflow applies redaction. Dialogflow does not redact data that it does not have access to – for example, Cloud logging. |
| `retention_window_days` | i64 |  | Retains data in interaction logging for the specified number of days. This does not apply to Cloud logging, which is owned by the user - not Dialogflow. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL. When data retention configuration is changed, it only applies to the data created after the change; the TTL of existing data created before the change stays intact. |
| `purge_data_types` | Vec<String> |  | List of types of data to remove when retention settings triggers purge. |
| `retention_strategy` | String |  | Specifies the retention behavior defined by SecuritySettings.RetentionStrategy. |
| `redaction_strategy` | String |  | Strategy that defines how we do redaction. |
| `inspect_template` | String |  | [DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. The `DLP Inspect Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, we use the default DLP inspect config. The template name will have one of the following formats: `projects//locations//inspectTemplates/` OR `organizations//locations//inspectTemplates/` Note: `inspect_template` must be located in the same region as the `SecuritySettings`. |
| `audio_export_settings` | String |  | Controls audio export settings for post-conversation analytics when ingesting audio to conversations via Participants.AnalyzeContent or Participants.StreamingAnalyzeContent. If retention_strategy is set to REMOVE_AFTER_CONVERSATION or audio_export_settings.gcs_bucket is empty, audio export is disabled. If audio export is enabled, audio is recorded and saved to audio_export_settings.gcs_bucket, subject to retention policy of audio_export_settings.gcs_bucket. This setting won't effect audio input for implicit sessions via Sessions.DetectIntent or Sessions.StreamingDetectIntent. |
| `insights_export_settings` | String |  | Controls conversation exporting settings to Insights after conversation is completed. If retention_strategy is set to REMOVE_AFTER_CONVERSATION, Insights export is disabled no matter what you configure here. |
| `deidentify_template` | String |  | [DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. The `DLP De-identify Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, Dialogflow replaces sensitive info with `[redacted]` text. The template name will have one of the following formats: `projects//locations//deidentifyTemplates/` OR `organizations//locations//deidentifyTemplates/` Note: `deidentify_template` must be located in the same region as the `SecuritySettings`. |
| `display_name` | String |  | Required. The human-readable name of the security settings, unique within the location. |
| `name` | String |  | Resource name of the settings. Required for the SecuritySettingsService.UpdateSecuritySettings method. SecuritySettingsService.CreateSecuritySettings populates the name automatically. Format: `projects//locations//securitySettings/`. |
| `parent` | String | ✅ | Required. The location to create an SecuritySettings for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `redaction_scope` | String | Defines the data for which Dialogflow applies redaction. Dialogflow does not redact data that it does not have access to – for example, Cloud logging. |
| `retention_window_days` | i64 | Retains data in interaction logging for the specified number of days. This does not apply to Cloud logging, which is owned by the user - not Dialogflow. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL. When data retention configuration is changed, it only applies to the data created after the change; the TTL of existing data created before the change stays intact. |
| `purge_data_types` | Vec<String> | List of types of data to remove when retention settings triggers purge. |
| `retention_strategy` | String | Specifies the retention behavior defined by SecuritySettings.RetentionStrategy. |
| `redaction_strategy` | String | Strategy that defines how we do redaction. |
| `inspect_template` | String | [DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. The `DLP Inspect Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, we use the default DLP inspect config. The template name will have one of the following formats: `projects//locations//inspectTemplates/` OR `organizations//locations//inspectTemplates/` Note: `inspect_template` must be located in the same region as the `SecuritySettings`. |
| `audio_export_settings` | String | Controls audio export settings for post-conversation analytics when ingesting audio to conversations via Participants.AnalyzeContent or Participants.StreamingAnalyzeContent. If retention_strategy is set to REMOVE_AFTER_CONVERSATION or audio_export_settings.gcs_bucket is empty, audio export is disabled. If audio export is enabled, audio is recorded and saved to audio_export_settings.gcs_bucket, subject to retention policy of audio_export_settings.gcs_bucket. This setting won't effect audio input for implicit sessions via Sessions.DetectIntent or Sessions.StreamingDetectIntent. |
| `insights_export_settings` | String | Controls conversation exporting settings to Insights after conversation is completed. If retention_strategy is set to REMOVE_AFTER_CONVERSATION, Insights export is disabled no matter what you configure here. |
| `deidentify_template` | String | [DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. The `DLP De-identify Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, Dialogflow replaces sensitive info with `[redacted]` text. The template name will have one of the following formats: `projects//locations//deidentifyTemplates/` OR `organizations//locations//deidentifyTemplates/` Note: `deidentify_template` must be located in the same region as the `SecuritySettings`. |
| `display_name` | String | Required. The human-readable name of the security settings, unique within the location. |
| `name` | String | Resource name of the settings. Required for the SecuritySettingsService.UpdateSecuritySettings method. SecuritySettingsService.CreateSecuritySettings populates the name automatically. Format: `projects//locations//securitySettings/`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_setting
security_setting = provider.dialogflow_api.Security_setting {
    parent = "value"  # Required. The location to create an SecuritySettings for. Format: `projects//locations/`.
}

# Access security_setting outputs
security_setting_id = security_setting.id
security_setting_redaction_scope = security_setting.redaction_scope
security_setting_retention_window_days = security_setting.retention_window_days
security_setting_purge_data_types = security_setting.purge_data_types
security_setting_retention_strategy = security_setting.retention_strategy
security_setting_redaction_strategy = security_setting.redaction_strategy
security_setting_inspect_template = security_setting.inspect_template
security_setting_audio_export_settings = security_setting.audio_export_settings
security_setting_insights_export_settings = security_setting.insights_export_settings
security_setting_deidentify_template = security_setting.deidentify_template
security_setting_display_name = security_setting.display_name
security_setting_name = security_setting.name
```

---


### Test_case

Creates a test case for the given agent.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The human-readable name of the test case, unique within the agent. Limit of 200 characters. |
| `test_case_conversation_turns` | Vec<String> |  | The conversation turns uttered when the test case was created, in chronological order. These include the canonical set of agent utterances that should occur when the agent is working properly. |
| `tags` | Vec<String> |  | Tags are short descriptions that users may apply to test cases for organizational and filtering purposes. Each tag should start with "#" and has a limit of 30 characters. |
| `notes` | String |  | Additional freeform notes about the test case. Limit of 400 characters. |
| `test_config` | String |  | Config for the test case. |
| `name` | String |  | The unique identifier of the test case. TestCases.CreateTestCase will populate the name automatically. Otherwise use format: `projects//locations//agents//testCases/`. |
| `creation_time` | String |  | Output only. When the test was created. |
| `last_test_result` | String |  | The latest test result. |
| `parent` | String | ✅ | Required. The agent to create the test case for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The human-readable name of the test case, unique within the agent. Limit of 200 characters. |
| `test_case_conversation_turns` | Vec<String> | The conversation turns uttered when the test case was created, in chronological order. These include the canonical set of agent utterances that should occur when the agent is working properly. |
| `tags` | Vec<String> | Tags are short descriptions that users may apply to test cases for organizational and filtering purposes. Each tag should start with "#" and has a limit of 30 characters. |
| `notes` | String | Additional freeform notes about the test case. Limit of 400 characters. |
| `test_config` | String | Config for the test case. |
| `name` | String | The unique identifier of the test case. TestCases.CreateTestCase will populate the name automatically. Otherwise use format: `projects//locations//agents//testCases/`. |
| `creation_time` | String | Output only. When the test was created. |
| `last_test_result` | String | The latest test result. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create test_case
test_case = provider.dialogflow_api.Test_case {
    parent = "value"  # Required. The agent to create the test case for. Format: `projects//locations//agents/`.
}

# Access test_case outputs
test_case_id = test_case.id
test_case_display_name = test_case.display_name
test_case_test_case_conversation_turns = test_case.test_case_conversation_turns
test_case_tags = test_case.tags
test_case_notes = test_case.notes
test_case_test_config = test_case.test_config
test_case_name = test_case.name
test_case_creation_time = test_case.creation_time
test_case_last_test_result = test_case.last_test_result
```

---


### Experiment

Creates an Experiment in the specified Environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Creation time of this experiment. |
| `start_time` | String |  | Start time of this experiment. |
| `experiment_length` | String |  | Maximum number of days to run the experiment. If auto-rollout is not enabled, default value and maximum will be 30 days. If auto-rollout is enabled, default value and maximum will be 6 days. |
| `rollout_state` | String |  | State of the auto rollout process. |
| `rollout_failure_reason` | String |  | The reason why rollout has failed. Should only be set when state is ROLLOUT_FAILED. |
| `end_time` | String |  | End time of this experiment. |
| `result` | String |  | Inference result of the experiment. |
| `rollout_config` | String |  | The configuration for auto rollout. If set, there should be exactly two variants in the experiment (control variant being the default version of the flow), the traffic allocation for the non-control variant will gradually increase to 100% when conditions are met, and eventually replace the control variant to become the default version of the flow. |
| `definition` | String |  | The definition of the experiment. |
| `description` | String |  | The human-readable description of the experiment. |
| `display_name` | String |  | Required. The human-readable name of the experiment (unique in an environment). Limit of 64 characters. |
| `variants_history` | Vec<String> |  | The history of updates to the experiment variants. |
| `last_update_time` | String |  | Last update time of this experiment. |
| `state` | String |  | The current state of the experiment. Transition triggered by Experiments.StartExperiment: DRAFT->RUNNING. Transition triggered by Experiments.CancelExperiment: DRAFT->DONE or RUNNING->DONE. |
| `name` | String |  | The name of the experiment. Format: projects//locations//agents//environments//experiments/. |
| `parent` | String | ✅ | Required. The Agent to create an Environment for. Format: `projects//locations//agents//environments/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Creation time of this experiment. |
| `start_time` | String | Start time of this experiment. |
| `experiment_length` | String | Maximum number of days to run the experiment. If auto-rollout is not enabled, default value and maximum will be 30 days. If auto-rollout is enabled, default value and maximum will be 6 days. |
| `rollout_state` | String | State of the auto rollout process. |
| `rollout_failure_reason` | String | The reason why rollout has failed. Should only be set when state is ROLLOUT_FAILED. |
| `end_time` | String | End time of this experiment. |
| `result` | String | Inference result of the experiment. |
| `rollout_config` | String | The configuration for auto rollout. If set, there should be exactly two variants in the experiment (control variant being the default version of the flow), the traffic allocation for the non-control variant will gradually increase to 100% when conditions are met, and eventually replace the control variant to become the default version of the flow. |
| `definition` | String | The definition of the experiment. |
| `description` | String | The human-readable description of the experiment. |
| `display_name` | String | Required. The human-readable name of the experiment (unique in an environment). Limit of 64 characters. |
| `variants_history` | Vec<String> | The history of updates to the experiment variants. |
| `last_update_time` | String | Last update time of this experiment. |
| `state` | String | The current state of the experiment. Transition triggered by Experiments.StartExperiment: DRAFT->RUNNING. Transition triggered by Experiments.CancelExperiment: DRAFT->DONE or RUNNING->DONE. |
| `name` | String | The name of the experiment. Format: projects//locations//agents//environments//experiments/. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create experiment
experiment = provider.dialogflow_api.Experiment {
    parent = "value"  # Required. The Agent to create an Environment for. Format: `projects//locations//agents//environments/`.
}

# Access experiment outputs
experiment_id = experiment.id
experiment_create_time = experiment.create_time
experiment_start_time = experiment.start_time
experiment_experiment_length = experiment.experiment_length
experiment_rollout_state = experiment.rollout_state
experiment_rollout_failure_reason = experiment.rollout_failure_reason
experiment_end_time = experiment.end_time
experiment_result = experiment.result
experiment_rollout_config = experiment.rollout_config
experiment_definition = experiment.definition
experiment_description = experiment.description
experiment_display_name = experiment.display_name
experiment_variants_history = experiment.variants_history
experiment_last_update_time = experiment.last_update_time
experiment_state = experiment.state
experiment_name = experiment.name
```

---


### Continuous_test_result

Fetches a list of continuous test results for a given environment.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |
| `continuous_test_results` | Vec<String> | The list of continuous test results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access continuous_test_result outputs
continuous_test_result_id = continuous_test_result.id
continuous_test_result_next_page_token = continuous_test_result.next_page_token
continuous_test_result_continuous_test_results = continuous_test_result.continuous_test_results
```

---


### Answer_record

Deprecated. Retrieves a specific answer record.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The unique identifier of this answer record. Required for AnswerRecords.UpdateAnswerRecord method. Format: `projects//locations//answerRecords/`. |
| `agent_assistant_record` | String |  | Output only. The record for human agent assistant. |
| `answer_feedback` | String |  | Optional. The AnswerFeedback for this record. You can set this with AnswerRecords.UpdateAnswerRecord in order to give us feedback about this answer. |
| `name` | String | ✅ | The unique identifier of this answer record. Required for AnswerRecords.UpdateAnswerRecord method. Format: `projects//locations//answerRecords/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The unique identifier of this answer record. Required for AnswerRecords.UpdateAnswerRecord method. Format: `projects//locations//answerRecords/`. |
| `agent_assistant_record` | String | Output only. The record for human agent assistant. |
| `answer_feedback` | String | Optional. The AnswerFeedback for this record. You can set this with AnswerRecords.UpdateAnswerRecord in order to give us feedback about this answer. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access answer_record outputs
answer_record_id = answer_record.id
answer_record_name = answer_record.name
answer_record_agent_assistant_record = answer_record.agent_assistant_record
answer_record_answer_feedback = answer_record.answer_feedback
```

---


### Document

Creates a new document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_auto_reload` | bool |  | Optional. If true, we try to automatically reload the document every day (at a time picked by the system). If false or unspecified, we don't try to automatically reload the document. Currently you can only enable automatic reload for documents sourced from a public url, see `source` field for the source types. Reload status can be tracked in `latest_reload_status`. If a reload fails, we will keep the document unchanged. If a reload fails with internal errors, the system will try to reload the document on the next day. If a reload fails with non-retriable errors (e.g. PERMISSION_DENIED), the system will not try to reload the document anymore. You need to manually reload the document successfully by calling `ReloadDocument` and clear the errors. |
| `raw_content` | String |  | The raw content of the document. This field is only permitted for EXTRACTIVE_QA and FAQ knowledge types. |
| `state` | String |  | Output only. The current state of the document. |
| `metadata` | HashMap<String, String> |  | Optional. Metadata for the document. The metadata supports arbitrary key-value pairs. Suggested use cases include storing a document's title, an external URL distinct from the document's content_uri, etc. The max size of a `key` or a `value` of the metadata is 1024 bytes. |
| `latest_reload_status` | String |  | Output only. The time and status of the latest reload. This reload may have been triggered automatically or manually and may not have succeeded. |
| `knowledge_types` | Vec<String> |  | Required. The knowledge type of document content. |
| `name` | String |  | Optional. The document resource name. The name must be empty when creating a document. Format: `projects//locations//knowledgeBases//documents/`. |
| `display_name` | String |  | Required. The display name of the document. The name must be 1024 bytes or less; otherwise, the creation request fails. |
| `content_uri` | String |  | The URI where the file content is located. For documents stored in Google Cloud Storage, these URIs must have the form `gs:///`. NOTE: External URLs must correspond to public webpages, i.e., they must be indexed by Google Search. In particular, URLs for showing documents in Google Cloud Storage (i.e. the URL in your browser) are not supported. Instead use the `gs://` format URI described above. |
| `content` | String |  | The raw content of the document. This field is only permitted for EXTRACTIVE_QA and FAQ knowledge types. Note: This field is in the process of being deprecated, please use raw_content instead. |
| `mime_type` | String |  | Required. The MIME type of this document. |
| `parent` | String | ✅ | Required. The knowledge base to create a document for. Format: `projects//locations//knowledgeBases/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enable_auto_reload` | bool | Optional. If true, we try to automatically reload the document every day (at a time picked by the system). If false or unspecified, we don't try to automatically reload the document. Currently you can only enable automatic reload for documents sourced from a public url, see `source` field for the source types. Reload status can be tracked in `latest_reload_status`. If a reload fails, we will keep the document unchanged. If a reload fails with internal errors, the system will try to reload the document on the next day. If a reload fails with non-retriable errors (e.g. PERMISSION_DENIED), the system will not try to reload the document anymore. You need to manually reload the document successfully by calling `ReloadDocument` and clear the errors. |
| `raw_content` | String | The raw content of the document. This field is only permitted for EXTRACTIVE_QA and FAQ knowledge types. |
| `state` | String | Output only. The current state of the document. |
| `metadata` | HashMap<String, String> | Optional. Metadata for the document. The metadata supports arbitrary key-value pairs. Suggested use cases include storing a document's title, an external URL distinct from the document's content_uri, etc. The max size of a `key` or a `value` of the metadata is 1024 bytes. |
| `latest_reload_status` | String | Output only. The time and status of the latest reload. This reload may have been triggered automatically or manually and may not have succeeded. |
| `knowledge_types` | Vec<String> | Required. The knowledge type of document content. |
| `name` | String | Optional. The document resource name. The name must be empty when creating a document. Format: `projects//locations//knowledgeBases//documents/`. |
| `display_name` | String | Required. The display name of the document. The name must be 1024 bytes or less; otherwise, the creation request fails. |
| `content_uri` | String | The URI where the file content is located. For documents stored in Google Cloud Storage, these URIs must have the form `gs:///`. NOTE: External URLs must correspond to public webpages, i.e., they must be indexed by Google Search. In particular, URLs for showing documents in Google Cloud Storage (i.e. the URL in your browser) are not supported. Instead use the `gs://` format URI described above. |
| `content` | String | The raw content of the document. This field is only permitted for EXTRACTIVE_QA and FAQ knowledge types. Note: This field is in the process of being deprecated, please use raw_content instead. |
| `mime_type` | String | Required. The MIME type of this document. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create document
document = provider.dialogflow_api.Document {
    parent = "value"  # Required. The knowledge base to create a document for. Format: `projects//locations//knowledgeBases/`.
}

# Access document outputs
document_id = document.id
document_enable_auto_reload = document.enable_auto_reload
document_raw_content = document.raw_content
document_state = document.state
document_metadata = document.metadata
document_latest_reload_status = document.latest_reload_status
document_knowledge_types = document.knowledge_types
document_name = document.name
document_display_name = document.display_name
document_content_uri = document.content_uri
document_content = document.content
document_mime_type = document.mime_type
```

---


### Entity_type

Creates an entity type in the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The name of the entity type. |
| `auto_expansion_mode` | String |  | Optional. Indicates whether the entity type can be automatically expanded. |
| `enable_fuzzy_extraction` | bool |  | Optional. Enables fuzzy entity extraction during classification. |
| `kind` | String |  | Required. Indicates the kind of entity type. |
| `name` | String |  | The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType and EntityTypes.BatchUpdateEntityTypes methods. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/` |
| `entities` | Vec<String> |  | Optional. The collection of entity entries associated with the entity type. |
| `parent` | String | ✅ | Required. The agent to create a entity type for. Supported formats: - `projects//agent` - `projects//locations//agent` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The name of the entity type. |
| `auto_expansion_mode` | String | Optional. Indicates whether the entity type can be automatically expanded. |
| `enable_fuzzy_extraction` | bool | Optional. Enables fuzzy entity extraction during classification. |
| `kind` | String | Required. Indicates the kind of entity type. |
| `name` | String | The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType and EntityTypes.BatchUpdateEntityTypes methods. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/` |
| `entities` | Vec<String> | Optional. The collection of entity entries associated with the entity type. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entity_type
entity_type = provider.dialogflow_api.Entity_type {
    parent = "value"  # Required. The agent to create a entity type for. Supported formats: - `projects//agent` - `projects//locations//agent`
}

# Access entity_type outputs
entity_type_id = entity_type.id
entity_type_display_name = entity_type.display_name
entity_type_auto_expansion_mode = entity_type.auto_expansion_mode
entity_type_enable_fuzzy_extraction = entity_type.enable_fuzzy_extraction
entity_type_kind = entity_type.kind
entity_type_name = entity_type.name
entity_type_entities = entity_type.entities
```

---


### Evaluation

Creates evaluation of a generator.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `evaluation_status` | String |  | Output only. The result status of the evaluation pipeline. Provides the status information including if the evaluation is still in progress, completed or failed with certain error and user actionable message. |
| `satisfies_pzs` | bool |  | Output only. A read only boolean field reflecting Zone Separation status of the model. The field is an aggregated value of ZS status of its underlying dependencies. See more details in go/zicy-resource-placement#resource-status |
| `name` | String |  | Output only. Identifier. The resource name of the evaluation. Format: `projects//locations//generators// evaluations/` |
| `summarization_metrics` | String |  | Output only. Only available when the summarization generator is provided. |
| `satisfies_pzi` | bool |  | Output only. A read only boolean field reflecting Zone Isolation status of the model. The field is an aggregated value of ZI status of its underlying dependencies. See more details in go/zicy-resource-placement#resource-status |
| `create_time` | String |  | Output only. Creation time of this generator evaluation. |
| `initial_generator` | String |  | Required. The initial generator that was used when creating this evaluation. This is a copy of the generator read from storage when creating the evaluation. |
| `generator_evaluation_config` | String |  | Required. The configuration of the evaluation task. |
| `display_name` | String |  | Optional. The display name of the generator evaluation. At most 64 bytes long. |
| `complete_time` | String |  | Output only. Completion time of this generator evaluation. |
| `parent` | String | ✅ | Required. The generator resource name. Format: `projects//locations//generators/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `evaluation_status` | String | Output only. The result status of the evaluation pipeline. Provides the status information including if the evaluation is still in progress, completed or failed with certain error and user actionable message. |
| `satisfies_pzs` | bool | Output only. A read only boolean field reflecting Zone Separation status of the model. The field is an aggregated value of ZS status of its underlying dependencies. See more details in go/zicy-resource-placement#resource-status |
| `name` | String | Output only. Identifier. The resource name of the evaluation. Format: `projects//locations//generators// evaluations/` |
| `summarization_metrics` | String | Output only. Only available when the summarization generator is provided. |
| `satisfies_pzi` | bool | Output only. A read only boolean field reflecting Zone Isolation status of the model. The field is an aggregated value of ZI status of its underlying dependencies. See more details in go/zicy-resource-placement#resource-status |
| `create_time` | String | Output only. Creation time of this generator evaluation. |
| `initial_generator` | String | Required. The initial generator that was used when creating this evaluation. This is a copy of the generator read from storage when creating the evaluation. |
| `generator_evaluation_config` | String | Required. The configuration of the evaluation task. |
| `display_name` | String | Optional. The display name of the generator evaluation. At most 64 bytes long. |
| `complete_time` | String | Output only. Completion time of this generator evaluation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create evaluation
evaluation = provider.dialogflow_api.Evaluation {
    parent = "value"  # Required. The generator resource name. Format: `projects//locations//generators/`
}

# Access evaluation outputs
evaluation_id = evaluation.id
evaluation_evaluation_status = evaluation.evaluation_status
evaluation_satisfies_pzs = evaluation.satisfies_pzs
evaluation_name = evaluation.name
evaluation_summarization_metrics = evaluation.summarization_metrics
evaluation_satisfies_pzi = evaluation.satisfies_pzi
evaluation_create_time = evaluation.create_time
evaluation_initial_generator = evaluation.initial_generator
evaluation_generator_evaluation_config = evaluation.generator_evaluation_config
evaluation_display_name = evaluation.display_name
evaluation_complete_time = evaluation.complete_time
```

---


### Suggestion

Gets suggested faq answers for a participant based on specific historical messages.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `assist_query_params` | String |  | Optional. Parameters for a human assist query. |
| `latest_message` | String |  | Optional. The name of the latest conversation message to compile suggestion for. If empty, it will be the latest message of the conversation. Format: `projects//locations//conversations//messages/`. |
| `context_size` | i64 |  | Optional. Max number of messages prior to and including [latest_message] to use as context when compiling the suggestion. By default 20 and at most 50. |
| `parent` | String | ✅ | Required. The name of the participant to fetch suggestion for. Format: `projects//locations//conversations//participants/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `suggestions` | Vec<String> | Required. The list of suggestions. There will be a maximum number of items returned based on the page_size field in the request. `suggestions` is sorted by `create_time` in descending order. |
| `next_page_token` | String | Optional. Token to retrieve the next page of results or empty if there are no more results in the list. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create suggestion
suggestion = provider.dialogflow_api.Suggestion {
    parent = "value"  # Required. The name of the participant to fetch suggestion for. Format: `projects//locations//conversations//participants/`.
}

# Access suggestion outputs
suggestion_id = suggestion.id
suggestion_suggestions = suggestion.suggestions
suggestion_next_page_token = suggestion.next_page_token
```

---


### Generator

Creates a generator.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `summarization_context` | String |  | Input of Summarization feature. |
| `trigger_event` | String |  | Optional. The trigger event of the generator. It defines when the generator is triggered in a conversation. |
| `update_time` | String |  | Output only. Update time of this generator. |
| `create_time` | String |  | Output only. Creation time of this generator. |
| `name` | String |  | Output only. Identifier. The resource name of the generator. Format: `projects//locations//generators/` |
| `published_model` | String |  | Optional. The published Large Language Model name. * To use the latest model version, specify the model name without version number. Example: `text-bison` * To use a stable model version, specify the version number as well. Example: `text-bison@002`. |
| `agent_coaching_context` | String |  | Input of Agent Coaching feature. |
| `description` | String |  | Optional. Human readable description of the generator. |
| `suggestion_deduping_config` | String |  | Optional. Configuration for suggestion deduping. This is only applicable to AI Coach feature. |
| `free_form_context` | String |  | Input of free from generator to LLM. |
| `inference_parameter` | String |  | Optional. Inference parameters for this generator. |
| `tools` | Vec<String> |  | Optional. Resource names of the tools that the generator can choose from. Format: `projects//locations//tools/`. |
| `parent` | String | ✅ | Required. The project/location to create generator for. Format: `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `summarization_context` | String | Input of Summarization feature. |
| `trigger_event` | String | Optional. The trigger event of the generator. It defines when the generator is triggered in a conversation. |
| `update_time` | String | Output only. Update time of this generator. |
| `create_time` | String | Output only. Creation time of this generator. |
| `name` | String | Output only. Identifier. The resource name of the generator. Format: `projects//locations//generators/` |
| `published_model` | String | Optional. The published Large Language Model name. * To use the latest model version, specify the model name without version number. Example: `text-bison` * To use a stable model version, specify the version number as well. Example: `text-bison@002`. |
| `agent_coaching_context` | String | Input of Agent Coaching feature. |
| `description` | String | Optional. Human readable description of the generator. |
| `suggestion_deduping_config` | String | Optional. Configuration for suggestion deduping. This is only applicable to AI Coach feature. |
| `free_form_context` | String | Input of free from generator to LLM. |
| `inference_parameter` | String | Optional. Inference parameters for this generator. |
| `tools` | Vec<String> | Optional. Resource names of the tools that the generator can choose from. Format: `projects//locations//tools/`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create generator
generator = provider.dialogflow_api.Generator {
    parent = "value"  # Required. The project/location to create generator for. Format: `projects//locations/`
}

# Access generator outputs
generator_id = generator.id
generator_summarization_context = generator.summarization_context
generator_trigger_event = generator.trigger_event
generator_update_time = generator.update_time
generator_create_time = generator.create_time
generator_name = generator.name
generator_published_model = generator.published_model
generator_agent_coaching_context = generator.agent_coaching_context
generator_description = generator.description
generator_suggestion_deduping_config = generator.suggestion_deduping_config
generator_free_form_context = generator.free_form_context
generator_inference_parameter = generator.inference_parameter
generator_tools = generator.tools
```

---


### Conversation_profile

Creates a conversation profile in the specified project. ConversationProfile.CreateTime and ConversationProfile.UpdateTime aren't populated in the response. You can retrieve them via GetConversationProfile API.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `human_agent_handoff_config` | String |  | Configuration for connecting to a live agent. Currently, this feature is not general available, please contact Google to get access. |
| `name` | String |  | The unique identifier of this conversation profile. Format: `projects//locations//conversationProfiles/`. |
| `time_zone` | String |  | The time zone of this conversational profile from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. Defaults to America/New_York. |
| `language_code` | String |  | Language code for the conversation profile. If not specified, the language is en-US. Language at ConversationProfile should be set for all non en-us languages. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US". |
| `update_time` | String |  | Output only. Update time of the conversation profile. |
| `create_time` | String |  | Output only. Create time of the conversation profile. |
| `logging_config` | String |  | Configuration for logging conversation lifecycle events. |
| `stt_config` | String |  | Settings for speech transcription. |
| `new_message_event_notification_config` | String |  | Configuration for publishing new message events. Event will be sent in format of ConversationEvent |
| `human_agent_assistant_config` | String |  | Configuration for agent assistance to use with this profile. |
| `new_recognition_result_notification_config` | String |  | Optional. Configuration for publishing transcription intermediate results. Event will be sent in format of ConversationEvent. If configured, the following information will be populated as ConversationEvent Pub/Sub message attributes: - "participant_id" - "participant_role" - "message_id" |
| `notification_config` | String |  | Configuration for publishing conversation lifecycle events. |
| `security_settings` | String |  | Name of the CX SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`. |
| `automated_agent_config` | String |  | Configuration for an automated agent to use with this profile. |
| `tts_config` | String |  | Configuration for Text-to-Speech synthesization. Used by Phone Gateway to specify synthesization options. If agent defines synthesization options as well, agent settings overrides the option here. |
| `display_name` | String |  | Required. Human readable name for this profile. Max length 1024 bytes. |
| `parent` | String | ✅ | Required. The project to create a conversation profile for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `human_agent_handoff_config` | String | Configuration for connecting to a live agent. Currently, this feature is not general available, please contact Google to get access. |
| `name` | String | The unique identifier of this conversation profile. Format: `projects//locations//conversationProfiles/`. |
| `time_zone` | String | The time zone of this conversational profile from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. Defaults to America/New_York. |
| `language_code` | String | Language code for the conversation profile. If not specified, the language is en-US. Language at ConversationProfile should be set for all non en-us languages. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US". |
| `update_time` | String | Output only. Update time of the conversation profile. |
| `create_time` | String | Output only. Create time of the conversation profile. |
| `logging_config` | String | Configuration for logging conversation lifecycle events. |
| `stt_config` | String | Settings for speech transcription. |
| `new_message_event_notification_config` | String | Configuration for publishing new message events. Event will be sent in format of ConversationEvent |
| `human_agent_assistant_config` | String | Configuration for agent assistance to use with this profile. |
| `new_recognition_result_notification_config` | String | Optional. Configuration for publishing transcription intermediate results. Event will be sent in format of ConversationEvent. If configured, the following information will be populated as ConversationEvent Pub/Sub message attributes: - "participant_id" - "participant_role" - "message_id" |
| `notification_config` | String | Configuration for publishing conversation lifecycle events. |
| `security_settings` | String | Name of the CX SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`. |
| `automated_agent_config` | String | Configuration for an automated agent to use with this profile. |
| `tts_config` | String | Configuration for Text-to-Speech synthesization. Used by Phone Gateway to specify synthesization options. If agent defines synthesization options as well, agent settings overrides the option here. |
| `display_name` | String | Required. Human readable name for this profile. Max length 1024 bytes. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversation_profile
conversation_profile = provider.dialogflow_api.Conversation_profile {
    parent = "value"  # Required. The project to create a conversation profile for. Format: `projects//locations/`.
}

# Access conversation_profile outputs
conversation_profile_id = conversation_profile.id
conversation_profile_human_agent_handoff_config = conversation_profile.human_agent_handoff_config
conversation_profile_name = conversation_profile.name
conversation_profile_time_zone = conversation_profile.time_zone
conversation_profile_language_code = conversation_profile.language_code
conversation_profile_update_time = conversation_profile.update_time
conversation_profile_create_time = conversation_profile.create_time
conversation_profile_logging_config = conversation_profile.logging_config
conversation_profile_stt_config = conversation_profile.stt_config
conversation_profile_new_message_event_notification_config = conversation_profile.new_message_event_notification_config
conversation_profile_human_agent_assistant_config = conversation_profile.human_agent_assistant_config
conversation_profile_new_recognition_result_notification_config = conversation_profile.new_recognition_result_notification_config
conversation_profile_notification_config = conversation_profile.notification_config
conversation_profile_security_settings = conversation_profile.security_settings
conversation_profile_automated_agent_config = conversation_profile.automated_agent_config
conversation_profile_tts_config = conversation_profile.tts_config
conversation_profile_display_name = conversation_profile.display_name
```

---


### Project

Creates/updates the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `avatar_uri` | String |  | Optional. The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `display_name` | String |  | Required. The name of this agent. |
| `time_zone` | String |  | Required. The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `classification_threshold` | f64 |  | Optional. To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used. |
| `default_language_code` | String |  | Required. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the `Update` method. |
| `parent` | String |  | Required. The project of this agent. Format: `projects/` or `projects//locations/` |
| `tier` | String |  | Optional. The agent tier. If not specified, TIER_STANDARD is assumed. |
| `api_version` | String |  | Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version. |
| `description` | String |  | Optional. The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `enable_logging` | bool |  | Optional. Determines whether this agent should log conversation queries. |
| `match_mode` | String |  | Optional. Determines how intents are detected from user queries. |
| `supported_language_codes` | Vec<String> |  | Optional. The list of all languages supported by this agent (except for the `default_language_code`). |
| `parent` | String | ✅ | Required. The project of this agent. Format: `projects/` or `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `avatar_uri` | String | Optional. The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `display_name` | String | Required. The name of this agent. |
| `time_zone` | String | Required. The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `classification_threshold` | f64 | Optional. To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used. |
| `default_language_code` | String | Required. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the `Update` method. |
| `parent` | String | Required. The project of this agent. Format: `projects/` or `projects//locations/` |
| `tier` | String | Optional. The agent tier. If not specified, TIER_STANDARD is assumed. |
| `api_version` | String | Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version. |
| `description` | String | Optional. The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `enable_logging` | bool | Optional. Determines whether this agent should log conversation queries. |
| `match_mode` | String | Optional. Determines how intents are detected from user queries. |
| `supported_language_codes` | Vec<String> | Optional. The list of all languages supported by this agent (except for the `default_language_code`). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.dialogflow_api.Project {
    parent = "value"  # Required. The project of this agent. Format: `projects/` or `projects//locations/`
}

# Access project outputs
project_id = project.id
project_avatar_uri = project.avatar_uri
project_display_name = project.display_name
project_time_zone = project.time_zone
project_classification_threshold = project.classification_threshold
project_default_language_code = project.default_language_code
project_parent = project.parent
project_tier = project.tier
project_api_version = project.api_version
project_description = project.description
project_enable_logging = project.enable_logging
project_match_mode = project.match_mode
project_supported_language_codes = project.supported_language_codes
```

---


### Session

Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause contexts and session entity types to be updated, which in turn might affect results of future queries. If you might use [Agent Assist](https://cloud.google.com/dialogflow/docs/#aa) or other CCAI products now or in the future, consider using AnalyzeContent instead of `DetectIntent`. `AnalyzeContent` has additional functionality for Agent Assist and other CCAI products. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `output_audio_config_mask` | String |  | Mask for output_audio_config indicating which settings in this request-level config should override speech synthesizer settings defined at agent-level. If unspecified or empty, output_audio_config replaces the agent-level config in its entirety. |
| `query_params` | String |  | The parameters of this query. |
| `output_audio_config` | String |  | Instructs the speech synthesizer how to generate the output audio. If this field is not set and agent-level speech synthesizer is not configured, no output audio is generated. |
| `query_input` | String |  | Required. The input specification. It can be set to: 1. an audio config which instructs the speech recognizer how to process the speech audio, 2. a conversational query in the form of text, or 3. an event that specifies which intent to trigger. |
| `input_audio` | String |  | The natural language speech audio to be processed. This field should be populated iff `query_input` is set to an input audio config. A single request can contain up to 1 minute of speech audio data. |
| `session` | String | ✅ | Required. The name of the session this query is sent to. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment (`Environment ID` might be referred to as environment name at some places). If `User ID` is not specified, we are using "-". It's up to the API caller to choose an appropriate `Session ID` and `User Id`. They can be a random number or some type of user and session identifiers (preferably hashed). The length of the `Session ID` and `User ID` must not exceed 36 characters. For more information, see the [API interactions guide](https://cloud.google.com/dialogflow/docs/api-overview). Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions). |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create session
session = provider.dialogflow_api.Session {
    session = "value"  # Required. The name of the session this query is sent to. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment (`Environment ID` might be referred to as environment name at some places). If `User ID` is not specified, we are using "-". It's up to the API caller to choose an appropriate `Session ID` and `User Id`. They can be a random number or some type of user and session identifiers (preferably hashed). The length of the `Session ID` and `User ID` must not exceed 36 characters. For more information, see the [API interactions guide](https://cloud.google.com/dialogflow/docs/api-overview). Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).
}

```

---


### Version

Creates an agent version. The new version points to the agent instance in the "default" environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The creation time of this version. This field is read-only, i.e., it cannot be set by create and update methods. |
| `name` | String |  | Output only. The unique identifier of this agent version. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/` |
| `status` | String |  | Output only. The status of this version. This field is read-only and cannot be set by create and update methods. |
| `description` | String |  | Optional. The developer-provided description of this version. |
| `version_number` | i64 |  | Output only. The sequential number of this version. This field is read-only which means it cannot be set by create and update methods. |
| `parent` | String | ✅ | Required. The agent to create a version for. Supported formats: - `projects//agent` - `projects//locations//agent` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The creation time of this version. This field is read-only, i.e., it cannot be set by create and update methods. |
| `name` | String | Output only. The unique identifier of this agent version. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/` |
| `status` | String | Output only. The status of this version. This field is read-only and cannot be set by create and update methods. |
| `description` | String | Optional. The developer-provided description of this version. |
| `version_number` | i64 | Output only. The sequential number of this version. This field is read-only which means it cannot be set by create and update methods. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.dialogflow_api.Version {
    parent = "value"  # Required. The agent to create a version for. Supported formats: - `projects//agent` - `projects//locations//agent`
}

# Access version outputs
version_id = version.id
version_create_time = version.create_time
version_name = version.name
version_status = version.status
version_description = version.description
version_version_number = version.version_number
```

---


### Phone_number

Cancels the deletion request for a `PhoneNumber`. This method may only be called on a `PhoneNumber` in the DELETE_REQUESTED state.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The unique identifier of the `PhoneNumber` to delete. Format: `projects//phoneNumbers/`. Format: `projects//locations//phoneNumbers/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |
| `phone_numbers` | Vec<String> | The list of `PhoneNumber` resources. There is a maximum number of items returned based on the page_size field in the request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create phone_number
phone_number = provider.dialogflow_api.Phone_number {
    name = "value"  # Required. The unique identifier of the `PhoneNumber` to delete. Format: `projects//phoneNumbers/`. Format: `projects//locations//phoneNumbers/`.
}

# Access phone_number outputs
phone_number_id = phone_number.id
phone_number_next_page_token = phone_number.next_page_token
phone_number_phone_numbers = phone_number.phone_numbers
```

---


### Participant

Creates a new participant in a conversation.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `documents_metadata_filters` | HashMap<String, String> |  | Optional. Key-value filters on the metadata of documents returned by article suggestion. If specified, article suggestion only returns suggested documents that match all filters in their Document.metadata. Multiple values for a metadata key should be concatenated by comma. For example, filters to match all documents that have 'US' or 'CA' in their market metadata values and 'agent' in their user metadata values will be ``` documents_metadata_filters { key: "market" value: "US,CA" } documents_metadata_filters { key: "user" value: "agent" } ``` |
| `name` | String |  | Optional. The unique identifier of this participant. Format: `projects//locations//conversations//participants/`. |
| `role` | String |  | Immutable. The role this participant plays in the conversation. This field must be set during participant creation and is then immutable. |
| `obfuscated_external_user_id` | String |  | Optional. Obfuscated user id that should be associated with the created participant. You can specify a user id as follows: 1. If you set this field in CreateParticipantRequest or UpdateParticipantRequest, Dialogflow adds the obfuscated user id with the participant. 2. If you set this field in AnalyzeContent or StreamingAnalyzeContent, Dialogflow will update Participant.obfuscated_external_user_id. Dialogflow uses this user id for billing and measurement. If a user with the same obfuscated_external_user_id is created in a later conversation, Dialogflow will know it's the same user. Dialogflow also uses this user id for Agent Assist suggestion personalization. For example, Dialogflow can use it to provide personalized smart reply suggestions for this user. Note: * Please never pass raw user ids to Dialogflow. Always obfuscate your user id first. * Dialogflow only accepts a UTF-8 encoded string, e.g., a hex digest of a hash function like SHA-512. * The length of the user id must be <= 256 characters. |
| `parent` | String | ✅ | Required. Resource identifier of the conversation adding the participant. Format: `projects//locations//conversations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `documents_metadata_filters` | HashMap<String, String> | Optional. Key-value filters on the metadata of documents returned by article suggestion. If specified, article suggestion only returns suggested documents that match all filters in their Document.metadata. Multiple values for a metadata key should be concatenated by comma. For example, filters to match all documents that have 'US' or 'CA' in their market metadata values and 'agent' in their user metadata values will be ``` documents_metadata_filters { key: "market" value: "US,CA" } documents_metadata_filters { key: "user" value: "agent" } ``` |
| `name` | String | Optional. The unique identifier of this participant. Format: `projects//locations//conversations//participants/`. |
| `role` | String | Immutable. The role this participant plays in the conversation. This field must be set during participant creation and is then immutable. |
| `obfuscated_external_user_id` | String | Optional. Obfuscated user id that should be associated with the created participant. You can specify a user id as follows: 1. If you set this field in CreateParticipantRequest or UpdateParticipantRequest, Dialogflow adds the obfuscated user id with the participant. 2. If you set this field in AnalyzeContent or StreamingAnalyzeContent, Dialogflow will update Participant.obfuscated_external_user_id. Dialogflow uses this user id for billing and measurement. If a user with the same obfuscated_external_user_id is created in a later conversation, Dialogflow will know it's the same user. Dialogflow also uses this user id for Agent Assist suggestion personalization. For example, Dialogflow can use it to provide personalized smart reply suggestions for this user. Note: * Please never pass raw user ids to Dialogflow. Always obfuscate your user id first. * Dialogflow only accepts a UTF-8 encoded string, e.g., a hex digest of a hash function like SHA-512. * The length of the user id must be <= 256 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create participant
participant = provider.dialogflow_api.Participant {
    parent = "value"  # Required. Resource identifier of the conversation adding the participant. Format: `projects//locations//conversations/`.
}

# Access participant outputs
participant_id = participant.id
participant_documents_metadata_filters = participant.documents_metadata_filters
participant_name = participant.name
participant_role = participant.role
participant_obfuscated_external_user_id = participant.obfuscated_external_user_id
```

---


### Encryption_spec

Initializes a location-level encryption key specification. An error will be thrown if the location has resources already created before the initialization. Once the encryption specification is initialized at a location, it is immutable and all newly created resources under the location will be encrypted with the existing specification.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_spec` | String |  | Required. The encryption spec used for CMEK encryption. It is required that the kms key is in the same region as the endpoint. The same key will be used for all provisioned resources, if encryption is available. If the kms_key_name is left empty, no encryption will be enforced. |
| `name` | String | ✅ | Immutable. The resource name of the encryption key specification resource. Format: projects/{project}/locations/{location}/encryptionSpec |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create encryption_spec
encryption_spec = provider.dialogflow_api.Encryption_spec {
    name = "value"  # Immutable. The resource name of the encryption key specification resource. Format: projects/{project}/locations/{location}/encryptionSpec
}

```

---


### Sip_trunk

Creates a SipTrunk for a specified location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The unique identifier of the SIP trunk. Format: `projects//locations//sipTrunks/`. |
| `display_name` | String |  | Optional. Human readable alias for this trunk. |
| `expected_hostname` | Vec<String> |  | Required. The expected hostnames in the peer certificate from partner that is used for TLS authentication. |
| `connections` | Vec<String> |  | Output only. Connections of the SIP trunk. |
| `parent` | String | ✅ | Required. The location to create a SIP trunk for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The unique identifier of the SIP trunk. Format: `projects//locations//sipTrunks/`. |
| `display_name` | String | Optional. Human readable alias for this trunk. |
| `expected_hostname` | Vec<String> | Required. The expected hostnames in the peer certificate from partner that is used for TLS authentication. |
| `connections` | Vec<String> | Output only. Connections of the SIP trunk. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sip_trunk
sip_trunk = provider.dialogflow_api.Sip_trunk {
    parent = "value"  # Required. The location to create a SIP trunk for. Format: `projects//locations/`.
}

# Access sip_trunk outputs
sip_trunk_id = sip_trunk.id
sip_trunk_name = sip_trunk.name
sip_trunk_display_name = sip_trunk.display_name
sip_trunk_expected_hostname = sip_trunk.expected_hostname
sip_trunk_connections = sip_trunk.connections
```

---


### Agent

Exports the specified agent to a ZIP file. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: ExportAgentResponse

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `agent_uri` | String |  | Optional. The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to export the agent to. The format of this URI must be `gs:///`. If left unspecified, the serialized agent is returned inline. Dialogflow performs a write operation for the Cloud Storage object on the caller's behalf, so your request authentication must have write permissions for the object. For more information, see [Dialogflow access control](https://cloud.google.com/dialogflow/cx/docs/concept/access-control#storage). |
| `parent` | String | ✅ | Required. The project that the agent to export is associated with. Format: `projects/` or `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enabled` | bool | Whether fulfillment is enabled. |
| `features` | Vec<String> | The field defines whether the fulfillment is enabled for certain features. |
| `name` | String | Required. The unique identifier of the fulfillment. Supported formats: - `projects//agent/fulfillment` - `projects//locations//agent/fulfillment` This field is not used for Fulfillment in an Environment. |
| `generic_web_service` | String | Configuration for a generic web service. |
| `display_name` | String | The human-readable name of the fulfillment, unique within the agent. This field is not used for Fulfillment in an Environment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create agent
agent = provider.dialogflow_api.Agent {
    parent = "value"  # Required. The project that the agent to export is associated with. Format: `projects/` or `projects//locations/`.
}

# Access agent outputs
agent_id = agent.id
agent_enabled = agent.enabled
agent_features = agent.features
agent_name = agent.name
agent_generic_web_service = agent.generic_web_service
agent_display_name = agent.display_name
```

---


### Intent

Creates an intent in the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `priority` | i64 |  | Optional. The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests. |
| `reset_contexts` | bool |  | Optional. Indicates whether to delete all contexts in the current session when this intent is matched. |
| `webhook_state` | String |  | Optional. Indicates whether webhooks are enabled for the intent. |
| `events` | Vec<String> |  | Optional. The collection of event names that trigger the intent. If the collection of input contexts is not empty, all of the contexts must be present in the active user session for an event to trigger this intent. Event names are limited to 150 characters. |
| `end_interaction` | bool |  | Optional. Indicates that this intent ends an interaction. Some integrations (e.g., Actions on Google or Dialogflow phone gateway) use this information to close interaction with an end user. Default is false. |
| `default_response_platforms` | Vec<String> |  | Optional. The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform). |
| `display_name` | String |  | Required. The name of this intent. |
| `parent_followup_intent_name` | String |  | Optional. The unique identifier of the parent intent in the chain of followup intents. You can set this field when creating an intent, for example with CreateIntent or BatchUpdateIntents, in order to make this intent a followup intent. It identifies the parent followup intent. Format: `projects//agent/intents/`. |
| `followup_intent_info` | Vec<String> |  | Output only. Information about all followup intents that have this intent as a direct or indirect parent. We populate this field only in the output. |
| `root_followup_intent_name` | String |  | Output only. The unique identifier of the root intent in the chain of followup intents. It identifies the correct followup intents chain for this intent. Format: `projects//agent/intents/`. |
| `live_agent_handoff` | bool |  | Optional. Indicates that a live agent should be brought in to handle the interaction with the user. In most cases, when you set this flag to true, you would also want to set end_interaction to true as well. Default is false. |
| `is_fallback` | bool |  | Optional. Indicates whether this is a fallback intent. |
| `action` | String |  | Optional. The name of the action associated with the intent. Note: The action name must not contain whitespaces. |
| `ml_disabled` | bool |  | Optional. Indicates whether Machine Learning is disabled for the intent. Note: If `ml_disabled` setting is set to true, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off. |
| `training_phrases` | Vec<String> |  | Optional. The collection of examples that the agent is trained on. |
| `ml_enabled` | bool |  | Optional. Indicates whether Machine Learning is enabled for the intent. Note: If `ml_enabled` setting is set to false, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off. DEPRECATED! Please use `ml_disabled` field instead. NOTE: If both `ml_enabled` and `ml_disabled` are either not set or false, then the default value is determined as follows: - Before April 15th, 2018 the default is: ml_enabled = false / ml_disabled = true. - After April 15th, 2018 the default is: ml_enabled = true / ml_disabled = false. |
| `output_contexts` | Vec<String> |  | Optional. The collection of contexts that are activated when the intent is matched. Context messages in this collection should not set the parameters field. Setting the `lifespan_count` to 0 will reset the context when the intent is matched. Format: `projects//agent/sessions/-/contexts/`. |
| `input_context_names` | Vec<String> |  | Optional. The list of context names required for this intent to be triggered. Formats: - `projects//agent/sessions/-/contexts/` - `projects//locations//agent/sessions/-/contexts/` |
| `messages` | Vec<String> |  | Optional. The collection of rich messages corresponding to the `Response` field in the Dialogflow console. |
| `name` | String |  | Optional. The unique identifier of this intent. Required for Intents.UpdateIntent and Intents.BatchUpdateIntents methods. Supported formats: - `projects//agent/intents/` - `projects//locations//agent/intents/` |
| `parameters` | Vec<String> |  | Optional. The collection of parameters associated with the intent. |
| `parent` | String | ✅ | Required. The agent to create a intent for. Supported formats: - `projects//agent` - `projects//locations//agent` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `priority` | i64 | Optional. The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests. |
| `reset_contexts` | bool | Optional. Indicates whether to delete all contexts in the current session when this intent is matched. |
| `webhook_state` | String | Optional. Indicates whether webhooks are enabled for the intent. |
| `events` | Vec<String> | Optional. The collection of event names that trigger the intent. If the collection of input contexts is not empty, all of the contexts must be present in the active user session for an event to trigger this intent. Event names are limited to 150 characters. |
| `end_interaction` | bool | Optional. Indicates that this intent ends an interaction. Some integrations (e.g., Actions on Google or Dialogflow phone gateway) use this information to close interaction with an end user. Default is false. |
| `default_response_platforms` | Vec<String> | Optional. The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform). |
| `display_name` | String | Required. The name of this intent. |
| `parent_followup_intent_name` | String | Optional. The unique identifier of the parent intent in the chain of followup intents. You can set this field when creating an intent, for example with CreateIntent or BatchUpdateIntents, in order to make this intent a followup intent. It identifies the parent followup intent. Format: `projects//agent/intents/`. |
| `followup_intent_info` | Vec<String> | Output only. Information about all followup intents that have this intent as a direct or indirect parent. We populate this field only in the output. |
| `root_followup_intent_name` | String | Output only. The unique identifier of the root intent in the chain of followup intents. It identifies the correct followup intents chain for this intent. Format: `projects//agent/intents/`. |
| `live_agent_handoff` | bool | Optional. Indicates that a live agent should be brought in to handle the interaction with the user. In most cases, when you set this flag to true, you would also want to set end_interaction to true as well. Default is false. |
| `is_fallback` | bool | Optional. Indicates whether this is a fallback intent. |
| `action` | String | Optional. The name of the action associated with the intent. Note: The action name must not contain whitespaces. |
| `ml_disabled` | bool | Optional. Indicates whether Machine Learning is disabled for the intent. Note: If `ml_disabled` setting is set to true, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off. |
| `training_phrases` | Vec<String> | Optional. The collection of examples that the agent is trained on. |
| `ml_enabled` | bool | Optional. Indicates whether Machine Learning is enabled for the intent. Note: If `ml_enabled` setting is set to false, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off. DEPRECATED! Please use `ml_disabled` field instead. NOTE: If both `ml_enabled` and `ml_disabled` are either not set or false, then the default value is determined as follows: - Before April 15th, 2018 the default is: ml_enabled = false / ml_disabled = true. - After April 15th, 2018 the default is: ml_enabled = true / ml_disabled = false. |
| `output_contexts` | Vec<String> | Optional. The collection of contexts that are activated when the intent is matched. Context messages in this collection should not set the parameters field. Setting the `lifespan_count` to 0 will reset the context when the intent is matched. Format: `projects//agent/sessions/-/contexts/`. |
| `input_context_names` | Vec<String> | Optional. The list of context names required for this intent to be triggered. Formats: - `projects//agent/sessions/-/contexts/` - `projects//locations//agent/sessions/-/contexts/` |
| `messages` | Vec<String> | Optional. The collection of rich messages corresponding to the `Response` field in the Dialogflow console. |
| `name` | String | Optional. The unique identifier of this intent. Required for Intents.UpdateIntent and Intents.BatchUpdateIntents methods. Supported formats: - `projects//agent/intents/` - `projects//locations//agent/intents/` |
| `parameters` | Vec<String> | Optional. The collection of parameters associated with the intent. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create intent
intent = provider.dialogflow_api.Intent {
    parent = "value"  # Required. The agent to create a intent for. Supported formats: - `projects//agent` - `projects//locations//agent`
}

# Access intent outputs
intent_id = intent.id
intent_priority = intent.priority
intent_reset_contexts = intent.reset_contexts
intent_webhook_state = intent.webhook_state
intent_events = intent.events
intent_end_interaction = intent.end_interaction
intent_default_response_platforms = intent.default_response_platforms
intent_display_name = intent.display_name
intent_parent_followup_intent_name = intent.parent_followup_intent_name
intent_followup_intent_info = intent.followup_intent_info
intent_root_followup_intent_name = intent.root_followup_intent_name
intent_live_agent_handoff = intent.live_agent_handoff
intent_is_fallback = intent.is_fallback
intent_action = intent.action
intent_ml_disabled = intent.ml_disabled
intent_training_phrases = intent.training_phrases
intent_ml_enabled = intent.ml_enabled
intent_output_contexts = intent.output_contexts
intent_input_context_names = intent.input_context_names
intent_messages = intent.messages
intent_name = intent.name
intent_parameters = intent.parameters
```

---


### Entitie

Creates multiple new entities in the specified entity type. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entities` | Vec<String> |  | Required. The entities to create. |
| `language_code` | String |  | Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity). |
| `parent` | String | ✅ | Required. The name of the entity type to create entities in. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entitie
entitie = provider.dialogflow_api.Entitie {
    parent = "value"  # Required. The name of the entity type to create entities in. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/`
}

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
operation = provider.dialogflow_api.Operation {
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


### Tool

Creates a tool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connector_spec` | String |  | Integration connectors tool specification. |
| `display_name` | String |  | Optional. A human readable short name of the tool, to be shown on the UI. |
| `function_spec` | String |  | Client side executed function specification. |
| `description` | String |  | Optional. A human readable description of the tool. |
| `extension_spec` | String |  | Vertex extension tool specification. |
| `satisfies_pzi` | bool |  | Output only. A read only boolean field reflecting Zone Isolation status of the tool. If the field is absent, it means the status is unknown. |
| `create_time` | String |  | Output only. Creation time of this tool. |
| `tool_key` | String |  | Required. A human readable short name of the tool, which should be unique within the project. It should only contain letters, numbers, and underscores, and it will be used by LLM to identify the tool. |
| `name` | String |  | Output only. Identifier. The resource name of the tool. Format: `projects//locations//tools/`. |
| `satisfies_pzs` | bool |  | Output only. A read only boolean field reflecting Zone Separation status of the tool. If the field is absent, it means the status is unknown. |
| `update_time` | String |  | Output only. Update time of this tool. |
| `action_confirmation_requirement` | HashMap<String, String> |  | Optional. Confirmation requirement for the actions. Each key is an action name in the action_schemas. If an action's confirmation requirement is unspecified (either the key is not present, or its value is CONFIRMATION_REQUIREMENT_UNSPECIFIED), the requirement is inferred from the action's method_type - confirmation is not required if and only if method_type is GET. |
| `open_api_spec` | String |  | OpenAPI tool. |
| `parent` | String | ✅ | Required. The project/location to create tool for. Format: `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connector_spec` | String | Integration connectors tool specification. |
| `display_name` | String | Optional. A human readable short name of the tool, to be shown on the UI. |
| `function_spec` | String | Client side executed function specification. |
| `description` | String | Optional. A human readable description of the tool. |
| `extension_spec` | String | Vertex extension tool specification. |
| `satisfies_pzi` | bool | Output only. A read only boolean field reflecting Zone Isolation status of the tool. If the field is absent, it means the status is unknown. |
| `create_time` | String | Output only. Creation time of this tool. |
| `tool_key` | String | Required. A human readable short name of the tool, which should be unique within the project. It should only contain letters, numbers, and underscores, and it will be used by LLM to identify the tool. |
| `name` | String | Output only. Identifier. The resource name of the tool. Format: `projects//locations//tools/`. |
| `satisfies_pzs` | bool | Output only. A read only boolean field reflecting Zone Separation status of the tool. If the field is absent, it means the status is unknown. |
| `update_time` | String | Output only. Update time of this tool. |
| `action_confirmation_requirement` | HashMap<String, String> | Optional. Confirmation requirement for the actions. Each key is an action name in the action_schemas. If an action's confirmation requirement is unspecified (either the key is not present, or its value is CONFIRMATION_REQUIREMENT_UNSPECIFIED), the requirement is inferred from the action's method_type - confirmation is not required if and only if method_type is GET. |
| `open_api_spec` | String | OpenAPI tool. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tool
tool = provider.dialogflow_api.Tool {
    parent = "value"  # Required. The project/location to create tool for. Format: `projects//locations/`
}

# Access tool outputs
tool_id = tool.id
tool_connector_spec = tool.connector_spec
tool_display_name = tool.display_name
tool_function_spec = tool.function_spec
tool_description = tool.description
tool_extension_spec = tool.extension_spec
tool_satisfies_pzi = tool.satisfies_pzi
tool_create_time = tool.create_time
tool_tool_key = tool.tool_key
tool_name = tool.name
tool_satisfies_pzs = tool.satisfies_pzs
tool_update_time = tool.update_time
tool_action_confirmation_requirement = tool.action_confirmation_requirement
tool_open_api_spec = tool.open_api_spec
```

---


### Conversation

Creates a new conversation. Conversations are auto-completed after 24 hours. Conversation Lifecycle: There are two stages during a conversation: Automated Agent Stage and Assist Stage. For Automated Agent Stage, there will be a dialogflow agent responding to user queries. For Assist Stage, there's no dialogflow agent responding to user queries. But we will provide suggestions which are generated from conversation. If Conversation.conversation_profile is configured for a dialogflow agent, conversation will start from `Automated Agent Stage`, otherwise, it will start from `Assist Stage`. And during `Automated Agent Stage`, once an Intent with Intent.live_agent_handoff is triggered, conversation will transfer to Assist Stage.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `start_time` | String |  | Output only. The time the conversation was started. |
| `ingested_context_references` | HashMap<String, String> |  | Output only. The context reference updates provided by external systems. |
| `name` | String |  | Output only. Identifier. The unique identifier of this conversation. Format: `projects//locations//conversations/`. |
| `telephony_connection_info` | String |  | Output only. The telephony connection information. |
| `lifecycle_state` | String |  | Output only. The current state of the Conversation. |
| `conversation_stage` | String |  | Optional. The stage of a conversation. It indicates whether the virtual agent or a human agent is handling the conversation. If the conversation is created with the conversation profile that has Dialogflow config set, defaults to ConversationStage.VIRTUAL_AGENT_STAGE; Otherwise, defaults to ConversationStage.HUMAN_ASSIST_STAGE. If the conversation is created with the conversation profile that has Dialogflow config set but explicitly sets conversation_stage to ConversationStage.HUMAN_ASSIST_STAGE, it skips ConversationStage.VIRTUAL_AGENT_STAGE stage and directly goes to ConversationStage.HUMAN_ASSIST_STAGE. |
| `conversation_profile` | String |  | Required. The Conversation Profile to be used to configure this Conversation. This field cannot be updated. Format: `projects//locations//conversationProfiles/`. |
| `end_time` | String |  | Output only. The time the conversation was finished. |
| `phone_number` | String |  | Output only. Required if the conversation is to be connected over telephony. |
| `parent` | String | ✅ | Required. Resource identifier of the project creating the conversation. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | Output only. The time the conversation was started. |
| `ingested_context_references` | HashMap<String, String> | Output only. The context reference updates provided by external systems. |
| `name` | String | Output only. Identifier. The unique identifier of this conversation. Format: `projects//locations//conversations/`. |
| `telephony_connection_info` | String | Output only. The telephony connection information. |
| `lifecycle_state` | String | Output only. The current state of the Conversation. |
| `conversation_stage` | String | Optional. The stage of a conversation. It indicates whether the virtual agent or a human agent is handling the conversation. If the conversation is created with the conversation profile that has Dialogflow config set, defaults to ConversationStage.VIRTUAL_AGENT_STAGE; Otherwise, defaults to ConversationStage.HUMAN_ASSIST_STAGE. If the conversation is created with the conversation profile that has Dialogflow config set but explicitly sets conversation_stage to ConversationStage.HUMAN_ASSIST_STAGE, it skips ConversationStage.VIRTUAL_AGENT_STAGE stage and directly goes to ConversationStage.HUMAN_ASSIST_STAGE. |
| `conversation_profile` | String | Required. The Conversation Profile to be used to configure this Conversation. This field cannot be updated. Format: `projects//locations//conversationProfiles/`. |
| `end_time` | String | Output only. The time the conversation was finished. |
| `phone_number` | String | Output only. Required if the conversation is to be connected over telephony. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversation
conversation = provider.dialogflow_api.Conversation {
    parent = "value"  # Required. Resource identifier of the project creating the conversation. Format: `projects//locations/`.
}

# Access conversation outputs
conversation_id = conversation.id
conversation_start_time = conversation.start_time
conversation_ingested_context_references = conversation.ingested_context_references
conversation_name = conversation.name
conversation_telephony_connection_info = conversation.telephony_connection_info
conversation_lifecycle_state = conversation.lifecycle_state
conversation_conversation_stage = conversation.conversation_stage
conversation_conversation_profile = conversation.conversation_profile
conversation_end_time = conversation.end_time
conversation_phone_number = conversation.phone_number
```

---


### Message

Batch ingests messages to conversation. Customers can use this RPC to ingest historical messages to conversation.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Required. A maximum of 300 messages can be created in a batch. CreateMessageRequest.message.send_time is required. All created messages will have identical Message.create_time. |
| `parent` | String | ✅ | Required. Resource identifier of the conversation to create message. Format: `projects//locations//conversations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `messages` | Vec<String> | Required. The list of messages. There will be a maximum number of items returned based on the page_size field in the request. `messages` is sorted by `create_time` in descending order. |
| `next_page_token` | String | Optional. Token to retrieve the next page of results, or empty if there are no more results in the list. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create message
message = provider.dialogflow_api.Message {
    parent = "value"  # Required. Resource identifier of the conversation to create message. Format: `projects//locations//conversations/`.
}

# Access message outputs
message_id = message.id
message_messages = message.messages
message_next_page_token = message.next_page_token
```

---


### Stateless_suggestion

Generates and returns a suggestion for a conversation that does not have a resource created for it.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `generator_name` | String |  | The resource name of the existing created generator. Format: `projects//locations//generators/` |
| `generator` | String |  | Uncreated generator. It should be a complete generator that includes all information about the generator. |
| `security_settings` | String |  | Optional. Name of the CX SecuritySettings which is used to redact generated response. If this field is empty, try to fetch v2 security_settings, which is a project level setting. If this field is empty and no v2 security_settings set up in this project, no redaction will be done. Format: `projects//locations//securitySettings/`. |
| `trigger_events` | Vec<String> |  | Optional. A list of trigger events. Generator will be triggered only if it's trigger event is included here. |
| `context_references` | HashMap<String, String> |  | Optional. A section of ingested context information. The key is the name of the context reference and the value contains the contents of the context reference. The key is used to incorporate ingested context references to enhance the generator. |
| `conversation_context` | String |  | Optional. Context of the conversation, including transcripts. |
| `parent` | String | ✅ | Required. The parent resource to charge for the Suggestion's generation. Format: `projects//locations/`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create stateless_suggestion
stateless_suggestion = provider.dialogflow_api.Stateless_suggestion {
    parent = "value"  # Required. The parent resource to charge for the Suggestion's generation. Format: `projects//locations/`.
}

```

---


### Location

Creates/updates the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `avatar_uri` | String |  | Optional. The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `display_name` | String |  | Required. The name of this agent. |
| `time_zone` | String |  | Required. The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `classification_threshold` | f64 |  | Optional. To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used. |
| `default_language_code` | String |  | Required. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the `Update` method. |
| `parent` | String |  | Required. The project of this agent. Format: `projects/` or `projects//locations/` |
| `tier` | String |  | Optional. The agent tier. If not specified, TIER_STANDARD is assumed. |
| `api_version` | String |  | Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version. |
| `description` | String |  | Optional. The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `enable_logging` | bool |  | Optional. Determines whether this agent should log conversation queries. |
| `match_mode` | String |  | Optional. Determines how intents are detected from user queries. |
| `supported_language_codes` | Vec<String> |  | Optional. The list of all languages supported by this agent (except for the `default_language_code`). |
| `parent` | String | ✅ | Required. The project of this agent. Format: `projects/` or `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.dialogflow_api.Location {
    parent = "value"  # Required. The project of this agent. Format: `projects/` or `projects//locations/`
}

# Access location outputs
location_id = location.id
location_labels = location.labels
location_name = location.name
location_display_name = location.display_name
location_metadata = location.metadata
location_location_id = location.location_id
```

---


### Knowledge_base

Creates a knowledge base. Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The display name of the knowledge base. The name must be 1024 bytes or less; otherwise, the creation request fails. |
| `language_code` | String |  | Language which represents the KnowledgeBase. When the KnowledgeBase is created/updated, this is populated for all non en-us languages. If not populated, the default language en-us applies. |
| `name` | String |  | The knowledge base resource name. The name must be empty when creating a knowledge base. Format: `projects//locations//knowledgeBases/`. |
| `parent` | String | ✅ | Required. The project to create a knowledge base for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The display name of the knowledge base. The name must be 1024 bytes or less; otherwise, the creation request fails. |
| `language_code` | String | Language which represents the KnowledgeBase. When the KnowledgeBase is created/updated, this is populated for all non en-us languages. If not populated, the default language en-us applies. |
| `name` | String | The knowledge base resource name. The name must be empty when creating a knowledge base. Format: `projects//locations//knowledgeBases/`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create knowledge_base
knowledge_base = provider.dialogflow_api.Knowledge_base {
    parent = "value"  # Required. The project to create a knowledge base for. Format: `projects//locations/`.
}

# Access knowledge_base outputs
knowledge_base_id = knowledge_base.id
knowledge_base_display_name = knowledge_base.display_name
knowledge_base_language_code = knowledge_base.language_code
knowledge_base_name = knowledge_base.name
```

---


### Environment

Creates an agent environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. The developer-provided description for this environment. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `name` | String |  | Output only. The unique identifier of this agent environment. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/` |
| `state` | String |  | Output only. The state of this environment. This field is read-only, i.e., it cannot be set by create and update methods. |
| `text_to_speech_settings` | String |  | Optional. Text to speech settings for this environment. |
| `update_time` | String |  | Output only. The last update time of this environment. This field is read-only, i.e., it cannot be set by create and update methods. |
| `agent_version` | String |  | Optional. The agent version loaded into this environment. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/` |
| `fulfillment` | String |  | Optional. The fulfillment settings to use for this environment. |
| `parent` | String | ✅ | Required. The agent to create an environment for. Supported formats: - `projects//agent` - `projects//locations//agent` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. The developer-provided description for this environment. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `name` | String | Output only. The unique identifier of this agent environment. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/` |
| `state` | String | Output only. The state of this environment. This field is read-only, i.e., it cannot be set by create and update methods. |
| `text_to_speech_settings` | String | Optional. Text to speech settings for this environment. |
| `update_time` | String | Output only. The last update time of this environment. This field is read-only, i.e., it cannot be set by create and update methods. |
| `agent_version` | String | Optional. The agent version loaded into this environment. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/` |
| `fulfillment` | String | Optional. The fulfillment settings to use for this environment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create environment
environment = provider.dialogflow_api.Environment {
    parent = "value"  # Required. The agent to create an environment for. Supported formats: - `projects//agent` - `projects//locations//agent`
}

# Access environment outputs
environment_id = environment.id
environment_description = environment.description
environment_name = environment.name
environment_state = environment.state
environment_text_to_speech_settings = environment.text_to_speech_settings
environment_update_time = environment.update_time
environment_agent_version = environment.agent_version
environment_fulfillment = environment.fulfillment
```

---


### Context

Creates a context. If the specified context already exists, overrides the context.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameters` | HashMap<String, String> |  | Optional. The collection of parameters associated with this context. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: * MapKey type: string * MapKey value: parameter name * MapValue type: If parameter's entity type is a composite entity then use map, otherwise, depending on the parameter value type, it could be one of string, number, boolean, null, list or map. * MapValue value: If parameter's entity type is a composite entity then use map from composite entity property names to property values, otherwise, use parameter value. |
| `lifespan_count` | i64 |  | Optional. The number of conversational query requests after which the context expires. The default is `0`. If set to `0`, the context expires immediately. Contexts expire automatically after 20 minutes if there are no matching queries. |
| `name` | String |  | Required. The unique identifier of the context. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, The `Context ID` is always converted to lowercase, may only contain characters in `a-zA-Z0-9_-%` and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size` |
| `parent` | String | ✅ | Required. The session to create a context for. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameters` | HashMap<String, String> | Optional. The collection of parameters associated with this context. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: * MapKey type: string * MapKey value: parameter name * MapValue type: If parameter's entity type is a composite entity then use map, otherwise, depending on the parameter value type, it could be one of string, number, boolean, null, list or map. * MapValue value: If parameter's entity type is a composite entity then use map from composite entity property names to property values, otherwise, use parameter value. |
| `lifespan_count` | i64 | Optional. The number of conversational query requests after which the context expires. The default is `0`. If set to `0`, the context expires immediately. Contexts expire automatically after 20 minutes if there are no matching queries. |
| `name` | String | Required. The unique identifier of the context. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, The `Context ID` is always converted to lowercase, may only contain characters in `a-zA-Z0-9_-%` and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create context
context = provider.dialogflow_api.Context {
    parent = "value"  # Required. The session to create a context for. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
}

# Access context outputs
context_id = context.id
context_parameters = context.parameters
context_lifespan_count = context.lifespan_count
context_name = context.name
```

---


### Location

Creates/updates the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `api_version` | String |  | Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version. |
| `avatar_uri` | String |  | Optional. The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `default_language_code` | String |  | Required. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the `Update` method. |
| `time_zone` | String |  | Required. The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `enable_logging` | bool |  | Optional. Determines whether this agent should log conversation queries. |
| `classification_threshold` | f64 |  | Optional. To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used. |
| `display_name` | String |  | Required. The name of this agent. |
| `tier` | String |  | Optional. The agent tier. If not specified, TIER_STANDARD is assumed. |
| `description` | String |  | Optional. The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `match_mode` | String |  | Optional. Determines how intents are detected from user queries. |
| `parent` | String |  | Required. The project of this agent. Format: `projects/`. |
| `supported_language_codes` | Vec<String> |  | Optional. The list of all languages supported by this agent (except for the `default_language_code`). |
| `parent` | String | ✅ | Required. The project of this agent. Format: `projects/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.dialogflow_api.Location {
    parent = "value"  # Required. The project of this agent. Format: `projects/`.
}

# Access location outputs
location_id = location.id
location_labels = location.labels
location_name = location.name
location_metadata = location.metadata
location_location_id = location.location_id
location_display_name = location.display_name
```

---


### Conversation_dataset

Creates a new conversation dataset. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: CreateConversationDatasetOperationMetadata - `response`: ConversationDataset

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Creation time of this dataset. |
| `conversation_count` | String |  | Output only. The number of conversations this conversation dataset contains. |
| `name` | String |  | Output only. ConversationDataset resource name. Format: `projects//locations//conversationDatasets/` |
| `satisfies_pzi` | bool |  | Output only. A read only boolean field reflecting Zone Isolation status of the dataset. |
| `satisfies_pzs` | bool |  | Output only. A read only boolean field reflecting Zone Separation status of the dataset. |
| `conversation_info` | String |  | Output only. Metadata set during conversation data import. |
| `display_name` | String |  | Required. The display name of the dataset. Maximum of 64 bytes. |
| `input_config` | String |  | Output only. Input configurations set during conversation data import. |
| `description` | String |  | Optional. The description of the dataset. Maximum of 10000 bytes. |
| `parent` | String | ✅ | Required. The project to create conversation dataset for. Format: `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Creation time of this dataset. |
| `conversation_count` | String | Output only. The number of conversations this conversation dataset contains. |
| `name` | String | Output only. ConversationDataset resource name. Format: `projects//locations//conversationDatasets/` |
| `satisfies_pzi` | bool | Output only. A read only boolean field reflecting Zone Isolation status of the dataset. |
| `satisfies_pzs` | bool | Output only. A read only boolean field reflecting Zone Separation status of the dataset. |
| `conversation_info` | String | Output only. Metadata set during conversation data import. |
| `display_name` | String | Required. The display name of the dataset. Maximum of 64 bytes. |
| `input_config` | String | Output only. Input configurations set during conversation data import. |
| `description` | String | Optional. The description of the dataset. Maximum of 10000 bytes. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversation_dataset
conversation_dataset = provider.dialogflow_api.Conversation_dataset {
    parent = "value"  # Required. The project to create conversation dataset for. Format: `projects//locations/`
}

# Access conversation_dataset outputs
conversation_dataset_id = conversation_dataset.id
conversation_dataset_create_time = conversation_dataset.create_time
conversation_dataset_conversation_count = conversation_dataset.conversation_count
conversation_dataset_name = conversation_dataset.name
conversation_dataset_satisfies_pzi = conversation_dataset.satisfies_pzi
conversation_dataset_satisfies_pzs = conversation_dataset.satisfies_pzs
conversation_dataset_conversation_info = conversation_dataset.conversation_info
conversation_dataset_display_name = conversation_dataset.display_name
conversation_dataset_input_config = conversation_dataset.input_config
conversation_dataset_description = conversation_dataset.description
```

---


### Version

Creates an agent version. The new version points to the agent instance in the "default" environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The creation time of this version. This field is read-only, i.e., it cannot be set by create and update methods. |
| `name` | String |  | Output only. The unique identifier of this agent version. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/` |
| `status` | String |  | Output only. The status of this version. This field is read-only and cannot be set by create and update methods. |
| `description` | String |  | Optional. The developer-provided description of this version. |
| `version_number` | i64 |  | Output only. The sequential number of this version. This field is read-only which means it cannot be set by create and update methods. |
| `parent` | String | ✅ | Required. The agent to create a version for. Supported formats: - `projects//agent` - `projects//locations//agent` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The creation time of this version. This field is read-only, i.e., it cannot be set by create and update methods. |
| `name` | String | Output only. The unique identifier of this agent version. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/` |
| `status` | String | Output only. The status of this version. This field is read-only and cannot be set by create and update methods. |
| `description` | String | Optional. The developer-provided description of this version. |
| `version_number` | i64 | Output only. The sequential number of this version. This field is read-only which means it cannot be set by create and update methods. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.dialogflow_api.Version {
    parent = "value"  # Required. The agent to create a version for. Supported formats: - `projects//agent` - `projects//locations//agent`
}

# Access version outputs
version_id = version.id
version_create_time = version.create_time
version_name = version.name
version_status = version.status
version_description = version.description
version_version_number = version.version_number
```

---


### Conversation_model

Creates a model. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: CreateConversationModelOperationMetadata - `response`: ConversationModel

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `smart_reply_model_metadata` | String |  | Metadata for smart reply models. |
| `article_suggestion_model_metadata` | String |  | Metadata for article suggestion models. |
| `display_name` | String |  | Required. The display name of the model. At most 64 bytes long. |
| `create_time` | String |  | Output only. Creation time of this model. |
| `state` | String |  | Output only. State of the model. A model can only serve prediction requests after it gets deployed. |
| `name` | String |  | ConversationModel resource name. Format: `projects//conversationModels/` |
| `datasets` | Vec<String> |  | Required. Datasets used to create model. |
| `satisfies_pzi` | bool |  | Output only. A read only boolean field reflecting Zone Isolation status of the model. |
| `satisfies_pzs` | bool |  | Output only. A read only boolean field reflecting Zone Separation status of the model. |
| `language_code` | String |  | Language code for the conversation model. If not specified, the language is en-US. Language at ConversationModel should be set for all non en-us languages. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US". |
| `parent` | String | ✅ | The project to create conversation model for. Format: `projects/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `smart_reply_model_metadata` | String | Metadata for smart reply models. |
| `article_suggestion_model_metadata` | String | Metadata for article suggestion models. |
| `display_name` | String | Required. The display name of the model. At most 64 bytes long. |
| `create_time` | String | Output only. Creation time of this model. |
| `state` | String | Output only. State of the model. A model can only serve prediction requests after it gets deployed. |
| `name` | String | ConversationModel resource name. Format: `projects//conversationModels/` |
| `datasets` | Vec<String> | Required. Datasets used to create model. |
| `satisfies_pzi` | bool | Output only. A read only boolean field reflecting Zone Isolation status of the model. |
| `satisfies_pzs` | bool | Output only. A read only boolean field reflecting Zone Separation status of the model. |
| `language_code` | String | Language code for the conversation model. If not specified, the language is en-US. Language at ConversationModel should be set for all non en-us languages. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversation_model
conversation_model = provider.dialogflow_api.Conversation_model {
    parent = "value"  # The project to create conversation model for. Format: `projects/`
}

# Access conversation_model outputs
conversation_model_id = conversation_model.id
conversation_model_smart_reply_model_metadata = conversation_model.smart_reply_model_metadata
conversation_model_article_suggestion_model_metadata = conversation_model.article_suggestion_model_metadata
conversation_model_display_name = conversation_model.display_name
conversation_model_create_time = conversation_model.create_time
conversation_model_state = conversation_model.state
conversation_model_name = conversation_model.name
conversation_model_datasets = conversation_model.datasets
conversation_model_satisfies_pzi = conversation_model.satisfies_pzi
conversation_model_satisfies_pzs = conversation_model.satisfies_pzs
conversation_model_language_code = conversation_model.language_code
```

---


### Sip_trunk

Creates a SipTrunk for a specified location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connections` | Vec<String> |  | Output only. Connections of the SIP trunk. |
| `name` | String |  | Identifier. The unique identifier of the SIP trunk. Format: `projects//locations//sipTrunks/`. |
| `expected_hostname` | Vec<String> |  | Required. The expected hostnames in the peer certificate from partner that is used for TLS authentication. |
| `display_name` | String |  | Optional. Human readable alias for this trunk. |
| `parent` | String | ✅ | Required. The location to create a SIP trunk for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connections` | Vec<String> | Output only. Connections of the SIP trunk. |
| `name` | String | Identifier. The unique identifier of the SIP trunk. Format: `projects//locations//sipTrunks/`. |
| `expected_hostname` | Vec<String> | Required. The expected hostnames in the peer certificate from partner that is used for TLS authentication. |
| `display_name` | String | Optional. Human readable alias for this trunk. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sip_trunk
sip_trunk = provider.dialogflow_api.Sip_trunk {
    parent = "value"  # Required. The location to create a SIP trunk for. Format: `projects//locations/`.
}

# Access sip_trunk outputs
sip_trunk_id = sip_trunk.id
sip_trunk_connections = sip_trunk.connections
sip_trunk_name = sip_trunk.name
sip_trunk_expected_hostname = sip_trunk.expected_hostname
sip_trunk_display_name = sip_trunk.display_name
```

---


### Knowledge_base

Creates a knowledge base.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The knowledge base resource name. The name must be empty when creating a knowledge base. Format: `projects//locations//knowledgeBases/`. |
| `display_name` | String |  | Required. The display name of the knowledge base. The name must be 1024 bytes or less; otherwise, the creation request fails. |
| `language_code` | String |  | Language which represents the KnowledgeBase. When the KnowledgeBase is created/updated, expect this to be present for non en-us languages. When unspecified, the default language code en-us applies. |
| `parent` | String | ✅ | Required. The project to create a knowledge base for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The knowledge base resource name. The name must be empty when creating a knowledge base. Format: `projects//locations//knowledgeBases/`. |
| `display_name` | String | Required. The display name of the knowledge base. The name must be 1024 bytes or less; otherwise, the creation request fails. |
| `language_code` | String | Language which represents the KnowledgeBase. When the KnowledgeBase is created/updated, expect this to be present for non en-us languages. When unspecified, the default language code en-us applies. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create knowledge_base
knowledge_base = provider.dialogflow_api.Knowledge_base {
    parent = "value"  # Required. The project to create a knowledge base for. Format: `projects//locations/`.
}

# Access knowledge_base outputs
knowledge_base_id = knowledge_base.id
knowledge_base_name = knowledge_base.name
knowledge_base_display_name = knowledge_base.display_name
knowledge_base_language_code = knowledge_base.language_code
```

---


### Evaluation

Creates evaluation of a conversation model.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `conversation_model_evaluation` | String |  | Required. The conversation model evaluation to be created. |
| `parent` | String | ✅ | Required. The conversation model resource name. Format: `projects//locations//conversationModels/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name of the evaluation. Format: `projects//conversationModels//evaluations/` |
| `smart_reply_metrics` | String | Output only. Only available when model is for smart reply. |
| `display_name` | String | Optional. The display name of the model evaluation. At most 64 bytes long. |
| `create_time` | String | Output only. Creation time of this model. |
| `evaluation_config` | String | Optional. The configuration of the evaluation task. |
| `raw_human_eval_template_csv` | String | Output only. Human eval template in csv format. It takes real-world conversations provided through input dataset, generates example suggestions for customer to verify quality of the model. For Smart Reply, the generated csv file contains columns of Context, (Suggestions,Q1,Q2)*3, Actual reply. Context contains at most 10 latest messages in the conversation prior to the current suggestion. Q1: "Would you send it as the next message of agent?" Evaluated based on whether the suggest is appropriate to be sent by agent in current context. Q2: "Does the suggestion move the conversation closer to resolution?" Evaluated based on whether the suggestion provide solutions, or answers customer's question or collect information from customer to resolve the customer's issue. Actual reply column contains the actual agent reply sent in the context. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create evaluation
evaluation = provider.dialogflow_api.Evaluation {
    parent = "value"  # Required. The conversation model resource name. Format: `projects//locations//conversationModels/`
}

# Access evaluation outputs
evaluation_id = evaluation.id
evaluation_name = evaluation.name
evaluation_smart_reply_metrics = evaluation.smart_reply_metrics
evaluation_display_name = evaluation.display_name
evaluation_create_time = evaluation.create_time
evaluation_evaluation_config = evaluation.evaluation_config
evaluation_raw_human_eval_template_csv = evaluation.raw_human_eval_template_csv
```

---


### Document

Creates a new document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The display name of the document. The name must be 1024 bytes or less; otherwise, the creation request fails. |
| `state` | String |  | Output only. The current state of the document. |
| `mime_type` | String |  | Required. The MIME type of this document. |
| `enable_auto_reload` | bool |  | Optional. If true, we try to automatically reload the document every day (at a time picked by the system). If false or unspecified, we don't try to automatically reload the document. Currently you can only enable automatic reload for documents sourced from a public url, see `source` field for the source types. Reload status can be tracked in `latest_reload_status`. If a reload fails, we will keep the document unchanged. If a reload fails with internal errors, the system will try to reload the document on the next day. If a reload fails with non-retriable errors (e.g. PERMISSION_DENIED), the system will not try to reload the document anymore. You need to manually reload the document successfully by calling `ReloadDocument` and clear the errors. |
| `metadata` | HashMap<String, String> |  | Optional. Metadata for the document. The metadata supports arbitrary key-value pairs. Suggested use cases include storing a document's title, an external URL distinct from the document's content_uri, etc. The max size of a `key` or a `value` of the metadata is 1024 bytes. |
| `latest_reload_status` | String |  | Output only. The time and status of the latest reload. This reload may have been triggered automatically or manually and may not have succeeded. |
| `name` | String |  | Optional. The document resource name. The name must be empty when creating a document. Format: `projects//locations//knowledgeBases//documents/`. |
| `content_uri` | String |  | The URI where the file content is located. For documents stored in Google Cloud Storage, these URIs must have the form `gs:///`. NOTE: External URLs must correspond to public webpages, i.e., they must be indexed by Google Search. In particular, URLs for showing documents in Google Cloud Storage (i.e. the URL in your browser) are not supported. Instead use the `gs://` format URI described above. |
| `knowledge_types` | Vec<String> |  | Required. The knowledge type of document content. |
| `raw_content` | String |  | The raw content of the document. This field is only permitted for EXTRACTIVE_QA and FAQ knowledge types. |
| `parent` | String | ✅ | Required. The knowledge base to create a document for. Format: `projects//locations//knowledgeBases/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The display name of the document. The name must be 1024 bytes or less; otherwise, the creation request fails. |
| `state` | String | Output only. The current state of the document. |
| `mime_type` | String | Required. The MIME type of this document. |
| `enable_auto_reload` | bool | Optional. If true, we try to automatically reload the document every day (at a time picked by the system). If false or unspecified, we don't try to automatically reload the document. Currently you can only enable automatic reload for documents sourced from a public url, see `source` field for the source types. Reload status can be tracked in `latest_reload_status`. If a reload fails, we will keep the document unchanged. If a reload fails with internal errors, the system will try to reload the document on the next day. If a reload fails with non-retriable errors (e.g. PERMISSION_DENIED), the system will not try to reload the document anymore. You need to manually reload the document successfully by calling `ReloadDocument` and clear the errors. |
| `metadata` | HashMap<String, String> | Optional. Metadata for the document. The metadata supports arbitrary key-value pairs. Suggested use cases include storing a document's title, an external URL distinct from the document's content_uri, etc. The max size of a `key` or a `value` of the metadata is 1024 bytes. |
| `latest_reload_status` | String | Output only. The time and status of the latest reload. This reload may have been triggered automatically or manually and may not have succeeded. |
| `name` | String | Optional. The document resource name. The name must be empty when creating a document. Format: `projects//locations//knowledgeBases//documents/`. |
| `content_uri` | String | The URI where the file content is located. For documents stored in Google Cloud Storage, these URIs must have the form `gs:///`. NOTE: External URLs must correspond to public webpages, i.e., they must be indexed by Google Search. In particular, URLs for showing documents in Google Cloud Storage (i.e. the URL in your browser) are not supported. Instead use the `gs://` format URI described above. |
| `knowledge_types` | Vec<String> | Required. The knowledge type of document content. |
| `raw_content` | String | The raw content of the document. This field is only permitted for EXTRACTIVE_QA and FAQ knowledge types. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create document
document = provider.dialogflow_api.Document {
    parent = "value"  # Required. The knowledge base to create a document for. Format: `projects//locations//knowledgeBases/`.
}

# Access document outputs
document_id = document.id
document_display_name = document.display_name
document_state = document.state
document_mime_type = document.mime_type
document_enable_auto_reload = document.enable_auto_reload
document_metadata = document.metadata
document_latest_reload_status = document.latest_reload_status
document_name = document.name
document_content_uri = document.content_uri
document_knowledge_types = document.knowledge_types
document_raw_content = document.raw_content
```

---


### Suggestion

Get answers for the given query based on knowledge documents.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query_source` | String |  | Optional. The source of the query in the request. |
| `exact_search` | bool |  | Optional. Whether to search the query exactly without query rewrite. |
| `conversation` | String |  | Optional. The conversation (between human agent and end user) where the search request is triggered. Format: `projects//locations//conversations/`. |
| `latest_message` | String |  | Optional. The name of the latest conversation message when the request is triggered. Format: `projects//locations//conversations//messages/`. |
| `search_config` | String |  | Optional. Configuration specific to search queries with data stores. |
| `conversation_profile` | String |  | Required. The conversation profile used to configure the search. Format: `projects//locations//conversationProfiles/`. |
| `end_user_metadata` | HashMap<String, String> |  | Optional. Information about the end-user to improve the relevance and accuracy of generative answers. This will be interpreted and used by a language model, so, for good results, the data should be self-descriptive, and in a simple structure. Example: ```json { "subscription plan": "Business Premium Plus", "devices owned": [ {"model": "Google Pixel 7"}, {"model": "Google Pixel Tablet"} ] } ``` |
| `parent` | String |  | Required. The parent resource contains the conversation profile Format: 'projects/' or `projects//locations/`. |
| `query` | String |  | Required. The natural language text query for knowledge search. |
| `session_id` | String |  | Required. The ID of the search session. The session_id can be combined with Dialogflow V3 Agent ID retrieved from conversation profile or on its own to identify a search session. The search history of the same session will impact the search result. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length must not exceed 36 characters. |
| `conversation` | String | ✅ | Optional. The conversation (between human agent and end user) where the search request is triggered. Format: `projects//locations//conversations/`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create suggestion
suggestion = provider.dialogflow_api.Suggestion {
    conversation = "value"  # Optional. The conversation (between human agent and end user) where the search request is triggered. Format: `projects//locations//conversations/`.
}

```

---


### Entity_type

Creates a session entity type. If the specified session entity type already exists, overrides the session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entity_override_mode` | String |  | Required. Indicates whether the additional data should override or supplement the custom entity type definition. |
| `entities` | Vec<String> |  | Required. The collection of entities associated with this session entity type. |
| `name` | String |  | Required. The unique identifier of this session entity type. Format: `projects//agent/sessions//entityTypes/`, or `projects//agent/environments//users//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. `` must be the display name of an existing entity type in the same agent that will be overridden or supplemented. |
| `parent` | String | ✅ | Required. The session to create a session entity type for. Format: `projects//agent/sessions/` or `projects//agent/environments//users// sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entity_override_mode` | String | Required. Indicates whether the additional data should override or supplement the custom entity type definition. |
| `entities` | Vec<String> | Required. The collection of entities associated with this session entity type. |
| `name` | String | Required. The unique identifier of this session entity type. Format: `projects//agent/sessions//entityTypes/`, or `projects//agent/environments//users//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. `` must be the display name of an existing entity type in the same agent that will be overridden or supplemented. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entity_type
entity_type = provider.dialogflow_api.Entity_type {
    parent = "value"  # Required. The session to create a session entity type for. Format: `projects//agent/sessions/` or `projects//agent/environments//users// sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
}

# Access entity_type outputs
entity_type_id = entity_type.id
entity_type_entity_override_mode = entity_type.entity_override_mode
entity_type_entities = entity_type.entities
entity_type_name = entity_type.name
```

---


### Participant

Creates a new participant in a conversation.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role` | String |  | Immutable. The role this participant plays in the conversation. This field must be set during participant creation and is then immutable. |
| `obfuscated_external_user_id` | String |  | Optional. Obfuscated user id that should be associated with the created participant. You can specify a user id as follows: 1. If you set this field in CreateParticipantRequest or UpdateParticipantRequest, Dialogflow adds the obfuscated user id with the participant. 2. If you set this field in AnalyzeContent or StreamingAnalyzeContent, Dialogflow will update Participant.obfuscated_external_user_id. Dialogflow returns an error if you try to add a user id for a non-END_USER participant. Dialogflow uses this user id for billing and measurement purposes. For example, Dialogflow determines whether a user in one conversation returned in a later conversation. Note: * Please never pass raw user ids to Dialogflow. Always obfuscate your user id first. * Dialogflow only accepts a UTF-8 encoded string, e.g., a hex digest of a hash function like SHA-512. * The length of the user id must be <= 256 characters. |
| `name` | String |  | Optional. The unique identifier of this participant. Format: `projects//locations//conversations//participants/`. |
| `documents_metadata_filters` | HashMap<String, String> |  | Optional. Key-value filters on the metadata of documents returned by article suggestion. If specified, article suggestion only returns suggested documents that match all filters in their Document.metadata. Multiple values for a metadata key should be concatenated by comma. For example, filters to match all documents that have 'US' or 'CA' in their market metadata values and 'agent' in their user metadata values will be ``` documents_metadata_filters { key: "market" value: "US,CA" } documents_metadata_filters { key: "user" value: "agent" } ``` |
| `sip_recording_media_label` | String |  | Optional. Label applied to streams representing this participant in SIPREC XML metadata and SDP. This is used to assign transcriptions from that media stream to this participant. This field can be updated. |
| `parent` | String | ✅ | Required. Resource identifier of the conversation adding the participant. Format: `projects//locations//conversations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `role` | String | Immutable. The role this participant plays in the conversation. This field must be set during participant creation and is then immutable. |
| `obfuscated_external_user_id` | String | Optional. Obfuscated user id that should be associated with the created participant. You can specify a user id as follows: 1. If you set this field in CreateParticipantRequest or UpdateParticipantRequest, Dialogflow adds the obfuscated user id with the participant. 2. If you set this field in AnalyzeContent or StreamingAnalyzeContent, Dialogflow will update Participant.obfuscated_external_user_id. Dialogflow returns an error if you try to add a user id for a non-END_USER participant. Dialogflow uses this user id for billing and measurement purposes. For example, Dialogflow determines whether a user in one conversation returned in a later conversation. Note: * Please never pass raw user ids to Dialogflow. Always obfuscate your user id first. * Dialogflow only accepts a UTF-8 encoded string, e.g., a hex digest of a hash function like SHA-512. * The length of the user id must be <= 256 characters. |
| `name` | String | Optional. The unique identifier of this participant. Format: `projects//locations//conversations//participants/`. |
| `documents_metadata_filters` | HashMap<String, String> | Optional. Key-value filters on the metadata of documents returned by article suggestion. If specified, article suggestion only returns suggested documents that match all filters in their Document.metadata. Multiple values for a metadata key should be concatenated by comma. For example, filters to match all documents that have 'US' or 'CA' in their market metadata values and 'agent' in their user metadata values will be ``` documents_metadata_filters { key: "market" value: "US,CA" } documents_metadata_filters { key: "user" value: "agent" } ``` |
| `sip_recording_media_label` | String | Optional. Label applied to streams representing this participant in SIPREC XML metadata and SDP. This is used to assign transcriptions from that media stream to this participant. This field can be updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create participant
participant = provider.dialogflow_api.Participant {
    parent = "value"  # Required. Resource identifier of the conversation adding the participant. Format: `projects//locations//conversations/`.
}

# Access participant outputs
participant_id = participant.id
participant_role = participant.role
participant_obfuscated_external_user_id = participant.obfuscated_external_user_id
participant_name = participant.name
participant_documents_metadata_filters = participant.documents_metadata_filters
participant_sip_recording_media_label = participant.sip_recording_media_label
```

---


### Stateless_suggestion

Generates and returns a suggestion for a conversation that does not have a resource created for it.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `generator` | String |  | Uncreated generator. It should be a complete generator that includes all information about the generator. |
| `context_references` | HashMap<String, String> |  | Optional. A section of ingested context information. The key is the name of the context reference and the value contains the contents of the context reference. The key is used to incorporate ingested context references to enhance the generator. |
| `generator_name` | String |  | The resource name of the existing created generator. Format: `projects//locations//generators/` |
| `security_settings` | String |  | Optional. Name of the CX SecuritySettings which is used to redact generated response. If this field is empty, try to fetch v2 security_settings, which is a project level setting. If this field is empty and no v2 security_settings set up in this project, no redaction will be done. Format: `projects//locations//securitySettings/`. |
| `trigger_events` | Vec<String> |  | Optional. A list of trigger events. Generator will be triggered only if it's trigger event is included here. |
| `conversation_context` | String |  | Optional. Context of the conversation, including transcripts. |
| `parent` | String | ✅ | Required. The parent resource to charge for the Suggestion's generation. Format: `projects//locations/`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create stateless_suggestion
stateless_suggestion = provider.dialogflow_api.Stateless_suggestion {
    parent = "value"  # Required. The parent resource to charge for the Suggestion's generation. Format: `projects//locations/`.
}

```

---


### Encryption_spec

Initializes a location-level encryption key specification. An error will be thrown if the location has resources already created before the initialization. Once the encryption specification is initialized at a location, it is immutable and all newly created resources under the location will be encrypted with the existing specification.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_spec` | String |  | Required. The encryption spec used for CMEK encryption. It is required that the kms key is in the same region as the endpoint. The same key will be used for all provisioned resources, if encryption is available. If the kms_key_name is left empty, no encryption will be enforced. |
| `name` | String | ✅ | Immutable. The resource name of the encryption key specification resource. Format: projects/{project}/locations/{location}/encryptionSpec |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create encryption_spec
encryption_spec = provider.dialogflow_api.Encryption_spec {
    name = "value"  # Immutable. The resource name of the encryption key specification resource. Format: projects/{project}/locations/{location}/encryptionSpec
}

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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.dialogflow_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
```

---


### Context

Creates a context. If the specified context already exists, overrides the context.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameters` | HashMap<String, String> |  | Optional. The collection of parameters associated with this context. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: * MapKey type: string * MapKey value: parameter name * MapValue type: If parameter's entity type is a composite entity then use map, otherwise, depending on the parameter value type, it could be one of string, number, boolean, null, list or map. * MapValue value: If parameter's entity type is a composite entity then use map from composite entity property names to property values, otherwise, use parameter value. |
| `name` | String |  | Required. The unique identifier of the context. Format: `projects//agent/sessions//contexts/`, or `projects//agent/environments//users//sessions//contexts/`. The `Context ID` is always converted to lowercase, may only contain characters in `a-zA-Z0-9_-%` and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size` |
| `lifespan_count` | i64 |  | Optional. The number of conversational query requests after which the context expires. The default is `0`. If set to `0`, the context expires immediately. Contexts expire automatically after 20 minutes if there are no matching queries. |
| `parent` | String | ✅ | Required. The session to create a context for. Format: `projects//agent/sessions/` or `projects//agent/environments//users//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameters` | HashMap<String, String> | Optional. The collection of parameters associated with this context. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: * MapKey type: string * MapKey value: parameter name * MapValue type: If parameter's entity type is a composite entity then use map, otherwise, depending on the parameter value type, it could be one of string, number, boolean, null, list or map. * MapValue value: If parameter's entity type is a composite entity then use map from composite entity property names to property values, otherwise, use parameter value. |
| `name` | String | Required. The unique identifier of the context. Format: `projects//agent/sessions//contexts/`, or `projects//agent/environments//users//sessions//contexts/`. The `Context ID` is always converted to lowercase, may only contain characters in `a-zA-Z0-9_-%` and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size` |
| `lifespan_count` | i64 | Optional. The number of conversational query requests after which the context expires. The default is `0`. If set to `0`, the context expires immediately. Contexts expire automatically after 20 minutes if there are no matching queries. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create context
context = provider.dialogflow_api.Context {
    parent = "value"  # Required. The session to create a context for. Format: `projects//agent/sessions/` or `projects//agent/environments//users//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
}

# Access context outputs
context_id = context.id
context_parameters = context.parameters
context_name = context.name
context_lifespan_count = context.lifespan_count
```

---


### Conversation_profile

Creates a conversation profile in the specified project. ConversationProfile.create_time and ConversationProfile.update_time aren't populated in the response. You can retrieve them via GetConversationProfile API.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Create time of the conversation profile. |
| `display_name` | String |  | Required. Human readable name for this profile. Max length 1024 bytes. |
| `human_agent_handoff_config` | String |  | Configuration for connecting to a live agent. Currently, this feature is not general available, please contact Google to get access. |
| `human_agent_assistant_config` | String |  | Configuration for agent assistance to use with this profile. |
| `security_settings` | String |  | Name of the CX SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`. |
| `update_time` | String |  | Output only. Update time of the conversation profile. |
| `new_recognition_result_notification_config` | String |  | Optional. Configuration for publishing transcription intermediate results. Event will be sent in format of ConversationEvent. If configured, the following information will be populated as ConversationEvent Pub/Sub message attributes: - "participant_id" - "participant_role" - "message_id" |
| `logging_config` | String |  | Configuration for logging conversation lifecycle events. |
| `tts_config` | String |  | Configuration for Text-to-Speech synthesization. Used by Phone Gateway to specify synthesization options. If agent defines synthesization options as well, agent settings overrides the option here. |
| `new_message_event_notification_config` | String |  | Configuration for publishing new message events. Event will be sent in format of ConversationEvent |
| `automated_agent_config` | String |  | Configuration for an automated agent to use with this profile. |
| `stt_config` | String |  | Settings for speech transcription. |
| `name` | String |  | The unique identifier of this conversation profile. Format: `projects//locations//conversationProfiles/`. |
| `notification_config` | String |  | Configuration for publishing conversation lifecycle events. |
| `language_code` | String |  | Language code for the conversation profile. If not specified, the language is en-US. Language at ConversationProfile should be set for all non en-US languages. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US". |
| `time_zone` | String |  | The time zone of this conversational profile from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. Defaults to America/New_York. |
| `parent` | String | ✅ | Required. The project to create a conversation profile for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Create time of the conversation profile. |
| `display_name` | String | Required. Human readable name for this profile. Max length 1024 bytes. |
| `human_agent_handoff_config` | String | Configuration for connecting to a live agent. Currently, this feature is not general available, please contact Google to get access. |
| `human_agent_assistant_config` | String | Configuration for agent assistance to use with this profile. |
| `security_settings` | String | Name of the CX SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`. |
| `update_time` | String | Output only. Update time of the conversation profile. |
| `new_recognition_result_notification_config` | String | Optional. Configuration for publishing transcription intermediate results. Event will be sent in format of ConversationEvent. If configured, the following information will be populated as ConversationEvent Pub/Sub message attributes: - "participant_id" - "participant_role" - "message_id" |
| `logging_config` | String | Configuration for logging conversation lifecycle events. |
| `tts_config` | String | Configuration for Text-to-Speech synthesization. Used by Phone Gateway to specify synthesization options. If agent defines synthesization options as well, agent settings overrides the option here. |
| `new_message_event_notification_config` | String | Configuration for publishing new message events. Event will be sent in format of ConversationEvent |
| `automated_agent_config` | String | Configuration for an automated agent to use with this profile. |
| `stt_config` | String | Settings for speech transcription. |
| `name` | String | The unique identifier of this conversation profile. Format: `projects//locations//conversationProfiles/`. |
| `notification_config` | String | Configuration for publishing conversation lifecycle events. |
| `language_code` | String | Language code for the conversation profile. If not specified, the language is en-US. Language at ConversationProfile should be set for all non en-US languages. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US". |
| `time_zone` | String | The time zone of this conversational profile from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. Defaults to America/New_York. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversation_profile
conversation_profile = provider.dialogflow_api.Conversation_profile {
    parent = "value"  # Required. The project to create a conversation profile for. Format: `projects//locations/`.
}

# Access conversation_profile outputs
conversation_profile_id = conversation_profile.id
conversation_profile_create_time = conversation_profile.create_time
conversation_profile_display_name = conversation_profile.display_name
conversation_profile_human_agent_handoff_config = conversation_profile.human_agent_handoff_config
conversation_profile_human_agent_assistant_config = conversation_profile.human_agent_assistant_config
conversation_profile_security_settings = conversation_profile.security_settings
conversation_profile_update_time = conversation_profile.update_time
conversation_profile_new_recognition_result_notification_config = conversation_profile.new_recognition_result_notification_config
conversation_profile_logging_config = conversation_profile.logging_config
conversation_profile_tts_config = conversation_profile.tts_config
conversation_profile_new_message_event_notification_config = conversation_profile.new_message_event_notification_config
conversation_profile_automated_agent_config = conversation_profile.automated_agent_config
conversation_profile_stt_config = conversation_profile.stt_config
conversation_profile_name = conversation_profile.name
conversation_profile_notification_config = conversation_profile.notification_config
conversation_profile_language_code = conversation_profile.language_code
conversation_profile_time_zone = conversation_profile.time_zone
```

---


### Conversation

Creates a new conversation. Conversations are auto-completed after 24 hours. Conversation Lifecycle: There are two stages during a conversation: Automated Agent Stage and Assist Stage. For Automated Agent Stage, there will be a dialogflow agent responding to user queries. For Assist Stage, there's no dialogflow agent responding to user queries. But we will provide suggestions which are generated from conversation. If Conversation.conversation_profile is configured for a dialogflow agent, conversation will start from `Automated Agent Stage`, otherwise, it will start from `Assist Stage`. And during `Automated Agent Stage`, once an Intent with Intent.live_agent_handoff is triggered, conversation will transfer to Assist Stage.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `phone_number` | String |  | Output only. It will not be empty if the conversation is to be connected over telephony. |
| `lifecycle_state` | String |  | Output only. The current state of the Conversation. |
| `ingested_context_references` | HashMap<String, String> |  | Output only. The context reference updates provided by external systems. |
| `name` | String |  | Output only. Identifier. The unique identifier of this conversation. Format: `projects//locations//conversations/`. |
| `start_time` | String |  | Output only. The time the conversation was started. |
| `conversation_profile` | String |  | Required. The Conversation Profile to be used to configure this Conversation. This field cannot be updated. Format: `projects//locations//conversationProfiles/`. |
| `conversation_stage` | String |  | Optional. The stage of a conversation. It indicates whether the virtual agent or a human agent is handling the conversation. If the conversation is created with the conversation profile that has Dialogflow config set, defaults to ConversationStage.VIRTUAL_AGENT_STAGE; Otherwise, defaults to ConversationStage.HUMAN_ASSIST_STAGE. If the conversation is created with the conversation profile that has Dialogflow config set but explicitly sets conversation_stage to ConversationStage.HUMAN_ASSIST_STAGE, it skips ConversationStage.VIRTUAL_AGENT_STAGE stage and directly goes to ConversationStage.HUMAN_ASSIST_STAGE. |
| `telephony_connection_info` | String |  | Output only. The telephony connection information. |
| `end_time` | String |  | Output only. The time the conversation was finished. |
| `parent` | String | ✅ | Required. Resource identifier of the project creating the conversation. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `phone_number` | String | Output only. It will not be empty if the conversation is to be connected over telephony. |
| `lifecycle_state` | String | Output only. The current state of the Conversation. |
| `ingested_context_references` | HashMap<String, String> | Output only. The context reference updates provided by external systems. |
| `name` | String | Output only. Identifier. The unique identifier of this conversation. Format: `projects//locations//conversations/`. |
| `start_time` | String | Output only. The time the conversation was started. |
| `conversation_profile` | String | Required. The Conversation Profile to be used to configure this Conversation. This field cannot be updated. Format: `projects//locations//conversationProfiles/`. |
| `conversation_stage` | String | Optional. The stage of a conversation. It indicates whether the virtual agent or a human agent is handling the conversation. If the conversation is created with the conversation profile that has Dialogflow config set, defaults to ConversationStage.VIRTUAL_AGENT_STAGE; Otherwise, defaults to ConversationStage.HUMAN_ASSIST_STAGE. If the conversation is created with the conversation profile that has Dialogflow config set but explicitly sets conversation_stage to ConversationStage.HUMAN_ASSIST_STAGE, it skips ConversationStage.VIRTUAL_AGENT_STAGE stage and directly goes to ConversationStage.HUMAN_ASSIST_STAGE. |
| `telephony_connection_info` | String | Output only. The telephony connection information. |
| `end_time` | String | Output only. The time the conversation was finished. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversation
conversation = provider.dialogflow_api.Conversation {
    parent = "value"  # Required. Resource identifier of the project creating the conversation. Format: `projects//locations/`.
}

# Access conversation outputs
conversation_id = conversation.id
conversation_phone_number = conversation.phone_number
conversation_lifecycle_state = conversation.lifecycle_state
conversation_ingested_context_references = conversation.ingested_context_references
conversation_name = conversation.name
conversation_start_time = conversation.start_time
conversation_conversation_profile = conversation.conversation_profile
conversation_conversation_stage = conversation.conversation_stage
conversation_telephony_connection_info = conversation.telephony_connection_info
conversation_end_time = conversation.end_time
```

---


### Message

Lists messages that belong to a given conversation. `messages` are ordered by `create_time` in descending order. To fetch updates without duplication, send request with filter `create_time_epoch_microseconds > [first item's create_time of previous request]` and empty page_token.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `messages` | Vec<String> | The list of messages. There will be a maximum number of items returned based on the page_size field in the request. `messages` is sorted by `create_time` in descending order. |
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access message outputs
message_id = message.id
message_messages = message.messages
message_next_page_token = message.next_page_token
```

---


### Environment

Creates an agent environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. The developer-provided description for this environment. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `name` | String |  | Output only. The unique identifier of this agent environment. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/` The environment ID for the default environment is `-`. |
| `fulfillment` | String |  | Optional. The fulfillment settings to use for this environment. |
| `state` | String |  | Output only. The state of this environment. This field is read-only, i.e., it cannot be set by create and update methods. |
| `text_to_speech_settings` | String |  | Optional. Text to speech settings for this environment. |
| `agent_version` | String |  | Optional. The agent version loaded into this environment. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/` |
| `update_time` | String |  | Output only. The last update time of this environment. This field is read-only, i.e., it cannot be set by create and update methods. |
| `parent` | String | ✅ | Required. The agent to create an environment for. Supported formats: - `projects//agent` - `projects//locations//agent` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. The developer-provided description for this environment. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `name` | String | Output only. The unique identifier of this agent environment. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/` The environment ID for the default environment is `-`. |
| `fulfillment` | String | Optional. The fulfillment settings to use for this environment. |
| `state` | String | Output only. The state of this environment. This field is read-only, i.e., it cannot be set by create and update methods. |
| `text_to_speech_settings` | String | Optional. Text to speech settings for this environment. |
| `agent_version` | String | Optional. The agent version loaded into this environment. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/` |
| `update_time` | String | Output only. The last update time of this environment. This field is read-only, i.e., it cannot be set by create and update methods. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create environment
environment = provider.dialogflow_api.Environment {
    parent = "value"  # Required. The agent to create an environment for. Supported formats: - `projects//agent` - `projects//locations//agent`
}

# Access environment outputs
environment_id = environment.id
environment_description = environment.description
environment_name = environment.name
environment_fulfillment = environment.fulfillment
environment_state = environment.state
environment_text_to_speech_settings = environment.text_to_speech_settings
environment_agent_version = environment.agent_version
environment_update_time = environment.update_time
```

---


### Generator

Creates a generator.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `agent_coaching_context` | String |  | Input of prebuilt Agent Coaching feature. |
| `update_time` | String |  | Output only. Update time of this generator. |
| `published_model` | String |  | Optional. The published Large Language Model name. * To use the latest model version, specify the model name without version number. Example: `text-bison` * To use a stable model version, specify the version number as well. Example: `text-bison@002`. |
| `tools` | Vec<String> |  | Optional. Resource names of the tools that the generator can choose from. Format: `projects//locations//tools/`. |
| `description` | String |  | Optional. Human readable description of the generator. |
| `name` | String |  | Output only. Identifier. The resource name of the generator. Format: `projects//locations//generators/` |
| `free_form_context` | String |  | Input of free from generator to LLM. |
| `inference_parameter` | String |  | Optional. Inference parameters for this generator. |
| `suggestion_deduping_config` | String |  | Optional. Configuration for suggestion deduping. This is only applicable to AI Coach feature. |
| `summarization_context` | String |  | Input of prebuilt Summarization feature. |
| `create_time` | String |  | Output only. Creation time of this generator. |
| `trigger_event` | String |  | Optional. The trigger event of the generator. It defines when the generator is triggered in a conversation. |
| `parent` | String | ✅ | Required. The project/location to create generator for. Format: `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `agent_coaching_context` | String | Input of prebuilt Agent Coaching feature. |
| `update_time` | String | Output only. Update time of this generator. |
| `published_model` | String | Optional. The published Large Language Model name. * To use the latest model version, specify the model name without version number. Example: `text-bison` * To use a stable model version, specify the version number as well. Example: `text-bison@002`. |
| `tools` | Vec<String> | Optional. Resource names of the tools that the generator can choose from. Format: `projects//locations//tools/`. |
| `description` | String | Optional. Human readable description of the generator. |
| `name` | String | Output only. Identifier. The resource name of the generator. Format: `projects//locations//generators/` |
| `free_form_context` | String | Input of free from generator to LLM. |
| `inference_parameter` | String | Optional. Inference parameters for this generator. |
| `suggestion_deduping_config` | String | Optional. Configuration for suggestion deduping. This is only applicable to AI Coach feature. |
| `summarization_context` | String | Input of prebuilt Summarization feature. |
| `create_time` | String | Output only. Creation time of this generator. |
| `trigger_event` | String | Optional. The trigger event of the generator. It defines when the generator is triggered in a conversation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create generator
generator = provider.dialogflow_api.Generator {
    parent = "value"  # Required. The project/location to create generator for. Format: `projects//locations/`
}

# Access generator outputs
generator_id = generator.id
generator_agent_coaching_context = generator.agent_coaching_context
generator_update_time = generator.update_time
generator_published_model = generator.published_model
generator_tools = generator.tools
generator_description = generator.description
generator_name = generator.name
generator_free_form_context = generator.free_form_context
generator_inference_parameter = generator.inference_parameter
generator_suggestion_deduping_config = generator.suggestion_deduping_config
generator_summarization_context = generator.summarization_context
generator_create_time = generator.create_time
generator_trigger_event = generator.trigger_event
```

---


### Session

Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause contexts and session entity types to be updated, which in turn might affect results of future queries. If you might use [Agent Assist](https://cloud.google.com/dialogflow/docs/#aa) or other CCAI products now or in the future, consider using AnalyzeContent instead of `DetectIntent`. `AnalyzeContent` has additional functionality for Agent Assist and other CCAI products. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `output_audio_config_mask` | String |  | Mask for output_audio_config indicating which settings in this request-level config should override speech synthesizer settings defined at agent-level. If unspecified or empty, output_audio_config replaces the agent-level config in its entirety. |
| `output_audio_config` | String |  | Instructs the speech synthesizer how to generate the output audio. If this field is not set and agent-level speech synthesizer is not configured, no output audio is generated. |
| `input_audio` | String |  | The natural language speech audio to be processed. This field should be populated iff `query_input` is set to an input audio config. A single request can contain up to 1 minute of speech audio data. |
| `query_input` | String |  | Required. The input specification. It can be set to: 1. an audio config which instructs the speech recognizer how to process the speech audio, 2. a conversational query in the form of text, or 3. an event that specifies which intent to trigger. |
| `query_params` | String |  | The parameters of this query. |
| `session` | String | ✅ | Required. The name of the session this query is sent to. Format: `projects//agent/sessions/`, or `projects//agent/environments//users//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment (`Environment ID` might be referred to as environment name at some places). If `User ID` is not specified, we are using "-". It's up to the API caller to choose an appropriate `Session ID` and `User Id`. They can be a random number or some type of user and session identifiers (preferably hashed). The length of the `Session ID` and `User ID` must not exceed 36 characters. For more information, see the [API interactions guide](https://cloud.google.com/dialogflow/docs/api-overview). Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions). |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create session
session = provider.dialogflow_api.Session {
    session = "value"  # Required. The name of the session this query is sent to. Format: `projects//agent/sessions/`, or `projects//agent/environments//users//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment (`Environment ID` might be referred to as environment name at some places). If `User ID` is not specified, we are using "-". It's up to the API caller to choose an appropriate `Session ID` and `User Id`. They can be a random number or some type of user and session identifiers (preferably hashed). The length of the `Session ID` and `User ID` must not exceed 36 characters. For more information, see the [API interactions guide](https://cloud.google.com/dialogflow/docs/api-overview). Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).
}

```

---


### Intent

Creates an intent in the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `webhook_state` | String |  | Optional. Indicates whether webhooks are enabled for the intent. |
| `output_contexts` | Vec<String> |  | Optional. The collection of contexts that are activated when the intent is matched. Context messages in this collection should not set the parameters field. Setting the `lifespan_count` to 0 will reset the context when the intent is matched. Format: `projects//agent/sessions/-/contexts/`. |
| `end_interaction` | bool |  | Optional. Indicates that this intent ends an interaction. Some integrations (e.g., Actions on Google or Dialogflow phone gateway) use this information to close interaction with an end user. Default is false. |
| `events` | Vec<String> |  | Optional. The collection of event names that trigger the intent. If the collection of input contexts is not empty, all of the contexts must be present in the active user session for an event to trigger this intent. Event names are limited to 150 characters. |
| `ml_disabled` | bool |  | Optional. Indicates whether Machine Learning is disabled for the intent. Note: If `ml_disabled` setting is set to true, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off. |
| `action` | String |  | Optional. The name of the action associated with the intent. Note: The action name must not contain whitespaces. |
| `messages` | Vec<String> |  | Optional. The collection of rich messages corresponding to the `Response` field in the Dialogflow console. |
| `root_followup_intent_name` | String |  | Output only. Read-only. The unique identifier of the root intent in the chain of followup intents. It identifies the correct followup intents chain for this intent. We populate this field only in the output. Format: `projects//agent/intents/`. |
| `training_phrases` | Vec<String> |  | Optional. The collection of examples that the agent is trained on. |
| `priority` | i64 |  | Optional. The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests. |
| `display_name` | String |  | Required. The name of this intent. |
| `parameters` | Vec<String> |  | Optional. The collection of parameters associated with the intent. |
| `parent_followup_intent_name` | String |  | Read-only after creation. The unique identifier of the parent intent in the chain of followup intents. You can set this field when creating an intent, for example with CreateIntent or BatchUpdateIntents, in order to make this intent a followup intent. It identifies the parent followup intent. Format: `projects//agent/intents/`. |
| `default_response_platforms` | Vec<String> |  | Optional. The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform). |
| `input_context_names` | Vec<String> |  | Optional. The list of context names required for this intent to be triggered. Format: `projects//agent/sessions/-/contexts/`. |
| `followup_intent_info` | Vec<String> |  | Output only. Read-only. Information about all followup intents that have this intent as a direct or indirect parent. We populate this field only in the output. |
| `is_fallback` | bool |  | Optional. Indicates whether this is a fallback intent. |
| `live_agent_handoff` | bool |  | Optional. Indicates that a live agent should be brought in to handle the interaction with the user. In most cases, when you set this flag to true, you would also want to set end_interaction to true as well. Default is false. |
| `name` | String |  | Optional. The unique identifier of this intent. Required for Intents.UpdateIntent and Intents.BatchUpdateIntents methods. Format: `projects//agent/intents/`. |
| `reset_contexts` | bool |  | Optional. Indicates whether to delete all contexts in the current session when this intent is matched. |
| `parent` | String | ✅ | Required. The agent to create a intent for. Format: `projects//agent`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `webhook_state` | String | Optional. Indicates whether webhooks are enabled for the intent. |
| `output_contexts` | Vec<String> | Optional. The collection of contexts that are activated when the intent is matched. Context messages in this collection should not set the parameters field. Setting the `lifespan_count` to 0 will reset the context when the intent is matched. Format: `projects//agent/sessions/-/contexts/`. |
| `end_interaction` | bool | Optional. Indicates that this intent ends an interaction. Some integrations (e.g., Actions on Google or Dialogflow phone gateway) use this information to close interaction with an end user. Default is false. |
| `events` | Vec<String> | Optional. The collection of event names that trigger the intent. If the collection of input contexts is not empty, all of the contexts must be present in the active user session for an event to trigger this intent. Event names are limited to 150 characters. |
| `ml_disabled` | bool | Optional. Indicates whether Machine Learning is disabled for the intent. Note: If `ml_disabled` setting is set to true, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off. |
| `action` | String | Optional. The name of the action associated with the intent. Note: The action name must not contain whitespaces. |
| `messages` | Vec<String> | Optional. The collection of rich messages corresponding to the `Response` field in the Dialogflow console. |
| `root_followup_intent_name` | String | Output only. Read-only. The unique identifier of the root intent in the chain of followup intents. It identifies the correct followup intents chain for this intent. We populate this field only in the output. Format: `projects//agent/intents/`. |
| `training_phrases` | Vec<String> | Optional. The collection of examples that the agent is trained on. |
| `priority` | i64 | Optional. The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests. |
| `display_name` | String | Required. The name of this intent. |
| `parameters` | Vec<String> | Optional. The collection of parameters associated with the intent. |
| `parent_followup_intent_name` | String | Read-only after creation. The unique identifier of the parent intent in the chain of followup intents. You can set this field when creating an intent, for example with CreateIntent or BatchUpdateIntents, in order to make this intent a followup intent. It identifies the parent followup intent. Format: `projects//agent/intents/`. |
| `default_response_platforms` | Vec<String> | Optional. The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform). |
| `input_context_names` | Vec<String> | Optional. The list of context names required for this intent to be triggered. Format: `projects//agent/sessions/-/contexts/`. |
| `followup_intent_info` | Vec<String> | Output only. Read-only. Information about all followup intents that have this intent as a direct or indirect parent. We populate this field only in the output. |
| `is_fallback` | bool | Optional. Indicates whether this is a fallback intent. |
| `live_agent_handoff` | bool | Optional. Indicates that a live agent should be brought in to handle the interaction with the user. In most cases, when you set this flag to true, you would also want to set end_interaction to true as well. Default is false. |
| `name` | String | Optional. The unique identifier of this intent. Required for Intents.UpdateIntent and Intents.BatchUpdateIntents methods. Format: `projects//agent/intents/`. |
| `reset_contexts` | bool | Optional. Indicates whether to delete all contexts in the current session when this intent is matched. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create intent
intent = provider.dialogflow_api.Intent {
    parent = "value"  # Required. The agent to create a intent for. Format: `projects//agent`.
}

# Access intent outputs
intent_id = intent.id
intent_webhook_state = intent.webhook_state
intent_output_contexts = intent.output_contexts
intent_end_interaction = intent.end_interaction
intent_events = intent.events
intent_ml_disabled = intent.ml_disabled
intent_action = intent.action
intent_messages = intent.messages
intent_root_followup_intent_name = intent.root_followup_intent_name
intent_training_phrases = intent.training_phrases
intent_priority = intent.priority
intent_display_name = intent.display_name
intent_parameters = intent.parameters
intent_parent_followup_intent_name = intent.parent_followup_intent_name
intent_default_response_platforms = intent.default_response_platforms
intent_input_context_names = intent.input_context_names
intent_followup_intent_info = intent.followup_intent_info
intent_is_fallback = intent.is_fallback
intent_live_agent_handoff = intent.live_agent_handoff
intent_name = intent.name
intent_reset_contexts = intent.reset_contexts
```

---


### Project

Creates/updates the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `api_version` | String |  | Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version. |
| `avatar_uri` | String |  | Optional. The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `default_language_code` | String |  | Required. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the `Update` method. |
| `time_zone` | String |  | Required. The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `enable_logging` | bool |  | Optional. Determines whether this agent should log conversation queries. |
| `classification_threshold` | f64 |  | Optional. To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used. |
| `display_name` | String |  | Required. The name of this agent. |
| `tier` | String |  | Optional. The agent tier. If not specified, TIER_STANDARD is assumed. |
| `description` | String |  | Optional. The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `match_mode` | String |  | Optional. Determines how intents are detected from user queries. |
| `parent` | String |  | Required. The project of this agent. Format: `projects/`. |
| `supported_language_codes` | Vec<String> |  | Optional. The list of all languages supported by this agent (except for the `default_language_code`). |
| `parent` | String | ✅ | Required. The project of this agent. Format: `projects/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api_version` | String | Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version. |
| `avatar_uri` | String | Optional. The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `default_language_code` | String | Required. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the `Update` method. |
| `time_zone` | String | Required. The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `enable_logging` | bool | Optional. Determines whether this agent should log conversation queries. |
| `classification_threshold` | f64 | Optional. To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used. |
| `display_name` | String | Required. The name of this agent. |
| `tier` | String | Optional. The agent tier. If not specified, TIER_STANDARD is assumed. |
| `description` | String | Optional. The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `match_mode` | String | Optional. Determines how intents are detected from user queries. |
| `parent` | String | Required. The project of this agent. Format: `projects/`. |
| `supported_language_codes` | Vec<String> | Optional. The list of all languages supported by this agent (except for the `default_language_code`). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.dialogflow_api.Project {
    parent = "value"  # Required. The project of this agent. Format: `projects/`.
}

# Access project outputs
project_id = project.id
project_api_version = project.api_version
project_avatar_uri = project.avatar_uri
project_default_language_code = project.default_language_code
project_time_zone = project.time_zone
project_enable_logging = project.enable_logging
project_classification_threshold = project.classification_threshold
project_display_name = project.display_name
project_tier = project.tier
project_description = project.description
project_match_mode = project.match_mode
project_parent = project.parent
project_supported_language_codes = project.supported_language_codes
```

---


### Agent

Imports the specified agent from a ZIP file. Uploads new intents and entity types without deleting the existing ones. Intents and entity types with the same name are replaced with the new versions from ImportAgentRequest. After the import, the imported draft agent will be trained automatically (unless disabled in agent settings). However, once the import is done, training may not be completed yet. Please call TrainAgent and wait for the operation it returns in order to train explicitly. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) The operation only tracks when importing is complete, not when it is done training. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `agent_content` | String |  | Zip compressed raw byte content for agent. |
| `agent_uri` | String |  | The URI to a Google Cloud Storage file containing the agent to import. Note: The URI must start with "gs://". Dialogflow performs a read operation for the Cloud Storage object on the caller's behalf, so your request authentication must have read permissions for the object. For more information, see [Dialogflow access control](https://cloud.google.com/dialogflow/cx/docs/concept/access-control#storage). |
| `parent` | String | ✅ | Required. The project that the agent to import is associated with. Format: `projects/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `agents` | Vec<String> | The list of agents. There will be a maximum number of items returned based on the page_size field in the request. |
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create agent
agent = provider.dialogflow_api.Agent {
    parent = "value"  # Required. The project that the agent to import is associated with. Format: `projects/`.
}

# Access agent outputs
agent_id = agent.id
agent_agents = agent.agents
agent_next_page_token = agent.next_page_token
```

---


### Tool

Creates a tool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connector_spec` | String |  | Integration connectors tool specification. |
| `description` | String |  | Optional. A human readable description of the tool. |
| `display_name` | String |  | Optional. A human readable short name of the tool, to be shown on the UI. |
| `function_spec` | String |  | Client side executed function specification. |
| `create_time` | String |  | Output only. Creation time of this tool. |
| `action_confirmation_requirement` | HashMap<String, String> |  | Optional. Confirmation requirement for the actions. Each key is an action name in the action_schemas. If an action's confirmation requirement is unspecified (either the key is not present, or its value is CONFIRMATION_REQUIREMENT_UNSPECIFIED), the requirement is inferred from the action's method_type - confirmation is not required if and only if method_type is GET. |
| `open_api_spec` | String |  | OpenAPI tool. |
| `update_time` | String |  | Output only. Update time of this tool. |
| `tool_key` | String |  | Required. A human readable short name of the tool, which should be unique within the project. It should only contain letters, numbers, and underscores, and it will be used by LLM to identify the tool. |
| `extension_spec` | String |  | Vertex extension tool specification. |
| `satisfies_pzi` | bool |  | Output only. A read only boolean field reflecting Zone Isolation status of the tool. If the field is absent, it means the status is unknown. |
| `name` | String |  | Output only. Identifier. The resource name of the tool. Format: `projects//locations//tools/`. |
| `satisfies_pzs` | bool |  | Output only. A read only boolean field reflecting Zone Separation status of the tool. If the field is absent, it means the status is unknown. |
| `parent` | String | ✅ | Required. The project/location to create tool for. Format: `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connector_spec` | String | Integration connectors tool specification. |
| `description` | String | Optional. A human readable description of the tool. |
| `display_name` | String | Optional. A human readable short name of the tool, to be shown on the UI. |
| `function_spec` | String | Client side executed function specification. |
| `create_time` | String | Output only. Creation time of this tool. |
| `action_confirmation_requirement` | HashMap<String, String> | Optional. Confirmation requirement for the actions. Each key is an action name in the action_schemas. If an action's confirmation requirement is unspecified (either the key is not present, or its value is CONFIRMATION_REQUIREMENT_UNSPECIFIED), the requirement is inferred from the action's method_type - confirmation is not required if and only if method_type is GET. |
| `open_api_spec` | String | OpenAPI tool. |
| `update_time` | String | Output only. Update time of this tool. |
| `tool_key` | String | Required. A human readable short name of the tool, which should be unique within the project. It should only contain letters, numbers, and underscores, and it will be used by LLM to identify the tool. |
| `extension_spec` | String | Vertex extension tool specification. |
| `satisfies_pzi` | bool | Output only. A read only boolean field reflecting Zone Isolation status of the tool. If the field is absent, it means the status is unknown. |
| `name` | String | Output only. Identifier. The resource name of the tool. Format: `projects//locations//tools/`. |
| `satisfies_pzs` | bool | Output only. A read only boolean field reflecting Zone Separation status of the tool. If the field is absent, it means the status is unknown. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tool
tool = provider.dialogflow_api.Tool {
    parent = "value"  # Required. The project/location to create tool for. Format: `projects//locations/`
}

# Access tool outputs
tool_id = tool.id
tool_connector_spec = tool.connector_spec
tool_description = tool.description
tool_display_name = tool.display_name
tool_function_spec = tool.function_spec
tool_create_time = tool.create_time
tool_action_confirmation_requirement = tool.action_confirmation_requirement
tool_open_api_spec = tool.open_api_spec
tool_update_time = tool.update_time
tool_tool_key = tool.tool_key
tool_extension_spec = tool.extension_spec
tool_satisfies_pzi = tool.satisfies_pzi
tool_name = tool.name
tool_satisfies_pzs = tool.satisfies_pzs
```

---


### Answer_record

Returns the list of all answer records in the specified project in reverse chronological order.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `answer_feedback` | String |  | Required. The AnswerFeedback for this record. You can set this with AnswerRecords.UpdateAnswerRecord in order to give us feedback about this answer. |
| `name` | String |  | The unique identifier of this answer record. Format: `projects//locations//answerRecords/`. |
| `agent_assistant_record` | String |  | Output only. The record for human agent assistant. |
| `name` | String | ✅ | The unique identifier of this answer record. Format: `projects//locations//answerRecords/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `answer_records` | Vec<String> | The list of answer records. |
| `next_page_token` | String | A token to retrieve next page of results. Or empty if there are no more results. Pass this value in the ListAnswerRecordsRequest.page_token field in the subsequent call to `ListAnswerRecords` method to retrieve the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access answer_record outputs
answer_record_id = answer_record.id
answer_record_answer_records = answer_record.answer_records
answer_record_next_page_token = answer_record.next_page_token
```

---


### Entitie

Deletes entities in the specified entity type. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `language_code` | String |  | Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity). |
| `entity_values` | Vec<String> |  | Required. The reference `values` of the entities to delete. Note that these are not fully-qualified names, i.e. they don't start with `projects/`. |
| `parent` | String | ✅ | Required. The name of the entity type to delete entries for. Format: `projects//agent/entityTypes/`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entitie
entitie = provider.dialogflow_api.Entitie {
    parent = "value"  # Required. The name of the entity type to delete entries for. Format: `projects//agent/entityTypes/`.
}

```

---


### Deployment

Retrieves the specified Deployment.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | Start time of this deployment. |
| `name` | String | The name of the deployment. Format: projects//locations//agents//environments//deployments/. |
| `end_time` | String | End time of this deployment. |
| `state` | String | The current state of the deployment. |
| `result` | String | Result of the deployment. |
| `flow_version` | String | The name of the flow version for this deployment. Format: projects//locations//agents//flows//versions/. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access deployment outputs
deployment_id = deployment.id
deployment_start_time = deployment.start_time
deployment_name = deployment.name
deployment_end_time = deployment.end_time
deployment_state = deployment.state
deployment_result = deployment.result
deployment_flow_version = deployment.flow_version
```

---


### Tool

Creates a Tool in the specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The unique identifier of the Tool. Format: `projects//locations//agents//tools/`. |
| `tool_type` | String |  | Output only. The tool type. |
| `data_store_spec` | String |  | Data store search tool specification. |
| `function_spec` | String |  | Client side executed function specification. |
| `description` | String |  | Required. High level description of the Tool and its usage. |
| `open_api_spec` | String |  | OpenAPI specification of the Tool. |
| `display_name` | String |  | Required. The human-readable name of the Tool, unique within an agent. |
| `parent` | String | ✅ | Required. The agent to create a Tool for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The unique identifier of the Tool. Format: `projects//locations//agents//tools/`. |
| `tool_type` | String | Output only. The tool type. |
| `data_store_spec` | String | Data store search tool specification. |
| `function_spec` | String | Client side executed function specification. |
| `description` | String | Required. High level description of the Tool and its usage. |
| `open_api_spec` | String | OpenAPI specification of the Tool. |
| `display_name` | String | Required. The human-readable name of the Tool, unique within an agent. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tool
tool = provider.dialogflow_api.Tool {
    parent = "value"  # Required. The agent to create a Tool for. Format: `projects//locations//agents/`.
}

# Access tool outputs
tool_id = tool.id
tool_name = tool.name
tool_tool_type = tool.tool_type
tool_data_store_spec = tool.data_store_spec
tool_function_spec = tool.function_spec
tool_description = tool.description
tool_open_api_spec = tool.open_api_spec
tool_display_name = tool.display_name
```

---


### Page

Creates a page in the specified flow. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `transition_routes` | Vec<String> |  | A list of transitions for the transition rules of this page. They route the conversation to another page in the same flow, or another flow. When we are in a certain page, the TransitionRoutes are evaluated in the following order: * TransitionRoutes defined in the page with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in flow with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in the page with only condition specified. * TransitionRoutes defined in the transition route groups with only condition specified. |
| `description` | String |  | The description of the page. The maximum length is 500 characters. |
| `name` | String |  | The unique identifier of the page. Required for the Pages.UpdatePage method. Pages.CreatePage populates the name automatically. Format: `projects//locations//agents//flows//pages/`. |
| `knowledge_connector_settings` | String |  | Optional. Knowledge connector configuration. |
| `transition_route_groups` | Vec<String> |  | Ordered list of `TransitionRouteGroups` added to the page. Transition route groups must be unique within a page. If the page links both flow-level transition route groups and agent-level transition route groups, the flow-level ones will have higher priority and will be put before the agent-level ones. * If multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes. * If multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence. Format:`projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `event_handlers` | Vec<String> |  | Handlers associated with the page to handle events such as webhook errors, no match or no input. |
| `form` | String |  | The form associated with the page, used for collecting parameters relevant to the page. |
| `display_name` | String |  | Required. The human-readable name of the page, unique within the flow. |
| `advanced_settings` | String |  | Hierarchical advanced settings for this page. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `entry_fulfillment` | String |  | The fulfillment to call when the session is entering the page. |
| `parent` | String | ✅ | Required. The flow to create a page for. Format: `projects//locations//agents//flows/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transition_routes` | Vec<String> | A list of transitions for the transition rules of this page. They route the conversation to another page in the same flow, or another flow. When we are in a certain page, the TransitionRoutes are evaluated in the following order: * TransitionRoutes defined in the page with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in flow with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in the page with only condition specified. * TransitionRoutes defined in the transition route groups with only condition specified. |
| `description` | String | The description of the page. The maximum length is 500 characters. |
| `name` | String | The unique identifier of the page. Required for the Pages.UpdatePage method. Pages.CreatePage populates the name automatically. Format: `projects//locations//agents//flows//pages/`. |
| `knowledge_connector_settings` | String | Optional. Knowledge connector configuration. |
| `transition_route_groups` | Vec<String> | Ordered list of `TransitionRouteGroups` added to the page. Transition route groups must be unique within a page. If the page links both flow-level transition route groups and agent-level transition route groups, the flow-level ones will have higher priority and will be put before the agent-level ones. * If multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes. * If multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence. Format:`projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `event_handlers` | Vec<String> | Handlers associated with the page to handle events such as webhook errors, no match or no input. |
| `form` | String | The form associated with the page, used for collecting parameters relevant to the page. |
| `display_name` | String | Required. The human-readable name of the page, unique within the flow. |
| `advanced_settings` | String | Hierarchical advanced settings for this page. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `entry_fulfillment` | String | The fulfillment to call when the session is entering the page. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create page
page = provider.dialogflow_api.Page {
    parent = "value"  # Required. The flow to create a page for. Format: `projects//locations//agents//flows/`.
}

# Access page outputs
page_id = page.id
page_transition_routes = page.transition_routes
page_description = page.description
page_name = page.name
page_knowledge_connector_settings = page.knowledge_connector_settings
page_transition_route_groups = page.transition_route_groups
page_event_handlers = page.event_handlers
page_form = page.form
page_display_name = page.display_name
page_advanced_settings = page.advanced_settings
page_entry_fulfillment = page.entry_fulfillment
```

---


### Security_setting

Create security settings in the specified location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `retention_window_days` | i64 |  | Retains the data for the specified number of days. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL. When data retention configuration is changed, it only applies to the data created after the change; the TTL of existing data created before the change stays intact. |
| `audio_export_settings` | String |  | Controls audio export settings for post-conversation analytics when ingesting audio to conversations via Participants.AnalyzeContent or Participants.StreamingAnalyzeContent. If retention_strategy is set to REMOVE_AFTER_CONVERSATION or audio_export_settings.gcs_bucket is empty, audio export is disabled. If audio export is enabled, audio is recorded and saved to audio_export_settings.gcs_bucket, subject to retention policy of audio_export_settings.gcs_bucket. This setting won't effect audio input for implicit sessions via Sessions.DetectIntent or Sessions.StreamingDetectIntent. |
| `deidentify_template` | String |  | [DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. The `DLP De-identify Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, Dialogflow replaces sensitive info with `[redacted]` text. The template name will have one of the following formats: `projects//locations//deidentifyTemplates/` OR `organizations//locations//deidentifyTemplates/` Note: `deidentify_template` must be located in the same region as the `SecuritySettings`. |
| `insights_export_settings` | String |  | Controls conversation exporting settings to Insights after conversation is completed. If retention_strategy is set to REMOVE_AFTER_CONVERSATION, Insights export is disabled no matter what you configure here. |
| `inspect_template` | String |  | [DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. The `DLP Inspect Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, we use the default DLP inspect config. The template name will have one of the following formats: `projects//locations//inspectTemplates/` OR `organizations//locations//inspectTemplates/` Note: `inspect_template` must be located in the same region as the `SecuritySettings`. |
| `name` | String |  | Resource name of the settings. Required for the SecuritySettingsService.UpdateSecuritySettings method. SecuritySettingsService.CreateSecuritySettings populates the name automatically. Format: `projects//locations//securitySettings/`. |
| `retention_strategy` | String |  | Specifies the retention behavior defined by SecuritySettings.RetentionStrategy. |
| `purge_data_types` | Vec<String> |  | List of types of data to remove when retention settings triggers purge. |
| `display_name` | String |  | Required. The human-readable name of the security settings, unique within the location. |
| `redaction_scope` | String |  | Defines the data for which Dialogflow applies redaction. Dialogflow does not redact data that it does not have access to – for example, Cloud logging. |
| `redaction_strategy` | String |  | Strategy that defines how we do redaction. |
| `parent` | String | ✅ | Required. The location to create an SecuritySettings for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `retention_window_days` | i64 | Retains the data for the specified number of days. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL. When data retention configuration is changed, it only applies to the data created after the change; the TTL of existing data created before the change stays intact. |
| `audio_export_settings` | String | Controls audio export settings for post-conversation analytics when ingesting audio to conversations via Participants.AnalyzeContent or Participants.StreamingAnalyzeContent. If retention_strategy is set to REMOVE_AFTER_CONVERSATION or audio_export_settings.gcs_bucket is empty, audio export is disabled. If audio export is enabled, audio is recorded and saved to audio_export_settings.gcs_bucket, subject to retention policy of audio_export_settings.gcs_bucket. This setting won't effect audio input for implicit sessions via Sessions.DetectIntent or Sessions.StreamingDetectIntent. |
| `deidentify_template` | String | [DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. The `DLP De-identify Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, Dialogflow replaces sensitive info with `[redacted]` text. The template name will have one of the following formats: `projects//locations//deidentifyTemplates/` OR `organizations//locations//deidentifyTemplates/` Note: `deidentify_template` must be located in the same region as the `SecuritySettings`. |
| `insights_export_settings` | String | Controls conversation exporting settings to Insights after conversation is completed. If retention_strategy is set to REMOVE_AFTER_CONVERSATION, Insights export is disabled no matter what you configure here. |
| `inspect_template` | String | [DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. The `DLP Inspect Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, we use the default DLP inspect config. The template name will have one of the following formats: `projects//locations//inspectTemplates/` OR `organizations//locations//inspectTemplates/` Note: `inspect_template` must be located in the same region as the `SecuritySettings`. |
| `name` | String | Resource name of the settings. Required for the SecuritySettingsService.UpdateSecuritySettings method. SecuritySettingsService.CreateSecuritySettings populates the name automatically. Format: `projects//locations//securitySettings/`. |
| `retention_strategy` | String | Specifies the retention behavior defined by SecuritySettings.RetentionStrategy. |
| `purge_data_types` | Vec<String> | List of types of data to remove when retention settings triggers purge. |
| `display_name` | String | Required. The human-readable name of the security settings, unique within the location. |
| `redaction_scope` | String | Defines the data for which Dialogflow applies redaction. Dialogflow does not redact data that it does not have access to – for example, Cloud logging. |
| `redaction_strategy` | String | Strategy that defines how we do redaction. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_setting
security_setting = provider.dialogflow_api.Security_setting {
    parent = "value"  # Required. The location to create an SecuritySettings for. Format: `projects//locations/`.
}

# Access security_setting outputs
security_setting_id = security_setting.id
security_setting_retention_window_days = security_setting.retention_window_days
security_setting_audio_export_settings = security_setting.audio_export_settings
security_setting_deidentify_template = security_setting.deidentify_template
security_setting_insights_export_settings = security_setting.insights_export_settings
security_setting_inspect_template = security_setting.inspect_template
security_setting_name = security_setting.name
security_setting_retention_strategy = security_setting.retention_strategy
security_setting_purge_data_types = security_setting.purge_data_types
security_setting_display_name = security_setting.display_name
security_setting_redaction_scope = security_setting.redaction_scope
security_setting_redaction_strategy = security_setting.redaction_strategy
```

---


### Agent

Creates an agent in the specified location. Note: You should always train flows prior to sending them queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `personalization_settings` | String |  | Optional. Settings for end user personalization. |
| `security_settings` | String |  | Name of the SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`. |
| `advanced_settings` | String |  | Hierarchical advanced settings for this agent. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `enable_spell_correction` | bool |  | Indicates if automatic spell correction is enabled in detect intent requests. |
| `client_certificate_settings` | String |  | Optional. Settings for custom client certificates. |
| `locked` | bool |  | Indicates whether the agent is locked for changes. If the agent is locked, modifications to the agent will be rejected except for RestoreAgent. |
| `enable_multi_language_training` | bool |  | Optional. Enable training multi-lingual models for this agent. These models will be trained on all the languages supported by the agent. |
| `description` | String |  | The description of the agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `satisfies_pzi` | bool |  | Optional. Output only. A read only boolean field reflecting Zone Isolation status of the agent. |
| `answer_feedback_settings` | String |  | Optional. Answer feedback collection settings. |
| `default_language_code` | String |  | Required. Immutable. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the Agents.UpdateAgent method. |
| `gen_app_builder_settings` | String |  | Gen App Builder-related agent-level settings. |
| `enable_stackdriver_logging` | bool |  | Indicates if stackdriver logging is enabled for the agent. Please use agent.advanced_settings instead. |
| `satisfies_pzs` | bool |  | Optional. Output only. A read only boolean field reflecting Zone Separation status of the agent. |
| `name` | String |  | The unique identifier of the agent. Required for the Agents.UpdateAgent method. Agents.CreateAgent populates the name automatically. Format: `projects//locations//agents/`. |
| `start_flow` | String |  | Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//flows/`. Currently only the default start flow with id "00000000-0000-0000-0000-000000000000" is allowed. |
| `start_playbook` | String |  | Name of the start playbook in this agent. A start playbook will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//playbooks/`. Currently only the default playbook with id "00000000-0000-0000-0000-000000000000" is allowed. |
| `text_to_speech_settings` | String |  | Settings on instructing the speech synthesizer on how to generate the output audio content. |
| `time_zone` | String |  | Required. The time zone of the agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `avatar_uri` | String |  | The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `speech_to_text_settings` | String |  | Speech recognition related settings. |
| `supported_language_codes` | Vec<String> |  | The list of all languages supported by the agent (except for the `default_language_code`). |
| `display_name` | String |  | Required. The human-readable name of the agent, unique within the location. |
| `git_integration_settings` | String |  | Git integration settings for this agent. |
| `parent` | String | ✅ | Required. The location to create a agent for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `personalization_settings` | String | Optional. Settings for end user personalization. |
| `security_settings` | String | Name of the SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`. |
| `advanced_settings` | String | Hierarchical advanced settings for this agent. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `enable_spell_correction` | bool | Indicates if automatic spell correction is enabled in detect intent requests. |
| `client_certificate_settings` | String | Optional. Settings for custom client certificates. |
| `locked` | bool | Indicates whether the agent is locked for changes. If the agent is locked, modifications to the agent will be rejected except for RestoreAgent. |
| `enable_multi_language_training` | bool | Optional. Enable training multi-lingual models for this agent. These models will be trained on all the languages supported by the agent. |
| `description` | String | The description of the agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `satisfies_pzi` | bool | Optional. Output only. A read only boolean field reflecting Zone Isolation status of the agent. |
| `answer_feedback_settings` | String | Optional. Answer feedback collection settings. |
| `default_language_code` | String | Required. Immutable. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the Agents.UpdateAgent method. |
| `gen_app_builder_settings` | String | Gen App Builder-related agent-level settings. |
| `enable_stackdriver_logging` | bool | Indicates if stackdriver logging is enabled for the agent. Please use agent.advanced_settings instead. |
| `satisfies_pzs` | bool | Optional. Output only. A read only boolean field reflecting Zone Separation status of the agent. |
| `name` | String | The unique identifier of the agent. Required for the Agents.UpdateAgent method. Agents.CreateAgent populates the name automatically. Format: `projects//locations//agents/`. |
| `start_flow` | String | Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//flows/`. Currently only the default start flow with id "00000000-0000-0000-0000-000000000000" is allowed. |
| `start_playbook` | String | Name of the start playbook in this agent. A start playbook will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//playbooks/`. Currently only the default playbook with id "00000000-0000-0000-0000-000000000000" is allowed. |
| `text_to_speech_settings` | String | Settings on instructing the speech synthesizer on how to generate the output audio content. |
| `time_zone` | String | Required. The time zone of the agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `avatar_uri` | String | The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `speech_to_text_settings` | String | Speech recognition related settings. |
| `supported_language_codes` | Vec<String> | The list of all languages supported by the agent (except for the `default_language_code`). |
| `display_name` | String | Required. The human-readable name of the agent, unique within the location. |
| `git_integration_settings` | String | Git integration settings for this agent. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create agent
agent = provider.dialogflow_api.Agent {
    parent = "value"  # Required. The location to create a agent for. Format: `projects//locations/`.
}

# Access agent outputs
agent_id = agent.id
agent_personalization_settings = agent.personalization_settings
agent_security_settings = agent.security_settings
agent_advanced_settings = agent.advanced_settings
agent_enable_spell_correction = agent.enable_spell_correction
agent_client_certificate_settings = agent.client_certificate_settings
agent_locked = agent.locked
agent_enable_multi_language_training = agent.enable_multi_language_training
agent_description = agent.description
agent_satisfies_pzi = agent.satisfies_pzi
agent_answer_feedback_settings = agent.answer_feedback_settings
agent_default_language_code = agent.default_language_code
agent_gen_app_builder_settings = agent.gen_app_builder_settings
agent_enable_stackdriver_logging = agent.enable_stackdriver_logging
agent_satisfies_pzs = agent.satisfies_pzs
agent_name = agent.name
agent_start_flow = agent.start_flow
agent_start_playbook = agent.start_playbook
agent_text_to_speech_settings = agent.text_to_speech_settings
agent_time_zone = agent.time_zone
agent_avatar_uri = agent.avatar_uri
agent_speech_to_text_settings = agent.speech_to_text_settings
agent_supported_language_codes = agent.supported_language_codes
agent_display_name = agent.display_name
agent_git_integration_settings = agent.git_integration_settings
```

---


### Example

Creates an example in the specified playbook.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `actions` | Vec<String> |  | Required. The ordered list of actions performed by the end user and the Dialogflow agent. |
| `conversation_state` | String |  | Required. Example's output state. |
| `display_name` | String |  | Required. The display name of the example. |
| `language_code` | String |  | Optional. The language code of the example. If not specified, the agent's default language is used. Note: languages must be enabled in the agent before they can be used. Note: example's language code is not currently used in dialogflow agents. |
| `description` | String |  | Optional. The high level concise description of the example. The max number of characters is 200. |
| `name` | String |  | The unique identifier of the playbook example. Format: `projects//locations//agents//playbooks//examples/`. |
| `playbook_input` | String |  | Optional. The input to the playbook in the example. |
| `token_count` | String |  | Output only. Estimated number of tokes current example takes when sent to the LLM. |
| `playbook_output` | String |  | Optional. The output of the playbook in the example. |
| `create_time` | String |  | Output only. The timestamp of initial example creation. |
| `update_time` | String |  | Output only. Last time the example was updated. |
| `parent` | String | ✅ | Required. The playbook to create an example for. Format: `projects//locations//agents//playbooks/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `actions` | Vec<String> | Required. The ordered list of actions performed by the end user and the Dialogflow agent. |
| `conversation_state` | String | Required. Example's output state. |
| `display_name` | String | Required. The display name of the example. |
| `language_code` | String | Optional. The language code of the example. If not specified, the agent's default language is used. Note: languages must be enabled in the agent before they can be used. Note: example's language code is not currently used in dialogflow agents. |
| `description` | String | Optional. The high level concise description of the example. The max number of characters is 200. |
| `name` | String | The unique identifier of the playbook example. Format: `projects//locations//agents//playbooks//examples/`. |
| `playbook_input` | String | Optional. The input to the playbook in the example. |
| `token_count` | String | Output only. Estimated number of tokes current example takes when sent to the LLM. |
| `playbook_output` | String | Optional. The output of the playbook in the example. |
| `create_time` | String | Output only. The timestamp of initial example creation. |
| `update_time` | String | Output only. Last time the example was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create example
example = provider.dialogflow_api.Example {
    parent = "value"  # Required. The playbook to create an example for. Format: `projects//locations//agents//playbooks/`.
}

# Access example outputs
example_id = example.id
example_actions = example.actions
example_conversation_state = example.conversation_state
example_display_name = example.display_name
example_language_code = example.language_code
example_description = example.description
example_name = example.name
example_playbook_input = example.playbook_input
example_token_count = example.token_count
example_playbook_output = example.playbook_output
example_create_time = example.create_time
example_update_time = example.update_time
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |


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
location_location_id = location.location_id
location_metadata = location.metadata
location_display_name = location.display_name
location_name = location.name
```

---


### Experiment

Creates an Experiment in the specified Environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Creation time of this experiment. |
| `state` | String |  | The current state of the experiment. Transition triggered by Experiments.StartExperiment: DRAFT->RUNNING. Transition triggered by Experiments.CancelExperiment: DRAFT->DONE or RUNNING->DONE. |
| `end_time` | String |  | End time of this experiment. |
| `display_name` | String |  | Required. The human-readable name of the experiment (unique in an environment). Limit of 64 characters. |
| `start_time` | String |  | Start time of this experiment. |
| `description` | String |  | The human-readable description of the experiment. |
| `result` | String |  | Inference result of the experiment. |
| `rollout_config` | String |  | The configuration for auto rollout. If set, there should be exactly two variants in the experiment (control variant being the default version of the flow), the traffic allocation for the non-control variant will gradually increase to 100% when conditions are met, and eventually replace the control variant to become the default version of the flow. |
| `variants_history` | Vec<String> |  | The history of updates to the experiment variants. |
| `rollout_state` | String |  | State of the auto rollout process. |
| `name` | String |  | The name of the experiment. Format: projects//locations//agents//environments//experiments/. |
| `experiment_length` | String |  | Maximum number of days to run the experiment/rollout. If auto-rollout is not enabled, default value and maximum will be 30 days. If auto-rollout is enabled, default value and maximum will be 6 days. |
| `definition` | String |  | The definition of the experiment. |
| `last_update_time` | String |  | Last update time of this experiment. |
| `rollout_failure_reason` | String |  | The reason why rollout has failed. Should only be set when state is ROLLOUT_FAILED. |
| `parent` | String | ✅ | Required. The Agent to create an Environment for. Format: `projects//locations//agents//environments/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Creation time of this experiment. |
| `state` | String | The current state of the experiment. Transition triggered by Experiments.StartExperiment: DRAFT->RUNNING. Transition triggered by Experiments.CancelExperiment: DRAFT->DONE or RUNNING->DONE. |
| `end_time` | String | End time of this experiment. |
| `display_name` | String | Required. The human-readable name of the experiment (unique in an environment). Limit of 64 characters. |
| `start_time` | String | Start time of this experiment. |
| `description` | String | The human-readable description of the experiment. |
| `result` | String | Inference result of the experiment. |
| `rollout_config` | String | The configuration for auto rollout. If set, there should be exactly two variants in the experiment (control variant being the default version of the flow), the traffic allocation for the non-control variant will gradually increase to 100% when conditions are met, and eventually replace the control variant to become the default version of the flow. |
| `variants_history` | Vec<String> | The history of updates to the experiment variants. |
| `rollout_state` | String | State of the auto rollout process. |
| `name` | String | The name of the experiment. Format: projects//locations//agents//environments//experiments/. |
| `experiment_length` | String | Maximum number of days to run the experiment/rollout. If auto-rollout is not enabled, default value and maximum will be 30 days. If auto-rollout is enabled, default value and maximum will be 6 days. |
| `definition` | String | The definition of the experiment. |
| `last_update_time` | String | Last update time of this experiment. |
| `rollout_failure_reason` | String | The reason why rollout has failed. Should only be set when state is ROLLOUT_FAILED. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create experiment
experiment = provider.dialogflow_api.Experiment {
    parent = "value"  # Required. The Agent to create an Environment for. Format: `projects//locations//agents//environments/`.
}

# Access experiment outputs
experiment_id = experiment.id
experiment_create_time = experiment.create_time
experiment_state = experiment.state
experiment_end_time = experiment.end_time
experiment_display_name = experiment.display_name
experiment_start_time = experiment.start_time
experiment_description = experiment.description
experiment_result = experiment.result
experiment_rollout_config = experiment.rollout_config
experiment_variants_history = experiment.variants_history
experiment_rollout_state = experiment.rollout_state
experiment_name = experiment.name
experiment_experiment_length = experiment.experiment_length
experiment_definition = experiment.definition
experiment_last_update_time = experiment.last_update_time
experiment_rollout_failure_reason = experiment.rollout_failure_reason
```

---


### Test_case

Creates a test case for the given agent.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `last_test_result` | String |  | The latest test result. |
| `test_case_conversation_turns` | Vec<String> |  | The conversation turns uttered when the test case was created, in chronological order. These include the canonical set of agent utterances that should occur when the agent is working properly. |
| `creation_time` | String |  | Output only. When the test was created. |
| `test_config` | String |  | Config for the test case. |
| `notes` | String |  | Additional freeform notes about the test case. Limit of 400 characters. |
| `name` | String |  | The unique identifier of the test case. TestCases.CreateTestCase will populate the name automatically. Otherwise use format: `projects//locations//agents//testCases/`. |
| `display_name` | String |  | Required. The human-readable name of the test case, unique within the agent. Limit of 200 characters. |
| `tags` | Vec<String> |  | Tags are short descriptions that users may apply to test cases for organizational and filtering purposes. Each tag should start with "#" and has a limit of 30 characters. |
| `parent` | String | ✅ | Required. The agent to create the test case for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_test_result` | String | The latest test result. |
| `test_case_conversation_turns` | Vec<String> | The conversation turns uttered when the test case was created, in chronological order. These include the canonical set of agent utterances that should occur when the agent is working properly. |
| `creation_time` | String | Output only. When the test was created. |
| `test_config` | String | Config for the test case. |
| `notes` | String | Additional freeform notes about the test case. Limit of 400 characters. |
| `name` | String | The unique identifier of the test case. TestCases.CreateTestCase will populate the name automatically. Otherwise use format: `projects//locations//agents//testCases/`. |
| `display_name` | String | Required. The human-readable name of the test case, unique within the agent. Limit of 200 characters. |
| `tags` | Vec<String> | Tags are short descriptions that users may apply to test cases for organizational and filtering purposes. Each tag should start with "#" and has a limit of 30 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create test_case
test_case = provider.dialogflow_api.Test_case {
    parent = "value"  # Required. The agent to create the test case for. Format: `projects//locations//agents/`.
}

# Access test_case outputs
test_case_id = test_case.id
test_case_last_test_result = test_case.last_test_result
test_case_test_case_conversation_turns = test_case.test_case_conversation_turns
test_case_creation_time = test_case.creation_time
test_case_test_config = test_case.test_config
test_case_notes = test_case.notes
test_case_name = test_case.name
test_case_display_name = test_case.display_name
test_case_tags = test_case.tags
```

---


### Generator

Creates a generator in the specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The human-readable name of the generator, unique within the agent. The prompt contains pre-defined parameters such as $conversation, $last-user-utterance, etc. populated by Dialogflow. It can also contain custom placeholders which will be resolved during fulfillment. |
| `llm_model_settings` | String |  | The LLM model settings. |
| `prompt_text` | String |  | Required. Prompt for the LLM model. |
| `placeholders` | Vec<String> |  | Optional. List of custom placeholders in the prompt text. |
| `name` | String |  | The unique identifier of the generator. Must be set for the Generators.UpdateGenerator method. Generators.CreateGenerate populates the name automatically. Format: `projects//locations//agents//generators/`. |
| `model_parameter` | String |  | Parameters passed to the LLM to configure its behavior. |
| `parent` | String | ✅ | Required. The agent to create a generator for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The human-readable name of the generator, unique within the agent. The prompt contains pre-defined parameters such as $conversation, $last-user-utterance, etc. populated by Dialogflow. It can also contain custom placeholders which will be resolved during fulfillment. |
| `llm_model_settings` | String | The LLM model settings. |
| `prompt_text` | String | Required. Prompt for the LLM model. |
| `placeholders` | Vec<String> | Optional. List of custom placeholders in the prompt text. |
| `name` | String | The unique identifier of the generator. Must be set for the Generators.UpdateGenerator method. Generators.CreateGenerate populates the name automatically. Format: `projects//locations//agents//generators/`. |
| `model_parameter` | String | Parameters passed to the LLM to configure its behavior. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create generator
generator = provider.dialogflow_api.Generator {
    parent = "value"  # Required. The agent to create a generator for. Format: `projects//locations//agents/`.
}

# Access generator outputs
generator_id = generator.id
generator_display_name = generator.display_name
generator_llm_model_settings = generator.llm_model_settings
generator_prompt_text = generator.prompt_text
generator_placeholders = generator.placeholders
generator_name = generator.name
generator_model_parameter = generator.model_parameter
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.dialogflow_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_error = operation.error
operation_response = operation.response
operation_done = operation.done
operation_name = operation.name
```

---


### Version

Creates a Version in the specified Flow. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: CreateVersionOperationMetadata - `response`: Version

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Create time of the version. |
| `name` | String |  | Format: projects//locations//agents//flows//versions/. Version ID is a self-increasing number generated by Dialogflow upon version creation. |
| `state` | String |  | Output only. The state of this version. This field is read-only and cannot be set by create and update methods. |
| `nlu_settings` | String |  | Output only. The NLU settings of the flow at version creation. |
| `description` | String |  | The description of the version. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `display_name` | String |  | Required. The human-readable name of the version. Limit of 64 characters. |
| `parent` | String | ✅ | Required. The Flow to create an Version for. Format: `projects//locations//agents//flows/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Create time of the version. |
| `name` | String | Format: projects//locations//agents//flows//versions/. Version ID is a self-increasing number generated by Dialogflow upon version creation. |
| `state` | String | Output only. The state of this version. This field is read-only and cannot be set by create and update methods. |
| `nlu_settings` | String | Output only. The NLU settings of the flow at version creation. |
| `description` | String | The description of the version. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `display_name` | String | Required. The human-readable name of the version. Limit of 64 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.dialogflow_api.Version {
    parent = "value"  # Required. The Flow to create an Version for. Format: `projects//locations//agents//flows/`.
}

# Access version outputs
version_id = version.id
version_create_time = version.create_time
version_name = version.name
version_state = version.state
version_nlu_settings = version.nlu_settings
version_description = version.description
version_display_name = version.display_name
```

---


### Continuous_test_result

Fetches a list of continuous test results for a given environment.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `continuous_test_results` | Vec<String> | The list of continuous test results. |
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access continuous_test_result outputs
continuous_test_result_id = continuous_test_result.id
continuous_test_result_continuous_test_results = continuous_test_result.continuous_test_results
continuous_test_result_next_page_token = continuous_test_result.next_page_token
```

---


### Session

Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause session entity types to be updated, which in turn might affect results of future queries. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/cx/docs/concept/version).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query_params` | String |  | The parameters of this query. |
| `output_audio_config` | String |  | Instructs the speech synthesizer how to generate the output audio. |
| `query_input` | String |  | Required. The input specification. |
| `session` | String | ✅ | Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session). Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/cx/docs/concept/version). |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create session
session = provider.dialogflow_api.Session {
    session = "value"  # Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session). Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/cx/docs/concept/version).
}

```

---


### Flow

Creates a flow in the specified agent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `nlu_settings` | String |  | NLU related settings of the flow. |
| `transition_route_groups` | Vec<String> |  | A flow's transition route group serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition route groups. Transition route groups defined in the page have higher priority than those defined in the flow. Format: `projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `display_name` | String |  | Required. The human-readable name of the flow. |
| `description` | String |  | The description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `event_handlers` | Vec<String> |  | A flow's event handlers serve two purposes: * They are responsible for handling events (e.g. no match, webhook errors) in the flow. * They are inherited by every page's event handlers, which can be used to handle common events regardless of the current page. Event handlers defined in the page have higher priority than those defined in the flow. Unlike transition_routes, these handlers are evaluated on a first-match basis. The first one that matches the event get executed, with the rest being ignored. |
| `transition_routes` | Vec<String> |  | A flow's transition routes serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition routes and can support use cases such as the user saying "help" or "can I talk to a human?", which can be handled in a common way regardless of the current page. Transition routes defined in the page have higher priority than those defined in the flow. TransitionRoutes are evaluated in the following order: * TransitionRoutes with intent specified. * TransitionRoutes with only condition specified. TransitionRoutes with intent specified are inherited by pages in the flow. |
| `knowledge_connector_settings` | String |  | Optional. Knowledge connector configuration. |
| `multi_language_settings` | String |  | Optional. Multi-lingual agent settings for this flow. |
| `output_parameter_definitions` | Vec<String> |  | Optional. Defined structured output parameters for this flow. |
| `locked` | bool |  | Indicates whether the flow is locked for changes. If the flow is locked, modifications to the flow will be rejected. |
| `advanced_settings` | String |  | Hierarchical advanced settings for this flow. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `name` | String |  | The unique identifier of the flow. Format: `projects//locations//agents//flows/`. |
| `input_parameter_definitions` | Vec<String> |  | Optional. Defined structured input parameters for this flow. |
| `parent` | String | ✅ | Required. The agent to create a flow for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `nlu_settings` | String | NLU related settings of the flow. |
| `transition_route_groups` | Vec<String> | A flow's transition route group serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition route groups. Transition route groups defined in the page have higher priority than those defined in the flow. Format: `projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `display_name` | String | Required. The human-readable name of the flow. |
| `description` | String | The description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `event_handlers` | Vec<String> | A flow's event handlers serve two purposes: * They are responsible for handling events (e.g. no match, webhook errors) in the flow. * They are inherited by every page's event handlers, which can be used to handle common events regardless of the current page. Event handlers defined in the page have higher priority than those defined in the flow. Unlike transition_routes, these handlers are evaluated on a first-match basis. The first one that matches the event get executed, with the rest being ignored. |
| `transition_routes` | Vec<String> | A flow's transition routes serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition routes and can support use cases such as the user saying "help" or "can I talk to a human?", which can be handled in a common way regardless of the current page. Transition routes defined in the page have higher priority than those defined in the flow. TransitionRoutes are evaluated in the following order: * TransitionRoutes with intent specified. * TransitionRoutes with only condition specified. TransitionRoutes with intent specified are inherited by pages in the flow. |
| `knowledge_connector_settings` | String | Optional. Knowledge connector configuration. |
| `multi_language_settings` | String | Optional. Multi-lingual agent settings for this flow. |
| `output_parameter_definitions` | Vec<String> | Optional. Defined structured output parameters for this flow. |
| `locked` | bool | Indicates whether the flow is locked for changes. If the flow is locked, modifications to the flow will be rejected. |
| `advanced_settings` | String | Hierarchical advanced settings for this flow. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `name` | String | The unique identifier of the flow. Format: `projects//locations//agents//flows/`. |
| `input_parameter_definitions` | Vec<String> | Optional. Defined structured input parameters for this flow. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create flow
flow = provider.dialogflow_api.Flow {
    parent = "value"  # Required. The agent to create a flow for. Format: `projects//locations//agents/`.
}

# Access flow outputs
flow_id = flow.id
flow_nlu_settings = flow.nlu_settings
flow_transition_route_groups = flow.transition_route_groups
flow_display_name = flow.display_name
flow_description = flow.description
flow_event_handlers = flow.event_handlers
flow_transition_routes = flow.transition_routes
flow_knowledge_connector_settings = flow.knowledge_connector_settings
flow_multi_language_settings = flow.multi_language_settings
flow_output_parameter_definitions = flow.output_parameter_definitions
flow_locked = flow.locked
flow_advanced_settings = flow.advanced_settings
flow_name = flow.name
flow_input_parameter_definitions = flow.input_parameter_definitions
```

---


### Playbook

Creates a playbook in a specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `goal` | String |  | Required. High level description of the goal the playbook intend to accomplish. A goal should be concise since it's visible to other playbooks that may reference this playbook. |
| `referenced_playbooks` | Vec<String> |  | Output only. The resource name of other playbooks referenced by the current playbook in the instructions. |
| `update_time` | String |  | Output only. Last time the playbook version was updated. |
| `inline_actions` | Vec<String> |  | Optional. Output only. Names of inline actions scoped to this playbook. These actions are in addition to those belonging to referenced tools, child playbooks, and flows, e.g. actions that are defined in the playbook's code block. |
| `referenced_flows` | Vec<String> |  | Output only. The resource name of flows referenced by the current playbook in the instructions. |
| `display_name` | String |  | Required. The human-readable name of the playbook, unique within an agent. |
| `llm_model_settings` | String |  | Optional. Llm model settings for the playbook. |
| `name` | String |  | The unique identifier of the playbook. Format: `projects//locations//agents//playbooks/`. |
| `instruction` | String |  | Instruction to accomplish target goal. |
| `input_parameter_definitions` | Vec<String> |  | Optional. Defined structured input parameters for this playbook. |
| `create_time` | String |  | Output only. The timestamp of initial playbook creation. |
| `referenced_tools` | Vec<String> |  | Optional. The resource name of tools referenced by the current playbook in the instructions. If not provided explicitly, they are will be implied using the tool being referenced in goal and steps. |
| `handlers` | Vec<String> |  | Optional. A list of registered handlers to execuate based on the specified triggers. |
| `playbook_type` | String |  | Optional. Type of the playbook. |
| `code_block` | String |  | Optional. The playbook's scoped code block, which may implement handlers and actions. |
| `token_count` | String |  | Output only. Estimated number of tokes current playbook takes when sent to the LLM. |
| `output_parameter_definitions` | Vec<String> |  | Optional. Defined structured output parameters for this playbook. |
| `parent` | String | ✅ | Required. The agent to create a playbook for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `goal` | String | Required. High level description of the goal the playbook intend to accomplish. A goal should be concise since it's visible to other playbooks that may reference this playbook. |
| `referenced_playbooks` | Vec<String> | Output only. The resource name of other playbooks referenced by the current playbook in the instructions. |
| `update_time` | String | Output only. Last time the playbook version was updated. |
| `inline_actions` | Vec<String> | Optional. Output only. Names of inline actions scoped to this playbook. These actions are in addition to those belonging to referenced tools, child playbooks, and flows, e.g. actions that are defined in the playbook's code block. |
| `referenced_flows` | Vec<String> | Output only. The resource name of flows referenced by the current playbook in the instructions. |
| `display_name` | String | Required. The human-readable name of the playbook, unique within an agent. |
| `llm_model_settings` | String | Optional. Llm model settings for the playbook. |
| `name` | String | The unique identifier of the playbook. Format: `projects//locations//agents//playbooks/`. |
| `instruction` | String | Instruction to accomplish target goal. |
| `input_parameter_definitions` | Vec<String> | Optional. Defined structured input parameters for this playbook. |
| `create_time` | String | Output only. The timestamp of initial playbook creation. |
| `referenced_tools` | Vec<String> | Optional. The resource name of tools referenced by the current playbook in the instructions. If not provided explicitly, they are will be implied using the tool being referenced in goal and steps. |
| `handlers` | Vec<String> | Optional. A list of registered handlers to execuate based on the specified triggers. |
| `playbook_type` | String | Optional. Type of the playbook. |
| `code_block` | String | Optional. The playbook's scoped code block, which may implement handlers and actions. |
| `token_count` | String | Output only. Estimated number of tokes current playbook takes when sent to the LLM. |
| `output_parameter_definitions` | Vec<String> | Optional. Defined structured output parameters for this playbook. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create playbook
playbook = provider.dialogflow_api.Playbook {
    parent = "value"  # Required. The agent to create a playbook for. Format: `projects//locations//agents/`.
}

# Access playbook outputs
playbook_id = playbook.id
playbook_goal = playbook.goal
playbook_referenced_playbooks = playbook.referenced_playbooks
playbook_update_time = playbook.update_time
playbook_inline_actions = playbook.inline_actions
playbook_referenced_flows = playbook.referenced_flows
playbook_display_name = playbook.display_name
playbook_llm_model_settings = playbook.llm_model_settings
playbook_name = playbook.name
playbook_instruction = playbook.instruction
playbook_input_parameter_definitions = playbook.input_parameter_definitions
playbook_create_time = playbook.create_time
playbook_referenced_tools = playbook.referenced_tools
playbook_handlers = playbook.handlers
playbook_playbook_type = playbook.playbook_type
playbook_code_block = playbook.code_block
playbook_token_count = playbook.token_count
playbook_output_parameter_definitions = playbook.output_parameter_definitions
```

---


### Environment

Creates an Environment in the specified Agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: Environment

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The human-readable name of the environment (unique in an agent). Limit of 64 characters. |
| `name` | String |  | The name of the environment. Format: `projects//locations//agents//environments/`. |
| `update_time` | String |  | Output only. Update time of this environment. |
| `version_configs` | Vec<String> |  | A list of configurations for flow versions. You should include version configs for all flows that are reachable from `Start Flow` in the agent. Otherwise, an error will be returned. |
| `webhook_config` | String |  | The webhook configuration for this environment. |
| `test_cases_config` | String |  | The test cases config for continuous tests of this environment. |
| `description` | String |  | The human-readable description of the environment. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `parent` | String | ✅ | Required. The Agent to create an Environment for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The human-readable name of the environment (unique in an agent). Limit of 64 characters. |
| `name` | String | The name of the environment. Format: `projects//locations//agents//environments/`. |
| `update_time` | String | Output only. Update time of this environment. |
| `version_configs` | Vec<String> | A list of configurations for flow versions. You should include version configs for all flows that are reachable from `Start Flow` in the agent. Otherwise, an error will be returned. |
| `webhook_config` | String | The webhook configuration for this environment. |
| `test_cases_config` | String | The test cases config for continuous tests of this environment. |
| `description` | String | The human-readable description of the environment. The maximum length is 500 characters. If exceeded, the request is rejected. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create environment
environment = provider.dialogflow_api.Environment {
    parent = "value"  # Required. The Agent to create an Environment for. Format: `projects//locations//agents/`.
}

# Access environment outputs
environment_id = environment.id
environment_display_name = environment.display_name
environment_name = environment.name
environment_update_time = environment.update_time
environment_version_configs = environment.version_configs
environment_webhook_config = environment.webhook_config
environment_test_cases_config = environment.test_cases_config
environment_description = environment.description
```

---


### Webhook

Creates a webhook in the specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `generic_web_service` | String |  | Configuration for a generic web service. |
| `name` | String |  | The unique identifier of the webhook. Required for the Webhooks.UpdateWebhook method. Webhooks.CreateWebhook populates the name automatically. Format: `projects//locations//agents//webhooks/`. |
| `service_directory` | String |  | Configuration for a [Service Directory](https://cloud.google.com/service-directory) service. |
| `display_name` | String |  | Required. The human-readable name of the webhook, unique within the agent. |
| `disabled` | bool |  | Indicates whether the webhook is disabled. |
| `timeout` | String |  | Webhook execution timeout. Execution is considered failed if Dialogflow doesn't receive a response from webhook at the end of the timeout period. Defaults to 5 seconds, maximum allowed timeout is 30 seconds. |
| `parent` | String | ✅ | Required. The agent to create a webhook for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `generic_web_service` | String | Configuration for a generic web service. |
| `name` | String | The unique identifier of the webhook. Required for the Webhooks.UpdateWebhook method. Webhooks.CreateWebhook populates the name automatically. Format: `projects//locations//agents//webhooks/`. |
| `service_directory` | String | Configuration for a [Service Directory](https://cloud.google.com/service-directory) service. |
| `display_name` | String | Required. The human-readable name of the webhook, unique within the agent. |
| `disabled` | bool | Indicates whether the webhook is disabled. |
| `timeout` | String | Webhook execution timeout. Execution is considered failed if Dialogflow doesn't receive a response from webhook at the end of the timeout period. Defaults to 5 seconds, maximum allowed timeout is 30 seconds. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create webhook
webhook = provider.dialogflow_api.Webhook {
    parent = "value"  # Required. The agent to create a webhook for. Format: `projects//locations//agents/`.
}

# Access webhook outputs
webhook_id = webhook.id
webhook_generic_web_service = webhook.generic_web_service
webhook_name = webhook.name
webhook_service_directory = webhook.service_directory
webhook_display_name = webhook.display_name
webhook_disabled = webhook.disabled
webhook_timeout = webhook.timeout
```

---


### Entity_type

Creates an entity type in the specified agent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType. Format: `projects//locations//agents//entityTypes/`. |
| `kind` | String |  | Required. Indicates the kind of entity type. |
| `display_name` | String |  | Required. The human-readable name of the entity type, unique within the agent. |
| `auto_expansion_mode` | String |  | Indicates whether the entity type can be automatically expanded. |
| `excluded_phrases` | Vec<String> |  | Collection of exceptional words and phrases that shouldn't be matched. For example, if you have a size entity type with entry `giant`(an adjective), you might consider adding `giants`(a noun) as an exclusion. If the kind of entity type is `KIND_MAP`, then the phrases specified by entities and excluded phrases should be mutually exclusive. |
| `enable_fuzzy_extraction` | bool |  | Enables fuzzy entity extraction during classification. |
| `redact` | bool |  | Indicates whether parameters of the entity type should be redacted in log. If redaction is enabled, page parameters and intent parameters referring to the entity type will be replaced by parameter name when logging. |
| `entities` | Vec<String> |  | The collection of entity entries associated with the entity type. |
| `parent` | String | ✅ | Required. The agent to create a entity type for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType. Format: `projects//locations//agents//entityTypes/`. |
| `kind` | String | Required. Indicates the kind of entity type. |
| `display_name` | String | Required. The human-readable name of the entity type, unique within the agent. |
| `auto_expansion_mode` | String | Indicates whether the entity type can be automatically expanded. |
| `excluded_phrases` | Vec<String> | Collection of exceptional words and phrases that shouldn't be matched. For example, if you have a size entity type with entry `giant`(an adjective), you might consider adding `giants`(a noun) as an exclusion. If the kind of entity type is `KIND_MAP`, then the phrases specified by entities and excluded phrases should be mutually exclusive. |
| `enable_fuzzy_extraction` | bool | Enables fuzzy entity extraction during classification. |
| `redact` | bool | Indicates whether parameters of the entity type should be redacted in log. If redaction is enabled, page parameters and intent parameters referring to the entity type will be replaced by parameter name when logging. |
| `entities` | Vec<String> | The collection of entity entries associated with the entity type. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entity_type
entity_type = provider.dialogflow_api.Entity_type {
    parent = "value"  # Required. The agent to create a entity type for. Format: `projects//locations//agents/`.
}

# Access entity_type outputs
entity_type_id = entity_type.id
entity_type_name = entity_type.name
entity_type_kind = entity_type.kind
entity_type_display_name = entity_type.display_name
entity_type_auto_expansion_mode = entity_type.auto_expansion_mode
entity_type_excluded_phrases = entity_type.excluded_phrases
entity_type_enable_fuzzy_extraction = entity_type.enable_fuzzy_extraction
entity_type_redact = entity_type.redact
entity_type_entities = entity_type.entities
```

---


### Transition_route_group

Creates an TransitionRouteGroup in the specified flow. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The unique identifier of the transition route group. TransitionRouteGroups.CreateTransitionRouteGroup populates the name automatically. Format: `projects//locations//agents//flows//transitionRouteGroups/` . |
| `display_name` | String |  | Required. The human-readable name of the transition route group, unique within the flow. The display name can be no longer than 30 characters. |
| `transition_routes` | Vec<String> |  | Transition routes associated with the TransitionRouteGroup. |
| `parent` | String | ✅ | Required. The flow to create an TransitionRouteGroup for. Format: `projects//locations//agents//flows/` or `projects//locations//agents/` for agent-level groups. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The unique identifier of the transition route group. TransitionRouteGroups.CreateTransitionRouteGroup populates the name automatically. Format: `projects//locations//agents//flows//transitionRouteGroups/` . |
| `display_name` | String | Required. The human-readable name of the transition route group, unique within the flow. The display name can be no longer than 30 characters. |
| `transition_routes` | Vec<String> | Transition routes associated with the TransitionRouteGroup. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create transition_route_group
transition_route_group = provider.dialogflow_api.Transition_route_group {
    parent = "value"  # Required. The flow to create an TransitionRouteGroup for. Format: `projects//locations//agents//flows/` or `projects//locations//agents/` for agent-level groups.
}

# Access transition_route_group outputs
transition_route_group_id = transition_route_group.id
transition_route_group_name = transition_route_group.name
transition_route_group_display_name = transition_route_group.display_name
transition_route_group_transition_routes = transition_route_group.transition_routes
```

---


### Intent

Creates an intent in the specified agent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `is_fallback` | bool |  | Indicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation. Adding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event. |
| `name` | String |  | The unique identifier of the intent. Required for the Intents.UpdateIntent method. Intents.CreateIntent populates the name automatically. Format: `projects//locations//agents//intents/`. |
| `parameters` | Vec<String> |  | The collection of parameters associated with the intent. |
| `priority` | i64 |  | The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests. |
| `training_phrases` | Vec<String> |  | The collection of training phrases the agent is trained on to identify the intent. |
| `description` | String |  | Human readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters. |
| `labels` | HashMap<String, String> |  | The key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes. Prefix "sys-" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys-head * sys-contextual The above labels do not require value. "sys-head" means the intent is a head intent. "sys.contextual" means the intent is a contextual intent. |
| `display_name` | String |  | Required. The human-readable name of the intent, unique within the agent. |
| `parent` | String | ✅ | Required. The agent to create an intent for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `is_fallback` | bool | Indicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation. Adding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event. |
| `name` | String | The unique identifier of the intent. Required for the Intents.UpdateIntent method. Intents.CreateIntent populates the name automatically. Format: `projects//locations//agents//intents/`. |
| `parameters` | Vec<String> | The collection of parameters associated with the intent. |
| `priority` | i64 | The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests. |
| `training_phrases` | Vec<String> | The collection of training phrases the agent is trained on to identify the intent. |
| `description` | String | Human readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters. |
| `labels` | HashMap<String, String> | The key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes. Prefix "sys-" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys-head * sys-contextual The above labels do not require value. "sys-head" means the intent is a head intent. "sys.contextual" means the intent is a contextual intent. |
| `display_name` | String | Required. The human-readable name of the intent, unique within the agent. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create intent
intent = provider.dialogflow_api.Intent {
    parent = "value"  # Required. The agent to create an intent for. Format: `projects//locations//agents/`.
}

# Access intent outputs
intent_id = intent.id
intent_is_fallback = intent.is_fallback
intent_name = intent.name
intent_parameters = intent.parameters
intent_priority = intent.priority
intent_training_phrases = intent.training_phrases
intent_description = intent.description
intent_labels = intent.labels
intent_display_name = intent.display_name
```

---


### Result

Gets a test case result.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `test_time` | String | The time that the test was run. |
| `conversation_turns` | Vec<String> | The conversation turns uttered during the test case replay in chronological order. |
| `environment` | String | Environment where the test was run. If not set, it indicates the draft environment. |
| `name` | String | The resource name for the test case result. Format: `projects//locations//agents//testCases//results/`. |
| `test_result` | String | Whether the test case passed in the agent environment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access result outputs
result_id = result.id
result_test_time = result.test_time
result_conversation_turns = result.conversation_turns
result_environment = result.environment
result_name = result.name
result_test_result = result.test_result
```

---


### Changelog

Retrieves the specified Changelog.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `language_code` | String | The affected language code of the change. |
| `resource` | String | The affected resource name of the change. |
| `action` | String | The action of the change. |
| `create_time` | String | The timestamp of the change. |
| `name` | String | The unique identifier of the changelog. Format: `projects//locations//agents//changelogs/`. |
| `user_email` | String | Email address of the authenticated user. |
| `type` | String | The affected resource type. |
| `display_name` | String | The affected resource display name of the change. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access changelog outputs
changelog_id = changelog.id
changelog_language_code = changelog.language_code
changelog_resource = changelog.resource
changelog_action = changelog.action
changelog_create_time = changelog.create_time
changelog_name = changelog.name
changelog_user_email = changelog.user_email
changelog_type = changelog.type
changelog_display_name = changelog.display_name
```

---


### Operation

Starts asynchronous cancellation on a long-running operation.  The server
makes a best effort to cancel the operation, but success is not
guaranteed.  If the server doesn't support this method, it returns
`google.rpc.Code.UNIMPLEMENTED`.  Clients can use
Operations.GetOperation or
other methods to check whether the cancellation succeeded or whether the
operation completed despite cancellation. On successful cancellation,
the operation is not deleted; instead, it becomes an operation with
an Operation.error value with a google.rpc.Status.code of 1,
corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `done` | bool | If the value is `false`, it means the operation is still in progress.
If `true`, the operation is completed, and either `error` or `response` is
available. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success.  If the original
method returns no data on success, such as `Delete`, the response is
`google.protobuf.Empty`.  If the original method is standard
`Get`/`Create`/`Update`, the response should be the resource.  For other
methods, the response should have the type `XxxResponse`, where `Xxx`
is the original method name.  For example, if the original method name
is `TakeSnapshot()`, the inferred response type is
`TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation.  It typically
contains progress information and common metadata such as create time.
Some services might not provide such metadata.  Any method that returns a
long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that
originally returns it. If you use the default HTTP mapping, the
`name` should be a resource name ending with `operations/{unique_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.dialogflow_api.Operation {
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



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple operation resources
operation_0 = provider.dialogflow_api.Operation {
    name = "value-0"
}
operation_1 = provider.dialogflow_api.Operation {
    name = "value-1"
}
operation_2 = provider.dialogflow_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.dialogflow_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Dialogflow_api Documentation](https://cloud.google.com/dialogflow_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
